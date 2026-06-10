use std::path::Path;

use dialectica_capsule::{
    export_schema_dir, CapsuleBundle, CapsuleManifest, PraxisCapsulePackage, ReviewState,
    ValidationSeverity,
};
use dialectica_compiler::export_schema_dir as export_compiler_schema_dir;
use dialectica_extractor::{
    draft_reviewer_decision_set, export_schema_dir as export_extractor_schema_dir,
    load_build_request, load_source_pack, plan_capsule_build, promote_records, route_review_gates,
    validate_proposal_set, validate_reviewer_decision_set, validate_source_pack,
    BuildValidationSeverity, CapsuleType, ProposalSet, ReviewDecisionStatus, ReviewerDecisionSet,
};

fn golden_bundle_dir() -> std::path::PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("fixtures/golden-policy-capsule/expected-bundle")
}

fn canonical_situation_dir() -> std::path::PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("fixtures/canonical-capsules/conflict-situation-capsule")
}

fn golden_build_request_path() -> std::path::PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("fixtures/golden-policy-capsule/build_request.json")
}

fn golden_source_pack_path() -> std::path::PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("fixtures/golden-policy-capsule/source-pack/source_pack.json")
}

fn golden_proposals_dir() -> std::path::PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("fixtures/golden-policy-capsule/proposals")
}

fn golden_review_decisions_dir() -> std::path::PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("fixtures/golden-policy-capsule/review-decisions")
}

#[test]
fn golden_policy_bundle_loads_and_validates() {
    let bundle =
        CapsuleBundle::load_from_dir(&golden_bundle_dir()).expect("golden bundle should load");

    let report = bundle.validate();

    assert!(!report.has_errors(), "{:#?}", report.findings);
    assert_eq!(bundle.manifest.schema_version, "0.1.0");
    assert_eq!(
        bundle.manifest.review_state,
        ReviewState::ApprovedWithCaveats
    );
    assert_eq!(bundle.source_ledger.len(), 5);
    assert_eq!(bundle.graph_slice.edges.len(), 4);
}

#[test]
fn canonical_v3_situation_capsule_loads_and_validates() {
    let package = PraxisCapsulePackage::load_from_dir(&canonical_situation_dir())
        .expect("canonical v3 fixture should load");

    let report = package.validate();

    assert!(!report.has_errors(), "{:#?}", report.findings);
    assert_eq!(package.manifest.spec_version, "3.0");
    assert_eq!(package.manifest.capsule_type, "situation");
    assert!(package
        .manifest
        .layers_present
        .iter()
        .any(|layer| layer == "reasoning"));
}

#[test]
fn golden_source_pack_loads_and_validates() {
    let source_pack =
        load_source_pack(&golden_source_pack_path()).expect("source pack fixture should load");

    let report = validate_source_pack(&source_pack);

    assert!(!report.has_errors(), "{:#?}", report.findings);
    assert_eq!(
        source_pack.target_capsule_type,
        Some(CapsuleType::Situation)
    );
    assert_eq!(source_pack.documents.len(), 2);
    assert_eq!(source_pack.spans.len(), 4);
}

#[test]
fn golden_proposal_set_routes_plus_review_gates() {
    let build_request =
        load_build_request(&golden_build_request_path()).expect("build request should load");
    let source_pack =
        load_source_pack(&golden_source_pack_path()).expect("source pack fixture should load");
    let proposal_set =
        ProposalSet::load_from_dir(&golden_proposals_dir()).expect("proposals should load");

    let report = validate_proposal_set(&source_pack, &build_request, &proposal_set);
    let gates = route_review_gates(&build_request, &proposal_set);
    let plan = plan_capsule_build(&build_request, &source_pack, &proposal_set);

    assert!(!report.has_errors(), "{:#?}", report.findings);
    assert_eq!(plan.selected_type, CapsuleType::Situation);
    assert_eq!(plan.source_document_count, 2);
    assert_eq!(plan.source_span_count, 4);
    assert_eq!(plan.proposal_count, 12);
    assert_eq!(plan.blocking_gate_count, 9);
    assert_eq!(gates.len(), 9);
}

#[test]
fn proposal_without_source_span_fails_validation() {
    let build_request =
        load_build_request(&golden_build_request_path()).expect("build request should load");
    let source_pack =
        load_source_pack(&golden_source_pack_path()).expect("source pack fixture should load");
    let mut proposal_set =
        ProposalSet::load_from_dir(&golden_proposals_dir()).expect("proposals should load");
    proposal_set.proposals[0].source_span_ids.clear();

    let report = validate_proposal_set(&source_pack, &build_request, &proposal_set);

    assert!(report.findings.iter().any(|finding| {
        finding.severity == BuildValidationSeverity::Error
            && finding.code == "proposal_missing_source_spans"
    }));
}

