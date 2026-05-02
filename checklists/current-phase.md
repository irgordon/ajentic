---
truth_dimension: procedural
authority_level: authoritative
mutation_path: checklist_revision
---

# Current Phase: Phase 5 — Rust Authority Skeleton

This is the active procedural execution surface for Phase 5.

This document does not define governance rules or architecture authority.

This document does not record completed history. Completed accepted work moves to `CHANGELOG.md`.

## Phase

Phase 5 — Rust Authority Skeleton

## Phase goal

Establish compile-safe Rust authority module boundaries and typed skeletons that mirror architecture ownership without implementing runtime harness behavior.

## Allowed surfaces

- `core/src/lib.rs`
- `core/src/main.rs`
- `core/src/state/mod.rs`
- `core/src/context/mod.rs`
- `core/src/memory/mod.rs`
- `core/src/policy/mod.rs`
- `core/src/validation/mod.rs`
- `core/src/execution/mod.rs`
- `core/src/ledger/mod.rs`
- `core/src/replay/mod.rs`
- `core/src/audit/mod.rs`
- `core/src/api/mod.rs`
- `core/src/errors/mod.rs`
- `core/Cargo.toml` (metadata correction only if required)
- `checklists/current-phase.md`
- `CHANGELOG.md`

## Boundary rules for this checklist

- This checklist is procedural truth only for active Phase 5 execution.
- Governance authority remains in `docs/governance/GOVERNANCE.md` and subordinate governance documents.
- Architecture authority remains in `docs/architecture/ARCHITECTURE.md` and subordinate architecture documents.
- This phase defines module/type skeleton boundaries only and must not implement runtime harness behavior.
- Completed accepted work must be recorded in `CHANGELOG.md`, not in this checklist.

## Task checklist

- [ ] Normalize `checklists/current-phase.md` for Phase 5 procedural scope.
- [ ] Replace Rust placeholder module stubs with typed compile-safe skeletons.
- [ ] Ensure `core/src/lib.rs` exports all Rust authority modules.
- [ ] Keep `core/src/main.rs` as a minimal CLI placeholder.
- [ ] Add minimal compile-shape tests only for required boundary constructors.
- [ ] Add `v0.0.5` changelog entry.
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

## Deferred items

| Item | Reason deferred | Target phase |
| --- | --- | --- |
| Lifecycle transition logic | Out of scope for Phase 5 | Later phases |
| Policy evaluator behavior | Out of scope for Phase 5 | Later phases |
| Validation runtime behavior | Out of scope for Phase 5 | Later phases |
| Context/memory/ledger persistence behavior | Out of scope for Phase 5 | Later phases |
| Replay and audit runtime behavior | Out of scope for Phase 5 | Later phases |
| API serving, CLI commands, provider adapters | Out of scope for Phase 5 | Later phases |

## Validation log

| Date | Command | Result | Notes |
| --- | --- | --- | --- |
