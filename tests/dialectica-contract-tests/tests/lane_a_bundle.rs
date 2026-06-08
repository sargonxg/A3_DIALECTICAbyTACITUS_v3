use std::path::Path;

use dialectica_capsule::{
    export_schema_dir, CapsuleBundle, CapsuleManifest, PraxisCapsulePackage, ReviewState,
    ValidationSeverity,
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
