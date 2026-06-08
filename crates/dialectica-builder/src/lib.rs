//! Local document-to-capsule builder.
//!
//! This crate creates a deterministic build source from local text-like files,
//! promotes it through the same review/proposal/compiler contracts used by
//! fixture builds, and writes the capsule artifacts PRAXIS can import.

use std::{
    collections::BTreeSet,
    error::Error,
    fmt::{Display, Formatter},
    fs,
    path::{Path, PathBuf},
    time::{SystemTime, UNIX_EPOCH},
};

use dialectica_compiler::{
    compile_from_parts, export_praxis_context_pack, write_capsule_archive, CompilerError,
};
use dialectica_extractor::{
    route_review_gates, BuildMode, CapsuleBuildRequest, CapsuleType, ExtractionProposal,
    ExtractionRun, ModelInvocationReceipt, PromptInjectionRisk, ProposalKind, ProposalSet,
    ReviewDecisionStatus, ReviewPolicy, ReviewTrigger, ReviewTriggerLevel, ReviewerDecision,
    ReviewerDecisionSet, SourceDocument, SourcePack, SourceSpan, TypeInference,
    TypeInferenceSource, EXTRACTOR_CONTRACT_VERSION,
};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};

const LOCAL_BUILDER_ID: &str = "dialectica-local-document-builder/0.1.0";

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BuildDocumentsOptions {
    pub input_dir: PathBuf,
    pub output_dir: PathBuf,
    pub capsule_type: CapsuleType,
    pub build_mode: BuildMode,
    pub title: Option<String>,
    pub workflow: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BuildDocumentsReceipt {
    pub capsule_id: String,
    pub capsule_type: String,
    pub title: String,
    pub workflow: String,
    pub package_dir: PathBuf,
    pub archive_path: PathBuf,
    pub context_pack_path: PathBuf,
    pub praxis_import_path: PathBuf,
    pub build_request_path: PathBuf,
    pub source_pack_path: PathBuf,
    pub proposal_dir: PathBuf,
    pub review_decision_path: PathBuf,
    pub source_document_count: usize,
    pub source_span_count: usize,
    pub skipped_file_count: usize,
    pub proposal_count: usize,
    pub decision_count: usize,
    pub caveated_record_count: usize,
    pub bundle_digest: String,
    pub archive_digest: String,
}

#[derive(Debug)]
pub enum BuilderError {
    Io(std::io::Error),
    Json(serde_json::Error),
    Compiler(CompilerError),
    InvalidInput(String),
}

impl Display for BuilderError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(error) => write!(formatter, "{error}"),
            Self::Json(error) => write!(formatter, "{error}"),
            Self::Compiler(error) => write!(formatter, "{error}"),
            Self::InvalidInput(message) => formatter.write_str(message),
        }
    }
}

impl Error for BuilderError {}

impl From<std::io::Error> for BuilderError {
    fn from(error: std::io::Error) -> Self {
        Self::Io(error)
    }
}

impl From<serde_json::Error> for BuilderError {
    fn from(error: serde_json::Error) -> Self {
        Self::Json(error)
    }
}

impl From<CompilerError> for BuilderError {
    fn from(error: CompilerError) -> Self {
        Self::Compiler(error)
    }
}

