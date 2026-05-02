#!/usr/bin/env python3
"""
bootstrap_repo.py - Idempotent Phase 0 repository skeleton bootstrap.

Creates missing files and directories only. Does not overwrite existing files.
Writes UTF-8 files with LF newlines.

Usage:
    python3 scripts/bootstrap_repo.py
"""

import os
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent

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
    "docs/examples/prompts",
    "docs/examples/workflows",
    "schemas/docs",
    "schemas/context",
    "schemas/memory",
    "schemas/events",
    "schemas/intents",
    "schemas/traces",
]

FILES = {
    "AGENTS.md": """\
---
truth_dimension: navigation
authority_level: non_authoritative
mutation_path: agents_update
---

# AGENTS.md

This file is a short navigation contract for agents and tools working in this repository.

It is not the system of record.

It does not define governance rules, architecture authority, roadmap commitments, examples, or implementation details.

## Authoritative sources

| Source | Purpose |
| --- | --- |
| `GOVERNANCE.md` | Normative rules, authority model, prohibited patterns, and invariants. |
| `ARCHITECTURE.md` | Structural description of the system, component responsibilities, and data flow. |
| `docs/PHASE_MAP.md` | Planned phase sequence and active phase scope. |
| `checklists/current-phase.md` | Active execution checklist for the current phase. |
| `CHANGELOG.md` | Completed accepted work. |
| `schemas/` | Shared data contracts (JSON Schema). |

## Quick navigation

- **Rules and invariants**: `GOVERNANCE.md`, `docs/governance/`
- **System structure**: `ARCHITECTURE.md`, `docs/architecture/`
- **Roadmap and phases**: `docs/PHASE_MAP.md`, `docs/roadmap/`
- **Active phase tasks**: `checklists/current-phase.md`
- **Data contracts**: `schemas/`
- **Runtime source**: `core/`
- **Browser UI source**: `ui/`
- **Operator scripts**: `scripts/`

## Constraint reminder

Rust owns runtime authority.

TypeScript is a review and operator-intent surface only.

Bash scripts are operator wrappers only.

Model output is untrusted until validated through Rust-owned paths.

This file must remain short, stable, and non-authoritative.
""",

    "CHANGELOG.md": """\
---
truth_dimension: historical
authority_level: authoritative
mutation_path: changelog_entry
---

# CHANGELOG.md

## v0.0.0 - 2026-05-02

**Status:** Phase 0 - Initial Repo Setup

### Added

- Created the initial repository skeleton for AJENTIC.
- Added top-level navigation, orientation, and historical anchors.
- Added canonical directories for Rust core, browser UI, scripts, tests, memory, checklists, docs, schemas, and workflows.
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

    "memory/persistent/project-facts.json": "{}",
    "memory/persistent/conventions.json": "{}",

    "memory/ephemeral/.gitignore": """\
*
!.gitignore
""",

    "schemas/docs/artifact-frontmatter.schema.json": """\
{
  "$schema": "https://json-schema.org/draft/2020-12/schema",
  "type": "object"
}
""",
    "schemas/context/context-packet.schema.json": """\
{
  "$schema": "https://json-schema.org/draft/2020-12/schema",
  "type": "object"
}
""",
    "schemas/context/context-slice.schema.json": """\
{
  "$schema": "https://json-schema.org/draft/2020-12/schema",
  "type": "object"
}
""",
    "schemas/context/context-budget.schema.json": """\
{
  "$schema": "https://json-schema.org/draft/2020-12/schema",
  "type": "object"
}
""",
    "schemas/memory/memory-entry.schema.json": """\
{
  "$schema": "https://json-schema.org/draft/2020-12/schema",
  "type": "object"
}
""",
    "schemas/memory/memory-snapshot.schema.json": """\
{
  "$schema": "https://json-schema.org/draft/2020-12/schema",
  "type": "object"
}
""",
    "schemas/memory/memory-provenance.schema.json": """\
{
  "$schema": "https://json-schema.org/draft/2020-12/schema",
  "type": "object"
}
""",
    "schemas/events/ledger-event.schema.json": """\
{
  "$schema": "https://json-schema.org/draft/2020-12/schema",
  "type": "object"
}
""",
    "schemas/events/run-event.schema.json": """\
{
  "$schema": "https://json-schema.org/draft/2020-12/schema",
  "type": "object"
}
""",
    "schemas/events/audit-event.schema.json": """\
{
  "$schema": "https://json-schema.org/draft/2020-12/schema",
  "type": "object"
}
""",
    "schemas/intents/operator-intent.schema.json": """\
{
  "$schema": "https://json-schema.org/draft/2020-12/schema",
  "type": "object"
}
""",
    "schemas/intents/approve-intent.schema.json": """\
{
  "$schema": "https://json-schema.org/draft/2020-12/schema",
  "type": "object"
}
""",
    "schemas/intents/reject-intent.schema.json": """\
{
  "$schema": "https://json-schema.org/draft/2020-12/schema",
  "type": "object"
}
""",
    "schemas/intents/retry-intent.schema.json": """\
{
  "$schema": "https://json-schema.org/draft/2020-12/schema",
  "type": "object"
}
""",
    "schemas/intents/memory-intent.schema.json": """\
{
  "$schema": "https://json-schema.org/draft/2020-12/schema",
  "type": "object"
}
""",
    "schemas/traces/run-trace.schema.json": """\
{
  "$schema": "https://json-schema.org/draft/2020-12/schema",
  "type": "object"
}
""",
    "schemas/traces/replay-report.schema.json": """\
{
  "$schema": "https://json-schema.org/draft/2020-12/schema",
  "type": "object"
}
""",
    "schemas/traces/validation-report.schema.json": """\
{
  "$schema": "https://json-schema.org/draft/2020-12/schema",
  "type": "object"
}
""",
}


def write_file(path: Path, content: str) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(content, encoding="utf-8", newline="\n")
    print(f"  created: {path.relative_to(ROOT)}")


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

    for mod in [
        "state", "context", "memory", "policy",
        "validation", "execution", "ledger", "replay",
        "audit", "api", "errors",
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
