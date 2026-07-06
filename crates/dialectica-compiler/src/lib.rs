//! Deterministic capsule compiler.
//!
//! The current implementation is intentionally local and fixture-backed. It
//! compiles reviewed proposal records into the canonical v3 directory package,
//! writes a deterministic `.capsule` archive, and exports the PRAXIS context
//! pack used by the first local API slice.

use std::{
    collections::{BTreeMap, BTreeSet},
    error::Error,
    fmt::{Display, Formatter},
    fs::{self, File},
    io::{Read, Write},
    path::{Path, PathBuf},
};

use dialectica_capsule::{
    CapsuleManifest, LadybugProjectionBuildReceipt, LadybugProjectionManifest,
    PraxisCapsuleManifest, PraxisCapsulePackage, PurposeProfile, PurposeStalenessPolicy,
    CAPSULE_MIME_TYPE, CAPSULE_SPEC_VERSION, LADYBUG_BUILD_RECEIPT_PATH, LADYBUG_DATABASE_PATH,
    LADYBUG_PROJECTION_MANIFEST_PATH, LADYBUG_PROJECTION_PROFILE, LADYBUG_QUERIES_PATH,
    LADYBUG_SCHEMA_PATH,
};
use dialectica_extractor::{
    load_build_request, load_source_pack, promote_records, BuildValidationReport,
    CapsuleBuildRequest, CapsuleType, PromotedRecord, PromotedRecordSet, ProposalKind, ProposalSet,
    ReviewDecisionStatus, ReviewerDecisionSet, SourceDocument, SourcePack, SourceSpan,
};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use zip::{write::SimpleFileOptions, CompressionMethod, ZipWriter};

mod capsule_diff;
mod integrity;

pub use capsule_diff::{
    diff_capsules, export_diff_schema_dir, render_change_memo, write_capsule_diff, CapsuleDiff,
    CapsuleDiffEndpoint, CapsuleDiffReceipt, CapsuleDiffSummary, RecordChange, RecordFamilyDelta,
    RecordRef, SourcePackDelta, CAPSULE_DIFF_SCHEMA_VERSION,
};
pub use integrity::{
    verify_integrity_envelope, write_integrity_envelope, IntegrityEnvelope, IntegrityLeaf,
    IntegrityScope, IntegritySignature, IntegritySignaturePayload, IntegrityVerificationReport,
    SignatureIdentity, INTEGRITY_ENVELOPE_PATH, INTEGRITY_SCHEMA_VERSION,
};

/// Current compiler identifier.
pub const COMPILER_ID: &str = "dialectica-compiler/0.2.0";
pub const DIALECTICA_PRAXIS_CONTEXT_CONTRACT_VERSION: &str = "2026-05-15.prx-dialectica-context.v1";

/// Returns true when the legacy compiler may emit a signed bundle for this manifest.
pub fn can_emit_bundle(manifest: &CapsuleManifest) -> bool {
    manifest.is_export_ready()
}

#[derive(Debug)]
pub enum CompilerError {
    Io(std::io::Error),
    Json(serde_json::Error),
    Zip(zip::result::ZipError),
    Validation(BuildValidationReport),
    InvalidInput(String),
}

impl Display for CompilerError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(error) => write!(formatter, "{error}"),
            Self::Json(error) => write!(formatter, "{error}"),
            Self::Zip(error) => write!(formatter, "{error}"),
            Self::Validation(report) => {
                let codes = report
                    .findings
                    .iter()
                    .map(|finding| finding.code.as_str())
                    .collect::<Vec<_>>()
                    .join(", ");
                write!(formatter, "capsule build validation failed: {codes}")
            }
            Self::InvalidInput(message) => formatter.write_str(message),
        }
    }
}

impl Error for CompilerError {}

impl From<std::io::Error> for CompilerError {
    fn from(error: std::io::Error) -> Self {
        Self::Io(error)
    }
}

impl From<serde_json::Error> for CompilerError {
    fn from(error: serde_json::Error) -> Self {
        Self::Json(error)
    }
}