pub fn build_documents_capsule(
    options: &BuildDocumentsOptions,
) -> Result<BuildDocumentsReceipt, BuilderError> {
    if !options.input_dir.is_dir() {
        return Err(BuilderError::InvalidInput(format!(
            "input directory {} does not exist",
            options.input_dir.display()
        )));
    }
    if options.workflow.trim().is_empty() {
        return Err(BuilderError::InvalidInput(
            "workflow must not be empty".to_owned(),
        ));
    }

    let collected = collect_document_files(&options.input_dir)?;
    if collected.supported.is_empty() {
        return Err(BuilderError::InvalidInput(format!(
            "no supported text-like documents found in {}",
            options.input_dir.display()
        )));
    }

    let now = current_stamp();
    let title = options.title.clone().unwrap_or_else(|| {
        title_case(
            options
                .input_dir
                .file_name()
                .and_then(|name| name.to_str())
                .unwrap_or("Dialectica capsule"),
        )
    });
    let seed_digest = digest_strings(
        &collected
            .supported
            .iter()
            .map(|path| path.display().to_string())
            .collect::<Vec<_>>(),
    );
    let request_id = format!(
        "build_{}_{}",
        options.capsule_type.as_str(),
        &seed_digest.trim_start_matches("sha256:")[..12]
    );

    let mut source_documents = Vec::new();
    let mut source_spans = Vec::new();
    let mut source_summaries = Vec::new();
    for (index, path) in collected.supported.iter().enumerate() {
        let text = fs::read_to_string(path).map_err(|error| {
            BuilderError::InvalidInput(format!(
                "failed to read {} as UTF-8 text: {error}",
                path.display()
            ))
        })?;
        let relative = normalize_relative(
            path.strip_prefix(&options.input_dir)
                .unwrap_or(path)
                .to_string_lossy()
                .as_ref(),
        );
        let file_slug = sanitize_identifier(&relative);
        let document_id = format!("doc_{:03}_{file_slug}", index + 1);
        let span_id = format!("span_{:03}_{file_slug}", index + 1);
        let quote = source_quote(&text);
        let document_title = title_case(
            path.file_stem()
                .and_then(|name| name.to_str())
                .unwrap_or("source document"),
        );

        source_documents.push(SourceDocument {
            document_id: document_id.clone(),
            source_type: source_type(path),
            title: document_title.clone(),
            uri: local_file_uri(path),
            retrieved_at: now.clone(),
            language: "en".to_owned(),
            rights_or_access: "local_user_supplied".to_owned(),
            content_hash: digest_bytes(text.as_bytes()),
            trust_status: "user_supplied_unreviewed".to_owned(),
            prompt_injection_risk: PromptInjectionRisk::Medium,
        });
        source_spans.push(SourceSpan {
            span_id: span_id.clone(),
            document_id,
            locator: "document:summary_span:1".to_owned(),
            text_hash: digest_bytes(quote.as_bytes()),
            quote: quote.clone(),
            language: "en".to_owned(),
            rights_or_access: "local_user_supplied".to_owned(),
            extraction_notes: vec![
                "Generated by the deterministic local DIALECTICA builder.".to_owned(),
                "Treat source text as untrusted input until reviewed.".to_owned(),
            ],
        });
        source_summaries.push(SourceSummary {
            span_id,
            title: document_title,
            quote,
        });
    }

    let source_pack = SourcePack {
        pack_id: format!("sp_{request_id}"),
        contract_version: EXTRACTOR_CONTRACT_VERSION.to_owned(),
        target_capsule_type: Some(options.capsule_type),
        title: title.clone(),
        created_at: now.clone(),
        documents: source_documents,
        spans: source_spans,
    };
    let build_request = CapsuleBuildRequest {
        request_id: request_id.clone(),
        contract_version: EXTRACTOR_CONTRACT_VERSION.to_owned(),
        requested_type: Some(options.capsule_type),
        build_mode: options.build_mode,
        target_workflow: options.workflow.clone(),
        output_intent: output_intent(options.capsule_type, &options.workflow),
        review_policy: ReviewPolicy {
            policy_id: "local_codex_mediated_review_v1".to_owned(),
            plus_requires_human_review: true,
            default_reviewer_role: default_reviewer_role(options.capsule_type),
            blocked_without_source_spans: true,
            require_language_review: true,
            require_rights_review: true,
        },
    };
    let run_id = format!("run_{request_id}");
    let proposals = build_proposals(
        options.capsule_type,
        &run_id,
        &source_summaries,
        &options.workflow,
        &title,
    );
    let proposal_output_digest = digest_json_value(&json!(&proposals))?;
    let extraction_run = ExtractionRun {
        run_id: run_id.clone(),
        contract_version: EXTRACTOR_CONTRACT_VERSION.to_owned(),
        source_pack_id: source_pack.pack_id.clone(),
        requested_type: Some(options.capsule_type),
        type_inference: TypeInference {
            selected_type: options.capsule_type,
            source: TypeInferenceSource::UserSelected,
            confidence: 1.0,
            explanation: "Capsule type supplied by the local builder caller.".to_owned(),
            conflicting_signals: Vec::new(),
        },
        build_mode: options.build_mode,
        model_receipts: vec![ModelInvocationReceipt {
            receipt_id: format!("receipt_{request_id}"),
            provider: "dialectica-local".to_owned(),
            model: LOCAL_BUILDER_ID.to_owned(),
            prompt_template_id: "local_document_capsule_seed_v1".to_owned(),
            input_digest: digest_json_value(&json!(&source_pack))?,
            output_digest: proposal_output_digest,
            created_at: now.clone(),
            fixture_mode: false,
        }],
    };
    let proposal_set = ProposalSet {
        extraction_run,
        proposals,
    };
    let decision_set = auto_decision_set(&build_request, &proposal_set, options.capsule_type, &now);

    let package_dir = options.output_dir.join("package");
    let archive_path = options.output_dir.join(format!("{request_id}.capsule"));
    let context_pack_path = options.output_dir.join("praxis-context-pack.json");
    let praxis_import_path = options.output_dir.join("praxis-import.json");
    let build_source_dir = options.output_dir.join("build-source");

    fs::create_dir_all(&options.output_dir)?;
    write_build_source(
        &build_source_dir,
        &build_request,
        &source_pack,
        &proposal_set,
        &decision_set,
    )?;

    let compile_receipt = compile_from_parts(
        &build_request,
        &source_pack,
        &proposal_set,
        &decision_set,
        &package_dir,
    )?;
    let archive_receipt = write_capsule_archive(&package_dir, &archive_path)?;
    let context_pack = export_praxis_context_pack(&package_dir, &options.workflow)?;
    write_json(&context_pack_path, &context_pack)?;
    write_praxis_import(
        &praxis_import_path,
        &compile_receipt.capsule_id,
        options,
        &title,
        &package_dir,
        &archive_path,
        &context_pack_path,
        &build_source_dir.join("source-pack/source_pack.json"),
        &compile_receipt.bundle_digest,
        &archive_receipt.archive_digest,
    )?;

    Ok(BuildDocumentsReceipt {
        capsule_id: compile_receipt.capsule_id,
        capsule_type: options.capsule_type.as_str().to_owned(),
        title,
        workflow: options.workflow.clone(),
        package_dir,
        archive_path,
        context_pack_path,
        praxis_import_path,
        build_request_path: build_source_dir.join("build_request.json"),
        source_pack_path: build_source_dir.join("source-pack/source_pack.json"),
        proposal_dir: build_source_dir.join("proposals"),
        review_decision_path: build_source_dir.join("review-decisions/decision_set.json"),
        source_document_count: source_pack.documents.len(),
        source_span_count: source_pack.spans.len(),
        skipped_file_count: collected.skipped.len(),
        proposal_count: proposal_set.proposals.len(),
        decision_count: decision_set.decisions.len(),
        caveated_record_count: compile_receipt.caveated_record_count,
        bundle_digest: compile_receipt.bundle_digest,
        archive_digest: archive_receipt.archive_digest,
    })
}

