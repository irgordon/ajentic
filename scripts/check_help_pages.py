#!/usr/bin/env python3
"""Deterministic validation for local operator help pages."""

from __future__ import annotations

from pathlib import Path
import re
import sys

ROOT = Path(__file__).resolve().parents[1]
HELP_DIR = ROOT / "ui" / "help"
INDEX = HELP_DIR / "index.html"

REQUIRED_PAGES = [
    "index.html",
    "getting-started.html",
    "local-workflow.html",
    "provider-setup.html",
    "validation-review-candidates.html",
    "trial-package-evidence.html",
    "restore-verification.html",
    "errors-stop-conditions.html",
    "glossary.html",
    "safety-boundaries.html",
]

REQUIRED_PHRASES = {
    "index.html": [
        "AJENTIC helps",
        "local-only",
        "non-production",
        "explanatory only",
    ],
    "getting-started.html": ["npm run dev", "local UI", "workflow", "local-only"],
    "local-workflow.html": [
        "provider setup",
        "constrained invocation",
        "validation",
        "candidate review",
        "local candidate materialization",
        "evidence export",
        "restore/history",
    ],
    "provider-setup.html": [
        "Adapter contract",
        "dry run",
        "constrained invocation",
        "AJENTIC does not run arbitrary commands",
        "Provider output remains untrusted",
    ],
    "validation-review-candidates.html": [
        "staged proposal",
        "operator decision",
        "Local candidate output is not production approval",
        "Provider output remains untrusted",
    ],
    "trial-package-evidence.html": [
        "trial package",
        "runbook",
        "evidence capture",
        "trial execution harness",
        "local-only and non-public",
        "evidence is not authority",
    ],
    "restore-verification.html": [
        "Read-back validation",
        "Restore preview",
        "Replay/status",
        "verification does not repair replay",
        "does not promote recovery",
    ],
    "errors-stop-conditions.html": [
        "Blockers",
        "mismatch drilldowns",
        "Stop conditions are observed",
        "not automatically enforced",
        "Escalation guidance",
    ],
    "glossary.html": [
        "Adapter",
        "Provider",
        "Invocation",
        "Validation",
        "Staged proposal",
        "Operator decision",
        "Evidence",
        "Stop condition",
        "Escalation guidance",
        "Boundary",
    ],
    "safety-boundaries.html": [
        "No production approval",
        "No release approval",
        "No deployment approval",
        "No public/general-use approval",
        "Provider output remains untrusted",
        "No arbitrary command execution",
        "No automated stop-condition enforcement",
        "No action authorization",
        "No replay repair",
        "No recovery promotion",
    ],
}

FORBIDDEN_LABELS = [
    "production_ready",
    "release_ready",
    "deployment_ready",
    "public_use_ready",
    "controlled_human_use_approved",
    "production_use_approved",
    "trial_approved",
    "workflow_approved",
    "trusted_provider_output",
    "approved_candidate",
    "action_authorized",
    "replay_repaired",
    "recovery_promoted",
    "publish_ready",
    "deploy_ready",
    "signed_release",
    "public_release",
]

APPROVAL_CLAIM_PATTERNS = [
    r"\bproduction\s+ready\b",
    r"\brelease\s+ready\b",
    r"\bdeployment\s+ready\b",
    r"\bpublic\s+use\s+ready\b",
    r"\bapproved\s+for\s+production\b",
    r"\bapproved\s+for\s+release\b",
    r"\bapproved\s+for\s+deployment\b",
    r"\bapproved\s+for\s+public\s+use\b",
]


def fail(message: str) -> None:
    print(f"help validation failed: {message}", file=sys.stderr)
    raise SystemExit(1)


def read_page(name: str) -> str:
    path = HELP_DIR / name
    if not path.is_file():
        fail(f"missing {path.relative_to(ROOT)}")
    return path.read_text(encoding="utf-8")


def main() -> None:
    pages = {name: read_page(name) for name in REQUIRED_PAGES}
    index = pages["index.html"]

    for name in REQUIRED_PAGES[1:]:
        if f'href="{name}"' not in index:
            fail(f"index.html does not link to {name}")

    for name, phrases in REQUIRED_PHRASES.items():
        page = pages[name]
        for phrase in phrases:
            if phrase not in page:
                fail(f"{name} missing required phrase: {phrase}")

    for name, page in pages.items():
        for forbidden in FORBIDDEN_LABELS:
            if forbidden in page:
                fail(f"{name} contains forbidden label: {forbidden}")
        for pattern in APPROVAL_CLAIM_PATTERNS:
            if re.search(pattern, page, re.IGNORECASE):
                fail(f"{name} contains approval/readiness claim pattern: {pattern}")

    print(f"help pages OK ({len(REQUIRED_PAGES)} pages)")


if __name__ == "__main__":
    main()
