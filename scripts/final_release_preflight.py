#!/usr/bin/env python3
"""Validate local final-release boundaries.

This script performs non-mutating checks. It does not create tags, push refs,
create GitHub Releases, upload assets, publish packages, deploy, or contact
remote services unless --check-remote is selected.
"""

from __future__ import annotations

import argparse
import json
import re
import subprocess
import sys
import tomllib
import urllib.error
import urllib.request
from pathlib import Path


FINAL_TAG = "v1.0.0"
SOURCE_RC_TAG = "v1.0.0-rc.1"
FINAL_VERSION = "1.0.0"
FINAL_WORKFLOW = Path(".github/workflows/final-release.yml")
ATTESTATION_JOB = "attest_final_assets"
PUBLICATION_JOB = "publish_final"


def main() -> int:
    args = parse_args()
    repo_root = Path(args.repo).resolve()

    if args.check_local:
        return check_local(repo_root, args.final_tag, args.source_rc_tag, args.expected_commit)

    if args.check_remote:
        return check_remote(args.repo_name, args.final_tag, args.source_rc_tag)

    return 0


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(
        description="Validate final v1 release preflight boundaries."
    )
    parser.add_argument("--repo", default=".", help="Repository root.")
    parser.add_argument(
        "--repo-name",
        default="irgordon/ajentic",
        help="GitHub repository name for --check-remote.",
    )
    parser.add_argument("--final-tag", default=FINAL_TAG, help="Final release tag.")
    parser.add_argument("--source-rc-tag", default=SOURCE_RC_TAG, help="Source RC tag.")
    parser.add_argument(
        "--expected-commit",
        help="Optional commit SHA that must match the current HEAD.",
    )
    parser.add_argument(
        "--check-local",
        action="store_true",
        help="Run local non-mutating Phase 194 preflight checks.",
    )
    parser.add_argument(
        "--check-remote",
        action="store_true",
        help="Check remote final tag absence and source RC presence.",
    )
    return parser.parse_args()


def check_local(
    repo_root: Path, final_tag: str, source_rc_tag: str, expected_commit: str | None
) -> int:
    failures: list[str] = []
    check_tags(final_tag, source_rc_tag, failures)
    check_expected_commit(repo_root, expected_commit, failures)
    check_package_versions(repo_root, failures)
    check_release_notes(repo_root, final_tag, source_rc_tag, failures)
    check_final_workflow(repo_root, failures)

    if failures:
        for failure in failures:
            print(f"::error::{failure}")
        return 1

    print("Final release local preflight passed.")
    return 0


def check_tags(final_tag: str, source_rc_tag: str, failures: list[str]) -> None:
    if final_tag != FINAL_TAG:
        failures.append("final_tag must be v1.0.0")
    if source_rc_tag != SOURCE_RC_TAG:
        failures.append("source_rc_tag must be v1.0.0-rc.1")
    if re.search(r"rc|prerelease|draft|unstable", final_tag, flags=re.IGNORECASE):
        failures.append("final_tag must not contain prerelease markers")


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
    for name, version in package_versions(repo_root).items():
        if version == FINAL_VERSION:
            failures.append(
                f"{name} package metadata is {FINAL_VERSION}; Phase 194 keeps package versions separately governed"
            )


def check_release_notes(
    repo_root: Path, final_tag: str, source_rc_tag: str, failures: list[str]
) -> None:
    path = repo_root / "docs" / "releases" / f"{final_tag}.md"
    if not path.is_file():
        failures.append(f"release notes missing: {path.relative_to(repo_root)}")
        return

    text = path.read_text(encoding="utf-8")
    required = [
        "AJENTIC v1.0.0",
        "Status: final v1.0.0 GitHub Release",
        f"Source RC: {source_rc_tag}",
        f"Final tag: {final_tag}",
        "No npm publication",
        "No Cargo publication",
        "No installer",
        "No update channel",
        "No deployment",
        "No OS signing",
        "No notarization",
    ]
    for item in required:
        if item not in text:
            failures.append(f"release notes missing required text: {item}")


def check_final_workflow(repo_root: Path, failures: list[str]) -> None:
    workflow_path = repo_root / FINAL_WORKFLOW
    if not workflow_path.is_file():
        failures.append(f"workflow missing: {FINAL_WORKFLOW}")
        return

    text = workflow_path.read_text(encoding="utf-8")
    check_workflow_dispatch_only(text, failures)
    check_prohibited_permissions(text, failures)
    check_scoped_permissions(text, failures)
    check_required_inputs(text, failures)
    check_prohibited_commands(text, failures)


def check_workflow_dispatch_only(text: str, failures: list[str]) -> None:
    if "workflow_dispatch:" not in text:
        failures.append("final-release workflow must use workflow_dispatch")

    for trigger in ["\n  push:", "\n  pull_request:", "\n  schedule:", "\n  tags:"]:
        if trigger in text:
            failures.append(f"final-release workflow has prohibited trigger {trigger.strip()}")


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
            "contents: write must appear only in the publish_final job; "
            f"found {contents_blocks}"
        )

    for permission in ["id-token: write", "attestations: write"]:
        jobs = jobs_containing(text, permission)
        if jobs != [ATTESTATION_JOB]:
            failures.append(
                f"{permission} must appear only in the attest_final_assets job; found {jobs}"
            )


def check_required_inputs(text: str, failures: list[str]) -> None:
    for required in [
        "final_tag:",
        "expected_commit:",
        "source_rc_tag:",
        "confirm_final_release:",
        "confirm_no_package_publication:",
        "confirm_no_installer_update_deploy:",
        "attest_final_assets:",
        "FINAL_V1_RELEASE",
        "NO_PACKAGE_PUBLICATION",
        "NO_INSTALLER_UPDATE_DEPLOY",
    ]:
        if required not in text:
            failures.append(f"final-release workflow missing required input or confirmation: {required}")


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
        "git push --force",
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
            failures.append(f"prohibited command found in final-release workflow: {command}")


def check_remote(repo_name: str, final_tag: str, source_rc_tag: str) -> int:
    failures: list[str] = []
    if api_status(repo_name, f"git/ref/tags/{final_tag}") != 404:
        failures.append(f"remote final tag already exists: {final_tag}")
    if api_status(repo_name, f"releases/tags/{final_tag}") != 404:
        failures.append(f"remote final release already exists: {final_tag}")
    if api_status(repo_name, f"git/ref/tags/{source_rc_tag}") != 200:
        failures.append(f"source RC tag missing: {source_rc_tag}")
    if api_status(repo_name, f"releases/tags/{source_rc_tag}") != 200:
        failures.append(f"source RC release missing: {source_rc_tag}")

    if failures:
        for failure in failures:
            print(f"::error::{failure}")
        return 1

    print("Final release remote preflight passed.")
    return 0


def api_status(repo_name: str, path: str) -> int:
    request = urllib.request.Request(
        f"https://api.github.com/repos/{repo_name}/{path}",
        headers={
            "Accept": "application/vnd.github+json",
            "User-Agent": "ajentic-phase-194-final-preflight",
            "X-GitHub-Api-Version": "2022-11-28",
        },
    )
    try:
        with urllib.request.urlopen(request, timeout=30) as response:
            return response.status
    except urllib.error.HTTPError as error:
        return error.code


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
