//! Local fixture-backed DIALECTICA API.

use std::{
    collections::BTreeSet,
    path::{Path, PathBuf},
    sync::Arc,
};

use axum::{
    body::Body,
    extract::{Path as AxumPath, Query, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::{get, post},
    Json, Router,
};
use dialectica_capsule::{PraxisCapsulePackage, ValidationSeverity, CAPSULE_SPEC_VERSION};
use dialectica_compiler::{export_praxis_context_pack, PraxisContextPack};
use dialectica_extractor::{
    load_elicitation_protocol, score_elicitation_session, CapsuleType,
    ElicitationCompletenessScore, ElicitationProtocol, ElicitationSession,
};
use serde::Deserialize;
use serde_json::{json, Value};

#[derive(Debug, Clone)]
pub struct ApiState {
    package_dir: Arc<PathBuf>,
    protocol_dir: Arc<PathBuf>,
}

impl ApiState {
    pub fn new(package_dir: impl Into<PathBuf>) -> Self {
        Self {
            package_dir: Arc::new(package_dir.into()),
            protocol_dir: Arc::new(default_protocol_dir()),
        }
    }

    pub fn with_protocol_dir(
        package_dir: impl Into<PathBuf>,
        protocol_dir: impl Into<PathBuf>,
    ) -> Self {
        Self {
            package_dir: Arc::new(package_dir.into()),
            protocol_dir: Arc::new(protocol_dir.into()),
        }
    }

    pub fn default_fixture() -> Self {
        Self::new(default_fixture_dir())
    }
}

pub fn app(state: ApiState) -> Router {
    Router::new()
        .route("/health", get(health))
        .route("/version", get(version))
        .route("/v1/capsules/{capsule_id}/manifest", get(manifest))
        .route(
            "/v1/capsules/{capsule_id}/graph-preview",
            get(graph_preview),
        )
        .route(
            "/v1/capsules/{capsule_id}/praxis-context-pack",
            get(context_pack),
        )
        .route(
            "/v1/capsules/{capsule_id}/read-receipts",
            post(record_read_receipt),
        )
        .route("/v1/protocols/{capsule_type}", get(elicitation_protocol))
        .route(
            "/v1/protocols/{capsule_type}/score",
            post(score_elicitation_protocol),
        )
        .with_state(state)
}

pub fn default_fixture_dir() -> PathBuf {
    std::env::var("DIALECTICA_FIXTURE_CAPSULE_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|_| {
            Path::new(env!("CARGO_MANIFEST_DIR"))
                .join("../..")
                .join("fixtures")
                .join("canonical-capsules")
                .join("conflict-situation-capsule")
        })
}

pub fn default_protocol_dir() -> PathBuf {
    std::env::var("DIALECTICA_PROTOCOL_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|_| {
            Path::new(env!("CARGO_MANIFEST_DIR"))
                .join("../..")
                .join("fixtures")
                .join("elicitation-protocols")
        })
}

async fn health() -> Json<Value> {
    Json(json!({
        "status": "ok",
        "service": "dialectica-api",
        "version": env!("CARGO_PKG_VERSION"),
        "mode": "fixture",
        "dependencies": {
            "postgres": "not_configured",
            "cloud_storage": "not_configured",
            "task_queue": "not_configured"
        }
    }))
}

async fn version() -> Json<Value> {
    Json(json!({
        "service": "dialectica-api",
        "version": env!("CARGO_PKG_VERSION"),
        "capsule_spec_version": CAPSULE_SPEC_VERSION,
        "graph_preview_schema_version": "graph_preview_v1",
        "commit": option_env!("GITHUB_SHA").unwrap_or("local")
    }))
}

async fn elicitation_protocol(
    State(state): State<ApiState>,
    AxumPath(capsule_type): AxumPath<String>,
) -> Result<Json<ElicitationProtocol>, ApiError> {
    let requested_type = parse_capsule_type(&capsule_type)?;
    let protocol = load_protocol(&state, requested_type)?;
    Ok(Json(protocol))
}

async fn score_elicitation_protocol(
    State(state): State<ApiState>,
    AxumPath(capsule_type): AxumPath<String>,
    Json(session): Json<ElicitationSession>,
) -> Result<Json<ElicitationCompletenessScore>, ApiError> {
    let requested_type = parse_capsule_type(&capsule_type)?;
    let protocol = load_protocol(&state, requested_type)?;
    if session.capsule_type != requested_type {
        return Err(ApiError::bad_request(
            "session capsule_type must match the protocol route",
        ));
    }
    Ok(Json(score_elicitation_session(&protocol, &session)))
}

