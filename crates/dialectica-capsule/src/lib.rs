//! Core PRAXIS Capsule contract primitives.
//!
//! This crate owns the portable bundle vocabulary before ingestion, storage,
//! graph adapters, or model-powered extraction are added.

use std::{
    collections::{BTreeMap, BTreeSet},
    error::Error,
    fmt::{Display, Formatter},
    fs,
    path::{Path, PathBuf},
};

use schemars::{schema::RootSchema, schema_for, JsonSchema};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_json::Value;
use sha2::{Digest, Sha256};

/// Current canonical PRAXIS Capsule format version.
pub const CAPSULE_SPEC_VERSION: &str = "3.0";

/// MIME type written into the first uncompressed entry of a `.capsule` archive.
pub const CAPSULE_MIME_TYPE: &str = "application/vnd.tacitus.praxis-capsule+zip";

/// Legacy directory-bundle schema kept only for compatibility while the
/// compiler migrates to the v3 `.capsule` contract.
pub const LEGACY_BUNDLE_SCHEMA_VERSION: &str = "0.1.0";

/// Backward-compatible alias used by existing scaffold binaries.
pub const CAPSULE_SCHEMA_VERSION: &str = CAPSULE_SPEC_VERSION;

/// Canonical PRAXIS-importable capsule macro types.
pub const CANONICAL_CAPSULE_TYPES: &[&str] = &["user", "situation", "tool", "output"];

/// Legacy PRAXIS-importable capsule class names.
pub const LEGACY_CAPSULE_TYPES: &[&str] = &[
    "user_capsule",
    "situation_capsule",
    "tool_capsule",
    "output_capsule",
];

/// Backward-compatible alias for older code that still says capsule_type.
pub const APPROVED_CAPSULE_TYPES: &[&str] = LEGACY_CAPSULE_TYPES;

/// Canonical v3 layer names. The `memory` layer is supported but not required
/// for the first executable compiler slice.
pub const CANONICAL_LAYER_NAMES: &[&str] = &[
    "evidence",
    "claims",
    "situation",
    "temporal",
    "ontology",
    "reasoning",
    "memory",
    "governance",
    "runtime",
];

const REQUIRED_V3_LAYERS: &[&str] = &[
    "evidence",
    "claims",
    "situation",
    "temporal",
    "ontology",
    "reasoning",
    "governance",
    "runtime",
];

pub const LADYBUG_PROJECTION_PROFILE: &str = "ladybug_projection_v1";
pub const LADYBUG_DATABASE_PATH: &str = "graph/ladybug/capsule.lbug";
pub const LADYBUG_PROJECTION_MANIFEST_PATH: &str = "graph/ladybug/projection_manifest.json";
pub const LADYBUG_SCHEMA_PATH: &str = "graph/ladybug/schema.cypher";
pub const LADYBUG_QUERIES_PATH: &str = "graph/ladybug/queries.cypher";
pub const LADYBUG_BUILD_RECEIPT_PATH: &str = "graph/ladybug/build_receipt.json";
const REQUIRED_LADYBUG_PROJECTION_FILES: &[&str] = &[
    LADYBUG_DATABASE_PATH,
    LADYBUG_PROJECTION_MANIFEST_PATH,
    LADYBUG_SCHEMA_PATH,
    LADYBUG_QUERIES_PATH,
    LADYBUG_BUILD_RECEIPT_PATH,
];

const REGISTERED_NODE_TYPES: &[&str] = &[
    "actor",
    "institution",
    "source",
    "source_span",
    "event",
    "claim",
    "concept",
    "policy_instrument",
    "risk",
    "decision",
    "reasoning_device",
    "review_action",
    "output_contract",
    "rights_policy",
];

const REGISTERED_EDGE_TYPES: &[&str] = &[
    "supports",
    "contradicts",
    "mentions",
    "authored_by",
    "regulated_by",
    "influences",
    "incentivized_by",
    "causes",
    "depends_on",
    "supersedes",
    "belongs_to_frame",
    "uses_device",
    "reviewed_by",
    "forbidden_for",
    "has_output_rule",
    "has_rights_policy",
];

/// Returns true when the capsule type is part of the PRAXIS-importable contract.
pub fn is_approved_capsule_type(capsule_type: &str) -> bool {
    let capsule_type = capsule_type.trim();
    CANONICAL_CAPSULE_TYPES.contains(&capsule_type) || LEGACY_CAPSULE_TYPES.contains(&capsule_type)
}

/// Returns true when the capsule type uses the v3 canonical manifest value.
pub fn is_canonical_capsule_type(capsule_type: &str) -> bool {
    CANONICAL_CAPSULE_TYPES.contains(&capsule_type.trim())
}

/// Maps legacy type names to canonical v3 macro types.
pub fn canonical_capsule_type(capsule_type: &str) -> Option<&'static str> {
    match capsule_type.trim() {
        "user" | "user_capsule" => Some("user"),
        "situation" | "situation_capsule" => Some("situation"),
        "tool" | "tool_capsule" => Some("tool"),
        "output" | "output_capsule" => Some("output"),
        _ => None,
    }
}

/// Human-review state for a capsule or one of its internal objects.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ReviewState {
    /// Author-created or machine-proposed, not yet inspected.
    Draft,
    /// A reviewer must inspect before promotion.
    NeedsReview,
    /// Approved for the recorded scope.
    Approved,
    /// Approved, but caveats must travel with the capsule.
    ApprovedWithCaveats,
    /// Blocked by review.
    Rejected,
    /// Approved for PRAXIS use.
    Promoted,
}

impl ReviewState {
    /// Returns true when the state allows a capsule to be used in PRAXIS flows.
    pub const fn allows_praxis_use(self) -> bool {
        matches!(
            self,
            Self::Approved | Self::ApprovedWithCaveats | Self::Promoted
        )
    }
}

/// Severity for a deterministic capsule validation finding.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ValidationSeverity {
    Error,
    Warning,
    Info,
}

/// One validation result attached to a concrete capsule path.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct ValidationFinding {
    pub code: String,
    pub path: String,
    pub message: String,
    pub severity: ValidationSeverity,
    pub object_id: Option<String>,
    pub help: Option<String>,
}

impl ValidationFinding {
    fn error(
        code: &str,
        path: impl Into<String>,
        message: impl Into<String>,
        object_id: Option<String>,
        help: impl Into<String>,
    ) -> Self {
        Self {
            code: code.to_owned(),
            path: path.into(),
            message: message.into(),
            severity: ValidationSeverity::Error,
            object_id,
            help: Some(help.into()),
        }
    }

    fn warning(
        code: &str,
        path: impl Into<String>,
        message: impl Into<String>,
        object_id: Option<String>,
        help: impl Into<String>,
    ) -> Self {
        Self {
            code: code.to_owned(),
            path: path.into(),
            message: message.into(),
            severity: ValidationSeverity::Warning,
            object_id,
            help: Some(help.into()),
        }
    }
}

/// Validation report for a capsule bundle.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct ValidationReport {
    pub findings: Vec<ValidationFinding>,
}

impl ValidationReport {
    /// Returns true when any finding blocks bundle promotion.
    pub fn has_errors(&self) -> bool {
        self.findings
            .iter()
            .any(|finding| finding.severity == ValidationSeverity::Error)
    }

    fn push(&mut self, finding: ValidationFinding) {
        self.findings.push(finding);
    }
}

/// Minimal load error with path context.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CapsuleLoadError {
    message: String,
}

impl CapsuleLoadError {
    fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }
}

impl Display for CapsuleLoadError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(&self.message)
    }
}

impl Error for CapsuleLoadError {}

/// Manifest for a portable capsule bundle.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct CapsuleManifest {
    pub capsule_id: String,
    pub schema_version: String,
    pub title: String,
    pub summary: String,
    pub created_at: String,
    pub compiled_at: String,
    pub tenant_id: String,
    pub project_id: String,
    pub status: String,
    pub freshness: String,
    pub source_count: usize,
    pub review_state: ReviewState,
    #[schemars(regex(pattern = "^(user_capsule|situation_capsule|tool_capsule|output_capsule)$"))]
    pub capsule_type: String,
    pub rights_profile: String,
    pub graph_profile: String,
    pub reasoning_profile: String,
    pub language_profile: String,
    pub agent_guidance_profile: String,
    pub compatibility_profile: String,
    pub bundle_digest: String,
    pub compiler_version: String,
    pub capsule_health: String,
}

impl CapsuleManifest {
    /// Creates a minimal manifest for tests and scaffolded binaries.
    pub fn new(
        capsule_id: &str,
        title: &str,
        capsule_type: &str,
        review_state: ReviewState,
        bundle_digest: &str,
    ) -> Self {
        Self {
            capsule_id: capsule_id.to_owned(),
            schema_version: LEGACY_BUNDLE_SCHEMA_VERSION.to_owned(),
            title: title.to_owned(),
            summary: String::new(),
            created_at: String::new(),
            compiled_at: String::new(),
            tenant_id: String::new(),
            project_id: String::new(),
            status: "scaffold".to_owned(),
            freshness: "unknown".to_owned(),
            source_count: 0,
            review_state,
            capsule_type: capsule_type.to_owned(),
            rights_profile: String::new(),
            graph_profile: String::new(),
            reasoning_profile: String::new(),
            language_profile: String::new(),
            agent_guidance_profile: String::new(),
            compatibility_profile: String::new(),
            bundle_digest: bundle_digest.to_owned(),
            compiler_version: String::new(),
            capsule_health: String::new(),
        }
    }

