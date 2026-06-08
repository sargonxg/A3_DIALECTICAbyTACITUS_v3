//! Codex-friendly MCP stdio server for DIALECTICA.
//!
//! The stdio transport is newline-delimited JSON-RPC. This server never writes
//! non-MCP text to stdout.

use std::{
    error::Error,
    fmt::{Display, Formatter},
    fs,
    io::{self, BufRead, Write},
    path::{Path, PathBuf},
};

use dialectica_capsule::PraxisCapsulePackage;
use dialectica_extractor::CapsuleType;
use serde::Serialize;
use serde_json::{json, Value};

const PROTOCOL_VERSION: &str = "2025-11-25";
const WELCOME: &str = "Welcome to DIALECTICA by TACITUS. Build your PRAXIS context capsule from documents, sources, notes, and review decisions. Use the four capsule classes: user, situation, tool, output. The local MCP loop can build, inspect, archive, and export PRAXIS context packs while preserving source receipts, caveats, ontology, semantic graph, reasoning devices, language rules, and output contracts.";

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

pub fn handle_jsonrpc_line(line: &str) -> Result<Option<String>, serde_json::Error> {
    let message = match serde_json::from_str::<Value>(line) {
        Ok(value) => value,
        Err(error) => {
            let response = jsonrpc_error(Value::Null, -32700, &format!("parse error: {error}"));
            return serde_json::to_string(&response).map(Some);
        }
    };
    let response = handle_jsonrpc_value(&message);
    match response {
        Some(value) => serde_json::to_string(&value).map(Some),
        None => Ok(None),
    }
}

fn handle_jsonrpc_value(message: &Value) -> Option<Value> {
    let id = message.get("id").cloned()?;
    let method = message.get("method").and_then(Value::as_str);
    let Some(method) = method else {
        return Some(jsonrpc_error(id, -32600, "invalid JSON-RPC request"));
    };
    let params = message.get("params").cloned().unwrap_or_else(|| json!({}));

    Some(match method {
        "initialize" => jsonrpc_result(id, initialize_result(&params)),
        "ping" => jsonrpc_result(id, json!({})),
        "tools/list" => jsonrpc_result(id, json!({ "tools": tool_definitions() })),
        "tools/call" => jsonrpc_result(id, call_tool(&params)),
        "resources/list" => jsonrpc_result(id, json!({ "resources": resource_definitions() })),
        "resources/read" => jsonrpc_result(id, read_resource(&params)),
        "prompts/list" => jsonrpc_result(id, json!({ "prompts": prompt_definitions() })),
        "prompts/get" => jsonrpc_result(id, read_prompt(&params)),
        _ => jsonrpc_error(id, -32601, &format!("unknown method: {method}")),
    })
}

fn initialize_result(params: &Value) -> Value {
    let protocol_version = params
        .get("protocolVersion")
        .and_then(Value::as_str)
        .unwrap_or(PROTOCOL_VERSION);
    json!({
        "protocolVersion": protocol_version,
        "capabilities": {
            "tools": { "listChanged": false },
            "resources": { "subscribe": false, "listChanged": false },
            "prompts": { "listChanged": false }
        },
        "serverInfo": {
            "name": "dialectica-mcp",
            "title": "DIALECTICA by TACITUS",
            "version": env!("CARGO_PKG_VERSION")
        },
        "instructions": WELCOME
    })
}

fn call_tool(params: &Value) -> Value {
    let name = params.get("name").and_then(Value::as_str).unwrap_or("");
    let arguments = params
        .get("arguments")
        .cloned()
        .unwrap_or_else(|| json!({}));
    let result = match name {
        "dialectica_welcome" => Ok(json!({ "welcome": WELCOME })),
        "dialectica_build_capsule" => build_capsule_tool(&arguments),
        "dialectica_inspect_capsule" => inspect_capsule_tool(&arguments),
        "dialectica_archive_capsule" => archive_capsule_tool(&arguments),
        "dialectica_export_praxis_pack" => export_praxis_pack_tool(&arguments),
        "dialectica_ontology_plan" => ontology_plan_tool(&arguments),
        "dialectica_mcp_config" => Ok(json!({ "config": mcp_config_snippet() })),
        _ => Err(format!("unknown tool: {name}")),
    };

    match result {
        Ok(value) => tool_json(value, false),
        Err(error) => tool_text(error, true),
    }
}

