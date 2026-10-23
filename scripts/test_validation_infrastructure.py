"""Regression checks for validation infrastructure, without release operations."""

import subprocess
import shutil
import sys
import tempfile
import unittest
from pathlib import Path
from unittest.mock import patch

import reproducible_artifacts as artifacts
import check_ci_prerequisites as prerequisites

STRUCTURE_SCRIPT = Path(__file__).with_name("validate_structure.py").resolve()
ACTIONLINT = shutil.which("actionlint")
WORKFLOW_FIXTURE = """name: condition regression
on: workflow_dispatch
permissions:
  contents: read
jobs:
  validation:
    if: __CONDITION__
    runs-on: ubuntu-latest
    steps:
      - run: echo validation
"""


@unittest.skipUnless(ACTIONLINT, "actionlint is installed by the Actions Scripts job")
class WorkflowConditionTests(unittest.TestCase):
    def test_folded_expression_with_trailing_newline_is_rejected(self):
        result = lint_workflow(">\n      ${{ always() }}")
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("if-cond", result.stdout + result.stderr)

    def test_stripped_folded_expression_is_accepted(self):
        result = lint_workflow(">-\n      ${{ always() }}")
        self.assertEqual(result.returncode, 0, result.stdout + result.stderr)


class StructureValidationTests(unittest.TestCase):
    def test_wiki_orientation_is_allowed(self):
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            seed_repository(root)
            (root / "docs/wiki").mkdir()
            (root / "docs/wiki/README.md").write_text(frontmatter("orientation"), encoding="utf-8")
            result = validate_structure(root)
            self.assertEqual(result.returncode, 0, result.stdout + result.stderr)

    def test_wiki_cannot_become_a_governance_surface(self):
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            seed_repository(root)
            (root / "docs/wiki").mkdir()
            (root / "docs/wiki/README.md").write_text(frontmatter("normative"), encoding="utf-8")
            result = validate_structure(root)
            self.assertNotEqual(result.returncode, 0)
            self.assertIn("is not allowed in this location", result.stdout)

    def test_os_metadata_does_not_block_structure_validation(self):
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            seed_repository(root)
            (root / ".DS_Store").write_bytes(b"finder metadata")
            (root / "docs/.DS_Store").write_bytes(b"finder metadata")
            (root / "docs/architecture/.DS_Store").write_bytes(b"finder metadata")
            result = validate_structure(root)
            self.assertEqual(result.returncode, 0, result.stdout + result.stderr)

    def test_unexpected_repository_files_still_fail(self):
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            seed_repository(root)
            (root / "unexpected.txt").write_text("not OS metadata", encoding="utf-8")
            result = validate_structure(root)
            self.assertNotEqual(result.returncode, 0)
            self.assertIn("unexpected.txt", result.stdout)


class ArtifactProcessTests(unittest.TestCase):
    def test_build_environment_bounds_dependency_fetches(self):
        env = artifacts.deterministic_environment(Path("/tmp/fixture"), "1")
        self.assertEqual(env["npm_config_fetch_timeout"], "15000")
        self.assertEqual(env["npm_config_fetch_retries"], "1")
        self.assertEqual(env["npm_config_audit"], "false")

    def test_build_timeout_fails_with_actionable_diagnostic(self):
        timeout = subprocess.TimeoutExpired(["npm", "ci"], artifacts.NPM_INSTALL_TIMEOUT_SECONDS)
        with patch.object(artifacts.subprocess, "run", side_effect=timeout):
            with self.assertRaisesRegex(SystemExit, "command timed out"):
                artifacts.run(["npm", "ci"], Path("/tmp"), {})

    def test_dependency_install_has_a_shorter_timeout_than_compilation(self):
        self.assertLess(artifacts.command_timeout(["npm", "ci"]), artifacts.command_timeout(["cargo", "build"]))

    def test_failed_commands_propagate_failure(self):
        failure = subprocess.CalledProcessError(1, ["npm", "ci"])
        with patch.object(artifacts.subprocess, "run", side_effect=failure):
            with self.assertRaises(subprocess.CalledProcessError):
                artifacts.run(["npm", "ci"], Path("/tmp"), {})


class CiPrerequisiteTests(unittest.TestCase):
    def test_explicit_success_is_required_for_every_dependency(self):
        self.assertEqual(prerequisites.failed_prerequisites({"node24": {"result": "success"}}), [])

    def test_failed_skipped_cancelled_and_missing_results_block_the_gate(self):
        for result in ["failure", "skipped", "cancelled", "unknown", None]:
            with self.subTest(result=result):
                self.assertEqual(prerequisites.failed_prerequisites({"node24": {"result": result}}), ["node24"])

    def test_empty_or_malformed_dependency_evidence_fails_closed(self):
        for results in [{}, [], None]:
            with self.subTest(results=results), self.assertRaises(ValueError):
                prerequisites.failed_prerequisites(results)


def seed_repository(root):
    files = {
        "README.md": "orientation", "AGENTS.md": "navigation", "CHANGELOG.md": "historical",
        "docs/governance/GOVERNANCE.md": "normative",
        "docs/architecture/ARCHITECTURE.md": "structural",
    }
    for filename, dimension in files.items():
        target = root / filename
        target.parent.mkdir(parents=True, exist_ok=True)
        target.write_text(frontmatter(dimension), encoding="utf-8")


def frontmatter(dimension):
    return f"---\ntruth_dimension: {dimension}\nauthority_level: non_authoritative\nmutation_path: test\n---\n"


def validate_structure(root):
    return subprocess.run([sys.executable, str(STRUCTURE_SCRIPT)], cwd=root, capture_output=True, text=True, timeout=15)


def lint_workflow(condition):
    with tempfile.TemporaryDirectory() as directory:
        workflow = Path(directory) / "fixture.yml"
        workflow.write_text(WORKFLOW_FIXTURE.replace("__CONDITION__", condition), encoding="utf-8")
        return subprocess.run(
            [ACTIONLINT, "-shellcheck=", "-pyflakes=", str(workflow)],
            capture_output=True, text=True, timeout=15,
        )


if __name__ == "__main__":
    unittest.main()
