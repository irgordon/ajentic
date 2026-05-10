#!/usr/bin/env python3
"""
bootstrap_repo.py - Idempotent Phase 0 repository skeleton bootstrap.

Creates missing files and directories only. Does not overwrite existing files.
Writes UTF-8 files with LF newlines.

Usage:
    python3 scripts/bootstrap_repo.py
"""

from __future__ import annotations

from datetime import date
from pathlib import Path
import stat


ROOT = Path(__file__).resolve().parent.parent
TODAY = date.today().isoformat()


DIRS = [
    "core/src/state",
    "core/src/context",
    "core/src/memory",
    "core/src/policy",
    "core/src/validation",
    "core/src/execution",
    "core/src/ledger",
    "core/src/replay",
    "core/src/audit",
    "core/src/api",
    "core/src/errors",
    "ui/public",
    "ui/src/app",
    "ui/src/screens",
    "ui/src/components",
    "ui/src/api",
    "ui/src/types/generated",
    "ui/src/validation/generated",
    "ui/src/validation/adapters",
    "ui/src/hooks",
    "ui/src/styles",
    "scripts",
    "tests/context",
    "tests/memory",
    "tests/policy",
    "tests/validation",
    "tests/state-machine",
    "tests/ledger",
    "tests/replay",
    "tests/api",
    "tests/ui-contracts",
    "memory/persistent",
    "memory/ephemeral",
    "checklists",
    "docs/governance",
    "docs/architecture",
    "docs/roadmap",
    "docs/operations",
    "schemas/docs",
    "schemas/context",
    "schemas/memory",
    "schemas/events",
    "schemas/intents",
    "schemas/traces",
]


def governed_md(
    truth_dimension: str,
    authority_level: str,
    mutation_path: str,
    title: str,
    body: str,
) -> str:
    return f"""---
truth_dimension: {truth_dimension}
authority_level: {authority_level}
mutation_path: {mutation_path}
---

# {title}

{body}
"""


def schema_placeholder() -> str:
    return """{
  "$schema": "https://json-schema.org/draft/2020-12/schema",
  "type": "object"
}
"""


