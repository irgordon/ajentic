---
truth_dimension: procedural
authority_level: authoritative
mutation_path: checklist_revision
---

# Current Phase: Phase 16 — Promotion Recording Baseline

This is the active procedural execution surface for Phase 16.

## Phase

Phase 16 — Promotion Recording Baseline

## Phase goal

Implement a narrow Rust-owned deterministic promotion recording surface that constructs a caller-supplied ledger event shape only after an already-allowed promotion decision.

## Allowed surfaces

- `core/src/execution/mod.rs`
- `core/src/ledger/mod.rs` only if a minimal ledger event type/code helper is required
- `core/src/lib.rs` only if test placement or export cleanup requires it
- `checklists/current-phase.md`
- `CHANGELOG.md`

## Task checklist

- [x] Update checklist to Phase 16 scope.
- [x] Preserve existing Phase 14 and Phase 15 behavior.
- [x] Add deterministic promotion record request/result types and stable error code helpers.
- [x] Add deterministic `build_promotion_record(...)` using only caller-supplied promotion decision report, actor, evidence refs, and payload summary.
- [x] Fail closed unless promotion decision is `Allowed`.
- [x] Construct `LedgerEventType::StateTransition` with lifecycle transition payload targeting `LifecycleState::PromotedTier1`.
- [x] Do not append to `Ledger`, mutate `HarnessState`, call `HarnessState::transition_to`, or call decision functions inside record construction.
- [x] Add deterministic unit tests for promotion record construction behavior and non-goals.
- [x] Add `CHANGELOG.md` entry `v0.0.16`.
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