async fn manifest(
    State(state): State<ApiState>,
    AxumPath(capsule_id): AxumPath<String>,
) -> Result<Json<Value>, ApiError> {
    let package = load_package(&state, &capsule_id)?;
    let report = package.validate();
    let source_count = count_jsonl(&state.package_dir.join("evidence").join("sources.jsonl"))?;

    Ok(Json(json!({
        "capsule_id": package.manifest.capsule_id,
        "spec_version": package.manifest.spec_version,
        "title": package.manifest.title,
        "type": package.manifest.capsule_type,
        "category": package.manifest.category,
        "cores": package.manifest.cores,
        "status": "ready",
        "review_state": "approved_with_caveats",
        "freshness": "fixture",
        "source_count": source_count,
        "graph_profile": "situation_graph_v1",
        "provenance_root_hash": package.manifest.provenance_root_hash,
        "signature": package.manifest.signature,
        "compiled_at": package.manifest.updated_at,
        "compatible_workflows": ["decision_brief", "stakeholder_map", "conflict_map"],
        "warnings": report.findings.iter()
            .filter(|finding| finding.severity == ValidationSeverity::Warning)
            .map(|finding| finding.message.clone())
            .collect::<Vec<_>>()
    })))
}

async fn graph_preview(
    State(state): State<ApiState>,
    AxumPath(capsule_id): AxumPath<String>,
) -> Result<Json<Value>, ApiError> {
    let package = load_package(&state, &capsule_id)?;
    let graph = read_json(&state.package_dir.join("graph.jsonld"))?;
    Ok(Json(build_graph_preview(&package, &graph)))
}

async fn context_pack(
    State(state): State<ApiState>,
    AxumPath(capsule_id): AxumPath<String>,
    Query(query): Query<ContextPackQuery>,
) -> Result<Json<PraxisContextPack>, ApiError> {
    let _package = load_package(&state, &capsule_id)?;
    let workflow = query
        .workflow
        .unwrap_or_else(|| "decision_brief".to_owned());
    let pack = export_praxis_context_pack(&state.package_dir, &workflow)
        .map_err(|error| ApiError::internal(error.to_string()))?;
    Ok(Json(pack))
}

async fn record_read_receipt(
    State(state): State<ApiState>,
    AxumPath(capsule_id): AxumPath<String>,
    Json(request): Json<ReadReceiptRequest>,
) -> Result<Json<Value>, ApiError> {
    let _package = load_package(&state, &capsule_id)?;
    if request.praxis_run_id.trim().is_empty() || request.workflow.trim().is_empty() {
        return Err(ApiError::bad_request(
            "praxis_run_id and workflow are required for read receipts",
        ));
    }

    Ok(Json(json!({
        "receipt_id": deterministic_receipt_id(&capsule_id, &request),
        "status": "recorded",
        "capsule_id": capsule_id,
        "bundle_digest": request.bundle_digest,
        "warning_count": request.warnings_triggered.len()
    })))
}

fn load_protocol(
    state: &ApiState,
    capsule_type: CapsuleType,
) -> Result<ElicitationProtocol, ApiError> {
    let protocol_path = state
        .protocol_dir
        .join(format!("{}.v1.json", capsule_type.as_str()));
    let protocol = load_elicitation_protocol(&protocol_path)
        .map_err(|error| ApiError::internal(error.to_string()))?;
    if protocol.capsule_type != capsule_type {
        return Err(ApiError::internal(format!(
            "protocol {} has capsule_type {}, expected {}",
            protocol.protocol_id,
            protocol.capsule_type.as_str(),
            capsule_type.as_str()
        )));
    }
    Ok(protocol)
}

fn parse_capsule_type(value: &str) -> Result<CapsuleType, ApiError> {
    match value.trim().to_ascii_lowercase().replace('-', "_").as_str() {
        "user" => Ok(CapsuleType::User),
        "situation" => Ok(CapsuleType::Situation),
        "tool" => Ok(CapsuleType::Tool),
        "output" => Ok(CapsuleType::Output),
        _ => Err(ApiError::bad_request(
            "capsule_type must be user, situation, tool, or output",
        )),
    }
}

