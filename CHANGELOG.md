# Changelog

## v0.3.0

**Status:** Phase 3 CLI static validation surface.

### Added

- The CLI now supports static `validate <run-dir>` checks.
- The CLI now supports static `inspect <run-dir>` output.
- Required run files are checked for presence, non-empty content, and expected plain-text markers.

### Notes

- No YAML parsing, schema validation, adapter execution, evaluator execution, governance promotion, ledger, replay, audit emission, API, or UI behavior was implemented.

## v0.2.0

**Status:** Phase 2 candidate lifecycle state machine.

### Added

- Candidate lifecycle states were defined in Rust.
- Legal transition checks were added.
- Invalid lifecycle transitions fail through typed errors.
- Transition tests cover valid paths, invalid paths, typed error shape, and terminal-state boundaries.

### Notes

- No promotion governance, evaluator ingestion, ledger, replay, adapter execution, or UI behavior was implemented.
- Passed -> PromotedTier1 is a legal transition shape, not a promotion authorization engine.
- Validation of objectives, constraints, evaluators, policies, and evidence is reserved for later phases.

## v0.1.0

**Status:** Phase 1 contracts and schemas.

### Added

- Concrete Draft 2020-12 schemas for objective, constraints, domain, candidate solution,
  evaluation result, governance result, promotion decision, reuse event, audit record,
  adapter request, adapter response, and policy contracts.
- Rust contract shape modules under `core/src/*/contract.rs` and `core/src/execution/adapter_protocol.rs`.
- Phase 1 contract documentation updates.
- `TASK_LIST.md` for Owner-managed roadmap updates.

### Validation

- `cargo check --workspace`.
- `./scripts/check.sh`.
- Schema structural listing via `find schemas -name '*.schema.json' -type f -print`.

### Notes

This release defines contract shapes only. Validation behavior and runtime governance behavior
will be added in later phases.

## v0.0.0

**Status:** Bootstrap skeleton.

### Added

- Initial AJENTIC repository skeleton.
- Rust workspace with core and cli crates.
- Python adapter placeholder package.
- TypeScript UI placeholder.
- Schema placeholder directory.
- Example minimal run directory.
- Bootstrap shell scripts.
- Root governance documentation.
- AGENTS.md for LLM coding guidance.
- Initial language ownership model.

### Validation

- `cargo check --workspace` must pass.
- `./scripts/check.sh` must pass.
- No third-party dependencies added.
- No runtime behavior implemented.

### Notes

This version is skeleton-only. No model calls, governance engine, evaluators, ledger, replay, or UI dashboard are implemented.