fn build_capsule_tool(arguments: &Value) -> Result<Value, String> {
    let input_dir = required_string(arguments, "input_dir")?;
    let out_dir = required_string(arguments, "out_dir")?;
    let capsule_type =
        dialectica_builder::parse_capsule_type(&required_string(arguments, "capsule_type")?)
            .map_err(|error| error.to_string())?;
    let mode = optional_string(arguments, "mode").unwrap_or_else(|| "assisted".to_owned());
    let build_mode =
        dialectica_builder::parse_build_mode(&mode).map_err(|error| error.to_string())?;
    let workflow =
        optional_string(arguments, "workflow").unwrap_or_else(|| default_workflow(capsule_type));
    let title = optional_string(arguments, "title");

    let receipt =
        dialectica_builder::build_documents_capsule(&dialectica_builder::BuildDocumentsOptions {
            input_dir: PathBuf::from(input_dir),
            output_dir: PathBuf::from(out_dir),
            capsule_type,
            build_mode,
            title,
            workflow,
        })
        .map_err(|error| error.to_string())?;
    serde_json::to_value(receipt).map_err(|error| error.to_string())
}

fn inspect_capsule_tool(arguments: &Value) -> Result<Value, String> {
    let package_dir = PathBuf::from(required_string(arguments, "package_dir")?);
    let package =
        PraxisCapsulePackage::load_from_dir(&package_dir).map_err(|error| error.to_string())?;
    let inspection = package.inspection();
    let ladybug = dialectica_graph::check_ladybug_projection(&package_dir).ok();

    Ok(json!({
        "capsule_id": inspection.capsule_id,
        "capsule_type": inspection.capsule_type,
        "review_state": format!("{:?}", inspection.review_state).to_lowercase(),
        "source_count": inspection.source_count,
        "claim_count": inspection.claim_count,
        "graph_node_count": inspection.graph_node_count,
        "graph_edge_count": inspection.graph_edge_count,
        "warning_count": inspection.warning_count,
        "manifest": package.manifest,
        "ladybug_projection": ladybug
    }))
}

fn archive_capsule_tool(arguments: &Value) -> Result<Value, String> {
    let package_dir = PathBuf::from(required_string(arguments, "package_dir")?);
    let output_file = optional_string(arguments, "out_file")
        .map(PathBuf::from)
        .unwrap_or_else(|| package_dir.with_extension("capsule"));
    let receipt = dialectica_compiler::write_capsule_archive(&package_dir, &output_file)
        .map_err(|error| error.to_string())?;
    serde_json::to_value(receipt).map_err(|error| error.to_string())
}

fn export_praxis_pack_tool(arguments: &Value) -> Result<Value, String> {
    let package_dir = PathBuf::from(required_string(arguments, "package_dir")?);
    let workflow =
        optional_string(arguments, "workflow").unwrap_or_else(|| "decision_brief".to_owned());
    let pack = dialectica_compiler::export_praxis_context_pack(&package_dir, &workflow)
        .map_err(|error| error.to_string())?;
    if let Some(out_file) = optional_string(arguments, "out_file") {
        let out_path = PathBuf::from(out_file);
        write_json(&out_path, &pack)?;
        Ok(json!({
            "capsule_id": pack.capsule_id,
            "workflow": pack.workflow,
            "praxis_context_pack_path": out_path
        }))
    } else {
        serde_json::to_value(pack).map_err(|error| error.to_string())
    }
}

fn ontology_plan_tool(arguments: &Value) -> Result<Value, String> {
    let package_dir = PathBuf::from(required_string(arguments, "package_dir")?);
    let package =
        PraxisCapsulePackage::load_from_dir(&package_dir).map_err(|error| error.to_string())?;
    serde_json::to_value(package.manifest.ontology_blueprint()).map_err(|error| error.to_string())
}

