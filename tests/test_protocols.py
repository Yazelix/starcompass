from __future__ import annotations

import json
import shutil
import tempfile
import unittest
from pathlib import Path

from scripts import protocols


ROOT = Path(__file__).resolve().parents[1]
COMMIT = "0123456789abcdef0123456789abcdef01234567"


class ProtocolRepositoryTests(unittest.TestCase):
    def test_repository_and_all_profiles_are_valid_and_render_deterministically(
        self,
    ) -> None:
        canonical, profiles = protocols.validate_repository(ROOT)

        self.assertEqual(18, len(canonical))
        self.assertEqual(
            {"fork", "greenfield", "orchestrator", "terminal-engine"}, set(profiles)
        )
        for profile in profiles:
            first = protocols.assemble_import(
                ROOT,
                [profile],
                "Local truth.\n",
                None,
                protocols.SOURCE_REPOSITORY,
                COMMIT,
            )
            second = protocols.assemble_import(
                ROOT,
                [profile],
                "Local truth.\n",
                None,
                protocols.SOURCE_REPOSITORY,
                COMMIT,
            )
            self.assertEqual(first, second)
            manifest, agents = first
            self.assertIn(f"- Profiles: `{profile}`", agents)
            self.assertIn("Do not edit this generated file directly.", agents)
            self.assertIn("## Repository-local rules\n\nLocal truth.", agents)
            for imported in manifest["protocols"]:
                self.assertIn(f"### {imported['id']} ", agents)

    def test_exception_is_explicit_in_manifest_and_self_contained_agents(self) -> None:
        exceptions = {
            "schema": 1,
            "exceptions": [
                {
                    "protocol": "AP-CI-001",
                    "scope": "private alpha",
                    "reason": "Hosted jobs are disabled while cost is measured.",
                    "approved_by": "user",
                    "approved_at": "2026-08-04",
                    "review_condition": "Review before public beta.",
                }
            ],
        }

        manifest, agents = protocols.assemble_import(
            ROOT,
            ["greenfield"],
            "Local truth.\n",
            exceptions,
            protocols.SOURCE_REPOSITORY,
            COMMIT,
        )

        self.assertEqual("AP-CI-001", manifest["exceptions"][0]["protocol"])
        self.assertIn("Hosted jobs are disabled", agents)
        self.assertIn("Approved by user on 2026-08-04", agents)

    def test_protocol_content_change_invalidates_generated_index(self) -> None:
        with tempfile.TemporaryDirectory() as temporary:
            root = self.copy_catalog(Path(temporary))
            path = root / "protocols" / "AP-SCOPE-001.md"
            path.write_text(
                path.read_text(encoding="utf-8") + "\nDrift.\n", encoding="utf-8"
            )

            with self.assertRaisesRegex(protocols.ProtocolError, "index.json is stale"):
                protocols.validate_repository(root)

    def test_protocol_history_requires_permanent_ids_and_version_bumps(self) -> None:
        current = protocols.expected_index(ROOT)
        unchanged = json.loads(json.dumps(current))
        protocols.validate_version_transition(unchanged, current)

        unchanged["protocols"][0]["sha256"] = "0" * 64
        with self.assertRaisesRegex(protocols.ProtocolError, "without a version bump"):
            protocols.validate_version_transition(unchanged, current)

        older = json.loads(json.dumps(current))
        older["protocols"][0]["sha256"] = "0" * 64
        older["protocols"][0]["version"] = 0
        protocols.validate_version_transition(older, current)

        missing = json.loads(json.dumps(current))
        missing["protocols"].append(
            {
                "id": "AP-REMOVED-999",
                "version": 1,
                "title": "Removed",
                "path": "protocols/AP-REMOVED-999.md",
                "sha256": "0" * 64,
            }
        )
        with self.assertRaisesRegex(protocols.ProtocolError, "cannot be removed"):
            protocols.validate_version_transition(missing, current)

    def test_unknown_profile_protocol_is_rejected(self) -> None:
        with tempfile.TemporaryDirectory() as temporary:
            root = self.copy_catalog(Path(temporary))
            path = root / "profiles" / "greenfield.json"
            profile = json.loads(path.read_text(encoding="utf-8"))
            profile["protocols"].append("AP-NOT-REAL-999")
            path.write_text(protocols.json_text(profile), encoding="utf-8")

            with self.assertRaisesRegex(protocols.ProtocolError, "unknown protocols"):
                protocols.validate_repository(root)

    def test_manifest_detects_agents_overlay_and_source_drift(self) -> None:
        overlay = "Local truth.\n"
        manifest, agents = protocols.assemble_import(
            ROOT, ["greenfield"], overlay, None, protocols.SOURCE_REPOSITORY, COMMIT
        )
        protocols.check_import(ROOT, manifest, overlay, agents, COMMIT)

        cases = (
            (manifest, overlay, agents + "manual edit\n", COMMIT, "AGENTS.md drifted"),
            (manifest, "Changed local truth.\n", agents, COMMIT, "manifest drifted"),
            (manifest, overlay, agents, "f" * 40, "source checkout"),
        )
        for (
            candidate_manifest,
            candidate_overlay,
            candidate_agents,
            commit,
            message,
        ) in cases:
            with self.subTest(message=message):
                with self.assertRaisesRegex(protocols.ProtocolError, message):
                    protocols.check_import(
                        ROOT,
                        candidate_manifest,
                        candidate_overlay,
                        candidate_agents,
                        commit,
                    )

    def test_exception_authoring_file_is_part_of_the_drift_check(self) -> None:
        empty = {"schema": 1, "exceptions": []}
        manifest, agents = protocols.assemble_import(
            ROOT,
            ["greenfield"],
            "Local truth.\n",
            empty,
            protocols.SOURCE_REPOSITORY,
            COMMIT,
        )
        changed = {
            "schema": 1,
            "exceptions": [
                {
                    "protocol": "AP-CI-001",
                    "scope": "alpha",
                    "reason": "Changed after rendering.",
                    "approved_by": "user",
                    "approved_at": "2026-08-04",
                }
            ],
        }

        with self.assertRaisesRegex(protocols.ProtocolError, "authoring file drifted"):
            protocols.check_import(
                ROOT, manifest, "Local truth.\n", agents, COMMIT, changed
            )

    def test_exception_cannot_target_protocol_outside_profile(self) -> None:
        exceptions = {
            "schema": 1,
            "exceptions": [
                {
                    "protocol": "AP-TERMINAL-001",
                    "scope": "all",
                    "reason": "Invalid test case.",
                    "approved_by": "user",
                    "approved_at": "2026-08-04",
                }
            ],
        }

        with self.assertRaisesRegex(protocols.ProtocolError, "outside the profile"):
            protocols.assemble_import(
                ROOT,
                ["greenfield"],
                "Local truth.\n",
                exceptions,
                protocols.SOURCE_REPOSITORY,
                COMMIT,
            )

    def test_profiles_compose_in_order_without_duplicate_protocols(self) -> None:
        manifest, agents = protocols.assemble_import(
            ROOT,
            ["greenfield", "terminal-engine"],
            "Local truth.\n",
            None,
            protocols.SOURCE_REPOSITORY,
            COMMIT,
        )

        ids = [item["id"] for item in manifest["protocols"]]
        self.assertEqual(len(ids), len(set(ids)))
        self.assertEqual(["greenfield", "terminal-engine"], manifest["profiles"])
        self.assertIn("- Profiles: `greenfield`, `terminal-engine`", agents)
        self.assertIn("AP-TERMINAL-001", ids)

    @staticmethod
    def copy_catalog(destination: Path) -> Path:
        shutil.copytree(ROOT / "protocols", destination / "protocols")
        shutil.copytree(ROOT / "profiles", destination / "profiles")
        return destination


if __name__ == "__main__":
    unittest.main()