#[test]
fn golden_review_decisions_validate_and_promote_records() {
    let build_request =
        load_build_request(&golden_build_request_path()).expect("build request should load");
    let source_pack =
        load_source_pack(&golden_source_pack_path()).expect("source pack fixture should load");
    let proposal_set =
        ProposalSet::load_from_dir(&golden_proposals_dir()).expect("proposals should load");
    let decision_set = ReviewerDecisionSet::load_from_dir(&golden_review_decisions_dir())
        .expect("review decisions should load");

    let report =
        validate_reviewer_decision_set(&source_pack, &build_request, &proposal_set, &decision_set);
    let promoted = promote_records(&build_request, &source_pack, &proposal_set, &decision_set)
        .expect("golden decisions should promote");

    assert!(!report.has_errors(), "{:#?}", report.findings);
    assert!(promoted.ready_for_compiler);
    assert_eq!(promoted.required_decision_count, 9);
    assert_eq!(promoted.promoted_records.len(), 12);
    assert_eq!(promoted.caveated_record_count, 3);
    assert!(promoted
        .promoted_records
        .iter()
        .any(|record| record.object_id == "cav_army_leverage_uncorroborated"));
}

#[test]
fn missing_blocking_reviewer_decision_fails_validation() {
    let build_request =
        load_build_request(&golden_build_request_path()).expect("build request should load");
    let source_pack =
        load_source_pack(&golden_source_pack_path()).expect("source pack fixture should load");
    let proposal_set =
        ProposalSet::load_from_dir(&golden_proposals_dir()).expect("proposals should load");
    let mut decision_set = ReviewerDecisionSet::load_from_dir(&golden_review_decisions_dir())
        .expect("review decisions should load");
    decision_set
        .decisions
        .retain(|decision| decision.proposal_id != "prop_claim_certified_result");

    let report =
        validate_reviewer_decision_set(&source_pack, &build_request, &proposal_set, &decision_set);

    assert!(report.findings.iter().any(|finding| {
        finding.severity == BuildValidationSeverity::Error
            && finding.code == "missing_reviewer_decision"
    }));
}

#[test]
fn rejected_reviewer_decision_excludes_proposal_from_promotion() {
    let build_request =
        load_build_request(&golden_build_request_path()).expect("build request should load");
    let source_pack =
        load_source_pack(&golden_source_pack_path()).expect("source pack fixture should load");
    let proposal_set =
        ProposalSet::load_from_dir(&golden_proposals_dir()).expect("proposals should load");
    let mut decision_set = ReviewerDecisionSet::load_from_dir(&golden_review_decisions_dir())
        .expect("review decisions should load");
    let decision = decision_set
        .decisions
        .iter_mut()
        .find(|decision| decision.proposal_id == "prop_claim_army_rejected")
        .expect("fixture should include army rejection decision");
    decision.status = ReviewDecisionStatus::Reject;
    decision.caveats.clear();
    decision.rationale = "Rejected during test because corroboration is insufficient.".to_owned();

    let promoted = promote_records(&build_request, &source_pack, &proposal_set, &decision_set)
        .expect("rejected decision should still produce a promoted record set");

    assert!(promoted
        .rejected_proposal_ids
        .iter()
        .any(|proposal_id| proposal_id == "prop_claim_army_rejected"));
    assert!(!promoted
        .promoted_records
        .iter()
        .any(|record| record.object_id == "clm_army_rejected_certification"));
}