fn load_package(state: &ApiState, capsule_id: &str) -> Result<PraxisCapsulePackage, ApiError> {
    let package = PraxisCapsulePackage::load_from_dir(&state.package_dir)
        .map_err(|error| ApiError::internal(error.to_string()))?;
    if package.manifest.capsule_id != capsule_id {
        return Err(ApiError::not_found(format!(
            "capsule '{capsule_id}' is not available in fixture mode"
        )));
    }
    Ok(package)
}

fn build_graph_preview(package: &PraxisCapsulePackage, graph: &Value) -> Value {
    let records = graph_records(graph);
    let node_ids = records
        .iter()
        .filter_map(|record| record.get("@id").and_then(Value::as_str).map(str::to_owned))
        .collect::<BTreeSet<_>>();
    let nodes = records
        .iter()
        .filter(|record| review_state(record) != "rejected")
        .map(|record| {
            json!({
                "id": record.get("@id").and_then(Value::as_str).unwrap_or(""),
                "label": record
                    .get("dialectica:title")
                    .and_then(Value::as_str)
                    .or_else(|| record.get("@id").and_then(Value::as_str))
                    .unwrap_or(""),
                "node_type": node_type(record),
                "rank": 1.0,
                "why_surfaced": "source-backed capsule graph node",
                "review_state": review_state(record)
            })
        })
        .collect::<Vec<_>>();
    let edges = graph_edges(&records, &node_ids);

    JsonPreview {
        schema_version: "graph_preview_v1",
        capsule_id: &package.manifest.capsule_id,
        graph_profile: "situation_graph_v1",
        nodes,
        edges,
    }
    .into_value()
}

fn graph_records(graph: &Value) -> Vec<Value> {
    graph
        .get("@graph")
        .and_then(Value::as_array)
        .into_iter()
        .flatten()
        .flat_map(|layer| {
            layer
                .get("@graph")
                .and_then(Value::as_array)
                .into_iter()
                .flatten()
                .cloned()
        })
        .collect()
}

fn graph_edges(records: &[Value], node_ids: &BTreeSet<String>) -> Vec<Value> {
    let mut edges = Vec::new();
    let mut index = 0usize;
    for record in records {
        if review_state(record) == "rejected" {
            continue;
        }
        let Some(from_id) = record.get("@id").and_then(Value::as_str) else {
            continue;
        };
        let Some(object) = record.as_object() else {
            continue;
        };
        for (key, value) in object {
            if key.starts_with('@') {
                continue;
            }
            for target in target_ids(value) {
                if node_ids.contains(&target) {
                    index += 1;
                    edges.push(json!({
                        "id": format!("edge_preview_{index:04}"),
                        "from": from_id,
                        "to": target,
                        "edge_type": key.rsplit([':', '#', '/']).next().unwrap_or("related_to"),
                        "review_state": review_state(record),
                        "source_receipt_links": source_span_ids(record),
                        "temporal_status": "current"
                    }));
                }
            }
        }
    }
    edges
}

fn target_ids(value: &Value) -> Vec<String> {
    match value {
        Value::String(value) => vec![value.clone()],
        Value::Array(values) => values.iter().flat_map(target_ids).collect(),
        Value::Object(object) => object
            .get("@id")
            .and_then(Value::as_str)
            .map(|value| vec![value.to_owned()])
            .unwrap_or_default(),
        _ => Vec::new(),
    }
}

fn source_span_ids(record: &Value) -> Vec<String> {
    record
        .get("dialectica:sourceSpanIds")
        .and_then(Value::as_array)
        .into_iter()
        .flatten()
        .filter_map(Value::as_str)
        .map(str::to_owned)
        .collect()
}

fn node_type(record: &Value) -> String {
    record
        .get("@type")
        .and_then(Value::as_str)
        .and_then(|value| value.rsplit([':', '#', '/']).next())
        .unwrap_or("node")
        .to_owned()
}

fn review_state(record: &Value) -> &str {
    record
        .get("dialectica:reviewState")
        .and_then(Value::as_str)
        .unwrap_or("approved_with_caveats")
}

