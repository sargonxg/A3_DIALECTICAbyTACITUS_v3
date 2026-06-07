//! Deterministic capsule compiler scaffold.
//!
//! The compiler will assemble signed bundle directories from canonical records.

use dialectica_capsule::CapsuleManifest;

/// Returns true when the compiler may emit a signed bundle for this manifest.
pub fn can_emit_bundle(manifest: &CapsuleManifest) -> bool {
    manifest.is_export_ready()
}

/// Current scaffold compiler identifier.
pub const COMPILER_ID: &str = "dialectica-compiler/0.1.0";

#[cfg(test)]
mod tests {
    use dialectica_capsule::{CapsuleManifest, ReviewState};

    use super::can_emit_bundle;

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
}
