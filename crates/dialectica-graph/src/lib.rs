//! Embedded graph projection support for PRAXIS Capsules.

use std::{
    collections::{BTreeMap, BTreeSet},
    error::Error,
    fmt::{Display, Formatter},
    fs,
    path::Path,
};

#[cfg(feature = "ladybug")]
use dialectica_capsule::LadybugProjectionBuildReceipt;
use dialectica_capsule::{CapsuleLoadError, LadybugProjectionManifest};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sha2::{Digest, Sha256};

pub const LADYBUG_SCHEMA_CYPHER: &str = "CREATE NODE TABLE CapsuleNode(id STRING, node_type STRING, label STRING, review_state STRING, source_span_ids_json STRING, properties_json STRING, PRIMARY KEY(id));\nCREATE REL TABLE CapsuleEdge(FROM CapsuleNode TO CapsuleNode, id STRING, edge_type STRING, confidence DOUBLE, temporal_scope_json STRING, source_span_ids_json STRING, review_state STRING, explanation STRING);\n";

pub const LADYBUG_QUERIES_CYPHER: &str = "MATCH (n:CapsuleNode) RETURN count(n) AS node_count;\nMATCH ()-[e:CapsuleEdge]->() RETURN count(e) AS edge_count;\nMATCH (a:CapsuleNode)-[e:CapsuleEdge]->(b:CapsuleNode) RETURN a.id AS from_node_id, e.edge_type AS edge_type, b.id AS to_node_id LIMIT 50;\n";

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LadybugProjectionPlan {
    pub capsule_id: String,
    pub source_graph_digest: String,
    pub node_count: usize,
    pub edge_count: usize,
    pub schema_cypher: String,
    pub queries_cypher: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LadybugQueryOutput {
    pub columns: Vec<String>,
    pub rows: Vec<Vec<String>>,
}

#[derive(Debug, Clone)]
pub struct GraphProjectionError {
    message: String,
}

impl GraphProjectionError {
    fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }
}

impl Display for GraphProjectionError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(&self.message)
    }
}

impl Error for GraphProjectionError {}

impl From<CapsuleLoadError> for GraphProjectionError {
    fn from(value: CapsuleLoadError) -> Self {
        Self::new(value.to_string())
    }
}

impl From<std::io::Error> for GraphProjectionError {
    fn from(value: std::io::Error) -> Self {
        Self::new(value.to_string())
    }
}

impl From<serde_json::Error> for GraphProjectionError {
    fn from(value: serde_json::Error) -> Self {
        Self::new(value.to_string())
    }
}

#[cfg(feature = "ladybug")]
impl From<lbug::Error> for GraphProjectionError {
    fn from(value: lbug::Error) -> Self {
        Self::new(value.to_string())
    }
}

pub fn plan_ladybug_projection(
    capsule_dir: &Path,
) -> Result<LadybugProjectionPlan, GraphProjectionError> {
    let graph_path = capsule_dir.join("graph.jsonld");
    let graph = load_graph(&graph_path)?;
    let records = graph.records()?;

    Ok(LadybugProjectionPlan {
        capsule_id: graph.capsule_id,
        source_graph_digest: digest_file(&graph_path)?,
        node_count: records.nodes.len(),
        edge_count: records.edges.len(),
        schema_cypher: LADYBUG_SCHEMA_CYPHER.to_owned(),
        queries_cypher: LADYBUG_QUERIES_CYPHER.to_owned(),
    })
}

pub fn check_ladybug_projection(
    capsule_dir: &Path,
) -> Result<LadybugProjectionManifest, GraphProjectionError> {
    dialectica_capsule::validate_ladybug_projection_files(capsule_dir)?;
    let manifest_path = capsule_dir.join(dialectica_capsule::LADYBUG_PROJECTION_MANIFEST_PATH);
    let text = fs::read_to_string(manifest_path)?;
    Ok(serde_json::from_str(&text)?)
}

