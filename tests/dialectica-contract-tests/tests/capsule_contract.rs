use dialectica_capsule::{CapsuleManifest, ReviewState, CAPSULE_SCHEMA_VERSION};

#[test]
fn scaffold_manifest_uses_current_schema_and_review_gate() {
    let manifest = CapsuleManifest::new(
        "cap_contract_001",
        "Contract test capsule",
        "situation_capsule",
        ReviewState::Promoted,
        "sha256:contract",
    );

    assert_eq!(manifest.schema_version, CAPSULE_SCHEMA_VERSION);
    assert!(dialectica_compiler::can_emit_bundle(&manifest));
}
