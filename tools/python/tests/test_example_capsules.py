import json
import unittest
from pathlib import Path


REPO_ROOT = Path(__file__).resolve().parents[3]
EXAMPLE_DIR = REPO_ROOT / "fixtures" / "example-capsules"

REQUIRED_SECTIONS = {
    "manifest",
    "capsule",
    "source_ledger",
    "temporal_ledger",
    "ontology_slice",
    "graph_slice",
    "graph_semantics",
    "graph_constraints",
    "reasoning_playbook",
    "agent_guidance",
    "retrieval_pack",
    "output_contracts",
    "review_ledger",
    "rights_profile",
    "marketplace_listing",
    "capsule_health",
    "eval_report",
    "checksums",
    "signature",
}


class ExampleCapsuleTests(unittest.TestCase):
    def test_example_capsules_share_required_bundle_sections(self) -> None:
        examples = sorted(EXAMPLE_DIR.glob("*.example.json"))
        self.assertEqual(len(examples), 4)

        for example in examples:
            with self.subTest(example=example.name):
                payload = json.loads(example.read_text(encoding="utf-8"))
                missing = REQUIRED_SECTIONS.difference(payload)
                self.assertFalse(missing, f"{example.name} missing {sorted(missing)}")
                self.assertEqual(payload["manifest"]["schema_version"], "0.1.0")
                self.assertEqual(
                    payload["manifest"]["capsule_id"],
                    payload["graph_slice"]["capsule_id"],
                )
                self.assertTrue(payload["agent_guidance"]["allowed_workflows"])
                self.assertTrue(payload["graph_slice"]["nodes"])
                self.assertTrue(payload["graph_slice"]["edges"])


if __name__ == "__main__":
    unittest.main()
