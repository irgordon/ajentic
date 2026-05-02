---
truth_dimension: procedural
authority_level: authoritative
mutation_path: checklist_revision
---

# Current Phase: Phase 12 — Replay Reconstruction Baseline

This is the active procedural execution surface for Phase 12.

## Phase

Phase 12 — Replay Reconstruction Baseline

## Phase goal

Implement deterministic replay reconstruction from caller-supplied in-memory ledger events into `HarnessState`.

## Allowed surfaces

- `core/src/replay/mod.rs`
- `core/src/ledger/mod.rs` only if a minimal payload accessor or payload shape correction is required
- `core/src/state/mod.rs` only if a minimal read-only helper is required
- `core/src/lib.rs` only if test placement or export cleanup requires it
- `checklists/current-phase.md`
- `CHANGELOG.md`

## Task checklist

- [x] Update checklist to Phase 12 scope.
- [x] Add typed replay reconstruction surfaces in `core/src/replay/mod.rs`.
- [x] Gate reconstruction with `classify_replay_readiness` and fail closed on readiness failure.
- [x] Apply only `LedgerEventType::StateTransition` events.
- [x] Ignore non-state-transition events for reconstruction while preserving caller order.
- [x] Require explicit `payload.lifecycle_transition` for state transitions.
- [x] Apply lifecycle changes only through `HarnessState::transition_to`.
- [x] Fail immediately on invalid lifecycle transitions without returning partial state.
- [x] Add deterministic tests for reconstruction behavior and stable reconstruction error codes.
- [x] Add minimal ledger payload lifecycle-transition field and constructor helper.
- [x] Add `CHANGELOG.md` entry `v0.0.12`.
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
