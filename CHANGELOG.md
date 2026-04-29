# Changelog

## v0.0.10

**Status:** Phase 12 TypeScript UI review surface.

### Added

- Non-authoritative UI review surface scaffold.
- Static display of candidate, evaluation, governance, promotion, ledger, audit, and replay concepts.
- UI trust-boundary labels showing Rust authority and UI non-authority.

### Notes

- The UI does not compute promotion eligibility, mutate lifecycle state, call adapters/providers, write ledger records, emit audit records, perform replay, or provide API behavior.

## v0.0.9

**Status:** Phase 11 cloud model adapter capability.

### Added

- Python cloud provider adapter for OpenAI.
- Cloud provider response parsing and error mapping.
- Tests or documented checks showing cloud adapter output remains non-authoritative.

### Notes

- Cloud model output remains untrusted adapter output.
- No governance change, promotion change, ledger persistence, replay regeneration, API behavior, or UI behavior was implemented.

## v0.0.8

**Status:** Phase 10 local model adapter capability.

### Added

- Python local provider adapter for Ollama.
- Local provider response parsing and error mapping.
- Tests or documented checks showing local adapter output remains non-authoritative.

### Notes

- Local model output remains untrusted adapter output.
- No cloud provider integration, governance change, promotion change, ledger persistence, replay regeneration, API behavior, or UI behavior was implemented.

## v0.0.7

**Status:** Phase 9 replay from ledger facts.

### Added

- Rust-owned replay result model.
- Rust-owned replay from in-memory ledger facts.
- Typed replay errors.
- Replay validation for missing facts, invalid ordering, unsupported promotion, and missing evidence.

### Notes

- No provider calls, adapter calls, evaluator execution, ledger persistence, file IO, API behavior, or UI behavior was implemented.

## v0.0.6

**Status:** Phase 6 evaluation result ingestion.

### Added

- Rust-owned evaluation result ingestion was added.
- Evaluation results are linked to candidate records.
- Evaluation statuses include PASS, FAIL, BLOCKED, and UNKNOWN.
- Required evaluator presence and required evaluator satisfaction checks were added.
- UNKNOWN, FAIL, BLOCKED, malformed, missing, or incomplete results do not satisfy required evaluators.
- Evaluation ingestion does not mutate candidate lifecycle state.

### Notes

- No evaluator execution, governance approval, promotion, ledger persistence, replay, audit emission, real provider integration, API, or UI behavior was implemented.

## v0.0.5

**Status:** Phase 5 candidate creation and runtime adapter checks.

### Added

- Rust-owned candidate record creation from validated adapter responses was added.
- Candidate IDs are assigned deterministically by Rust using run_id and candidate_request_id.
- Candidates created from adapter output start in lifecycle state Created.
- Adapter output remains untrusted after candidate creation.
- Failed, blocked, unknown, malformed, mismatched, incomplete, or empty adapter responses do not create candidates.

### Notes

- No evaluator execution, evaluation result ingestion, governance approval, promotion, ledger persistence, replay, audit emission, real provider integration, API, or UI behavior was implemented.

## v0.0.4

**Status:** Phase 4 adapter protocol and mock adapter.

### Added

- A Rust adapter protocol boundary was added.
- A deterministic Python mock adapter was added.
- Rust validates adapter response linkage and basic shape.
- Malformed, mismatched, empty, over-limit, or failed adapter responses are rejected through typed errors.

### Notes

- No candidate creation, lifecycle mutation, evaluator execution, governance promotion, ledger, replay, audit emission, real provider integration, API, or UI behavior was implemented.

## v0.0.3

**Status:** Phase 3 CLI static validation surface.

### Added

- The CLI now supports static `validate <run-dir>` checks.
- The CLI now supports static `inspect <run-dir>` output.
- Required run files are checked for presence, non-empty content, and expected plain-text markers.

### Notes

- No YAML parsing, schema validation, adapter execution, evaluator execution, governance promotion, ledger, replay, audit emission, API, or UI behavior was implemented.

## v0.0.2

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

## v0.0.1

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