    /// Returns true when required manifest identity fields are present.
    pub fn has_required_identity(&self) -> bool {
        !self.capsule_id.trim().is_empty()
            && !self.title.trim().is_empty()
            && !self.capsule_type.trim().is_empty()
            && !self.schema_version.trim().is_empty()
    }

    /// Returns true when the manifest uses a PRAXIS-importable capsule class.
    pub fn has_approved_capsule_type(&self) -> bool {
        is_approved_capsule_type(&self.capsule_type)
    }

    /// Returns true when the manifest can be exported as a PRAXIS-visible bundle.
    pub fn is_export_ready(&self) -> bool {
        self.has_required_identity()
            && self.has_approved_capsule_type()
            && self.review_state.allows_praxis_use()
            && !self.bundle_digest.trim().is_empty()
    }

    /// Creates a capsule-specific ontology and semantic-layer blueprint.
    pub fn ontology_blueprint(&self) -> CapsuleOntologyBlueprint {
        CapsuleOntologyBlueprint::from_manifest(self)
    }
}

/// Bundle-level index for layer paths and digests.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct CapsuleBundleIndex {
    pub capsule_id: String,
    pub schema_version: String,
    pub layers: Vec<CapsuleLayerIndex>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct CapsuleLayerIndex {
    pub layer: String,
    pub path: String,
    pub digest: String,
}

/// One source span or source record used by a capsule.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct SourceLedgerRecord {
    pub source_id: String,
    pub source_type: String,
    pub title: String,
    pub uri: String,
    pub publisher: String,
    pub published_at: String,
    pub retrieved_at: String,
    pub language: String,
    pub license_or_access: String,
    pub trust_status: String,
    pub span_id: String,
    pub span_locator: String,
    pub hash: String,
    pub notes: String,
}

/// Alias for source spans. The current bundle keeps sources and spans together.
pub type SourceSpanRecord = SourceLedgerRecord;

