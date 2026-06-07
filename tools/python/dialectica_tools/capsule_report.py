"""Small report helpers for local capsule fixture inspection."""

from __future__ import annotations

from dataclasses import dataclass
from typing import Mapping


@dataclass(frozen=True)
class CapsuleReport:
    """Human-readable summary extracted from a capsule manifest."""

    capsule_id: str
    title: str
    capsule_type: str
    review_state: str
    source_count: int

    @property
    def is_reviewed(self) -> bool:
        """Return whether the review state is usable for PRAXIS workflows."""
        return self.review_state in {
            "approved",
            "approved_with_caveats",
            "promoted",
        }


def summarize_manifest(manifest: Mapping[str, object]) -> CapsuleReport:
    """Build a compact report from a manifest-like mapping."""
    return CapsuleReport(
        capsule_id=str(manifest.get("capsule_id", "")),
        title=str(manifest.get("title", "")),
        capsule_type=str(manifest.get("capsule_type", "")),
        review_state=str(manifest.get("review_state", "draft")),
        source_count=int(manifest.get("source_count", 0) or 0),
    )
