---
truth_dimension: procedural
authority_level: authoritative
mutation_path: checklist_revision
---

# Current Phase: Phase 9 — Memory Model Baseline

This is the active procedural execution surface for Phase 9.

This document does not define governance rules or architecture authority.

This document does not record completed history. Completed accepted work moves to `CHANGELOG.md`.

## Phase

Phase 9 — Memory Model Baseline

## Phase goal

Implement deterministic, typed, in-memory memory entry and memory snapshot behavior in Rust from caller-supplied inputs only.

## Allowed surfaces

- `core/src/memory/mod.rs`
- `core/src/errors/mod.rs` only if needed for a minimal shared error shape
- `core/src/lib.rs` only if test placement or export cleanup requires it
- `checklists/current-phase.md`
- `CHANGELOG.md`

## Boundary rules for this checklist

- This checklist is procedural truth only for active Phase 9 execution.
- Governance authority remains in `docs/governance/GOVERNANCE.md` and subordinate governance documents.
- Architecture authority remains in `docs/architecture/ARCHITECTURE.md` and subordinate architecture documents.
- This phase is deterministic in-memory memory-model construction and projection only.
- All memory entries, provenance values, timestamps, and snapshot entries are caller-supplied.
- This phase must not implement file IO, persistence, loading, retrieval, ranking, semantic search, vector search, embeddings, policy authorization, ledger persistence, replay, audit, provider adapters, API server behavior, CLI command behavior, or UI behavior.
- Completed accepted work must be recorded in `CHANGELOG.md`, not in this checklist.

## Task checklist

- [x] Normalize `checklists/current-phase.md` for Phase 9 procedural scope.
- [x] Implement typed memory provenance, entry, snapshot, and memory error surfaces in `core/src/memory/mod.rs`.
- [x] Ensure every memory entry requires caller-supplied provenance and status.
- [x] Implement deterministic snapshot construction and active-entry projection without mutation or reordering.
- [x] Add deterministic unit tests covering required constructors, active-status behavior, snapshot order, deterministic construction, and stable error codes.
- [x] Update `CHANGELOG.md` with `v0.0.9` entry.
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
| Memory file persistence and loading | Out of scope for deterministic in-memory baseline | Later phases |
| Memory retrieval, ranking, semantic search, vector search, and embeddings | Out of scope for deterministic in-memory baseline | Later phases |
| Policy authorization, ledger persistence, replay reconstruction, audit projection, provider adapters, API server behavior, CLI command behavior, and UI behavior | Out of scope for deterministic in-memory baseline | Later phases |

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
| 2026-05-02 | `cargo test --manifest-path core/Cargo.toml --all-targets` | Pass | Rust tests passed including memory tests. |
| 2026-05-02 | `cargo clippy --manifest-path core/Cargo.toml --all-targets -- -D warnings` | Pass | Clippy passed with no warnings. |
| 2026-05-02 | `cd ui && npm run typecheck && npm run lint && npm run build` | Pass | UI validation checks passed in current environment. |
