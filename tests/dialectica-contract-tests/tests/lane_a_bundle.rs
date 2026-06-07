use std::path::Path;

use dialectica_capsule::{export_schema_dir, CapsuleBundle, ReviewState, ValidationSeverity};

fn golden_bundle_dir() -> std::path::PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("fixtures/golden-policy-capsule/expected-bundle")
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
    let capsule_schema = output_dir.join("capsule.schema.json");

    assert!(manifest_schema.exists());
    assert!(capsule_schema.exists());
    assert!(std::fs::read_to_string(manifest_schema)
        .expect("manifest schema should be readable")
        .contains("CapsuleManifest"));

    std::fs::remove_dir_all(output_dir).expect("temp schema dir should clean up");
}
