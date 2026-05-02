---
truth_dimension: historical
authority_level: authoritative
mutation_path: changelog_entry
---

# CHANGELOG.md

## v0.0.16 - 2026-05-02

**Status:** Phase 16 - Promotion Recording Baseline

### Changed

- Updated `core/src/execution/mod.rs` to add deterministic promotion record construction types, stable promotion record error codes, and fail-closed record assembly from caller-supplied promotion decision and ledger event inputs.
- Updated `checklists/current-phase.md` for Phase 16 promotion recording scope.
- Updated `CHANGELOG.md` with `v0.0.16`.

### Notes

- Phase 16 constructs a promotion ledger event shape only after an allowed promotion decision.
- build_promotion_record(...) creates a caller-supplied StateTransition ledger event targeting PromotedTier1.
- It does not append that event, apply it, authorize it independently, persist it, or mutate HarnessState.
- The promotion record is not appended to a ledger and does not mutate HarnessState.
- No lifecycle transition, promotion execution, ledger persistence, replay, audit, provider adapter, API server, CLI command, or UI behavior was implemented.
- No new dependencies were added.

## v0.0.15 - 2026-05-02

**Status:** Phase 15 - Promotion Decision Baseline

### Changed

- Updated `core/src/execution/mod.rs` to add deterministic promotion decision classification types, stable reason code helpers, and fail-closed decision ordering over caller-supplied lifecycle and execution decision inputs.
- Updated `checklists/current-phase.md` for Phase 15 promotion scope.
- Updated `CHANGELOG.md` with `v0.0.15`.

### Notes

- Phase 15 implements promotion decision classification only.
- Promotion decisions use caller-supplied lifecycle and execution decision results.
- No lifecycle transition, promotion execution, ledger append, provider adapter, API server, CLI command, or UI behavior was implemented.
- PromotedTier1 remains a lifecycle state shape; entering it still requires a separate authorized transition path.
- No new dependencies were added.

## v0.0.14 - 2026-05-02

**Status:** Phase 14 - Execution Decision Baseline

### Changed

- Updated `core/src/execution/mod.rs` to add deterministic execution decision classification types, stable decision-reason codes, and fail-closed decision ordering over caller-supplied lifecycle, policy, validation, and replay-readiness inputs.
- Updated `checklists/current-phase.md` for Phase 14 execution scope.
- Updated `CHANGELOG.md` with `v0.0.14`.

### Notes

- Phase 14 implements execution decision classification only.
- decide_execution(...) classifies caller-supplied lifecycle, policy, validation, and replay readiness results.
- It does not evaluate those inputs, mutate them, or execute anything.
- Decisions combine caller-supplied lifecycle, policy, validation, and replay-readiness results.
- No execution engine, tool invocation, provider adapter, ledger append, replay reconstruction, audit projection, API server, CLI command, or UI behavior was implemented.
- No new dependencies were added.

## v0.0.13 - 2026-05-02

**Status:** Phase 13 - Audit Projection Baseline

### Changed

- Updated `core/src/audit/mod.rs` to replace the Phase 5 skeleton with deterministic read-only audit projection types, typed audit errors with stable codes, and pure projection helpers for ledger timelines, replay summaries, and reconstruction summaries.
- Updated `core/src/replay/mod.rs` with stable replay status/integrity/readiness code helpers and deterministic tests used by audit projection summaries.
- Updated `core/src/ledger/mod.rs` with stable ledger event type code helpers and deterministic tests used by timeline projection summaries.
- Updated `checklists/current-phase.md` for Phase 13 execution scope.
- Updated `CHANGELOG.md` with `v0.0.13`.

### Notes

- Phase 13 implements read-only audit projections over caller-supplied in-memory ledger and replay data only.
- Audit projections are deterministic views over caller-supplied in-memory ledger/replay data.
- They are not durable audit records, exported reports, replay reconstruction, or runtime authority.
- Audit projections do not persist, export, reconstruct replay, classify readiness, mutate state, call providers, serve APIs, execute CLI commands, or render UI.
- Stable code methods were added only where required to avoid Debug-format-derived audit text.
- No new dependencies were added.

