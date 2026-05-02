---
truth_dimension: procedural
authority_level: authoritative
mutation_path: checklist_revision
---

# Current Phase: Phase 10 — Ledger Event Model Baseline

This is the active procedural execution surface for Phase 10.

This document does not define governance rules or architecture authority.

This document does not record completed history. Completed accepted work moves to `CHANGELOG.md`.

## Phase

Phase 10 — Ledger Event Model Baseline

## Phase goal

Implement deterministic, typed, in-memory ledger event construction and append-only sequence validation in Rust from caller-supplied inputs only.

## Allowed surfaces

- `core/src/ledger/mod.rs`
- `core/src/errors/mod.rs` only if needed for a minimal shared error shape
- `core/src/lib.rs` only if test placement or export cleanup requires it
- `checklists/current-phase.md`
- `CHANGELOG.md`

## Boundary rules for this checklist

- This checklist is procedural truth only for active Phase 10 execution.
- Governance authority remains in `docs/governance/GOVERNANCE.md` and subordinate governance documents.
- Architecture authority remains in `docs/architecture/ARCHITECTURE.md` and subordinate architecture documents.
- This phase is deterministic in-memory ledger event construction and sequence validation only.
- All ledger events, actors, revisions, evidence references, and payload summaries are caller-supplied.
- This phase must not implement file IO, persistence, loading, database storage, serialization, JSON parsing, schema-validation engine integration, replay, audit, state-machine orchestration, policy authorization, provider adapters, API server behavior, CLI command behavior, or UI behavior.
- Completed accepted work must be recorded in `CHANGELOG.md`, not in this checklist.

## Task checklist

- [x] Normalize `checklists/current-phase.md` for Phase 10 procedural scope.
- [x] Implement typed ledger actor, payload, event, error, and ledger surfaces in `core/src/ledger/mod.rs`.
- [x] Preserve existing `LedgerEventType` variants and add required constructor validation for caller-supplied fields.
- [x] Implement deterministic append-only in-memory sequence validation with immutable failure behavior.
- [x] Add deterministic unit tests covering required constructors, required evidence references, sequence constraints, append immutability, insertion order, immutable event access, last revision projection, and stable error codes.
- [x] Update `CHANGELOG.md` with `v0.0.10` entry.
- [x] Run required validation commands and record results.

## Validation checklist

- [x] `python3 scripts/bootstrap_repo.py`
- [x] `python3 -m compileall scripts/bootstrap_repo.py scripts/validate_structure.py scripts/validate_docs.py`
- [x] `python3 scripts/validate_structure.py`
- [x] `python3 scripts/validate_docs.py`
- [x] `bash -n scripts/*.sh`
- [x] `find schemas -type f -name "*.json" -print0 | xargs -0 -n1 python3 -m json.tool > /dev/null`
- [x] `cargo fmt --manifest-path core/Cargo.toml -- --check`
- [x] `cargo check --manifest-path core/Cargo.toml --all-targets`
- [x] `cargo test --manifest-path core/Cargo.toml --all-targets`
- [x] `cargo clippy --manifest-path core/Cargo.toml --all-targets -- -D warnings`
- [x] `cd ui && npm run typecheck && npm run lint && npm run build`

## Deferred items

| Item | Reason deferred | Target phase |
| --- | --- | --- |
| Ledger file persistence, loading, and database storage | Out of scope for deterministic in-memory baseline | Later phases |
| Serialization, JSON parsing, and schema-validation engine integration | Out of scope for deterministic in-memory baseline | Later phases |
| Replay reconstruction, audit projection, state orchestration, policy authorization, provider adapters, API server behavior, CLI command behavior, and UI behavior | Out of scope for deterministic in-memory baseline | Later phases |

## Validation log

| Date | Command | Result | Notes |
| --- | --- | --- | --- |
| 2026-05-02 | `python3 scripts/bootstrap_repo.py` | Pass | Repository bootstrap check passed. |
| 2026-05-02 | `python3 -m compileall scripts/bootstrap_repo.py scripts/validate_structure.py scripts/validate_docs.py` | Pass | Compiled listed Python scripts successfully. |
| 2026-05-02 | `python3 scripts/validate_structure.py` | Pass | Structure validation passed. |
| 2026-05-02 | `python3 scripts/validate_docs.py` | Pass | Documentation validation passed. |
| 2026-05-02 | `bash -n scripts/*.sh` | Pass | Shell syntax checks passed. |
| 2026-05-02 | `find schemas -type f -name "*.json" -print0 | xargs -0 -n1 python3 -m json.tool > /dev/null` | Pass | Schema JSON syntax checks passed. |
| 2026-05-02 | `cargo fmt --manifest-path core/Cargo.toml -- --check` | Pass | Rust formatting check passed. |
| 2026-05-02 | `cargo check --manifest-path core/Cargo.toml --all-targets` | Pass | Rust compile checks passed. |
| 2026-05-02 | `cargo test --manifest-path core/Cargo.toml --all-targets` | Pass | Rust tests passed including ledger tests. |
| 2026-05-02 | `cargo clippy --manifest-path core/Cargo.toml --all-targets -- -D warnings` | Pass | Clippy passed with no warnings. |
| 2026-05-02 | `cd ui && npm run typecheck && npm run lint && npm run build` | Pass | UI validation checks passed in current environment. |