impl From<zip::result::ZipError> for CompilerError {
    fn from(error: zip::result::ZipError) -> Self {
        Self::Zip(error)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CompileReceipt {
    pub capsule_id: String,
    pub output_dir: PathBuf,
    pub promoted_record_count: usize,
    pub rejected_record_count: usize,
    pub caveated_record_count: usize,
    pub package_file_count: usize,
    pub bundle_digest: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ArchiveReceipt {
    pub capsule_id: String,
    pub archive_path: PathBuf,
    pub entry_count: usize,
    pub entries: Vec<String>,
    pub archive_digest: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PraxisContextPack {
    pub capsule_id: String,
    pub spec_version: String,
    pub workflow: String,
    pub purpose_profile: PurposeProfile,
    pub summary: String,
    pub agent_context: String,
    pub operations: String,
    pub retrieval_records: Vec<Value>,
    pub citation_hints: Vec<String>,
    pub temporal_warnings: Vec<String>,
    pub reasoning_devices: Vec<Value>,
    pub language_profile: Value,
    pub runtime_contract: Value,
    pub output_contract: Value,
    pub graph_focus: Vec<Value>,
    pub graph_warnings: Vec<String>,
    pub capsule_health: Value,
    pub read_receipt_hints: ReadReceiptHints,
    pub forbidden_claims: Vec<String>,
    pub review_state: String,
    pub claim_ids: Vec<String>,
    pub rejected_record_ids: Vec<String>,
    pub caveats: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DialecticaPraxisContextExport {
    pub contract_version: String,
    pub package_id: String,
    pub exported_at: String,
    pub title: String,
    pub claims: Vec<DialecticaPraxisClaim>,
    pub assumptions: Vec<String>,
    pub uncertainties: Vec<String>,
    pub evidence_gaps: Vec<String>,
    pub source_receipts: Vec<DialecticaPraxisSourceReceipt>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DialecticaPraxisClaim {
    pub id: String,
    pub kind: DialecticaPraxisClaimKind,
    pub text: String,
    pub source_ids: Vec<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DialecticaPraxisClaimKind {
    Fact,
    EvidenceGap,
    ConflictingClaim,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DialecticaPraxisSourceReceipt {
    pub id: String,
    pub title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub citation: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub excerpt: Option<String>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReadReceiptHints {
    pub bundle_digest: String,
    pub source_ids: Vec<String>,
    pub claim_ids: Vec<String>,
    pub graph_node_ids: Vec<String>,
    pub graph_edge_ids: Vec<String>,
    pub reasoning_device_ids: Vec<String>,
    pub language_rule_ids: Vec<String>,
    pub agent_guidance_ids: Vec<String>,
}

pub fn compile_fixture(
    fixture_dir: &Path,
    output_dir: &Path,
) -> Result<CompileReceipt, CompilerError> {
    let build_request = load_build_request(&fixture_dir.join("build_request.json"))
        .map_err(|error| CompilerError::InvalidInput(error.to_string()))?;
    let source_pack = load_source_pack(&fixture_dir.join("source-pack/source_pack.json"))
        .map_err(|error| CompilerError::InvalidInput(error.to_string()))?;
    let proposal_set = ProposalSet::load_from_dir(&fixture_dir.join("proposals"))
        .map_err(|error| CompilerError::InvalidInput(error.to_string()))?;
    let decision_set = ReviewerDecisionSet::load_from_dir(&fixture_dir.join("review-decisions"))
        .map_err(|error| CompilerError::InvalidInput(error.to_string()))?;

    compile_from_parts(
        &build_request,
        &source_pack,
        &proposal_set,
        &decision_set,
        output_dir,
    )
}

pub fn compile_from_parts(
    build_request: &CapsuleBuildRequest,
    source_pack: &SourcePack,
    proposal_set: &ProposalSet,
    decision_set: &ReviewerDecisionSet,
    output_dir: &Path,
) -> Result<CompileReceipt, CompilerError> {
    let promoted = promote_records(build_request, source_pack, proposal_set, decision_set)
        .map_err(CompilerError::Validation)?;
    if !promoted.ready_for_compiler {
        return Err(CompilerError::InvalidInput(
            "promoted record set is not ready for compiler".to_owned(),
        ));
    }

    prepare_output_dir(output_dir)?;
    write_package(
        build_request,
        source_pack,
        proposal_set,
        decision_set,
        &promoted,
        output_dir,
    )?;
    let package = PraxisCapsulePackage::load_from_dir(output_dir)
        .map_err(|error| CompilerError::InvalidInput(error.to_string()))?;
    let report = package.validate();
    if report.has_errors() {
        return Err(CompilerError::InvalidInput(format!(
            "compiled package failed validation: {:?}",
            report.findings
        )));
    }
    write_integrity_envelope(output_dir)?;

    Ok(CompileReceipt {
        capsule_id: package.manifest.capsule_id,
        output_dir: output_dir.to_path_buf(),
        promoted_record_count: promoted.promoted_records.len(),
        rejected_record_count: promoted.rejected_proposal_ids.len(),
        caveated_record_count: promoted.caveated_record_count,
        package_file_count: list_package_files(output_dir)?.len(),
        bundle_digest: digest_directory(output_dir)?,
    })
}

pub fn write_capsule_archive(
    package_dir: &Path,
    archive_path: &Path,
) -> Result<ArchiveReceipt, CompilerError> {
    ensure_archive_outside_package(package_dir, archive_path)?;
    let package = PraxisCapsulePackage::load_from_dir(package_dir)
        .map_err(|error| CompilerError::InvalidInput(error.to_string()))?;
    let report = package.validate();
    if report.has_errors() {
        return Err(CompilerError::InvalidInput(format!(
            "cannot archive invalid package: {:?}",
            report.findings
        )));
    }

    if let Some(parent) = archive_path.parent() {
        fs::create_dir_all(parent)?;
    }
    let file = File::create(archive_path)?;
    let mut zip = ZipWriter::new(file);
    let options = SimpleFileOptions::default()
        .compression_method(CompressionMethod::Stored)
        .last_modified_time(zip::DateTime::default());
    let mut entries = list_package_files(package_dir)?;
    entries.sort();
    entries.retain(|entry| entry != "mimetype");
    entries.insert(0, "mimetype".to_owned());

    for entry in &entries {
        zip.start_file(entry, options)?;
        let mut input = File::open(package_dir.join(entry))?;
        let mut bytes = Vec::new();
        input.read_to_end(&mut bytes)?;
        zip.write_all(&bytes)?;
    }
    zip.finish()?;

    Ok(ArchiveReceipt {
        capsule_id: package.manifest.capsule_id,
        archive_path: archive_path.to_path_buf(),
        entry_count: entries.len(),
        entries,
        archive_digest: digest_file(archive_path)?,
    })
}

fn ensure_archive_outside_package(
    package_dir: &Path,
    archive_path: &Path,
) -> Result<(), CompilerError> {
    let package_root = fs::canonicalize(package_dir)?;
    let archive_parent = archive_path.parent().unwrap_or_else(|| Path::new("."));
    let archive_parent = if archive_parent.exists() {
        fs::canonicalize(archive_parent)?
    } else {
        let parent = archive_parent.parent().unwrap_or_else(|| Path::new("."));
        fs::canonicalize(parent)?
    };

    if archive_parent.starts_with(&package_root) {
        return Err(CompilerError::InvalidInput(format!(
            "archive output {} must be outside package directory {}",
            archive_path.display(),
            package_dir.display()
        )));
    }

    Ok(())
}

pub fn export_praxis_context_pack(
    package_dir: &Path,
    workflow: &str,
) -> Result<PraxisContextPack, CompilerError> {
    let package = PraxisCapsulePackage::load_from_dir(package_dir)
        .map_err(|error| CompilerError::InvalidInput(error.to_string()))?;
    let manifest = package.manifest;
    let purpose_profile = manifest.purpose_profile.clone();
    let agent_context = fs::read_to_string(package_dir.join("agent_context.md"))?;
    let operations = fs::read_to_string(package_dir.join("operations.md"))?;
    let claims = read_jsonl_values(&package_dir.join("claims.jsonl"))?;
    let sources = read_jsonl_values(&package_dir.join("evidence/sources.jsonl"))?;
    let reasoning_devices: Vec<Value> = serde_json::from_str(&fs::read_to_string(
        package_dir.join("reasoning/devices.json"),
    )?)?;
    let runtime: Value =
        serde_json::from_str(&fs::read_to_string(package_dir.join("runtime.json"))?)?;
    let review: Value =
        serde_json::from_str(&fs::read_to_string(package_dir.join("review/review.json"))?)?;
    let graph: Value =
        serde_json::from_str(&fs::read_to_string(package_dir.join("graph.jsonld"))?)?;

    let claim_ids = claims
        .iter()
        .filter_map(|claim| {
            claim
                .get("claim_id")
                .and_then(Value::as_str)
                .map(str::to_owned)
        })
        .collect::<Vec<_>>();
    let source_ids = sources
        .iter()
        .filter_map(|source| {
            source
                .get("document_id")
                .and_then(Value::as_str)
                .map(str::to_owned)
        })
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect::<Vec<_>>();
    let forbidden_claims = runtime
        .get("output_contract")
        .and_then(|contract| contract.get("forbidden_content"))
        .and_then(Value::as_array)
        .into_iter()
        .flatten()
        .filter_map(Value::as_str)
        .map(str::to_owned)
        .collect::<Vec<_>>();
    let caveats = review
        .get("caveats")
        .and_then(Value::as_array)
        .into_iter()
        .flatten()
        .filter_map(Value::as_str)
        .map(str::to_owned)
        .collect::<Vec<_>>();
    let rejected_record_ids = review
        .get("rejected_records")
        .and_then(Value::as_array)
        .into_iter()
        .flatten()
        .filter_map(|record| {
            record
                .get("object_id")
                .and_then(Value::as_str)
                .map(str::to_owned)
        })
        .collect::<Vec<_>>();
    let graph_focus = graph_focus(&graph);
    let graph_node_ids = graph_focus
        .iter()
        .filter_map(|node| node.get("id").and_then(Value::as_str).map(str::to_owned))
        .collect::<Vec<_>>();
    let graph_edge_ids = graph_focus
        .iter()
        .filter_map(|node| {
            node.get("edge_id")
                .and_then(Value::as_str)
                .map(str::to_owned)
        })
        .collect::<Vec<_>>();
    let reasoning_device_ids = reasoning_devices
        .iter()
        .filter_map(|device| {
            device
                .get("device_id")
                .and_then(Value::as_str)
                .map(str::to_owned)
        })
        .collect::<Vec<_>>();

    Ok(PraxisContextPack {
        capsule_id: manifest.capsule_id,
        spec_version: manifest.spec_version,
        workflow: workflow.to_owned(),
        purpose_profile: purpose_profile.clone(),
        summary: format!(
            "{}. Use this capsule for {} with explicit source receipts and caveats.",
            manifest.title, workflow
        ),
        agent_context,
        operations,
        retrieval_records: claims.clone(),
        citation_hints: source_ids
            .iter()
            .map(|source_id| format!("cite source receipt {source_id}"))
            .collect(),
        temporal_warnings: temporal_warnings(&claims),
        reasoning_devices,
        language_profile: runtime
            .get("language_profile")
            .cloned()
            .unwrap_or_else(|| json!({})),
        runtime_contract: runtime.clone(),
        output_contract: runtime
            .get("output_contract")
            .cloned()
            .unwrap_or_else(|| json!({})),
        graph_focus,
        graph_warnings: caveats
            .iter()
            .map(|caveat| format!("review caveat: {caveat}"))
            .collect(),
        capsule_health: json!({
            "status": "usable_with_caveats",
            "purpose_status": "complete",
            "staleness_policy": {
                "warn_after_days": purpose_profile.staleness_policy.warn_after_days,
                "refuse_after_days": purpose_profile.staleness_policy.refuse_after_days,
                "refuse_generation": false
            },
            "warning_count": caveats.len(),
            "rejected_record_count": rejected_record_ids.len()
        }),
        read_receipt_hints: ReadReceiptHints {
            bundle_digest: digest_directory(package_dir)?,
            source_ids,
            claim_ids: claim_ids.clone(),
            graph_node_ids,
            graph_edge_ids,
            reasoning_device_ids,
            language_rule_ids: runtime_array_ids(&runtime, "language_rules", "rule_id"),
            agent_guidance_ids: vec![format!("agent_guidance:{}", workflow)],
        },
        forbidden_claims,
        review_state: "approved_with_caveats".to_owned(),
        claim_ids,
        rejected_record_ids,
        caveats,
    })
}

pub fn export_praxis_import_package(
    package_dir: &Path,
) -> Result<DialecticaPraxisContextExport, CompilerError> {
    let package = PraxisCapsulePackage::load_from_dir(package_dir)
        .map_err(|error| CompilerError::InvalidInput(error.to_string()))?;
    let manifest = package.manifest;
    let claims = read_jsonl_values(&package_dir.join("claims.jsonl"))?;
    let sources = read_jsonl_values(&package_dir.join("evidence/sources.jsonl"))?;
    let review: Value =
        serde_json::from_str(&fs::read_to_string(package_dir.join("review/review.json"))?)?;
    let source_receipts = sources
        .iter()
        .filter_map(praxis_source_receipt)
        .collect::<Vec<_>>();
    let mut exported_claims = claims
        .iter()
        .filter_map(praxis_claim_from_value)
        .collect::<Vec<_>>();
    exported_claims.extend(rejected_claims_from_review(&review));
    let evidence_gaps = evidence_gaps_from_review(&review);
    exported_claims.extend(evidence_gaps.iter().enumerate().map(|(index, gap)| {
        DialecticaPraxisClaim {
            id: format!("evidence_gap_{:03}", index + 1),
            kind: DialecticaPraxisClaimKind::EvidenceGap,
            text: gap.clone(),
            source_ids: Vec::new(),
        }
    }));
    let uncertainties = review
        .get("caveats")
        .and_then(Value::as_array)
        .into_iter()
        .flatten()
        .filter_map(Value::as_str)
        .map(str::to_owned)
        .collect::<Vec<_>>();

    Ok(DialecticaPraxisContextExport {
        contract_version: DIALECTICA_PRAXIS_CONTEXT_CONTRACT_VERSION.to_owned(),
        package_id: manifest.capsule_id,
        exported_at: manifest.updated_at,
        title: manifest.title,
        claims: exported_claims,
        assumptions: Vec::new(),
        uncertainties,
        evidence_gaps,
        source_receipts,
    })
}

pub fn export_schema_dir(path: &Path) -> Result<(), CompilerError> {
    fs::create_dir_all(path)?;
    capsule_diff::export_diff_schema_dir(path)?;
    integrity::export_integrity_schema_dir(path)
}

fn write_package(
    build_request: &CapsuleBuildRequest,
    source_pack: &SourcePack,
    proposal_set: &ProposalSet,
    decision_set: &ReviewerDecisionSet,
    promoted: &PromotedRecordSet,
    output_dir: &Path,
) -> Result<(), CompilerError> {
    write_text(&output_dir.join("mimetype"), CAPSULE_MIME_TYPE)?;
    let capsule_id = format!("cap_{}", build_request.request_id);
    let created_at = source_pack.created_at.clone();
    let promoted_digest = digest_json(promoted)?;
    let manifest = PraxisCapsuleManifest {
        capsule_id: capsule_id.clone(),
        spec_version: CAPSULE_SPEC_VERSION.to_owned(),
        version: 1,
        capsule_type: capsule_type_value(promoted.selected_type).to_owned(),
        category: build_request.target_workflow.clone(),
        title: source_pack.title.clone(),
        purpose_profile: purpose_profile(build_request, source_pack),
        owner_uid: None,
        situation_id: Some(build_request.request_id.clone()),
        cores: vec!["aco".to_owned(), "capsule_specific_ontology".to_owned()],
        created_at: created_at.clone(),
        updated_at: created_at.clone(),
        provenance_root_hash: promoted_digest.clone(),
        signature: format!(
            "unsigned:fixture:{}",
            promoted_digest.trim_start_matches("sha256:")
        ),
        depends_on: Vec::new(),
        layers_present: vec![
            "evidence".to_owned(),
            "claims".to_owned(),
            "situation".to_owned(),
            "temporal".to_owned(),
            "ontology".to_owned(),
            "reasoning".to_owned(),
            "governance".to_owned(),
            "runtime".to_owned(),
        ],
    };

    let sources = source_records(source_pack);
    let claims = promoted_records_by_kind(promoted, ProposalKind::Claim);
    let episodes = promoted_records_by_kind(promoted, ProposalKind::Episode);
    let graph_nodes = promoted_records_by_kind(promoted, ProposalKind::GraphNode);
    let graph_edges = promoted_records_by_kind(promoted, ProposalKind::GraphEdge);
    let ontology_terms = promoted_records_by_kind(promoted, ProposalKind::OntologyTerm);
    let reasoning_devices = promoted_records_by_kind(promoted, ProposalKind::ReasoningDevice);
    let caveats = promoted_records_by_kind(promoted, ProposalKind::Caveat);
    let language_rules = promoted_records_by_kind(promoted, ProposalKind::LanguageRule);
    let rights_rules = promoted_records_by_kind(promoted, ProposalKind::RightsRule);
    let output_rules = promoted_records_by_kind(promoted, ProposalKind::OutputRule);

    write_json(&output_dir.join("manifest.json"), &manifest)?;
    write_jsonl_values(&output_dir.join("evidence/sources.jsonl"), &sources)?;
    write_jsonl_values(&output_dir.join("claims.jsonl"), &claim_values(&claims))?;
    write_json(
        &output_dir.join("episodes.json"),
        &episode_values(&episodes),
    )?;
    write_json(
        &output_dir.join("graph.jsonld"),
        &graph_jsonld(GraphInput {
            capsule_id: &capsule_id,
            source_pack,
            claims: &claims,
            episodes: &episodes,
            graph_nodes: &graph_nodes,
            graph_edges: &graph_edges,
            ontology_terms: &ontology_terms,
            reasoning_devices: &reasoning_devices,
            decision_set,
        }),
    )?;
    write_json(
        &output_dir.join("reasoning/devices.json"),
        &reasoning_device_values(&reasoning_devices),
    )?;
    write_json(
        &output_dir.join("reasoning/annotations.json"),
        &json!({
            "schema_version": "reasoning_annotations_v1",
            "caveats": caveat_values(&caveats),
            "source_pack_id": source_pack.pack_id,
            "extraction_run_id": proposal_set.extraction_run.run_id
        }),
    )?;
    write_json(
        &output_dir.join("reasoning/traps.json"),
        &json!({
            "schema_version": "reasoning_traps_v1",
            "traps": traps(&language_rules, &rights_rules, &caveats)
        }),
    )?;
    write_json(
        &output_dir.join("review/review.json"),
        &review_json(decision_set, promoted),
    )?;
    write_json(
        &output_dir.join("runtime.json"),
        &runtime_json(build_request, &language_rules, &rights_rules, &output_rules),
    )?;
    write_text(
        &output_dir.join("agent_context.md"),
        &agent_context_markdown(
            &manifest,
            source_pack,
            &claims,
            &reasoning_devices,
            &caveats,
            &language_rules,
            &output_rules,
        ),
    )?;
    write_text(
        &output_dir.join("operations.md"),
        &operations_markdown(&manifest),
    )?;
    write_ladybug_projection(output_dir)?;
    Ok(())
}

fn prepare_output_dir(output_dir: &Path) -> Result<(), CompilerError> {
    if output_dir.exists() {
        if !output_dir.is_dir() {
            return Err(CompilerError::InvalidInput(format!(
                "output path {} exists and is not a directory",
                output_dir.display()
            )));
        }
        if fs::read_dir(output_dir)?.next().is_some() {
            return Err(CompilerError::InvalidInput(format!(
                "output directory {} must be empty or absent",
                output_dir.display()
            )));
        }
    }
    fs::create_dir_all(output_dir)?;
    Ok(())
}

fn purpose_profile(
    build_request: &CapsuleBuildRequest,
    source_pack: &SourcePack,
) -> PurposeProfile {
    let source_titles = source_pack
        .documents
        .iter()
        .map(|document| format!("source `{}`: {}", document.document_id, document.title))
        .collect::<Vec<_>>();
    let source_scope = if source_titles.is_empty() {
        vec![
            "No source documents were supplied; compilation should fail before promotion."
                .to_owned(),
        ]
    } else {
        source_titles
    };

    PurposeProfile {
        serves_decision: if build_request.output_intent.trim().is_empty() {
            format!(
                "Support the `{}` PRAXIS workflow with reviewed capsule context.",
                build_request.target_workflow
            )
        } else {
            build_request.output_intent.clone()
        },
        audience: "PRAXIS operator and downstream policy-analysis agents".to_owned(),
        time_horizon: format!("Built from source pack timestamp {}", source_pack.created_at),
        scope_in: source_scope,
        scope_out: vec![
            "Rejected, unreviewed, or evidence-requested proposals are lineage only.".to_owned(),
            "The capsule must not be used as uncaveated legal, electoral, medical, or financial advice.".to_owned(),
        ],
        success_criteria: vec![
            "Every promoted record remains source-span-backed or review-backed.".to_owned(),
            "PRAXIS receives Purpose before claims, devices, caveats, and output rules.".to_owned(),
            "Stale or caveated records remain visible as warnings instead of silent context.".to_owned(),
        ],
        red_lines: vec![
            "Do not promote model-derived content without Rust validation and human review.".to_owned(),
            "Do not hide caveats, rejected records, source gaps, or freshness warnings.".to_owned(),
        ],
        misuse_conditions: vec![
            "Refuse or re-gate generation when the capsule is beyond its refusal staleness policy.".to_owned(),
            "Do not use this capsule outside the requested workflow without a new review decision.".to_owned(),
        ],
        staleness_policy: PurposeStalenessPolicy {
            warn_after_days: 14,
            refuse_after_days: 30,
        },
    }
}

fn source_records(source_pack: &SourcePack) -> Vec<Value> {
    let documents = source_pack
        .documents
        .iter()
        .map(|document| (document.document_id.as_str(), document))
        .collect::<BTreeMap<_, _>>();
    source_pack
        .spans
        .iter()
        .map(|span| source_record(span, documents.get(span.document_id.as_str()).copied()))
        .collect()
}

fn source_record(span: &SourceSpan, document: Option<&SourceDocument>) -> Value {
    json!({
        "source_id": span.span_id,
        "document_id": span.document_id,
        "source_type": document.map(|document| document.source_type.as_str()).unwrap_or("unknown"),
        "title": document.map(|document| document.title.as_str()).unwrap_or("Unknown source"),
        "uri": document.map(|document| document.uri.as_str()).unwrap_or(""),
        "retrieved_at": document.map(|document| document.retrieved_at.as_str()).unwrap_or(""),
        "language": span.language,
        "rights_or_access": span.rights_or_access,
        "trust_status": document.map(|document| document.trust_status.as_str()).unwrap_or("unknown"),
        "span_id": span.span_id,
        "span_locator": span.locator,
        "text_hash": span.text_hash,
        "quote": span.quote,
        "notes": span.extraction_notes,
    })
}

fn praxis_claim_from_value(claim: &Value) -> Option<DialecticaPraxisClaim> {
    let id = string_value(claim, &["claim_id", "id"])?;
    let text = string_value(claim, &["claim_text", "text"])?;
    let source_ids = source_ids_from_claim(claim);
    let kind = if claim
        .get("disputed")
        .and_then(Value::as_bool)
        .unwrap_or(false)
    {
        DialecticaPraxisClaimKind::ConflictingClaim
    } else {
        match string_value(claim, &["review_state"]).as_deref() {
            Some("rejected") => DialecticaPraxisClaimKind::ConflictingClaim,
            Some("needs_review") => DialecticaPraxisClaimKind::EvidenceGap,
            _ => DialecticaPraxisClaimKind::Fact,
        }
    };

    Some(DialecticaPraxisClaim {
        id,
        kind,
        text,
        source_ids,
    })
}

fn rejected_claims_from_review(review: &Value) -> Vec<DialecticaPraxisClaim> {
    review
        .get("rejected_records")
        .and_then(Value::as_array)
        .into_iter()
        .flatten()
        .filter_map(|record| {
            let id = string_value(record, &["object_id", "proposal_id"])?;
            let rationale = string_value(record, &["rationale"]).unwrap_or_else(|| {
                "Reviewer rejected this record for promoted PRAXIS context.".to_owned()
            });
            Some(DialecticaPraxisClaim {
                id,
                kind: DialecticaPraxisClaimKind::ConflictingClaim,
                text: format!("Review-rejected record: {rationale}"),
                source_ids: Vec::new(),
            })
        })
        .collect()
}

fn evidence_gaps_from_review(review: &Value) -> Vec<String> {
    let mut gaps = Vec::new();
    gaps.extend(
        review
            .get("open_questions")
            .and_then(Value::as_array)
            .into_iter()
            .flatten()
            .filter_map(Value::as_str)
            .map(str::to_owned),
    );
    gaps.extend(
        review
            .get("promotion")
            .and_then(|promotion| promotion.get("evidence_requested_proposal_ids"))
            .and_then(Value::as_array)
            .into_iter()
            .flatten()
            .filter_map(Value::as_str)
            .map(|proposal_id| {
                format!("Evidence requested before promoting proposal {proposal_id}.")
            }),
    );
    gaps.extend(
        review
            .get("review_actions")
            .and_then(Value::as_array)
            .into_iter()
            .flatten()
            .filter(|action| string_value(action, &["decision"]).as_deref() == Some("needs_review"))
            .map(|action| {
                let object_ids = action
                    .get("object_ids")
                    .and_then(Value::as_array)
                    .into_iter()
                    .flatten()
                    .filter_map(Value::as_str)
                    .collect::<Vec<_>>()
                    .join(", ");
                let rationale = string_value(action, &["rationale"])
                    .unwrap_or_else(|| "Reviewer requested more evidence.".to_owned());
                if object_ids.is_empty() {
                    rationale
                } else {
                    format!("{object_ids}: {rationale}")
                }
            }),
    );
    gaps
}

fn praxis_source_receipt(source: &Value) -> Option<DialecticaPraxisSourceReceipt> {
    let id = string_value(source, &["source_id", "document_id", "id"])?;
    let title = string_value(source, &["title"]).unwrap_or_else(|| id.clone());
    let uri = string_value(source, &["uri", "url"]);
    let publisher = string_value(source, &["publisher"]);
    let retrieved_at = string_value(source, &["retrieved_at"]);
    let citation = [
        publisher.as_deref(),
        retrieved_at.as_deref(),
        uri.as_deref(),
    ]
    .into_iter()
    .flatten()
    .filter(|part| !part.is_empty())
    .collect::<Vec<_>>()
    .join("; ");
    Some(DialecticaPraxisSourceReceipt {
        id,
        title,
        url: uri.filter(|value| is_http_url(value)),
        citation: if citation.is_empty() {
            None
        } else {
            Some(citation)
        },
        excerpt: string_value(source, &["quote", "excerpt"]),
    })
}

fn source_ids_from_claim(claim: &Value) -> Vec<String> {
    let mut source_ids = claim
        .get("source_span_ids")
        .and_then(Value::as_array)
        .into_iter()
        .flatten()
        .filter_map(Value::as_str)
        .map(str::to_owned)
        .collect::<Vec<_>>();
    if source_ids.is_empty() {
        if let Some(source_id) = claim
            .get("source_span")
            .and_then(|span| span.get("source_id"))
            .and_then(Value::as_str)
        {
            source_ids.push(source_id.to_owned());
        }
    }
    source_ids
}

fn string_value(value: &Value, keys: &[&str]) -> Option<String> {
    keys.iter()
        .find_map(|key| value.get(*key).and_then(Value::as_str))
        .map(str::trim)
        .filter(|text| !text.is_empty())
        .map(str::to_owned)
}

fn is_http_url(value: &str) -> bool {
    value.starts_with("https://") || value.starts_with("http://")
}

fn claim_values(records: &[&PromotedRecord]) -> Vec<Value> {
    records
        .iter()
        .map(|record| {
            json!({
                "claim_id": record.object_id,
                "claim_text": string_payload(record, "claim_text", &record.object_id),
                "trust_layer": string_payload(record, "trust_layer", "T2"),
                "temporal_status": string_payload(record, "temporal_status", "unknown"),
                "confidence": record.confidence,
                "uncertainty": record.uncertainty,
                "source_span_ids": record.source_span_ids,
                "review_state": review_state(record),
                "review_decision_id": record.review_decision_id,
                "caveats": record.caveats,
            })
        })
        .collect()
}

fn episode_values(records: &[&PromotedRecord]) -> Value {
    json!({
        "schema_version": "episodes_v1",
        "episodes": records
            .iter()
            .map(|record| json!({
                "episode_id": record.object_id,
                "label": string_payload(record, "episode_label", &record.object_id),
                "valid_from": string_payload(record, "valid_from", ""),
                "status": string_payload(record, "status", "unknown"),
                "source_span_ids": record.source_span_ids,
                "review_state": review_state(record)
            }))
            .collect::<Vec<_>>()
    })
}

struct GraphInput<'a> {
    capsule_id: &'a str,
    source_pack: &'a SourcePack,
    claims: &'a [&'a PromotedRecord],
    episodes: &'a [&'a PromotedRecord],
    graph_nodes: &'a [&'a PromotedRecord],
    graph_edges: &'a [&'a PromotedRecord],
    ontology_terms: &'a [&'a PromotedRecord],
    reasoning_devices: &'a [&'a PromotedRecord],
    decision_set: &'a ReviewerDecisionSet,
}

fn graph_jsonld(input: GraphInput<'_>) -> Value {
    let mut situation_records = input
        .graph_nodes
        .iter()
        .map(|record| {
            let node_type = string_payload(record, "node_type", "node");
            json!({
                "@id": record.object_id,
                "@type": format!("dialectica:{}", title_case(&node_type)),
                "dialectica:title": string_payload(record, "label", &record.object_id),
                "dialectica:reviewState": review_state(record),
                "dialectica:sourceSpanIds": record.source_span_ids
            })
        })
        .collect::<Vec<_>>();
    add_graph_edges(&mut situation_records, input.graph_edges);

    json!({
        "@context": {
            "dialectica": "https://tacitus.me/dialectica#",
            "prov": "http://www.w3.org/ns/prov#",
            "g": "urn:praxis:graph:"
        },
        "@id": format!("urn:praxis:capsule:{}", input.capsule_id),
        "@type": "dialectica:SituationCapsule",
        "@graph": [
            {
                "@id": "g:evidence",
                "@graph": input.source_pack.documents.iter().map(|document| json!({
                    "@id": document.document_id,
                    "@type": "prov:Entity",
                    "dialectica:title": document.title,
                    "dialectica:reviewState": "approved_with_caveats"
                })).collect::<Vec<_>>()
            },
            {
                "@id": "g:claims",
                "@graph": input.claims.iter().map(|record| json!({
                    "@id": record.object_id,
                    "@type": "dialectica:Claim",
                    "dialectica:title": string_payload(record, "claim_text", &record.object_id),
                    "dialectica:trustLayer": string_payload(record, "trust_layer", "T2"),
                    "dialectica:reviewState": review_state(record),
                    "dialectica:sourceSpanIds": record.source_span_ids
                })).collect::<Vec<_>>()
            },
            {
                "@id": "g:situation",
                "@graph": situation_records
            },
            {
                "@id": "g:temporal",
                "@graph": input.episodes.iter().map(|record| json!({
                    "@id": record.object_id,
                    "@type": "dialectica:Episode",
                    "dialectica:title": string_payload(record, "episode_label", &record.object_id),
                    "dialectica:start": string_payload(record, "valid_from", ""),
                    "dialectica:reviewState": review_state(record),
                    "dialectica:sourceSpanIds": record.source_span_ids
                })).collect::<Vec<_>>()
            },
            {
                "@id": "g:ontology",
                "@graph": input.ontology_terms.iter().map(|record| json!({
                    "@id": record.object_id,
                    "@type": "dialectica:OntologyTerm",
                    "dialectica:title": string_payload(record, "label", &record.object_id),
                    "dialectica:definition": string_payload(record, "definition", ""),
                    "dialectica:reviewState": review_state(record),
                    "dialectica:sourceSpanIds": record.source_span_ids
                })).collect::<Vec<_>>()
            },
            {
                "@id": "g:reasoning",
                "@graph": input.reasoning_devices.iter().map(|record| json!({
                    "@id": record.object_id,
                    "@type": "dialectica:ReasoningDevice",
                    "dialectica:title": string_payload(record, "device_id", &record.object_id),
                    "dialectica:reviewState": review_state(record),
                    "dialectica:sourceSpanIds": record.source_span_ids
                })).collect::<Vec<_>>()
            },
            {
                "@id": "g:governance",
                "@graph": input.decision_set.decisions.iter().map(|decision| json!({
                    "@id": decision.decision_id,
                    "@type": "dialectica:ReviewAction",
                    "dialectica:decision": format!("{:?}", decision.status).to_lowercase(),
                    "dialectica:reviewState": "approved_with_caveats",
                    "dialectica:reviewedObject": decision.object_id
                })).collect::<Vec<_>>()
            },
            {
                "@id": "g:runtime",
                "@graph": [{
                    "@id": "runtime_contract",
                    "@type": "dialectica:RuntimeContract",
                    "dialectica:title": "PRAXIS runtime contract",
                    "dialectica:reviewState": "approved_with_caveats"
                }]
            }
        ]
    })
}

fn add_graph_edges(situation_records: &mut [Value], graph_edges: &[&PromotedRecord]) {
    for edge in graph_edges {
        let from_node_id = string_payload(edge, "from_node_id", "");
        let to_node_id = string_payload(edge, "to_node_id", "");
        let edge_type = string_payload(edge, "edge_type", "mentions");
        if from_node_id.is_empty() || to_node_id.is_empty() {
            continue;
        }
        if let Some(record) = situation_records
            .iter_mut()
            .find(|record| record.get("@id").and_then(Value::as_str) == Some(from_node_id.as_str()))
        {
            if let Some(object) = record.as_object_mut() {
                object.insert(format!("dialectica:{edge_type}"), Value::String(to_node_id));
                object.insert(
                    "dialectica:edgeId".to_owned(),
                    Value::String(edge.object_id.clone()),
                );
            }
        }
    }
}

fn reasoning_device_values(records: &[&PromotedRecord]) -> Vec<Value> {
    records
        .iter()
        .map(|record| {
            json!({
                "device_id": string_payload(record, "device_id", &record.object_id),
                "record_id": record.object_id,
                "steps": record.payload.get("steps").cloned().unwrap_or_else(|| json!([])),
                "review_state": review_state(record),
                "source_span_ids": record.source_span_ids,
                "caveats": record.caveats,
            })
        })
        .collect()
}

fn caveat_values(records: &[&PromotedRecord]) -> Vec<Value> {
    records
        .iter()
        .map(|record| {
            json!({
                "caveat_id": record.object_id,
                "text": string_payload(record, "caveat_text", &record.object_id),
                "applies_to": record.payload.get("applies_to").cloned().unwrap_or_else(|| json!([])),
                "source_span_ids": record.source_span_ids,
                "review_state": review_state(record)
            })
        })
        .collect()
}

fn traps(
    language_rules: &[&PromotedRecord],
    rights_rules: &[&PromotedRecord],
    caveats: &[&PromotedRecord],
) -> Vec<Value> {
    language_rules
        .iter()
        .map(|record| {
            json!({
                "trap_id": record.object_id,
                "kind": "language_rule",
                "blocked": string_payload(record, "blocked_phrase", ""),
                "approved": string_payload(record, "approved_phrase", ""),
                "reason": string_payload(record, "reason", ""),
            })
        })
        .chain(rights_rules.iter().map(|record| {
            json!({
                "trap_id": record.object_id,
                "kind": "rights_rule",
                "prohibition": string_payload(record, "prohibition", ""),
                "duty": string_payload(record, "duty", ""),
            })
        }))
        .chain(caveats.iter().map(|record| {
            json!({
                "trap_id": record.object_id,
                "kind": "caveat",
                "text": string_payload(record, "caveat_text", ""),
            })
        }))
        .collect()
}

fn review_json(decision_set: &ReviewerDecisionSet, promoted: &PromotedRecordSet) -> Value {
    let caveats = promoted
        .promoted_records
        .iter()
        .flat_map(|record| record.caveats.iter().cloned())
        .collect::<Vec<_>>();
    let rejected_records = promoted
        .rejected_proposal_ids
        .iter()
        .filter_map(|proposal_id| {
            decision_set
                .decisions
                .iter()
                .find(|decision| &decision.proposal_id == proposal_id)
        })
        .map(|decision| {
            json!({
                "proposal_id": decision.proposal_id,
                "object_id": decision.object_id,
                "decision_id": decision.decision_id,
                "reviewer_role": decision.reviewer_role,
                "rationale": decision.rationale
            })
        })
        .collect::<Vec<_>>();

    json!({
        "schema_version": "review_v1",
        "decision_set_id": decision_set.decision_set_id,
        "extraction_run_id": decision_set.extraction_run_id,
        "review_actions": decision_set.decisions.iter().map(|decision| json!({
            "id": decision.decision_id,
            "reviewer_role": decision.reviewer_role,
            "decision": review_decision_status(decision.status),
            "object_ids": [decision.object_id.clone()],
            "caveats": decision.caveats,
            "rationale": decision.rationale,
            "reviewed_at": decision.decided_at
        })).collect::<Vec<_>>(),
        "caveats": caveats,
        "rejected_records": rejected_records,
        "promotion": {
            "promoted_record_count": promoted.promoted_records.len(),
            "rejected_proposal_ids": promoted.rejected_proposal_ids,
            "evidence_requested_proposal_ids": promoted.evidence_requested_proposal_ids,
            "ready_for_compiler": promoted.ready_for_compiler
        }
    })
}

fn runtime_json(
    build_request: &CapsuleBuildRequest,
    language_rules: &[&PromotedRecord],
    rights_rules: &[&PromotedRecord],
    output_rules: &[&PromotedRecord],
) -> Value {
    let output_rule = output_rules.first().copied();
    json!({
        "verbs": ["seek", "understand", "connect", "critique", "improve", "apply_device", "diff", "compose"],
        "target_workflow": build_request.target_workflow,
        "citation_style": "source_span_receipt",
        "trust_rules": {
            "T1": "assert",
            "T2": "attribute",
            "T3": "hedge"
        },
        "must_surface_disputed": true,
        "must_cite": true,
        "graph_policy": "Use graph.jsonld for the semantic source graph and graph/ladybug/capsule.lbug for read-only embedded graph previews.",
        "language_profile": {
            "profile_id": "language:compiled-fixture-v1",
            "rules": language_rules.iter().map(|record| json!({
                "rule_id": record.object_id,
                "blocked_phrase": string_payload(record, "blocked_phrase", ""),
                "approved_phrase": string_payload(record, "approved_phrase", ""),
                "reason": string_payload(record, "reason", ""),
                "review_state": review_state(record)
            })).collect::<Vec<_>>()
        },
        "language_rules": language_rules.iter().map(|record| json!({
            "rule_id": record.object_id,
            "review_state": review_state(record)
        })).collect::<Vec<_>>(),
        "rights_rules": rights_rules.iter().map(|record| json!({
            "rule_id": record.object_id,
            "prohibition": string_payload(record, "prohibition", ""),
            "duty": string_payload(record, "duty", ""),
            "review_state": review_state(record)
        })).collect::<Vec<_>>(),
        "output_contract": output_rule.map(|record| json!({
            "output_contract_id": record.object_id,
            "output_type": string_payload(record, "output_type", "decision_brief"),
            "required_sections": record.payload.get("required_sections").cloned().unwrap_or_else(|| json!([])),
            "forbidden_content": record.payload.get("forbidden_content").cloned().unwrap_or_else(|| json!([])),
            "review_state": review_state(record)
        })).unwrap_or_else(|| json!({}))
    })
}

fn agent_context_markdown(
    manifest: &PraxisCapsuleManifest,
    source_pack: &SourcePack,
    claims: &[&PromotedRecord],
    reasoning_devices: &[&PromotedRecord],
    caveats: &[&PromotedRecord],
    language_rules: &[&PromotedRecord],
    output_rules: &[&PromotedRecord],
) -> String {
    let mut text = String::new();
    text.push_str("# PURPOSE\n\n");
    text.push_str(&format!(
        "- Serves decision: {}\n",
        manifest.purpose_profile.serves_decision
    ));
    text.push_str(&format!(
        "- Audience: {}\n",
        manifest.purpose_profile.audience
    ));
    text.push_str(&format!(
        "- Time horizon: {}\n",
        manifest.purpose_profile.time_horizon
    ));
    text.push_str(&format!(
        "- Staleness: warn after {} days; refuse after {} days\n",
        manifest.purpose_profile.staleness_policy.warn_after_days,
        manifest.purpose_profile.staleness_policy.refuse_after_days
    ));
    if !manifest.purpose_profile.scope_in.is_empty() {
        text.push_str("- Scope in: ");
        text.push_str(&manifest.purpose_profile.scope_in.join("; "));
        text.push('\n');
    }
    if !manifest.purpose_profile.scope_out.is_empty() {
        text.push_str("- Scope out: ");
        text.push_str(&manifest.purpose_profile.scope_out.join("; "));
        text.push('\n');
    }
    if !manifest.purpose_profile.misuse_conditions.is_empty() {
        text.push_str("- Misuse conditions: ");
        text.push_str(&manifest.purpose_profile.misuse_conditions.join("; "));
        text.push('\n');
    }
    text.push('\n');
    text.push_str("# CONTRACT\n\n");
    text.push_str("Use this PRAXIS Capsule as source-grounded context. Cite source span ids, surface caveats, and do not promote rejected or unreviewed records.\n\n");
    text.push_str("# CAPSULE\n\n");
    text.push_str(&format!(
        "- `{}`: {} ({})\n",
        manifest.capsule_id, manifest.title, manifest.capsule_type
    ));
    text.push_str(&format!("- Source pack: `{}`\n\n", source_pack.pack_id));
    text.push_str("# FACTS BY TRUST\n\n");
    for claim in claims {
        text.push_str(&format!(
            "- `{}`: {} [{}]\n",
            claim.object_id,
            string_payload(claim, "claim_text", &claim.object_id),
            string_payload(claim, "trust_layer", "T2")
        ));
    }
    text.push_str("\n# REASONING SCAFFOLD\n\n");
    for device in reasoning_devices {
        text.push_str(&format!(
            "- Apply `{}` and keep sourced positions separate from inference.\n",
            string_payload(device, "device_id", &device.object_id)
        ));
    }
    text.push_str("\n# CAVEATS\n\n");
    for caveat in caveats {
        text.push_str(&format!(
            "- `{}`: {}\n",
            caveat.object_id,
            string_payload(caveat, "caveat_text", &caveat.object_id)
        ));
    }
    text.push_str("\n# LANGUAGE\n\n");
    for rule in language_rules {
        text.push_str(&format!(
            "- Avoid `{}`; prefer `{}`.\n",
            string_payload(rule, "blocked_phrase", ""),
            string_payload(rule, "approved_phrase", "")
        ));
    }
    text.push_str("\n# OUTPUT RULES\n\n");
    for rule in output_rules {
        text.push_str(&format!(
            "- `{}` requires caveats and source receipts.\n",
            string_payload(rule, "output_type", "decision_brief")
        ));
    }
    text
}

fn operations_markdown(manifest: &PraxisCapsuleManifest) -> String {
    format!(
        "# Operations\n\nCapsule `{}` supports: seek, understand, connect, critique, improve, apply_device, diff, compose.\n\nUse `claims.jsonl`, `graph.jsonld`, `episodes.json`, and `evidence/sources.jsonl` for source-cited reasoning. Use `graph/ladybug/capsule.lbug` for read-only embedded graph previews.\n",
        manifest.capsule_id
    )
}

fn write_ladybug_projection(output_dir: &Path) -> Result<(), CompilerError> {
    let build_error = match dialectica_graph::build_ladybug_projection(output_dir) {
        Ok(_) => return Ok(()),
        Err(error) => error,
    };
    let plan = dialectica_graph::plan_ladybug_projection(output_dir)
        .map_err(|error| CompilerError::InvalidInput(error.to_string()))?;
    let ladybug_dir = output_dir.join("graph").join("ladybug");
    fs::create_dir_all(&ladybug_dir)?;
    write_text(
        &output_dir.join(LADYBUG_SCHEMA_PATH),
        dialectica_graph::LADYBUG_SCHEMA_CYPHER,
    )?;
    write_text(
        &output_dir.join(LADYBUG_QUERIES_PATH),
        dialectica_graph::LADYBUG_QUERIES_CYPHER,
    )?;
    write_json(
        &output_dir.join(LADYBUG_DATABASE_PATH),
        &json!({
            "engine": "ladybug",
            "mode": "deterministic_fixture_projection",
            "capsule_id": plan.capsule_id,
            "source_graph_digest": plan.source_graph_digest,
            "node_count": plan.node_count,
            "edge_count": plan.edge_count
        }),
    )?;

    let projection_digest = digest_file(&output_dir.join(LADYBUG_DATABASE_PATH))?;
    let schema_digest = digest_file(&output_dir.join(LADYBUG_SCHEMA_PATH))?;
    let query_digest = digest_file(&output_dir.join(LADYBUG_QUERIES_PATH))?;
    let manifest = LadybugProjectionManifest {
        schema_version: "ladybug_projection_manifest_v1".to_owned(),
        capsule_id: plan.capsule_id.clone(),
        profile: LADYBUG_PROJECTION_PROFILE.to_owned(),
        engine: "ladybug".to_owned(),
        engine_crate: "lbug".to_owned(),
        engine_crate_version: "fixture-rebuildable".to_owned(),
        storage_version: 0,
        database_path: normalize_relative_path(LADYBUG_DATABASE_PATH),
        source_graph_path: "graph.jsonld".to_owned(),
        source_graph_digest: plan.source_graph_digest.clone(),
        projection_digest: projection_digest.clone(),
        schema_path: normalize_relative_path(LADYBUG_SCHEMA_PATH),
        schema_digest,
        query_path: normalize_relative_path(LADYBUG_QUERIES_PATH),
        query_digest,
        node_count: plan.node_count,
        edge_count: plan.edge_count,
        read_only: true,
        rebuildable: true,
    };
    let receipt = LadybugProjectionBuildReceipt {
        schema_version: "ladybug_projection_build_receipt_v1".to_owned(),
        capsule_id: plan.capsule_id,
        profile: LADYBUG_PROJECTION_PROFILE.to_owned(),
        built_at_unix_seconds: 0,
        source_graph_digest: plan.source_graph_digest,
        projection_digest,
        node_count: plan.node_count,
        edge_count: plan.edge_count,
        query_check: format!("projection_rebuild_required: {build_error}"),
    };
    write_json(
        &output_dir.join(LADYBUG_PROJECTION_MANIFEST_PATH),
        &manifest,
    )?;
    write_json(&output_dir.join(LADYBUG_BUILD_RECEIPT_PATH), &receipt)?;
    Ok(())
}

fn promoted_records_by_kind(
    promoted: &PromotedRecordSet,
    kind: ProposalKind,
) -> Vec<&PromotedRecord> {
    let mut records = promoted
        .promoted_records
        .iter()
        .filter(|record| record.kind == kind)
        .collect::<Vec<_>>();
    records.sort_by(|left, right| left.object_id.cmp(&right.object_id));
    records
}

fn string_payload<'a>(record: &'a PromotedRecord, key: &str, fallback: &'a str) -> String {
    record
        .payload
        .get(key)
        .and_then(Value::as_str)
        .unwrap_or(fallback)
        .to_owned()
}

fn review_state(record: &PromotedRecord) -> &'static str {
    match record.review_status {
        Some(ReviewDecisionStatus::Approve) => "approved",
        Some(ReviewDecisionStatus::ApproveWithCaveats) => "approved_with_caveats",
        Some(ReviewDecisionStatus::Reject) => "rejected",
        Some(ReviewDecisionStatus::RequestMoreEvidence) => "needs_review",
        None => "approved_with_caveats",
    }
}

