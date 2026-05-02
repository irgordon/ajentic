---
truth_dimension: procedural
authority_level: authoritative
mutation_path: checklist_revision
---

# Current Phase: Phase 13 — Audit Projection Baseline

This is the active procedural execution surface for Phase 13.

## Phase

Phase 13 — Audit Projection Baseline

## Phase goal

Implement deterministic, read-only audit projections over caller-supplied ledger events and replay outputs.

## Allowed surfaces

- `core/src/replay/mod.rs`
- `core/src/ledger/mod.rs` only if a minimal payload accessor or payload shape correction is required
- `core/src/state/mod.rs` only if a minimal read-only helper is required
- `core/src/lib.rs` only if test placement or export cleanup requires it
- `checklists/current-phase.md`
- `CHANGELOG.md`

## Task checklist

- [x] Update checklist to Phase 13 scope.
- [x] Add deterministic read-only audit projection types and constructor validation in `core/src/audit/mod.rs`.
- [x] Add typed `AuditError` variants and stable `AuditError::code()` values.
- [x] Add deterministic `project_ledger_timeline(...)` projection over caller-supplied ledger events.
- [x] Add deterministic `project_replay_summary(...)` projection over caller-supplied replay reports.
- [x] Add deterministic `project_reconstruction_summary(...)` projection over caller-supplied replay reconstruction outputs.
- [x] Preserve caller-supplied ledger event order in timeline projections.
- [x] Keep projection helpers read-only and avoid replay reconstruction/readiness calls.
- [x] Add stable code helpers only where required for deterministic audit summaries.
- [x] Add deterministic unit tests for audit projection behavior and stable code helpers.
- [x] Add `CHANGELOG.md` entry `v0.0.13`.
- [x] Run required validation commands.

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
