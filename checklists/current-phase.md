---
truth_dimension: procedural
authority_level: authoritative
mutation_path: checklist_revision
---

# Current Phase: Phase 6 — Candidate Lifecycle State Machine

This is the active procedural execution surface for Phase 6.

This document does not define governance rules or architecture authority.

This document does not record completed history. Completed accepted work moves to `CHANGELOG.md`.

## Phase

Phase 6 — Candidate Lifecycle State Machine

## Phase goal

Implement authoritative candidate lifecycle transition behavior in Rust with typed lifecycle errors and deterministic tests.

## Allowed surfaces

- `core/src/state/mod.rs`
- `core/src/lib.rs` (only if test placement or export cleanup requires it)
- `checklists/current-phase.md`
- `CHANGELOG.md`

## Boundary rules for this checklist

- This checklist is procedural truth only for active Phase 6 execution.
- Governance authority remains in `docs/governance/GOVERNANCE.md` and subordinate governance documents.
- Architecture authority remains in `docs/architecture/ARCHITECTURE.md` and subordinate architecture documents.
- This phase is limited to lifecycle transition state behavior in Rust.
- This phase must not implement governance approval, promotion authorization, evaluator execution, ledger persistence, replay, audit projection, provider adapters, API behavior, or UI mutation.
- Completed accepted work must be recorded in `CHANGELOG.md`, not in this checklist.

## Task checklist

- [ ] Normalize `checklists/current-phase.md` for Phase 6 procedural scope.
- [ ] Implement legal lifecycle transition checks in `core/src/state/mod.rs`.
- [ ] Add typed `LifecycleError` and stable error code mapping.
- [ ] Enforce UNKNOWN is not PASS and CREATED cannot directly become PROMOTED_TIER_1.
- [ ] Keep terminal lifecycle states terminal and deterministic.
- [ ] Implement `HarnessState::transition_to` revision behavior.
- [ ] Add deterministic unit tests for lifecycle and `HarnessState` transition behavior.
- [ ] Add `v0.0.6` changelog entry.
- [ ] Run required validation commands and record results.

## Validation checklist

- [ ] `python3 scripts/bootstrap_repo.py`
- [ ] `python3 -m compileall scripts/bootstrap_repo.py`
- [ ] `bash -n scripts/*.sh`
- [ ] `find schemas -type f -name "*.json" -print0 | xargs -0 -n1 python3 -m json.tool > /dev/null`
- [ ] `cargo fmt --manifest-path core/Cargo.toml -- --check`
- [ ] `cargo check --manifest-path core/Cargo.toml --all-targets`
- [ ] `cargo test --manifest-path core/Cargo.toml --all-targets`
- [ ] `cargo clippy --manifest-path core/Cargo.toml --all-targets -- -D warnings`
- [ ] `cd ui && npm run typecheck && npm run lint && npm run build`
- [ ] Embedded Python validation checks from `.github/workflows/structure-lint.yml`
- [ ] Embedded Python validation checks from `.github/workflows/docs-gate.yml`

## Deferred items

| Item | Reason deferred | Target phase |
| --- | --- | --- |
| Governance approval and promotion authorization | Out of scope for Phase 6 | Later phases |
| Evaluator execution integration | Out of scope for Phase 6 | Later phases |
| Ledger persistence and replay reconstruction | Out of scope for Phase 6 | Later phases |
| Audit projection and policy gate implementation | Out of scope for Phase 6 | Later phases |
| Provider adapters, API server behavior, and UI mutation | Out of scope for Phase 6 | Later phases |

## Validation log

| Date | Command | Result | Notes |
| --- | --- | --- | --- |