FILES = {
    "AGENTS.md": governed_md(
        "navigation",
        "non_authoritative",
        "agents_update",
        "AGENTS.md",
        """This file is a short navigation contract for agents and tools working in this repository.

It is not the system of record.

It does not define governance rules, architecture authority, roadmap commitments, examples, or implementation details.

## Authoritative sources

| Source | Purpose |
| --- | --- |
| `docs/governance/GOVERNANCE.md` | Normative rules, authority model, prohibited patterns, and invariants. |
| `docs/architecture/ARCHITECTURE.md` | Structural description of the system, component responsibilities, and data flow. |
| `docs/roadmap/phase-map.md` | Planned phase sequence and phase scope. |
| `checklists/current-phase.md` | Active execution checklist for the current phase. |
| `CHANGELOG.md` | Completed accepted work. |
| `schemas/` | Shared data contracts through JSON Schema. |

## Quick navigation

- Rules and invariants: `docs/governance/GOVERNANCE.md`, `docs/governance/`
- System structure: `docs/architecture/ARCHITECTURE.md`, `docs/architecture/`
- Roadmap and phases: `docs/roadmap/phase-map.md`, `docs/roadmap/`
- Active phase tasks: `checklists/current-phase.md`
- Data contracts: `schemas/`
- Runtime source: `core/`
- Browser UI source: `ui/`
- Operator scripts: `scripts/`

## Constraint reminder

Rust owns runtime authority.

TypeScript is a review and operator-intent surface only.

Bash scripts are operator wrappers only.

Model output is untrusted until validated through Rust-owned paths.

This file must remain short, stable, and non-authoritative.
""",
    ),

    "README.md": governed_md(
        "orientation",
        "non_authoritative",
        "readme_update",
        "AJENTIC",
        """AJENTIC is a deterministic control layer for model-driven work.

It is designed for teams that want the speed of local or cloud large language models without surrendering review, validation, replay, or operator control.

The harness treats model output as candidate material. It assembles bounded context, validates outputs, records accepted events, supports replay, and presents the run through a human-readable browser UI.

This README is orientation only. It is not an authority source.
""",
    ),

    "CHANGELOG.md": f"""---
truth_dimension: historical
authority_level: authoritative
mutation_path: changelog_entry
---

# CHANGELOG.md

## v0.0.0 - {TODAY}

**Status:** Phase 0 - Initial Repo Setup

### Added

- Created the initial repository skeleton for AJENTIC.
- Added top-level navigation, orientation, and historical anchors.
- Added canonical directories for Rust core, browser UI, scripts, tests, memory, checklists, docs, and schemas.
- Added minimal Rust core compile skeleton under `core/`.
- Added minimal TypeScript browser UI placeholder structure under `ui/`.
- Added initial governed documentation directories under `docs/`.
- Added placeholder JSON Schema files under `schemas/`.

### Notes

- This is the initial repository build.
- No runtime harness behavior is implemented in this version.
- Placeholder files establish structure only and must not be treated as completed runtime capability.
""",

    "core/Cargo.toml": """\
[package]
name = "ajentic-core"
version = "0.1.0"
edition = "2021"

[dependencies]
""",

    "core/src/lib.rs": """\
pub mod api;
pub mod audit;
pub mod context;
pub mod errors;
pub mod execution;
pub mod ledger;
pub mod memory;
pub mod policy;
pub mod replay;
pub mod state;
pub mod validation;
""",

    "core/src/main.rs": """\
fn main() {
    println!("ajentic core");
}
""",

    "ui/package.json": """\
{
  "name": "ajentic-ui",
  "private": true,
  "version": "0.1.0",
  "type": "module",
  "scripts": {
    "lint": "echo \\"ui lint placeholder\\"",
    "typecheck": "tsc --noEmit",
    "build": "echo \\"ui build placeholder\\""
  },
  "devDependencies": {
    "typescript": "^5.0.0"
  }
}
""",

    "ui/tsconfig.json": """\
{
  "compilerOptions": {
    "target": "ES2020",
    "module": "ESNext",
    "moduleResolution": "bundler",
    "strict": true,
    "noEmit": true,
    "jsx": "preserve",
    "esModuleInterop": true,
    "skipLibCheck": true,
    "outDir": "dist",
    "rootDir": "src"
  },
  "include": ["src"]
}
""",

    "ui/src/app/AppShell.tsx": """\
// Placeholder. No UI behavior implemented.
// AppShell is the root shell component for the browser UI.
// It will display harness state projections and operator intent controls.
export {};
""",

    "ui/src/app/routes.tsx": """\
// Placeholder. No routing behavior implemented.
// This file will define the browser UI route tree.
export {};
""",

    "ui/src/app/navigation.ts": """\
// Placeholder. No navigation behavior implemented.
// This file will define the navigation structure for the browser UI.
export {};
""",

    "ui/src/styles/tokens.css": """\
/* Placeholder. No design tokens implemented. */
""",

    "ui/src/styles/layout.css": """\
/* Placeholder. No layout implemented. */
""",

    "memory/persistent/project-facts.json": "{}\n",
    "memory/persistent/conventions.json": "{}\n",

    "memory/ephemeral/.gitignore": """\
*
!.gitignore
""",

    "checklists/current-phase.md": governed_md(
        "procedural",
        "authoritative",
        "checklist_revision",
        "Current phase checklist",
        """This checklist is the active working surface for the current phase.

It does not define governance.

It does not define architecture.

It does not record completed history.

## Phase

Phase 0 - Initial Repo Setup

## Goal

Bring the blank repository to the Phase 0 skeleton baseline.

## Tasks

- [ ] Create required directories.
- [ ] Create required placeholder files.
- [ ] Validate bootstrap idempotence.
- [ ] Validate Rust compile skeleton.
- [ ] Validate UI placeholder typecheck when dependencies are available.
- [ ] Validate JSON placeholder schemas.

## Deferred items

| Item | Reason | Target phase |
| --- | --- | --- |

## Validation log

| Check | Result | Notes |
| --- | --- | --- |
""",
    ),

    "checklists/release.md": governed_md(
        "procedural",
        "authoritative",
        "checklist_revision",
        "Release checklist",
        "Placeholder. No release procedure implemented.\n",
    ),

    "checklists/migration.md": governed_md(
        "procedural",
        "authoritative",
        "checklist_revision",
        "Migration checklist",
        "Placeholder. No migration procedure implemented.\n",
    ),

    "checklists/audit.md": governed_md(
        "procedural",
        "authoritative",
        "checklist_revision",
        "Audit checklist",
        "Placeholder. No audit procedure implemented.\n",
    ),

    "docs/governance/invariants.md": governed_md(
        "normative",
        "authoritative",
        "governance_pr",
        "Governance invariants",
        "Subordinate to `docs/governance/GOVERNANCE.md`.\n",
    ),

    "docs/governance/artifact-placement.md": governed_md(
        "normative",
        "authoritative",
        "governance_pr",
        "Artifact placement rules",
        "Subordinate to `docs/governance/GOVERNANCE.md`.\n",
    ),

    "docs/governance/mutation-paths.md": governed_md(
        "normative",
        "authoritative",
        "governance_pr",
        "Mutation paths",
        "Subordinate to `docs/governance/GOVERNANCE.md`.\n",
    ),

    "docs/governance/negative-patterns.md": governed_md(
        "normative",
        "authoritative",
        "governance_pr",
        "Negative patterns",
        "Subordinate to `docs/governance/GOVERNANCE.md`.\n",
    ),

    "docs/governance/non-goals.md": governed_md(
        "normative",
        "authoritative",
        "governance_pr",
        "Non-goals",
        "Subordinate to `docs/governance/GOVERNANCE.md`.\n",
    ),

    "docs/architecture/system-architecture.md": governed_md(
        "structural",
        "authoritative",
        "architecture_pr",
        "System architecture",
        "Subordinate to `docs/architecture/ARCHITECTURE.md`.\n",
    ),

    "docs/architecture/browser-ui.md": governed_md(
        "structural",
        "authoritative",
        "architecture_pr",
        "Browser UI architecture",
        "Subordinate to `docs/architecture/ARCHITECTURE.md`.\n",
    ),

    "docs/architecture/state-machine.md": governed_md(
        "structural",
        "authoritative",
        "architecture_pr",
        "State machine architecture",
        "Subordinate to `docs/architecture/ARCHITECTURE.md`.\n",
    ),

    "docs/architecture/context-pipeline.md": governed_md(
        "structural",
        "authoritative",
        "architecture_pr",
        "Context pipeline architecture",
        "Subordinate to `docs/architecture/ARCHITECTURE.md`.\n",
    ),

    "docs/architecture/memory-model.md": governed_md(
        "structural",
        "authoritative",
        "architecture_pr",
        "Memory model architecture",
        "Subordinate to `docs/architecture/ARCHITECTURE.md`.\n",
    ),

    "docs/architecture/policy-validation-boundary.md": governed_md(
        "structural",
        "authoritative",
        "architecture_pr",
        "Policy validation boundary",
        "Subordinate to `docs/architecture/ARCHITECTURE.md`.\n",
    ),

    "docs/architecture/ledger-replay-model.md": governed_md(
        "structural",
        "authoritative",
        "architecture_pr",
        "Ledger replay model",
        "Subordinate to `docs/architecture/ARCHITECTURE.md`.\n",
    ),

    "docs/roadmap/phases.md": governed_md(
        "planned",
        "advisory",
        "roadmap_update",
        "Roadmap phases",
        "Planned truth. Does not record completed work.\n",
    ),

    "docs/roadmap/sequencing.md": governed_md(
        "planned",
        "advisory",
        "roadmap_update",
        "Roadmap sequencing",
        "Planned truth. Does not record completed work.\n",
    ),

    "schemas/docs/artifact-frontmatter.schema.json": schema_placeholder(),
    "schemas/context/context-packet.schema.json": schema_placeholder(),
    "schemas/context/context-slice.schema.json": schema_placeholder(),
    "schemas/context/context-budget.schema.json": schema_placeholder(),
    "schemas/memory/memory-entry.schema.json": schema_placeholder(),
    "schemas/memory/memory-snapshot.schema.json": schema_placeholder(),
    "schemas/memory/memory-provenance.schema.json": schema_placeholder(),
    "schemas/events/ledger-event.schema.json": schema_placeholder(),
    "schemas/events/run-event.schema.json": schema_placeholder(),
    "schemas/events/audit-event.schema.json": schema_placeholder(),
    "schemas/intents/operator-intent.schema.json": schema_placeholder(),
    "schemas/intents/approve-intent.schema.json": schema_placeholder(),
    "schemas/intents/reject-intent.schema.json": schema_placeholder(),
    "schemas/intents/retry-intent.schema.json": schema_placeholder(),
    "schemas/intents/memory-intent.schema.json": schema_placeholder(),
    "schemas/traces/run-trace.schema.json": schema_placeholder(),
    "schemas/traces/replay-report.schema.json": schema_placeholder(),
    "schemas/traces/validation-report.schema.json": schema_placeholder(),
}


