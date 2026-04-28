# AJENTIC Roadmap
## 1. Purpose
This roadmap defines the phased implementation path for AJENTIC from a blank repository to early production capability.
AJENTIC is a governed harness for local and cloud-based large language models (LLMs). It treats generated output as untrusted, evaluates candidate solutions against explicit objectives and constraints, and promotes only governed outputs for downstream review.

The implementation must progress incrementally. Each phase has a narrow scope, explicit validation gates, and a defined stopping point. Later phases must not backfill hidden behavior into earlier phases.
The primary goal is not feature volume. The primary goal is a usable, maintainable, auditable harness whose control layer remains understandable to first-time developers and extensible by experienced developers.
---
## 2. Architecture rule
The language ownership model is fixed for this roadmap:

```text
Rust = authority
Python = adaptation
TypeScript = visibility
Bash = glue
Go = optional service wrapper, not part of the initial path

Rust owns authoritative lifecycle, governance, ledger, replay, validation, promotion, and audit decisions.

Python adapters may call local or cloud models, but their outputs are untrusted.

TypeScript may display harness state and request actions, but it must not compute authoritative promotion eligibility.

Bash may invoke commands, but it must not encode policy or lifecycle decisions.

Go is not used before early production unless a service wrapper becomes necessary.

â¸»

3. Release track

AJENTIC uses pre-production semantic version anchors.

v0.0.0 = skeleton and governance placeholders (2026-04-27)
v0.1.0 = contracts and schemas
v0.2.0 = lifecycle state machine
v0.3.0 = adapter protocol and mock generation
v0.4.0 = evaluation result ingestion
v0.5.0 = governance and promotion gates
v0.6.0 = ledger and audit trail
v0.7.0 = replay
v0.8.0 = local model adapter capability
v0.9.0 = cloud model adapter capability
v0.10.0 = UI review surface
v0.11.0 = multi-domain capability
v0.12.0 = reuse and bounded improvement records
v0.13.0 = hardening and failure testing
v0.14.0 = early production capability

Version numbers are gates, not marketing labels. A version must not be tagged until its validation criteria pass.

â¸»

4. Global invariants

These apply to every phase.

Generated output is untrusted by default.
Only Rust may promote a candidate to Tier-1.
UNKNOWN is not PASS.
Missing validation is not PASS.
Malformed evidence is not PASS.
Adapters do not decide.
UI does not decide.
Scripts do not decide.
Replay does not regenerate.
Every promoted output must be traceable.
No unsafe bypass flags are allowed.

Disallowed flags:

--force-promote
--skip-policy
--trust-adapter-output
--ignore-unknown
--promote-anyway

Experimental modes may exist later, but they must not emit Tier-1 outputs.

â¸»

5. Phase gate model

Each phase must end with four checks:

Build gate:
  The repository builds or checks successfully.
Contract gate:
  Public file shapes, schemas, and command surfaces match the phase scope.
Behavior gate:
  Only the behavior allowed in the phase exists.
Documentation gate:
  README, roadmap, changelog, and relevant docs reflect current reality.

A phase is not complete if documentation claims behavior that does not exist.

â¸»

Phase 0: Repository bootstrap

Target version: v0.0.0
Primary goal: Create the blank repo skeleton with clear language ownership and no runtime behavior.

Scope

Create the repository structure:

core/       Rust trusted core placeholder
cli/        Rust CLI placeholder
adapters/   Python adapter placeholder
schemas/    JSON Schema placeholders
examples/   Minimal example shape
ui/         TypeScript visibility placeholder
scripts/    Thin shell scripts
docs/       Architecture and governance docs

Allowed

- Rust workspace setup
- core crate skeleton
- cli crate skeleton
- Python adapter package skeleton
- UI placeholder
- schema placeholder files
- example YAML placeholders
- governance documents
- simple shell scripts

Not allowed

- Runtime governance
- Model calls
- Provider integrations
- Evaluators
- Ledger persistence
- Replay logic
- UI dashboard implementation
- API server
- Third-party dependencies

Validation gate

Required commands:

cargo check --workspace
./scripts/check.sh

Expected result:

- Rust workspace compiles
- scripts are executable
- no placeholder file is empty except .gitkeep
- no third-party dependencies are added
- documentation states skeleton-only status

Exit criteria

Phase 0 is complete when a first-time developer can clone the repo, run the check script, and understand the intended architecture from README.md and AGENTS.md.
