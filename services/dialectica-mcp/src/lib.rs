//! Codex-friendly MCP stdio server for DIALECTICA.
//!
//! The stdio transport is newline-delimited JSON-RPC. This server never writes
//! non-MCP text to stdout.

use std::{
    error::Error,
    fmt::{Display, Formatter},
    io::{self, BufRead, Write},
};

use axum::{
    body::Bytes,
    extract::State,
    http::{
        header::{AUTHORIZATION, CONTENT_TYPE, ORIGIN},
        HeaderMap, HeaderValue, StatusCode,
    },
    response::{IntoResponse, Response},
    routing::get,
    Json, Router,
};
use serde_json::json;

mod protocol;
mod tools;

pub use protocol::{handle_jsonrpc_line, handle_jsonrpc_lines};

pub const PROTOCOL_VERSION: &str = "2025-11-25";
pub const SERVER_NAME: &str = "dialectica-mcp";
pub const WELCOME: &str = "Welcome to DIALECTICA by TACITUS. Build your PRAXIS context capsule from documents, sources, notes, and review decisions. Use the four capsule classes: user, situation, tool, output. The local MCP loop can build, inspect, archive, validate, and export PRAXIS context packs while preserving source receipts, caveats, ontology, semantic graph, reasoning devices, language rules, and output contracts. MCP-created capsules are draft or assisted outputs; expert promotion remains an explicit human review path.";

#[derive(Debug, Clone)]
pub struct HttpMcpState {
    bearer_token: Option<String>,
    allowed_origins: Vec<String>,
}

impl HttpMcpState {
    pub fn from_env() -> Self {
        Self {
            bearer_token: std::env::var("DIALECTICA_MCP_BEARER_TOKEN")
                .ok()
                .filter(|value| !value.trim().is_empty()),
            allowed_origins: std::env::var("DIALECTICA_MCP_ALLOWED_ORIGINS")
                .unwrap_or_default()
                .split(',')
                .map(str::trim)
                .filter(|value| !value.is_empty())
                .map(str::to_owned)
                .collect(),
        }
    }

    pub fn auth_required(&self) -> bool {
        self.bearer_token.is_some()
    }
}

#[derive(Debug)]
pub enum McpServerError {
    Io(std::io::Error),
    Json(serde_json::Error),
}

impl Display for McpServerError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(error) => write!(formatter, "{error}"),
            Self::Json(error) => write!(formatter, "{error}"),
        }
    }
}

impl Error for McpServerError {}

impl From<std::io::Error> for McpServerError {
    fn from(error: std::io::Error) -> Self {
        Self::Io(error)
    }
}

impl From<serde_json::Error> for McpServerError {
    fn from(error: serde_json::Error) -> Self {
        Self::Json(error)
    }
}

pub fn run_stdio() -> Result<(), McpServerError> {
    let stdin = io::stdin();
    let mut stdout = io::stdout().lock();
    for line in stdin.lock().lines() {
        let line = line?;
        if line.trim().is_empty() {
            continue;
        }
        if let Some(response) = handle_jsonrpc_line(&line)? {
            stdout.write_all(response.as_bytes())?;
            stdout.write_all(b"\n")?;
            stdout.flush()?;
        }
    }
    Ok(())
}

pub fn http_app(state: HttpMcpState) -> Router {
    Router::new()
        .route("/health", get(health))
        .route("/mcp", get(mcp_info).post(mcp_post))
        .with_state(state)
}

pub async fn run_http(bind: &str, state: HttpMcpState) -> Result<(), std::io::Error> {
    let listener = tokio::net::TcpListener::bind(bind).await?;
    eprintln!("dialectica-mcp http bind={bind}");
    axum::serve(listener, http_app(state)).await
}

pub fn default_http_bind() -> String {
    std::env::var("PORT")
        .map(|port| format!("0.0.0.0:{port}"))
        .or_else(|_| std::env::var("DIALECTICA_MCP_BIND"))
        .unwrap_or_else(|_| "127.0.0.1:8090".to_owned())
}

async fn health() -> impl IntoResponse {
    Json(json!({
        "status": "ok",
        "server": SERVER_NAME,
        "protocol_version": PROTOCOL_VERSION
    }))
}

async fn mcp_info(State(state): State<HttpMcpState>, headers: HeaderMap) -> Response {
    if let Err((status, message)) = authorize(&state, &headers) {
        return json_error(status, message);
    }
    Json(json!({
        "server": SERVER_NAME,
        "transport": "streamable_http",
        "endpoint": "/mcp",
        "protocol_version": PROTOCOL_VERSION,
        "auth_required": state.auth_required(),
        "methods": ["POST"],
        "note": "POST one JSON-RPC MCP request per HTTP call. Server-sent event streaming is not required for the DIALECTICA build loop."
    }))
    .into_response()
}