fn review_decision_status(status: ReviewDecisionStatus) -> &'static str {
    match status {
        ReviewDecisionStatus::Approve => "approved",
        ReviewDecisionStatus::ApproveWithCaveats => "approved_with_caveats",
        ReviewDecisionStatus::Reject => "rejected",
        ReviewDecisionStatus::RequestMoreEvidence => "needs_review",
    }
}

fn capsule_type_value(capsule_type: CapsuleType) -> &'static str {
    match capsule_type {
        CapsuleType::User => "user",
        CapsuleType::Situation => "situation",
        CapsuleType::Tool => "tool",
        CapsuleType::Output => "output",
    }
}

fn title_case(value: &str) -> String {
    value
        .split(['_', '-', ' '])
        .filter(|part| !part.is_empty())
        .map(|part| {
            let mut chars = part.chars();
            match chars.next() {
                Some(first) => first.to_uppercase().chain(chars).collect::<String>(),
                None => String::new(),
            }
        })
        .collect::<String>()
}

fn graph_focus(graph: &Value) -> Vec<Value> {
    graph
        .get("@graph")
        .and_then(Value::as_array)
        .into_iter()
        .flatten()
        .filter(|layer| layer.get("@id").and_then(Value::as_str) == Some("g:situation"))
        .flat_map(|layer| {
            layer
                .get("@graph")
                .and_then(Value::as_array)
                .into_iter()
                .flatten()
        })
        .filter(|record| {
            record
                .get("dialectica:reviewState")
                .and_then(Value::as_str)
                != Some("rejected")
        })
        .map(|record| {
            json!({
                "id": record.get("@id").and_then(Value::as_str).unwrap_or(""),
                "label": record
                    .get("dialectica:title")
                    .and_then(Value::as_str)
                    .or_else(|| record.get("@id").and_then(Value::as_str))
                    .unwrap_or(""),
                "node_type": record.get("@type").and_then(Value::as_str).unwrap_or("node"),
                "review_state": record.get("dialectica:reviewState").and_then(Value::as_str).unwrap_or("approved_with_caveats"),
                "edge_id": record.get("dialectica:edgeId").and_then(Value::as_str).unwrap_or("")
            })
        })
        .collect()
}

