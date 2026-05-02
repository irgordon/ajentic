---
truth_dimension: procedural
authority_level: authoritative
mutation_path: checklist_revision
---

# Current Phase: Phase 11 — Replay Readiness and Integrity Baseline

This is the active procedural execution surface for Phase 11.

This document does not define governance rules or architecture authority.

This document does not record completed history. Completed accepted work moves to `CHANGELOG.md`.

## Phase

Phase 11 — Replay Readiness and Integrity Baseline

## Phase goal

Implement deterministic, read-only replay-readiness and ledger-integrity classification in Rust over caller-supplied in-memory ledger events.

## Allowed surfaces

- `core/src/replay/mod.rs`
- `core/src/ledger/mod.rs` only if a minimal read-only accessor is required and cannot be avoided
- `core/src/errors/mod.rs` only if needed for a minimal shared error shape
- `core/src/lib.rs` only if test placement or export cleanup requires it
- `checklists/current-phase.md`
- `CHANGELOG.md`

## Boundary rules for this checklist

- This checklist is procedural truth only for active Phase 11 execution.
- Governance authority remains in `docs/governance/GOVERNANCE.md` and subordinate governance documents.
- Architecture authority remains in `docs/architecture/ARCHITECTURE.md` and subordinate architecture documents.
- This phase is deterministic replay-readiness and ledger-integrity classification only.
- All replay classification input events are caller-supplied and evaluated in caller-supplied order.
- This phase must not reconstruct state, apply events, repair history, sort events, infer missing events, persist ledger data, load ledger data, parse JSON, validate schemas at runtime, generate audit outputs, invoke providers, serve APIs, execute CLI behavior, or render UI behavior.
- Completed accepted work must be recorded in `CHANGELOG.md`, not in this checklist.

## Task checklist

- [x] Normalize `checklists/current-phase.md` for Phase 11 procedural scope.
- [x] Implement typed replay readiness, integrity, report, and replay error surfaces in `core/src/replay/mod.rs`.
- [x] Preserve and minimally adapt existing replay status/integrity/report shapes from prior phases.
- [x] Add deterministic `classify_replay_readiness` classification over caller-supplied ledger events.
- [x] Add `report_from_error` projection helper with required integrity and reason mapping.
- [x] Add deterministic unit tests for unknown-not-pass behavior, stable error codes, empty-ledger handling, revision checks, conflicting event IDs, input-order preservation, replayable counting, and error-to-report mapping.
- [x] Update `CHANGELOG.md` with `v0.0.11` entry.
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
| Replay reconstruction and state rehydration | Out of scope for read-only replay-readiness baseline | Later phases |
| Event application, replay repair, and history inference | Out of scope for classification-only baseline | Later phases |
| Ledger persistence, loading, serialization, JSON parsing, and schema-validation integration | Out of scope for classification-only baseline | Later phases |
| Audit projection, provider adapters, API server behavior, CLI command behavior, and UI behavior | Out of scope for classification-only baseline | Later phases |

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
| 2026-05-02 | `cargo test --manifest-path core/Cargo.toml --all-targets` | Pass | Rust tests passed including replay tests. |
| 2026-05-02 | `cargo clippy --manifest-path core/Cargo.toml --all-targets -- -D warnings` | Pass | Clippy passed with no warnings. |
| 2026-05-02 | `cd ui && npm run typecheck && npm run lint && npm run build` | Pass | UI validation checks passed in current environment. |
