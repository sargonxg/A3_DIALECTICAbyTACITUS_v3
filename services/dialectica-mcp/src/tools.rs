use std::{
    fs,
    path::{Component, Path, PathBuf},
};

use dialectica_capsule::{PraxisCapsulePackage, ValidationSeverity};
use dialectica_extractor::CapsuleType;
use serde::Serialize;
use serde_json::{json, Value};
use sha2::{Digest, Sha256};

use crate::WELCOME;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum PathKind {
    ExistingDir,
    ExistingFile,
    OutputDir,
    OutputFile,
}

pub fn call_tool(name: &str, arguments: &Value) -> Value {
    let result = match name {
        "dialectica_welcome" => Ok(json!({ "welcome": WELCOME })),
        "dialectica_build_capsule" => build_capsule_tool(arguments),
        "dialectica_capture_discussion" => capture_discussion_tool(arguments),
        "dialectica_inspect_capsule" => inspect_capsule_tool(arguments),
        "dialectica_validate_capsule" => validate_capsule_tool(arguments),
        "dialectica_capsule_status" => capsule_status_tool(arguments),
        "dialectica_review_queue" => review_queue_tool(arguments),
        "dialectica_archive_capsule" => archive_capsule_tool(arguments),
        "dialectica_export_praxis_pack" => export_praxis_pack_tool(arguments),
        "dialectica_ontology_plan" => ontology_plan_tool(arguments),
        "dialectica_ladybug_query" => ladybug_query_tool(arguments),
        "dialectica_praxis_handoff" => praxis_handoff_tool(arguments),
        "dialectica_mcp_config" => Ok(json!({ "config": mcp_config_snippet() })),
        _ => Err(format!("unknown tool: {name}")),
    };

    match result {
        Ok(value) => tool_json(value, false),
        Err(error) => tool_error(error),
    }
}

fn build_capsule_tool(arguments: &Value) -> Result<Value, String> {
    let input_dir = required_path(arguments, "input_dir", PathKind::ExistingDir)?;
    let out_dir = required_path(arguments, "out_dir", PathKind::OutputDir)?;
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
            input_dir,
            output_dir: out_dir,
            capsule_type,
            build_mode,
            title,
            workflow,
        })
        .map_err(|error| error.to_string())?;
    let mut value = serde_json::to_value(receipt).map_err(|error| error.to_string())?;
    value["promotion_note"] = json!(
        "Local MCP builds create draft or assisted capsule artifacts. Expert promotion remains an explicit review decision outside this tool."
    );
    Ok(value)
}

fn capture_discussion_tool(arguments: &Value) -> Result<Value, String> {
    let out_file = required_path(arguments, "out_file", PathKind::OutputFile)?;
    if out_file.extension().and_then(|value| value.to_str()) != Some("jsonl") {
        return Err("discussion capture output must use the .jsonl extension".to_owned());
    }
    let turns = arguments
        .get("turns")
        .and_then(Value::as_array)
        .ok_or_else(|| "missing required argument: turns".to_owned())?;
    if turns.is_empty() {
        return Err("turns must contain at least one discussion turn".to_owned());
    }

    let mut text = String::new();
    for (index, turn) in turns.iter().enumerate() {
        let role = turn
            .get("role")
            .and_then(Value::as_str)
            .ok_or_else(|| format!("turns[{index}].role is required"))?;
        let content = turn
            .get("content")
            .and_then(Value::as_str)
            .ok_or_else(|| format!("turns[{index}].content is required"))?;
        if role.trim().is_empty() || content.trim().is_empty() {
            return Err(format!("turns[{index}] role and content must not be empty"));
        }
        let timestamp = turn
            .get("timestamp")
            .and_then(Value::as_str)
            .unwrap_or("unspecified");
        let record = json!({
            "role": role,
            "timestamp": timestamp,
            "content": content
        });
        text.push_str(&serde_json::to_string(&record).map_err(|error| error.to_string())?);
        text.push('\n');
    }
    if let Some(parent) = out_file.parent() {
        fs::create_dir_all(parent).map_err(|error| error.to_string())?;
    }
    fs::write(&out_file, text).map_err(|error| error.to_string())?;
    Ok(json!({
        "discussion_file": out_file,
        "turn_count": turns.len(),
        "source_type": "conversation_jsonl",
        "next_step": "Use dialectica_build_capsule with the parent directory as input_dir."
    }))
}

