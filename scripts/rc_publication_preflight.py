#!/usr/bin/env python3
"""Validate local release-candidate publication boundaries.

This script performs local checks only. It does not create tags, push refs,
create GitHub Releases, upload assets, publish packages, deploy, or contact
remote services.
"""

from __future__ import annotations

import argparse
import json
import re
import subprocess
import sys
import tomllib
from pathlib import Path


RC_TAG_PATTERN = re.compile(r"^v1\.0\.0-rc\.[1-9][0-9]*$")
FINAL_VERSION = "1.0.0"
RC_WORKFLOW = Path(".github/workflows/rc-publication.yml")
ATTESTATION_JOB = "attest_rc_assets"
PUBLICATION_JOB = "publish_rc"


def main() -> int:
    args = parse_args()
    repo_root = Path(args.repo).resolve()

    if args.check_local:
        return check_local(repo_root, args.rc_tag, args.expected_commit)

    return 0


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(
        description="Validate local RC publication boundaries."
    )
    parser.add_argument("--repo", default=".", help="Repository root.")
    parser.add_argument("--rc-tag", default="v1.0.0-rc.1", help="RC tag.")
    parser.add_argument(
        "--expected-commit",
        help="Optional commit SHA that must match the current HEAD.",
    )
    parser.add_argument(
        "--check-local",
        action="store_true",
        help="Run local non-mutating Phase 193 preflight checks.",
    )
    return parser.parse_args()


def check_local(repo_root: Path, rc_tag: str, expected_commit: str | None) -> int:
    failures: list[str] = []
    check_rc_tag(rc_tag, failures)
    check_expected_commit(repo_root, expected_commit, failures)
    check_package_versions(repo_root, failures)
    check_release_notes(repo_root, rc_tag, failures)
    check_rc_workflow(repo_root, failures)

    if failures:
        for failure in failures:
            print(f"::error::{failure}")
        return 1

    print("RC publication local preflight passed.")
    return 0


def check_rc_tag(rc_tag: str, failures: list[str]) -> None:
    if rc_tag == "v1.0.0":
        failures.append("rc_tag must not be final v1.0.0")
    if not RC_TAG_PATTERN.fullmatch(rc_tag):
        failures.append("rc_tag must match v1.0.0-rc.<number>")


def check_expected_commit(
    repo_root: Path, expected_commit: str | None, failures: list[str]
) -> None:
    if not expected_commit:
        return
    current = git_output(repo_root, ["rev-parse", "HEAD"])
    if current != expected_commit:
        failures.append(
            f"expected_commit {expected_commit} does not match HEAD {current}"
        )


def check_package_versions(repo_root: Path, failures: list[str]) -> None:
    versions = package_versions(repo_root)
    for name, version in versions.items():
        if version == FINAL_VERSION:
            failures.append(f"{name} package version must not be final {FINAL_VERSION}")


def check_release_notes(repo_root: Path, rc_tag: str, failures: list[str]) -> None:
    path = repo_root / "docs" / "releases" / f"{rc_tag}.md"
    if not path.is_file():
        failures.append(f"release notes missing: {path.relative_to(repo_root)}")
        return

    text = path.read_text(encoding="utf-8")
    required = [
        "AJENTIC v1.0.0-rc.1 Release Candidate",
        "Status: public release candidate",
        "Not final v1.0.0",
        "Not production-approved",
        "No update channel",
        "No deployment",
        "No package registry publication",
    ]
    for item in required:
        if item not in text:
            failures.append(f"release notes missing required text: {item}")


def check_rc_workflow(repo_root: Path, failures: list[str]) -> None:
    workflow_path = repo_root / RC_WORKFLOW
    if not workflow_path.is_file():
        failures.append(f"workflow missing: {RC_WORKFLOW}")
        return

    text = workflow_path.read_text(encoding="utf-8")
    check_workflow_dispatch_only(text, failures)
    check_prohibited_permissions(text, failures)
    check_scoped_permissions(text, failures)
    check_prohibited_commands(text, failures)


def check_workflow_dispatch_only(text: str, failures: list[str]) -> None:
    if "workflow_dispatch:" not in text:
        failures.append("rc-publication workflow must use workflow_dispatch")

    prohibited_triggers = [
        "\n  push:",
        "\n  pull_request:",
        "\n  schedule:",
        "\n  tags:",
    ]
    for trigger in prohibited_triggers:
        if trigger in text:
            failures.append(f"rc-publication workflow has prohibited trigger {trigger.strip()}")


def check_prohibited_permissions(text: str, failures: list[str]) -> None:
    for permission in [
        "packages: write",
        "deployments: write",
        "actions: write",
        "artifact-metadata: write",
    ]:
        if permission in text:
            failures.append(f"prohibited workflow permission present: {permission}")


def check_scoped_permissions(text: str, failures: list[str]) -> None:
    contents_blocks = jobs_containing(text, "contents: write")
    if contents_blocks != [PUBLICATION_JOB]:
        failures.append(
            "contents: write must appear only in the publish_rc job; "
            f"found {contents_blocks}"
        )

    for permission in ["id-token: write", "attestations: write"]:
        jobs = jobs_containing(text, permission)
        if jobs != [ATTESTATION_JOB]:
            failures.append(
                f"{permission} must appear only in the attest_rc_assets job; found {jobs}"
            )


def jobs_containing(text: str, needle: str) -> list[str]:
    jobs: list[str] = []
    current_job: str | None = None

    for line in text.splitlines():
        job_match = re.match(r"^  ([A-Za-z0-9_-]+):\s*$", line)
        if job_match:
            current_job = job_match.group(1)
        if needle in line and current_job and current_job not in jobs:
            jobs.append(current_job)

    return jobs


def check_prohibited_commands(text: str, failures: list[str]) -> None:
    prohibited = [
        "git push --tags",
        "npm publish",
        "cargo publish",
        "docker push",
        "kubectl",
        "aws deploy",
        "gh release create",
        "gh release upload",
    ]
    for command in prohibited:
        if command in text:
            failures.append(f"prohibited command found in rc-publication workflow: {command}")


def package_versions(repo_root: Path) -> dict[str, str]:
    ui = read_json(repo_root / "ui" / "package.json")
    core = read_toml(repo_root / "core" / "Cargo.toml")["package"]
    return {
        "core": str(core["version"]),
        "ui": str(ui["version"]),
    }


def read_json(path: Path) -> dict[str, object]:
    with path.open("r", encoding="utf-8") as handle:
        return json.load(handle)


def read_toml(path: Path) -> dict[str, object]:
    with path.open("rb") as handle:
        return tomllib.load(handle)


def git_output(repo_root: Path, command: list[str]) -> str:
    result = subprocess.run(
        ["git", *command],
        cwd=repo_root,
        check=True,
        text=True,
        capture_output=True,
    )
    return result.stdout.strip()


if __name__ == "__main__":
    sys.exit(main())