#[cfg(feature = "ladybug")]
pub fn build_ladybug_projection(
    capsule_dir: &Path,
) -> Result<LadybugProjectionManifest, GraphProjectionError> {
    use lbug::{Connection, Database, SystemConfig};

    let graph_path = capsule_dir.join("graph.jsonld");
    let graph = load_graph(&graph_path)?;
    let records = graph.records()?;
    let ladybug_dir = capsule_dir.join("graph").join("ladybug");
    fs::create_dir_all(&ladybug_dir)?;

    let database_path = capsule_dir.join(dialectica_capsule::LADYBUG_DATABASE_PATH);
    remove_existing_database_files(&database_path)?;

    {
        let db = Database::new(&database_path, SystemConfig::default())?;
        let conn = Connection::new(&db)?;
        for statement in LADYBUG_SCHEMA_CYPHER
            .split(';')
            .map(str::trim)
            .filter(|statement| !statement.is_empty())
        {
            conn.query(statement)?;
        }
        for node in &records.nodes {
            conn.query(&node_insert_cypher(node))?;
        }
        for edge in &records.edges {
            conn.query(&edge_insert_cypher(edge))?;
        }
    }

    let schema_path = capsule_dir.join(dialectica_capsule::LADYBUG_SCHEMA_PATH);
    let queries_path = capsule_dir.join(dialectica_capsule::LADYBUG_QUERIES_PATH);
    fs::write(&schema_path, LADYBUG_SCHEMA_CYPHER)?;
    fs::write(&queries_path, LADYBUG_QUERIES_CYPHER)?;

    let source_graph_digest = digest_file(&graph_path)?;
    let projection_digest = digest_file(&database_path)?;
    let schema_digest = digest_file(&schema_path)?;
    let query_digest = digest_file(&queries_path)?;

    let manifest = LadybugProjectionManifest {
        schema_version: "ladybug_projection_manifest_v1".to_owned(),
        capsule_id: graph.capsule_id.clone(),
        profile: dialectica_capsule::LADYBUG_PROJECTION_PROFILE.to_owned(),
        engine: "ladybug".to_owned(),
        engine_crate: "lbug".to_owned(),
        engine_crate_version: lbug::VERSION.to_owned(),
        storage_version: lbug::get_storage_version(),
        database_path: normalize_relative_path(dialectica_capsule::LADYBUG_DATABASE_PATH),
        source_graph_path: "graph.jsonld".to_owned(),
        source_graph_digest: source_graph_digest.clone(),
        projection_digest: projection_digest.clone(),
        schema_path: normalize_relative_path(dialectica_capsule::LADYBUG_SCHEMA_PATH),
        schema_digest,
        query_path: normalize_relative_path(dialectica_capsule::LADYBUG_QUERIES_PATH),
        query_digest,
        node_count: records.nodes.len(),
        edge_count: records.edges.len(),
        read_only: true,
        rebuildable: true,
    };

    let receipt = LadybugProjectionBuildReceipt {
        schema_version: "ladybug_projection_build_receipt_v1".to_owned(),
        capsule_id: graph.capsule_id,
        profile: dialectica_capsule::LADYBUG_PROJECTION_PROFILE.to_owned(),
        built_at_unix_seconds: unix_now(),
        source_graph_digest,
        projection_digest,
        node_count: records.nodes.len(),
        edge_count: records.edges.len(),
        query_check: "build_completed".to_owned(),
    };

    write_json_pretty(
        &capsule_dir.join(dialectica_capsule::LADYBUG_PROJECTION_MANIFEST_PATH),
        &manifest,
    )?;
    write_json_pretty(
        &capsule_dir.join(dialectica_capsule::LADYBUG_BUILD_RECEIPT_PATH),
        &receipt,
    )?;

    Ok(manifest)
}

#[cfg(not(feature = "ladybug"))]
pub fn build_ladybug_projection(
    _capsule_dir: &Path,
) -> Result<LadybugProjectionManifest, GraphProjectionError> {
    Err(GraphProjectionError::new(
        "dialectica-graph was built without the `ladybug` feature",
    ))
}

#[cfg(feature = "ladybug")]
pub fn query_ladybug_projection(
    capsule_dir: &Path,
    cypher: &str,
) -> Result<LadybugQueryOutput, GraphProjectionError> {
    use lbug::{Connection, Database, SystemConfig};

    let database_path = capsule_dir.join(dialectica_capsule::LADYBUG_DATABASE_PATH);
    let db = Database::new(database_path, SystemConfig::default().read_only(true))?;
    let conn = Connection::new(&db)?;
    let mut result = conn.query(cypher)?;
    let columns = result.get_column_names();
    let rows = result
        .by_ref()
        .map(|row| row.into_iter().map(|value| value.to_string()).collect())
        .collect();

    Ok(LadybugQueryOutput { columns, rows })
}

#[cfg(not(feature = "ladybug"))]
pub fn query_ladybug_projection(
    _capsule_dir: &Path,
    _cypher: &str,
) -> Result<LadybugQueryOutput, GraphProjectionError> {
    Err(GraphProjectionError::new(
        "dialectica-graph was built without the `ladybug` feature",
    ))
}

