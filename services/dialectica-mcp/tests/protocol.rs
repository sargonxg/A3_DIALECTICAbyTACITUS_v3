use std::{
    fs,
    path::PathBuf,
    time::{SystemTime, UNIX_EPOCH},
};

use serde_json::{json, Value};

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("repo root should canonicalize")
}

fn canonical_capsule_fixture() -> PathBuf {
    repo_root().join("fixtures/canonical-capsules/conflict-situation-capsule")
}

fn temp_case(name: &str) -> PathBuf {
    let stamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("clock should be after epoch")
        .as_nanos();
    std::env::temp_dir().join(format!(
        "dialectica-mcp-{name}-{}-{stamp}",
        std::process::id()
    ))
}

fn response(line: &str) -> Value {
    let text = dialectica_mcp::handle_jsonrpc_line(line)
        .expect("response should serialize")
        .expect("request should produce a response");
    serde_json::from_str(&text).expect("response should parse")
}

#[test]
fn initialize_accepts_supported_version() {
    let value = response(
        r#"{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":"2025-11-25","capabilities":{"roots":{"listChanged":true}},"clientInfo":{"name":"codex","version":"1"}}}"#,
    );

    assert_eq!(value["result"]["protocolVersion"], "2025-11-25");
    assert_eq!(
        value["result"]["capabilities"]["tools"]["listChanged"],
        false
    );
    assert_eq!(value["result"]["serverInfo"]["name"], "dialectica-mcp");
}

#[test]
fn initialize_rejects_unsupported_version() {
    let value = response(
        r#"{"jsonrpc":"2.0","id":"init-1","method":"initialize","params":{"protocolVersion":"2024-11-05","capabilities":{},"clientInfo":{"name":"legacy","version":"1"}}}"#,
    );

    assert_eq!(value["error"]["code"], -32602);
    assert_eq!(value["error"]["data"]["supported"], json!(["2025-11-25"]));
    assert_eq!(value["error"]["data"]["requested"], "2024-11-05");
}

#[test]
fn initialized_notification_produces_no_response() {
    let response = dialectica_mcp::handle_jsonrpc_line(
        r#"{"jsonrpc":"2.0","method":"notifications/initialized"}"#,
    )
    .expect("notification should parse");

    assert!(response.is_none());
}