/// One temporal claim record.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct TemporalLedgerRecord {
    pub claim_id: String,
    pub claim_text: String,
    pub valid_time_start: String,
    pub valid_time_end: Option<String>,
    pub published_at: String,
    pub observed_at: String,
    pub status: TemporalStatus,
    pub confidence: f64,
    pub supersedes: Vec<String>,
    pub superseded_by: Vec<String>,
    pub source_ids: Vec<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum TemporalStatus {
    Current,
    Stale,
    Superseded,
    Forecast,
    Contested,
    Unknown,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct OntologySlice {
    pub ontology_id: String,
    pub version: String,
    pub namespace: String,
    pub language: String,
    pub domains: Vec<String>,
    pub terms: Vec<OntologyTerm>,
    pub mappings: Vec<OntologyMapping>,
    pub frame_memberships: Vec<FrameMembership>,
    pub legal_document_profile: Option<LegalDocumentProfile>,
    pub deprecations: Vec<String>,
    pub review_notes: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct OntologyTerm {
    pub term_id: String,
    pub label: String,
    pub definition: String,
    pub source_span_ids: Vec<String>,
    pub broader: Vec<String>,
    pub review_state: ReviewState,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct OntologyMapping {
    pub source_label: String,
    pub term_id: String,
    pub confidence: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct FrameMembership {
    pub frame_id: String,
    pub term_ids: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct LegalDocumentProfile {
    pub applies: bool,
    pub citation_style: String,
    pub jurisdiction: String,
    pub authority_type: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct GraphSlice {
    pub schema_version: String,
    pub capsule_id: String,
    pub graph_profile: String,
    pub nodes: Vec<GraphNode>,
    pub edges: Vec<GraphEdge>,
    pub communities: Vec<GraphCommunity>,
    pub layout_hints: BTreeMap<String, Value>,
    pub health: GraphHealth,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct GraphNode {
    pub id: String,
    pub node_type: String,
    pub label: String,
    pub review_state: ReviewState,
    #[serde(default)]
    pub review_scope: Option<String>,
    #[serde(default)]
    pub review_action_ids: Vec<String>,
    #[serde(default)]
    pub caveat_ids: Vec<String>,
    #[serde(default)]
    pub expires_at: Option<String>,
    #[serde(default)]
    pub blocked_workflows: Vec<String>,
    #[serde(default)]
    pub reviewed_at: Option<String>,
    #[serde(default)]
    pub source_span_ids: Vec<String>,
    #[serde(default)]
    pub properties: BTreeMap<String, Value>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct GraphEdge {
    pub id: String,
    pub from_node_id: String,
    pub to_node_id: String,
    pub edge_type: String,
    #[serde(default)]
    pub source_span_ids: Vec<String>,
    pub confidence: f64,
    pub review_state: ReviewState,
    #[serde(default)]
    pub review_scope: Option<String>,
    #[serde(default)]
    pub review_action_ids: Vec<String>,
    #[serde(default)]
    pub caveat_ids: Vec<String>,
    #[serde(default)]
    pub expires_at: Option<String>,
    #[serde(default)]
    pub blocked_workflows: Vec<String>,
    #[serde(default)]
    pub reviewed_at: Option<String>,
    #[serde(default)]
    pub temporal_scope: Option<TemporalScope>,
    pub explanation: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct TemporalScope {
    pub valid_from: String,
    pub valid_until: Option<String>,
    pub status: TemporalStatus,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct GraphCommunity {
    pub id: String,
    pub label: String,
    pub node_ids: Vec<String>,
    pub why_surfaced: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct GraphHealth {
    pub unsupported_edge_count: usize,
    pub unreviewed_edge_count: usize,
    pub stale_edge_count: usize,
    pub contradiction_cluster_count: usize,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct ReasoningDevice {
    pub device_id: String,
    pub title: String,
    pub purpose: String,
    pub steps: Vec<String>,
    pub source_requirements: Vec<String>,
    pub failure_modes: Vec<String>,
    pub review_state: ReviewState,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct LanguageProfile {
    pub profile_id: String,
    pub primary_language: String,
    pub secondary_languages: Vec<String>,
    pub audience_register: String,
    pub approved_terms: Vec<LanguageRule>,
    pub deprecated_terms: Vec<LanguageRule>,
    pub blocked_phrases: Vec<LanguageRule>,
    pub framing_rules: Vec<LanguageRule>,
    pub translation_notes: Vec<String>,
    pub citation_language: String,
    pub uncertainty_language: String,
    pub review_state: ReviewState,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct LanguageRule {
    pub rule_id: String,
    pub text: String,
    pub reason: String,
    #[serde(default)]
    pub source_span_ids: Vec<String>,
    #[serde(default)]
    pub review_action_ids: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct AgentGuidance {
    pub allowed_workflows: Vec<String>,
    pub tool_policy: ToolPolicy,
    pub citation_policy: CitationPolicy,
    pub graph_use_policy: GraphUsePolicy,
    pub language_profile_refs: Vec<String>,
    pub reasoning_sequence: Vec<String>,
    pub context_budget_policy: String,
    pub stop_conditions: Vec<String>,
    pub handoff_policy: String,
    pub audit_receipts_required: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct ToolPolicy {
    pub allowed_tools: Vec<String>,
    pub blocked_tools: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct CitationPolicy {
    pub required_for: Vec<String>,
    pub default_style: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct GraphUsePolicy {
    pub default_lens: String,
    pub hide_rejected_by_default: bool,
    pub require_reviewed_edges: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct OutputContract {
    pub output_contract_id: String,
    pub output_type: String,
    pub required_sections: Vec<String>,
    pub source_receipts_required: bool,
    pub forbidden_claims: Vec<String>,
    pub review_state: ReviewState,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct ReviewLedgerRecord {
    pub review_action_id: String,
    pub reviewer_role: String,
    pub reviewed_at: String,
    pub object_ids: Vec<String>,
    pub decision: ReviewState,
    pub caveats: Vec<String>,
    pub expires_at: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct RightsProfile {
    pub rights_profile_id: String,
    pub permissions: Vec<RightsRule>,
    pub prohibitions: Vec<RightsRule>,
    pub duties: Vec<RightsRule>,
    pub review_state: ReviewState,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct RightsRule {
    pub rule_id: String,
    pub action: String,
    pub target: String,
    pub workflow: String,
    pub reason: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct MarketplaceListing {
    pub listing_id: String,
    pub capsule_id: String,
    pub visibility: String,
    pub review_level: String,
    pub freshness: String,
    pub caveats: Vec<String>,
    pub rights_summary: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct CapsuleHealthReport {
    pub capsule_id: String,
    pub status: String,
    pub source_coverage: String,
    pub temporal_status: String,
    pub review_status: String,
    pub warnings: Vec<String>,
}

/// Canonical v3 manifest for a portable `.capsule` package.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct PraxisCapsuleManifest {
    pub capsule_id: String,
    pub spec_version: String,
    pub version: u64,
    #[serde(rename = "type")]
    #[schemars(regex(pattern = "^(user|situation|tool|output)$"))]
    pub capsule_type: String,
    pub category: String,
    pub title: String,
    #[serde(default)]
    pub owner_uid: Option<String>,
    #[serde(default)]
    pub situation_id: Option<String>,
    pub cores: Vec<String>,
    pub created_at: String,
    pub updated_at: String,
    pub provenance_root_hash: String,
    pub signature: String,
    #[serde(default)]
    pub depends_on: Vec<String>,
    pub layers_present: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct LadybugProjectionManifest {
    pub schema_version: String,
    pub capsule_id: String,
    pub profile: String,
    pub engine: String,
    pub engine_crate: String,
    pub engine_crate_version: String,
    pub storage_version: u64,
    pub database_path: String,
    pub source_graph_path: String,
    pub source_graph_digest: String,
    pub projection_digest: String,
    pub schema_path: String,
    pub schema_digest: String,
    pub query_path: String,
    pub query_digest: String,
    pub node_count: usize,
    pub edge_count: usize,
    pub read_only: bool,
    pub rebuildable: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct LadybugProjectionBuildReceipt {
    pub schema_version: String,
    pub capsule_id: String,
    pub profile: String,
    pub built_at_unix_seconds: u64,
    pub source_graph_digest: String,
    pub projection_digest: String,
    pub node_count: usize,
    pub edge_count: usize,
    pub query_check: String,
}

impl PraxisCapsuleManifest {
    pub fn has_required_identity(&self) -> bool {
        !self.capsule_id.trim().is_empty()
            && !self.title.trim().is_empty()
            && !self.capsule_type.trim().is_empty()
            && !self.spec_version.trim().is_empty()
    }

    pub fn is_canonical_type(&self) -> bool {
        is_canonical_capsule_type(&self.capsule_type)
    }
}

/// Canonical v3 directory projection of a `.capsule` archive.
///
/// The production artifact is a zip archive. This type validates the same
/// canonical files after extraction or before archive assembly.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct PraxisCapsulePackage {
    pub manifest: PraxisCapsuleManifest,
    pub root: PathBuf,
}

impl PraxisCapsulePackage {
    pub fn load_from_dir(path: &Path) -> Result<Self, CapsuleLoadError> {
        Ok(Self {
            manifest: read_json(path, "manifest.json")?,
            root: path.to_path_buf(),
        })
    }

    pub fn validate(&self) -> ValidationReport {
        let mut report = ValidationReport::default();

        validate_v3_manifest(self, &mut report);
        validate_v3_required_files(self, &mut report);
        validate_v3_ladybug_projection(self, &mut report);
        validate_v3_compiled_views(self, &mut report);
        validate_v3_minimum_records(self, &mut report);

        report
    }

    pub fn inspection(&self) -> CapsuleInspection {
        CapsuleInspection {
            capsule_id: self.manifest.capsule_id.clone(),
            capsule_type: self.manifest.capsule_type.clone(),
            review_state: ReviewState::ApprovedWithCaveats,
            source_count: count_jsonl_records(&self.root.join("evidence/sources.jsonl"))
                .unwrap_or_default(),
            claim_count: count_jsonl_records(&self.root.join("claims.jsonl")).unwrap_or_default(),
            graph_node_count: 0,
            graph_edge_count: 0,
            warning_count: self
                .validate()
                .findings
                .iter()
                .filter(|finding| finding.severity == ValidationSeverity::Warning)
                .count(),
        }
    }
}

/// Complete local capsule bundle assembled from the canonical directory shape.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct CapsuleBundle {
    pub manifest: CapsuleManifest,
    pub capsule: Value,
    pub source_ledger: Vec<SourceLedgerRecord>,
    pub temporal_ledger: Vec<TemporalLedgerRecord>,
    pub ontology_slice: OntologySlice,
    pub graph_slice: GraphSlice,
    pub graph_semantics: Value,
    pub graph_constraints: Value,
    pub reasoning_playbook: Vec<ReasoningDevice>,
    pub language_profile: LanguageProfile,
    pub agent_guidance: AgentGuidance,
    pub retrieval_pack: Vec<Value>,
    pub output_contracts: Vec<OutputContract>,
    pub review_ledger: Vec<ReviewLedgerRecord>,
    pub rights_profile: RightsProfile,
    pub marketplace_listing: MarketplaceListing,
    pub capsule_health: CapsuleHealthReport,
    pub eval_report: Value,
}

impl CapsuleBundle {
    /// Loads a capsule bundle from the expected bundle directory layout.
    pub fn load_from_dir(path: &Path) -> Result<Self, CapsuleLoadError> {
        Ok(Self {
            manifest: read_json(path, "manifest.json")?,
            capsule: read_json(path, "capsule.json")?,
            source_ledger: read_jsonl(path, "source_ledger.jsonl")?,
            temporal_ledger: read_jsonl(path, "temporal_ledger.jsonl")?,
            ontology_slice: read_json(path, "ontology_slice.json")?,
            graph_slice: read_json(path, "graph_slice.json")?,
            graph_semantics: read_json(path, "graph_semantics.jsonld")?,
            graph_constraints: read_json(path, "graph_constraints.json")?,
            reasoning_playbook: read_json(path, "reasoning_playbook.json")?,
            language_profile: read_json(path, "language_profile.json")?,
            agent_guidance: read_json(path, "agent_guidance.json")?,
            retrieval_pack: read_jsonl(path, "retrieval_pack.jsonl")?,
            output_contracts: read_json(path, "output_contracts.json")?,
            review_ledger: read_jsonl(path, "review_ledger.jsonl")?,
            rights_profile: read_json(path, "rights_profile.json")?,
            marketplace_listing: read_json(path, "marketplace_listing.json")?,
            capsule_health: read_json(path, "capsule_health.json")?,
            eval_report: read_json(path, "eval_report.json")?,
        })
    }

    /// Validates sourceability, graph registry use, review gates, and temporal state.
    pub fn validate(&self) -> ValidationReport {
        let mut report = ValidationReport::default();

        validate_manifest(self, &mut report);
        validate_temporal_ledger(self, &mut report);
        validate_graph_slice(self, &mut report);
        validate_language_profile(self, &mut report);
        validate_rights_profile(self, &mut report);

        report
    }

    /// Returns a compact human-readable summary for CLI inspection.
    pub fn inspection(&self) -> CapsuleInspection {
        CapsuleInspection {
            capsule_id: self.manifest.capsule_id.clone(),
            capsule_type: self.manifest.capsule_type.clone(),
            review_state: self.manifest.review_state,
            source_count: self.source_ledger.len(),
            claim_count: self.temporal_ledger.len(),
            graph_node_count: self.graph_slice.nodes.len(),
            graph_edge_count: self.graph_slice.edges.len(),
            warning_count: self
                .validate()
                .findings
                .iter()
                .filter(|finding| finding.severity == ValidationSeverity::Warning)
                .count(),
        }
    }

    /// Builds the semantic blueprint PRAXIS should use when loading this capsule.
    pub fn ontology_blueprint(&self) -> CapsuleOntologyBlueprint {
        CapsuleOntologyBlueprint::from_bundle(self)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct CapsuleInspection {
    pub capsule_id: String,
    #[schemars(regex(
        pattern = "^(user|situation|tool|output|user_capsule|situation_capsule|tool_capsule|output_capsule)$"
    ))]
    pub capsule_type: String,
    pub review_state: ReviewState,
    pub source_count: usize,
    pub claim_count: usize,
    pub graph_node_count: usize,
    pub graph_edge_count: usize,
    pub warning_count: usize,
}

/// A callable plan for developing the ontology and semantic layer of a capsule.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct CapsuleOntologyBlueprint {
    pub blueprint_id: String,
    pub capsule_id: String,
    #[schemars(regex(pattern = "^(user_capsule|situation_capsule|tool_capsule|output_capsule)$"))]
    pub capsule_type: String,
    pub ontology_family: String,
    pub graph_profile: String,
    pub semantic_layers: Vec<SemanticLayerPlan>,
    pub suggested_node_classes: Vec<String>,
    pub suggested_edge_classes: Vec<String>,
    pub reasoning_lenses: Vec<String>,
    pub extraction_questions: Vec<String>,
    pub praxis_context_guidance: Vec<String>,
    pub review_gates: Vec<String>,
}

impl CapsuleOntologyBlueprint {
    /// Creates a blueprint from manifest metadata when no bundle exists yet.
    pub fn from_manifest(manifest: &CapsuleManifest) -> Self {
        let mut blueprint = capsule_type_blueprint(manifest);
        blueprint.review_gates = universal_review_gates();
        blueprint
    }

    /// Creates a blueprint enriched with the bundle's actual graph and ontology.
    pub fn from_bundle(bundle: &CapsuleBundle) -> Self {
        let mut blueprint = Self::from_manifest(&bundle.manifest);

        merge_unique(
            &mut blueprint.suggested_node_classes,
            bundle
                .graph_slice
                .nodes
                .iter()
                .map(|node| node.node_type.clone()),
        );
        merge_unique(
            &mut blueprint.suggested_edge_classes,
            bundle
                .graph_slice
                .edges
                .iter()
                .map(|edge| edge.edge_type.clone()),
        );
        merge_unique(
            &mut blueprint.reasoning_lenses,
            bundle
                .reasoning_playbook
                .iter()
                .map(|device| device.title.clone()),
        );
        merge_unique(
            &mut blueprint.extraction_questions,
            bundle.ontology_slice.terms.iter().map(|term| {
                format!(
                    "Which source spans define or qualify the concept '{}'?",
                    term.label
                )
            }),
        );
        merge_unique(
            &mut blueprint.praxis_context_guidance,
            bundle
                .agent_guidance
                .allowed_workflows
                .iter()
                .map(|workflow| format!("Prepare semantic context for `{workflow}` workflow.")),
        );

        blueprint
    }
}

/// One semantic layer that the capsule should capture or expose to PRAXIS.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct SemanticLayerPlan {
    pub layer_id: String,
    pub purpose: String,
    pub captures: Vec<String>,
    pub required_evidence: Vec<String>,
    pub praxis_use: String,
}

/// Writes deterministic schema snapshots for the current Rust contract.
pub fn export_schema_dir(path: &Path) -> Result<(), CapsuleLoadError> {
    fs::create_dir_all(path).map_err(|error| {
        CapsuleLoadError::new(format!(
            "failed to create schema dir {}: {error}",
            path.display()
        ))
    })?;

    write_schema(path, "manifest.schema.json", schema_for!(CapsuleManifest))?;
    write_schema(
        path,
        "praxis_capsule_manifest.schema.json",
        schema_for!(PraxisCapsuleManifest),
    )?;
    write_schema(
        path,
        "praxis_capsule_package.schema.json",
        schema_for!(PraxisCapsulePackage),
    )?;
    write_schema(path, "capsule.schema.json", schema_for!(CapsuleBundle))?;
    write_schema(
        path,
        "source_ledger.schema.json",
        schema_for!(SourceLedgerRecord),
    )?;
    write_schema(
        path,
        "temporal_ledger.schema.json",
        schema_for!(TemporalLedgerRecord),
    )?;
    write_schema(
        path,
        "ontology_slice.schema.json",
        schema_for!(OntologySlice),
    )?;
    write_schema(path, "graph_slice.schema.json", schema_for!(GraphSlice))?;
    write_schema(
        path,
        "reasoning_playbook.schema.json",
        schema_for!(Vec<ReasoningDevice>),
    )?;
    write_schema(
        path,
        "language_profile.schema.json",
        schema_for!(LanguageProfile),
    )?;
    write_schema(
        path,
        "agent_guidance.schema.json",
        schema_for!(AgentGuidance),
    )?;
    write_schema(
        path,
        "output_contracts.schema.json",
        schema_for!(Vec<OutputContract>),
    )?;
    write_schema(
        path,
        "review_ledger.schema.json",
        schema_for!(ReviewLedgerRecord),
    )?;
    write_schema(
        path,
        "rights_profile.schema.json",
        schema_for!(RightsProfile),
    )?;
    write_schema(
        path,
        "marketplace_listing.schema.json",
        schema_for!(MarketplaceListing),
    )?;
    write_schema(
        path,
        "ontology_blueprint.schema.json",
        schema_for!(CapsuleOntologyBlueprint),
    )?;
    write_schema(
        path,
        "ladybug_projection_manifest.schema.json",
        schema_for!(LadybugProjectionManifest),
    )?;
    write_schema(
        path,
        "ladybug_projection_build_receipt.schema.json",
        schema_for!(LadybugProjectionBuildReceipt),
    )?;

    Ok(())
}

pub fn validate_ladybug_projection_files(root: &Path) -> Result<(), CapsuleLoadError> {
    for relative in REQUIRED_LADYBUG_PROJECTION_FILES {
        if !root.join(relative).is_file() {
            return Err(CapsuleLoadError::new(format!(
                "required Ladybug projection file '{}' is missing",
                relative
            )));
        }
    }

    let manifest: LadybugProjectionManifest = read_json(root, LADYBUG_PROJECTION_MANIFEST_PATH)?;
    validate_ladybug_projection_manifest(root, &manifest)?;
    Ok(())
}

fn capsule_type_blueprint(manifest: &CapsuleManifest) -> CapsuleOntologyBlueprint {
    let mut blueprint = match manifest.capsule_type.as_str() {
        "user_capsule" => CapsuleOntologyBlueprint {
            blueprint_id: format!("ontology-blueprint:{}", manifest.capsule_id),
            capsule_id: manifest.capsule_id.clone(),
            capsule_type: manifest.capsule_type.clone(),
            ontology_family: "user_context_ontology".to_owned(),
            graph_profile: default_graph_profile(manifest, "user_context_graph_v1"),
            semantic_layers: vec![
                semantic_layer(
                    "preference_and_authority_layer",
                    "Capture how the user works, what authority they have, and where PRAXIS must stop.",
                    ["role", "mandate", "preferences", "authority boundaries", "privacy constraints"],
                    ["user-approved onboarding notes", "prior output summaries", "review actions"],
                    "Personalize PRAXIS output without exposing private user context or inventing authority.",
                ),
                semantic_layer(
                    "output_style_layer",
                    "Capture reviewed writing, citation, and decision-support preferences.",
                    ["voice", "register", "preferred artifacts", "blocked public claims"],
                    ["language profile", "prior outputs", "reviewed preferences"],
                    "Guide answer shape and tone while preserving sourceability.",
                ),
            ],
            suggested_node_classes: strings(["actor", "concept", "output_contract", "rights_policy"]),
            suggested_edge_classes: strings(["uses_device", "has_output_rule", "has_rights_policy"]),
            reasoning_lenses: strings(["sourceability check", "decision-clock preference", "authority boundary check"]),
            extraction_questions: strings([
                "What should PRAXIS know about the user's role, mandate, and authority limits?",
                "Which preferences are reviewed facts rather than inferred habits?",
                "Which private context must never leak into outputs?",
            ]),
            praxis_context_guidance: strings([
                "Use user ontology as personalization and authority context, not as domain truth.",
                "Apply user language rules after source and situation facts are resolved.",
            ]),
            review_gates: Vec::new(),
        },
        "tool_capsule" => CapsuleOntologyBlueprint {
            blueprint_id: format!("ontology-blueprint:{}", manifest.capsule_id),
            capsule_id: manifest.capsule_id.clone(),
            capsule_type: manifest.capsule_type.clone(),
            ontology_family: "tool_method_ontology".to_owned(),
            graph_profile: default_graph_profile(manifest, "tool_method_graph_v1"),
            semantic_layers: vec![semantic_layer(
                "method_trace_layer",
                "Capture the intellectual tool, philosophical lens, source requirements, steps, outputs, and failure modes.",
                ["method steps", "input requirements", "failure modes", "review caveats", "epistemic stance"],
                ["expert review", "method examples", "source requirements", "counterexamples"],
                "Tell PRAXIS how to reason through a class of work, not only what facts to retrieve.",
            )],
            suggested_node_classes: strings(["reasoning_device", "claim", "risk", "output_contract"]),
            suggested_edge_classes: strings(["uses_device", "depends_on", "has_output_rule"]),
            reasoning_lenses: strings([
                "method transfer",
                "failure-mode scan",
                "sourceability check",
                "expert-lens alignment",
            ]),
            extraction_questions: strings([
                "What are the method's required inputs?",
                "Which reasoning steps must happen in order?",
                "What would make this method unsafe or misleading?",
                "Which tacit expert distinctions must the agent preserve?",
            ]),
            praxis_context_guidance: strings([
                "Use the tool ontology to sequence PRAXIS reasoning before drafting.",
                "Do not let the tool invent situation facts; combine it with a Situation Capsule for evidence.",
            ]),
            review_gates: Vec::new(),
        },
        "output_capsule" => CapsuleOntologyBlueprint {
            blueprint_id: format!("ontology-blueprint:{}", manifest.capsule_id),
            capsule_id: manifest.capsule_id.clone(),
            capsule_type: manifest.capsule_type.clone(),
            ontology_family: "output_trace_ontology".to_owned(),
            graph_profile: default_graph_profile(manifest, "output_trace_graph_v1"),
            semantic_layers: vec![semantic_layer(
                "artifact_lineage_layer",
                "Capture how an output was produced, what it cites, and how it may be reused.",
                ["artifact sections", "claim lineage", "source receipts", "reuse rules"],
                ["output contract", "source ledger", "review ledger"],
                "Let PRAXIS update or reuse an artifact without losing its reasoning trail.",
            )],
            suggested_node_classes: strings([
                "output_contract",
                "claim",
                "source_span",
                "reasoning_device",
                "review_action",
            ]),
            suggested_edge_classes: strings(["supports", "uses_device", "reviewed_by", "has_output_rule"]),
            reasoning_lenses: strings(["artifact trace", "reuse risk", "sourceability check"]),
            extraction_questions: strings([
                "Which claims in the output are reusable and under what caveats?",
                "Which sources and reasoning devices shaped each section?",
            ]),
            praxis_context_guidance: strings([
                "Use output ontology as artifact memory, not as fresh situation truth.",
            ]),
            review_gates: Vec::new(),
        },
        _ => CapsuleOntologyBlueprint {
            blueprint_id: format!("ontology-blueprint:{}", manifest.capsule_id),
            capsule_id: manifest.capsule_id.clone(),
            capsule_type: manifest.capsule_type.clone(),
            ontology_family: "capsule_specific_ontology".to_owned(),
            graph_profile: default_graph_profile(manifest, "embedded_graph_v1"),
            semantic_layers: vec![semantic_layer(
                "capsule_specific_meaning_layer",
                "Capture the entities, concepts, relations, lenses, and constraints this capsule actually needs.",
                ["entities", "concepts", "relations", "reasoning lenses", "usage constraints"],
                ["source spans", "review actions", "expert notes"],
                "Provide PRAXIS with a capsule-specific meaning map instead of a generic context blob.",
            )],
            suggested_node_classes: strings(["source", "source_span", "concept", "claim", "review_action"]),
            suggested_edge_classes: strings(["supports", "mentions", "reviewed_by"]),
            reasoning_lenses: strings(["sourceability check", "review-state check"]),
            extraction_questions: strings([
                "What are the capsule-specific objects that PRAXIS must understand?",
                "Which relationships matter for this capsule's workflows?",
            ]),
            praxis_context_guidance: strings([
                "Use this ontology as a capsule-specific meaning map for PRAXIS.",
            ]),
            review_gates: Vec::new(),
        },
    };

    if manifest.capsule_type == "situation_capsule" {
        blueprint.ontology_family = "situation_policy_ontology".to_owned();
        blueprint.graph_profile = default_graph_profile(manifest, "situation_graph_v1");
        blueprint.semantic_layers = vec![
            semantic_layer(
                "source_proof_layer",
                "Capture what each source directly supports, contradicts, qualifies, or does not prove.",
                ["source spans", "claim support", "contradictions", "trust status", "scope of proof"],
                ["source ledger", "hashes", "span locators", "review notes"],
                "Give PRAXIS precise receipts before it synthesizes situation context.",
            ),
            semantic_layer(
                "actor_claim_temporal_graph",
                "Capture the situation's actors, claims, events, risks, decisions, and valid-time state.",
                ["actors", "claims", "events", "risks", "decisions", "temporal status"],
                ["source spans", "temporal ledger", "review actions"],
                "Use the source-backed situation graph to guide PRAXIS analysis and warnings.",
            ),
            semantic_layer(
                "domain_semantic_layer",
                "Capture the domain terms, authorities, instruments, frames, and contested meanings that define the issue.",
                ["concepts", "synonyms", "authorities", "policy instruments", "frame memberships"],
                ["domain sources", "expert definitions", "reviewed mappings"],
                "Help PRAXIS interpret source language through the right policy vocabulary.",
            ),
            semantic_layer(
                "stakeholder_power_layer",
                "Capture actors, incentives, constraints, influence channels, legitimacy, missing affected groups, and uncertainty.",
                ["actors", "incentives", "constraints", "influence", "legitimacy", "missing actors"],
                ["source spans", "expert caveats", "review actions"],
                "Surface who matters, what they can do, and where actor reasoning is weak or speculative.",
            ),
            semantic_layer(
                "scenario_causality_layer",
                "Capture causal hypotheses, plausible branches, assumptions, indicators, and triggers without turning forecasts into facts.",
                ["events", "signals", "assumptions", "risks", "decision triggers", "causal claims"],
                ["source spans", "temporal ledger", "scenario review notes"],
                "Let PRAXIS reason about possible futures while preserving current-fact boundaries.",
            ),
            semantic_layer(
                "policy_instrument_layer",
                "Capture instruments, authorities, constraints, and feasibility tradeoffs.",
                ["policy instruments", "authorities", "constraints", "tradeoffs"],
                ["official sources", "expert caveats", "ontology terms"],
                "Help PRAXIS distinguish legal, fiscal, operational, and political feasibility.",
            ),
            semantic_layer(
                "risk_and_decision_layer",
                "Capture decision points, risk state, uncertainty, and escalation triggers.",
                ["risks", "uncertainties", "decision clocks", "escalation triggers"],
                ["reviewed risk notes", "temporal claims", "source receipts"],
                "Tell PRAXIS what must be caveated or escalated before output.",
            ),
        ];
        blueprint.suggested_node_classes = strings([
            "actor",
            "institution",
            "claim",
            "event",
            "risk",
            "decision",
            "source_span",
            "policy_instrument",
        ]);
        blueprint.suggested_edge_classes = strings([
            "supports",
            "mentions",
            "contradicts",
            "supersedes",
            "depends_on",
            "regulated_by",
            "influences",
            "incentivized_by",
            "causes",
        ]);
        blueprint.reasoning_lenses = strings([
            "stakeholder analysis",
            "conflict mapping",
            "scenario tree",
            "definition check",
            "scope-of-proof check",
            "decision-clock analysis",
            "sourceability check",
            "temporal freshness check",
        ]);
        blueprint.extraction_questions = strings([
            "Which actors, institutions, claims, risks, and decisions define the situation?",
            "Which claims are current, stale, superseded, forecast, contested, or unknown?",
            "Which terms, authorities, and instruments carry domain-specific meaning?",
            "Which actor incentives or constraints are sourced, inferred, or disputed?",
            "Which causal hypotheses and scenario branches require explicit uncertainty?",
            "Which causal or feasibility claims need explicit caveats?",
        ]);
        blueprint.praxis_context_guidance = strings([
            "Use the source-backed situation graph before drafting.",
            "Surface stale or contested claims as warnings, not as silent context.",
            "Apply domain ontology and expert reasoning lenses before final language rules.",
        ]);
    }

    merge_universal_layers(&mut blueprint.semantic_layers);
    blueprint
}

fn semantic_layer(
    layer_id: &str,
    purpose: &str,
    captures: impl IntoIterator<Item = &'static str>,
    required_evidence: impl IntoIterator<Item = &'static str>,
    praxis_use: &str,
) -> SemanticLayerPlan {
    SemanticLayerPlan {
        layer_id: layer_id.to_owned(),
        purpose: purpose.to_owned(),
        captures: strings(captures),
        required_evidence: strings(required_evidence),
        praxis_use: praxis_use.to_owned(),
    }
}

fn merge_universal_layers(layers: &mut Vec<SemanticLayerPlan>) {
    let universal_layers = [
        semantic_layer(
            "sourceability_layer",
            "Keep every material object connected to source spans, review actions, or explicit expert notes.",
            ["source ids", "source span ids", "hashes", "review action ids"],
            ["source ledger", "review ledger"],
            "Let PRAXIS answer 'why do we know this?' for every important claim.",
        ),
        semantic_layer(
            "temporal_validity_layer",
            "Track whether information is current, stale, superseded, forecast, contested, or unknown.",
            ["valid time", "observed time", "freshness", "supersession"],
            ["temporal ledger", "source publication dates"],
            "Prevent PRAXIS from flattening live context into timeless facts.",
        ),
        semantic_layer(
            "language_semantics_layer",
            "Carry reviewed terms, blocked framings, caveats, translation notes, and audience register.",
            ["approved terms", "deprecated terms", "blocked phrases", "uncertainty language"],
            ["language profile", "review actions"],
            "Guide final wording after facts, graph, and reasoning constraints are resolved.",
        ),
        semantic_layer(
            "review_and_rights_layer",
            "Preserve human gates, caveats, expiry, permissions, prohibitions, and duties.",
            ["review state", "rights rules", "blocked workflows", "expiry"],
            ["review ledger", "rights profile"],
            "Stop PRAXIS from using unreviewed or rights-blocked context.",
        ),
        semantic_layer(
            "praxis_agent_guidance_layer",
            "Encode how agents should retrieve, reason, cite, stop, and hand off.",
            ["allowed workflows", "tool policy", "citation policy", "stop conditions"],
            ["agent guidance", "output contracts"],
            "Turn the capsule into operational context for agentic workflows.",
        ),
    ];

    for layer in universal_layers {
        if !layers
            .iter()
            .any(|existing| existing.layer_id == layer.layer_id)
        {
            layers.push(layer);
        }
    }
}

fn default_graph_profile(manifest: &CapsuleManifest, fallback: &str) -> String {
    if manifest.graph_profile.trim().is_empty() {
        fallback.to_owned()
    } else {
        manifest.graph_profile.clone()
    }
}

fn universal_review_gates() -> Vec<String> {
    strings([
        "Every semantic layer must declare source spans, review actions, or expert notes.",
        "Every capsule-specific ontology term must be scoped to its capsule, domain, language, and review state.",
        "PRAXIS context packs must hide rejected objects by default and show caveats for reviewed-with-caveats objects.",
        "Language rules apply after sourceability, temporal validity, graph constraints, and rights checks.",
    ])
}

fn strings(items: impl IntoIterator<Item = &'static str>) -> Vec<String> {
    items.into_iter().map(str::to_owned).collect()
}

fn merge_unique(target: &mut Vec<String>, items: impl IntoIterator<Item = String>) {
    for item in items {
        if !target.contains(&item) {
            target.push(item);
        }
    }
}

fn validate_v3_manifest(package: &PraxisCapsulePackage, report: &mut ValidationReport) {
    let manifest = &package.manifest;

    if !manifest.has_required_identity() {
        report.push(ValidationFinding::error(
            "missing_v3_manifest_identity",
            "manifest",
            "Manifest requires capsule_id, title, type, and spec_version.",
            Some(manifest.capsule_id.clone()),
            "Populate the v3 manifest identity fields before validation.",
        ));
    }

    if manifest.spec_version != CAPSULE_SPEC_VERSION {
        report.push(ValidationFinding::error(
            "unsupported_capsule_spec_version",
            "manifest.spec_version",
            format!(
                "Expected capsule spec version {CAPSULE_SPEC_VERSION}, found {}.",
                manifest.spec_version
            ),
            Some(manifest.capsule_id.clone()),
            "Export with the canonical v3 capsule contract or add a migration.",
        ));
    }

    if !manifest.is_canonical_type() {
        report.push(ValidationFinding::error(
            "unsupported_capsule_type",
            "manifest.type",
            format!(
                "Capsule type '{}' is not canonical. Use one of: {}.",
                manifest.capsule_type,
                CANONICAL_CAPSULE_TYPES.join(", ")
            ),
            Some(manifest.capsule_id.clone()),
            "Use user, situation, tool, or output. Model source packs, stakeholder maps, scenarios, ontology modules, and expert picks as internal layers.",
        ));
    }

    for layer in &manifest.layers_present {
        if !CANONICAL_LAYER_NAMES.contains(&layer.as_str()) {
            report.push(ValidationFinding::error(
                "unknown_capsule_layer",
                "manifest.layers_present",
                format!("Layer '{layer}' is not part of the v3 capsule layer vocabulary."),
                Some(manifest.capsule_id.clone()),
                "Use evidence, claims, situation, temporal, ontology, reasoning, memory, governance, or runtime.",
            ));
        }
    }

    for required_layer in REQUIRED_V3_LAYERS {
        if !manifest
            .layers_present
            .iter()
            .any(|layer| layer == required_layer)
        {
            report.push(ValidationFinding::error(
                "missing_required_capsule_layer",
                "manifest.layers_present",
                format!("Required v3 layer '{required_layer}' is missing."),
                Some(manifest.capsule_id.clone()),
                "Populate layers_present from the canonical bundle files during compilation.",
            ));
        }
    }

    if manifest.cores.is_empty() {
        report.push(ValidationFinding::warning(
            "manifest_has_no_cores",
            "manifest.cores",
            "Manifest has no ontology cores declared.",
            Some(manifest.capsule_id.clone()),
            "Declare at least the ACO core plus the capsule-specific domain or method core.",
        ));
    }

    if manifest.provenance_root_hash.trim().is_empty() || manifest.signature.trim().is_empty() {
        report.push(ValidationFinding::warning(
            "manifest_integrity_not_finalized",
            "manifest.provenance_root_hash",
            "Manifest integrity hash or signature is not finalized.",
            Some(manifest.capsule_id.clone()),
            "Development fixtures may use placeholders, but promoted bundles must be signed.",
        ));
    }
}

fn validate_v3_required_files(package: &PraxisCapsulePackage, report: &mut ValidationReport) {
    let root = &package.root;
    let manifest = &package.manifest;

    let mut required_paths = vec![
        "mimetype",
        "manifest.json",
        "claims.jsonl",
        "graph.jsonld",
        "episodes.json",
        "evidence/sources.jsonl",
        "reasoning/devices.json",
        "reasoning/annotations.json",
        "reasoning/traps.json",
        "review/review.json",
        "runtime.json",
        "agent_context.md",
        "operations.md",
    ];

    match manifest.capsule_type.as_str() {
        "user" => required_paths.push("payload.user.json"),
        "tool" => required_paths.push("payload.tool.json"),
        "output" => required_paths.push("payload.output.json"),
        "situation" => {}
        _ => {}
    }

    for relative in required_paths {
        if !root.join(relative).is_file() {
            report.push(ValidationFinding::error(
                "missing_v3_bundle_file",
                relative,
                format!("Required v3 capsule file '{relative}' is missing."),
                Some(manifest.capsule_id.clone()),
                "Compile the capsule using the v3 bundle shape from docs/CAPSULE_SPEC.md.",
            ));
        }
    }

    for relative in REQUIRED_LADYBUG_PROJECTION_FILES {
        if !root.join(relative).is_file() {
            report.push(ValidationFinding::error(
                "missing_ladybug_projection_file",
                *relative,
                format!("Required Ladybug projection file '{relative}' is missing."),
                Some(manifest.capsule_id.clone()),
                "Build graph/ladybug/capsule.lbug from graph.jsonld before exporting a promoted v3 capsule.",
            ));
        }
    }

    validate_mimetype(root, manifest, report);
    validate_json_file(root, "graph.jsonld", manifest, report);
    validate_json_file(root, "episodes.json", manifest, report);
    validate_json_file(root, "runtime.json", manifest, report);
    validate_json_file(root, "review/review.json", manifest, report);
}

fn validate_v3_ladybug_projection(package: &PraxisCapsulePackage, report: &mut ValidationReport) {
    let root = &package.root;
    let manifest = &package.manifest;

    let projection_path = root.join(LADYBUG_PROJECTION_MANIFEST_PATH);
    if !projection_path.is_file() {
        return;
    }

    let projection: LadybugProjectionManifest =
        match read_json(root, LADYBUG_PROJECTION_MANIFEST_PATH) {
            Ok(projection) => projection,
            Err(error) => {
                report.push(ValidationFinding::error(
                    "invalid_ladybug_projection_manifest",
                    LADYBUG_PROJECTION_MANIFEST_PATH,
                    error.to_string(),
                    Some(manifest.capsule_id.clone()),
                    "Emit valid JSON that matches ladybug_projection_manifest.schema.json.",
                ));
                return;
            }
        };

    if let Err(error) = validate_ladybug_projection_manifest(root, &projection) {
        report.push(ValidationFinding::error(
            "invalid_ladybug_projection",
            LADYBUG_PROJECTION_MANIFEST_PATH,
            error.to_string(),
            Some(manifest.capsule_id.clone()),
            "Regenerate the Ladybug projection from graph.jsonld and update its receipt.",
        ));
    }

    if projection.capsule_id != manifest.capsule_id {
        report.push(ValidationFinding::error(
            "ladybug_capsule_id_mismatch",
            LADYBUG_PROJECTION_MANIFEST_PATH,
            format!(
                "Ladybug projection capsule_id '{}' does not match manifest capsule_id '{}'.",
                projection.capsule_id, manifest.capsule_id
            ),
            Some(manifest.capsule_id.clone()),
            "Regenerate the Ladybug projection from this capsule directory.",
        ));
    }

    if projection.node_count == 0 {
        report.push(ValidationFinding::warning(
            "ladybug_projection_empty",
            LADYBUG_PROJECTION_MANIFEST_PATH,
            "Ladybug projection has no nodes.",
            Some(manifest.capsule_id.clone()),
            "Inspect graph.jsonld and regenerate the projection.",
        ));
    }

    match read_json::<LadybugProjectionBuildReceipt>(root, LADYBUG_BUILD_RECEIPT_PATH) {
        Ok(receipt) => {
            if let Err(error) = validate_ladybug_build_receipt(&projection, &receipt) {
                report.push(ValidationFinding::error(
                    "invalid_ladybug_build_receipt",
                    LADYBUG_BUILD_RECEIPT_PATH,
                    error.to_string(),
                    Some(manifest.capsule_id.clone()),
                    "Regenerate the Ladybug projection so the build receipt matches the manifest.",
                ));
            }
        }
        Err(error) => report.push(ValidationFinding::error(
            "invalid_ladybug_build_receipt",
            LADYBUG_BUILD_RECEIPT_PATH,
            error.to_string(),
            Some(manifest.capsule_id.clone()),
            "Emit valid JSON that matches ladybug_projection_build_receipt.schema.json.",
        )),
    }
}

fn validate_v3_compiled_views(package: &PraxisCapsulePackage, report: &mut ValidationReport) {
    let root = &package.root;
    let manifest = &package.manifest;

    for relative in ["agent_context.md", "operations.md"] {
        match read_optional_string(&root.join(relative)) {
            Ok(Some(text)) if !text.trim().is_empty() => {}
            Ok(Some(_)) => report.push(ValidationFinding::error(
                "empty_compiled_capsule_view",
                relative,
                format!("Compiled view '{relative}' is empty."),
                Some(manifest.capsule_id.clone()),
                "Generate a bounded agent context and self-describing operations card for every capsule.",
            )),
            Ok(None) => {}
            Err(error) => report.push(ValidationFinding::error(
                "unreadable_compiled_capsule_view",
                relative,
                error.to_string(),
                Some(manifest.capsule_id.clone()),
                "Ensure compiled markdown views are readable UTF-8 files.",
            )),
        }
    }
}

fn validate_v3_minimum_records(package: &PraxisCapsulePackage, report: &mut ValidationReport) {
    let manifest = &package.manifest;

    if manifest.capsule_type == "situation" {
        let claims = count_jsonl_records(&package.root.join("claims.jsonl")).unwrap_or_default();
        let sources =
            count_jsonl_records(&package.root.join("evidence/sources.jsonl")).unwrap_or_default();

        if claims == 0 {
            report.push(ValidationFinding::error(
                "situation_capsule_has_no_claims",
                "claims.jsonl",
                "Situation capsules must carry at least one atomic claim.",
                Some(manifest.capsule_id.clone()),
                "Extract or author claim atoms before compiling the situation capsule.",
            ));
        }

        if sources == 0 {
            report.push(ValidationFinding::error(
                "situation_capsule_has_no_sources",
                "evidence/sources.jsonl",
                "Situation capsules must carry at least one evidence source.",
                Some(manifest.capsule_id.clone()),
                "Attach evidence sources and spans before compiling the situation capsule.",
            ));
        }
    }
}

fn validate_mimetype(root: &Path, manifest: &PraxisCapsuleManifest, report: &mut ValidationReport) {
    match read_optional_string(&root.join("mimetype")) {
        Ok(Some(text)) if text.trim() == CAPSULE_MIME_TYPE => {}
        Ok(Some(_)) => report.push(ValidationFinding::error(
            "invalid_capsule_mimetype",
            "mimetype",
            "The mimetype file does not contain the PRAXIS Capsule MIME type.",
            Some(manifest.capsule_id.clone()),
            "Write application/vnd.tacitus.praxis-capsule+zip as the first uncompressed archive entry.",
        )),
        Ok(None) => {}
        Err(error) => report.push(ValidationFinding::error(
            "unreadable_capsule_mimetype",
            "mimetype",
            error.to_string(),
            Some(manifest.capsule_id.clone()),
            "Ensure the mimetype marker is readable before archive assembly.",
        )),
    }
}

fn validate_json_file(
    root: &Path,
    relative: &str,
    manifest: &PraxisCapsuleManifest,
    report: &mut ValidationReport,
) {
    let path = root.join(relative);
    if !path.is_file() {
        return;
    }

    if let Err(error) = read_json::<Value>(root, relative) {
        report.push(ValidationFinding::error(
            "invalid_v3_json_file",
            relative,
            error.to_string(),
            Some(manifest.capsule_id.clone()),
            "Emit valid JSON for every canonical capsule file.",
        ));
    }
}

fn validate_ladybug_projection_manifest(
    root: &Path,
    manifest: &LadybugProjectionManifest,
) -> Result<(), CapsuleLoadError> {
    if manifest.profile != LADYBUG_PROJECTION_PROFILE {
        return Err(CapsuleLoadError::new(format!(
            "expected Ladybug profile {}, found {}",
            LADYBUG_PROJECTION_PROFILE, manifest.profile
        )));
    }
    if manifest.engine != "ladybug" || manifest.engine_crate != "lbug" {
        return Err(CapsuleLoadError::new(
            "Ladybug projection manifest must use engine 'ladybug' and crate 'lbug'",
        ));
    }
    if manifest.database_path != normalize_relative_path(LADYBUG_DATABASE_PATH)
        || manifest.source_graph_path != "graph.jsonld"
        || manifest.schema_path != normalize_relative_path(LADYBUG_SCHEMA_PATH)
        || manifest.query_path != normalize_relative_path(LADYBUG_QUERIES_PATH)
    {
        return Err(CapsuleLoadError::new(
            "Ladybug projection manifest paths must match the v3 capsule layout",
        ));
    }
    if !manifest.read_only || !manifest.rebuildable {
        return Err(CapsuleLoadError::new(
            "Ladybug projection must be marked read_only=true and rebuildable=true",
        ));
    }

    assert_digest(
        "graph.jsonld",
        &manifest.source_graph_digest,
        &root.join("graph.jsonld"),
    )?;
    assert_digest(
        LADYBUG_DATABASE_PATH,
        &manifest.projection_digest,
        &root.join(LADYBUG_DATABASE_PATH),
    )?;
    assert_digest(
        LADYBUG_SCHEMA_PATH,
        &manifest.schema_digest,
        &root.join(LADYBUG_SCHEMA_PATH),
    )?;
    assert_digest(
        LADYBUG_QUERIES_PATH,
        &manifest.query_digest,
        &root.join(LADYBUG_QUERIES_PATH),
    )?;

    Ok(())
}

fn validate_ladybug_build_receipt(
    manifest: &LadybugProjectionManifest,
    receipt: &LadybugProjectionBuildReceipt,
) -> Result<(), CapsuleLoadError> {
    if receipt.schema_version != "ladybug_projection_build_receipt_v1" {
        return Err(CapsuleLoadError::new(
            "Ladybug build receipt schema_version must be ladybug_projection_build_receipt_v1",
        ));
    }
    if receipt.profile != LADYBUG_PROJECTION_PROFILE {
        return Err(CapsuleLoadError::new(format!(
            "expected Ladybug build receipt profile {}, found {}",
            LADYBUG_PROJECTION_PROFILE, receipt.profile
        )));
    }
    if receipt.capsule_id != manifest.capsule_id
        || receipt.source_graph_digest != manifest.source_graph_digest
        || receipt.projection_digest != manifest.projection_digest
        || receipt.node_count != manifest.node_count
        || receipt.edge_count != manifest.edge_count
    {
        return Err(CapsuleLoadError::new(
            "Ladybug build receipt must match capsule_id, digests, and counts from projection_manifest.json",
        ));
    }
    if receipt.query_check.trim().is_empty() {
        return Err(CapsuleLoadError::new(
            "Ladybug build receipt query_check must not be empty",
        ));
    }
    Ok(())
}

fn assert_digest(relative: &str, expected: &str, path: &Path) -> Result<(), CapsuleLoadError> {
    let actual = digest_file(path)?;
    if expected != actual {
        return Err(CapsuleLoadError::new(format!(
            "digest mismatch for {relative}: expected {expected}, found {actual}"
        )));
    }
    Ok(())
}

fn validate_manifest(bundle: &CapsuleBundle, report: &mut ValidationReport) {
    if !bundle.manifest.has_required_identity() {
        report.push(ValidationFinding::error(
            "missing_manifest_identity",
            "manifest",
            "Manifest requires capsule_id, title, capsule_type, and schema_version.",
            Some(bundle.manifest.capsule_id.clone()),
            "Populate the manifest identity fields before validation.",
        ));
    }

    if bundle.manifest.schema_version != LEGACY_BUNDLE_SCHEMA_VERSION {
        report.push(ValidationFinding::error(
            "unsupported_legacy_schema_version",
            "manifest.schema_version",
            format!(
                "Expected legacy bundle schema version {LEGACY_BUNDLE_SCHEMA_VERSION}, found {}.",
                bundle.manifest.schema_version
            ),
            Some(bundle.manifest.capsule_id.clone()),
            "Use the v3 package validator for canonical `.capsule` artifacts or add a migration for this legacy directory bundle.",
        ));
    }

    if !bundle.manifest.has_approved_capsule_type() {
        report.push(ValidationFinding::error(
            "unsupported_capsule_type",
            "manifest.capsule_type",
            format!(
                "Capsule type '{}' is not PRAXIS-importable. Use one of: {}.",
                bundle.manifest.capsule_type,
                APPROVED_CAPSULE_TYPES.join(", ")
            ),
            Some(bundle.manifest.capsule_id.clone()),
            "Model specialized source, stakeholder, domain, scenario, expert-pick, or graph concepts as semantic layers inside a user, situation, tool, or output capsule.",
        ));
    }

    if bundle.manifest.source_count != bundle.source_ledger.len() {
        report.push(ValidationFinding::error(
            "source_count_mismatch",
            "manifest.source_count",
            format!(
                "Manifest source_count is {}, but source_ledger has {} records.",
                bundle.manifest.source_count,
                bundle.source_ledger.len()
            ),
            Some(bundle.manifest.capsule_id.clone()),
            "Update manifest.source_count during deterministic bundle assembly.",
        ));
    }

    if !bundle.manifest.review_state.allows_praxis_use() {
        report.push(ValidationFinding::error(
            "capsule_not_reviewed_for_praxis",
            "manifest.review_state",
            "Capsule review state does not allow PRAXIS workflow use.",
            Some(bundle.manifest.capsule_id.clone()),
            "Promote only after human review approves the capsule scope.",
        ));
    }
}

fn validate_temporal_ledger(bundle: &CapsuleBundle, report: &mut ValidationReport) {
    let source_ids = bundle.source_ids();

    for (index, claim) in bundle.temporal_ledger.iter().enumerate() {
        if claim.source_ids.is_empty() {
            report.push(ValidationFinding::error(
                "temporal_claim_missing_source",
                format!("temporal_ledger[{index}].source_ids"),
                "Temporal claim must cite at least one source.",
                Some(claim.claim_id.clone()),
                "Attach source_ids for every factual temporal claim.",
            ));
        }

        for source_id in &claim.source_ids {
            if !source_ids.contains(source_id) {
                report.push(ValidationFinding::error(
                    "temporal_claim_unknown_source",
                    format!("temporal_ledger[{index}].source_ids"),
                    format!("Temporal claim references unknown source {source_id}."),
                    Some(claim.claim_id.clone()),
                    "Use a source_id present in source_ledger.jsonl.",
                ));
            }
        }

        if matches!(
            claim.status,
            TemporalStatus::Stale | TemporalStatus::Superseded
        ) {
            report.push(ValidationFinding::warning(
                "stale_temporal_claim",
                format!("temporal_ledger[{index}].status"),
                "Temporal claim is not current and must be surfaced with a warning.",
                Some(claim.claim_id.clone()),
                "Keep stale or superseded claims in lineage, but do not use them silently.",
            ));
        }
    }
}

fn validate_graph_slice(bundle: &CapsuleBundle, report: &mut ValidationReport) {
    let source_span_ids = bundle.source_span_ids();
    let node_ids = bundle.graph_node_ids();

    for (index, node) in bundle.graph_slice.nodes.iter().enumerate() {
        if !REGISTERED_NODE_TYPES.contains(&node.node_type.as_str()) {
            report.push(ValidationFinding::error(
                "unregistered_graph_node_type",
                format!("graph_slice.nodes[{index}].node_type"),
                format!("Graph node type {} is not registered.", node.node_type),
                Some(node.id.clone()),
                "Register the node class or map it to an approved alias.",
            ));
        }

        validate_source_spans(
            &node.source_span_ids,
            &source_span_ids,
            format!("graph_slice.nodes[{index}].source_span_ids"),
            &node.id,
            report,
        );
    }

    for (index, edge) in bundle.graph_slice.edges.iter().enumerate() {
        if !REGISTERED_EDGE_TYPES.contains(&edge.edge_type.as_str()) {
            report.push(ValidationFinding::error(
                "unregistered_graph_edge_type",
                format!("graph_slice.edges[{index}].edge_type"),
                format!("Graph edge type {} is not registered.", edge.edge_type),
                Some(edge.id.clone()),
                "Normalize to a registered edge class or add an approved alias.",
            ));
        }

        if !node_ids.contains(&edge.from_node_id) {
            report.push(ValidationFinding::error(
                "graph_edge_unknown_from_node",
                format!("graph_slice.edges[{index}].from_node_id"),
                format!("Unknown from_node_id {}.", edge.from_node_id),
                Some(edge.id.clone()),
                "Add the source node to graph_slice.nodes before referencing it.",
            ));
        }

        if !node_ids.contains(&edge.to_node_id) {
            report.push(ValidationFinding::error(
                "graph_edge_unknown_to_node",
                format!("graph_slice.edges[{index}].to_node_id"),
                format!("Unknown to_node_id {}.", edge.to_node_id),
                Some(edge.id.clone()),
                "Add the target node to graph_slice.nodes before referencing it.",
            ));
        }

        if edge.source_span_ids.is_empty() && edge.review_action_ids.is_empty() {
            report.push(ValidationFinding::error(
                "missing_graph_edge_provenance",
                format!("graph_slice.edges[{index}].source_span_ids"),
                "Graph edge must include at least one source span or review action.",
                Some(edge.id.clone()),
                "Add source_span_ids or review_action_ids before promotion.",
            ));
        }

        validate_source_spans(
            &edge.source_span_ids,
            &source_span_ids,
            format!("graph_slice.edges[{index}].source_span_ids"),
            &edge.id,
            report,
        );

        if !edge.review_state.allows_praxis_use() && edge.review_state != ReviewState::Rejected {
            report.push(ValidationFinding::error(
                "graph_edge_not_reviewed",
                format!("graph_slice.edges[{index}].review_state"),
                "Graph edge is neither reviewed nor rejected.",
                Some(edge.id.clone()),
                "Review or reject graph edges before PRAXIS context-pack export.",
            ));
        }
    }
}

fn validate_language_profile(bundle: &CapsuleBundle, report: &mut ValidationReport) {
    if !bundle.language_profile.review_state.allows_praxis_use() {
        report.push(ValidationFinding::error(
            "language_profile_not_reviewed",
            "language_profile.review_state",
            "Language profile must be human reviewed before PRAXIS use.",
            Some(bundle.language_profile.profile_id.clone()),
            "Approve or caveat the language profile in review_ledger.jsonl.",
        ));
    }
}

fn validate_rights_profile(bundle: &CapsuleBundle, report: &mut ValidationReport) {
    if bundle.rights_profile.prohibitions.is_empty() {
        report.push(ValidationFinding::warning(
            "rights_profile_has_no_prohibitions",
            "rights_profile.prohibitions",
            "Rights profile has no explicit blocked workflow.",
            Some(bundle.rights_profile.rights_profile_id.clone()),
            "Add at least one explicit prohibition for sensitive policy use.",
        ));
    }
}

fn validate_source_spans(
    ids: &[String],
    known_ids: &BTreeSet<String>,
    path: String,
    object_id: &str,
    report: &mut ValidationReport,
) {
    for source_span_id in ids {
        if !known_ids.contains(source_span_id) {
            report.push(ValidationFinding::error(
                "unknown_source_span",
                path.clone(),
                format!("Unknown source span {source_span_id}."),
                Some(object_id.to_owned()),
                "Use a span_id present in source_ledger.jsonl.",
            ));
        }
    }
}

impl CapsuleBundle {
    fn source_ids(&self) -> BTreeSet<String> {
        self.source_ledger
            .iter()
            .map(|record| record.source_id.clone())
            .collect()
    }

    fn source_span_ids(&self) -> BTreeSet<String> {
        self.source_ledger
            .iter()
            .map(|record| record.span_id.clone())
            .collect()
    }

    fn graph_node_ids(&self) -> BTreeSet<String> {
        self.graph_slice
            .nodes
            .iter()
            .map(|node| node.id.clone())
            .collect()
    }
}

fn read_json<T: DeserializeOwned>(base: &Path, relative: &str) -> Result<T, CapsuleLoadError> {
    let path = base.join(relative);
    let text = read_to_string(&path)?;
    serde_json::from_str(&text).map_err(|error| {
        CapsuleLoadError::new(format!("failed to parse {}: {error}", path.display()))
    })
}

fn read_jsonl<T: DeserializeOwned>(
    base: &Path,
    relative: &str,
) -> Result<Vec<T>, CapsuleLoadError> {
    let path = base.join(relative);
    let text = read_to_string(&path)?;
    text.lines()
        .enumerate()
        .filter_map(|(index, line)| {
            let trimmed = line.trim();
            if trimmed.is_empty() {
                None
            } else {
                Some((index + 1, trimmed))
            }
        })
        .map(|(line_number, line)| {
            serde_json::from_str(line).map_err(|error| {
                CapsuleLoadError::new(format!(
                    "failed to parse {} line {}: {error}",
                    path.display(),
                    line_number
                ))
            })
        })
        .collect()
}

fn read_to_string(path: &Path) -> Result<String, CapsuleLoadError> {
    fs::read_to_string(path).map_err(|error| {
        CapsuleLoadError::new(format!("failed to read {}: {error}", path.display()))
    })
}

fn read_optional_string(path: &Path) -> Result<Option<String>, CapsuleLoadError> {
    if !path.exists() {
        return Ok(None);
    }

    read_to_string(path).map(Some)
}

fn count_jsonl_records(path: &Path) -> Result<usize, CapsuleLoadError> {
    let text = read_to_string(path)?;
    Ok(text.lines().filter(|line| !line.trim().is_empty()).count())
}

fn digest_file(path: &Path) -> Result<String, CapsuleLoadError> {
    let bytes = fs::read(path).map_err(|error| {
        CapsuleLoadError::new(format!("failed to read {}: {error}", path.display()))
    })?;
    Ok(format!("sha256:{:x}", Sha256::digest(bytes)))
}

fn normalize_relative_path(path: &str) -> String {
    path.replace('\\', "/")
}

fn write_schema(path: &Path, filename: &str, schema: RootSchema) -> Result<(), CapsuleLoadError> {
    let output = path.join(filename);
    let json = serde_json::to_string_pretty(&schema).map_err(|error| {
        CapsuleLoadError::new(format!("failed to serialize schema {filename}: {error}"))
    })?;
    fs::write(&output, format!("{json}\n")).map_err(|error| {
        CapsuleLoadError::new(format!("failed to write {}: {error}", output.display()))
    })
}

/// Returns the canonical schema directory for a base path.
pub fn schema_version_dir(base: impl Into<PathBuf>) -> PathBuf {
    base.into()
        .join(format!("capsule-{CAPSULE_SCHEMA_VERSION}"))
}

#[cfg(test)]
mod tests {
    use std::{
        fs,
        path::{Path, PathBuf},
        time::{SystemTime, UNIX_EPOCH},
    };

    use super::{CapsuleManifest, PraxisCapsulePackage, ReviewState, CAPSULE_MIME_TYPE};

    #[test]
    fn approved_manifest_with_digest_is_export_ready() {
        let manifest = CapsuleManifest::new(
            "cap_situation_001",
            "Situation capsule",
            "situation_capsule",
            ReviewState::Approved,
            "sha256:test",
        );

        assert!(manifest.is_export_ready());
    }

    #[test]
    fn draft_manifest_is_not_export_ready() {
        let manifest = CapsuleManifest::new(
            "cap_tool_001",
            "Tool capsule",
            "tool_capsule",
            ReviewState::Draft,
            "sha256:test",
        );

        assert!(!manifest.is_export_ready());
    }

    #[test]
    fn unsupported_capsule_type_is_not_export_ready() {
        let manifest = CapsuleManifest::new(
            "cap_source_001",
            "Source pack",
            "source_capsule",
            ReviewState::Approved,
            "sha256:test",
        );

        assert!(!manifest.is_export_ready());
        assert!(!manifest.has_approved_capsule_type());
    }

    #[test]
    fn v3_package_requires_ladybug_projection_files() {
        let root = temp_package_dir("missing_ladybug_projection");
        write_minimal_v3_package(&root);

        let package = PraxisCapsulePackage::load_from_dir(&root).expect("package should load");
        let report = package.validate();

        assert!(report
            .findings
            .iter()
            .any(|finding| finding.code == "missing_ladybug_projection_file"));

        fs::remove_dir_all(root).ok();
    }

    fn temp_package_dir(name: &str) -> PathBuf {
        let unique = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system time should be valid")
            .as_nanos();
        let root = std::env::temp_dir().join(format!("dialectica_{name}_{unique}"));
        fs::create_dir_all(&root).expect("temp package root should be created");
        root
    }

    fn write_minimal_v3_package(root: &Path) {
        fs::create_dir_all(root.join("evidence")).expect("evidence dir should be created");
        fs::create_dir_all(root.join("reasoning")).expect("reasoning dir should be created");
        fs::create_dir_all(root.join("review")).expect("review dir should be created");
        fs::write(root.join("mimetype"), CAPSULE_MIME_TYPE).expect("mimetype should write");
        fs::write(
            root.join("manifest.json"),
            r#"{
  "capsule_id": "cap_missing_ladybug_test",
  "spec_version": "3.0",
  "version": 1,
  "type": "situation",
  "category": "test",
  "title": "Missing Ladybug Test",
  "owner_uid": null,
  "situation_id": null,
  "cores": ["aco"],
  "created_at": "2026-06-08T00:00:00Z",
  "updated_at": "2026-06-08T00:00:00Z",
  "provenance_root_hash": "sha256:test",
  "signature": "test",
  "depends_on": [],
  "layers_present": ["evidence", "claims", "situation", "temporal", "ontology", "reasoning", "governance", "runtime"]
}
"#,
        )
        .expect("manifest should write");
        fs::write(root.join("claims.jsonl"), "{\"id\":\"clm_test\"}\n")
            .expect("claims should write");
        fs::write(
            root.join("graph.jsonld"),
            r#"{"@id":"urn:praxis:capsule:cap_missing_ladybug_test","@graph":[]}"#,
        )
        .expect("graph should write");
        fs::write(root.join("episodes.json"), "{}").expect("episodes should write");
        fs::write(
            root.join("evidence").join("sources.jsonl"),
            "{\"id\":\"src_test\"}\n",
        )
        .expect("sources should write");
        fs::write(root.join("reasoning").join("devices.json"), "[]").expect("devices should write");
        fs::write(root.join("reasoning").join("annotations.json"), "[]")
            .expect("annotations should write");
        fs::write(root.join("reasoning").join("traps.json"), "[]").expect("traps should write");
        fs::write(root.join("review").join("review.json"), "{}").expect("review should write");
        fs::write(root.join("runtime.json"), "{}").expect("runtime should write");
        fs::write(root.join("agent_context.md"), "test").expect("agent context should write");
        fs::write(root.join("operations.md"), "test").expect("operations should write");
    }
}