#[derive(Debug, Clone)]
struct JsonLdGraph {
    capsule_id: String,
    named_graphs: Vec<NamedGraph>,
}

#[derive(Debug, Clone)]
struct NamedGraph {
    layer_id: String,
    records: Vec<Value>,
}

#[derive(Debug, Clone)]
struct ProjectionRecords {
    nodes: Vec<ProjectionNode>,
    edges: Vec<ProjectionEdge>,
}

#[cfg_attr(not(feature = "ladybug"), allow(dead_code))]
#[derive(Debug, Clone)]
struct ProjectionNode {
    id: String,
    node_type: String,
    label: String,
    review_state: String,
    source_span_ids_json: String,
    properties_json: String,
}

#[cfg_attr(not(feature = "ladybug"), allow(dead_code))]
#[derive(Debug, Clone)]
struct ProjectionEdge {
    from_id: String,
    to_id: String,
    id: String,
    edge_type: String,
    confidence: f64,
    temporal_scope_json: String,
    source_span_ids_json: String,
    review_state: String,
    explanation: String,
}

fn load_graph(path: &Path) -> Result<JsonLdGraph, GraphProjectionError> {
    let value: Value = serde_json::from_str(&fs::read_to_string(path)?)?;
    let capsule_id = value
        .get("@id")
        .and_then(Value::as_str)
        .unwrap_or("urn:praxis:capsule:unknown")
        .trim_start_matches("urn:praxis:capsule:")
        .to_owned();
    let named_graphs = value
        .get("@graph")
        .and_then(Value::as_array)
        .ok_or_else(|| GraphProjectionError::new("graph.jsonld must contain an @graph array"))?
        .iter()
        .filter_map(|entry| {
            Some(NamedGraph {
                layer_id: entry.get("@id")?.as_str()?.to_owned(),
                records: entry.get("@graph")?.as_array()?.clone(),
            })
        })
        .collect();

    Ok(JsonLdGraph {
        capsule_id,
        named_graphs,
    })
}

impl JsonLdGraph {
    fn records(&self) -> Result<ProjectionRecords, GraphProjectionError> {
        let mut nodes = Vec::new();
        let mut node_ids = BTreeSet::new();
        let mut node_layer = BTreeMap::new();

        for named_graph in &self.named_graphs {
            for record in &named_graph.records {
                let Some(id) = record.get("@id").and_then(Value::as_str) else {
                    continue;
                };
                if node_ids.insert(id.to_owned()) {
                    node_layer.insert(id.to_owned(), named_graph.layer_id.clone());
                    nodes.push(ProjectionNode {
                        id: id.to_owned(),
                        node_type: node_type(record),
                        label: label(record, id),
                        review_state: review_state(record),
                        source_span_ids_json: json_string(
                            record
                                .get("dialectica:sourceSpanIds")
                                .unwrap_or(&Value::Array(vec![])),
                        )?,
                        properties_json: json_string(record)?,
                    });
                }
            }
        }

        let mut edges = Vec::new();
        let mut edge_index = 0usize;
        for named_graph in &self.named_graphs {
            for record in &named_graph.records {
                let Some(from_id) = record.get("@id").and_then(Value::as_str) else {
                    continue;
                };
                let Some(properties) = record.as_object() else {
                    continue;
                };
                for (property, value) in properties {
                    if property.starts_with('@') {
                        continue;
                    }
                    for to_id in target_ids(value) {
                        if !node_ids.contains(&to_id) {
                            continue;
                        }
                        edge_index += 1;
                        edges.push(ProjectionEdge {
                            from_id: from_id.to_owned(),
                            to_id: to_id.clone(),
                            id: format!("edge_{edge_index:04}"),
                            edge_type: edge_type(property),
                            confidence: 1.0,
                            temporal_scope_json: "{}".to_owned(),
                            source_span_ids_json: "[]".to_owned(),
                            review_state: review_state(record),
                            explanation: format!(
                                "{} links {} to {} in {}.",
                                property,
                                from_id,
                                to_id,
                                node_layer.get(from_id).unwrap_or(&named_graph.layer_id)
                            ),
                        });
                    }
                }
            }
        }

        Ok(ProjectionRecords { nodes, edges })
    }
}

fn node_type(record: &Value) -> String {
    record
        .get("@type")
        .and_then(Value::as_str)
        .map(edge_type)
        .unwrap_or_else(|| "node".to_owned())
}

