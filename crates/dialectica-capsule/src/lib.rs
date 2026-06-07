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
        "team_capsule" => CapsuleOntologyBlueprint {
            blueprint_id: format!("ontology-blueprint:{}", manifest.capsule_id),
            capsule_id: manifest.capsule_id.clone(),
            capsule_type: manifest.capsule_type.clone(),
            ontology_family: "team_memory_ontology".to_owned(),
            graph_profile: default_graph_profile(manifest, "team_memory_graph_v1"),
            semantic_layers: vec![semantic_layer(
                "team_authority_and_memory_layer",
                "Capture team mandate, shared sources, recurring decisions, output standards, and review authority.",
                ["team mandate", "shared sources", "output standards", "review roles"],
                ["team-approved notes", "review ledger", "source ledger"],
                "Let PRAXIS use team memory without collapsing it into a single user's preferences.",
            )],
            suggested_node_classes: strings(["actor", "institution", "source", "output_contract"]),
            suggested_edge_classes: strings([
                "authored_by",
                "supports",
                "reviewed_by",
                "has_output_rule",
            ]),
            reasoning_lenses: strings(["institutional memory check", "review authority check"]),
            extraction_questions: strings([
                "What belongs to the team rather than one user's private context?",
                "Which output standards and review authorities govern this team?",
            ]),
            praxis_context_guidance: strings([
                "Use team ontology as shared workflow context with explicit review authority.",
            ]),
            review_gates: Vec::new(),
        },
        "thinking_device_capsule" | "tool_capsule" => CapsuleOntologyBlueprint {
            blueprint_id: format!("ontology-blueprint:{}", manifest.capsule_id),
            capsule_id: manifest.capsule_id.clone(),
            capsule_type: manifest.capsule_type.clone(),
            ontology_family: "expert_method_ontology".to_owned(),
            graph_profile: default_graph_profile(manifest, "reasoning_device_graph_v1"),
            semantic_layers: vec![semantic_layer(
                "method_trace_layer",
                "Capture the expert method, its inputs, steps, outputs, and failure modes.",
                ["method steps", "input requirements", "failure modes", "review caveats"],
                ["expert review", "method examples", "source requirements"],
                "Tell PRAXIS how to reason, not only what facts to retrieve.",
            )],
            suggested_node_classes: strings(["reasoning_device", "claim", "risk", "output_contract"]),
            suggested_edge_classes: strings(["uses_device", "depends_on", "has_output_rule"]),
            reasoning_lenses: strings(["method transfer", "failure-mode scan", "sourceability check"]),
            extraction_questions: strings([
                "What are the method's required inputs?",
                "Which reasoning steps must happen in order?",
                "What would make this method unsafe or misleading?",
            ]),
            praxis_context_guidance: strings([
                "Use the method ontology to sequence PRAXIS reasoning before drafting.",
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
        "source_capsule" => CapsuleOntologyBlueprint {
            blueprint_id: format!("ontology-blueprint:{}", manifest.capsule_id),
            capsule_id: manifest.capsule_id.clone(),
            capsule_type: manifest.capsule_type.clone(),
            ontology_family: "source_proof_ontology".to_owned(),
            graph_profile: default_graph_profile(manifest, "source_proof_graph_v1"),
            semantic_layers: vec![semantic_layer(
                "source_proof_layer",
                "Capture what the source actually says, where it says it, and what it does not prove.",
                ["source spans", "claims", "definitions", "contradictions", "trust status"],
                ["source ledger", "hashes", "span locators", "review notes"],
                "Give PRAXIS precise receipts before it synthesizes or drafts.",
            )],
            suggested_node_classes: strings(["source", "source_span", "claim", "concept"]),
            suggested_edge_classes: strings(["supports", "mentions", "contradicts", "supersedes"]),
            reasoning_lenses: strings(["sourceability check", "scope-of-proof check"]),
            extraction_questions: strings([
                "What does this source directly support?",
                "Which claims are outside the source scope?",
            ]),
            praxis_context_guidance: strings([
                "Use source ontology to ground claims and prevent unsupported synthesis.",
            ]),
            review_gates: Vec::new(),
        },
        "domain_capsule" => CapsuleOntologyBlueprint {
            blueprint_id: format!("ontology-blueprint:{}", manifest.capsule_id),
            capsule_id: manifest.capsule_id.clone(),
            capsule_type: manifest.capsule_type.clone(),
            ontology_family: "domain_semantic_ontology".to_owned(),
            graph_profile: default_graph_profile(manifest, "domain_ontology_graph_v1"),
            semantic_layers: vec![semantic_layer(
                "domain_vocabulary_layer",
                "Capture domain terms, authorities, instruments, frames, mappings, and contested meanings.",
                ["concepts", "synonyms", "authorities", "policy instruments", "frame memberships"],
                ["domain sources", "expert definitions", "reviewed mappings"],
                "Let PRAXIS understand domain meaning before answering domain-specific questions.",
            )],
            suggested_node_classes: strings(["concept", "institution", "policy_instrument", "source"]),
            suggested_edge_classes: strings(["belongs_to_frame", "regulated_by", "mentions"]),
            reasoning_lenses: strings(["definition check", "frame conflict check", "authority map"]),
            extraction_questions: strings([
                "Which terms carry domain-specific meaning?",
                "Which authorities define or contest those terms?",
            ]),
            praxis_context_guidance: strings([
                "Use domain ontology as semantic grounding for all downstream capsules.",
            ]),
            review_gates: Vec::new(),
        },
        "stakeholder_capsule" => CapsuleOntologyBlueprint {
            blueprint_id: format!("ontology-blueprint:{}", manifest.capsule_id),
            capsule_id: manifest.capsule_id.clone(),
            capsule_type: manifest.capsule_type.clone(),
            ontology_family: "stakeholder_power_ontology".to_owned(),
            graph_profile: default_graph_profile(manifest, "stakeholder_graph_v1"),
            semantic_layers: vec![semantic_layer(
                "stakeholder_power_layer",
                "Capture actors, incentives, constraints, influence channels, legitimacy, and uncertainty.",
                ["actors", "incentives", "constraints", "influence", "legitimacy"],
                ["source spans", "expert caveats", "review actions"],
                "Help PRAXIS reason about who matters, what they can do, and where evidence is weak.",
            )],
            suggested_node_classes: strings([
                "actor",
                "institution",
                "claim",
                "risk",
                "policy_instrument",
            ]),
            suggested_edge_classes: strings([
                "influences",
                "incentivized_by",
                "regulated_by",
                "supports",
                "contradicts",
            ]),
            reasoning_lenses: strings([
                "stakeholder analysis",
                "incentive map",
                "missing-actor scan",
            ]),
            extraction_questions: strings([
                "Which actors have formal authority, informal influence, or implementation capacity?",
                "Which incentive claims are sourced and which are expert caveats?",
            ]),
            praxis_context_guidance: strings([
                "Use stakeholder ontology to surface influence, incentives, and missing actors with caveats.",
            ]),
            review_gates: Vec::new(),
        },
        "scenario_capsule" => CapsuleOntologyBlueprint {
            blueprint_id: format!("ontology-blueprint:{}", manifest.capsule_id),
            capsule_id: manifest.capsule_id.clone(),
            capsule_type: manifest.capsule_type.clone(),
            ontology_family: "scenario_causality_ontology".to_owned(),
            graph_profile: default_graph_profile(manifest, "scenario_graph_v1"),
            semantic_layers: vec![semantic_layer(
                "scenario_branch_layer",
                "Capture plausible futures, indicators, causal hypotheses, assumptions, and decision triggers.",
                ["events", "signals", "assumptions", "risks", "decision triggers"],
                ["source spans", "temporal ledger", "scenario review notes"],
                "Let PRAXIS reason over futures without presenting forecasts as settled facts.",
            )],
            suggested_node_classes: strings(["event", "claim", "risk", "decision", "source_span"]),
            suggested_edge_classes: strings(["causes", "depends_on", "supersedes", "supports"]),
            reasoning_lenses: strings(["scenario tree", "indicator watch", "assumption audit"]),
            extraction_questions: strings([
                "Which assumptions separate one scenario branch from another?",
                "Which indicators would update or invalidate this branch?",
            ]),
            praxis_context_guidance: strings([
                "Use scenario ontology to distinguish plausible futures from current facts.",
            ]),
            review_gates: Vec::new(),
        },
        "expert_pick_capsule" => CapsuleOntologyBlueprint {
            blueprint_id: format!("ontology-blueprint:{}", manifest.capsule_id),
            capsule_id: manifest.capsule_id.clone(),
            capsule_type: manifest.capsule_type.clone(),
            ontology_family: "expert_trust_ontology".to_owned(),
            graph_profile: default_graph_profile(manifest, "expert_pick_graph_v1"),
            semantic_layers: vec![semantic_layer(
                "expert_trust_layer",
                "Capture reviewer judgment, caveats, freshness, scope, rights, and marketplace trust signals.",
                ["reviewer judgment", "caveats", "freshness", "rights", "scope"],
                ["review ledger", "rights profile", "marketplace listing"],
                "Let PRAXIS know why this capsule is recommended and where that recommendation stops.",
            )],
            suggested_node_classes: strings(["review_action", "source", "claim", "rights_policy"]),
            suggested_edge_classes: strings([
                "reviewed_by",
                "supports",
                "has_rights_policy",
                "forbidden_for",
            ]),
            reasoning_lenses: strings(["trust receipt", "scope check", "freshness check"]),
            extraction_questions: strings([
                "What exactly did the expert approve, caveat, or recommend?",
                "Where does the expert-pick scope expire or require escalation?",
            ]),
            praxis_context_guidance: strings([
                "Use expert-pick ontology as trust metadata, not as replacement for source receipts.",
            ]),
            review_gates: Vec::new(),
        },
        "graph_ontology_capsule" => CapsuleOntologyBlueprint {
            blueprint_id: format!("ontology-blueprint:{}", manifest.capsule_id),
            capsule_id: manifest.capsule_id.clone(),
            capsule_type: manifest.capsule_type.clone(),
            ontology_family: "semantic_module_ontology".to_owned(),
            graph_profile: default_graph_profile(manifest, "domain_ontology_graph_v1"),
            semantic_layers: vec![semantic_layer(
                "semantic_module_layer",
                "Capture reusable terms, aliases, constraints, mappings, and graph-profile guidance for other capsules.",
                ["terms", "aliases", "constraints", "mappings", "graph profiles"],
                ["domain sources", "expert review", "schema constraints"],
                "Let PRAXIS and DIALECTICA reuse a reviewed semantic module across compatible capsules.",
            )],
            suggested_node_classes: strings(["concept", "institution", "policy_instrument", "source"]),
            suggested_edge_classes: strings(["belongs_to_frame", "regulated_by", "mentions"]),
            reasoning_lenses: strings([
                "semantic compatibility",
                "mapping review",
                "constraint check",
            ]),
            extraction_questions: strings([
                "Which local terms can be reused across capsules?",
                "Which mappings or aliases require expert review before inheritance?",
            ]),
            praxis_context_guidance: strings([
                "Use graph/ontology capsules as semantic modules with compatibility checks.",
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
                "actor_claim_temporal_graph",
                "Capture the situation's actors, claims, events, risks, decisions, and valid-time state.",
                ["actors", "claims", "events", "risks", "decisions", "temporal status"],
                ["source spans", "temporal ledger", "review actions"],
                "Use the source-backed situation graph to guide PRAXIS analysis and warnings.",
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
        ]);
        blueprint.reasoning_lenses = strings([
            "stakeholder analysis",
            "decision-clock analysis",
            "sourceability check",
            "temporal freshness check",
        ]);
        blueprint.extraction_questions = strings([
            "Which actors, institutions, claims, risks, and decisions define the situation?",
            "Which claims are current, stale, superseded, forecast, contested, or unknown?",
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