fn temporal_warnings(claims: &[Value]) -> Vec<String> {
    claims
        .iter()
        .filter_map(|claim| {
            let status = claim.get("temporal_status").and_then(Value::as_str)?;
            if matches!(status, "stale" | "superseded" | "contested" | "forecast") {
                Some(format!(
                    "{} is {}",
                    claim
                        .get("claim_id")
                        .and_then(Value::as_str)
                        .unwrap_or("claim"),
                    status
                ))
            } else {
                None
            }
        })
        .collect()
}

fn runtime_array_ids(runtime: &Value, array_key: &str, id_key: &str) -> Vec<String> {
    runtime
        .get(array_key)
        .and_then(Value::as_array)
        .into_iter()
        .flatten()
        .filter_map(|value| value.get(id_key).and_then(Value::as_str).map(str::to_owned))
        .collect()
}

fn list_package_files(root: &Path) -> Result<Vec<String>, CompilerError> {
    let mut files = Vec::new();
    collect_files(root, root, &mut files)?;
    files.sort();
    Ok(files)
}

fn collect_files(
    root: &Path,
    current: &Path,
    files: &mut Vec<String>,
) -> Result<(), CompilerError> {
    let mut entries = fs::read_dir(current)?.collect::<Result<Vec<_>, _>>()?;
    entries.sort_by_key(|entry| entry.file_name());
    for entry in entries {
        let path = entry.path();
        if path.is_dir() {
            collect_files(root, &path, files)?;
        } else if path.is_file() {
            files.push(normalize_relative_path(
                path.strip_prefix(root)
                    .map_err(|error| CompilerError::InvalidInput(error.to_string()))?
                    .to_string_lossy()
                    .as_ref(),
            ));
        }
    }
    Ok(())
}