fn label(record: &Value, fallback: &str) -> String {
    record
        .get("dialectica:title")
        .or_else(|| record.get("skos:prefLabel"))
        .or_else(|| record.get("rdfs:label"))
        .and_then(Value::as_str)
        .unwrap_or(fallback)
        .to_owned()
}

fn review_state(record: &Value) -> String {
    record
        .get("dialectica:reviewState")
        .and_then(Value::as_str)
        .unwrap_or("approved_with_caveats")
        .to_owned()
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

fn edge_type(value: impl AsRef<str>) -> String {
    value
        .as_ref()
        .rsplit([':', '#', '/'])
        .next()
        .unwrap_or("related_to")
        .replace('-', "_")
}

#[cfg(feature = "ladybug")]
fn node_insert_cypher(node: &ProjectionNode) -> String {
    format!(
        "CREATE (:CapsuleNode {{id: {}, node_type: {}, label: {}, review_state: {}, source_span_ids_json: {}, properties_json: {}}})",
        cypher_string(&node.id),
        cypher_string(&node.node_type),
        cypher_string(&node.label),
        cypher_string(&node.review_state),
        cypher_string(&node.source_span_ids_json),
        cypher_string(&node.properties_json)
    )
}

#[cfg(feature = "ladybug")]
fn edge_insert_cypher(edge: &ProjectionEdge) -> String {
    format!(
        "MATCH (a:CapsuleNode), (b:CapsuleNode) WHERE a.id = {} AND b.id = {} CREATE (a)-[:CapsuleEdge {{id: {}, edge_type: {}, confidence: {}, temporal_scope_json: {}, source_span_ids_json: {}, review_state: {}, explanation: {}}}]->(b)",
        cypher_string(&edge.from_id),
        cypher_string(&edge.to_id),
        cypher_string(&edge.id),
        cypher_string(&edge.edge_type),
        edge.confidence,
        cypher_string(&edge.temporal_scope_json),
        cypher_string(&edge.source_span_ids_json),
        cypher_string(&edge.review_state),
        cypher_string(&edge.explanation)
    )
}

#[cfg(feature = "ladybug")]
fn cypher_string(value: &str) -> String {
    format!("'{}'", value.replace('\\', "\\\\").replace('\'', "\\'"))
}

fn json_string(value: &Value) -> Result<String, GraphProjectionError> {
    Ok(serde_json::to_string(value)?)
}

fn digest_file(path: &Path) -> Result<String, GraphProjectionError> {
    let bytes = fs::read(path)?;
    Ok(format!("sha256:{:x}", Sha256::digest(bytes)))
}

#[cfg(feature = "ladybug")]
fn write_json_pretty<T: Serialize>(path: &Path, value: &T) -> Result<(), GraphProjectionError> {
    let text = serde_json::to_string_pretty(value)?;
    fs::write(path, format!("{text}\n"))?;
    Ok(())
}

#[cfg(feature = "ladybug")]
fn remove_existing_database_files(path: &Path) -> Result<(), GraphProjectionError> {
    if path.is_dir() {
        fs::remove_dir_all(path)?;
    } else if path.exists() {
        fs::remove_file(path)?;
    }

    for extension in ["wal", "tmp", "lock"] {
        let sidecar = std::path::PathBuf::from(format!("{}.{}", path.display(), extension));
        if sidecar.exists() {
            fs::remove_file(sidecar)?;
        }
    }

    Ok(())
}

#[cfg(feature = "ladybug")]
fn normalize_relative_path(path: &str) -> String {
    path.replace('\\', "/")
}

#[cfg(feature = "ladybug")]
fn unix_now() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|duration| duration.as_secs())
        .unwrap_or_default()
}

#[cfg(test)]
mod tests {
    use super::{plan_ladybug_projection, LADYBUG_SCHEMA_CYPHER};
    use std::path::Path;

    #[test]
    fn projection_plan_reads_canonical_fixture_graph() {
        let plan = plan_ladybug_projection(Path::new(
            "../../fixtures/canonical-capsules/conflict-situation-capsule",
        ))
        .expect("fixture graph should project");

        assert_eq!(plan.capsule_id, "cap_conflict_situation_fixture_v3");
        assert!(plan.node_count >= 8);
        assert!(plan.edge_count >= 3);
        assert_eq!(plan.schema_cypher, LADYBUG_SCHEMA_CYPHER);
    }
}
