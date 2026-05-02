---
truth_dimension: procedural
authority_level: authoritative
mutation_path: checklist_revision
---

# Current Phase: Phase 15 — Promotion Decision Baseline

This is the active procedural execution surface for Phase 15.

## Phase

Phase 15 — Promotion Decision Baseline

## Phase goal

Implement a narrow Rust-owned deterministic promotion decision classification over caller-supplied lifecycle and execution decision results.

## Allowed surfaces

- `core/src/execution/mod.rs`
- `core/src/lib.rs` only if test placement or export cleanup requires it
- `checklists/current-phase.md`
- `CHANGELOG.md`

## Task checklist

- [x] Update checklist to Phase 15 scope.
- [x] Preserve existing `ExecutionStatus` and `ExecutionPlan` names.
- [x] Add deterministic promotion decision types and stable reason code helpers.
- [x] Add deterministic `decide_promotion(...)` using caller-supplied lifecycle and execution decision inputs only.
- [x] Fail closed unless lifecycle is Passed and execution decision is Allowed.
- [x] Add deterministic unit tests for decision behavior, reason-code stability, precedence, and reason-string independence.
- [x] Add `CHANGELOG.md` entry `v0.0.15`.
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
