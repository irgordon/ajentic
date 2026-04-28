# Changelog

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
