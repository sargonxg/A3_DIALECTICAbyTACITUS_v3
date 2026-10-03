use dialectica_compiler::{
    compile_cognitive_draft, write_native_cognitive_capsule, CognitiveDraftRequest,
};
use dialectica_extractor::SourcePack;
use serde_json::{json, Value};
use sha2::{Digest, Sha256};

fn inputs() -> (CognitiveDraftRequest, SourcePack) {
    let quote = "We are prepared to consider the proposal subject to monitoring.";
    let hash = format!("sha256:{:x}", Sha256::digest(quote.as_bytes()));
    let source: SourcePack = serde_json::from_value(json!({"pack_id":"fixture-source", "contract_version":"0.1.0", "title":"Synthetic diplomatic statement", "created_at":"2026-10-03T00:00:00Z", "documents":[{"document_id":"doc-1", "source_type":"official_statement", "title":"Synthetic statement", "uri":"fixture://statement", "retrieved_at":"2026-10-03T00:00:00Z", "language":"en", "rights_or_access":"private_fixture", "content_hash":hash, "trust_status":"synthetic", "prompt_injection_risk":"low"}], "spans":[{"span_id":"e1", "document_id":"doc-1", "locator":"paragraph:1", "text_hash":hash, "quote":quote, "language":"en", "rights_or_access":"private_fixture"}]})).expect("fixture source pack");
    let request: CognitiveDraftRequest = serde_json::from_value(json!({"contractVersion":"dialectica-cognitive-draft/1", "capsuleId":"tacitus/synthetic-diplomacy", "version":"0.1.0", "profile":"language", "nodes":[], "statements":[], "knowledge":[{"id":"language-1", "kind":"languageRule", "label":"Considering is not accepting", "scope":["diplomacy"], "expression":"prepared to consider", "language":"en", "domain":"diplomacy", "meaning":"Conditional willingness to examine a proposal.", "nonImplications":["Formal acceptance"], "reviewState":"proposed", "provenance":{"generatedBy":{"type":"human", "id":"fixture-expert", "at":"2026-10-03T00:00:00Z"}, "sourceRefs":["e1"], "confidenceKind":"human-authored"}}]})).expect("fixture request");
    (request, source)
}

#[test]
fn draft_preserves_explicit_knowledge_and_verifiable_source_versions() {
    let (request, source) = inputs();
    let a = compile_cognitive_draft(&request, &source).expect("draft compilation");
    let b = compile_cognitive_draft(&request, &source).expect("repeated draft");
    assert_eq!(
        serde_json::to_vec(&a).expect("JSON"),
        serde_json::to_vec(&b).expect("JSON")
    );
    assert_eq!(a["schemaVersion"], "cognitive-capsule/vNext");
    assert_eq!(a["evidence"][0]["sourceId"], "doc-1");
    assert_eq!(
        a["evidence"][0]["sourceVersion"],
        source.documents[0].content_hash
    );
    assert_eq!(a["evidence"][0]["quote"], source.spans[0].quote);
    assert_eq!(a["knowledge"][0]["reviewState"], "proposed");
    assert_eq!(a["rights"]["mode"], "private");
    assert!(a.get("documents").is_none());
}

#[test]
fn draft_rejects_hash_tampering_unknown_anchors_and_accepted_records() {
    let (mut request, mut source) = inputs();
    source.spans[0].quote.push_str("tampered");
    assert!(compile_cognitive_draft(&request, &source).is_err());
    let (_, source) = inputs();
    request.knowledge[0]["provenance"]["sourceRefs"] = json!(["missing"]);
    assert!(compile_cognitive_draft(&request, &source).is_err());
    let (mut request, source) = inputs();
    request.knowledge[0]["reviewState"] = json!("accepted");
    assert!(compile_cognitive_draft(&request, &source).is_err());
    let (request, mut source) = inputs();
    source.documents.clear();
    assert!(compile_cognitive_draft(&request, &source).is_err());
}

#[test]
fn native_writer_requires_explicit_write_gate_before_any_tool_invocation() {
    let (request, source) = inputs();
    let draft = compile_cognitive_draft(&request, &source).expect("draft");
    assert!(write_native_cognitive_capsule(
        &draft,
        std::path::Path::new("missing-checkout"),
        std::path::Path::new("unused.capsule"),
        false
    )
    .is_err());
}

#[test]
#[ignore = "Requires an explicitly supplied installed PRAXIS_ROOT checkout"]
fn native_bridge_roundtrips_with_shared_praxis_schema() {
    let root = std::env::var_os("PRAXIS_ROOT").expect("PRAXIS_ROOT");
    let (request, source) = inputs();
    let draft = compile_cognitive_draft(&request, &source).expect("draft");
    let path = std::env::temp_dir().join(format!(
        "dialectica-cognitive-{}.capsule",
        std::process::id()
    ));
    let receipt = write_native_cognitive_capsule(&draft, std::path::Path::new(&root), &path, true)
        .expect("native export");
    assert!(receipt.archive_digest.starts_with("sha256:"));
    let file = std::fs::File::open(&path).expect("archive");
    let mut archive = zip::ZipArchive::new(file).expect("ZIP");
    use std::io::Read;
    let mut manifest = String::new();
    archive
        .by_name("manifest.json")
        .expect("manifest")
        .read_to_string(&mut manifest)
        .expect("manifest text");
    let manifest: Value = serde_json::from_str(&manifest).expect("manifest JSON");
    assert_eq!(manifest["schemaVersion"], "cognitive-capsule/vNext");
    assert_eq!(manifest["capsuleId"], "tacitus/synthetic-diplomacy");
    drop(archive);
    std::fs::remove_file(path).expect("owned artifact cleanup");
}
