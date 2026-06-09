//! Codex-friendly MCP stdio server for DIALECTICA.
//!
//! The stdio transport is newline-delimited JSON-RPC. This server never writes
//! non-MCP text to stdout.

use std::{
    error::Error,
    fmt::{Display, Formatter},
    io::{self, BufRead, Write},
};

mod protocol;
mod tools;

pub use protocol::{handle_jsonrpc_line, handle_jsonrpc_lines};

pub const PROTOCOL_VERSION: &str = "2025-11-25";
pub const SERVER_NAME: &str = "dialectica-mcp";
pub const WELCOME: &str = "Welcome to DIALECTICA by TACITUS. Build your PRAXIS context capsule from documents, sources, notes, and review decisions. Use the four capsule classes: user, situation, tool, output. The local MCP loop can build, inspect, archive, validate, and export PRAXIS context packs while preserving source receipts, caveats, ontology, semantic graph, reasoning devices, language rules, and output contracts. MCP-created capsules are draft or assisted outputs; expert promotion remains an explicit human review path.";

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

#[cfg(test)]
mod tests {
    use serde_json::Value;

    use super::handle_jsonrpc_line;

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
}