#[test]
fn drafted_golden_review_decisions_accept_human_edits() {
    let build_request =
        load_build_request(&golden_build_request_path()).expect("build request should load");
    let source_pack =
        load_source_pack(&golden_source_pack_path()).expect("source pack fixture should load");
    let proposal_set =
        ProposalSet::load_from_dir(&golden_proposals_dir()).expect("proposals should load");
    let mut decision_set =
        draft_reviewer_decision_set(&build_request, &proposal_set, "2026-06-10T00:00:00Z");

    let draft_report =
        validate_reviewer_decision_set(&source_pack, &build_request, &proposal_set, &decision_set);
    assert!(!draft_report.has_errors(), "{:#?}", draft_report.findings);
    assert_eq!(decision_set.decisions.len(), 9);
    assert!(decision_set.decisions.iter().all(|decision| decision.status
        == ReviewDecisionStatus::ApproveWithCaveats
        && !decision.caveats.is_empty()));

    let approved = decision_set
        .decisions
        .iter_mut()
        .find(|decision| decision.proposal_id == "prop_claim_certified_result")
        .expect("fixture should include certified-result decision");
    approved.status = ReviewDecisionStatus::Approve;
    approved.caveats.clear();
    approved.rationale =
        "Human reviewer approved this source-backed claim without draft caveats.".to_owned();

    let rejected = decision_set
        .decisions
        .iter_mut()
        .find(|decision| decision.proposal_id == "prop_claim_army_rejected")
        .expect("fixture should include army-rejection decision");
    rejected.status = ReviewDecisionStatus::Reject;
    rejected.caveats.clear();
    rejected.rationale =
        "Human reviewer rejected this claim for promoted PRAXIS context.".to_owned();

    let edited_report =
        validate_reviewer_decision_set(&source_pack, &build_request, &proposal_set, &decision_set);
    let promoted = promote_records(&build_request, &source_pack, &proposal_set, &decision_set)
        .expect("edited draft decisions should promote");
    let certified = promoted
        .promoted_records
        .iter()
        .find(|record| record.object_id == "clm_commission_certified_result")
        .expect("approved claim should remain promoted");

    assert!(!edited_report.has_errors(), "{:#?}", edited_report.findings);
    assert_eq!(certified.review_status, Some(ReviewDecisionStatus::Approve));
    assert!(certified.caveats.is_empty());
    assert!(promoted
        .rejected_proposal_ids
        .iter()
        .any(|proposal_id| proposal_id == "prop_claim_army_rejected"));
    assert!(!promoted
        .promoted_records
        .iter()
        .any(|record| record.object_id == "clm_army_rejected_certification"));
}

#[test]
fn canonical_v3_capsule_rejects_extra_macro_types() {
    let mut package = PraxisCapsulePackage::load_from_dir(&canonical_situation_dir())
        .expect("canonical v3 fixture should load");
    package.manifest.capsule_type = "stakeholder".to_owned();

    let report = package.validate();

    assert!(report.findings.iter().any(|finding| {
        finding.code == "unsupported_capsule_type"
            && finding.severity == ValidationSeverity::Error
            && finding.path == "manifest.type"
    }));
}

#[test]
fn graph_edge_without_source_or_review_fails_validation() {
    let mut bundle =
        CapsuleBundle::load_from_dir(&golden_bundle_dir()).expect("golden bundle should load");

    let edge = bundle
        .graph_slice
        .edges
        .first_mut()
        .expect("fixture should contain at least one edge");
    edge.source_span_ids.clear();
    edge.review_action_ids.clear();

    let report = bundle.validate();

    assert!(report.findings.iter().any(|finding| {
        finding.code == "missing_graph_edge_provenance"
            && finding.severity == ValidationSeverity::Error
            && finding.path == "graph_slice.edges[0].source_span_ids"
    }));
}

#[test]
fn stale_temporal_claim_produces_warning_not_error() {
    let bundle =
        CapsuleBundle::load_from_dir(&golden_bundle_dir()).expect("golden bundle should load");

    let report = bundle.validate();

    assert!(!report.has_errors(), "{:#?}", report.findings);
    assert!(report.findings.iter().any(|finding| {
        finding.code == "stale_temporal_claim"
            && finding.severity == ValidationSeverity::Warning
            && finding.path == "temporal_ledger[1].status"
    }));
}

