use serde_json::{json, Value};

use crate::{tools, PROTOCOL_VERSION, SERVER_NAME, WELCOME};

const JSONRPC_VERSION: &str = "2.0";
const PARSE_ERROR: i64 = -32700;
const INVALID_REQUEST: i64 = -32600;
const METHOD_NOT_FOUND: i64 = -32601;
const INVALID_PARAMS: i64 = -32602;

#[derive(Debug)]
struct RpcError {
    code: i64,
    message: String,
    data: Option<Value>,
}

impl RpcError {
    fn new(code: i64, message: impl Into<String>) -> Self {
        Self {
            code,
            message: message.into(),
            data: None,
        }
    }

    fn with_data(code: i64, message: impl Into<String>, data: Value) -> Self {
        Self {
            code,
            message: message.into(),
            data: Some(data),
        }
    }
}

pub fn handle_jsonrpc_lines(input: &str) -> Result<String, serde_json::Error> {
    let mut responses = Vec::new();
    for line in input.lines() {
        if line.trim().is_empty() {
            continue;
        }
        if let Some(response) = handle_jsonrpc_line(line)? {
            responses.push(response);
        }
    }

    if responses.is_empty() {
        Ok(String::new())
    } else {
        Ok(format!("{}\n", responses.join("\n")))
    }
}

pub fn handle_jsonrpc_line(line: &str) -> Result<Option<String>, serde_json::Error> {
    let message = match serde_json::from_str::<Value>(line) {
        Ok(value) => value,
        Err(error) => {
            let response = jsonrpc_error(
                Value::Null,
                RpcError::new(PARSE_ERROR, format!("Parse error: {error}")),
            );
            return serde_json::to_string(&response).map(Some);
        }
    };

    match handle_jsonrpc_value(&message) {
        Some(value) => serde_json::to_string(&value).map(Some),
        None => Ok(None),
    }
}

fn handle_jsonrpc_value(message: &Value) -> Option<Value> {
    let Some(object) = message.as_object() else {
        return Some(jsonrpc_error(
            Value::Null,
            RpcError::new(INVALID_REQUEST, "Invalid JSON-RPC request"),
        ));
    };
    let id = object.get("id").cloned();
    let is_notification = id.is_none();

    if object.get("jsonrpc").and_then(Value::as_str) != Some(JSONRPC_VERSION) {
        return request_error_or_none(
            id,
            RpcError::new(INVALID_REQUEST, "Invalid JSON-RPC version"),
        );
    }

    let Some(method) = object.get("method").and_then(Value::as_str) else {
        return request_error_or_none(
            id,
            RpcError::new(INVALID_REQUEST, "Invalid JSON-RPC request"),
        );
    };

    if is_notification {
        accept_notification(method);
        return None;
    }

    let id = id.expect("request id checked above");
    match dispatch_request(method, object.get("params")) {
        Ok(result) => Some(jsonrpc_result(id, result)),
        Err(error) => Some(jsonrpc_error(id, error)),
    }
}

fn dispatch_request(method: &str, params: Option<&Value>) -> Result<Value, RpcError> {
    match method {
        "initialize" => initialize_result(params),
        "ping" => Ok(json!({})),
        "tools/list" => Ok(json!({ "tools": tools::tool_definitions() })),
        "tools/call" => {
            let params = params.and_then(Value::as_object).ok_or_else(|| {
                RpcError::new(INVALID_PARAMS, "tools/call params must be an object")
            })?;
            let name = params.get("name").and_then(Value::as_str).ok_or_else(|| {
                RpcError::new(INVALID_PARAMS, "tools/call params.name must be a string")
            })?;
            let arguments = params.get("arguments").unwrap_or(&Value::Null);
            if !arguments.is_object() {
                return Err(RpcError::new(
                    INVALID_PARAMS,
                    "tools/call params.arguments must be an object",
                ));
            }
            Ok(tools::call_tool(name, arguments))
        }
        "resources/list" => Ok(json!({ "resources": tools::resource_definitions() })),
        "resources/read" => {
            let params = object_params(params, "resources/read")?;
            Ok(tools::read_resource(params))
        }
        "prompts/list" => Ok(json!({ "prompts": tools::prompt_definitions() })),
        "prompts/get" => {
            let params = object_params(params, "prompts/get")?;
            Ok(tools::read_prompt(params))
        }
        _ => Err(RpcError::new(
            METHOD_NOT_FOUND,
            format!("Method not found: {method}"),
        )),
    }
}

fn object_params<'a>(params: Option<&'a Value>, method: &str) -> Result<&'a Value, RpcError> {
    params
        .filter(|value| value.is_object())
        .ok_or_else(|| RpcError::new(INVALID_PARAMS, format!("{method} params must be an object")))
}

fn initialize_result(params: Option<&Value>) -> Result<Value, RpcError> {
    let params = params
        .and_then(Value::as_object)
        .ok_or_else(|| RpcError::new(INVALID_PARAMS, "initialize params must be an object"))?;
    let requested = params
        .get("protocolVersion")
        .and_then(Value::as_str)
        .ok_or_else(|| {
            RpcError::with_data(
                INVALID_PARAMS,
                "Unsupported protocol version",
                json!({ "supported": [PROTOCOL_VERSION], "requested": Value::Null }),
            )
        })?;

    if requested != PROTOCOL_VERSION {
        return Err(RpcError::with_data(
            INVALID_PARAMS,
            "Unsupported protocol version",
            json!({ "supported": [PROTOCOL_VERSION], "requested": requested }),
        ));
    }

    Ok(json!({
        "protocolVersion": PROTOCOL_VERSION,
        "capabilities": {
            "tools": { "listChanged": false },
            "resources": { "subscribe": false, "listChanged": false },
            "prompts": { "listChanged": false }
        },
        "serverInfo": {
            "name": SERVER_NAME,
            "title": "DIALECTICA by TACITUS",
            "version": env!("CARGO_PKG_VERSION")
        },
        "instructions": WELCOME
    }))
}

fn accept_notification(method: &str) {
    match method {
        "notifications/initialized" | "notifications/roots/list_changed" => {}
        _ => {}
    }
}

fn request_error_or_none(id: Option<Value>, error: RpcError) -> Option<Value> {
    id.map(|id| jsonrpc_error(id, error))
}

fn jsonrpc_result(id: Value, result: Value) -> Value {
    json!({
        "jsonrpc": JSONRPC_VERSION,
        "id": id,
        "result": result
    })
}

fn jsonrpc_error(id: Value, error: RpcError) -> Value {
    let mut value = json!({
        "jsonrpc": JSONRPC_VERSION,
        "id": id,
        "error": {
            "code": error.code,
            "message": error.message
        }
    });
    if let Some(data) = error.data {
        value["error"]["data"] = data;
    }
    value
}