pub fn parse_capsule_type(value: &str) -> Result<CapsuleType, BuilderError> {
    match value.trim().to_ascii_lowercase().replace('-', "_").as_str() {
        "user" => Ok(CapsuleType::User),
        "situation" => Ok(CapsuleType::Situation),
        "tool" => Ok(CapsuleType::Tool),
        "output" => Ok(CapsuleType::Output),
        other => Err(BuilderError::InvalidInput(format!(
            "unknown capsule type '{other}'; expected user, situation, tool, or output"
        ))),
    }
}

pub fn parse_build_mode(value: &str) -> Result<BuildMode, BuilderError> {
    match value.trim().to_ascii_lowercase().replace('-', "_").as_str() {
        "auto_draft" | "auto" | "draft" => Ok(BuildMode::AutoDraft),
        "assisted" => Ok(BuildMode::Assisted),
        "plus_promoted" | "plus" | "promoted" => Ok(BuildMode::PlusPromoted),
        other => Err(BuilderError::InvalidInput(format!(
            "unknown build mode '{other}'; expected auto-draft, assisted, or plus-promoted"
        ))),
    }
}

fn build_proposals(
    capsule_type: CapsuleType,
    run_id: &str,
    source_summaries: &[SourceSummary],
    workflow: &str,
    title: &str,
) -> Vec<ExtractionProposal> {
    let mut proposals = Vec::new();
    for (index, summary) in source_summaries.iter().enumerate() {
        let object_slug = sanitize_identifier(&summary.title);
        proposals.push(proposal(
            capsule_type,
            run_id,
            ProposalKind::Claim,
            format!("prop_claim_{:03}_{object_slug}", index + 1),
            format!("clm_{:03}_{object_slug}", index + 1),
            vec![summary.span_id.clone()],
            0.62,
            "Deterministic local extraction from one source span; requires human interpretation before institutional use.",
            json!({
                "claim_text": format!("{}: {}", summary.title, compact_sentence(&summary.quote)),
                "trust_layer": "T2",
                "temporal_status": "unknown"
            }),
            vec![gate(
                "material_claim",
                default_reviewer_role(capsule_type),
                "Claim enters the PRAXIS context pack and should be source-checked.",
            )],
        ));
        proposals.push(proposal(
            capsule_type,
            run_id,
            ProposalKind::GraphNode,
            format!("prop_node_{:03}_{object_slug}", index + 1),
            format!("node_{:03}_{object_slug}", index + 1),
            vec![summary.span_id.clone()],
            0.7,
            "Document-derived graph node; label should be normalized by a domain reviewer.",
            json!({
                "node_type": node_type_for(capsule_type),
                "label": summary.title
            }),
            Vec::new(),
        ));
    }

    if source_summaries.len() > 1 {
        let first = &source_summaries[0];
        for (index, summary) in source_summaries.iter().enumerate().skip(1) {
            proposals.push(proposal(
                capsule_type,
                run_id,
                ProposalKind::GraphEdge,
                format!("prop_edge_{:03}", index + 1),
                format!("edge_{:03}_contextual_relation", index + 1),
                vec![first.span_id.clone(), summary.span_id.clone()],
                0.52,
                "Contextual relation inferred from co-presence in the same local source pack.",
                json!({
                    "from_node_id": format!("node_001_{}", sanitize_identifier(&first.title)),
                    "to_node_id": format!("node_{:03}_{}", index + 1, sanitize_identifier(&summary.title)),
                    "edge_type": "contextually_related_to",
                    "explanation": "Both nodes were supplied in the same capsule build and should be inspected together."
                }),
                vec![gate(
                    "semantic_edge_review",
                    "domain_reviewer",
                    "Inferred semantic graph edges must be checked before relied on.",
                )],
            ));
        }
    }

    let all_spans = source_summaries
        .iter()
        .map(|summary| summary.span_id.clone())
        .collect::<Vec<_>>();
    let first_span = all_spans.first().cloned().into_iter().collect::<Vec<_>>();

    proposals.push(proposal(
        capsule_type,
        run_id,
        ProposalKind::Episode,
        "prop_episode_source_window".to_owned(),
        "ep_source_window".to_owned(),
        all_spans.clone(),
        0.58,
        "Temporal envelope is the local document set creation window, not a verified event chronology.",
        json!({
            "episode_label": "Source pack ingestion window",
            "valid_from": "unknown",
            "status": "source_pack_scope"
        }),
        Vec::new(),
    ));
    proposals.push(proposal(
        capsule_type,
        run_id,
        ProposalKind::OntologyTerm,
        "prop_ontology_capsule_lens".to_owned(),
        format!("term_{}_semantic_lens", capsule_type.as_str()),
        first_span.clone(),
        0.66,
        "Ontology term is capsule-scoped and should be refined by the operator or expert reviewer.",
        ontology_payload(capsule_type, title),
        vec![gate(
            "ontology_term_review",
            "domain_reviewer",
            "Capsule-specific semantic terms shape how agents interpret the material.",
        )],
    ));
    proposals.push(proposal(
        capsule_type,
        run_id,
        ProposalKind::ReasoningDevice,
        "prop_device_capsule_reasoning".to_owned(),
        format!("device_{}", reasoning_device_id(capsule_type)),
        all_spans.clone(),
        0.72,
        "Reasoning device is selected from capsule type and must be checked for the domain.",
        reasoning_payload(capsule_type),
        vec![gate(
            "reasoning_device_review",
            "method_reviewer",
            "Reasoning devices guide PRAXIS outputs and need method review.",
        )],
    ));
    proposals.push(proposal(
        capsule_type,
        run_id,
        ProposalKind::Caveat,
        "prop_caveat_local_builder".to_owned(),
        "cav_local_builder_requires_review".to_owned(),
        first_span.clone(),
        0.92,
        "Operational caveat generated by DIALECTICA.",
        json!({
            "caveat_text": "This capsule was built from local user-supplied documents with deterministic extraction. Treat it as usable draft context until human review confirms source meaning, temporal status, and rights.",
            "applies_to": ["claims", "graph", "ontology", "reasoning"],
            "required_surface": "agent_context_warning"
        }),
        vec![gate(
            "caveat_gate",
            default_reviewer_role(capsule_type),
            "The caveat controls how PRAXIS should use draft context.",
        )],
    ));
    proposals.push(proposal(
        capsule_type,
        run_id,
        ProposalKind::LanguageRule,
        "prop_language_sourceability".to_owned(),
        "lang_prefer_sourced_caveated_language".to_owned(),
        first_span.clone(),
        0.9,
        "Language rule is generic and should be adapted to the team voice.",
        json!({
            "blocked_phrase": "the evidence proves",
            "approved_phrase": "the current source pack indicates",
            "reason": "Draft capsules must avoid overstating local, unreviewed evidence."
        }),
        vec![gate(
            "language_gate",
            "policy_editor",
            "Human-gated language keeps PRAXIS outputs precise and usable.",
        )],
    ));
    proposals.push(proposal(
        capsule_type,
        run_id,
        ProposalKind::RightsRule,
        "prop_rights_local_sources".to_owned(),
        "rights_local_sources_need_review".to_owned(),
        first_span.clone(),
        0.86,
        "Rights rule assumes user-supplied local documents and must be checked before external publication.",
        json!({
            "prohibition": "Do not quote or redistribute source material externally unless rights and access are confirmed.",
            "duty": "Use source-span receipts inside PRAXIS and surface access caveats in external outputs."
        }),
        vec![gate(
            "rights_gate",
            "privacy_reviewer",
            "Rights and access rules block unsafe reuse of local source material.",
        )],
    ));
    proposals.push(proposal(
        capsule_type,
        run_id,
        ProposalKind::OutputRule,
        "prop_output_capsule_contract".to_owned(),
        format!("output_{}_requires_receipts", workflow_slug(workflow)),
        all_spans,
        0.84,
        "Output rule is generated from the selected workflow and should be customized by the operator.",
        output_rule_payload(capsule_type, workflow),
        vec![gate(
            "output_gate",
            "policy_editor",
            "Output rules govern what PRAXIS may compose from the capsule.",
        )],
    ));

    proposals
}

