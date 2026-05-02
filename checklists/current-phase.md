---
truth_dimension: procedural
authority_level: authoritative
mutation_path: checklist_revision
---

# Current Phase: Phase 7 — Policy and Validation Baseline

This is the active procedural execution surface for Phase 7.

This document does not define governance rules or architecture authority.

This document does not record completed history. Completed accepted work moves to `CHANGELOG.md`.

## Phase

Phase 7 — Policy and Validation Baseline

## Phase goal

Implement deterministic fail-closed policy and validation result surfaces over typed evidence summaries in Rust.

## Allowed surfaces

- `core/src/policy/mod.rs`
- `core/src/validation/mod.rs`
- `core/src/errors/mod.rs` (only if needed for minimal shared error shape)
- `core/src/lib.rs` (only if test placement or export cleanup requires it)
- `checklists/current-phase.md`
- `CHANGELOG.md`

## Boundary rules for this checklist

- This checklist is procedural truth only for active Phase 7 execution.
- Governance authority remains in `docs/governance/GOVERNANCE.md` and subordinate governance documents.
- Architecture authority remains in `docs/architecture/ARCHITECTURE.md` and subordinate architecture documents.
- This phase is limited to deterministic policy and validation evidence-result baselines in Rust.
- This phase must not implement a full policy engine, full schema validation engine, evaluator execution, governance approval, ledger persistence, replay reconstruction, audit projection generation, provider adapters, API behavior, CLI commands, or UI behavior.
- Completed accepted work must be recorded in `CHANGELOG.md`, not in this checklist.

## Task checklist

- [x] Normalize `checklists/current-phase.md` for Phase 7 procedural scope.
- [x] Implement deterministic typed policy baseline in `core/src/policy/mod.rs`.
- [x] Implement deterministic typed validation baseline in `core/src/validation/mod.rs`.
- [x] Preserve `unknown_is_not_pass` semantics for policy and validation unknown constructors.
- [x] Ensure missing evidence, malformed evidence, and model-output claims do not pass.
- [x] Add deterministic unit tests for policy and validation ordering and stable reason/message behavior.
- [x] Add `v0.0.7` changelog entry.
- [x] Run required validation commands and record results.

## Validation checklist

- [x] `python3 scripts/bootstrap_repo.py`
- [x] `python3 -m compileall scripts/bootstrap_repo.py`
- [x] `bash -n scripts/*.sh`
- [x] `find schemas -type f -name "*.json" -print0 | xargs -0 -n1 python3 -m json.tool > /dev/null`
- [ ] `cargo fmt --manifest-path core/Cargo.toml -- --check`
- [x] `cargo check --manifest-path core/Cargo.toml --all-targets`
- [x] `cargo test --manifest-path core/Cargo.toml --all-targets`
- [x] `cargo clippy --manifest-path core/Cargo.toml --all-targets -- -D warnings`
- [x] `cd ui && npm run typecheck && npm run lint && npm run build`
- [x] Embedded Python validation checks from `.github/workflows/structure-lint.yml`
- [x] Embedded Python validation checks from `.github/workflows/docs-gate.yml`

## Deferred items

| Item | Reason deferred | Target phase |
| --- | --- | --- |
| Full policy engine | Out of scope for Phase 7 | Later phases |
| JSON Schema validation engine integration | Out of scope for Phase 7 | Later phases |
| Evaluator execution and governance approval flow | Out of scope for Phase 7 | Later phases |
| Ledger persistence, replay reconstruction, and audit projection generation | Out of scope for Phase 7 | Later phases |
| Provider adapters, API server behavior, CLI command behavior, and UI behavior | Out of scope for Phase 7 | Later phases |

## Validation log

| Date | Command | Result | Notes |
| --- | --- | --- | --- |
| 2026-05-02 | `cargo fmt --manifest-path core/Cargo.toml -- --check` | Fail | Existing formatting drift in `core/src/state/mod.rs`; file outside Phase 7 allowed surfaces. |
| 2026-05-02 | `python3 scripts/bootstrap_repo.py` | Pass | No files created; repository already bootstrapped. |
| 2026-05-02 | `python3 -m compileall scripts/bootstrap_repo.py` | Pass | Compiled successfully. |
| 2026-05-02 | `bash -n scripts/*.sh` | Pass | Shell syntax checks passed. |
| 2026-05-02 | `find schemas -type f -name "*.json" -print0 | xargs -0 -n1 python3 -m json.tool > /dev/null` | Pass | All schema JSON files parse correctly. |
| 2026-05-02 | `cargo check --manifest-path core/Cargo.toml --all-targets` | Pass | Rust core checks passed. |
| 2026-05-02 | `cargo test --manifest-path core/Cargo.toml --all-targets` | Pass | All tests passed. |
| 2026-05-02 | `cargo clippy --manifest-path core/Cargo.toml --all-targets -- -D warnings` | Pass | No clippy warnings. |
| 2026-05-02 | `cd ui && npm run typecheck && npm run lint && npm run build` | Pass | Placeholder UI checks succeeded. |
| 2026-05-02 | Embedded Python from `structure-lint.yml` | Pass | Repository structure is valid. |
| 2026-05-02 | Embedded Python from `docs-gate.yml` | Pass | Documentation boundary checks passed. |
