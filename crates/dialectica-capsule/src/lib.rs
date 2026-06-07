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

/// Current capsule schema version used by scaffold fixtures and API contracts.
pub const CAPSULE_SCHEMA_VERSION: &str = "0.1.0";

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
            schema_version: CAPSULE_SCHEMA_VERSION.to_owned(),
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

    /// Returns true when the manifest can be exported as a PRAXIS-visible bundle.
    pub fn is_export_ready(&self) -> bool {
        self.has_required_identity()
            && self.review_state.allows_praxis_use()
            && !self.bundle_digest.trim().is_empty()
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
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct CapsuleInspection {
    pub capsule_id: String,
    pub capsule_type: String,
    pub review_state: ReviewState,
    pub source_count: usize,
    pub claim_count: usize,
    pub graph_node_count: usize,
    pub graph_edge_count: usize,
    pub warning_count: usize,
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

    if bundle.manifest.schema_version != CAPSULE_SCHEMA_VERSION {
        report.push(ValidationFinding::error(
            "unsupported_schema_version",
            "manifest.schema_version",
            format!(
                "Expected schema version {CAPSULE_SCHEMA_VERSION}, found {}.",
                bundle.manifest.schema_version
            ),
            Some(bundle.manifest.capsule_id.clone()),
            "Export with the current schema version or add a migration.",
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
    use super::{CapsuleManifest, ReviewState};

    #[test]
    fn approved_manifest_with_digest_is_export_ready() {
        let manifest = CapsuleManifest::new(
            "cap_stakeholder_001",
            "Stakeholder capsule",
            "stakeholder_capsule",
            ReviewState::Approved,
            "sha256:test",
        );

        assert!(manifest.is_export_ready());
    }

    #[test]
    fn draft_manifest_is_not_export_ready() {
        let manifest = CapsuleManifest::new(
            "cap_stakeholder_001",
            "Stakeholder capsule",
            "stakeholder_capsule",
            ReviewState::Draft,
            "sha256:test",
        );

        assert!(!manifest.is_export_ready());
    }
}