fn digest_directory(root: &Path) -> Result<String, CompilerError> {
    let mut hasher = Sha256::new();
    for relative in list_package_files(root)? {
        hasher.update(relative.as_bytes());
        hasher.update([0]);
        hasher.update(fs::read(root.join(&relative))?);
        hasher.update([0]);
    }
    Ok(format!("sha256:{:x}", hasher.finalize()))
}

fn digest_file(path: &Path) -> Result<String, CompilerError> {
    Ok(format!("sha256:{:x}", Sha256::digest(fs::read(path)?)))
}

fn digest_json<T: Serialize>(value: &T) -> Result<String, CompilerError> {
    Ok(format!(
        "sha256:{:x}",
        Sha256::digest(serde_json::to_vec(value)?)
    ))
}

fn write_text(path: &Path, text: &str) -> Result<(), CompilerError> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(path, text)?;
    Ok(())
}

fn write_json<T: Serialize>(path: &Path, value: &T) -> Result<(), CompilerError> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    let text = serde_json::to_string_pretty(value)?;
    fs::write(path, format!("{text}\n"))?;
    Ok(())
}

fn write_jsonl_values(path: &Path, values: &[Value]) -> Result<(), CompilerError> {
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

fn read_jsonl_values(path: &Path) -> Result<Vec<Value>, CompilerError> {
    let text = fs::read_to_string(path)?;
    text.lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| Ok(serde_json::from_str(line)?))
        .collect()
}