fn deterministic_receipt_id(capsule_id: &str, request: &ReadReceiptRequest) -> String {
    let base = format!(
        "{}_{}_{}",
        capsule_id, request.workflow, request.praxis_run_id
    );
    format!(
        "receipt_{}",
        base.chars()
            .map(|character| {
                if character.is_ascii_alphanumeric() {
                    character.to_ascii_lowercase()
                } else {
                    '_'
                }
            })
            .collect::<String>()
    )
}

fn count_jsonl(path: &Path) -> Result<usize, ApiError> {
    let text =
        std::fs::read_to_string(path).map_err(|error| ApiError::internal(error.to_string()))?;
    Ok(text.lines().filter(|line| !line.trim().is_empty()).count())
}

fn read_json(path: &Path) -> Result<Value, ApiError> {
    let text =
        std::fs::read_to_string(path).map_err(|error| ApiError::internal(error.to_string()))?;
    serde_json::from_str(&text).map_err(|error| ApiError::internal(error.to_string()))
}

#[derive(Debug, Deserialize)]
struct ContextPackQuery {
    workflow: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct ReadReceiptRequest {
    pub praxis_run_id: String,
    pub workflow: String,
    pub bundle_digest: String,
    #[serde(default)]
    pub source_ids: Vec<String>,
    #[serde(default)]
    pub claim_ids: Vec<String>,
    #[serde(default)]
    pub graph_node_ids: Vec<String>,
    #[serde(default)]
    pub graph_edge_ids: Vec<String>,
    #[serde(default)]
    pub reasoning_device_ids: Vec<String>,
    #[serde(default)]
    pub agent_guidance_ids: Vec<String>,
    #[serde(default)]
    pub warnings_triggered: Vec<String>,
}

struct JsonPreview<'a> {
    schema_version: &'a str,
    capsule_id: &'a str,
    graph_profile: &'a str,
    nodes: Vec<Value>,
    edges: Vec<Value>,
}

impl JsonPreview<'_> {
    fn into_value(self) -> Value {
        json!({
            "schema_version": self.schema_version,
            "capsule_id": self.capsule_id,
            "graph_profile": self.graph_profile,
            "graph_lens": "stakeholder_power_lens",
            "nodes": self.nodes,
            "edges": self.edges,
            "clusters": [],
            "review_styles": {
                "approved": "solid",
                "approved_with_caveats": "dashed",
                "needs_review": "muted",
                "rejected": "hidden_by_default"
            },
            "temporal_filters": ["current", "stale", "superseded", "forecast", "contested"],
            "source_receipt_links": [],
            "warnings": []
        })
    }
}

#[derive(Debug)]
struct ApiError {
    status: StatusCode,
    code: &'static str,
    message: String,
}

impl ApiError {
    fn internal(message: impl Into<String>) -> Self {
        Self {
            status: StatusCode::INTERNAL_SERVER_ERROR,
            code: "internal_error",
            message: message.into(),
        }
    }

    fn not_found(message: impl Into<String>) -> Self {
        Self {
            status: StatusCode::NOT_FOUND,
            code: "capsule_not_found",
            message: message.into(),
        }
    }

    fn bad_request(message: impl Into<String>) -> Self {
        Self {
            status: StatusCode::BAD_REQUEST,
            code: "bad_request",
            message: message.into(),
        }
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response<Body> {
        (
            self.status,
            Json(json!({
                "error": {
                    "code": self.code,
                    "message": self.message
                }
            })),
        )
            .into_response()
    }
}

#[cfg(test)]
mod tests {
    use axum::{
        body::{to_bytes, Body},
        http::{Method, Request, StatusCode},
    };
    use serde_json::{json, Value};
    use tower::ServiceExt;

    use super::{app, ApiState};

    #[tokio::test]
    async fn fixture_routes_serve_manifest_graph_and_context_pack() {
        let app = app(ApiState::default_fixture());

        let health = request_json(app.clone(), Method::GET, "/health", None).await;
        assert_eq!(health.0, StatusCode::OK);
        assert_eq!(health.1["status"], "ok");

        let manifest = request_json(
            app.clone(),
            Method::GET,
            "/v1/capsules/cap_conflict_situation_fixture_v3/manifest",
            None,
        )
        .await;
        assert_eq!(manifest.0, StatusCode::OK);
        assert_eq!(manifest.1["type"], "situation");

        let graph = request_json(
            app.clone(),
            Method::GET,
            "/v1/capsules/cap_conflict_situation_fixture_v3/graph-preview",
            None,
        )
        .await;
        assert_eq!(graph.0, StatusCode::OK);
        assert_eq!(graph.1["schema_version"], "graph_preview_v1");
        assert!(!graph.1["nodes"].as_array().expect("nodes array").is_empty());

        let context = request_json(
            app,
            Method::GET,
            "/v1/capsules/cap_conflict_situation_fixture_v3/praxis-context-pack?workflow=decision_brief",
            None,
        )
        .await;
        assert_eq!(context.0, StatusCode::OK);
        assert_eq!(context.1["workflow"], "decision_brief");
    }

