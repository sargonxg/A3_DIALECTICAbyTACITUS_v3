//! Evaluation scaffold for capsule quality and PRAXIS outcome checks.

/// A deterministic check result for early contract tests.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EvalCheck {
    /// Stable check id.
    pub id: &'static str,
    /// Whether the check passed.
    pub passed: bool,
    /// Human-readable summary.
    pub summary: &'static str,
}

impl EvalCheck {
    /// Creates a passing check.
    pub const fn pass(id: &'static str, summary: &'static str) -> Self {
        Self {
            id,
            passed: true,
            summary,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::EvalCheck;

    #[test]
    fn pass_helper_marks_check_successful() {
        let check = EvalCheck::pass("capsule_schema", "schema validates");
        assert!(check.passed);
    }
}