fn auto_decision_set(
    build_request: &CapsuleBuildRequest,
    proposal_set: &ProposalSet,
    capsule_type: CapsuleType,
    now: &str,
) -> ReviewerDecisionSet {
    let mut seen = BTreeSet::new();
    let mut decisions = Vec::new();
    for gate in route_review_gates(build_request, proposal_set) {
        if !seen.insert(gate.proposal_id.clone()) {
            continue;
        }
        decisions.push(ReviewerDecision {
            decision_id: format!("decision_{}", gate.proposal_id.trim_start_matches("prop_")),
            proposal_id: gate.proposal_id,
            object_id: gate.object_id,
            status: ReviewDecisionStatus::ApproveWithCaveats,
            reviewer_role: gate.reviewer_role,
            decided_at: now.to_owned(),
            rationale: "Codex-mediated local build accepted this proposal as draft capsule truth with explicit caveats. Expert review can replace this decision before publication.".to_owned(),
            caveats: vec![
                "Generated by the deterministic local DIALECTICA builder; not expert-reviewed.".to_owned(),
                format!(
                    "Before promoted use, a {} reviewer should confirm source meaning, temporal status, ontology fit, and output language.",
                    capsule_type.as_str()
                ),
            ],
            requested_evidence: Vec::new(),
        });
    }

    ReviewerDecisionSet {
        decision_set_id: format!(
            "reviewset_{}",
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

#[allow(clippy::too_many_arguments)]
fn proposal(
    capsule_type: CapsuleType,
    run_id: &str,
    kind: ProposalKind,
    proposal_id: String,
    object_id: String,
    source_span_ids: Vec<String>,
    confidence: f64,
    uncertainty: &str,
    payload: Value,
    review_triggers: Vec<ReviewTrigger>,
) -> ExtractionProposal {
    ExtractionProposal {
        proposal_id,
        run_id: run_id.to_owned(),
        capsule_type,
        kind,
        object_id,
        source_span_ids,
        confidence,
        uncertainty: uncertainty.to_owned(),
        payload,
        review_triggers,
    }
}

fn gate(trigger_id: &str, reviewer_role: impl Into<String>, reason: &str) -> ReviewTrigger {
    ReviewTrigger {
        trigger_id: trigger_id.to_owned(),
        level: ReviewTriggerLevel::RequiredGate,
        reviewer_role: reviewer_role.into(),
        reason: reason.to_owned(),
        blocking: true,
    }
}

fn write_build_source(
    build_source_dir: &Path,
    build_request: &CapsuleBuildRequest,
    source_pack: &SourcePack,
    proposal_set: &ProposalSet,
    decision_set: &ReviewerDecisionSet,
) -> Result<(), BuilderError> {
    write_json(&build_source_dir.join("build_request.json"), build_request)?;
    write_json(
        &build_source_dir.join("source-pack/source_pack.json"),
        source_pack,
    )?;
    write_json(
        &build_source_dir.join("proposals/extraction_run.json"),
        &proposal_set.extraction_run,
    )?;
    write_jsonl(
        &build_source_dir.join("proposals/proposals.jsonl"),
        &proposal_set.proposals,
    )?;
    write_json(
        &build_source_dir.join("review-decisions/decision_set.json"),
        decision_set,
    )?;
    Ok(())
}

#[allow(clippy::too_many_arguments)]
fn write_praxis_import(
    path: &Path,
    capsule_id: &str,
    options: &BuildDocumentsOptions,
    title: &str,
    package_dir: &Path,
    archive_path: &Path,
    context_pack_path: &Path,
    source_pack_path: &Path,
    bundle_digest: &str,
    archive_digest: &str,
) -> Result<(), BuilderError> {
    write_json(
        path,
        &json!({
            "schema_version": "praxis_capsule_import_v1",
            "capsule_id": capsule_id,
            "title": title,
            "capsule_type": options.capsule_type.as_str(),
            "workflow": options.workflow,
            "build_state": "compiled",
            "review_state": "approved_with_caveats",
            "import_mode": "local_file",
            "package_dir": package_dir,
            "manifest_path": package_dir.join("manifest.json"),
            "graph_path": package_dir.join("graph.jsonld"),
            "ladybug_projection_manifest_path": package_dir.join("graph/ladybug/projection_manifest.json"),
            "capsule_archive_path": archive_path,
            "praxis_context_pack_path": context_pack_path,
            "source_pack_path": source_pack_path,
            "bundle_digest": bundle_digest,
            "archive_digest": archive_digest,
            "cloud_handoff": {
                "storage_uri": null,
                "signed_download_url": null,
                "firestore_document_path": null
            },
            "praxis_bridge": {
                "recommended_import": "Read praxis_context_pack_path for immediate PRAXIS agent context. Store capsule_archive_path as the portable artifact. Use manifest_path and graph_path for inspection.",
                "future_cloud_shape": "Upload the .capsule artifact to Cloud Storage, store this import record in Firestore or PostgreSQL, and let PRAXIS fetch the context pack by capsule_id."
            }
        }),
    )
}

fn ontology_payload(capsule_type: CapsuleType, title: &str) -> Value {
    json!({
        "label": format!("{} semantic lens", title),
        "definition": format!(
            "Capsule-specific ontology lens for a {} capsule. It names the entities, meanings, methods, caveats, and output constraints that PRAXIS should preserve.",
            capsule_type.as_str()
        ),
        "local_scope": capsule_type.as_str(),
        "maps_to": ["dialectica:Concept", "dialectica:OntologyTerm"]
    })
}

fn reasoning_payload(capsule_type: CapsuleType) -> Value {
    let (device_id, steps) = match capsule_type {
        CapsuleType::User => (
            "user_context_interpretation_v1",
            vec![
                "Identify the user's role, institution, preferences, and constraints.",
                "Separate stable profile context from situational assumptions.",
                "Surface language, rights, and delegation caveats.",
                "Translate user context into PRAXIS-safe agent guidance.",
            ],
        ),
        CapsuleType::Situation => (
            "situation_mapping_v1",
            vec![
                "Name the issue, actors, claims, sources, dates, and uncertainties.",
                "Separate observed facts from inference and disputed interpretations.",
                "Build the semantic graph around entities, claims, relationships, and temporal caveats.",
                "Surface what PRAXIS must cite, hedge, or ask a human to review.",
            ],
        ),
        CapsuleType::Tool => (
            "expert_method_trace_v1",
            vec![
                "Describe the intellectual tool and when to apply it.",
                "Record the expert sequence of questions, distinctions, and failure modes.",
                "Bind each step to sourceable examples or review notes.",
                "Expose the method as agent guidance with caveats.",
            ],
        ),
        CapsuleType::Output => (
            "output_contract_review_v1",
            vec![
                "Name the expected deliverable and audience.",
                "Define required sections, source receipts, language rules, and forbidden moves.",
                "Attach review criteria and caveats.",
                "Make the output reusable by PRAXIS without losing provenance.",
            ],
        ),
    };
    json!({
        "device_id": device_id,
        "steps": steps
    })
}

fn output_rule_payload(capsule_type: CapsuleType, workflow: &str) -> Value {
    let required_sections = match capsule_type {
        CapsuleType::User => vec![
            "user profile",
            "institutional context",
            "preferences and constraints",
            "delegation caveats",
            "source receipts",
        ],
        CapsuleType::Situation => vec![
            "current facts",
            "contested claims",
            "actor and issue graph",
            "temporal caveats",
            "source receipts",
        ],
        CapsuleType::Tool => vec![
            "method purpose",
            "expert reasoning steps",
            "misuse warnings",
            "application checklist",
            "source receipts",
        ],
        CapsuleType::Output => vec![
            "deliverable contract",
            "required sections",
            "quality checks",
            "review gates",
            "source receipts",
        ],
    };
    json!({
        "output_type": workflow,
        "required_sections": required_sections,
        "forbidden_content": [
            "uncited material claims",
            "hidden source uncertainty",
            "unreviewed rights-sensitive quotations",
            "method conclusions without caveats"
        ]
    })
}

fn reasoning_device_id(capsule_type: CapsuleType) -> &'static str {
    match capsule_type {
        CapsuleType::User => "user_context_interpretation_v1",
        CapsuleType::Situation => "situation_mapping_v1",
        CapsuleType::Tool => "expert_method_trace_v1",
        CapsuleType::Output => "output_contract_review_v1",
    }
}

fn node_type_for(capsule_type: CapsuleType) -> &'static str {
    match capsule_type {
        CapsuleType::User => "user_context_source",
        CapsuleType::Situation => "situation_source",
        CapsuleType::Tool => "method_source",
        CapsuleType::Output => "output_source",
    }
}

fn default_reviewer_role(capsule_type: CapsuleType) -> String {
    match capsule_type {
        CapsuleType::User => "profile_reviewer",
        CapsuleType::Situation => "policy_reviewer",
        CapsuleType::Tool => "method_reviewer",
        CapsuleType::Output => "policy_editor",
    }
    .to_owned()
}

fn output_intent(capsule_type: CapsuleType, workflow: &str) -> String {
    format!(
        "Create a {} capsule for PRAXIS workflow '{}', preserving sources, caveats, ontology, reasoning, graph, language, rights, and output rules.",
        capsule_type.as_str(),
        workflow
    )
}

#[derive(Debug)]
struct SourceSummary {
    span_id: String,
    title: String,
    quote: String,
}

#[derive(Debug, Default)]
struct CollectedFiles {
    supported: Vec<PathBuf>,
    skipped: Vec<PathBuf>,
}

fn collect_document_files(input_dir: &Path) -> Result<CollectedFiles, BuilderError> {
    let mut collected = CollectedFiles::default();
    collect_document_files_inner(input_dir, &mut collected)?;
    collected.supported.sort();
    collected.skipped.sort();
    Ok(collected)
}

fn collect_document_files_inner(
    current: &Path,
    collected: &mut CollectedFiles,
) -> Result<(), BuilderError> {
    let mut entries = fs::read_dir(current)?.collect::<Result<Vec<_>, _>>()?;
    entries.sort_by_key(|entry| entry.file_name());
    for entry in entries {
        let path = entry.path();
        let name = entry.file_name().to_string_lossy().to_string();
        if path.is_dir() {
            if should_skip_dir(&name) {
                continue;
            }
            collect_document_files_inner(&path, collected)?;
        } else if path.is_file() {
            if is_supported_document(&path) {
                collected.supported.push(path);
            } else {
                collected.skipped.push(path);
            }
        }
    }
    Ok(())
}

fn should_skip_dir(name: &str) -> bool {
    matches!(
        name,
        ".git" | "target" | "node_modules" | "graphify-out" | "build-source" | "package"
    )
}

fn is_supported_document(path: &Path) -> bool {
    path.extension()
        .and_then(|extension| extension.to_str())
        .map(|extension| {
            matches!(
                extension.to_ascii_lowercase().as_str(),
                "txt" | "md" | "markdown" | "json" | "jsonl" | "csv" | "tsv"
            )
        })
        .unwrap_or(false)
}

fn source_type(path: &Path) -> String {
    path.extension()
        .and_then(|extension| extension.to_str())
        .map(|extension| match extension.to_ascii_lowercase().as_str() {
            "md" | "markdown" => "markdown",
            "json" | "jsonl" => "structured_text",
            "csv" | "tsv" => "table_text",
            _ => "text",
        })
        .unwrap_or("text")
        .to_owned()
}

fn source_quote(text: &str) -> String {
    let normalized = text
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .collect::<Vec<_>>()
        .join(" ");
    let quote = take_chars(&normalized, 900);
    if quote.is_empty() {
        "[empty source text]".to_owned()
    } else {
        quote
    }
}

fn compact_sentence(text: &str) -> String {
    let end = text
        .find(". ")
        .map(|index| index + 1)
        .unwrap_or_else(|| text.len().min(260));
    take_chars(&text[..end], 260)
}

fn take_chars(text: &str, max_chars: usize) -> String {
    let mut output = String::new();
    for character in text.chars().take(max_chars) {
        output.push(character);
    }
    output
}

fn title_case(value: &str) -> String {
    value
        .split(['_', '-', ' ', '.', '/'])
        .filter(|part| !part.is_empty())
        .map(|part| {
            let mut chars = part.chars();
            match chars.next() {
                Some(first) => first.to_uppercase().chain(chars).collect::<String>(),
                None => String::new(),
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}

fn sanitize_identifier(value: &str) -> String {
    let mut output = String::new();
    let mut last_was_separator = false;
    for character in value.chars() {
        if character.is_ascii_alphanumeric() {
            output.push(character.to_ascii_lowercase());
            last_was_separator = false;
        } else if !last_was_separator {
            output.push('_');
            last_was_separator = true;
        }
    }
    let trimmed = output.trim_matches('_').to_owned();
    if trimmed.is_empty() {
        "item".to_owned()
    } else {
        trimmed
    }
}

fn workflow_slug(value: &str) -> String {
    sanitize_identifier(value)
}

fn normalize_relative(path: &str) -> String {
    path.replace('\\', "/")
}

fn local_file_uri(path: &Path) -> String {
    format!("file://{}", normalize_relative(&path.display().to_string()))
}

fn current_stamp() -> String {
    let seconds = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_secs())
        .unwrap_or(0);
    format!("unix:{seconds}")
}

fn digest_strings(values: &[String]) -> String {
    let mut hasher = Sha256::new();
    for value in values {
        hasher.update(value.as_bytes());
        hasher.update([0]);
    }
    format!("sha256:{:x}", hasher.finalize())
}

fn digest_bytes(bytes: &[u8]) -> String {
    format!("sha256:{:x}", Sha256::digest(bytes))
}

fn digest_json_value(value: &Value) -> Result<String, BuilderError> {
    Ok(digest_bytes(&serde_json::to_vec(value)?))
}

fn write_json<T: Serialize>(path: &Path, value: &T) -> Result<(), BuilderError> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    let text = serde_json::to_string_pretty(value)?;
    fs::write(path, format!("{text}\n"))?;
    Ok(())
}

fn write_jsonl<T: Serialize>(path: &Path, values: &[T]) -> Result<(), BuilderError> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    let mut text = String::new();
    for value in values {
        text.push_str(&serde_json::to_string(value)?);
        text.push('\n');
    }
    fs::write(path, text)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::{fs, path::PathBuf};

    use dialectica_capsule::PraxisCapsulePackage;
    use dialectica_extractor::{BuildMode, CapsuleType};

    use super::{build_documents_capsule, BuildDocumentsOptions};

    #[test]
    fn local_documents_compile_to_portable_capsule_and_praxis_pack() {
        let root = fresh_temp_dir("dialectica_builder_docs");
        let input = root.join("input");
        let output = root.join("output");
        fs::create_dir_all(&input).expect("input dir should be created");
        fs::write(
            input.join("situation.md"),
            "The commission certified provisional results. The opposition disputes the timetable.",
        )
        .expect("source should write");
        fs::write(
            input.join("analyst_note.txt"),
            "Analyst caveat: do not treat the certification dispute as legal nullification.",
        )
        .expect("source should write");

        let receipt = build_documents_capsule(&BuildDocumentsOptions {
            input_dir: input,
            output_dir: output.clone(),
            capsule_type: CapsuleType::Situation,
            build_mode: BuildMode::Assisted,
            title: Some("Election Certification Dispute".to_owned()),
            workflow: "conflict_map".to_owned(),
        })
        .expect("documents should build");

        assert_eq!(receipt.source_document_count, 2);
        assert!(receipt.archive_path.is_file());
        assert!(receipt.context_pack_path.is_file());
        assert!(receipt.praxis_import_path.is_file());
        assert!(receipt.source_pack_path.is_file());

        let package =
            PraxisCapsulePackage::load_from_dir(&receipt.package_dir).expect("package should load");
        let report = package.validate();
        assert!(!report.has_errors(), "{:#?}", report.findings);

        let context_pack =
            fs::read_to_string(output.join("praxis-context-pack.json")).expect("pack should read");
        assert!(context_pack.contains("conflict_map"));
        assert!(context_pack.contains("source span"));

        let _ = fs::remove_dir_all(root);
    }

    fn fresh_temp_dir(name: &str) -> PathBuf {
        let root = std::env::temp_dir().join(format!("{}_{}", name, std::process::id()));
        let _ = fs::remove_dir_all(&root);
        root
    }
}
