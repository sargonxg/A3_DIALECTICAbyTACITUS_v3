//! Source-pack, proposal, and review-gate contracts for capsule building.
//!
//! This crate is intentionally fixture-first: it validates local source packs
//! and LLM-style proposal records without calling a model provider.

use std::{
    collections::BTreeSet,
    error::Error,
    fmt::{Display, Formatter},
    fs,
    path::Path,
};

use schemars::{schema::RootSchema, schema_for, JsonSchema};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_json::Value;

pub const EXTRACTOR_CONTRACT_VERSION: &str = "0.1.0";

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, JsonSchema,
)]
#[serde(rename_all = "snake_case")]
pub enum CapsuleType {
    User,
    Situation,
    Tool,
    Output,
}

impl CapsuleType {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::User => "user",
            Self::Situation => "situation",
            Self::Tool => "tool",
            Self::Output => "output",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum BuildMode {
    AutoDraft,
    Assisted,
    PlusPromoted,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum TypeInferenceSource {
    UserSelected,
    AiSuggested,
    Mixed,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct CapsuleBuildRequest {
    pub request_id: String,
    pub contract_version: String,
    #[serde(default)]
    pub requested_type: Option<CapsuleType>,
    pub build_mode: BuildMode,
    pub target_workflow: String,
    pub output_intent: String,
    pub review_policy: ReviewPolicy,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct ReviewPolicy {
    pub policy_id: String,
    pub plus_requires_human_review: bool,
    pub default_reviewer_role: String,
    pub blocked_without_source_spans: bool,
    pub require_language_review: bool,
    pub require_rights_review: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct TypeInference {
    pub selected_type: CapsuleType,
    pub source: TypeInferenceSource,
    pub confidence: f64,
    pub explanation: String,
    #[serde(default)]
    pub conflicting_signals: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct CapsuleBuildPlan {
    pub request_id: String,
    pub source_pack_id: String,
    pub selected_type: CapsuleType,
    pub type_inference: TypeInference,
    pub build_mode: BuildMode,
    pub source_document_count: usize,
    pub source_span_count: usize,
    pub proposal_count: usize,
    pub review_gate_count: usize,
    pub blocking_gate_count: usize,
    pub next_phase: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct SourcePack {
    pub pack_id: String,
    pub contract_version: String,
    #[serde(default)]
    pub target_capsule_type: Option<CapsuleType>,
    pub title: String,
    pub created_at: String,
    pub documents: Vec<SourceDocument>,
    pub spans: Vec<SourceSpan>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct SourceDocument {
    pub document_id: String,
    pub source_type: String,
    pub title: String,
    pub uri: String,
    pub retrieved_at: String,
    pub language: String,
    pub rights_or_access: String,
    pub content_hash: String,
    pub trust_status: String,
    pub prompt_injection_risk: PromptInjectionRisk,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum PromptInjectionRisk {
    None,
    Low,
    Medium,
    High,
    Quarantined,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct SourceSpan {
    pub span_id: String,
    pub document_id: String,
    pub locator: String,
    pub text_hash: String,
    pub quote: String,
    pub language: String,
    pub rights_or_access: String,
    #[serde(default)]
    pub extraction_notes: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct ExtractionRun {
    pub run_id: String,
    pub contract_version: String,
    pub source_pack_id: String,
    #[serde(default)]
    pub requested_type: Option<CapsuleType>,
    pub type_inference: TypeInference,
    pub build_mode: BuildMode,
    pub model_receipts: Vec<ModelInvocationReceipt>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct ModelInvocationReceipt {
    pub receipt_id: String,
    pub provider: String,
    pub model: String,
    pub prompt_template_id: String,
    pub input_digest: String,
    pub output_digest: String,
    pub created_at: String,
    pub fixture_mode: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct ProposalSet {
    pub extraction_run: ExtractionRun,
    pub proposals: Vec<ExtractionProposal>,
}

impl ProposalSet {
    pub fn load_from_dir(path: &Path) -> Result<Self, ExtractorLoadError> {
        Ok(Self {
            extraction_run: read_json(path, "extraction_run.json")?,
            proposals: read_jsonl(path, "proposals.jsonl")?,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct ExtractionProposal {
    pub proposal_id: String,
    pub run_id: String,
    pub capsule_type: CapsuleType,
    pub kind: ProposalKind,
    pub object_id: String,
    #[serde(default)]
    pub source_span_ids: Vec<String>,
    pub confidence: f64,
    pub uncertainty: String,
    pub payload: Value,
    #[serde(default)]
    pub review_triggers: Vec<ReviewTrigger>,
}

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, JsonSchema,
)]
#[serde(rename_all = "snake_case")]
pub enum ProposalKind {
    Claim,
    Episode,
    GraphNode,
    GraphEdge,
    OntologyTerm,
    ReasoningDevice,
    LanguageRule,
    Caveat,
    RightsRule,
    OutputRule,
}

impl ProposalKind {
    const fn requires_plus_review(self) -> bool {
        matches!(
            self,
            Self::Claim
                | Self::GraphEdge
                | Self::OntologyTerm
                | Self::ReasoningDevice
                | Self::LanguageRule
                | Self::Caveat
                | Self::RightsRule
                | Self::OutputRule
        )
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct ReviewTrigger {
    pub trigger_id: String,
    pub level: ReviewTriggerLevel,
    pub reviewer_role: String,
    pub reason: String,
    pub blocking: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ReviewTriggerLevel {
    Info,
    HumanReview,
    RequiredGate,
    PromotionBlocker,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct ReviewGate {
    pub gate_id: String,
    pub proposal_id: String,
    pub object_id: String,
    pub proposal_kind: ProposalKind,
    pub reviewer_role: String,
    pub reason: String,
    pub blocking: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum BuildValidationSeverity {
    Error,
    Warning,
    Info,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct BuildValidationFinding {
    pub severity: BuildValidationSeverity,
    pub code: String,
    pub path: String,
    pub message: String,
}

impl BuildValidationFinding {
    fn error(code: &str, path: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            severity: BuildValidationSeverity::Error,
            code: code.to_owned(),
            path: path.into(),
            message: message.into(),
        }
    }

    fn warning(code: &str, path: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            severity: BuildValidationSeverity::Warning,
            code: code.to_owned(),
            path: path.into(),
            message: message.into(),
        }
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct BuildValidationReport {
    pub findings: Vec<BuildValidationFinding>,
}

impl BuildValidationReport {
    pub fn has_errors(&self) -> bool {
        self.findings
            .iter()
            .any(|finding| finding.severity == BuildValidationSeverity::Error)
    }

    fn push(&mut self, finding: BuildValidationFinding) {
        self.findings.push(finding);
    }

    fn extend(&mut self, other: BuildValidationReport) {
        self.findings.extend(other.findings);
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExtractorLoadError {
    message: String,
}

impl ExtractorLoadError {
    fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }
}

impl Display for ExtractorLoadError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(&self.message)
    }
}

impl Error for ExtractorLoadError {}

pub fn load_source_pack(path: &Path) -> Result<SourcePack, ExtractorLoadError> {
    if path.is_dir() {
        read_json(path, "source_pack.json")
    } else {
        let text = fs::read_to_string(path).map_err(|error| {
            ExtractorLoadError::new(format!("failed to read {}: {error}", path.display()))
        })?;
        serde_json::from_str(&text).map_err(|error| {
            ExtractorLoadError::new(format!("failed to parse {}: {error}", path.display()))
        })
    }
}

pub fn load_build_request(path: &Path) -> Result<CapsuleBuildRequest, ExtractorLoadError> {
    let text = fs::read_to_string(path).map_err(|error| {
        ExtractorLoadError::new(format!("failed to read {}: {error}", path.display()))
    })?;
    serde_json::from_str(&text).map_err(|error| {
        ExtractorLoadError::new(format!("failed to parse {}: {error}", path.display()))
    })
}

pub fn validate_source_pack(source_pack: &SourcePack) -> BuildValidationReport {
    let mut report = BuildValidationReport::default();

    if source_pack.pack_id.trim().is_empty() {
        report.push(BuildValidationFinding::error(
            "missing_source_pack_id",
            "source_pack.pack_id",
            "Source pack requires a stable pack_id.",
        ));
    }
    if source_pack.contract_version != EXTRACTOR_CONTRACT_VERSION {
        report.push(BuildValidationFinding::warning(
            "source_pack_contract_version_mismatch",
            "source_pack.contract_version",
            format!(
                "Source pack contract version '{}' differs from expected '{}'.",
                source_pack.contract_version, EXTRACTOR_CONTRACT_VERSION
            ),
        ));
    }
    if source_pack.documents.is_empty() {
        report.push(BuildValidationFinding::error(
            "source_pack_has_no_documents",
            "source_pack.documents",
            "Source pack must contain at least one source document.",
        ));
    }
    if source_pack.spans.is_empty() {
        report.push(BuildValidationFinding::error(
            "source_pack_has_no_spans",
            "source_pack.spans",
            "Source pack must contain at least one source span.",
        ));
    }

    let mut document_ids = BTreeSet::new();
    for (index, document) in source_pack.documents.iter().enumerate() {
        let path = format!("source_pack.documents[{index}]");
        if !document_ids.insert(document.document_id.clone()) {
            report.push(BuildValidationFinding::error(
                "duplicate_source_document_id",
                format!("{path}.document_id"),
                format!("Duplicate source document id '{}'.", document.document_id),
            ));
        }
        if document.document_id.trim().is_empty()
            || document.title.trim().is_empty()
            || document.uri.trim().is_empty()
        {
            report.push(BuildValidationFinding::error(
                "incomplete_source_document_identity",
                path.clone(),
                "Source document requires document_id, title, and uri.",
            ));
        }
        if !is_sha256_digest(&document.content_hash) {
            report.push(BuildValidationFinding::error(
                "invalid_source_document_hash",
                format!("{path}.content_hash"),
                "Source document content_hash must be a sha256 digest.",
            ));
        }
        if document.rights_or_access.trim().is_empty() {
            report.push(BuildValidationFinding::error(
                "missing_source_rights",
                format!("{path}.rights_or_access"),
                "Source document rights_or_access is required.",
            ));
        }
    }

    let mut span_ids = BTreeSet::new();
    for (index, span) in source_pack.spans.iter().enumerate() {
        let path = format!("source_pack.spans[{index}]");
        if !span_ids.insert(span.span_id.clone()) {
            report.push(BuildValidationFinding::error(
                "duplicate_source_span_id",
                format!("{path}.span_id"),
                format!("Duplicate source span id '{}'.", span.span_id),
            ));
        }
        if !document_ids.contains(&span.document_id) {
            report.push(BuildValidationFinding::error(
                "unknown_span_document",
                format!("{path}.document_id"),
                format!(
                    "Source span '{}' references unknown document '{}'.",
                    span.span_id, span.document_id
                ),
            ));
        }
        if span.locator.trim().is_empty() || span.quote.trim().is_empty() {
            report.push(BuildValidationFinding::error(
                "incomplete_source_span",
                path.clone(),
                "Source span requires locator and quote.",
            ));
        }
        if !is_sha256_digest(&span.text_hash) {
            report.push(BuildValidationFinding::error(
                "invalid_source_span_hash",
                format!("{path}.text_hash"),
                "Source span text_hash must be a sha256 digest.",
            ));
        }
        if span.rights_or_access.trim().is_empty() {
            report.push(BuildValidationFinding::error(
                "missing_span_rights",
                format!("{path}.rights_or_access"),
                "Source span rights_or_access is required.",
            ));
        }
    }

    report
}

pub fn validate_build_request(request: &CapsuleBuildRequest) -> BuildValidationReport {
    let mut report = BuildValidationReport::default();

    if request.request_id.trim().is_empty() {
        report.push(BuildValidationFinding::error(
            "missing_build_request_id",
            "build_request.request_id",
            "Build request requires request_id.",
        ));
    }
    if request.contract_version != EXTRACTOR_CONTRACT_VERSION {
        report.push(BuildValidationFinding::warning(
            "build_request_contract_version_mismatch",
            "build_request.contract_version",
            format!(
                "Build request contract version '{}' differs from expected '{}'.",
                request.contract_version, EXTRACTOR_CONTRACT_VERSION
            ),
        ));
    }
    if request.target_workflow.trim().is_empty() || request.output_intent.trim().is_empty() {
        report.push(BuildValidationFinding::error(
            "missing_build_intent",
            "build_request.target_workflow",
            "Build request requires target_workflow and output_intent.",
        ));
    }
    if matches!(request.build_mode, BuildMode::PlusPromoted)
        && !request.review_policy.plus_requires_human_review
    {
        report.push(BuildValidationFinding::error(
            "plus_build_without_human_gate",
            "build_request.review_policy.plus_requires_human_review",
            "Plus/promoted builds must require human review.",
        ));
    }

    report
}

pub fn validate_proposal_set(
    source_pack: &SourcePack,
    request: &CapsuleBuildRequest,
    proposal_set: &ProposalSet,
) -> BuildValidationReport {
    let mut report = BuildValidationReport::default();
    report.extend(validate_source_pack(source_pack));
    report.extend(validate_build_request(request));

    let run = &proposal_set.extraction_run;
    if run.source_pack_id != source_pack.pack_id {
        report.push(BuildValidationFinding::error(
            "proposal_source_pack_mismatch",
            "extraction_run.source_pack_id",
            format!(
                "Extraction run references source pack '{}', but loaded source pack is '{}'.",
                run.source_pack_id, source_pack.pack_id
            ),
        ));
    }
    if run.contract_version != EXTRACTOR_CONTRACT_VERSION {
        report.push(BuildValidationFinding::warning(
            "extraction_run_contract_version_mismatch",
            "extraction_run.contract_version",
            format!(
                "Extraction run contract version '{}' differs from expected '{}'.",
                run.contract_version, EXTRACTOR_CONTRACT_VERSION
            ),
        ));
    }
    if let Some(requested_type) = request.requested_type {
        if run.type_inference.selected_type != requested_type {
            report.push(BuildValidationFinding::warning(
                "ai_type_differs_from_user_type",
                "extraction_run.type_inference.selected_type",
                format!(
                    "AI suggested '{}', but user-selected type '{}' remains authoritative.",
                    run.type_inference.selected_type.as_str(),
                    requested_type.as_str()
                ),
            ));
        }
    }
    if run.model_receipts.is_empty() {
        report.push(BuildValidationFinding::error(
            "missing_model_receipt",
            "extraction_run.model_receipts",
            "Extraction run must record at least one model invocation receipt, even in fixture mode.",
        ));
    }
    for (index, receipt) in run.model_receipts.iter().enumerate() {
        validate_receipt(receipt, index, &mut report);
    }

    if proposal_set.proposals.is_empty() {
        report.push(BuildValidationFinding::error(
            "proposal_set_empty",
            "proposals",
            "Proposal set must contain at least one extraction proposal.",
        ));
    }

    let source_span_ids = source_pack
        .spans
        .iter()
        .map(|span| span.span_id.as_str())
        .collect::<BTreeSet<_>>();
    let selected_type = request
        .requested_type
        .unwrap_or(run.type_inference.selected_type);
    let mut proposal_ids = BTreeSet::new();

    for (index, proposal) in proposal_set.proposals.iter().enumerate() {
        validate_proposal(
            proposal,
            index,
            run,
            selected_type,
            &source_span_ids,
            &mut proposal_ids,
            &mut report,
        );
    }

    if matches!(request.build_mode, BuildMode::PlusPromoted)
        || matches!(run.build_mode, BuildMode::PlusPromoted)
    {
        for (index, proposal) in proposal_set.proposals.iter().enumerate() {
            if proposal.kind.requires_plus_review() && !has_blocking_review_trigger(proposal) {
                report.push(BuildValidationFinding::error(
                    "proposal_missing_plus_review_gate",
                    format!("proposals[{index}].review_triggers"),
                    format!(
                        "Proposal '{}' of kind {:?} requires a blocking human gate for plus/promoted export.",
                        proposal.proposal_id, proposal.kind
                    ),
                ));
            }
        }
    }

    report
}

pub fn route_review_gates(
    request: &CapsuleBuildRequest,
    proposal_set: &ProposalSet,
) -> Vec<ReviewGate> {
    let mut gates = Vec::new();
    let plus_mode = matches!(request.build_mode, BuildMode::PlusPromoted)
        || matches!(
            proposal_set.extraction_run.build_mode,
            BuildMode::PlusPromoted
        );

    for proposal in &proposal_set.proposals {
        let mut routed = false;
        for trigger in &proposal.review_triggers {
            if trigger.blocking
                || matches!(
                    trigger.level,
                    ReviewTriggerLevel::RequiredGate | ReviewTriggerLevel::PromotionBlocker
                )
            {
                routed = true;
                gates.push(ReviewGate {
                    gate_id: format!("gate:{}:{}", proposal.proposal_id, trigger.trigger_id),
                    proposal_id: proposal.proposal_id.clone(),
                    object_id: proposal.object_id.clone(),
                    proposal_kind: proposal.kind,
                    reviewer_role: trigger.reviewer_role.clone(),
                    reason: trigger.reason.clone(),
                    blocking: trigger.blocking,
                });
            }
        }

        if plus_mode && proposal.kind.requires_plus_review() && !routed {
            gates.push(ReviewGate {
                gate_id: format!("gate:{}:default_plus_review", proposal.proposal_id),
                proposal_id: proposal.proposal_id.clone(),
                object_id: proposal.object_id.clone(),
                proposal_kind: proposal.kind,
                reviewer_role: request.review_policy.default_reviewer_role.clone(),
                reason: "Plus/promoted capsules require human review for this proposal kind."
                    .to_owned(),
                blocking: true,
            });
        }
    }

    gates
}

pub fn plan_capsule_build(
    request: &CapsuleBuildRequest,
    source_pack: &SourcePack,
    proposal_set: &ProposalSet,
) -> CapsuleBuildPlan {
    let type_inference = resolve_type_inference(request, &proposal_set.extraction_run);
    let review_gates = route_review_gates(request, proposal_set);
    let blocking_gate_count = review_gates.iter().filter(|gate| gate.blocking).count();

    CapsuleBuildPlan {
        request_id: request.request_id.clone(),
        source_pack_id: source_pack.pack_id.clone(),
        selected_type: type_inference.selected_type,
        type_inference,
        build_mode: request.build_mode,
        source_document_count: source_pack.documents.len(),
        source_span_count: source_pack.spans.len(),
        proposal_count: proposal_set.proposals.len(),
        review_gate_count: review_gates.len(),
        blocking_gate_count,
        next_phase: if blocking_gate_count > 0 {
            "route_human_review_before_compiler".to_owned()
        } else {
            "ready_for_draft_compiler".to_owned()
        },
    }
}

pub fn export_schema_dir(path: &Path) -> Result<(), ExtractorLoadError> {
    fs::create_dir_all(path).map_err(|error| {
        ExtractorLoadError::new(format!(
            "failed to create schema dir {}: {error}",
            path.display()
        ))
    })?;

    write_schema(
        path,
        "capsule_build_request.schema.json",
        schema_for!(CapsuleBuildRequest),
    )?;
    write_schema(
        path,
        "capsule_build_plan.schema.json",
        schema_for!(CapsuleBuildPlan),
    )?;
    write_schema(path, "source_pack.schema.json", schema_for!(SourcePack))?;
    write_schema(
        path,
        "extraction_run.schema.json",
        schema_for!(ExtractionRun),
    )?;
    write_schema(
        path,
        "extraction_proposal.schema.json",
        schema_for!(ExtractionProposal),
    )?;
    write_schema(path, "proposal_set.schema.json", schema_for!(ProposalSet))?;
    write_schema(path, "review_gate.schema.json", schema_for!(ReviewGate))?;
    Ok(())
}

fn validate_receipt(
    receipt: &ModelInvocationReceipt,
    index: usize,
    report: &mut BuildValidationReport,
) {
    let path = format!("extraction_run.model_receipts[{index}]");
    if receipt.receipt_id.trim().is_empty()
        || receipt.provider.trim().is_empty()
        || receipt.model.trim().is_empty()
        || receipt.prompt_template_id.trim().is_empty()
    {
        report.push(BuildValidationFinding::error(
            "incomplete_model_receipt",
            path.clone(),
            "Model receipt requires receipt_id, provider, model, and prompt_template_id.",
        ));
    }
    if !is_sha256_digest(&receipt.input_digest) || !is_sha256_digest(&receipt.output_digest) {
        report.push(BuildValidationFinding::error(
            "invalid_model_receipt_digest",
            path,
            "Model receipt input_digest and output_digest must be sha256 digests.",
        ));
    }
}

fn validate_proposal(
    proposal: &ExtractionProposal,
    index: usize,
    run: &ExtractionRun,
    selected_type: CapsuleType,
    source_span_ids: &BTreeSet<&str>,
    proposal_ids: &mut BTreeSet<String>,
    report: &mut BuildValidationReport,
) {
    let path = format!("proposals[{index}]");
    if !proposal_ids.insert(proposal.proposal_id.clone()) {
        report.push(BuildValidationFinding::error(
            "duplicate_proposal_id",
            format!("{path}.proposal_id"),
            format!("Duplicate proposal id '{}'.", proposal.proposal_id),
        ));
    }
    if proposal.run_id != run.run_id {
        report.push(BuildValidationFinding::error(
            "proposal_run_mismatch",
            format!("{path}.run_id"),
            format!(
                "Proposal '{}' references run '{}', but extraction run is '{}'.",
                proposal.proposal_id, proposal.run_id, run.run_id
            ),
        ));
    }
    if proposal.capsule_type != selected_type {
        report.push(BuildValidationFinding::error(
            "proposal_capsule_type_mismatch",
            format!("{path}.capsule_type"),
            format!(
                "Proposal '{}' is for '{}', but selected capsule type is '{}'.",
                proposal.proposal_id,
                proposal.capsule_type.as_str(),
                selected_type.as_str()
            ),
        ));
    }
    if proposal.object_id.trim().is_empty() {
        report.push(BuildValidationFinding::error(
            "proposal_missing_object_id",
            format!("{path}.object_id"),
            "Proposal requires object_id.",
        ));
    }
    if proposal.source_span_ids.is_empty() {
        report.push(BuildValidationFinding::error(
            "proposal_missing_source_spans",
            format!("{path}.source_span_ids"),
            "Proposal must cite at least one source span or explicit expert-note span.",
        ));
    }
    for source_span_id in &proposal.source_span_ids {
        if !source_span_ids.contains(source_span_id.as_str()) {
            report.push(BuildValidationFinding::error(
                "proposal_unknown_source_span",
                format!("{path}.source_span_ids"),
                format!(
                    "Proposal '{}' references unknown source span '{}'.",
                    proposal.proposal_id, source_span_id
                ),
            ));
        }
    }
    if !(0.0..=1.0).contains(&proposal.confidence) {
        report.push(BuildValidationFinding::error(
            "proposal_confidence_out_of_range",
            format!("{path}.confidence"),
            "Proposal confidence must be between 0 and 1.",
        ));
    }
    if proposal.uncertainty.trim().is_empty() {
        report.push(BuildValidationFinding::error(
            "proposal_missing_uncertainty",
            format!("{path}.uncertainty"),
            "Proposal uncertainty must explain what is weak, inferred, or incomplete.",
        ));
    }
    if proposal.payload.is_null()
        || proposal
            .payload
            .as_object()
            .map(|object| object.is_empty())
            .unwrap_or(false)
    {
        report.push(BuildValidationFinding::error(
            "proposal_empty_payload",
            format!("{path}.payload"),
            "Proposal payload must contain the proposed object.",
        ));
    }
}

fn resolve_type_inference(request: &CapsuleBuildRequest, run: &ExtractionRun) -> TypeInference {
    if let Some(requested_type) = request.requested_type {
        let mut conflicting_signals = run.type_inference.conflicting_signals.clone();
        if run.type_inference.selected_type != requested_type {
            conflicting_signals.push(format!(
                "ai_suggested_{}",
                run.type_inference.selected_type.as_str()
            ));
        }
        TypeInference {
            selected_type: requested_type,
            source: TypeInferenceSource::UserSelected,
            confidence: 1.0,
            explanation: "User-selected capsule type is authoritative for this build request."
                .to_owned(),
            conflicting_signals,
        }
    } else {
        run.type_inference.clone()
    }
}

fn has_blocking_review_trigger(proposal: &ExtractionProposal) -> bool {
    proposal.review_triggers.iter().any(|trigger| {
        trigger.blocking
            || matches!(
                trigger.level,
                ReviewTriggerLevel::RequiredGate | ReviewTriggerLevel::PromotionBlocker
            )
    })
}

fn is_sha256_digest(value: &str) -> bool {
    let Some(hex) = value.strip_prefix("sha256:") else {
        return false;
    };
    hex.len() == 64 && hex.chars().all(|character| character.is_ascii_hexdigit())
}

fn read_json<T: DeserializeOwned>(base: &Path, relative: &str) -> Result<T, ExtractorLoadError> {
    let path = base.join(relative);
    let text = fs::read_to_string(&path).map_err(|error| {
        ExtractorLoadError::new(format!("failed to read {}: {error}", path.display()))
    })?;
    serde_json::from_str(&text).map_err(|error| {
        ExtractorLoadError::new(format!("failed to parse {}: {error}", path.display()))
    })
}

fn read_jsonl<T: DeserializeOwned>(
    base: &Path,
    relative: &str,
) -> Result<Vec<T>, ExtractorLoadError> {
    let path = base.join(relative);
    let text = fs::read_to_string(&path).map_err(|error| {
        ExtractorLoadError::new(format!("failed to read {}: {error}", path.display()))
    })?;
    text.lines()
        .enumerate()
        .filter(|(_, line)| !line.trim().is_empty())
        .map(|(index, line)| {
            serde_json::from_str(line).map_err(|error| {
                ExtractorLoadError::new(format!(
                    "failed to parse {} line {}: {error}",
                    path.display(),
                    index + 1
                ))
            })
        })
        .collect()
}

fn write_schema(path: &Path, filename: &str, schema: RootSchema) -> Result<(), ExtractorLoadError> {
    let output = path.join(filename);
    let json = serde_json::to_string_pretty(&schema).map_err(|error| {
        ExtractorLoadError::new(format!("failed to serialize schema {filename}: {error}"))
    })?;
    fs::write(&output, format!("{json}\n")).map_err(|error| {
        ExtractorLoadError::new(format!("failed to write {}: {error}", output.display()))
    })
}

#[cfg(test)]
mod tests {
    use super::{
        plan_capsule_build, route_review_gates, validate_proposal_set, validate_source_pack,
        BuildMode, BuildValidationSeverity, CapsuleBuildRequest, CapsuleType, ExtractionProposal,
        ExtractionRun, ModelInvocationReceipt, PromptInjectionRisk, ProposalKind, ProposalSet,
        ReviewPolicy, ReviewTrigger, ReviewTriggerLevel, SourceDocument, SourcePack, SourceSpan,
        TypeInference, TypeInferenceSource, EXTRACTOR_CONTRACT_VERSION,
    };
    use serde_json::json;

    #[test]
    fn source_pack_rejects_unstable_span_references() {
        let mut source_pack = fixture_source_pack();
        source_pack.spans[0].document_id = "missing_doc".to_owned();

        let report = validate_source_pack(&source_pack);

        assert!(report.findings.iter().any(|finding| {
            finding.severity == BuildValidationSeverity::Error
                && finding.code == "unknown_span_document"
        }));
    }

    #[test]
    fn plus_promoted_proposals_require_blocking_review_gates() {
        let source_pack = fixture_source_pack();
        let request = fixture_request();
        let mut proposal_set = fixture_proposal_set();
        proposal_set.proposals[0].review_triggers.clear();

        let report = validate_proposal_set(&source_pack, &request, &proposal_set);

        assert!(report.findings.iter().any(|finding| {
            finding.severity == BuildValidationSeverity::Error
                && finding.code == "proposal_missing_plus_review_gate"
        }));
    }

    #[test]
    fn user_selected_type_overrides_ai_type_for_build_plan() {
        let source_pack = fixture_source_pack();
        let request = fixture_request();
        let mut proposal_set = fixture_proposal_set();
        proposal_set.extraction_run.type_inference.selected_type = CapsuleType::Tool;

        let plan = plan_capsule_build(&request, &source_pack, &proposal_set);

        assert_eq!(plan.selected_type, CapsuleType::Situation);
        assert_eq!(
            plan.type_inference.source,
            TypeInferenceSource::UserSelected
        );
        assert!(plan
            .type_inference
            .conflicting_signals
            .iter()
            .any(|signal| signal == "ai_suggested_tool"));
    }

    #[test]
    fn review_router_surfaces_blocking_gates() {
        let request = fixture_request();
        let proposal_set = fixture_proposal_set();

        let gates = route_review_gates(&request, &proposal_set);

        assert_eq!(gates.len(), 1);
        assert!(gates[0].blocking);
        assert_eq!(gates[0].proposal_kind, ProposalKind::Claim);
    }

    fn fixture_source_pack() -> SourcePack {
        SourcePack {
            pack_id: "srcpack_conflict_fixture".to_owned(),
            contract_version: EXTRACTOR_CONTRACT_VERSION.to_owned(),
            target_capsule_type: Some(CapsuleType::Situation),
            title: "Conflict source pack".to_owned(),
            created_at: "2026-06-08T00:00:00Z".to_owned(),
            documents: vec![SourceDocument {
                document_id: "doc_commission".to_owned(),
                source_type: "official_statement".to_owned(),
                title: "Commission statement".to_owned(),
                uri: "fixture://commission".to_owned(),
                retrieved_at: "2026-06-08T00:00:00Z".to_owned(),
                language: "en".to_owned(),
                rights_or_access: "fixture_internal".to_owned(),
                content_hash:
                    "sha256:aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"
                        .to_owned(),
                trust_status: "reviewed_fixture".to_owned(),
                prompt_injection_risk: PromptInjectionRisk::Low,
            }],
            spans: vec![SourceSpan {
                span_id: "span_commission_001".to_owned(),
                document_id: "doc_commission".to_owned(),
                locator: "paragraph:1".to_owned(),
                text_hash:
                    "sha256:bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb"
                        .to_owned(),
                quote: "The commission certified the result.".to_owned(),
                language: "en".to_owned(),
                rights_or_access: "fixture_internal".to_owned(),
                extraction_notes: vec!["fixture span".to_owned()],
            }],
        }
    }

    fn fixture_request() -> CapsuleBuildRequest {
        CapsuleBuildRequest {
            request_id: "build_conflict_fixture".to_owned(),
            contract_version: EXTRACTOR_CONTRACT_VERSION.to_owned(),
            requested_type: Some(CapsuleType::Situation),
            build_mode: BuildMode::PlusPromoted,
            target_workflow: "conflict_map".to_owned(),
            output_intent: "Build a reviewed situation capsule.".to_owned(),
            review_policy: ReviewPolicy {
                policy_id: "review_policy_fixture".to_owned(),
                plus_requires_human_review: true,
                default_reviewer_role: "policy_reviewer".to_owned(),
                blocked_without_source_spans: true,
                require_language_review: true,
                require_rights_review: true,
            },
        }
    }

    fn fixture_proposal_set() -> ProposalSet {
        ProposalSet {
            extraction_run: ExtractionRun {
                run_id: "run_conflict_fixture".to_owned(),
                contract_version: EXTRACTOR_CONTRACT_VERSION.to_owned(),
                source_pack_id: "srcpack_conflict_fixture".to_owned(),
                requested_type: Some(CapsuleType::Situation),
                type_inference: TypeInference {
                    selected_type: CapsuleType::Situation,
                    source: TypeInferenceSource::Mixed,
                    confidence: 0.94,
                    explanation: "Sources describe a live political conflict.".to_owned(),
                    conflicting_signals: Vec::new(),
                },
                build_mode: BuildMode::PlusPromoted,
                model_receipts: vec![ModelInvocationReceipt {
                    receipt_id: "receipt_fixture".to_owned(),
                    provider: "fixture".to_owned(),
                    model: "fixture-model".to_owned(),
                    prompt_template_id: "extractor_fixture_v1".to_owned(),
                    input_digest:
                        "sha256:cccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccc"
                            .to_owned(),
                    output_digest:
                        "sha256:dddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddd"
                            .to_owned(),
                    created_at: "2026-06-08T00:00:00Z".to_owned(),
                    fixture_mode: true,
                }],
            },
            proposals: vec![ExtractionProposal {
                proposal_id: "prop_claim_001".to_owned(),
                run_id: "run_conflict_fixture".to_owned(),
                capsule_type: CapsuleType::Situation,
                kind: ProposalKind::Claim,
                object_id: "claim_certification".to_owned(),
                source_span_ids: vec!["span_commission_001".to_owned()],
                confidence: 0.86,
                uncertainty: "Single-source fixture claim.".to_owned(),
                payload: json!({
                    "text": "The commission certified the result.",
                    "trust_layer": "T2"
                }),
                review_triggers: vec![ReviewTrigger {
                    trigger_id: "material_claim".to_owned(),
                    level: ReviewTriggerLevel::RequiredGate,
                    reviewer_role: "policy_reviewer".to_owned(),
                    reason: "Material situation claim enters promoted context.".to_owned(),
                    blocking: true,
                }],
            }],
        }
    }
}