## v0.0.12 - 2026-05-02

**Status:** Phase 12 - Replay Reconstruction Baseline

### Changed

- Updated `core/src/replay/mod.rs` with typed replay reconstruction surfaces, fail-closed reconstruction flow, and deterministic unit tests for supported lifecycle state-transition replay behavior.
- Updated `core/src/ledger/mod.rs` to extend `LedgerPayload` with optional lifecycle-transition requests and a minimal constructor helper for state-transition payloads.
- Updated `checklists/current-phase.md` for Phase 12 execution scope.
- Updated `CHANGELOG.md` with `v0.0.12`.

### Notes

- Phase 12 reconstructs HarnessState from supported in-memory ledger events only.
- reconstruct_harness_state(...) reconstructs only supported lifecycle state transitions from in-memory caller-supplied ledger events.
- It does not reconstruct arbitrary runtime state, infer transitions from text, repair ledger history, persist/load ledgers, audit, serve APIs, or invoke providers.
- Replay readiness classification still gates reconstruction.
- Unsupported or invalid state-transition payloads fail closed.
- No persistence, loading, repair, audit, provider, API, CLI, or UI behavior was implemented.
- No new dependencies were added.

## v0.0.11 - 2026-05-02

**Status:** Phase 11 - Replay Readiness and Integrity Baseline

### Changed

- Updated `core/src/replay/mod.rs` with replay readiness, integrity, report, and typed replay error surfaces.
- Added deterministic replay-readiness classification over caller-supplied ledger events with checks for empty ledger, first revision validity, revision continuity, duplicate revisions, conflicting event IDs, and replayable event counting.
- Added unit tests covering unknown-not-ready behavior, stable error codes, empty ledgers, invalid first revision, revision gaps, duplicate revisions, conflicting event IDs, input-order preservation, and error-to-report projection mappings.
- Updated `checklists/current-phase.md` for Phase 11 execution scope.

### Notes

- Phase 11 implements replay-readiness and ledger-integrity classification only.
- classify_replay_readiness(...) classifies whether caller-supplied ledger events are structurally replay-ready.
- It does not replay, reconstruct, repair, persist, audit, or apply events.
- No replay reconstruction, event application, state rehydration, replay repair, ledger persistence, audit projection, provider adapter, API server, CLI command, or UI behavior was implemented.
- Replay classification uses caller-supplied ledger event order and does not sort or infer missing events.
- No new dependencies were added.

## v0.0.10 - 2026-05-02

**Status:** Phase 10 - Ledger Event Model Baseline

### Changed

- Updated `core/src/ledger/mod.rs` with typed ledger actor, payload, event, ledger, and error surfaces.
- Added deterministic append-only in-memory ledger sequence checks from caller-supplied events.
- Added unit tests covering required event fields, evidence references, revision sequencing, append failure immutability, event ordering, immutable event access, last revision projection, and stable error codes.
- Updated `checklists/current-phase.md` for Phase 10 execution scope.

### Notes

- Phase 10 implements an in-memory ledger event model and append sequence checks only.
- Ledger::append validates revision sequence and returns a new in-memory ledger.
- It does not persist, serialize, authorize, replay, audit, or orchestrate state.
- No ledger file IO, persistence, loading, database storage, serialization, replay, audit, state orchestration, policy authorization, provider adapter, API server, CLI command, or UI behavior was implemented.
- All ledger events, actors, evidence refs, revisions, and payload summaries are caller-supplied.
- `Ledger::append` is append-only and returns a new ledger on success without mutating the existing ledger.
- No new dependencies were added.

## v0.0.9 - 2026-05-02

**Status:** Phase 9 - Memory Model Baseline

### Changed

