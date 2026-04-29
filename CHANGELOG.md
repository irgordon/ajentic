# Changelog

## v0.0.17 - 2026-04-29
**Status:** Phase 17 replay integrity and ordering stability verification capability.

### Added
- Replay integrity verification classifications for ordering stability, idempotence, completeness, and overall integrity status.
- Deterministic replay integrity verification output from replay reconstruction based on ledger entries.
- Replay tests for ordering stability determinism, idempotence, completeness detection, integrity violations, non-authoritative behavior, and deterministic replay output.

### Notes
- Replay integrity verification is descriptive only.
- Replay integrity verification does not alter lifecycle state.
- Replay integrity verification does not alter governance approval.
- Replay integrity verification does not alter promotion eligibility.
- Replay integrity verification does not alter ledger authority.
- Replay integrity verification does not introduce persistence, provider integration, API behavior, or UI authority.

## v0.0.15 - 2026-04-29
**Status:** Phase 16 boundary and negative-path hardening.

### Added
- Negative-path tests for governance, promotion, ledger, audit, replay, adapter, domain, UI, and script boundaries.
- Boundary-regression coverage proving existing authority surfaces fail closed.
- Validation coverage confirming non-Rust layers remain non-authoritative.

### Notes
- This phase adds test hardening only.
- No new runtime capability was introduced.
- No lifecycle, governance, promotion, ledger, replay, audit, adapter, provider, API, or UI authority behavior was added.
- CHANGELOG remains the authoritative implementation history.

## v0.0.14 - 2026-04-29
**Status:** Phase 15 replay readiness and status classification capability.

### Added
- Replay readiness classification enum with explicit closed states.
- Replay completion classification enum with explicit closed states.
- Replay final status classification enum with explicit closed states.
- Deterministic replay classification output fields for readiness, completion, and final status.
- Deterministic classification functions deriving readiness, completion, and final status from ledger facts.
- Replay tests for deterministic status classification and deterministic replay output.
- Explicit readiness classification for missing candidate, evaluation, governance, and promotion facts.

### Notes
- Classification is descriptive only.
- Classification is derived from ledger facts and is not stored as authoritative state.
- Incomplete replay states return classification results rather than failing as hard errors.
- Classification does not alter lifecycle state.
- Classification does not alter governance approval.
- Classification does not alter promotion eligibility.
- Classification does not alter ledger authority.
- Classification does not introduce persistence, provider integration, API behavior, or UI authority.

## v0.0.12 - 2026-04-29
**Status:** Phase 14 advisory candidate reuse capability.

### Added
- Rust-owned advisory reuse module.
- Deterministic candidate-to-candidate reuse discovery surface.
- Immutable ReuseApplied ledger record type.
- Ledger append validation requiring both referenced candidates to exist.
- Replay visibility for reuse events reconstructed from ledger facts.

### Notes
- Reuse records trace candidate reuse relationships only. They do not alter lifecycle state, governance approval, promotion eligibility, ledger authority, audit semantics, replay authority, adapter trust, API behavior, or UI authority.


## v0.0.13 - 2026-04-29
**Status:** Phase 14.5 reuse validation and replay integrity testing.

### Added
- Append validation tests proving reuse append fails when source candidate is missing.
- Append validation tests proving reuse append fails when target candidate is missing.
- Replay tests proving reuse events appear for both source and target candidate views.
- Replay tests proving reuse does not affect final status or authoritative identifiers.

### Notes
- These tests strengthen validation coverage only. They do not introduce new runtime capability, lifecycle behavior, governance logic, promotion logic, replay semantics, persistence behavior, adapter integration, API behavior, or UI authority.

## v0.0.11 - 2026-04-28

**Status:** Phase 13 multi-domain capability.

### Added

- Rust-owned domain profile model.
- Built-in sample domain profiles.
- Domain profile validation and compatibility checks.
- Typed domain validation errors.

### Notes

- Domains configure compatibility only. They do not alter lifecycle, governance promotion, ledger, audit, replay, adapter trust, API behavior, or UI authority.

## v0.0.10 - 2026-04-27

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

## v0.0.16 - 2026-04-29
**Status:** Phase 16 architecture alignment and determinism verification capability.

### Added
- Rust-owned architecture alignment verification status enums with explicit, closed vocabularies.
- Deterministic, read-only architecture alignment verification engine over architecture rules, governance boundaries, determinism guarantees, fail-closed behavior, and documentation consistency evidence.
- Verification tests for deterministic output, authority boundary violation detection, determinism stability, fail-closed enforcement, documentation mismatch detection, non-authoritative behavior, and byte-equivalent output consistency.

### Notes
- Verification is descriptive only.
- Verification does not alter lifecycle state.
- Verification does not alter governance approval.
- Verification does not alter promotion eligibility.
- Verification does not alter ledger authority.
- Verification does not introduce persistence, provider integration, API behavior, or UI authority.