fn normalize_relative_path(path: &str) -> String {
    path.replace('\\', "/")
}

#[cfg(test)]
mod tests {
    use std::{
        fs::{self, File},
        path::{Path, PathBuf},
    };

    use dialectica_capsule::{CapsuleManifest, PraxisCapsulePackage, ReviewState};
    use dialectica_extractor::{
        load_build_request, load_source_pack, ProposalSet, ReviewDecisionStatus,
        ReviewerDecisionSet,
    };
    use serde_json::Value;

    use super::{
        can_emit_bundle, compile_fixture, compile_from_parts, export_praxis_context_pack,
        export_praxis_import_package, verify_integrity_envelope, write_capsule_archive,
        write_capsule_diff, DialecticaPraxisClaimKind, DialecticaPraxisContextExport,
        DIALECTICA_PRAXIS_CONTEXT_CONTRACT_VERSION,
    };

    #[test]
    fn compiler_respects_review_gate() {
        let manifest = CapsuleManifest::new(
            "cap_001",
            "Draft capsule",
            "situation_capsule",
            ReviewState::NeedsReview,
            "sha256:test",
        );

        assert!(!can_emit_bundle(&manifest));
    }

    #[test]
    fn fixture_compiler_writes_valid_v3_package() {
        let out = fresh_temp_dir("dialectica_compile_fixture");

        let receipt = compile_fixture(&golden_fixture_dir(), &out).expect("fixture should compile");
        let package = PraxisCapsulePackage::load_from_dir(&out).expect("package should load");
        let report = package.validate();

        assert_eq!(
            receipt.capsule_id,
            "cap_build_conflict_situation_fixture_v1"
        );
        assert!(!report.has_errors(), "{:#?}", report.findings);
        assert!(out.join("graph/ladybug/projection_manifest.json").is_file());
        let build_receipt: Value = serde_json::from_str(
            &fs::read_to_string(out.join("graph/ladybug/build_receipt.json"))
                .expect("ladybug build receipt should read"),
        )
        .expect("ladybug build receipt should parse");
        assert!(build_receipt["query_check"]
            .as_str()
            .expect("query_check should be text")
            .contains("projection_rebuild_required"));
        assert!(fs::read_to_string(out.join("agent_context.md"))
            .expect("agent context should read")
            .contains("# CONTRACT"));

        let _ = fs::remove_dir_all(out);
    }