fn inspect_capsule_tool(arguments: &Value) -> Result<Value, String> {
    let package_dir = required_path(arguments, "package_dir", PathKind::ExistingDir)?;
    let package =
        PraxisCapsulePackage::load_from_dir(&package_dir).map_err(|error| error.to_string())?;
    let inspection = package.inspection();
    let validation = package.validate();

    let manifest = manifest_with_capsule_type_alias(&package.manifest)?;
    Ok(json!({
        "package_dir": package_dir,
        "capsule_id": inspection.capsule_id,
        "capsule_type": inspection.capsule_type,
        "review_state": format!("{:?}", inspection.review_state).to_lowercase(),
        "source_count": inspection.source_count,
        "claim_count": inspection.claim_count,
        "graph_node_count": inspection.graph_node_count,
        "graph_edge_count": inspection.graph_edge_count,
        "warning_count": inspection.warning_count,
        "valid": !validation.has_errors(),
        "manifest": manifest,
        "ladybug_projection": ladybug_status(&package_dir)
    }))
}

fn validate_capsule_tool(arguments: &Value) -> Result<Value, String> {
    let package_dir = required_path(arguments, "package_dir", PathKind::ExistingDir)?;
    let package =
        PraxisCapsulePackage::load_from_dir(&package_dir).map_err(|error| error.to_string())?;
    let report = package.validate();
    let error_count = report
        .findings
        .iter()
        .filter(|finding| finding.severity == ValidationSeverity::Error)
        .count();
    let warning_count = report
        .findings
        .iter()
        .filter(|finding| finding.severity == ValidationSeverity::Warning)
        .count();
    let info_count = report
        .findings
        .iter()
        .filter(|finding| finding.severity == ValidationSeverity::Info)
        .count();

    Ok(json!({
        "package_dir": package_dir,
        "capsule_id": package.manifest.capsule_id,
        "valid": error_count == 0,
        "error_count": error_count,
        "warning_count": warning_count,
        "info_count": info_count,
        "findings": report.findings
    }))
}

fn capsule_status_tool(arguments: &Value) -> Result<Value, String> {
    let package_dir = required_path(arguments, "package_dir", PathKind::ExistingDir)?;
    let package =
        PraxisCapsulePackage::load_from_dir(&package_dir).map_err(|error| error.to_string())?;
    let validation = package.validate();
    let inspection = package.inspection();
    let archive_path = optional_path(arguments, "archive_file", PathKind::OutputFile)?
        .or_else(|| find_archive_candidate(&package_dir));
    let praxis_pack_path = optional_path(arguments, "praxis_pack_file", PathKind::OutputFile)?
        .or_else(|| {
            package_dir
                .parent()
                .map(|parent| parent.join("praxis-context-pack.json"))
        });

    let manifest = manifest_with_capsule_type_alias(&package.manifest)?;
    Ok(json!({
        "package_dir": package_dir,
        "capsule_id": inspection.capsule_id,
        "manifest": manifest,
        "review_state": format!("{:?}", inspection.review_state).to_lowercase(),
        "validation": {
            "valid": !validation.has_errors(),
            "finding_count": validation.findings.len()
        },
        "ladybug_projection": ladybug_status(&package_dir),
        "archive": file_status(archive_path.as_deref()),
        "praxis_context_pack": file_status(praxis_pack_path.as_deref()),
        "hosted_mode": {
            "ready": false,
            "planned_endpoint": "/mcp",
            "artifact_addressing": "build_id, capsule_id, or artifact_id only; no raw local filesystem paths",
            "auth_required": true
        }
    }))
}

fn review_queue_tool(arguments: &Value) -> Result<Value, String> {
    let review_queue_file =
        match optional_path(arguments, "review_queue_file", PathKind::ExistingFile)? {
            Some(path) => path,
            None => {
                let build_source_dir =
                    required_path(arguments, "build_source_dir", PathKind::ExistingDir)?;
                resolve_existing_file(&build_source_dir.join("review_queue.json"))?
            }
        };
    read_json_file(&review_queue_file)
}

