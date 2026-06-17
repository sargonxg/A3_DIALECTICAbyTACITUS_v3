//! Source-pack, proposal, and review-gate contracts for capsule building.
//!
//! This crate is intentionally fixture-first: it validates local source packs
//! and LLM-style proposal records without calling a model provider.

use std::{
    collections::{BTreeMap, BTreeSet},
    error::Error,
    fmt::{Display, Formatter},
    fs,
    path::Path,
};

use schemars::{schema::RootSchema, schema_for, JsonSchema};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_json::{json, Value};

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
pub struct ElicitationProtocol {
    pub protocol_id: String,
    pub schema_version: String,
    pub capsule_type: CapsuleType,
    pub version: String,
    pub title: String,
    pub description: String,
    pub stages: Vec<ElicitationStage>,
    pub completeness: ElicitationCompleteness,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct ElicitationStage {
    pub stage_id: String,
    pub order: u32,
    pub title: String,
    pub question_templates: Vec<String>,
    pub target_record_families: Vec<String>,
    pub completion_criteria: Vec<String>,
    #[serde(default)]
    pub follow_up_hints: Vec<ElicitationFollowUpHint>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct ElicitationFollowUpHint {
    pub hint_id: String,
    pub trigger: String,
    pub prompt: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct ElicitationCompleteness {
    pub minimum_answered_stages: usize,
    pub criteria: Vec<ElicitationCompletenessCriterion>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct ElicitationCompletenessCriterion {
    pub criterion_id: String,
    pub target_record_family: String,
    pub minimum_count: usize,
    pub description: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct ElicitationSession {
    pub session_id: String,
    pub protocol_id: String,
    pub capsule_type: CapsuleType,
    pub current_stage_id: String,
    pub answers: Vec<ElicitationAnswer>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct ElicitationAnswer {
    pub answer_id: String,
    pub stage_id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source_span_id: Option<String>,
    pub text: String,
    #[serde(default)]
    pub derived_record_counts: BTreeMap<String, usize>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct ElicitationCompletenessScore {
    pub protocol_id: String,
    pub complete_enough: bool,
    pub answered_stage_count: usize,
    pub minimum_answered_stages: usize,
    pub missing_stage_ids: Vec<String>,
    pub criterion_scores: Vec<ElicitationCriterionScore>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct ElicitationCriterionScore {
    pub criterion_id: String,
    pub target_record_family: String,
    pub observed_count: usize,
    pub minimum_count: usize,
    pub passed: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct ElicitationProposalDraft {
    pub source_pack: SourcePack,
    pub build_request: CapsuleBuildRequest,
    pub proposal_set: ProposalSet,
    pub completeness_score: ElicitationCompletenessScore,
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
pub enum ReviewDecisionStatus {
    Approve,
    ApproveWithCaveats,
    Reject,
    RequestMoreEvidence,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct ReviewerDecisionSet {
    pub decision_set_id: String,
    pub contract_version: String,
    pub extraction_run_id: String,
    pub decisions: Vec<ReviewerDecision>,
}

impl ReviewerDecisionSet {
    pub fn load_from_dir(path: &Path) -> Result<Self, ExtractorLoadError> {
        read_json(path, "decision_set.json")
    }
}

pub fn draft_reviewer_decision_set(
    request: &CapsuleBuildRequest,
    proposal_set: &ProposalSet,
    decided_at: &str,
) -> ReviewerDecisionSet {
    let mut seen = BTreeSet::new();
    let mut decisions = Vec::new();
    for gate in route_review_gates(request, proposal_set) {
        if !seen.insert(gate.proposal_id.clone()) {
            continue;
        }
        decisions.push(ReviewerDecision {
            decision_id: format!("decision_{}", gate.proposal_id.trim_start_matches("prop_")),
            proposal_id: gate.proposal_id,
            object_id: gate.object_id,
            status: ReviewDecisionStatus::ApproveWithCaveats,
            reviewer_role: gate.reviewer_role,
            decided_at: decided_at.to_owned(),
            rationale: "Draft reviewer decision generated from the review queue. Edit status, rationale, caveats, and requested evidence before promoted use.".to_owned(),
            caveats: vec![
                "Generated as an editable DIALECTICA review draft; not expert-reviewed.".to_owned(),
                "Confirm source meaning, temporal status, ontology fit, rights, and output language before promotion.".to_owned(),
            ],
            requested_evidence: Vec::new(),
        });
    }

    ReviewerDecisionSet {
        decision_set_id: format!(
            "reviewset_{}_editable",
            proposal_set
                .extraction_run
                .run_id
                .trim_start_matches("run_")
        ),
        contract_version: EXTRACTOR_CONTRACT_VERSION.to_owned(),
        extraction_run_id: proposal_set.extraction_run.run_id.clone(),
        decisions,
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct ReviewerDecision {
    pub decision_id: String,
    pub proposal_id: String,
    pub object_id: String,
    pub status: ReviewDecisionStatus,
    pub reviewer_role: String,
    pub decided_at: String,
    pub rationale: String,
    #[serde(default)]
    pub caveats: Vec<String>,
    #[serde(default)]
    pub requested_evidence: Vec<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum PromotionBasis {
    ReviewerDecision,
    DeterministicNoGate,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct PromotedRecordSet {
    pub request_id: String,
    pub source_pack_id: String,
    pub extraction_run_id: String,
    pub selected_type: CapsuleType,
    pub build_mode: BuildMode,
    pub ready_for_compiler: bool,
    pub required_decision_count: usize,
    pub decision_count: usize,
    pub promoted_records: Vec<PromotedRecord>,
    pub rejected_proposal_ids: Vec<String>,
    pub evidence_requested_proposal_ids: Vec<String>,
    pub caveated_record_count: usize,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct PromotedRecord {
    pub record_id: String,
    pub source_proposal_id: String,
    pub object_id: String,
    pub kind: ProposalKind,
    pub capsule_type: CapsuleType,
    pub promotion_basis: PromotionBasis,
    pub source_span_ids: Vec<String>,
    pub confidence: f64,
    pub uncertainty: String,
    pub payload: Value,
    #[serde(default)]
    pub review_decision_id: Option<String>,
    #[serde(default)]
    pub review_status: Option<ReviewDecisionStatus>,
    #[serde(default)]
    pub caveats: Vec<String>,
    #[serde(default)]
    pub reviewer_role: Option<String>,
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

pub fn load_elicitation_protocol(path: &Path) -> Result<ElicitationProtocol, ExtractorLoadError> {
    let text = fs::read_to_string(path).map_err(|error| {
        ExtractorLoadError::new(format!("failed to read {}: {error}", path.display()))
    })?;
    serde_json::from_str(&text).map_err(|error| {
        ExtractorLoadError::new(format!("failed to parse {}: {error}", path.display()))
    })
}

pub fn score_elicitation_session(
    protocol: &ElicitationProtocol,
    session: &ElicitationSession,
) -> ElicitationCompletenessScore {
    let answered_stage_ids = session
        .answers
        .iter()
        .filter(|answer| !answer.text.trim().is_empty())
        .map(|answer| answer.stage_id.as_str())
        .collect::<BTreeSet<_>>();
    let required_stage_ids = protocol
        .stages
        .iter()
        .map(|stage| stage.stage_id.as_str())
        .collect::<BTreeSet<_>>();
    let missing_stage_ids = required_stage_ids
        .difference(&answered_stage_ids)
        .map(|stage_id| (*stage_id).to_owned())
        .collect::<Vec<_>>();
    let mut observed_counts = BTreeMap::<String, usize>::new();
    for answer in &session.answers {
        for (family, count) in &answer.derived_record_counts {
            *observed_counts.entry(family.clone()).or_default() += *count;
        }
    }
    let criterion_scores = protocol
        .completeness
        .criteria
        .iter()
        .map(|criterion| {
            let observed_count = observed_counts
                .get(&criterion.target_record_family)
                .copied()
                .unwrap_or_default();
            ElicitationCriterionScore {
                criterion_id: criterion.criterion_id.clone(),
                target_record_family: criterion.target_record_family.clone(),
                observed_count,
                minimum_count: criterion.minimum_count,
                passed: observed_count >= criterion.minimum_count,
            }
        })
        .collect::<Vec<_>>();
    let answered_stage_count = answered_stage_ids.len();
    let complete_enough = answered_stage_count >= protocol.completeness.minimum_answered_stages
        && criterion_scores.iter().all(|score| score.passed);

    ElicitationCompletenessScore {
        protocol_id: protocol.protocol_id.clone(),
        complete_enough,
        answered_stage_count,
        minimum_answered_stages: protocol.completeness.minimum_answered_stages,
        missing_stage_ids,
        criterion_scores,
    }
}

pub fn draft_elicitation_proposals(
    protocol: &ElicitationProtocol,
    session: &ElicitationSession,
    build_mode: BuildMode,
    target_workflow: &str,
    output_intent: &str,
    created_at: &str,
) -> ElicitationProposalDraft {
    let completeness_score = score_elicitation_session(protocol, session);
    let capsule_type = protocol.capsule_type;
    let session_slug = stable_id_part(&session.session_id);
    let document_id = format!("doc_{session_slug}_transcript");
    let transcript_text = session
        .answers
        .iter()
        .map(|answer| answer.text.as_str())
        .collect::<Vec<_>>()
        .join("\n\n");

    let source_pack = SourcePack {
        pack_id: format!("srcpack_{session_slug}"),
        contract_version: EXTRACTOR_CONTRACT_VERSION.to_owned(),
        target_capsule_type: Some(capsule_type),
        title: format!("{} Transcript", protocol.title),
        created_at: created_at.to_owned(),
        documents: vec![SourceDocument {
            document_id: document_id.clone(),
            source_type: "elicitation_transcript".to_owned(),
            title: format!("{} session {}", protocol.title, session.session_id),
            uri: format!("dialectica://elicitation-sessions/{}", session.session_id),
            retrieved_at: created_at.to_owned(),
            language: "en".to_owned(),
            rights_or_access: "operator_provided_transcript".to_owned(),
            content_hash: digest_text(&transcript_text),
            trust_status: "transcript_source_material".to_owned(),
            prompt_injection_risk: PromptInjectionRisk::Low,
        }],
        spans: session
            .answers
            .iter()
            .map(|answer| SourceSpan {
                span_id: stable_answer_span_id(answer),
                document_id: document_id.clone(),
                locator: format!("stage:{};answer:{}", answer.stage_id, answer.answer_id),
                text_hash: digest_text(&answer.text),
                quote: answer.text.clone(),
                language: "en".to_owned(),
                rights_or_access: "operator_provided_transcript".to_owned(),
                extraction_notes: vec![
                    format!("elicitation_stage:{}", answer.stage_id),
                    format!("answer_id:{}", answer.answer_id),
                ],
            })
            .collect(),
    };

    let review_policy = ReviewPolicy {
        policy_id: format!("review_policy_{session_slug}"),
        plus_requires_human_review: matches!(build_mode, BuildMode::PlusPromoted),
        default_reviewer_role: default_reviewer_role(capsule_type).to_owned(),
        blocked_without_source_spans: true,
        require_language_review: true,
        require_rights_review: true,
    };
    let build_request = CapsuleBuildRequest {
        request_id: format!("build_{session_slug}"),
        contract_version: EXTRACTOR_CONTRACT_VERSION.to_owned(),
        requested_type: Some(capsule_type),
        build_mode,
        target_workflow: target_workflow.to_owned(),
        output_intent: output_intent.to_owned(),
        review_policy,
    };
    let run_id = format!("run_{session_slug}");
    let proposals = if completeness_score.complete_enough {
        session
            .answers
            .iter()
            .filter_map(|answer| {
                proposal_from_elicitation_answer(protocol, session, answer, &run_id)
            })
            .collect()
    } else {
        Vec::new()
    };
    let proposal_digest = proposals
        .iter()
        .map(|proposal| proposal.proposal_id.as_str())
        .collect::<Vec<_>>()
        .join("\n");
    let extraction_run = ExtractionRun {
        run_id: run_id.clone(),
        contract_version: EXTRACTOR_CONTRACT_VERSION.to_owned(),
        source_pack_id: source_pack.pack_id.clone(),
        requested_type: Some(capsule_type),
        type_inference: TypeInference {
            selected_type: capsule_type,
            source: TypeInferenceSource::UserSelected,
            confidence: 1.0,
            explanation: "Protocol capsule_type is authoritative for elicitation drafts."
                .to_owned(),
            conflicting_signals: Vec::new(),
        },
        build_mode,
        model_receipts: vec![ModelInvocationReceipt {
            receipt_id: format!("receipt_{run_id}"),
            provider: "dialectica_fixture".to_owned(),
            model: "deterministic_elicitation_mapper_v1".to_owned(),
            prompt_template_id: protocol.protocol_id.clone(),
            input_digest: digest_text(&transcript_text),
            output_digest: digest_text(&proposal_digest),
            created_at: created_at.to_owned(),
            fixture_mode: true,
        }],
    };

    ElicitationProposalDraft {
        source_pack,
        build_request,
        proposal_set: ProposalSet {
            extraction_run,
            proposals,
        },
        completeness_score,
    }
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

pub fn validate_reviewer_decision_set(
    source_pack: &SourcePack,
    request: &CapsuleBuildRequest,
    proposal_set: &ProposalSet,
    decision_set: &ReviewerDecisionSet,
) -> BuildValidationReport {
    let mut report = validate_proposal_set(source_pack, request, proposal_set);

    if decision_set.decision_set_id.trim().is_empty() {
        report.push(BuildValidationFinding::error(
            "missing_decision_set_id",
            "review_decisions.decision_set_id",
            "Reviewer decision set requires a stable decision_set_id.",
        ));
    }
    if decision_set.contract_version != EXTRACTOR_CONTRACT_VERSION {
        report.push(BuildValidationFinding::warning(
            "decision_set_contract_version_mismatch",
            "review_decisions.contract_version",
            format!(
                "Reviewer decision set contract version '{}' differs from expected '{}'.",
                decision_set.contract_version, EXTRACTOR_CONTRACT_VERSION
            ),
        ));
    }
    if decision_set.extraction_run_id != proposal_set.extraction_run.run_id {
        report.push(BuildValidationFinding::error(
            "decision_set_run_mismatch",
            "review_decisions.extraction_run_id",
            format!(
                "Reviewer decisions reference run '{}', but proposal run is '{}'.",
                decision_set.extraction_run_id, proposal_set.extraction_run.run_id
            ),
        ));
    }

    let proposal_map = proposal_set
        .proposals
        .iter()
        .map(|proposal| (proposal.proposal_id.as_str(), proposal))
        .collect::<BTreeMap<_, _>>();
    let required_proposal_ids = route_review_gates(request, proposal_set)
        .into_iter()
        .map(|gate| gate.proposal_id)
        .collect::<BTreeSet<_>>();

    let mut decision_ids = BTreeSet::new();
    let mut decision_proposal_ids = BTreeSet::new();
    for (index, decision) in decision_set.decisions.iter().enumerate() {
        validate_decision(
            decision,
            index,
            &proposal_map,
            &mut decision_ids,
            &mut decision_proposal_ids,
            &mut report,
        );
    }

    for proposal_id in required_proposal_ids {
        if !decision_proposal_ids.contains(&proposal_id) {
            report.push(BuildValidationFinding::error(
                "missing_reviewer_decision",
                "review_decisions.decisions",
                format!(
                    "Proposal '{proposal_id}' has a blocking review gate and requires a reviewer decision."
                ),
            ));
        }
    }

    report
}

pub fn promote_records(
    request: &CapsuleBuildRequest,
    source_pack: &SourcePack,
    proposal_set: &ProposalSet,
    decision_set: &ReviewerDecisionSet,
) -> Result<PromotedRecordSet, BuildValidationReport> {
    let validation_report =
        validate_reviewer_decision_set(source_pack, request, proposal_set, decision_set);
    if validation_report.has_errors() {
        return Err(validation_report);
    }

    let selected_type = request
        .requested_type
        .unwrap_or(proposal_set.extraction_run.type_inference.selected_type);
    let required_proposal_ids = route_review_gates(request, proposal_set)
        .into_iter()
        .map(|gate| gate.proposal_id)
        .collect::<BTreeSet<_>>();
    let decisions_by_proposal = decision_set
        .decisions
        .iter()
        .map(|decision| (decision.proposal_id.as_str(), decision))
        .collect::<BTreeMap<_, _>>();

    let mut promoted_records = Vec::new();
    let mut rejected_proposal_ids = Vec::new();
    let mut evidence_requested_proposal_ids = Vec::new();

    for proposal in &proposal_set.proposals {
        match decisions_by_proposal.get(proposal.proposal_id.as_str()) {
            Some(decision) => match decision.status {
                ReviewDecisionStatus::Approve | ReviewDecisionStatus::ApproveWithCaveats => {
                    promoted_records.push(promoted_from_decision(proposal, decision));
                }
                ReviewDecisionStatus::Reject => {
                    rejected_proposal_ids.push(proposal.proposal_id.clone());
                }
                ReviewDecisionStatus::RequestMoreEvidence => {
                    evidence_requested_proposal_ids.push(proposal.proposal_id.clone());
                }
            },
            None if !required_proposal_ids.contains(&proposal.proposal_id) => {
                promoted_records.push(promoted_without_gate(proposal));
            }
            None => {}
        }
    }

    let caveated_record_count = promoted_records
        .iter()
        .filter(|record| !record.caveats.is_empty())
        .count();
    let ready_for_compiler = evidence_requested_proposal_ids.is_empty();

    Ok(PromotedRecordSet {
        request_id: request.request_id.clone(),
        source_pack_id: source_pack.pack_id.clone(),
        extraction_run_id: proposal_set.extraction_run.run_id.clone(),
        selected_type,
        build_mode: request.build_mode,
        ready_for_compiler,
        required_decision_count: required_proposal_ids.len(),
        decision_count: decision_set.decisions.len(),
        promoted_records,
        rejected_proposal_ids,
        evidence_requested_proposal_ids,
        caveated_record_count,
    })
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
    write_schema(
        path,
        "reviewer_decision_set.schema.json",
        schema_for!(ReviewerDecisionSet),
    )?;
    write_schema(
        path,
        "promoted_record_set.schema.json",
        schema_for!(PromotedRecordSet),
    )?;
    write_schema(
        path,
        "elicitation_protocol.schema.json",
        schema_for!(ElicitationProtocol),
    )?;
    write_schema(
        path,
        "elicitation_session.schema.json",
        schema_for!(ElicitationSession),
    )?;
    write_schema(
        path,
        "elicitation_completeness_score.schema.json",
        schema_for!(ElicitationCompletenessScore),
    )?;
    write_schema(
        path,
        "elicitation_proposal_draft.schema.json",
        schema_for!(ElicitationProposalDraft),
    )?;
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

fn proposal_from_elicitation_answer(
    protocol: &ElicitationProtocol,
    session: &ElicitationSession,
    answer: &ElicitationAnswer,
    run_id: &str,
) -> Option<ExtractionProposal> {
    let stage = protocol
        .stages
        .iter()
        .find(|stage| stage.stage_id == answer.stage_id)?;
    let (record_family, _) = answer
        .derived_record_counts
        .iter()
        .filter(|(_, count)| **count > 0)
        .find(|(family, _)| stage.target_record_families.contains(*family))?;
    let kind = proposal_kind_for_record_family(record_family)?;
    let session_slug = stable_id_part(&session.session_id);
    let family_slug = stable_id_part(record_family);
    let answer_slug = stable_id_part(&answer.answer_id);
    let object_id = format!(
        "{}_{}_{}",
        session.capsule_type.as_str(),
        family_slug,
        answer_slug
    );

    Some(ExtractionProposal {
        proposal_id: format!("prop_{session_slug}_{family_slug}_{answer_slug}"),
        run_id: run_id.to_owned(),
        capsule_type: session.capsule_type,
        kind,
        object_id: object_id.clone(),
        source_span_ids: vec![stable_answer_span_id(answer)],
        confidence: 0.72,
        uncertainty: "Derived deterministically from an elicitation transcript; human review must confirm the proposed record before promotion.".to_owned(),
        payload: elicitation_payload(kind, &object_id, stage, answer, record_family),
        review_triggers: vec![ReviewTrigger {
            trigger_id: format!("elicitation_{family_slug}_review"),
            level: ReviewTriggerLevel::RequiredGate,
            reviewer_role: default_reviewer_role(session.capsule_type).to_owned(),
            reason: format!(
                "Elicitation-derived {record_family} requires human review before canonical capsule promotion."
            ),
            blocking: true,
        }],
    })
}

fn proposal_kind_for_record_family(record_family: &str) -> Option<ProposalKind> {
    match record_family {
        "claim" | "source_requirement" | "citation_rule" => Some(ProposalKind::Claim),
        "episode" => Some(ProposalKind::Episode),
        "graph_node" | "graph_pattern" | "precedent" => Some(ProposalKind::GraphNode),
        "graph_edge" => Some(ProposalKind::GraphEdge),
        "ontology_term" | "salience_prior" => Some(ProposalKind::OntologyTerm),
        "reasoning_device" | "heuristic" => Some(ProposalKind::ReasoningDevice),
        "language_rule" => Some(ProposalKind::LanguageRule),
        "caveat" | "trap" => Some(ProposalKind::Caveat),
        "rights_rule" => Some(ProposalKind::RightsRule),
        "output_rule" => Some(ProposalKind::OutputRule),
        _ => None,
    }
}

fn elicitation_payload(
    kind: ProposalKind,
    object_id: &str,
    stage: &ElicitationStage,
    answer: &ElicitationAnswer,
    record_family: &str,
) -> Value {
    match kind {
        ProposalKind::ReasoningDevice => json!({
            "device_id": object_id,
            "label": stage.title,
            "record_family": record_family,
            "steps": [answer.text],
            "source_answer_id": answer.answer_id,
            "stage_id": answer.stage_id,
        }),
        ProposalKind::Caveat => json!({
            "caveat_id": object_id,
            "statement": answer.text,
            "record_family": record_family,
            "source_answer_id": answer.answer_id,
            "stage_id": answer.stage_id,
        }),
        ProposalKind::OutputRule => json!({
            "output_rule_id": object_id,
            "rule": answer.text,
            "record_family": record_family,
            "source_answer_id": answer.answer_id,
            "stage_id": answer.stage_id,
        }),
        ProposalKind::LanguageRule => json!({
            "language_rule_id": object_id,
            "rule": answer.text,
            "record_family": record_family,
            "source_answer_id": answer.answer_id,
            "stage_id": answer.stage_id,
        }),
        ProposalKind::RightsRule => json!({
            "rights_rule_id": object_id,
            "rule": answer.text,
            "record_family": record_family,
            "source_answer_id": answer.answer_id,
            "stage_id": answer.stage_id,
        }),
        ProposalKind::Claim => json!({
            "claim_id": object_id,
            "claim_text": answer.text,
            "record_family": record_family,
            "source_answer_id": answer.answer_id,
            "stage_id": answer.stage_id,
        }),
        ProposalKind::GraphNode => json!({
            "node_id": object_id,
            "label": stage.title,
            "description": answer.text,
            "record_family": record_family,
            "source_answer_id": answer.answer_id,
            "stage_id": answer.stage_id,
        }),
        ProposalKind::GraphEdge => json!({
            "edge_id": object_id,
            "description": answer.text,
            "record_family": record_family,
            "source_answer_id": answer.answer_id,
            "stage_id": answer.stage_id,
        }),
        ProposalKind::OntologyTerm => json!({
            "term_id": object_id,
            "label": stage.title,
            "definition": answer.text,
            "record_family": record_family,
            "source_answer_id": answer.answer_id,
            "stage_id": answer.stage_id,
        }),
        ProposalKind::Episode => json!({
            "episode_id": object_id,
            "label": stage.title,
            "description": answer.text,
            "record_family": record_family,
            "source_answer_id": answer.answer_id,
            "stage_id": answer.stage_id,
        }),
    }
}

fn default_reviewer_role(capsule_type: CapsuleType) -> &'static str {
    match capsule_type {
        CapsuleType::User => "user_context_reviewer",
        CapsuleType::Situation => "policy_reviewer",
        CapsuleType::Tool => "expert_method_reviewer",
        CapsuleType::Output => "output_owner",
    }
}

fn stable_answer_span_id(answer: &ElicitationAnswer) -> String {
    let value = answer.source_span_id.as_deref().unwrap_or("").trim();
    if value.is_empty() {
        format!("span_{}", stable_id_part(&answer.answer_id))
    } else {
        stable_id_part(value)
    }
}

fn stable_id_part(value: &str) -> String {
    let normalized = value
        .trim()
        .chars()
        .map(|character| {
            if character.is_ascii_alphanumeric() {
                character.to_ascii_lowercase()
            } else {
                '_'
            }
        })
        .collect::<String>();
    let compact = normalized
        .split('_')
        .filter(|part| !part.is_empty())
        .collect::<Vec<_>>()
        .join("_");
    if compact.is_empty() {
        "item".to_owned()
    } else {
        compact
    }
}

fn digest_text(value: &str) -> String {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    let mut output = String::with_capacity(64);
    for salt in 0_u8..4 {
        let mut hasher = DefaultHasher::new();
        salt.hash(&mut hasher);
        value.hash(&mut hasher);
        output.push_str(&format!("{:016x}", hasher.finish()));
    }
    format!("sha256:{output}")
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

fn validate_decision(
    decision: &ReviewerDecision,
    index: usize,
    proposal_map: &BTreeMap<&str, &ExtractionProposal>,
    decision_ids: &mut BTreeSet<String>,
    decision_proposal_ids: &mut BTreeSet<String>,
    report: &mut BuildValidationReport,
) {
    let path = format!("review_decisions.decisions[{index}]");
    if !decision_ids.insert(decision.decision_id.clone()) {
        report.push(BuildValidationFinding::error(
            "duplicate_reviewer_decision_id",
            format!("{path}.decision_id"),
            format!("Duplicate reviewer decision id '{}'.", decision.decision_id),
        ));
    }
    if !decision_proposal_ids.insert(decision.proposal_id.clone()) {
        report.push(BuildValidationFinding::error(
            "duplicate_reviewer_decision_for_proposal",
            format!("{path}.proposal_id"),
            format!(
                "Proposal '{}' has more than one reviewer decision in this set.",
                decision.proposal_id
            ),
        ));
    }
    if decision.decision_id.trim().is_empty()
        || decision.proposal_id.trim().is_empty()
        || decision.object_id.trim().is_empty()
        || decision.reviewer_role.trim().is_empty()
        || decision.decided_at.trim().is_empty()
        || decision.rationale.trim().is_empty()
    {
        report.push(BuildValidationFinding::error(
            "incomplete_reviewer_decision",
            path.clone(),
            "Reviewer decision requires decision_id, proposal_id, object_id, reviewer_role, decided_at, and rationale.",
        ));
    }
    match proposal_map.get(decision.proposal_id.as_str()) {
        Some(proposal) if proposal.object_id != decision.object_id => {
            report.push(BuildValidationFinding::error(
                "reviewer_decision_object_mismatch",
                format!("{path}.object_id"),
                format!(
                    "Decision '{}' references object '{}', but proposal '{}' targets '{}'.",
                    decision.decision_id,
                    decision.object_id,
                    proposal.proposal_id,
                    proposal.object_id
                ),
            ));
        }
        Some(_) => {}
        None => report.push(BuildValidationFinding::error(
            "reviewer_decision_unknown_proposal",
            format!("{path}.proposal_id"),
            format!(
                "Reviewer decision '{}' references unknown proposal '{}'.",
                decision.decision_id, decision.proposal_id
            ),
        )),
    }
    if matches!(decision.status, ReviewDecisionStatus::ApproveWithCaveats)
        && decision.caveats.is_empty()
    {
        report.push(BuildValidationFinding::error(
            "caveated_decision_missing_caveats",
            format!("{path}.caveats"),
            "approve_with_caveats decisions must include at least one caveat.",
        ));
    }
    if matches!(decision.status, ReviewDecisionStatus::RequestMoreEvidence)
        && decision.requested_evidence.is_empty()
    {
        report.push(BuildValidationFinding::error(
            "evidence_request_missing_requirements",
            format!("{path}.requested_evidence"),
            "request_more_evidence decisions must state what evidence is required.",
        ));
    }
}

fn promoted_from_decision(
    proposal: &ExtractionProposal,
    decision: &ReviewerDecision,
) -> PromotedRecord {
    PromotedRecord {
        record_id: format!("promoted:{}", proposal.object_id),
        source_proposal_id: proposal.proposal_id.clone(),
        object_id: proposal.object_id.clone(),
        kind: proposal.kind,
        capsule_type: proposal.capsule_type,
        promotion_basis: PromotionBasis::ReviewerDecision,
        source_span_ids: proposal.source_span_ids.clone(),
        confidence: proposal.confidence,
        uncertainty: proposal.uncertainty.clone(),
        payload: proposal.payload.clone(),
        review_decision_id: Some(decision.decision_id.clone()),
        review_status: Some(decision.status),
        caveats: decision.caveats.clone(),
        reviewer_role: Some(decision.reviewer_role.clone()),
    }
}

fn promoted_without_gate(proposal: &ExtractionProposal) -> PromotedRecord {
    PromotedRecord {
        record_id: format!("promoted:{}", proposal.object_id),
        source_proposal_id: proposal.proposal_id.clone(),
        object_id: proposal.object_id.clone(),
        kind: proposal.kind,
        capsule_type: proposal.capsule_type,
        promotion_basis: PromotionBasis::DeterministicNoGate,
        source_span_ids: proposal.source_span_ids.clone(),
        confidence: proposal.confidence,
        uncertainty: proposal.uncertainty.clone(),
        payload: proposal.payload.clone(),
        review_decision_id: None,
        review_status: None,
        caveats: Vec::new(),
        reviewer_role: None,
    }
}

#[cfg(test)]
mod tests {
    use super::{
        draft_elicitation_proposals, draft_reviewer_decision_set, plan_capsule_build,
        promote_records, route_review_gates, score_elicitation_session, validate_proposal_set,
        validate_reviewer_decision_set, validate_source_pack, BuildMode, BuildValidationSeverity,
        CapsuleBuildRequest, CapsuleType, ElicitationAnswer, ElicitationCompleteness,
        ElicitationCompletenessCriterion, ElicitationProtocol, ElicitationSession,
        ElicitationStage, ExtractionProposal, ExtractionRun, ModelInvocationReceipt,
        PromptInjectionRisk, ProposalKind, ProposalSet, ReviewDecisionStatus, ReviewPolicy,
        ReviewTrigger, ReviewTriggerLevel, SourceDocument, SourcePack, SourceSpan, TypeInference,
        TypeInferenceSource, EXTRACTOR_CONTRACT_VERSION,
    };
    use std::collections::BTreeMap;

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

    #[test]
    fn drafted_reviewer_decisions_are_editable_before_promotion() {
        let source_pack = fixture_source_pack();
        let request = fixture_request();
        let proposal_set = fixture_proposal_set();
        let mut decision_set =
            draft_reviewer_decision_set(&request, &proposal_set, "2026-06-10T00:00:00Z");

        let report =
            validate_reviewer_decision_set(&source_pack, &request, &proposal_set, &decision_set);
        assert!(!report.has_errors(), "{:#?}", report.findings);
        assert_eq!(decision_set.extraction_run_id, "run_conflict_fixture");
        assert_eq!(decision_set.decisions.len(), 1);
        assert_eq!(
            decision_set.decisions[0].status,
            ReviewDecisionStatus::ApproveWithCaveats
        );
        assert!(!decision_set.decisions[0].caveats.is_empty());

        decision_set.decisions[0].status = ReviewDecisionStatus::Approve;
        decision_set.decisions[0].caveats.clear();
        decision_set.decisions[0].rationale =
            "Human reviewer approved the source-backed claim without draft caveats.".to_owned();

        let promoted = promote_records(&request, &source_pack, &proposal_set, &decision_set)
            .expect("edited approval should promote");
        assert_eq!(promoted.promoted_records.len(), 1);
        assert_eq!(promoted.caveated_record_count, 0);
        assert!(promoted.promoted_records[0].caveats.is_empty());

        decision_set.decisions[0].status = ReviewDecisionStatus::Reject;
        decision_set.decisions[0].rationale =
            "Human reviewer rejected the claim for promoted PRAXIS use.".to_owned();

        let promoted = promote_records(&request, &source_pack, &proposal_set, &decision_set)
            .expect("edited rejection should remain compiler-ready lineage");
        assert!(promoted.promoted_records.is_empty());
        assert_eq!(promoted.rejected_proposal_ids, vec!["prop_claim_001"]);
    }

    #[test]
    fn elicitation_completeness_scores_protocol_counts() {
        let protocol = ElicitationProtocol {
            protocol_id: "tool.v1".to_owned(),
            schema_version: "elicitation_protocol_v1".to_owned(),
            capsule_type: CapsuleType::Tool,
            version: "1.0.0".to_owned(),
            title: "Tool Capsule Expert Elicitation".to_owned(),
            description: "Fixture protocol".to_owned(),
            stages: vec![
                ElicitationStage {
                    stage_id: "walkthrough".to_owned(),
                    order: 1,
                    title: "Walkthrough".to_owned(),
                    question_templates: vec!["Walk me through the method.".to_owned()],
                    target_record_families: vec!["reasoning_device".to_owned()],
                    completion_criteria: vec!["At least one concrete step.".to_owned()],
                    follow_up_hints: Vec::new(),
                },
                ElicitationStage {
                    stage_id: "traps".to_owned(),
                    order: 2,
                    title: "Traps".to_owned(),
                    question_templates: vec!["What do people get wrong?".to_owned()],
                    target_record_families: vec!["trap".to_owned()],
                    completion_criteria: vec!["Named failure cases.".to_owned()],
                    follow_up_hints: Vec::new(),
                },
            ],
            completeness: ElicitationCompleteness {
                minimum_answered_stages: 2,
                criteria: vec![
                    ElicitationCompletenessCriterion {
                        criterion_id: "devices".to_owned(),
                        target_record_family: "reasoning_device".to_owned(),
                        minimum_count: 8,
                        description: "Minimum method devices.".to_owned(),
                    },
                    ElicitationCompletenessCriterion {
                        criterion_id: "traps".to_owned(),
                        target_record_family: "trap".to_owned(),
                        minimum_count: 3,
                        description: "Minimum traps.".to_owned(),
                    },
                ],
            },
        };
        let session = ElicitationSession {
            session_id: "sess_tool_fixture".to_owned(),
            protocol_id: "tool.v1".to_owned(),
            capsule_type: CapsuleType::Tool,
            current_stage_id: "traps".to_owned(),
            answers: vec![
                ElicitationAnswer {
                    answer_id: "ans_walkthrough".to_owned(),
                    stage_id: "walkthrough".to_owned(),
                    source_span_id: Some("span_walkthrough".to_owned()),
                    text: "The expert supplied the method steps.".to_owned(),
                    derived_record_counts: BTreeMap::from([("reasoning_device".to_owned(), 10)]),
                },
                ElicitationAnswer {
                    answer_id: "ans_traps".to_owned(),
                    stage_id: "traps".to_owned(),
                    source_span_id: Some("span_traps".to_owned()),
                    text: "The expert supplied common failure modes.".to_owned(),
                    derived_record_counts: BTreeMap::from([("trap".to_owned(), 3)]),
                },
            ],
        };

        let score = score_elicitation_session(&protocol, &session);

        assert!(score.complete_enough);
        assert_eq!(score.answered_stage_count, 2);
        assert!(score.missing_stage_ids.is_empty());
        assert!(score
            .criterion_scores
            .iter()
            .all(|criterion| criterion.passed));
    }

    #[test]
    fn elicitation_session_drafts_source_pack_and_review_gated_proposals() {
        let protocol = fixture_tool_protocol();
        let session = fixture_tool_session();

        let draft = draft_elicitation_proposals(
            &protocol,
            &session,
            BuildMode::PlusPromoted,
            "decision_brief",
            "Capture the expert method for PRAXIS use.",
            "2026-06-17T00:00:00Z",
        );

        assert!(draft.completeness_score.complete_enough);
        assert_eq!(draft.source_pack.pack_id, "srcpack_sess_tool_fixture");
        assert_eq!(draft.source_pack.documents.len(), 1);
        assert_eq!(draft.source_pack.spans.len(), 2);
        assert_eq!(draft.source_pack.spans[0].span_id, "span_walkthrough");
        assert_eq!(
            draft.source_pack.spans[0].extraction_notes,
            vec!["elicitation_stage:walkthrough", "answer_id:ans_walkthrough"]
        );
        assert_eq!(draft.build_request.requested_type, Some(CapsuleType::Tool));
        assert_eq!(
            draft.proposal_set.extraction_run.source_pack_id,
            draft.source_pack.pack_id
        );
        assert_eq!(draft.proposal_set.proposals.len(), 2);
        assert!(draft
            .proposal_set
            .proposals
            .iter()
            .all(|proposal| !proposal.source_span_ids.is_empty()));
        assert!(draft.proposal_set.proposals.iter().all(|proposal| proposal
            .review_triggers
            .iter()
            .any(|trigger| trigger.blocking)));

        let report = validate_proposal_set(
            &draft.source_pack,
            &draft.build_request,
            &draft.proposal_set,
        );
        assert!(!report.has_errors(), "{:#?}", report.findings);

        let gates = route_review_gates(&draft.build_request, &draft.proposal_set);
        assert_eq!(gates.len(), draft.proposal_set.proposals.len());
    }

    #[test]
    fn elicitation_session_keeps_incomplete_answers_as_source_but_no_proposals() {
        let protocol = fixture_tool_protocol();
        let session = ElicitationSession {
            session_id: "sess_tool_incomplete".to_owned(),
            protocol_id: "tool.v1".to_owned(),
            capsule_type: CapsuleType::Tool,
            current_stage_id: "walkthrough".to_owned(),
            answers: vec![ElicitationAnswer {
                answer_id: "ans_walkthrough".to_owned(),
                stage_id: "walkthrough".to_owned(),
                source_span_id: None,
                text: "The answer describes the method but no derived records are counted."
                    .to_owned(),
                derived_record_counts: BTreeMap::new(),
            }],
        };

        let draft = draft_elicitation_proposals(
            &protocol,
            &session,
            BuildMode::Assisted,
            "decision_brief",
            "Capture the expert method for PRAXIS use.",
            "2026-06-17T00:00:00Z",
        );

        assert!(!draft.completeness_score.complete_enough);
        assert_eq!(draft.source_pack.spans.len(), 1);
        assert_eq!(draft.source_pack.spans[0].span_id, "span_ans_walkthrough");
        assert!(draft.proposal_set.proposals.is_empty());
    }

    fn fixture_tool_protocol() -> ElicitationProtocol {
        ElicitationProtocol {
            protocol_id: "tool.v1".to_owned(),
            schema_version: "elicitation_protocol_v1".to_owned(),
            capsule_type: CapsuleType::Tool,
            version: "1.0.0".to_owned(),
            title: "Tool Capsule Expert Elicitation".to_owned(),
            description: "Fixture protocol".to_owned(),
            stages: vec![
                ElicitationStage {
                    stage_id: "walkthrough".to_owned(),
                    order: 1,
                    title: "Walkthrough".to_owned(),
                    question_templates: vec!["Walk me through the method.".to_owned()],
                    target_record_families: vec!["reasoning_device".to_owned()],
                    completion_criteria: vec!["At least one concrete step.".to_owned()],
                    follow_up_hints: Vec::new(),
                },
                ElicitationStage {
                    stage_id: "traps".to_owned(),
                    order: 2,
                    title: "Traps".to_owned(),
                    question_templates: vec!["What do people get wrong?".to_owned()],
                    target_record_families: vec!["trap".to_owned(), "caveat".to_owned()],
                    completion_criteria: vec!["Named failure cases.".to_owned()],
                    follow_up_hints: Vec::new(),
                },
            ],
            completeness: ElicitationCompleteness {
                minimum_answered_stages: 2,
                criteria: vec![
                    ElicitationCompletenessCriterion {
                        criterion_id: "devices".to_owned(),
                        target_record_family: "reasoning_device".to_owned(),
                        minimum_count: 1,
                        description: "Minimum method devices.".to_owned(),
                    },
                    ElicitationCompletenessCriterion {
                        criterion_id: "traps".to_owned(),
                        target_record_family: "trap".to_owned(),
                        minimum_count: 1,
                        description: "Minimum traps.".to_owned(),
                    },
                ],
            },
        }
    }

    fn fixture_tool_session() -> ElicitationSession {
        ElicitationSession {
            session_id: "sess_tool_fixture".to_owned(),
            protocol_id: "tool.v1".to_owned(),
            capsule_type: CapsuleType::Tool,
            current_stage_id: "traps".to_owned(),
            answers: vec![
                ElicitationAnswer {
                    answer_id: "ans_walkthrough".to_owned(),
                    stage_id: "walkthrough".to_owned(),
                    source_span_id: Some("span_walkthrough".to_owned()),
                    text: "First separate the source-backed fact from the analyst inference."
                        .to_owned(),
                    derived_record_counts: BTreeMap::from([("reasoning_device".to_owned(), 1)]),
                },
                ElicitationAnswer {
                    answer_id: "ans_traps".to_owned(),
                    stage_id: "traps".to_owned(),
                    source_span_id: Some("span_traps".to_owned()),
                    text:
                        "The main trap is letting plausible leverage become an uncited conclusion."
                            .to_owned(),
                    derived_record_counts: BTreeMap::from([("trap".to_owned(), 1)]),
                },
            ],
        }
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