    #[test]
    fn fixture_archive_writes_mimetype_first() {
        let out = fresh_temp_dir("dialectica_archive_fixture");
        let archive_path = out.with_extension("capsule");

        compile_fixture(&golden_fixture_dir(), &out).expect("fixture should compile");
        let receipt =
            write_capsule_archive(&out, &archive_path).expect("archive should be written");

        assert_eq!(receipt.entry_count, receipt.entries.len());
        let file = File::open(&archive_path).expect("archive should open");
        let mut archive = zip::ZipArchive::new(file).expect("zip should parse");
        assert_eq!(archive.by_index(0).expect("first entry").name(), "mimetype");

        let _ = fs::remove_dir_all(out);
        let _ = fs::remove_file(archive_path);
    }

    #[test]
    fn archive_rejects_output_inside_package_directory() {
        let out = fresh_temp_dir("dialectica_archive_inside_fixture");
        let archive_path = out.join("self.capsule");

        compile_fixture(&golden_fixture_dir(), &out).expect("fixture should compile");
        let error =
            write_capsule_archive(&out, &archive_path).expect_err("archive path must be rejected");

        assert!(error
            .to_string()
            .contains("must be outside package directory"));

        let _ = fs::remove_dir_all(out);
    }

    #[test]
    fn context_pack_contains_praxis_runtime_fields() {
        let out = fresh_temp_dir("dialectica_context_pack_fixture");

        compile_fixture(&golden_fixture_dir(), &out).expect("fixture should compile");
        let pack =
            export_praxis_context_pack(&out, "conflict_map").expect("context pack should export");

        assert_eq!(pack.capsule_id, "cap_build_conflict_situation_fixture_v1");
        assert_eq!(pack.workflow, "conflict_map");
        assert!(pack
            .purpose_profile
            .serves_decision
            .contains("decision brief"));
        assert!(pack.agent_context.starts_with("# PURPOSE"));
        assert!(!pack.agent_context.trim().is_empty());
        assert!(!pack.retrieval_records.is_empty());
        assert!(!pack.reasoning_devices.is_empty());
        assert!(pack
            .forbidden_claims
            .iter()
            .any(|claim| claim.contains("uncaveated legal conclusion")));
        assert!(pack
            .read_receipt_hints
            .source_ids
            .iter()
            .any(|source_id| source_id == "doc_election_commission_statement"));

        let _ = fs::remove_dir_all(out);
    }

    #[test]
    fn praxis_import_export_round_trips_with_copied_praxis_contract() {
        let out = fresh_temp_dir("dialectica_praxis_import_export_fixture");

        compile_fixture(&golden_fixture_dir(), &out).expect("fixture should compile");
        let export = export_praxis_import_package(&out).expect("PRAXIS import export should build");
        let export_path = out.join("praxis-export.json");
        let text = serde_json::to_string_pretty(&export).expect("export should serialize");
        fs::write(&export_path, format!("{text}\n")).expect("export should write");
        let value: Value =
            serde_json::from_str(&fs::read_to_string(&export_path).expect("export should read"))
                .expect("export JSON should parse");
        assert_matches_copied_praxis_zod_contract(&value);
        let round_trip: DialecticaPraxisContextExport =
            serde_json::from_value(value).expect("copied PRAXIS contract should deserialize");

        assert_eq!(
            round_trip.contract_version,
            DIALECTICA_PRAXIS_CONTEXT_CONTRACT_VERSION
        );
        assert_eq!(
            round_trip.package_id,
            "cap_build_conflict_situation_fixture_v1"
        );
        assert!(round_trip
            .claims
            .iter()
            .any(|claim| claim.kind == DialecticaPraxisClaimKind::Fact));
        assert!(round_trip
            .source_receipts
            .iter()
            .all(|receipt| match receipt.url.as_deref() {
                Some(url) => url.starts_with("http"),
                None => true,
            }));

        let _ = fs::remove_dir_all(out);
    }

    #[test]
    fn praxis_import_export_maps_rejected_reviews_to_conflicting_claims() {
        let build_request = load_build_request(&golden_fixture_dir().join("build_request.json"))
            .expect("build request should load");
        let source_pack =
            load_source_pack(&golden_fixture_dir().join("source-pack/source_pack.json"))
                .expect("source pack should load");
        let proposal_set = ProposalSet::load_from_dir(&golden_fixture_dir().join("proposals"))
            .expect("proposal set should load");
        let mut decision_set =
            ReviewerDecisionSet::load_from_dir(&golden_fixture_dir().join("review-decisions"))
                .expect("decision set should load");
        let rejected = decision_set
            .decisions
            .iter_mut()
            .find(|decision| decision.proposal_id == "prop_claim_army_rejected")
            .expect("fixture decision should exist");
        rejected.status = ReviewDecisionStatus::Reject;
        rejected.caveats.clear();
        rejected.rationale =
            "Rejected because the contested leverage inference needs a second source.".to_owned();
        let out = fresh_temp_dir("dialectica_praxis_rejected_export_fixture");

        compile_from_parts(
            &build_request,
            &source_pack,
            &proposal_set,
            &decision_set,
            &out,
        )
        .expect("rejected records should compile as lineage");
        let export = export_praxis_import_package(&out).expect("PRAXIS import export should build");

        assert!(export.claims.iter().any(|claim| {
            claim.id == "clm_army_rejected_certification"
                && claim.kind == DialecticaPraxisClaimKind::ConflictingClaim
                && claim.text.contains("contested leverage inference")
        }));

        let _ = fs::remove_dir_all(out);
    }