fn archive_capsule_tool(arguments: &Value) -> Result<Value, String> {
    let package_dir = required_path(arguments, "package_dir", PathKind::ExistingDir)?;
    let output_file = optional_path(arguments, "out_file", PathKind::OutputFile)?
        .unwrap_or_else(|| package_dir.with_extension("capsule"));
    if output_file.extension().and_then(|value| value.to_str()) != Some("capsule") {
        return Err("archive output must use the .capsule extension".to_owned());
    }
    let receipt = dialectica_compiler::write_capsule_archive(&package_dir, &output_file)
        .map_err(|error| error.to_string())?;
    serde_json::to_value(receipt).map_err(|error| error.to_string())
}

fn export_praxis_pack_tool(arguments: &Value) -> Result<Value, String> {
    let package_dir = required_path(arguments, "package_dir", PathKind::ExistingDir)?;
    let workflow =
        optional_string(arguments, "workflow").unwrap_or_else(|| "decision_brief".to_owned());
    let pack = dialectica_compiler::export_praxis_context_pack(&package_dir, &workflow)
        .map_err(|error| error.to_string())?;
    if let Some(out_path) = optional_path(arguments, "out_file", PathKind::OutputFile)? {
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
    let package_dir = required_path(arguments, "package_dir", PathKind::ExistingDir)?;
    let package =
        PraxisCapsulePackage::load_from_dir(&package_dir).map_err(|error| error.to_string())?;
    serde_json::to_value(package.manifest.ontology_blueprint()).map_err(|error| error.to_string())
}

fn ladybug_query_tool(arguments: &Value) -> Result<Value, String> {
    let package_dir = required_path(arguments, "package_dir", PathKind::ExistingDir)?;
    let query = required_string(arguments, "query")?;
    let trimmed = query.trim();
    validate_read_only_ladybug_query(trimmed)?;
    let output = dialectica_graph::query_ladybug_projection(&package_dir, trimmed)
        .map_err(|error| error.to_string())?;
    let row_count = output.rows.len();
    Ok(json!({
        "package_dir": package_dir,
        "query": trimmed,
        "columns": output.columns,
        "rows": output.rows,
        "row_count": row_count
    }))
}

fn praxis_handoff_tool(arguments: &Value) -> Result<Value, String> {
    let import_file = match optional_path(arguments, "import_file", PathKind::ExistingFile)? {
        Some(path) => path,
        None => {
            let package_dir = required_path(arguments, "package_dir", PathKind::ExistingDir)?;
            package_dir
                .parent()
                .ok_or_else(|| {
                    "package_dir has no parent for praxis-import.json discovery".to_owned()
                })?
                .join("praxis-import.json")
        }
    };
    let import_file = resolve_existing_file(&import_file)?;
    let mut value = read_json_file(&import_file)?;
    value["handoff_note"] = json!(
        "Attach praxis-context-pack.json to PRAXIS agent context and keep the .capsule archive as the portable source-of-truth artifact."
    );
    Ok(value)
}

pub fn read_resource(params: &Value) -> Value {
    let uri = params.get("uri").and_then(Value::as_str).unwrap_or("");
    let text = match uri {
        "dialectica://welcome" => WELCOME.to_owned(),
        "dialectica://builder/contract" => {
            "DIALECTICA builds four capsule classes: user, situation, tool, output. Each capsule contains source receipts, claims, temporal status, ontology terms, graph nodes/edges, reasoning devices, caveats, language rules, rights rules, output contracts, embedded Ladybug projection metadata, and a PRAXIS context pack.".to_owned()
        }
        "dialectica://praxis/bridge" => {
            "Local bridge: use praxis-context-pack.json for immediate PRAXIS agent context and store the .capsule archive as the portable artifact. Cloud bridge: upload the archive to Cloud Storage, persist state in Cloud SQL PostgreSQL, and let PRAXIS fetch by authenticated capsule_id or signed artifact URL.".to_owned()
        }
        "dialectica://hosted/mcp" => {
            "Future hosted MCP runs at /mcp over Streamable HTTP, requires authentication, validates tenant ownership and token audience, and accepts build_id, capsule_id, or artifact_id instead of raw filesystem paths.".to_owned()
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

pub fn read_prompt(params: &Value) -> Value {
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
                "text": "Use DIALECTICA to build a PRAXIS context capsule. First identify whether this is a user, situation, tool, or output capsule. Then call dialectica_build_capsule with the document folder and desired output folder. Inspect and validate the package, check capsule status, review caveats, and return the .capsule path plus praxis-context-pack.json path."
            }
        }]
    })
}