EXECUTABLE_FILES = {
    "scripts/check.sh": """\
#!/usr/bin/env bash
set -euo pipefail

cargo check --manifest-path core/Cargo.toml
""",
}


def write_file(path: Path, content: str) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(content, encoding="utf-8", newline="\n")
    print(f"  created: {path.relative_to(ROOT)}")


def write_executable_file(path: Path, content: str) -> None:
    write_file(path, content)
    mode = path.stat().st_mode
    path.chmod(mode | stat.S_IXUSR | stat.S_IXGRP | stat.S_IXOTH)


def ensure_dir(path: Path) -> None:
    path.mkdir(parents=True, exist_ok=True)


def main() -> None:
    print("bootstrap_repo.py: Phase 0 skeleton bootstrap")
    print(f"Root: {ROOT}")

    for rel in DIRS:
        ensure_dir(ROOT / rel)

    created = 0
    skipped = 0

    for rel, content in FILES.items():
        target = ROOT / rel
        if target.exists():
            print(f"  skipped (exists): {rel}")
            skipped += 1
        else:
            write_file(target, content)
            created += 1

    for rel, content in EXECUTABLE_FILES.items():
        target = ROOT / rel
        if target.exists():
            print(f"  skipped (exists): {rel}")
            skipped += 1
        else:
            write_executable_file(target, content)
            created += 1

    for mod in [
        "state",
        "context",
        "memory",
        "policy",
        "validation",
        "execution",
        "ledger",
        "replay",
        "audit",
        "api",
        "errors",
    ]:
        target = ROOT / f"core/src/{mod}/mod.rs"
        if not target.exists():
            write_file(target, "// Placeholder module. No runtime behavior implemented.\n")
            created += 1
        else:
            skipped += 1

    print(f"\nDone. {created} file(s) created, {skipped} file(s) already existed.")


if __name__ == "__main__":
    main()