#[test]
fn malformed_json_returns_parse_error() {
    let value = response(r#"{"jsonrpc":"2.0","id":1,"method":"tools/list""#);

    assert_eq!(value["id"], Value::Null);
    assert_eq!(value["error"]["code"], -32700);
}

#[test]
fn missing_params_returns_invalid_params() {
    let value = response(r#"{"jsonrpc":"2.0","id":7,"method":"tools/call"}"#);

    assert_eq!(value["error"]["code"], -32602);
}

#[test]
fn unknown_method_returns_method_not_found() {
    let value = response(r#"{"jsonrpc":"2.0","id":8,"method":"capsules/nope","params":{}}"#);

    assert_eq!(value["error"]["code"], -32601);
}

#[test]
fn tools_list_contains_stable_names_and_schemas() {
    let value = response(r#"{"jsonrpc":"2.0","id":2,"method":"tools/list","params":{}}"#);
    let tools = value["result"]["tools"].as_array().expect("tools array");
    let names = tools
        .iter()
        .map(|tool| tool["name"].as_str().expect("tool name"))
        .collect::<Vec<_>>();

    assert!(names.contains(&"dialectica_build_capsule"));
    assert!(names.contains(&"dialectica_validate_capsule"));
    assert!(names.contains(&"dialectica_capsule_status"));
    for tool in tools {
        assert_eq!(tool["inputSchema"]["type"], "object");
        assert_eq!(tool["outputSchema"]["type"], "object");
    }
}

#[test]
fn tool_call_returns_structured_content_and_text_json() {
    let value = response(
        r#"{"jsonrpc":"2.0","id":3,"method":"tools/call","params":{"name":"dialectica_welcome","arguments":{}}}"#,
    );

    assert_eq!(value["result"]["isError"], false);
    assert!(value["result"]["structuredContent"]["welcome"]
        .as_str()
        .expect("welcome text")
        .contains("DIALECTICA by TACITUS"));
    let text = value["result"]["content"][0]["text"]
        .as_str()
        .expect("text JSON");
    let parsed_text: Value = serde_json::from_str(text).expect("text should be JSON");
    assert_eq!(
        parsed_text["welcome"],
        value["result"]["structuredContent"]["welcome"]
    );
}

#[test]
fn invalid_tool_arguments_return_mcp_tool_error() {
    let value = response(
        r#"{"jsonrpc":"2.0","id":4,"method":"tools/call","params":{"name":"dialectica_build_capsule","arguments":{"capsule_type":"situation"}}}"#,
    );

    assert_eq!(value["result"]["isError"], true);
    assert!(value["result"]["content"][0]["text"]
        .as_str()
        .expect("tool error text")
        .contains("missing required argument"));
}

#[test]
fn validate_and_status_work_for_fixture_capsule() {
    let package_dir = canonical_capsule_fixture();
    let validate = response(
        &json!({
            "jsonrpc": "2.0",
            "id": 5,
            "method": "tools/call",
            "params": {
                "name": "dialectica_validate_capsule",
                "arguments": { "package_dir": package_dir }
            }
        })
        .to_string(),
    );

    assert_eq!(validate["result"]["isError"], false);
    assert_eq!(validate["result"]["structuredContent"]["valid"], true);

    let status = response(
        &json!({
            "jsonrpc": "2.0",
            "id": 6,
            "method": "tools/call",
            "params": {
                "name": "dialectica_capsule_status",
                "arguments": { "package_dir": canonical_capsule_fixture() }
            }
        })
        .to_string(),
    );

    assert_eq!(status["result"]["isError"], false);
    assert_eq!(
        status["result"]["structuredContent"]["manifest"]["capsule_type"],
        "situation"
    );
    assert_eq!(
        status["result"]["structuredContent"]["ladybug_projection"]["available"],
        true
    );
}

#[test]
fn build_capsule_smoke_works_through_mcp() {
    let root = temp_case("build-smoke");
    let input_dir = root.join("docs");
    let out_dir = root.join("out");
    fs::create_dir_all(&input_dir).expect("input dir should be created");
    fs::write(
        input_dir.join("brief.md"),
        "# Local situation\n\nA source-grounded tariff timetable needs review before PRAXIS use.\n",
    )
    .expect("source doc should be written");

    let value = response(
        &json!({
            "jsonrpc": "2.0",
            "id": 10,
            "method": "tools/call",
            "params": {
                "name": "dialectica_build_capsule",
                "arguments": {
                    "capsule_type": "situation",
                    "input_dir": input_dir,
                    "out_dir": out_dir,
                    "title": "MCP Situation Capsule",
                    "workflow": "decision_brief",
                    "mode": "assisted"
                }
            }
        })
        .to_string(),
    );

    assert_eq!(value["result"]["isError"], false);
    assert_eq!(
        value["result"]["structuredContent"]["capsule_type"],
        "situation"
    );
    let package_dir = value["result"]["structuredContent"]["package_dir"]
        .as_str()
        .expect("package path should be returned");
    assert!(PathBuf::from(package_dir).join("manifest.json").is_file());
    assert!(value["result"]["structuredContent"]["promotion_note"]
        .as_str()
        .expect("promotion note")
        .contains("Expert promotion"));

    let _ = fs::remove_dir_all(root);
}

#[test]
fn multi_message_stdio_flow_preserves_response_order_and_skips_notifications() {
    let output = dialectica_mcp::handle_jsonrpc_lines(
        r#"{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":"2025-11-25","capabilities":{},"clientInfo":{"name":"codex","version":"1"}}}
{"jsonrpc":"2.0","method":"notifications/initialized"}
{"jsonrpc":"2.0","id":2,"method":"tools/list","params":{}}
"#,
    )
    .expect("stdio input should serialize");
    let lines = output.lines().collect::<Vec<_>>();

    assert_eq!(lines.len(), 2);
    assert_eq!(
        serde_json::from_str::<Value>(lines[0]).expect("first response should parse")["id"],
        1
    );
    assert_eq!(
        serde_json::from_str::<Value>(lines[1]).expect("second response should parse")["id"],
        2
    );
}

#[test]
fn archive_output_inside_package_returns_tool_error() {
    let package_dir = canonical_capsule_fixture();
    let out_file = package_dir.join("bad.capsule");
    let value = response(
        &json!({
            "jsonrpc": "2.0",
            "id": 9,
            "method": "tools/call",
            "params": {
                "name": "dialectica_archive_capsule",
                "arguments": {
                    "package_dir": package_dir,
                    "out_file": out_file
                }
            }
        })
        .to_string(),
    );

    assert_eq!(value["result"]["isError"], true);
    assert!(!out_file.exists());
    let _ = fs::remove_file(out_file);
}