- Updated `core/src/memory/mod.rs` with typed memory provenance, entry, snapshot, status, and error surfaces.
- Added deterministic in-memory snapshot assembly from caller-supplied entries.
- Added unit tests covering required provenance fields, required entry fields, active-status projection, snapshot order preservation, deterministic construction, and stable error codes.
- Updated `checklists/current-phase.md` for Phase 9 execution scope.

### Notes

- Phase 9 implements in-memory memory model shapes and deterministic projections only.
- MemorySnapshot::active_entries is a deterministic projection, not retrieval.
- MemoryEntry::is_active is classification, not authorization.
- No memory file IO, persistence, loading, retrieval, ranking, semantic search, vector search, embeddings, policy authorization, ledger persistence, replay, audit, provider adapter, API server, CLI command, or UI behavior was implemented.
- All memory entries, provenance values, timestamps, and snapshot entries are caller-supplied.
- No new dependencies were added.

## v0.0.8.5 - 2026-05-02

**Status:** Phase 8.5 - CI Validation Script Extraction

### Added

- Added `scripts/validate_structure.py` as the first-class repository structure validation entrypoint.
- Added `scripts/validate_docs.py` as the first-class documentation boundary validation entrypoint.

### Changed

- Updated `.github/workflows/structure-lint.yml` to call `scripts/validate_structure.py` instead of embedding Python in workflow YAML.
- Updated `.github/workflows/docs-gate.yml` to call `scripts/validate_docs.py` instead of embedding Python in workflow YAML.
- Updated `checklists/current-phase.md` for Phase 8.5 execution scope.
- Updated `CHANGELOG.md` with the Phase 8.5 maintenance entry.

### Notes

- Phase 8.5 is CI/CD maintenance only.
- No runtime harness behavior was implemented.
- Local validation and GitHub Actions now use the same Python script entrypoints for structure and documentation gates.
- No new dependencies were added.

## v0.0.8 - 2026-05-02

**Status:** Phase 8 - Context Packet Assembly

### Changed

- Updated `core/src/context/mod.rs` with deterministic context packet assembly types for truth dimensions, sources, tasks, budgets, and provenance-carrying slices.
- Added typed fail-closed `ContextError` variants with stable error codes and explicit constructor validation for packet, task, source, slice, budget, and packet provenance boundaries.
- Added deterministic unit tests covering budget checks, required provenance fields, required source metadata, deterministic packet assembly, slice/source order preservation, and stable context error codes.
- Updated `checklists/current-phase.md` for Phase 8 execution scope, boundaries, tasks, validation checklist, deferred items, and validation log table.

### Notes

- Phase 8 implements context packet assembly from caller-supplied inputs only.
- No repository scanning, context retrieval, memory retrieval, tokenization, model invocation, provider adapter, ledger persistence, replay reconstruction, audit projection, API server behavior, CLI command behavior, or UI behavior was implemented.
- Budget values are caller-supplied and only checked for `used_units <= max_units`.
- Sources and slices are caller-supplied and are not reconciled in this phase.
- No new dependencies were added.

## v0.0.7 - 2026-05-02

**Status:** Phase 7 - Policy and Validation Baseline

### Changed

- Updated `core/src/policy/mod.rs` with deterministic policy evidence evaluation, fail-closed decision ordering, and stable reason helpers.
- Updated `core/src/validation/mod.rs` with deterministic validation evidence evaluation, fail-closed status ordering, and stable message helpers.
- Added unit tests covering missing evidence behavior, malformed evidence behavior, unknown handling, model-output non-authority, deterministic failure ordering, and stable result messages.
- Updated `checklists/current-phase.md` for Phase 7 execution scope, boundaries, tasks, validation checklist, deferred items, and validation log table.

### Notes

- Phase 7 implements narrow policy and validation result baselines only.
- No full policy engine, JSON Schema validation engine, evaluator execution, governance approval, ledger persistence, replay reconstruction, audit projection generation, provider adapter, API server, CLI command, or UI behavior was implemented.
- Model output claims do not create policy approval or validation pass results.
- No new dependencies were added.
- `core/src/state/mod.rs` received rustfmt-only whitespace/layout normalization; lifecycle logic was unchanged.

