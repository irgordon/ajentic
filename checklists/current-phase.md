---
truth_dimension: procedural
authority_level: authoritative
mutation_path: checklist_revision
---

# Current Phase: Phase 17 — Promotion Ledger Append Baseline

This is the active procedural execution surface for Phase 17.

## Phase

Phase 17 — Promotion Ledger Append Baseline

## Phase goal

Implement a narrow Rust-owned deterministic helper that appends an already-built promotion record to a caller-supplied in-memory ledger using `Ledger::append` sequencing.

## Allowed surfaces

- `core/src/execution/mod.rs`
- `core/src/lib.rs` only if test placement or export cleanup requires it
- `checklists/current-phase.md`
- `CHANGELOG.md`

## Task checklist

- [x] Update checklist to Phase 17 scope.
- [x] Preserve existing Phase 14, Phase 15, and Phase 16 behavior.
- [x] Add deterministic promotion append error type with stable code mapping.
- [x] Add deterministic `append_promotion_record(...)` using only caller-supplied `Ledger` and `PromotionRecord`.
- [x] Delegate revision sequencing to `Ledger::append`.
- [x] Return a new `Ledger` on success.
- [x] Do not mutate input `Ledger`.
- [x] Do not mutate `HarnessState` and do not call `HarnessState::transition_to`.
- [x] Do not call `decide_execution`, `decide_promotion`, or `build_promotion_record` inside append helper.
- [x] Add deterministic unit tests for required append behavior and non-goals.
- [x] Add `CHANGELOG.md` entry `v0.0.17`.
- [x] Run required validation commands.

## Validation checklist

- [x] `./scripts/check.sh`
- [x] `cd ui && npm run typecheck && npm run lint && npm run build`
