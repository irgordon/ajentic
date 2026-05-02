---
truth_dimension: procedural
authority_level: authoritative
mutation_path: checklist_revision
---

# Current Phase: Phase 18 — Promotion Replay Verification Baseline

This is the active procedural execution surface for Phase 18.

## Phase

Phase 18 — Promotion Replay Verification Baseline

## Phase goal

Implement a narrow read-only Rust-owned verification helper that confirms an existing in-memory ledger replay-classifies and reconstructs to `LifecycleState::PromotedTier1`.

## Allowed surfaces

- `core/src/execution/mod.rs`
- `core/src/lib.rs` only if test placement or export cleanup requires it
- `checklists/current-phase.md`
- `CHANGELOG.md`

## Task checklist

- [x] Update checklist to Phase 18 scope.
- [x] Preserve existing Phase 14, 15, 16, and 17 behavior.
- [x] Add read-only promotion replay verification types with stable reason codes.
- [x] Add deterministic `verify_promotion_replay(...)` using caller-supplied in-memory `Ledger` only.
- [x] Verify readiness via `classify_replay_readiness(events)`.
- [x] Verify reconstruction via `reconstruct_harness_state(events)`.
- [x] Return `NotVerified` on replay-not-ready, reconstruction failure, or non-promoted final lifecycle.
- [x] Return `Verified` with reconstructed revision and replay counters when final lifecycle is `PromotedTier1`.
- [x] Do not mutate ledger events, append events, mutate `HarnessState`, or repair/reorder replay.
- [x] Add deterministic unit tests for required behavior.
- [x] Add `CHANGELOG.md` entry `v0.0.18`.
- [x] Run required validation commands.

## Validation checklist

- [x] `./scripts/check.sh`
- [x] `cd ui && npm run typecheck && npm run lint && npm run build`