async fn mcp_post(State(state): State<HttpMcpState>, headers: HeaderMap, body: Bytes) -> Response {
    if let Err((status, message)) = authorize(&state, &headers) {
        return json_error(status, message);
    }
    let Ok(text) = std::str::from_utf8(&body) else {
        return json_error(StatusCode::BAD_REQUEST, "request body must be UTF-8 JSON");
    };
    let trimmed = text.trim();
    if trimmed.is_empty() {
        return json_error(StatusCode::BAD_REQUEST, "request body must not be empty");
    }

    match handle_jsonrpc_line(trimmed) {
        Ok(Some(response)) => {
            let mut http_response = (StatusCode::OK, response).into_response();
            http_response.headers_mut().insert(
                CONTENT_TYPE,
                HeaderValue::from_static("application/json; charset=utf-8"),
            );
            http_response
        }
        Ok(None) => StatusCode::ACCEPTED.into_response(),
        Err(error) => json_error(
            StatusCode::INTERNAL_SERVER_ERROR,
            &format!("failed to serialize MCP response: {error}"),
        ),
    }
}

fn authorize(state: &HttpMcpState, headers: &HeaderMap) -> Result<(), (StatusCode, &'static str)> {
    if !state.allowed_origins.is_empty() {
        let origin = headers.get(ORIGIN).and_then(|value| value.to_str().ok());
        if let Some(origin) = origin {
            if !state
                .allowed_origins
                .iter()
                .any(|allowed| allowed == origin)
            {
                return Err((StatusCode::FORBIDDEN, "origin is not allowed"));
            }
        }
    }

    if let Some(expected) = &state.bearer_token {
        let expected = format!("Bearer {expected}");
        let authorized = headers
            .get(AUTHORIZATION)
            .and_then(|value| value.to_str().ok())
            .map(|value| value == expected)
            .unwrap_or(false);
        if !authorized {
            return Err((StatusCode::UNAUTHORIZED, "missing or invalid bearer token"));
        }
    }

    Ok(())
}

fn json_error(status: StatusCode, message: &str) -> Response {
    (status, Json(json!({ "error": message }))).into_response()
}

#[cfg(test)]
mod tests {
    use axum::{
        body::{to_bytes, Body},
        http::{header::AUTHORIZATION, Request, StatusCode},
    };
    use serde_json::Value;
    use tower::ServiceExt;

    use super::{handle_jsonrpc_line, http_app, HttpMcpState};

    #[test]
    fn initialize_advertises_dialectica_tools() {
        let response = handle_jsonrpc_line(
            r#"{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":"2025-11-25","capabilities":{},"clientInfo":{"name":"codex","version":"1"}}}"#,
        )
        .expect("initialize should serialize")
        .expect("initialize should respond");
        let value: Value = serde_json::from_str(&response).expect("response should parse");

        assert_eq!(value["result"]["serverInfo"]["name"], "dialectica-mcp");
        assert!(value["result"]["instructions"]
            .as_str()
            .unwrap_or("")
            .contains("DIALECTICA by TACITUS"));
    }

    #[test]
    fn tools_list_contains_build_capsule() {
        let response =
            handle_jsonrpc_line(r#"{"jsonrpc":"2.0","id":2,"method":"tools/list","params":{}}"#)
                .expect("tools/list should serialize")
                .expect("tools/list should respond");
        let value: Value = serde_json::from_str(&response).expect("response should parse");
        let tools = value["result"]["tools"].as_array().expect("tools array");

        assert!(tools
            .iter()
            .any(|tool| tool["name"] == "dialectica_build_capsule"));
        assert!(tools
            .iter()
            .any(|tool| tool["name"] == "dialectica_upload_sources"));
    }

    #[test]
    fn welcome_tool_returns_operator_copy() {
        let response = handle_jsonrpc_line(
            r#"{"jsonrpc":"2.0","id":3,"method":"tools/call","params":{"name":"dialectica_welcome","arguments":{}}}"#,
        )
        .expect("tool call should serialize")
        .expect("tool call should respond");
        let value: Value = serde_json::from_str(&response).expect("response should parse");
        let text = value["result"]["content"][0]["text"].as_str().unwrap_or("");

        assert!(text.contains("Build your PRAXIS context capsule"));
        assert_eq!(value["result"]["isError"], false);
    }

    #[tokio::test]
    async fn hosted_mcp_requires_bearer_token_when_configured() {
        let app = http_app(HttpMcpState {
            bearer_token: Some("secret".to_owned()),
            allowed_origins: Vec::new(),
        });
        let response = app
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/mcp")
                    .body(Body::from(r#"{"jsonrpc":"2.0","id":1,"method":"ping"}"#))
                    .expect("request"),
            )
            .await
            .expect("response");

        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    }

    #[tokio::test]
    async fn hosted_mcp_posts_json_rpc() {
        let app = http_app(HttpMcpState {
            bearer_token: Some("secret".to_owned()),
            allowed_origins: Vec::new(),
        });
        let response = app
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/mcp")
                    .header(AUTHORIZATION, "Bearer secret")
                    .body(Body::from(r#"{"jsonrpc":"2.0","id":1,"method":"ping"}"#))
                    .expect("request"),
            )
            .await
            .expect("response");
        let status = response.status();
        let body = to_bytes(response.into_body(), 1024 * 1024)
            .await
            .expect("body");
        let value: Value = serde_json::from_slice(&body).expect("json response");

        assert_eq!(status, StatusCode::OK);
        assert_eq!(value["result"], serde_json::json!({}));
    }
}