    #[test]
    fn missing_required_review_decision_blocks_compilation() {
        let build_request = load_build_request(&golden_fixture_dir().join("build_request.json"))
            .expect("build request should load");
        let source_pack =
            load_source_pack(&golden_fixture_dir().join("source-pack/source_pack.json"))
                .expect("source pack should load");
        let proposal_set = ProposalSet::load_from_dir(&golden_fixture_dir().join("proposals"))
            .expect("proposal set should load");
        let mut decision_set =
            ReviewerDecisionSet::load_from_dir(&golden_fixture_dir().join("review-decisions"))
                .expect("decision set should load");
        decision_set
            .decisions
            .retain(|decision| decision.proposal_id != "prop_claim_certified_result");
        let out = fresh_temp_dir("dialectica_missing_review_fixture");
        let result = compile_from_parts(
            &build_request,
            &source_pack,
            &proposal_set,
            &decision_set,
            &out,
        );

        assert!(result
            .expect_err("missing review must block compilation")
            .to_string()
            .contains("missing_reviewer_decision"));

        let _ = fs::remove_dir_all(out);
    }

    #[test]
    fn rejected_review_decision_stays_out_of_context_pack() {
        let build_request = load_build_request(&golden_fixture_dir().join("build_request.json"))
            .expect("build request should load");
        let source_pack =
            load_source_pack(&golden_fixture_dir().join("source-pack/source_pack.json"))
                .expect("source pack should load");
        let proposal_set = ProposalSet::load_from_dir(&golden_fixture_dir().join("proposals"))
            .expect("proposal set should load");
        let mut decision_set =
            ReviewerDecisionSet::load_from_dir(&golden_fixture_dir().join("review-decisions"))
                .expect("decision set should load");
        let decision = decision_set
            .decisions
            .iter_mut()
            .find(|decision| decision.proposal_id == "prop_claim_army_rejected")
            .expect("fixture decision should exist");
        decision.status = ReviewDecisionStatus::Reject;
        decision.caveats.clear();
        decision.rationale = "Rejected by test reviewer.".to_owned();
        let out = fresh_temp_dir("dialectica_rejected_review_fixture");

        compile_from_parts(
            &build_request,
            &source_pack,
            &proposal_set,
            &decision_set,
            &out,
        )
        .expect("rejected records should compile as lineage");
        let pack =
            export_praxis_context_pack(&out, "conflict_map").expect("context pack should export");

        assert!(!pack
            .claim_ids
            .iter()
            .any(|claim_id| claim_id == "clm_army_rejected_certification"));
        assert!(pack
            .rejected_record_ids
            .iter()
            .any(|claim_id| claim_id == "clm_army_rejected_certification"));

        let _ = fs::remove_dir_all(out);
    }

    #[test]
    fn capsule_diff_reports_retractions_supersessions_and_citations() {
        let old_out = fresh_temp_dir("dialectica_diff_old_fixture");
        let new_out = fresh_temp_dir("dialectica_diff_new_fixture");
        let diff_out = fresh_temp_dir("dialectica_diff_output_fixture");
        compile_fixture(&golden_fixture_dir(), &old_out).expect("old fixture should compile");

        let build_request = load_build_request(&golden_fixture_dir().join("build_request.json"))
            .expect("build request should load");
        let source_pack =
            load_source_pack(&golden_fixture_dir().join("source-pack/source_pack.json"))
                .expect("source pack should load");
        let proposal_set = ProposalSet::load_from_dir(&golden_fixture_dir().join("proposals"))
            .expect("proposal set should load");
        let mut decision_set =
            ReviewerDecisionSet::load_from_dir(&golden_fixture_dir().join("review-decisions"))
                .expect("decision set should load");
        let rejected = decision_set
            .decisions
            .iter_mut()
            .find(|decision| decision.proposal_id == "prop_claim_army_rejected")
            .expect("fixture decision should exist");
        rejected.status = ReviewDecisionStatus::Reject;
        rejected.caveats.clear();
        rejected.rationale = "Rejected by test reviewer.".to_owned();
        compile_from_parts(
            &build_request,
            &source_pack,
            &proposal_set,
            &decision_set,
            &new_out,
        )
        .expect("new fixture should compile");
        revise_certified_claim(&new_out);

        let receipt = write_capsule_diff(&old_out, &new_out, &diff_out).expect("diff should write");
        let diff: super::CapsuleDiff = serde_json::from_str(
            &fs::read_to_string(&receipt.diff_path).expect("diff should read"),
        )
        .expect("diff should parse");
        let memo = fs::read_to_string(&receipt.change_memo_path).expect("memo should read");

        assert_eq!(receipt.retracted_claim_count, 1);
        assert_eq!(receipt.superseded_claim_count, 1);
        assert!(diff
            .claims
            .retracted
            .iter()
            .any(|record| record.record_id == "clm_army_rejected_certification"));
        assert!(diff
            .claims
            .superseded
            .iter()
            .any(|record| record.record_id == "clm_commission_certified_result"));
        assert!(diff.trust_transitions.iter().any(|transition| {
            transition.record_id == "clm_commission_certified_result"
                && transition.from.as_deref() == Some("T2")
                && transition.to.as_deref() == Some("T1")
        }));
        assert!(diff.citations.new_source_hashes.iter().any(|hash| hash
            == "sha256:3333333333333333333333333333333333333333333333333333333333333333"));
        assert!(memo.contains("Review receipt: `decision_claim_certified_result`"));
        assert!(memo.contains("Source hash: `sha256:3333333333333333333333333333333333333333333333333333333333333333`"));

        let _ = fs::remove_dir_all(old_out);
        let _ = fs::remove_dir_all(new_out);
        let _ = fs::remove_dir_all(diff_out);
    }

    #[test]
    fn compiled_fixture_writes_verifiable_integrity_envelope() {
        let out = fresh_temp_dir("dialectica_integrity_fixture");

        compile_fixture(&golden_fixture_dir(), &out).expect("fixture should compile");
        let report = verify_integrity_envelope(&out).expect("integrity should verify");

        assert!(out.join("integrity/envelope.json").is_file());
        assert!(report.verified, "{:#?}", report.findings);
        assert!(report.checked_file_count >= 10);
        assert!(report.merkle_root.starts_with("sha256:"));

        let _ = fs::remove_dir_all(out);
    }

    #[test]
    fn integrity_verification_catches_tampered_canonical_file() {
        let out = fresh_temp_dir("dialectica_integrity_tamper_fixture");

        compile_fixture(&golden_fixture_dir(), &out).expect("fixture should compile");
        let mut claims = fs::read_to_string(out.join("claims.jsonl")).expect("claims should read");
        claims.push_str("{\"claim_id\":\"tampered\"}\n");
        fs::write(out.join("claims.jsonl"), claims).expect("claims should write");

        let report = verify_integrity_envelope(&out).expect("integrity report should load");

        assert!(!report.verified);
        assert!(report
            .findings
            .iter()
            .any(|finding| finding.contains("claims.jsonl")));

        let _ = fs::remove_dir_all(out);
    }

    fn golden_fixture_dir() -> PathBuf {
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../..")
            .join("fixtures/golden-policy-capsule")
    }

    fn fresh_temp_dir(name: &str) -> PathBuf {
        let root = std::env::temp_dir().join(format!("{}_{}", name, std::process::id()));
        let _ = fs::remove_dir_all(&root);
        root
    }

    fn assert_matches_copied_praxis_zod_contract(value: &Value) {
        assert_eq!(
            value.get("contractVersion").and_then(Value::as_str),
            Some(DIALECTICA_PRAXIS_CONTEXT_CONTRACT_VERSION)
        );
        assert_non_empty_string(value, "packageId");
        assert_non_empty_string(value, "exportedAt");
        assert_non_empty_string(value, "title");
        let claims = value
            .get("claims")
            .and_then(Value::as_array)
            .expect("claims should be an array");
        assert!(!claims.is_empty());
        for claim in claims {
            assert_non_empty_string(claim, "id");
            assert_non_empty_string(claim, "text");
            assert!(matches!(
                claim.get("kind").and_then(Value::as_str),
                Some("fact" | "evidence_gap" | "conflicting_claim")
            ));
            assert!(claim
                .get("sourceIds")
                .and_then(Value::as_array)
                .expect("sourceIds should be an array")
                .iter()
                .all(Value::is_string));
        }
        for key in ["assumptions", "uncertainties", "evidenceGaps"] {
            assert!(value.get(key).and_then(Value::as_array).is_some());
        }
        let source_receipts = value
            .get("sourceReceipts")
            .and_then(Value::as_array)
            .expect("sourceReceipts should be an array");
        assert!(!source_receipts.is_empty());
        for receipt in source_receipts {
            assert_non_empty_string(receipt, "id");
            assert_non_empty_string(receipt, "title");
            if let Some(url) = receipt.get("url").and_then(Value::as_str) {
                assert!(url.starts_with("http://") || url.starts_with("https://"));
            }
        }
    }

    fn assert_non_empty_string(value: &Value, key: &str) {
        assert!(value
            .get(key)
            .and_then(Value::as_str)
            .is_some_and(|text| !text.trim().is_empty()));
    }

    fn revise_certified_claim(package_dir: &Path) {
        let claims_path = package_dir.join("claims.jsonl");
        let mut claims = fs::read_to_string(&claims_path)
            .expect("claims should read")
            .lines()
            .map(|line| serde_json::from_str::<Value>(line).expect("claim should parse"))
            .collect::<Vec<_>>();
        for claim in &mut claims {
            if claim.get("claim_id").and_then(Value::as_str)
                == Some("clm_commission_certified_result")
            {
                claim["claim_text"] = Value::String(
                    "The election commission certified the provisional result and opened the statutory challenge and audit window.".to_owned(),
                );
                claim["trust_layer"] = Value::String("T1".to_owned());
            }
        }
        let mut text = String::new();
        for claim in claims {
            text.push_str(&serde_json::to_string(&claim).expect("claim should serialize"));
            text.push('\n');
        }
        fs::write(claims_path, text).expect("claims should write");
    }
}
