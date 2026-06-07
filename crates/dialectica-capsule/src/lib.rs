//! Core PRAXIS Capsule contract primitives.
//!
//! This crate owns the portable bundle vocabulary before ingestion, storage,
//! graph adapters, or model-powered extraction are added.

/// Current capsule schema version used by scaffold fixtures and API contracts.
pub const CAPSULE_SCHEMA_VERSION: &str = "0.1.0";

/// Human-review state for a capsule or one of its internal objects.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReviewState {
    /// Author-created or machine-proposed, not yet inspected.
    Draft,
    /// A reviewer must inspect before promotion.
    NeedsReview,
    /// Approved for the recorded scope.
    Approved,
    /// Approved, but caveats must travel with the capsule.
    ApprovedWithCaveats,
    /// Blocked by review.
    Rejected,
    /// Approved for PRAXIS use.
    Promoted,
}

impl ReviewState {
    /// Returns true when the state allows a capsule to be used in PRAXIS flows.
    pub const fn allows_praxis_use(self) -> bool {
        matches!(
            self,
            Self::Approved | Self::ApprovedWithCaveats | Self::Promoted
        )
    }
}

/// Minimal manifest shape used by the first workspace smoke tests.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CapsuleManifest<'a> {
    /// Stable capsule id.
    pub capsule_id: &'a str,
    /// Human-readable capsule title.
    pub title: &'a str,
    /// Capsule category, such as `situation_capsule`.
    pub capsule_type: &'a str,
    /// Schema version.
    pub schema_version: &'a str,
    /// Review state at export time.
    pub review_state: ReviewState,
    /// Content digest for the signed bundle.
    pub bundle_digest: &'a str,
}

impl<'a> CapsuleManifest<'a> {
    /// Creates a minimal manifest for tests and scaffolded binaries.
    pub const fn new(
        capsule_id: &'a str,
        title: &'a str,
        capsule_type: &'a str,
        review_state: ReviewState,
        bundle_digest: &'a str,
    ) -> Self {
        Self {
            capsule_id,
            title,
            capsule_type,
            schema_version: CAPSULE_SCHEMA_VERSION,
            review_state,
            bundle_digest,
        }
    }

    /// Returns true when required manifest identity fields are present.
    pub fn has_required_identity(&self) -> bool {
        !self.capsule_id.trim().is_empty()
            && !self.title.trim().is_empty()
            && !self.capsule_type.trim().is_empty()
            && !self.schema_version.trim().is_empty()
    }

    /// Returns true when the manifest can be exported as a PRAXIS-visible bundle.
    pub fn is_export_ready(&self) -> bool {
        self.has_required_identity()
            && self.review_state.allows_praxis_use()
            && !self.bundle_digest.trim().is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::{CapsuleManifest, ReviewState};

    #[test]
    fn approved_manifest_with_digest_is_export_ready() {
        let manifest = CapsuleManifest::new(
            "cap_stakeholder_001",
            "Stakeholder capsule",
            "stakeholder_capsule",
            ReviewState::Approved,
            "sha256:test",
        );

        assert!(manifest.is_export_ready());
    }

    #[test]
    fn draft_manifest_is_not_export_ready() {
        let manifest = CapsuleManifest::new(
            "cap_stakeholder_001",
            "Stakeholder capsule",
            "stakeholder_capsule",
            ReviewState::Draft,
            "sha256:test",
        );

        assert!(!manifest.is_export_ready());
    }
}