pub fn tool_definitions() -> Vec<Value> {
    vec![
        json!({
            "name": "dialectica_welcome",
            "title": "DIALECTICA Welcome",
            "description": "Show the DIALECTICA by TACITUS operator welcome.",
            "inputSchema": empty_input_schema(),
            "outputSchema": object_schema([("welcome", string_schema("Operator welcome text."))], ["welcome"])
        }),
        json!({
            "name": "dialectica_build_capsule",
            "title": "Build PRAXIS Capsule",
            "description": "Build a local PRAXIS context capsule from text-like documents and emit package, .capsule archive, PRAXIS context pack, and import bridge receipt. This creates draft or assisted outputs; expert promotion stays behind review logic.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "capsule_type": { "type": "string", "enum": ["user", "situation", "tool", "output"] },
                    "input_dir": { "type": "string", "description": "Existing local source directory. Relative paths resolve from the MCP process cwd." },
                    "out_dir": { "type": "string", "description": "Local output directory. Root paths and parent traversal are rejected." },
                    "title": { "type": "string" },
                    "workflow": { "type": "string" },
                    "mode": { "type": "string", "enum": ["auto-draft", "assisted", "plus-promoted"] }
                },
                "required": ["capsule_type", "input_dir", "out_dir"],
                "additionalProperties": false
            },
            "outputSchema": loose_object_schema()
        }),
        json!({
            "name": "dialectica_capture_discussion",
            "title": "Capture Discussion",
            "description": "Write a local user/assistant discussion transcript as JSONL so it can be ingested as capsule source material.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "out_file": { "type": "string", "description": "Local .jsonl file to write." },
                    "turns": {
                        "type": "array",
                        "items": {
                            "type": "object",
                            "properties": {
                                "role": { "type": "string" },
                                "timestamp": { "type": "string" },
                                "content": { "type": "string" }
                            },
                            "required": ["role", "content"],
                            "additionalProperties": false
                        }
                    }
                },
                "required": ["out_file", "turns"],
                "additionalProperties": false
            },
            "outputSchema": loose_object_schema()
        }),
        json!({
            "name": "dialectica_inspect_capsule",
            "title": "Inspect Capsule",
            "description": "Inspect a compiled v3 package and its embedded Ladybug projection metadata.",
            "inputSchema": package_dir_input_schema(),
            "outputSchema": loose_object_schema()
        }),
        json!({
            "name": "dialectica_validate_capsule",
            "title": "Validate Capsule",
            "description": "Validate a compiled v3 package and return precise findings without crashing the MCP process.",
            "inputSchema": package_dir_input_schema(),
            "outputSchema": object_schema([
                ("package_dir", string_schema("Normalized package directory.")),
                ("capsule_id", string_schema("Capsule identifier.")),
                ("valid", boolean_schema("True when no error-severity findings exist.")),
                ("error_count", integer_schema("Error finding count.")),
                ("warning_count", integer_schema("Warning finding count.")),
                ("info_count", integer_schema("Info finding count.")),
                ("findings", array_schema("Validation findings."))
            ], ["package_dir", "capsule_id", "valid", "error_count", "warning_count", "info_count", "findings"])
        }),
        json!({
            "name": "dialectica_capsule_status",
            "title": "Capsule Status",
            "description": "Return manifest, review state, Ladybug projection status, archive status, PRAXIS pack availability, and hosted MCP readiness notes.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "package_dir": { "type": "string" },
                    "archive_file": { "type": "string" },
                    "praxis_pack_file": { "type": "string" }
                },
                "required": ["package_dir"],
                "additionalProperties": false
            },
            "outputSchema": loose_object_schema()
        }),
        json!({
            "name": "dialectica_review_queue",
            "title": "Read Review Queue",
            "description": "Read a local build review_queue.json artifact so Codex can show required human gates before PRAXIS use.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "review_queue_file": { "type": "string" },
                    "build_source_dir": { "type": "string" }
                },
                "additionalProperties": false
            },
            "outputSchema": loose_object_schema()
        }),
        json!({
            "name": "dialectica_archive_capsule",
            "title": "Archive Capsule",
            "description": "Write a portable .capsule archive from a compiled package. The output must be outside the package directory.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "package_dir": { "type": "string" },
                    "out_file": { "type": "string" }
                },
                "required": ["package_dir"],
                "additionalProperties": false
            },
            "outputSchema": loose_object_schema()
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
                "required": ["package_dir"],
                "additionalProperties": false
            },
            "outputSchema": loose_object_schema()
        }),
        json!({
            "name": "dialectica_ontology_plan",
            "title": "Capsule Ontology Plan",
            "description": "Return the capsule-specific ontology blueprint from the compiled manifest.",
            "inputSchema": package_dir_input_schema(),
            "outputSchema": loose_object_schema()
        }),
        json!({
            "name": "dialectica_ladybug_query",
            "title": "Query Ladybug Projection",
            "description": "Run a read-only Cypher query against the embedded Ladybug capsule projection when the Ladybug feature is available.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "package_dir": { "type": "string" },
                    "query": { "type": "string" }
                },
                "required": ["package_dir", "query"],
                "additionalProperties": false
            },
            "outputSchema": loose_object_schema()
        }),
        json!({
            "name": "dialectica_praxis_handoff",
            "title": "PRAXIS Handoff",
            "description": "Read the local praxis-import.json bridge receipt and return the paths/status needed to attach the capsule to PRAXIS agent context.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "import_file": { "type": "string" },
                    "package_dir": { "type": "string" }
                },
                "additionalProperties": false
            },
            "outputSchema": loose_object_schema()
        }),
        json!({
            "name": "dialectica_mcp_config",
            "title": "Codex MCP Config",
            "description": "Return a local Codex MCP config snippet for this repository.",
            "inputSchema": empty_input_schema(),
            "outputSchema": object_schema([("config", string_schema("TOML snippet."))], ["config"])
        }),
    ]
}