fn read_resource(params: &Value) -> Value {
    let uri = params.get("uri").and_then(Value::as_str).unwrap_or("");
    let text = match uri {
        "dialectica://welcome" => WELCOME.to_owned(),
        "dialectica://builder/contract" => {
            "DIALECTICA builds four capsule classes: user, situation, tool, output. Each capsule contains source receipts, claims, temporal status, ontology terms, graph nodes/edges, reasoning devices, caveats, language rules, rights rules, output contracts, embedded Ladybug projection metadata, and a PRAXIS context pack.".to_owned()
        }
        "dialectica://praxis/bridge" => {
            "Local bridge: use praxis-context-pack.json for immediate PRAXIS agent context and store the .capsule archive as the portable artifact. Cloud bridge: upload the archive to Cloud Storage, persist praxis-import.json in Firestore or PostgreSQL, then let PRAXIS fetch by capsule_id.".to_owned()
        }
        _ => format!("Unknown DIALECTICA resource: {uri}"),
    };
    json!({
        "contents": [{
            "uri": uri,
            "mimeType": "text/plain",
            "text": text
        }]
    })
}

fn read_prompt(params: &Value) -> Value {
    let name = params.get("name").and_then(Value::as_str).unwrap_or("");
    if name != "build_context_capsule" {
        return json!({
            "description": "Unknown prompt.",
            "messages": []
        });
    }
    json!({
        "description": "Guide Codex through a mediated DIALECTICA capsule build.",
        "messages": [{
            "role": "user",
            "content": {
                "type": "text",
                "text": "Use DIALECTICA to build a PRAXIS context capsule. First identify whether this is a user, situation, tool, or output capsule. Then call dialectica_build_capsule with the document folder and desired output folder. Inspect the package, review caveats, and return the .capsule path plus praxis-context-pack.json path."
            }
        }]
    })
}

fn tool_definitions() -> Vec<Value> {
    vec![
        json!({
            "name": "dialectica_welcome",
            "title": "DIALECTICA Welcome",
            "description": "Show the DIALECTICA by TACITUS operator welcome.",
            "inputSchema": { "type": "object", "properties": {}, "required": [] }
        }),
        json!({
            "name": "dialectica_build_capsule",
            "title": "Build PRAXIS Capsule",
            "description": "Build a local PRAXIS context capsule from text-like documents and emit package, .capsule archive, PRAXIS context pack, and import bridge receipt.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "capsule_type": { "type": "string", "enum": ["user", "situation", "tool", "output"] },
                    "input_dir": { "type": "string" },
                    "out_dir": { "type": "string" },
                    "title": { "type": "string" },
                    "workflow": { "type": "string" },
                    "mode": { "type": "string", "enum": ["auto-draft", "assisted", "plus-promoted"] }
                },
                "required": ["capsule_type", "input_dir", "out_dir"]
            }
        }),
        json!({
            "name": "dialectica_inspect_capsule",
            "title": "Inspect Capsule",
            "description": "Inspect a compiled v3 package and its embedded Ladybug projection metadata.",
            "inputSchema": {
                "type": "object",
                "properties": { "package_dir": { "type": "string" } },
                "required": ["package_dir"]
            }
        }),
        json!({
            "name": "dialectica_archive_capsule",
            "title": "Archive Capsule",
            "description": "Write a portable .capsule archive from a compiled package.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "package_dir": { "type": "string" },
                    "out_file": { "type": "string" }
                },
                "required": ["package_dir"]
            }
        }),
        json!({
            "name": "dialectica_export_praxis_pack",
            "title": "Export PRAXIS Context Pack",
            "description": "Export the PRAXIS-readable context pack from a compiled package.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "package_dir": { "type": "string" },
                    "workflow": { "type": "string" },
                    "out_file": { "type": "string" }
                },
                "required": ["package_dir"]
            }
        }),
        json!({
            "name": "dialectica_ontology_plan",
            "title": "Capsule Ontology Plan",
            "description": "Return the capsule-specific ontology blueprint from the compiled manifest.",
            "inputSchema": {
                "type": "object",
                "properties": { "package_dir": { "type": "string" } },
                "required": ["package_dir"]
            }
        }),
        json!({
            "name": "dialectica_mcp_config",
            "title": "Codex MCP Config",
            "description": "Return a local Codex MCP config snippet for this repository.",
            "inputSchema": { "type": "object", "properties": {}, "required": [] }
        }),
    ]
}

