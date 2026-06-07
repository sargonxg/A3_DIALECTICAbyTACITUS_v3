import unittest
from pathlib import Path
import sys

sys.path.insert(0, str(Path(__file__).resolve().parents[1]))

from dialectica_tools import summarize_manifest


class CapsuleReportTests(unittest.TestCase):
    def test_reviewed_manifest_is_marked_reviewed(self) -> None:
        report = summarize_manifest(
            {
                "capsule_id": "cap_001",
                "title": "Fixture capsule",
                "capsule_type": "situation_capsule",
                "review_state": "approved_with_caveats",
                "source_count": 5,
            }
        )

        self.assertTrue(report.is_reviewed)
        self.assertEqual(report.source_count, 5)


if __name__ == "__main__":
    unittest.main()