## v0.0.6 - 2026-05-02

**Status:** Phase 6 - Candidate Lifecycle State Machine

### Changed

- Updated `core/src/state/mod.rs` with deterministic lifecycle transition checks, typed `LifecycleError`, lifecycle error codes, and immutable `HarnessState::transition_to` revision behavior.
- Added lifecycle unit tests covering valid transitions, invalid transitions, terminal states, UNKNOWN handling, and `HarnessState` revision behavior.
- Updated `checklists/current-phase.md` for Phase 6 execution scope, boundaries, tasks, validation checklist, deferred items, and validation log table.

### Notes

- Phase 6 implements lifecycle transition behavior only.
- No governance approval, promotion authorization, evaluator execution, ledger persistence, replay, audit projection, provider adapter, API server, or UI behavior was implemented.
- `PromotedTier1` remains a lifecycle state shape only; authorization to enter it belongs to later Rust-owned governance or promotion logic.
- No new dependencies were added.
- `core/src/state/mod.rs` received rustfmt-only whitespace/layout normalization; lifecycle logic was unchanged.

## v0.0.5 - 2026-05-02

**Status:** Phase 5 - Rust Authority Skeleton

### Changed

- Normalized `checklists/current-phase.md` to Phase 5 procedural scope including allowed surfaces, task checklist, validation checklist, deferred items, and validation log table.
- Replaced placeholder Rust module stubs under `core/src/` with compile-safe authority boundary skeleton types for state, context, memory, policy, validation, execution, ledger, replay, audit, api, and errors.
- Added minimal compile-shape tests in `core/src/lib.rs` for `HarnessState::genesis`, `PolicyResult::unknown`, `ValidationResult::unknown`, and `ReplayReport::unknown`.
- Kept `core/src/main.rs` as a minimal CLI placeholder surface.

### Notes

- Phase 5 establishes Rust authority module shapes only.
- No runtime harness behavior was implemented.
- Lifecycle transitions, policy gates, validation logic, ledger persistence, replay reconstruction, audit generation, API serving, and provider adapters remain unimplemented.
- No new dependencies were added.
- `core/src/state/mod.rs` received rustfmt-only whitespace/layout normalization; lifecycle logic was unchanged.

## v0.0.4 - 2026-05-02

**Status:** Phase 4 - Contract Schema Baseline

### Changed

- Normalized `checklists/current-phase.md` to Phase 4 procedural scope with updated allowed surfaces, task checklist, validation checklist, deferred items, and validation log table.
- Replaced placeholder contract schema files under `schemas/docs/`, `schemas/context/`, `schemas/memory/`, `schemas/events/`, `schemas/intents/`, and `schemas/traces/` with strict Draft 2020-12 JSON Schema baselines including explicit required fields, stable enums, boundary versioning, and closed top-level shapes.
- Reviewed Phase 4 workflow surfaces and kept `.github/workflows/ci.yml`, `.github/workflows/structure-lint.yml`, `.github/workflows/docs-gate.yml`, and `.github/workflows/memory-lint.yml` unchanged because existing checks remain compatible with schema baseline changes.

### Notes

- Phase 4 defines contract shapes only.
- No runtime harness behavior was implemented.
- TypeScript bindings, runtime validators, and Rust schema consumers are not implemented in this phase.
- JSON Schema remains the contract truth root under `schemas/`.

## v0.0.3 - 2026-05-02

**Status:** Phase 3 - CI and Structure Drift Gates

### Changed

- Normalized `checklists/current-phase.md` to Phase 3 procedural scope, boundaries, task checklist, validation checklist, deferred items, and validation log table.
- Hardened `.github/workflows/structure-lint.yml` to reject root `PHASE_MAP.md` in addition to root governance and architecture anchors.
- Hardened `.github/workflows/docs-gate.yml` to reject root `PHASE_MAP.md` and exclude `.github/instructions/*.instructions.md` from governed documentation boundary checks.