    #[tokio::test]
    async fn protocol_routes_serve_and_score_tool_protocol() {
        let app = app(ApiState::default_fixture());

        let protocol = request_json(app.clone(), Method::GET, "/v1/protocols/tool", None).await;
        assert_eq!(protocol.0, StatusCode::OK);
        assert_eq!(protocol.1["protocol_id"], "tool.v1");
        assert_eq!(
            protocol.1["completeness"]["criteria"][0]["target_record_family"],
            "reasoning_device"
        );

        let score = request_json(
            app,
            Method::POST,
            "/v1/protocols/tool/score",
            Some(json!({
                "session_id": "sess_tool_fixture",
                "protocol_id": "tool.v1",
                "capsule_type": "tool",
                "current_stage_id": "output_contract",
                "answers": [
                    {
                        "answer_id": "ans_walkthrough",
                        "stage_id": "walkthrough",
                        "source_span_id": "span_walkthrough",
                        "text": "Expert walkthrough.",
                        "derived_record_counts": {
                            "reasoning_device": 10
                        }
                    },
                    {
                        "answer_id": "ans_traps",
                        "stage_id": "traps",
                        "source_span_id": "span_traps",
                        "text": "Expert traps.",
                        "derived_record_counts": {
                            "trap": 3,
                            "precedent": 2
                        }
                    },
                    {
                        "answer_id": "ans_precedents",
                        "stage_id": "precedents",
                        "source_span_id": "span_precedents",
                        "text": "Expert precedents.",
                        "derived_record_counts": {}
                    },
                    {
                        "answer_id": "ans_devices",
                        "stage_id": "devices",
                        "source_span_id": "span_devices",
                        "text": "Expert devices.",
                        "derived_record_counts": {}
                    },
                    {
                        "answer_id": "ans_output",
                        "stage_id": "output_contract",
                        "source_span_id": "span_output",
                        "text": "Expert output contract.",
                        "derived_record_counts": {}
                    }
                ]
            })),
        )
        .await;

        assert_eq!(score.0, StatusCode::OK);
        assert_eq!(score.1["complete_enough"], true);
    }

    #[tokio::test]
    async fn read_receipt_route_returns_deterministic_fixture_receipt() {
        let app = app(ApiState::default_fixture());
        let receipt = request_json(
            app,
            Method::POST,
            "/v1/capsules/cap_conflict_situation_fixture_v3/read-receipts",
            Some(json!({
                "praxis_run_id": "run_fixture_001",
                "workflow": "decision_brief",
                "bundle_digest": "sha256:fixture",
                "warnings_triggered": ["approved_with_caveats"]
            })),
        )
        .await;

        assert_eq!(receipt.0, StatusCode::OK);
        assert_eq!(receipt.1["status"], "recorded");
        assert_eq!(
            receipt.1["receipt_id"],
            "receipt_cap_conflict_situation_fixture_v3_decision_brief_run_fixture_001"
        );
    }

    async fn request_json(
        app: axum::Router,
        method: Method,
        uri: &str,
        body: Option<Value>,
    ) -> (StatusCode, Value) {
        let request_body = body
            .map(|value| Body::from(value.to_string()))
            .unwrap_or_else(Body::empty);
        let request = Request::builder()
            .method(method)
            .uri(uri)
            .header("content-type", "application/json")
            .body(request_body)
            .expect("request should build");
        let response = app.oneshot(request).await.expect("response should return");
        let status = response.status();
        let bytes = to_bytes(response.into_body(), 1024 * 1024)
            .await
            .expect("body should read");
        let value = serde_json::from_slice(&bytes).expect("json body should parse");
        (status, value)
    }
}