pub fn resource_definitions() -> Vec<Value> {
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
            "title": "How PRAXIS consumes local and cloud capsule artifacts.",
            "description": "How PRAXIS consumes local and cloud capsule artifacts.",
            "mimeType": "text/plain"
        }),
        json!({
            "uri": "dialectica://hosted/mcp",
            "name": "hosted-mcp",
            "title": "Hosted MCP Design Boundary",
            "description": "Future Streamable HTTP /mcp behavior and auth/path restrictions.",
            "mimeType": "text/plain"
        }),
    ]
}

pub fn prompt_definitions() -> Vec<Value> {
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
    json!({
        "content": [{ "type": "text", "text": text }],
        "structuredContent": value,
        "isError": is_error
    })
}

fn tool_error(error: String) -> Value {
    json!({
        "content": [{ "type": "text", "text": error }],
        "isError": true
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

fn required_path(arguments: &Value, key: &str, kind: PathKind) -> Result<PathBuf, String> {
    let raw = required_string(arguments, key)?;
    normalize_local_path(&raw, kind)
}

fn optional_path(arguments: &Value, key: &str, kind: PathKind) -> Result<Option<PathBuf>, String> {
    optional_string(arguments, key)
        .map(|value| normalize_local_path(&value, kind))
        .transpose()
}

fn normalize_local_path(raw: &str, kind: PathKind) -> Result<PathBuf, String> {
    let trimmed = raw.trim();
    if trimmed.is_empty() {
        return Err("path must not be empty".to_owned());
    }
    let path = PathBuf::from(trimmed);
    if path
        .components()
        .any(|component| component == Component::ParentDir)
    {
        return Err(format!("path must not contain parent traversal: {trimmed}"));
    }
    let absolute = if path.is_absolute() {
        path
    } else {
        std::env::current_dir()
            .map_err(|error| error.to_string())?
            .join(path)
    };

    match kind {
        PathKind::ExistingDir => {
            let canonical = fs::canonicalize(&absolute).map_err(|error| {
                format!(
                    "failed to resolve directory {}: {error}",
                    absolute.display()
                )
            })?;
            if !canonical.is_dir() {
                return Err(format!("path is not a directory: {}", canonical.display()));
            }
            ensure_within_configured_roots(&canonical)?;
            Ok(canonical)
        }
        PathKind::ExistingFile => resolve_existing_file(&absolute),
        PathKind::OutputDir | PathKind::OutputFile => {
            reject_dangerous_output_target(&absolute)?;
            ensure_output_within_configured_roots(&absolute)?;
            Ok(absolute)
        }
    }
}

fn resolve_existing_file(path: &Path) -> Result<PathBuf, String> {
    let canonical = fs::canonicalize(path)
        .map_err(|error| format!("failed to resolve file {}: {error}", path.display()))?;
    if !canonical.is_file() {
        return Err(format!("path is not a file: {}", canonical.display()));
    }
    ensure_within_configured_roots(&canonical)?;
    Ok(canonical)
}

fn validate_read_only_ladybug_query(query: &str) -> Result<(), String> {
    if query.is_empty() {
        return Err("Ladybug MCP query must not be empty".to_owned());
    }
    if query.contains(';') {
        return Err("Ladybug MCP query must be one read-only statement".to_owned());
    }

    let lower = query.to_ascii_lowercase();
    let tokens = lower
        .split(|character: char| !(character.is_ascii_alphanumeric() || character == '_'))
        .filter(|token| !token.is_empty())
        .collect::<Vec<_>>();
    let Some(first) = tokens.first() else {
        return Err("Ladybug MCP query must not be empty".to_owned());
    };
    if !matches!(*first, "match" | "return") {
        return Err(
            "Ladybug MCP query must be read-only and start with MATCH or RETURN".to_owned(),
        );
    }

    const MUTATING_KEYWORDS: &[&str] = &[
        "call", "copy", "create", "delete", "detach", "drop", "load", "merge", "remove", "set",
    ];
    if let Some(keyword) = tokens
        .iter()
        .copied()
        .find(|token| MUTATING_KEYWORDS.contains(token))
    {
        return Err(format!(
            "Ladybug MCP query must be read-only; rejected Cypher keyword `{keyword}`"
        ));
    }

    Ok(())
}

fn reject_dangerous_output_target(path: &Path) -> Result<(), String> {
    if path.file_name().is_none() {
        return Err(format!(
            "refusing to write to filesystem root: {}",
            path.display()
        ));
    }
    let cwd = std::env::current_dir().map_err(|error| error.to_string())?;
    if path == cwd {
        return Err(format!(
            "refusing to use current working directory as an output target: {}",
            path.display()
        ));
    }
    Ok(())
}

fn ensure_within_configured_roots(path: &Path) -> Result<(), String> {
    let roots = configured_roots()?;
    if roots.is_empty() || roots.iter().any(|root| path.starts_with(root)) {
        return Ok(());
    }
    Err(format!(
        "path {} is outside DIALECTICA_MCP_ROOTS",
        path.display()
    ))
}

fn ensure_output_within_configured_roots(path: &Path) -> Result<(), String> {
    let roots = configured_roots()?;
    if roots.is_empty() {
        return Ok(());
    }
    let parent = path.parent().unwrap_or_else(|| Path::new("."));
    let canonical_parent = nearest_existing_ancestor(parent)?;
    if roots.iter().any(|root| canonical_parent.starts_with(root)) {
        Ok(())
    } else {
        Err(format!(
            "output path {} is outside DIALECTICA_MCP_ROOTS",
            path.display()
        ))
    }
}

fn configured_roots() -> Result<Vec<PathBuf>, String> {
    let Some(raw) = std::env::var_os("DIALECTICA_MCP_ROOTS") else {
        return Ok(Vec::new());
    };
    raw.to_string_lossy()
        .split(';')
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(|value| fs::canonicalize(value).map_err(|error| error.to_string()))
        .collect()
}

fn nearest_existing_ancestor(path: &Path) -> Result<PathBuf, String> {
    let mut current = path;
    loop {
        if current.exists() {
            return fs::canonicalize(current).map_err(|error| error.to_string());
        }
        current = current.parent().ok_or_else(|| {
            format!(
                "could not find existing ancestor for output path {}",
                path.display()
            )
        })?;
    }
}

fn ladybug_status(package_dir: &Path) -> Value {
    match dialectica_graph::check_ladybug_projection(package_dir) {
        Ok(manifest) => json!({
            "available": true,
            "manifest": manifest
        }),
        Err(error) => json!({
            "available": false,
            "error": error.to_string()
        }),
    }
}

fn manifest_with_capsule_type_alias<T: Serialize>(manifest: &T) -> Result<Value, String> {
    let mut value = serde_json::to_value(manifest).map_err(|error| error.to_string())?;
    if value.get("capsule_type").is_none() {
        value["capsule_type"] = value.get("type").cloned().unwrap_or(Value::Null);
    }
    Ok(value)
}

fn file_status(path: Option<&Path>) -> Value {
    let Some(path) = path else {
        return json!({ "available": false, "path": Value::Null });
    };
    match fs::metadata(path) {
        Ok(metadata) if metadata.is_file() => json!({
            "available": true,
            "path": path,
            "bytes": metadata.len(),
            "digest": digest_file(path).ok()
        }),
        Ok(_) => json!({
            "available": false,
            "path": path,
            "error": "path exists but is not a file"
        }),
        Err(_) => json!({
            "available": false,
            "path": path
        }),
    }
}

fn find_archive_candidate(package_dir: &Path) -> Option<PathBuf> {
    let parent = package_dir.parent()?;
    let mut candidates = fs::read_dir(parent)
        .ok()?
        .filter_map(Result::ok)
        .map(|entry| entry.path())
        .filter(|path| path.extension().and_then(|value| value.to_str()) == Some("capsule"))
        .collect::<Vec<_>>();
    candidates.sort();
    candidates.into_iter().next()
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

fn read_json_file(path: &Path) -> Result<Value, String> {
    let text = fs::read_to_string(path).map_err(|error| error.to_string())?;
    serde_json::from_str(&text).map_err(|error| error.to_string())
}

fn digest_file(path: &Path) -> Result<String, String> {
    let bytes = fs::read(path).map_err(|error| error.to_string())?;
    let mut hasher = Sha256::new();
    hasher.update(bytes);
    Ok(format!("sha256:{:x}", hasher.finalize()))
}

fn empty_input_schema() -> Value {
    json!({
        "type": "object",
        "properties": {},
        "required": [],
        "additionalProperties": false
    })
}

fn package_dir_input_schema() -> Value {
    json!({
        "type": "object",
        "properties": {
            "package_dir": { "type": "string" }
        },
        "required": ["package_dir"],
        "additionalProperties": false
    })
}

fn loose_object_schema() -> Value {
    json!({
        "type": "object",
        "additionalProperties": true
    })
}

fn object_schema<const P: usize, const R: usize>(
    properties: [(&str, Value); P],
    required: [&str; R],
) -> Value {
    let properties = properties
        .into_iter()
        .map(|(key, value)| (key.to_owned(), value))
        .collect::<serde_json::Map<_, _>>();
    let required = required.to_vec();
    json!({
        "type": "object",
        "properties": properties,
        "required": required,
        "additionalProperties": true
    })
}

fn string_schema(description: &str) -> Value {
    json!({ "type": "string", "description": description })
}

fn boolean_schema(description: &str) -> Value {
    json!({ "type": "boolean", "description": description })
}

fn integer_schema(description: &str) -> Value {
    json!({ "type": "integer", "minimum": 0, "description": description })
}

fn array_schema(description: &str) -> Value {
    json!({ "type": "array", "description": description })
}
