//! Deterministic MVP evaluation for capsule quality and PRAXIS handoff use.

use std::{
    error::Error,
    fmt::{Display, Formatter},
    path::Path,
};

use dialectica_capsule::{PraxisCapsulePackage, ValidationSeverity};
use dialectica_compiler::export_praxis_context_pack;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EvalCheck {
    pub id: String,
    pub passed: bool,
    pub summary: String,
}

impl EvalCheck {
    pub fn pass(id: impl Into<String>, summary: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            passed: true,
            summary: summary.into(),
        }
    }

    pub fn fail(id: impl Into<String>, summary: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            passed: false,
            summary: summary.into(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EvalReport {
    pub schema_version: String,
    pub capsule_id: String,
    pub workflow: String,
    pub passed: bool,
    pub score: u8,
    pub checks: Vec<EvalCheck>,
}

#[derive(Debug)]
pub enum EvalError {
    Capsule(String),
    Compiler(String),
}

impl Display for EvalError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Capsule(message) | Self::Compiler(message) => formatter.write_str(message),
        }
    }
}

impl Error for EvalError {}

pub fn evaluate_praxis_mvp(package_dir: &Path, workflow: &str) -> Result<EvalReport, EvalError> {
    let package = PraxisCapsulePackage::load_from_dir(package_dir)
        .map_err(|error| EvalError::Capsule(error.to_string()))?;
    let validation = package.validate();
    let pack = export_praxis_context_pack(package_dir, workflow)
        .map_err(|error| EvalError::Compiler(error.to_string()))?;

    let mut checks = Vec::new();
    let error_count = validation
        .findings
        .iter()
        .filter(|finding| finding.severity == ValidationSeverity::Error)
        .count();
    checks.push(if error_count == 0 {
        EvalCheck::pass(
            "capsule_validity",
            "capsule validation has no error findings",
        )
    } else {
        EvalCheck::fail(
            "capsule_validity",
            format!("capsule validation has {error_count} error findings"),
        )
    });

    checks.push(
        if pack.agent_context.trim().len() >= 80 && pack.operations.contains("seek") {
            EvalCheck::pass(
                "agent_handoff_text",
                "agent_context and operations are populated for PRAXIS",
            )
        } else {
            EvalCheck::fail(
                "agent_handoff_text",
                "agent_context or operations are too thin for PRAXIS",
            )
        },
    );

    checks.push(if !pack.retrieval_records.is_empty() {
        EvalCheck::pass(
            "retrieval_records",
            format!(
                "{} retrieval records available",
                pack.retrieval_records.len()
            ),
        )
    } else {
        EvalCheck::fail("retrieval_records", "no retrieval records available")
    });

    checks.push(
        if retrieval_records_have_source_receipts(&pack.retrieval_records) {
            EvalCheck::pass(
                "source_fidelity",
                "every retrieval record carries a source span receipt",
            )
        } else {
            EvalCheck::fail(
                "source_fidelity",
                "one or more retrieval records lack source span receipts",
            )
        },
    );

    checks.push(if !pack.reasoning_devices.is_empty() {
        EvalCheck::pass("reasoning_devices", "reasoning devices are available")
    } else {
        EvalCheck::fail("reasoning_devices", "no reasoning devices available")
    });

    checks.push(if !pack.graph_focus.is_empty() {
        EvalCheck::pass("graph_focus", "graph focus records are available")
    } else {
        EvalCheck::fail("graph_focus", "no graph focus records available")
    });

    checks.push(
        if review_or_caveat_visible(&pack.review_state, &pack.caveats, &pack.retrieval_records) {
            EvalCheck::pass(
                "review_caveat_visibility",
                "review state or caveats are visible to PRAXIS",
            )
        } else {
            EvalCheck::fail(
                "review_caveat_visibility",
                "review state and caveats are not visible enough",
            )
        },
    );

    let passed_count = checks.iter().filter(|check| check.passed).count();
    let score = ((passed_count * 100) / checks.len()) as u8;
    Ok(EvalReport {
        schema_version: "dialectica_eval_report_v1".to_owned(),
        capsule_id: pack.capsule_id,
        workflow: pack.workflow,
        passed: checks.iter().all(|check| check.passed),
        score,
        checks,
    })
}

fn retrieval_records_have_source_receipts(records: &[Value]) -> bool {
    records.iter().all(|record| {
        record
            .get("source_span_ids")
            .and_then(Value::as_array)
            .map(|values| !values.is_empty())
            .unwrap_or(false)
            || record.get("source_span").is_some()
            || record
                .get("source_receipts")
                .and_then(Value::as_array)
                .map(|values| !values.is_empty())
                .unwrap_or(false)
    })
}

fn review_or_caveat_visible(review_state: &str, caveats: &[String], records: &[Value]) -> bool {
    !review_state.trim().is_empty()
        || !caveats.is_empty()
        || records.iter().any(|record| {
            record
                .get("caveats")
                .and_then(Value::as_array)
                .map(|values| !values.is_empty())
                .unwrap_or(false)
                || record.get("review_state").and_then(Value::as_str).is_some()
        })
}

#[cfg(test)]
mod tests {
    use std::path::{Path, PathBuf};

    use super::{evaluate_praxis_mvp, EvalCheck};

    #[test]
    fn pass_helper_marks_check_successful() {
        let check = EvalCheck::pass("capsule_schema", "schema validates");
        assert!(check.passed);
    }

    #[test]
    fn canonical_capsule_passes_praxis_mvp_eval() {
        let report = evaluate_praxis_mvp(&canonical_capsule_fixture(), "conflict_map")
            .expect("fixture should evaluate");

        assert!(report.passed, "{:#?}", report.checks);
        assert_eq!(report.schema_version, "dialectica_eval_report_v1");
        assert_eq!(report.workflow, "conflict_map");
        assert!(report
            .checks
            .iter()
            .any(|check| check.id == "source_fidelity" && check.passed));
        assert!(report.score >= 80);
    }

    fn canonical_capsule_fixture() -> PathBuf {
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../..")
            .join("fixtures/canonical-capsules/conflict-situation-capsule")
    }
}