### Notes

- Phase 3 is a CI and drift-gate hardening phase only.
- No runtime harness behavior was implemented.
- Workflows remain enforcement wiring and do not create standalone governance.
- Updated .github/workflows/docs-gate.yml to narrow pattern checks and avoid false positives on valid governance and README boundary language.

## v0.0.2 - 2026-05-02

**Status:** Phase 2 - Phase Execution Loop and Active Checklist

### Changed

- Audited and normalized `docs/roadmap/phase-map.md` so phase structure remains agent-executable and planned-only.
- Clarified `docs/governance/phase-execution-contract.md` execution surfaces and changelog handoff boundaries.
- Replaced `checklists/current-phase.md` with Phase 2 procedural execution surface content and validation tracking tables.
- Updated `AGENTS.md` navigation links to include the phase execution contract as an authoritative source pointer.

### Notes

- Phase 2 is a planning and execution-discipline phase only.
- No runtime harness behavior was implemented.
- `docs/roadmap/phase-map.md` remains planned truth and must not record completed work.
- `checklists/current-phase.md` remains the only active phase execution surface.
- Updated .github/workflows/structure-lint.yml so GitHub Copilot instruction files use applyTo metadata without requiring governed artifact frontmatter.
- Updated .gitignore to exclude Rust, Node.js, Python, editor, environment, and ephemeral memory artifacts from repository tracking.

## v0.0.1 - 2026-05-02

**Status:** Phase 1 - Governance and Architecture Baseline

### Added

- No new files were added in this phase.

### Changed

- Audited and normalized the Phase 1 execution surface in `checklists/current-phase.md`.
- Audited governance anchor and governance subdocuments under `docs/governance/` for authoritative frontmatter, scope, and subordination.
- Audited architecture anchor and architecture subdocuments under `docs/architecture/` for structural scope and non-implementation wording.
- Verified `AGENTS.md` remains navigation-only and non-authoritative.
- Verified `README.md` remains orientation-only and non-authoritative.

### Notes

- Phase 1 is a documentation and boundary-hardening phase only.
- No runtime harness behavior was implemented.
- Governance and architecture subdocuments remain subordinate to their docs-owned anchors.

## v0.0.0 - 2026-05-02

**Status:** Phase 0 - Initial Repo Setup

### Added

- Created the initial repository skeleton for AJENTIC.
- Added top-level navigation, orientation, and historical anchors.
- Added canonical directories for Rust core, browser UI, scripts, tests, memory, checklists, docs, schemas, and workflows.
- Added minimal Rust core compile skeleton under `core/`.
- Added minimal TypeScript browser UI placeholder structure under `ui/`.
- Added initial governed documentation directories under `docs/`.
- Added placeholder JSON Schema files under `schemas/`.

### Changed

- Moved `GOVERNANCE.md` from repository root to `docs/governance/GOVERNANCE.md` per structure-lint contract.
- Moved `ARCHITECTURE.md` from repository root to `docs/architecture/ARCHITECTURE.md` per structure-lint contract.
- Normalized `AGENTS.md` as a navigation-only contract using docs-owned governance, architecture, and phase-map paths.
- Updated `checklists/current-phase.md` to reference canonical paths for governance and architecture anchors.
- Updated `docs/roadmap/phases.md` and `docs/roadmap/sequencing.md` to reference `docs/roadmap/phase-map.md` instead of `docs/PHASE_MAP.md`.
- Updated docs subdocuments in `docs/governance/`, `docs/architecture/`, and `docs/examples/` to reference moved anchor paths.
- Created missing UI placeholder directories: `screens/`, `components/`, `api/`, `types/generated/`, `validation/generated/`, `validation/adapters/`, `hooks/`.

### Notes

- Phase 0 remains a skeleton validation phase.
- No runtime harness behavior was implemented.
- Placeholder files establish repository shape only.