fn resource_definitions() -> Vec<Value> {
    vec![
        json!({
            "uri": "dialectica://welcome",
            "name": "welcome",
            "title": "Welcome to DIALECTICA",
            "description": "Operator welcome and capsule build orientation.",
            "mimeType": "text/plain"
        }),
        json!({
            "uri": "dialectica://builder/contract",
            "name": "builder-contract",
            "title": "Capsule Builder Contract",
            "description": "What the local builder emits into each capsule.",
            "mimeType": "text/plain"
        }),
        json!({
            "uri": "dialectica://praxis/bridge",
            "name": "praxis-bridge",
            "title": "PRAXIS Bridge",
            "description": "How PRAXIS consumes local and cloud capsule artifacts.",
            "mimeType": "text/plain"
        }),
    ]
}

fn prompt_definitions() -> Vec<Value> {
    vec![json!({
        "name": "build_context_capsule",
        "title": "Build Context Capsule",
        "description": "Mediated Codex workflow for building a DIALECTICA capsule for PRAXIS.",
        "arguments": [{
            "name": "capsule_goal",
            "description": "What the capsule should help PRAXIS understand or do.",
            "required": false
        }]
    })]
}

fn tool_json(value: Value, is_error: bool) -> Value {
    let text = serde_json::to_string_pretty(&value).unwrap_or_else(|error| {
        format!(
            "{{\"serialization_error\":\"{}\"}}",
            error.to_string().replace('"', "'")
        )
    });
    tool_text(text, is_error)
}

fn tool_text(text: String, is_error: bool) -> Value {
    json!({
        "content": [{ "type": "text", "text": text }],
        "isError": is_error
    })
}

fn required_string(arguments: &Value, key: &str) -> Result<String, String> {
    optional_string(arguments, key).ok_or_else(|| format!("missing required argument: {key}"))
}

fn optional_string(arguments: &Value, key: &str) -> Option<String> {
    arguments
        .get(key)
        .and_then(Value::as_str)
        .map(str::to_owned)
}

fn default_workflow(capsule_type: CapsuleType) -> String {
    match capsule_type {
        CapsuleType::User => "user_context",
        CapsuleType::Situation => "decision_brief",
        CapsuleType::Tool => "method_application",
        CapsuleType::Output => "output_review",
    }
    .to_owned()
}

fn mcp_config_snippet() -> String {
    let cwd = std::env::current_dir()
        .map(|path| path.display().to_string())
        .unwrap_or_else(|_| ".".to_owned())
        .replace('\\', "\\\\");
    format!(
        "[mcp_servers.dialectica]\ncommand = \"cargo\"\nargs = [\"run\", \"-p\", \"dialectica-mcp\", \"--\"]\ncwd = \"{cwd}\""
    )
}

fn write_json<T: Serialize>(path: &Path, value: &T) -> Result<(), String> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|error| error.to_string())?;
    }
    let text = serde_json::to_string_pretty(value).map_err(|error| error.to_string())?;
    fs::write(path, format!("{text}\n")).map_err(|error| error.to_string())
}

fn jsonrpc_result(id: Value, result: Value) -> Value {
    json!({
        "jsonrpc": "2.0",
        "id": id,
        "result": result
    })
}

fn jsonrpc_error(id: Value, code: i64, message: &str) -> Value {
    json!({
        "jsonrpc": "2.0",
        "id": id,
        "error": {
            "code": code,
            "message": message
        }
    })
}

#[cfg(test)]
mod tests {
    use serde_json::Value;

    use super::handle_jsonrpc_line;

    #[test]
    fn initialize_advertises_dialectica_tools() {
        let response = handle_jsonrpc_line(
            r#"{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":"2025-11-25"}}"#,
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
