use std::{
    fs,
    path::{Path, PathBuf},
};

use dialectica_compiler::{compile_fixture, compile_from_parts, write_capsule_diff};
use dialectica_extractor::{
    load_build_request, load_source_pack, ProposalSet, ReviewDecisionStatus, ReviewerDecisionSet,
};
use serde_json::Value;

#[test]
fn golden_policy_diff_matches_expected_output_byte_for_byte() {
    let old_out = fresh_temp_dir("dialectica_contract_diff_old");
    let new_out = fresh_temp_dir("dialectica_contract_diff_new");
    let diff_out = fresh_temp_dir("dialectica_contract_diff_output");
    compile_fixture(&golden_fixture_dir(), &old_out).expect("old fixture should compile");

    let build_request = load_build_request(&golden_fixture_dir().join("build_request.json"))
        .expect("build request should load");
    let source_pack = load_source_pack(&golden_fixture_dir().join("source-pack/source_pack.json"))
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

    write_capsule_diff(&old_out, &new_out, &diff_out).expect("diff should write");

    assert_eq!(
        fs::read_to_string(diff_out.join("diff.json")).expect("diff should read"),
        fs::read_to_string(golden_expected_diff_dir().join("diff.json"))
            .expect("expected diff should read")
    );
    assert_eq!(
        fs::read_to_string(diff_out.join("change-memo.md")).expect("memo should read"),
        fs::read_to_string(golden_expected_diff_dir().join("change-memo.md"))
            .expect("expected memo should read")
    );

    let _ = fs::remove_dir_all(old_out);
    let _ = fs::remove_dir_all(new_out);
    let _ = fs::remove_dir_all(diff_out);
}

fn golden_fixture_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("fixtures/golden-policy-capsule")
}

fn golden_expected_diff_dir() -> PathBuf {
    golden_fixture_dir().join("expected-diff")
}

fn fresh_temp_dir(name: &str) -> PathBuf {
    let root = std::env::temp_dir().join(format!("{}_{}", name, std::process::id()));
    let _ = fs::remove_dir_all(&root);
    root
}

fn revise_certified_claim(package_dir: &Path) {
    let claims_path = package_dir.join("claims.jsonl");
    let mut claims = fs::read_to_string(&claims_path)
        .expect("claims should read")
        .lines()
        .map(|line| serde_json::from_str::<Value>(line).expect("claim should parse"))
        .collect::<Vec<_>>();
    for claim in &mut claims {
        if claim.get("claim_id").and_then(Value::as_str) == Some("clm_commission_certified_result")
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