#[test]
fn schema_export_writes_required_snapshots() {
    let output_dir =
        std::env::temp_dir().join(format!("dialectica-schema-test-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&output_dir);

    export_schema_dir(&output_dir).expect("schema export should succeed");

    let manifest_schema = output_dir.join("manifest.schema.json");
    let v3_manifest_schema = output_dir.join("praxis_capsule_manifest.schema.json");
    let capsule_schema = output_dir.join("capsule.schema.json");
    let ontology_blueprint_schema = output_dir.join("ontology_blueprint.schema.json");

    assert!(manifest_schema.exists());
    assert!(v3_manifest_schema.exists());
    assert!(capsule_schema.exists());
    assert!(ontology_blueprint_schema.exists());
    assert!(std::fs::read_to_string(manifest_schema)
        .expect("manifest schema should be readable")
        .contains("CapsuleManifest"));

    std::fs::remove_dir_all(output_dir).expect("temp schema dir should clean up");
}

#[test]
fn extractor_schema_export_writes_builder_contracts() {
    let output_dir = std::env::temp_dir().join(format!(
        "dialectica-extractor-schema-test-{}",
        std::process::id()
    ));
    let _ = std::fs::remove_dir_all(&output_dir);

    export_extractor_schema_dir(&output_dir).expect("extractor schema export should succeed");

    assert!(output_dir
        .join("capsule_build_request.schema.json")
        .exists());
    assert!(output_dir.join("source_pack.schema.json").exists());
    assert!(output_dir.join("extraction_run.schema.json").exists());
    assert!(output_dir.join("extraction_proposal.schema.json").exists());
    assert!(output_dir.join("proposal_set.schema.json").exists());
    assert!(output_dir.join("review_gate.schema.json").exists());
    assert!(output_dir
        .join("reviewer_decision_set.schema.json")
        .exists());
    assert!(output_dir.join("promoted_record_set.schema.json").exists());

    std::fs::remove_dir_all(output_dir).expect("temp schema dir should clean up");
}

#[test]
fn compiler_schema_export_writes_diff_contract() {
    let output_dir = std::env::temp_dir().join(format!(
        "dialectica-compiler-schema-test-{}",
        std::process::id()
    ));
    let _ = std::fs::remove_dir_all(&output_dir);

    export_compiler_schema_dir(&output_dir).expect("compiler schema export should succeed");

    let diff_schema = output_dir.join("capsule_diff.schema.json");
    let integrity_schema = output_dir.join("integrity_envelope.schema.json");
    assert!(diff_schema.exists());
    assert!(integrity_schema.exists());
    assert!(std::fs::read_to_string(diff_schema)
        .expect("diff schema should be readable")
        .contains("CapsuleDiff"));
    assert!(std::fs::read_to_string(integrity_schema)
        .expect("integrity schema should be readable")
        .contains("IntegrityEnvelope"));

    std::fs::remove_dir_all(output_dir).expect("temp schema dir should clean up");
}

#[test]
fn situation_capsule_ontology_plan_uses_policy_lens() {
    let bundle =
        CapsuleBundle::load_from_dir(&golden_bundle_dir()).expect("golden bundle should load");

    let blueprint = bundle.ontology_blueprint();

    assert_eq!(blueprint.ontology_family, "situation_policy_ontology");
    assert!(blueprint
        .semantic_layers
        .iter()
        .any(|layer| layer.layer_id == "actor_claim_temporal_graph"));
    assert!(blueprint
        .praxis_context_guidance
        .iter()
        .any(|guidance| guidance.contains("source-backed situation graph")));
}

#[test]
fn user_capsule_ontology_plan_does_not_use_situation_defaults() {
    let manifest = CapsuleManifest::new(
        "cap_user_fixture",
        "User context fixture",
        "user_capsule",
        ReviewState::Approved,
        "sha256:user",
    );

    let blueprint = manifest.ontology_blueprint();

    assert_eq!(blueprint.ontology_family, "user_context_ontology");
    assert!(!blueprint
        .semantic_layers
        .iter()
        .any(|layer| layer.layer_id == "actor_claim_temporal_graph"));
    assert!(blueprint
        .semantic_layers
        .iter()
        .any(|layer| layer.layer_id == "preference_and_authority_layer"));
}

#[test]
fn ontology_plan_covers_documented_capsule_families() {
    let cases = [
        ("user_capsule", "user_context_ontology"),
        ("situation_capsule", "situation_policy_ontology"),
        ("tool_capsule", "tool_method_ontology"),
        ("output_capsule", "output_trace_ontology"),
    ];

    for (capsule_type, expected_family) in cases {
        let manifest = CapsuleManifest::new(
            "cap_family_fixture",
            "Family fixture",
            capsule_type,
            ReviewState::Approved,
            "sha256:family",
        );

        let blueprint = manifest.ontology_blueprint();

        assert_eq!(
            blueprint.ontology_family, expected_family,
            "capsule type {capsule_type} should map to documented ontology family"
        );
        assert!(
            blueprint
                .semantic_layers
                .iter()
                .any(|layer| layer.layer_id == "sourceability_layer"),
            "capsule type {capsule_type} should include universal sourceability layer"
        );
    }
}

#[test]
fn unsupported_top_level_capsule_type_fails_validation() {
    let mut bundle =
        CapsuleBundle::load_from_dir(&golden_bundle_dir()).expect("golden bundle should load");
    bundle.manifest.capsule_type = "stakeholder_capsule".to_owned();

    let report = bundle.validate();

    assert!(report.findings.iter().any(|finding| {
        finding.code == "unsupported_capsule_type"
            && finding.severity == ValidationSeverity::Error
            && finding.path == "manifest.capsule_type"
    }));
}
