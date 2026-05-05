---
truth_dimension: historical
authority_level: authoritative
mutation_path: changelog_entry
---
# CHANGELOG.md

## v0.0.76 - 2026-05-05
**Status:** Phase 76 - UI/Rust Transport Boundary

### Added
- Added `docs/operations/ui-rust-transport-boundary-phase-76.md` documenting transport-shaped contract scope and non-authority posture.
- Added transport-shaped UI/Rust envelope types and fixtures for read-projection response and intent-preview request surfaces.

### Changed
- Updated `ui/src/api/projections.ts` with `UiRustTransport*` typed envelope contracts and capability model.
- Updated `ui/src/api/readModel.ts` with display-only envelope builder helpers.
- Updated `ui/src/api/fixtures.ts` with display-only transport envelope fixtures.
- Updated `checklists/current-phase.md` to Phase 76 procedural truth.
- Updated `CHANGELOG.md` with `v0.0.76`.

### Notes
- Phase 76 defines transport-shaped contracts only.
- No live transport, submission wiring, mutation power, execution, persistence wiring, or authorized action behavior was added.
- Phase 77 remains responsible for submission wiring.
- Phase 78 remains responsible for authorized action execution boundary.

## v0.0.75.1 - 2026-05-05
**Status:** Phase 75.1 - Out-of-Band Operations Audit Metadata and Terminology Correction

### Changed
- Corrected Phase 75 operations audit wording to avoid overstating CI wiring, runtime posture, or mutation scope.
- Preserved Phase 75 alignment/audit conclusions and Phase 76 planning boundary.

### Notes
- Phase 75.1 is an out-of-band maintenance fix before Phase 76.
- Phase 75.1 inspected Phase 75 operations audit frontmatter and left it unchanged because it matched the repository’s accepted convention.
- No Rust, TypeScript, script, workflow, roadmap, governance, architecture, dependency, runtime behavior, UI/Rust transport, release-candidate readiness claim, production-readiness claim, or public-usability claim was changed.

## v0.0.75 - 2026-05-05
**Status:** Phase 75 - Roadmap and Changelog Alignment Check + Script/Workflow Alignment Audit

### Added
- Added `docs/operations/repository-audit-phase-75.md` with advisory roadmap/changelog reconciliation, boundary enforcement coverage, script/workflow alignment audit findings, and Phase 80 preparation notes.

### Changed
- Updated `checklists/current-phase.md` to Phase 75 procedural truth including allowed surfaces, boundary rules, alignment/audit checklists, findings/deferred/validation tables, and required validation evidence logs.
- Updated `CHANGELOG.md` with `v0.0.75`.

### Notes
- Phase 75 is alignment, documentation hygiene, and automation audit only.
- Phase 75 does not implement UI/Rust transport, provider behavior changes, persistence behavior changes, recovery behavior changes, action execution wiring, or runtime behavior.
- Phase 75 does not approve release-candidate readiness, production readiness, or public usability.

## v0.0.74 - 2026-05-05
**Status:** Phase 74 - Application State Recovery Boundary

### Added
- Added `docs/operations/application-state-recovery-phase-74.md` documenting candidate-only recovery scope, verification relationship, and non-authority boundaries.

### Changed
- Updated `core/src/api/application_state.rs` with typed application recovery request/candidate/report/status/reason surfaces and deterministic `prepare_application_recovery_candidate(...)` mapping from verified ledger bytes.
- Updated `checklists/current-phase.md` to Phase 74 procedural truth.
- Updated `CHANGELOG.md` with `v0.0.74`.

### Notes
- Phase 74 prepares recovery candidates only from verified ledger bytes.
- Phase 74 does not replace `LocalApplicationState`, promote recovered state, repair replay, execute providers, record provider retries, or mutate/write ledger persistence.
- Phase 74 does not add UI/Rust transport, authorized action execution, CLI live recovery commands, or readiness/public-usability claims.

## v0.0.73 - 2026-05-05
**Status:** Phase 73 - Durable Ledger Persistence Lifecycle

### Added
- Added `docs/operations/ledger-persistence-lifecycle-phase-73.md` documenting Phase 73 durable ledger lifecycle scope, Phase 61/62 relationship, and non-recovery boundary.

### Changed
- Updated `core/src/api/persistence.rs` with typed ledger persistence lifecycle surfaces (prepare/write/verify/load), classification mapping, and deterministic tests.
- Updated `checklists/current-phase.md` to Phase 73 procedural truth.
- Updated `CHANGELOG.md` with `v0.0.73`.

### Notes
- Phase 73 persists and verifies typed ledger record bytes only.
- Phase 73 does not recover application state, and verified bytes are not automatically authoritative state.
- Phase 73 does not add provider output persistence, provider retry recording, replay repair, promotion, UI/Rust transport, action execution, or readiness/public-usability claims.

## v0.0.72 - 2026-05-05
**Status:** Phase 72 - Provider Failure, Timeout, and Retry Boundary

### Added
- Added `core/src/execution/provider_failure.rs` with deterministic provider failure kind classification, typed retry policy validation, typed eligibility/reason reporting, and non-authority/non-scheduling helper functions.
- Added `docs/operations/provider-failure-boundary-phase-72.md` documenting failure/retry scope, no-scheduling boundary, no-authority boundary, and Phase 73 relationship.

### Changed
- Updated `core/src/execution/mod.rs` to declare/re-export the focused provider failure module.
- Updated `checklists/current-phase.md` to Phase 72 procedural truth.
- Updated `CHANGELOG.md` with `v0.0.72`.

### Notes
- Phase 72 classifies provider failures and retry eligibility only.
- Phase 72 does not schedule or execute retries and does not perform async/network/provider calls.
- Phase 72 does not mutate authority, ledger, persistence, replay, or application state.
- Phase 72 does not claim release-candidate readiness, production readiness, or public usability.

## v0.0.71.5 - 2026-05-05
**Status:** Phase 71.5 - Provider Execution Structural Extraction

### Added
- Added `core/src/execution/provider_execution.rs` as focused ownership for provider execution adapter request/result/mode/status/reason types, execution function, and provider-execution tests.
- Added `docs/operations/provider-execution-extraction-phase-71-5.md` documenting out-of-band maintenance scope, lint impact, and deferred boundaries.

### Changed
- Updated `core/src/execution/mod.rs` to declare/re-export `provider_execution` while removing in-file provider execution implementation ownership.
- Updated `scripts/rust_boundary_lint.mjs` to remove warning-only grandfathering for ProviderExecution in `core/src/execution/mod.rs` after extraction.
- Updated `scripts/test_rust_boundary_lint.mjs` self-tests to align with post-extraction lint behavior.
- Updated `checklists/current-phase.md` to Phase 71.5 procedural truth.
- Updated `CHANGELOG.md` with `v0.0.71.5`.

### Notes
- Phase 71.5 is an out-of-band maintenance extraction before Phase 72.
- Phase 71.5 is structural extraction only and does not add provider failure/timeout/retry behavior.
- Phase 71.5 does not add real provider execution, async runtime, network I/O, persistence, ledger append, replay repair, UI/Rust transport, or CLI live command behavior.
- Phase 71.5 does not claim release-candidate readiness, production readiness, or public usability.

## v0.0.71.3 - 2026-05-05
**Status:** Phase 71.3 - Rust Boundary Enforcement Baseline

### Added
- Added `scripts/rust_boundary_lint.mjs` with dependency-free Rust boundary enforcement checks, IDE-friendly diagnostics, warning-only ProviderExecution grandfathering, and nonzero failure on blocking violations.
- Added `scripts/test_rust_boundary_lint.mjs` self-tests that validate pass/fail/warning behavior with temporary Rust fixtures.
- Added `docs/operations/rust-boundary-lint-baseline-phase-71-3.md` documenting scope, deferred AST work, and Phase 71.5 extraction relationship.

### Changed
- Updated `scripts/check.sh` to run Rust boundary lint self-tests before production Rust boundary lint.
- Updated `checklists/current-phase.md` to Phase 71.3 procedural truth.
- Updated `CHANGELOG.md` with `v0.0.71.3`.

### Notes
- Phase 71.3 is an out-of-band enforcement tooling insertion before Phase 71.5 and Phase 72.
- Phase 71.3 does not refactor Rust code and does not implement provider retry/failure logic.
- ProviderExecution in `core/src/execution/mod.rs` remains warning-only and deferred to out-of-band fix Phase 71.5.
- Phase 71.3 does not claim release-candidate readiness, production readiness, or public usability.

## v0.0.71 - 2026-05-05
**Status:** Phase 71 - Provider Execution Adapter Implementation

### Added
- Added `docs/operations/provider-execution-adapter-phase-71.md` documenting bounded provider execution adapter scope, transport relationship, trust model, deterministic local behavior, and deferred real-provider execution posture.

### Changed
- Updated `core/src/execution/mod.rs` with bounded provider execution adapter request/result/mode/status/reason types, deterministic local execution path, and Phase-69 transport envelope validation composition.
- Updated `core/src/main.rs` dry-run tests to assert dry-run does not execute the provider adapter path.
- Updated `checklists/current-phase.md` to Phase 71 procedural truth.
- Updated `CHANGELOG.md` with `v0.0.71`.

### Notes
- Provider execution output remains untrusted and non-authoritative candidate material.
- Phase 71 does not add real network provider calls, persistence, ledger append, promotion, replay repair, UI/Rust transport, or live provider CLI wiring.
- Phase 71 does not claim release-candidate readiness, production readiness, or public usability.

## v0.0.70 - 2026-05-05
**Status:** Phase 70 - Roadmap Documentation Realignment and Production Candidate Gap Audit

### Added
- Added `docs/operations/repository-audit-phase-70.md` with advisory documentation drift findings, roadmap-surface role separation, hardened-shell assessment, and production-candidate gap audit.

### Changed
- Updated `docs/roadmap/phase-map.md` into a compact planned phase index and confirmed planned sequence through Phase 80.
- Updated `docs/roadmap/phases.md` as active phase catalog with expanded planning details for Phases 70-80.
- Updated `docs/roadmap/sequencing.md` with production-path dependency rationale and explicit Phase 80 gap-audit posture.
- Updated `checklists/current-phase.md` to Phase 70 procedural truth with required validation/role/alignment/gap-audit checklists.
- Updated `CHANGELOG.md` with `v0.0.70`.

### Notes
- Phase 70 is documentation realignment, governance hygiene, and production-candidate gap audit only.
- No runtime behavior, provider execution wiring, UI/Rust transport wiring, or action execution behavior was implemented.
- `CHANGELOG.md` remains historical truth, roadmap files remain planned truth, and checklist remains procedural truth.
- Phase 70 does not claim release-candidate readiness, production readiness, or public usability.

## v0.0.69 - 2026-05-05
**Status:** Phase 69 - Async Provider Transport Boundary

### Added
- Added `core/src/api/provider_transport.rs` with typed provider transport envelope/cursor/report/status/reason/trust models, deterministic sequence validation, and non-authority/non-execution helper functions.
- Added `docs/operations/provider-transport-boundary-phase-69.md` documenting async-origin ingress scope, sequencing/trust invariants, and deferred runtime/network/provider execution work.

### Changed
- Updated `core/src/api/mod.rs` to declare/re-export the new provider transport module.
- Updated `core/src/main.rs` dry-run tests to assert dry-run does not invoke provider transport validation.
- Updated `checklists/current-phase.md` to Phase 69 procedural truth.
- Updated `CHANGELOG.md` with `v0.0.69`.

### Notes
- Phase 69 is async boundary without async runtime.
- Provider output remains untrusted.
- Sequencing validation is deterministic and non-mutating.
- Phase 69 does not add tokio, async/await, sockets, HTTP, provider execution, background tasks, channels, threads, UI/Rust transport, ledger mutation, or persistence.
- `core/src/execution/mod.rs` and `core/src/api/persistence.rs` were not expanded.

## v0.0.68 - 2026-05-04
**Status:** Phase 68 - Bounded Read Projection Slices

### Added
- Added `docs/operations/bounded-projection-phase-68.md` documenting Rust-owned bounded projection scope, read-only invariants, and deferred UI/transport mirrors.

### Changed
- Updated `core/src/api/read_projection.rs` with typed projection-slice surface/mode/status/reason/request/metadata/result models, deterministic request bounds, and in-memory bounded slice derivation helpers.
- Added deterministic bounded projection tests including bounds validation, determinism checks, non-mutation checks, multi-surface coverage, and non-authority/non-side-effect assertions.
- Updated `checklists/current-phase.md` to Phase 68 procedural truth.
- Updated `CHANGELOG.md` with `v0.0.68`.

### Notes
- Projection slicing is Rust-owned and read-only.
- Phase 68 does not add UI transport, UI caching/pagination, TypeScript mirrors, persistence reads, provider/model execution, replay repair, ledger append, or action execution.
- `core/src/execution/mod.rs` and `core/src/api/persistence.rs` were not expanded.

## v0.0.67 - 2026-05-04
**Status:** Phase 67 - Operator Intent Audit Record Boundary

### Added
- Added `core/src/api/intent_audit.rs` with typed operator-intent audit eligibility, deterministic record construction, and strict non-execution/non-persistence helpers.
- Added `docs/operations/intent-audit-boundary-phase-67.md` documenting proof-object scope and deferred physical append/persistence boundaries.

### Changed
- Updated `core/src/api/mod.rs` to declare/re-export the new intent-audit module.
- Updated `core/src/api/diagnostics.rs` with `OperatorIntentAudit` diagnostic family and `operator_intent_audit_reason_diagnostic(...)` mapping.
- Updated `core/src/main.rs` dry-run coverage to assert no operator-intent audit record construction surface.
- Updated `checklists/current-phase.md` to Phase 67 procedural truth.
- Updated `CHANGELOG.md` with `v0.0.67`.

### Notes
- Audit record construction is not audit record persistence.
- Phase 67 builds typed proof objects only.
- Phase 67 does not append ledgers/audit records, persist state, execute actions, or call provider/model flows.
- `core/src/execution/mod.rs` and `core/src/api/persistence.rs` were not expanded.
- Later phases remain responsible for physical append, persistence, verification, and recovery.

## v0.0.66 - 2026-05-04
**Status:** Phase 66 - Identity-Bound Operator Intent Authorization

### Added
- Added `core/src/api/authorization.rs` with typed identity/safety/target authorization request+decision models, deterministic fail-closed authorization checks, and non-execution helper coverage.
- Added `docs/operations/identity-authorization-phase-66.md` documenting scope, non-execution boundary, and structural-risk constraints.

### Changed
- Updated `core/src/api/mod.rs` to declare/re-export the new authorization module.
- Updated `core/src/api/diagnostics.rs` with `OperatorAuthorization` diagnostic family and `operator_authorization_reason_diagnostic(...)` mapping.
- Updated `checklists/current-phase.md` to Phase 66 procedural truth.
- Updated `CHANGELOG.md` with `v0.0.66`.

### Notes
- Authorization is not execution.
- `AuthorizedForFutureExecution` is metadata-only and keeps `execution_enabled=false`.
- Phase 66 does not execute operator actions, append ledger/audit records, persist state, call providers/models, or mutate execution flows.
- `core/src/execution/mod.rs` was not expanded.
- Phase 67 remains responsible for intent audit record boundary.

## v0.0.65 - 2026-05-04
**Status:** Phase 65 - Roadmap and Changelog Alignment Check

### Added
- Added `docs/operations/repository-audit-phase-65.md` with advisory alignment findings, Phase 61-64 boundary review, structural-risk assessment, and Phase 66 readiness decision.

### Changed
- Updated `checklists/current-phase.md` to Phase 65 procedural truth, alignment checklists, structural-risk checklist, findings/deferred tables, and validation log.
- Updated `CHANGELOG.md` with `v0.0.65`.

### Notes
- Phase 65 reconciles Phases 61-64 across roadmap/changelog/checklist/audit surfaces.
- Phase 65 assesses pre-Phase 66 structural risk and records whether Phase 66 can proceed directly, requires an inserted cleanup phase, or must restrict new code to a focused module.
- Outcome recorded: proceed with Phase 66 while restricting authorization implementation to a new focused module to avoid worsening oversized-file risk.
- No authorization implementation, refactor, runtime behavior, Rust behavior, UI behavior, provider execution, persistence behavior, API server, UI transport, script/workflow change, dependency change, release-candidate readiness claim, production-readiness claim, or public-usability claim was implemented.

## v0.0.64 - 2026-05-04
**Status:** Phase 64 - Rust/TypeScript Contract Synchronization Boundary

### Added
- Added `docs/operations/contract-sync-phase-64.md` with advisory Phase 64 boundary, mirror coverage, validation evidence, and non-readiness posture.

### Changed
- Updated `ui/src/api/projections.ts` to align TypeScript contract mirror types with Rust-owned diagnostics, persistence verification, intent submission semantics, and read projection display boundaries.
- Updated `ui/src/api/fixtures.ts` to exercise diagnostic and persisted-record verification mirror shapes in fixture/read-model data.
- Updated `checklists/current-phase.md` to Phase 64 procedural truth and required validation/checklist coverage.
- Updated `CHANGELOG.md` with `v0.0.64`.

### Notes
- Phase 64 aligns TypeScript mirror shapes with Rust-owned diagnostics, persistence verification, read projection, and intent submission semantics.
- Rust remains authoritative.
- TypeScript shapes are compile-time/UI display contracts only.
- No transport, generated bindings, runtime validation, fetch/API client, WebAssembly, FFI, UI submission, provider execution, persistence wiring, CLI behavior change, dependency change, release-candidate readiness claim, production-readiness claim, or public-usability claim was added.

## v0.0.63.5 - 2026-05-04
**Status:** Phase 63.5 - Procedural Evidence Closure

### Changed
- Updated `checklists/current-phase.md` to close Phase 63 procedural evidence drift and accurately record completed validation evidence.
- Updated `CHANGELOG.md` with `v0.0.63.5`.

### Notes
- Phase 63.5 is a out-of-band fix that corrects checklist evidence drift from Phase 63.
- No Rust code, runtime behavior, diagnostic behavior, UI behavior, roadmap, governance, architecture, script, workflow, schema, dependency, release-candidate readiness claim, production-readiness claim, or public-usability claim was changed.
- Execution-owned diagnostic mappings remain deferred.
- Phase 64 may begin after this procedural closure.

## v0.0.62 - 2026-05-04
**Status:** Phase 62 - Persistence Recovery and Corruption Detection

### Added
- Added `docs/operations/persistence-recovery-phase-62.md` with advisory scope, integrity metadata, read-only verification boundary, recovery model, deferred hardening, validation evidence, and non-readiness language.

### Changed
- Updated `core/src/api/persistence.rs` to add deterministic persisted-record envelope metadata, FNV-1a checksum helper, deterministic encode/decode helpers, and read-only path verification/recovery reporting.
- Updated `checklists/current-phase.md` to Phase 62 procedural truth, boundary rules, task/validation checklists, findings, and deferred tables.
- Updated `CHANGELOG.md` with `v0.0.62`.

### Notes
- Phase 62 adds deterministic persisted-record integrity metadata and read-only verification helpers.
- Persisted bytes are treated as untrusted until verified.
- The checksum model is deterministic corruption detection only, not cryptographic tamper-proofing.
- Recovery is descriptive and fail-closed only.
- Phase 62 does not auto-repair, delete, rewrite, roll back, restore, repair replay, mutate ledgers, serialize `LocalApplicationState`, decode application state, call providers, invoke models, wire UI/Rust transport, serve an API, change CLI behavior, add async/network/process behavior, add dependencies, or claim release-candidate/production/public usability.
- Directory sync after rename remains deferred unless implemented and tested.
- Phase 63 remains responsible for error-code family/context standardization.

## v0.0.61 - 2026-05-04
**Status:** Phase 61 - Data Durability and Atomic Persistence Implementation

### Added
- Added `docs/operations/persistence-atomicity-phase-61.md` with advisory scope, atomic-write boundary notes, deferred recovery posture, and explicit non-readiness language.

### Changed
- Updated `core/src/api/persistence.rs` to implement physical local persistence only through `execute_local_persistence_plan(...)` with typed-plan validation, non-empty payload enforcement, temp-path write, flush/sync, and fail-closed atomic rename behavior.
- Updated `checklists/current-phase.md` to Phase 61 procedural truth, boundaries, checklists, and validation log structure.
- Updated `CHANGELOG.md` with `v0.0.61`.

### Notes
- Phase 61 implements physical local persistence only through `execute_local_persistence_plan(...)`.
- Persistence remains explicit, typed, caller-supplied, opt-in, and validated through `LocalPersistencePlan`.
- The write path uses temp-path write, flush/sync, and rename semantics.
- `execute_local_persistence_plan(...)` is the only physical write boundary.
- Payload bytes are caller-supplied; Phase 61 does not serialize `LocalApplicationState` or infer payload meaning.
- Dry-run, local workflow, read projection, replay verification, provider, integration, and UI paths do not call persistence.
- Phase 62 remains responsible for recovery and corruption detection.
- No automatic persistence, hidden writes, parent directory creation, path canonicalization, serialization, `LocalApplicationState` serialization, provider execution, model invocation, replay repair, UI/Rust transport, API server, CLI behavior change, schema change, workflow change, script change, lint weakening, dependency change, roadmap change, governance change, architecture change, central error registry, release-candidate readiness claim, production-readiness claim, or public-usability claim was implemented.

## v0.0.60 - 2026-05-04
**Status:** Phase 60 - Roadmap and Changelog Alignment Check + Production-Path Expansion

### Added
- Added `docs/operations/repository-audit-phase-60.md` with advisory-only Phase 60 scope, roadmap/changelog truth alignment findings, Phase 51-59 boundary review, production-path risk expansion, and explicit non-readiness posture.

### Changed
- Updated `docs/roadmap/phase-map.md` to expand Phase 60-70 planning with durability-first sequencing, post-59 production-path risk boundaries, and preserved every-fifth-phase alignment checkpoints.
- Updated `checklists/current-phase.md` to Phase 60 procedural truth with required alignment, boundary review, risk, roadmap expansion, and validation tracking sections.
- Updated `CHANGELOG.md` with `v0.0.60`.
- Updated `docs/operations/repository-audit-phase-60.md` during Phase 60 audit finalization.

### Notes
- Phase 60 verifies roadmap/changelog alignment after Phases 51-59.
- Phase 60 expands the production-path roadmap around data durability, async determinism, identity-bound operator intent, bounded projections, error-code standardization, and Rust/TypeScript contract synchronization.
- Phase 61 starts with data durability and atomic persistence rather than real provider execution or UI/Rust transport.
- Phase 58 evidence collection did not approve release-candidate readiness.
- Phase 59 failure hardening did not implement production recovery.
- `CHANGELOG.md` remains the authoritative historical record.
- `docs/roadmap/phase-map.md` remains planned truth.
- No runtime harness behavior, Rust behavior, UI behavior, provider execution, real provider adapter, persistence implementation, physical write behavior, API server, CLI behavior change, UI/Rust transport, schema change, workflow change, script change, governance change, architecture change, lint-tooling change, dependency change, central error registry implementation, release-candidate readiness claim, production-readiness claim, or public-usability claim was implemented.

## v0.0.59 - 2026-05-04
**Status:** Phase 59 - Failure Injection and Recovery Hardening

### Added
- Added `docs/operations/failure-hardening-phase-59.md` with advisory-only scope, deterministic failure-hardening focus, production-path risk carry-forward, deferred contract-sync evidence, and explicit non-readiness posture.

### Changed
- Added deterministic failure-injection boundary tests in `core/src/api/persistence.rs` for validation priority, fail-closed stubs, hidden-write guard behavior, and payload-agnostic execution stubs.
- Added intent-ingress authority leakage tests in `core/src/api/operator_intent.rs` confirming non-execution behavior and pre-routing rejection boundaries.
- Added runtime safety/config hardening tests in `core/src/api/runtime_config.rs` for strict/preview defaults and unsafe/secret validation boundaries.
- Added application-state and local-workflow hardening tests in `core/src/api/application_state.rs` and `core/src/api/local_workflow.rs` for deterministic in-memory behavior and non-capability summary invariants.
- Updated `checklists/current-phase.md` to Phase 59 procedural truth and validation/failure-injection/risk tracking.
- Updated `CHANGELOG.md` with Phase 59 historical record.

### Notes
- Phase 59 adds deterministic failure-injection and recovery-hardening tests around existing in-memory boundaries.
- Phase 59 records production-path risks for async determinism, persistence atomicity, intent authority leakage, wide projections, error-code family/registry ambiguity, and Rust/TypeScript contract drift.
- The tests exercise existing fail-closed behavior and do not implement production recovery mechanisms.
- No runtime harness behavior, Rust behavior outside tests, UI behavior, provider execution, physical persistence, file IO, network/socket behavior, async runtime, process spawning, API server, UI/Rust transport, CLI behavior change, replay repair, operator action execution, schema change, workflow change, script change, lint weakening, dependency change, roadmap change, governance change, architecture change, central error registry, release-candidate readiness claim, or production-readiness claim was implemented.

## v0.0.58 - 2026-05-04

**Status:** Phase 58 - Release-Candidate Evidence Pass

### Added

- Added `docs/operations/release-candidate-evidence-phase-58.md` as an advisory evidence-collection report for current validation and deterministic workflow outputs.

### Changed

- Updated `checklists/current-phase.md` to Phase 58 procedural truth with evidence-only boundaries, required checklists, deferred-evidence tracking, and validation log.
- Updated `checklists/release.md` with Phase 58 conservative evidence rows and deferred capability rows without readiness approval language.
- Updated `CHANGELOG.md` with `v0.0.58`.

### Notes

- Phase 58 collects release-candidate evidence from current validation and dry-run/local workflow surfaces.
- Phase 58 is evidence collection only and does not approve release-candidate readiness.
- Evidence rows distinguish passed local validation from deferred functional capabilities.
- Real provider/model invocation, physical persistence, live UI/Rust transport, API server behavior, operator action execution, release packaging/installer, and failure-injection/recovery hardening remain deferred.
- No runtime harness behavior, Rust behavior, UI behavior, provider execution, persistence, physical write behavior, API server, CLI behavior change, UI transport, schema change, workflow change, script change, lint weakening, dependency change, roadmap change, governance change, architecture change, release-candidate readiness claim, or production-readiness claim was implemented.

## v0.0.57 - 2026-05-04

**Status:** Phase 57 - Packaging and Startup Boundary

### Added

- Added `docs/operations/local-startup-boundary-phase-57.md` as an advisory startup/packaging boundary reference for local validation and dry-run only commands.

### Changed

- Updated `checklists/current-phase.md` to Phase 57 procedural truth with startup boundary checks, API decomposition carry-forward checks, and validation/static-scan logs.
- Updated `README.md` with a concise local validation and dry-run command reference and explicit non-readiness posture.
- Updated `CHANGELOG.md` with `v0.0.57`.

### Notes

- Phase 57 defines the local startup/packaging boundary for current safe validation and dry-run commands.
- The startup boundary is documentation and command-surface clarification only.
- No real packaging, installer, release workflow, service, daemon, API server, UI transport, provider call, model invocation, persistence, physical write, workspace scanning, file IO, environment read, socket/HTTP behavior, async runtime, process spawning, replay repair, operator intent execution, schema change, workflow change, script change, lint weakening, dependency change, roadmap change, governance change, or architecture change was implemented.
- Phase 46 dry-run remains deterministic, in-memory, no-provider-call, and no-persistence.
- Phase 54 local harness workflow remains in-memory and is not wired into CLI live behavior.
- Phase 56/56.5 API decomposition remains intact.
- Release-candidate readiness is not claimed.
- Production readiness is not claimed.

## v0.0.56.5 - 2026-05-04

**Status:** Phase 56.5 - API Decomposition Validation Closure

### Changed

- Updated `checklists/current-phase.md` with normalized Phase 56.5 procedural truth and final-state validation/static-scan closure evidence.
- Updated `CHANGELOG.md` with `v0.0.56.5`.

### Notes

- Phase 56.5 is an out-of-band validation and correctness sweep before moving to higher-level functional code work.
- Phase 56.5 closes Phase 56 validation gaps after API module decomposition.
- Phase 56.5 normalizes current-phase procedural truth for the decomposed API state.
- Phase 56.5 completes the full validation/static-scan suite from the final decomposed state.
- `core/src/api/mod.rs` remains the compatibility and re-export surface.
- Behavior, public semantics, validation rules, error-code strings, helper behavior, and test expectations remain preserved.
- No runtime harness behavior, provider execution, persistence, physical write behavior, CLI behavior change, UI behavior, schema change, workflow change, script change, governance change, architecture change, roadmap change, dependency change, release-candidate readiness claim, or production-readiness claim was implemented.

## v0.0.56 - 2026-05-04

**Status:** Phase 56 - API Module Decomposition and Boundary Cleanup

### Added

- Added `core/src/api/operator_intent.rs` for operator intent API surfaces and helpers.
- Added `core/src/api/integration.rs` for integration boundary API surfaces and helpers.
- Added `core/src/api/runtime_config.rs` for local runtime config API surfaces and helpers.
- Added `core/src/api/read_projection.rs` for read projection API surfaces and helpers.
- Added `core/src/api/application_state.rs` for local application state API surfaces and helpers.
- Added `core/src/api/persistence.rs` for local persistence API surfaces and helpers.
- Added `core/src/api/local_workflow.rs` for local harness workflow API surfaces and helpers.

### Changed

- Updated `core/src/api/mod.rs` to remain the compatibility and re-export surface for `crate::api::*`.
- Updated `checklists/current-phase.md` to Phase 56 - API Module Decomposition and Boundary Cleanup.
- Updated `CHANGELOG.md` with `v0.0.56`.

### Notes

- Phase 56 decomposes `core/src/api/mod.rs` into focused API submodules.
- `core/src/api/mod.rs` remains the compatibility and re-export surface.
- Behavior, public semantics, validation rules, error-code strings, helper behavior, and test expectations are preserved.
- No runtime harness behavior, provider execution, persistence, physical write behavior, CLI behavior, UI behavior, schema change, workflow change, script change, governance change, architecture change, central error registry, dependency change, release-candidate readiness claim, or production-readiness claim was implemented.

## v0.0.55 - 2026-05-04

**Status:** Phase 55 - Roadmap and Changelog Alignment Check + API Decomposition Planning

### Added

- Added `docs/operations/repository-audit-phase-55.md` advisory report covering Phase 55 alignment findings, Phase 51-54 boundary review, API module density risk, and Phase 56 decomposition planning constraints.

### Changed

- Updated `docs/roadmap/phase-map.md` to insert Phase 56 as API Module Decomposition and Boundary Cleanup, shift later planned phases forward by one number, preserve every-fifth-phase alignment cadence, and require Phase 57+ functional continuation only after full decomposition validation.
- Updated `checklists/current-phase.md` to Phase 55 procedural scope, roadmap/changelog alignment checklist, Phase 51-54 boundary review checklist, API density/decomposition checklist, findings/deferred tables, and validation log.
- Updated `CHANGELOG.md` with `v0.0.55`.
- Updated `docs/operations/repository-audit-phase-55.md` during Phase 55 audit finalization.

### Notes

- Phase 55 verifies roadmap/changelog alignment after Phases 51-54.
- Phase 55 inserts Phase 56 as API Module Decomposition and Boundary Cleanup.
- Phase 55 shifts later planned phases forward to preserve the roadmap after the inserted structural phase.
- Phase 56 must preserve behavior, public semantics, validation rules, error codes, and test expectations.
- `CHANGELOG.md` remains the authoritative historical record.
- `docs/roadmap/phase-map.md` remains planned truth.
- Phase 54 remains an in-memory workflow composition baseline only.
- No API decomposition was implemented in Phase 55.
- No runtime harness behavior, UI behavior, provider execution, real provider adapter, persistence, physical write behavior, API server, CLI live command, schema change, workflow change, script change, governance change, architecture change, lint-tooling change, dependency change, release-candidate readiness claim, or production-readiness claim was implemented.

## v0.0.54 - 2026-05-04

**Status:** Phase 54 - End-to-End Local Harness Workflow

### Changed

- Updated `core/src/api/mod.rs` with typed in-memory local harness workflow request/result/status/reason/error surfaces, deterministic stub-provider composition, controlled-flow wiring, local application state/read projection derivation, and deterministic Phase 54 tests.
- Updated `core/src/main.rs` tests to assert CLI dry-run does not call the local harness workflow.
- Updated `checklists/current-phase.md` to Phase 54 scope and validation evidence.
- Updated `CHANGELOG.md` with `v0.0.54`.

### Notes

- Phase 54 adds an in-memory local harness workflow composition only.
- The workflow composes existing typed surfaces using the deterministic stub provider and controlled flow.
- Provider output remains untrusted and non-authoritative.
- The workflow does not call real providers, local models, cloud models, files, environment variables, sockets, HTTP, async runtime, persistence helpers, replay repair, operator intent execution, API server, CLI live command, UI transport, or UI behavior.
- The workflow does not physically persist or serialize state.
- Phase 46 dry-run remains no-provider-call and no-persistence and does not call the workflow.
- Phase 47 persistence remains validation/stub-only and is not physically implemented.
- UI intent previews remain non-functional and are not wired to this workflow.
- No schema change, workflow change, script change, lint weakening, or new dependency was implemented.
- Release-candidate readiness is not claimed.
- Production readiness is not claimed.

## v0.0.53 - 2026-05-04

**Status:** Phase 53 - UI Operator Intent Submission Boundary

### Changed

- Updated `ui/src/api/projections.ts` to add typed `IntentSubmissionUiProjection` metadata on operator intent previews.
- Updated `ui/src/api/readModel.ts` to keep intent submission, execution, and ledger-recording constants explicitly disabled and to provide pure submission-preview shaping.
- Updated `ui/src/api/fixtures.ts` to include deterministic submission-shaped metadata for approve, reject, retry, replay request, context rebuild request, and memory snapshot request preview intents.
- Updated `ui/src/components/IntentPreviewPanel.tsx` to display submission-shaped metadata and explicit non-submission/non-execution boundary text without controls.
- Updated `ui/src/screens/OverviewScreen.tsx` to state submission-shaped preview boundary and no Rust ingress/action execution behavior.
- Updated `checklists/current-phase.md` to Phase 53 scope and validation evidence.
- Updated `CHANGELOG.md` with `v0.0.53`.

### Notes

- Phase 53 adds typed UI operator intent submission-shaped data only.
- UI intent previews remain non-functional and display-only.
- `UI_INTENT_SUBMISSION_ENABLED`, `UI_INTENT_EXECUTION_ENABLED`, and `UI_INTENT_LEDGER_RECORDING_ENABLED` remain false.
- The UI does not call Rust ingress or `submit_operator_intent`.
- No action executes from UI previews.
- No fetch/API client, Rust bridge, UI mutation, UI event handlers, forms, buttons, anchor hrefs, local/session storage, sockets, timers, provider call, model invocation, persistence, physical write, ledger append, replay repair, context rebuild, memory snapshot execution, audit writing, CLI live command, schema change, workflow change, lint weakening, Rust behavior change, or new dependency was implemented.
- Release-candidate readiness is not claimed.
- Production readiness is not claimed.

## v0.0.52 - 2026-05-04

**Status:** Phase 52 - UI Live Read Projection Integration

### Changed

- Updated `ui/src/api/projections.ts` with typed `ApplicationUiProjection` and `RuntimeSafetyUiProjection` surfaces aligned to Rust-owned read-projection shape for non-authoritative UI consumption.
- Updated `ui/src/api/readModel.ts` with pure synchronous `buildUiReadModelFromApplicationProjection(...)`, fixture-or-supplied projection consumption, and explicit non-mutation/non-submission constants.
- Updated `ui/src/api/fixtures.ts` to provide an `ApplicationUiProjection`-shaped fixture that preserves untrusted provider/integration/output trust posture and closed runtime safety defaults.
- Updated UI shell/screens to read from the typed application projection boundary and display runtime safety, trust posture, and read-only summaries for lifecycle, run, ledger, replay, audit, context, memory, and output surfaces.
- Updated `checklists/current-phase.md` to Phase 52 checklist scope.
- Updated `CHANGELOG.md` with `v0.0.52`.

### Notes

- Phase 52 adds a typed UI consumption boundary for Rust-owned read projection-shaped data.
- The UI remains non-authoritative and side-effect-free.
- Fixture fallback remains explicit.
- Runtime safety posture is visible in UI read projections.
- Provider and integration outputs remain untrusted and non-authoritative in the UI.
- Raw provider/model output remains distinct from clean output.
- UI preview controls remain non-functional; operator intent submission is not wired in this phase.
- No fetch/API client, server integration, Rust bridge, UI mutation, UI event handlers, forms, buttons, anchor hrefs, local/session storage, sockets, timers, provider call, model invocation, persistence, physical write, ledger append, replay repair, context mutation, memory mutation, audit writing, CLI live command, schema change, workflow change, lint weakening, or new dependency was implemented.
- Release-candidate readiness is not claimed.
- Production readiness is not claimed.

## v0.0.51 - 2026-05-04

**Status:** Phase 51 - Rust-Owned Operator Intent Submission

### Changed

- Updated `core/src/api/mod.rs` with Rust-owned operator intent ingress submission types, deterministic validation/classification, routing-only submission reporting, explicit non-execution helper, and deterministic tests for ingress behavior and non-execution guarantees.
- Updated `core/src/main.rs` tests to assert CLI dry-run does not submit operator intents and does not wire UI preview controls.
- Updated `checklists/current-phase.md` to Phase 51 procedural scope and validation evidence.
- Updated `CHANGELOG.md` with `v0.0.51`.

### Notes

- Phase 51 adds a Rust-owned operator intent ingress boundary only.
- Operator intent submission validates, classifies, and routes caller-supplied requests but does not execute requested actions.
- Accepted intent routing does not imply approval execution, rejection execution, retry execution, replay execution, context rebuild, memory snapshot execution, output publication, ledger append, persistence, provider call, controlled-flow execution, replay repair, CLI live command, or UI wiring.
- UI preview controls remain non-functional and are not wired by this phase.
- Phase 46 dry-run remains no-provider-call and no-persistence.
- Phase 47 persistence remains validation/stub-only and is not physically implemented.
- No provider call, model invocation, file IO, environment read, socket/HTTP behavior, async behavior, provider authentication, controlled-flow execution, ledger append, replay repair, context mutation, memory mutation, audit writing, persistence, physical write, API server, CLI intent command, UI behavior, schema change, workflow change, lint weakening, central error registry, or new dependency was implemented.
- Release-candidate readiness is not claimed.
- Production readiness is not claimed.

## v0.0.50 - 2026-05-04

**Status:** Phase 50 - Roadmap and Changelog Alignment Check + Error-Code Registry Audit

### Added

- Added `docs/operations/repository-audit-phase-50.md` advisory report covering roadmap/changelog alignment, Phase 46-49 boundary review, Rust error-code string collision/ambiguity audit, and provider authority bridge risk findings.

### Changed

- Updated `checklists/current-phase.md` to Phase 50 procedural scope, alignment and boundary checklists, error-code audit checklist, provider authority bridge checklist, findings/deferred tables, and validation log.
- Updated `CHANGELOG.md` with `v0.0.50`.
- Updated `docs/operations/repository-audit-phase-50.md` during Phase 50 audit finalization.

### Notes

- Phase 50 verifies roadmap/changelog alignment after Phases 46-49.
- Phase 50 audits Rust error-code string mappings for collision, ambiguity, and future registry risk.
- Phase 50 audits provider authority bridge risk and confirms provider capability metadata remains descriptive only.
- `CHANGELOG.md` remains the authoritative historical record.
- `docs/roadmap/phase-map.md` remains planned truth.
- Phase 51 remains the next planned implementation phase.
- No central error registry, runtime harness behavior, UI behavior, provider execution, real provider adapter, persistence, physical write behavior, API server, CLI live command, schema change, workflow change, script change, governance change, architecture change, lint-tooling change, dependency change, release-candidate readiness claim, or production-readiness claim was implemented.

## v0.0.49 - 2026-05-04

**Status:** Phase 49 - Real Local Provider Adapter Boundary

### Changed

- Updated `core/src/execution/mod.rs` with typed local provider adapter configuration and capability metadata surfaces, deterministic validation, secret-marker rejection, and deterministic tests enforcing metadata-only/non-authoritative behavior.
- Updated `core/src/main.rs` tests to assert dry-run still states no provider/model call and does not use local provider config surfaces.
- Updated `checklists/current-phase.md` to Phase 49 procedural scope and validation evidence.
- Updated `CHANGELOG.md` with `v0.0.49`.

### Notes

- Phase 49 adds local provider adapter configuration and capability metadata only.
- LocalProcess and LocalHttp endpoint kinds are metadata only in this phase.
- Capability flags are descriptive only and do not grant authority.
- DeterministicStubProvider remains the only implemented invoking adapter.
- Phase 49 does not make AJENTIC capable of calling real local providers or local models.
- Phase 46 dry-run remains no-provider-call and no-persistence.
- Phase 47 persistence remains validation/stub-only and is not physically implemented.
- No real provider adapter, local model invocation, cloud model invocation, IDE connection, process spawning, file IO, file watching, environment-variable read, secrets storage, socket/HTTP behavior, async behavior, provider authentication, endpoint URL parsing, config-file loading, controlled-flow execution, ledger append, replay repair, audit writing, persistence, serialization, API server, CLI provider command, UI behavior, schema change, workflow change, lint weakening, or new dependency was implemented.
- Release-candidate readiness is not claimed.
- Production readiness is not claimed.

## v0.0.48 - 2026-05-04

**Status:** Phase 48 - Provider Adapter Trait and Deterministic Stub

### Changed

- Updated `core/src/execution/mod.rs` with a typed provider adapter trait, adapter invocation/result types, deterministic stub provider implementation, and deterministic unit tests that keep provider output untrusted and non-authoritative.
- Updated `core/src/main.rs` tests to assert dry-run does not use stub provider output and remains no-provider-call behavior.
- Updated `checklists/current-phase.md` to Phase 48 procedural scope and validation evidence.
- Updated `CHANGELOG.md` with `v0.0.48`.

### Notes

- Phase 48 adds a provider adapter trait and deterministic stub provider only.
- Stub provider output is deterministic, untrusted, and non-authoritative.
- The stub provider does not call real providers, local models, cloud models, files, environment variables, sockets, HTTP, async runtime, persistence helpers, controlled flow, ledger append, replay repair, audit writing, API server, CLI live command, or UI behavior.
- Phase 46 dry-run remains no-provider-call and no-persistence.
- Phase 47 persistence remains validation/stub-only and is not physically implemented.
- No real provider adapter, provider authentication, model invocation, file IO, socket/HTTP behavior, async behavior, controlled-flow execution, ledger append, persistence, serialization, API server, CLI provider command, UI behavior, schema change, workflow change, lint weakening, or new dependency was implemented.
- Release-candidate readiness is not claimed.
- Production readiness is not claimed.

## v0.0.47 - 2026-05-04

**Status:** Phase 47 - Local Persistence Boundary

### Changed

- Updated `core/src/api/mod.rs` with typed local persistence planning surfaces, deterministic validation, explicit atomicity plan requirements, opt-in execution boundary stub, and deterministic unit tests.
- Updated `core/src/main.rs` tests to assert dry-run no-persistence text remains unchanged and dry-run does not call the persistence execution helper.
- Updated `checklists/current-phase.md` to Phase 47 procedural scope and validation evidence.
- Updated `CHANGELOG.md` with `v0.0.47`.

### Notes

- Phase 47 adds typed local persistence planning and validation boundaries only.
- Persistence is explicit, typed, atomic by plan requirement, and opt-in.
- No read-only, dry-run, replay verification, projection, or UI path writes to disk.
- The Phase 47 execution helper returns `PhysicalWriteNotImplemented` for valid plans and does not perform physical writes.
- Paths are caller-supplied metadata only and are not read, canonicalized, created, scanned, or checked for existence.
- Phase 46 dry-run remains no-persistence.
- No automatic persistence, file reading, file writing, path canonicalization, directory scanning, environment read, secrets storage, socket/HTTP behavior, async behavior, provider call, model invocation, controlled-flow execution, replay repair, audit writing, `LocalApplicationState` serialization, API server, CLI live command, UI behavior, schema change, workflow change, lint weakening, or new dependency was implemented.
- Release-candidate readiness is not claimed.
- Production readiness is not claimed.

## v0.0.46 - 2026-05-04

**Status:** Phase 46 - Local CLI Dry-Run Entry

### Changed

- Updated `core/src/main.rs` with a minimal deterministic local CLI dry-run entry, safe default/no-arg behavior, safe unknown-command usage output, and unit tests.
- Updated `checklists/current-phase.md` to Phase 46 procedural scope and validation evidence.
- Updated `CHANGELOG.md` with `v0.0.46`.

### Notes

- Phase 46 adds a minimal local CLI dry-run entry only.
- Dry-run output is deterministic and in-memory.
- Dry-run output states that provider output remains untrusted, no files are read or written, no provider/model is called, no persistence occurs, and no release-candidate or production readiness is claimed.
- Phase 46 does not introduce provider/model calls, file IO, file watching, workspace scanning, environment-variable reads, socket/HTTP behavior, async behavior, provider authentication, durable persistence, replay repair, audit writing, API server, background service, process spawning, UI behavior, schema change, workflow change, lint weakening, or new dependency.
- Phase 47 remains the explicit persistence boundary.
- Release-candidate readiness is not claimed.
- Production readiness is not claimed.

## v0.0.45 - 2026-05-04

**Status:** Phase 45 - Roadmap and Changelog Alignment Check

### Added

- Added `docs/operations/repository-audit-phase-45.md` advisory report documenting Phase 45 alignment findings, boundary review, and carry-forward risks.

### Changed

- Updated `checklists/current-phase.md` to Phase 45 procedural scope, alignment/boundary checklists, findings, deferred items, and validation log.
- Updated `CHANGELOG.md` with `v0.0.45`.
- Updated `docs/operations/repository-audit-phase-45.md` during Phase 45 audit finalization.

### Notes

- Phase 45 verifies roadmap/changelog alignment after Phases 42-44.
- `CHANGELOG.md` remains the authoritative historical record.
- `docs/roadmap/phase-map.md` remains planned truth.
- Phase 46 remains the planned no-persistence local CLI dry-run entry.
- Phase 47 remains the planned explicit local persistence boundary.
- `LocalApplicationState` remains an in-memory typed container, not a persistence surface.
- `ApplicationContextMetadata` and `ApplicationMemoryMetadata` remain bounded summaries, not arbitrary payload stores.
- No runtime harness behavior, UI behavior, provider work, API server, CLI command, persistence, serialization, file IO, schema change, workflow change, script change, governance change, architecture change, lint-tooling change, dependency change, release-candidate readiness claim, or production-readiness claim was implemented.

## v0.0.44 - 2026-05-04

**Status:** Phase 44 - Local Application State Container

### Changed

- Updated `core/src/api/mod.rs` with in-memory typed local application state container surfaces, constructor validation for required state identity fields, side-effect-free projection derivation, and deterministic tests.
- Updated `checklists/current-phase.md` to Phase 44 procedural scope and required validation checklist.
- Updated `CHANGELOG.md` with `v0.0.44`.

### Notes

- Phase 44 adds an in-memory local application state container only.
- LocalApplicationState owns supplied typed state; ApplicationReadProjection remains a derived read-only snapshot.
- Reading or deriving projections is side-effect-free and does not execute controlled flow, call providers, verify or repair replay, persist or serialize state, read files, serve APIs, execute CLI commands, or wire UI behavior.
- Runtime safety posture remains visible in derived projections.
- Provider and integration outputs remain untrusted and non-authoritative.
- Raw provider output remains untrusted.
- No provider call, local model invocation, cloud model invocation, IDE connection, file IO, file watching, environment read, secrets storage, socket/HTTP behavior, async behavior, controlled-flow execution, ledger mutation, replay repair, persistence, serialization, API server, CLI command, UI behavior, schema change, workflow change, lint weakening, or new dependency was implemented.
- Release-candidate readiness is not claimed.
- Production readiness is not claimed.

## v0.0.43 - 2026-05-04

**Status:** Phase 43 - Rust Read Projection Boundary

### Changed

- Updated `core/src/api/mod.rs` with Rust-owned read projection structs, deterministic constructor validation, runtime safety posture projection, and deterministic read-only projection tests.
- Updated `checklists/current-phase.md` to Phase 43 procedural scope and required validation checklist.
- Updated `CHANGELOG.md` with `v0.0.43`.

### Notes

- Phase 43 adds Rust-owned read projection boundaries only.
- Runtime safety posture is visible in read projections.
- Read projections are built only from supplied typed inputs and do not execute controlled flow, call providers, verify or repair replay, persist or serialize state, read files, serve APIs, execute CLI commands, or wire UI behavior.
- Provider and integration outputs remain untrusted and non-authoritative in projections.
- Raw provider output remains untrusted.
- No provider call, local model invocation, cloud model invocation, IDE connection, file IO, file watching, environment read, secrets storage, socket/HTTP behavior, async behavior, controlled-flow execution, ledger mutation, replay repair, API server, CLI command, UI behavior, schema change, workflow change, lint weakening, or new dependency was implemented.
- Release-candidate readiness is not claimed.
- Production readiness is not claimed.

## v0.0.42 - 2026-05-04

**Status:** Phase 42 - Local Runtime Configuration Boundary

### Changed

- Updated `core/src/api/mod.rs` with typed local runtime configuration boundary surfaces, deterministic validation, deterministic secret-marker rejection, closed safety-default constructors, authority-bypass helper, and deterministic unit tests.
- Updated `checklists/current-phase.md` to Phase 42 procedural scope and required validation checklist.
- Updated `CHANGELOG.md` with `v0.0.42`.

### Notes

- Phase 42 adds typed local runtime configuration boundaries only.
- Workspace paths are caller-supplied metadata only and are not read, canonicalized, watched, created, or validated against the filesystem.
- Safety defaults remain closed for provider network, file IO, and UI mutation.
- No provider call, local model invocation, cloud model invocation, IDE connection, file IO, file watching, environment read, secrets storage, socket/HTTP behavior, async behavior, controlled-flow execution, ledger mutation, replay repair, API server, CLI command, UI behavior, schema change, workflow change, lint weakening, or new dependency was implemented.
- Release-candidate readiness is not claimed.
- Production readiness is not claimed.

## v0.0.41 - 2026-05-03

**Status:** Phase 41 - Functional Gap Audit and Roadmap Expansion

### Added

- Added `docs/operations/functional-gap-audit-phase-41.md` advisory report documenting implemented baseline, partial surfaces, missing surfaces, production-outcome gap analysis, and non-readiness boundaries.

### Changed

- Updated `docs/roadmap/phase-map.md` to expand planned phases from Phase 42 through Phase 60 into concrete incremental local-harness implementation and alignment checkpoints.
- Updated `checklists/current-phase.md` to Phase 41 procedural truth with cleanup guard, functional gap checklist, roadmap expansion checklist, findings/deferred/validation logs, and required validation commands.
- Updated `CHANGELOG.md` with `v0.0.41` historical entry.

### Notes

- Phase 41 audits the gap between the current implemented baseline and the production outcome.
- Phase 41 expands future roadmap phases toward a fully functional local harness.
- `CHANGELOG.md` remains the authoritative historical record.
- `docs/roadmap/phase-map.md` remains planned truth.
- AJENTIC is not yet a fully functional local harness.
- Release-candidate readiness is not claimed.
- Production readiness is not claimed.
- No runtime harness behavior, UI behavior, provider work, API server, CLI command, schema change, workflow change, governance change, architecture change, lint-tooling change, or dependency change was implemented.

## v0.0.40 - 2026-05-03

**Status:** Phase 40 - Roadmap/Changelog Reconciliation + AST Lint CI Alignment

### Changed

- Updated `docs/roadmap/phase-map.md` planned future sequence to explicitly account for Phase 38/39 maintenance deviations, define Phase 40 reconciliation scope, and confirm Phase 41 planned continuation.
- Updated `.github/workflows/ci.yml` to run `node scripts/test_lint_ui_boundaries.mjs` and `node scripts/lint_ui_boundaries.mjs` on pull requests through the UI job.
- Updated `checklists/release.md` to record AST-aware UI boundary lint local+CI enforcement evidence and completed Phase 40 reconciliation responsibility.
- Updated `checklists/current-phase.md` to Phase 40 procedural truth with roadmap/changelog alignment, AST lint CI alignment, findings, deferred items, and validation logs.
- Updated `CHANGELOG.md` with `v0.0.40`.

### Notes

- Phase 40 reconciles roadmap/changelog sequencing after the Phase 38/39 static-boundary lint maintenance deviation.
- Phase 40 verifies that CI reflects the Phase 38/39 AST-aware UI boundary lint baseline.
- `CHANGELOG.md` remains the authoritative historical record.
- `docs/roadmap/phase-map.md` remains planned truth.
- No runtime harness behavior, UI behavior, provider work, API server, CLI command, schema change, script behavior change, governance change, architecture change, dependency change, release-candidate readiness claim, or production-readiness claim was implemented.

## v0.0.39 - 2026-05-03

**Status:** Phase 39 - UI Boundary Lint Diagnostic Hardening

### Added

- Added `scripts/test_lint_ui_boundaries.mjs` with deterministic temporary-file self-tests for allowed static text/clean TSX cases and forbidden UI runtime/control patterns.
- Added `docs/operations/static-boundary-lint-hardening-phase-39.md` as an advisory report for diagnostic and self-test hardening scope.

### Changed

- Updated `scripts/lint_ui_boundaries.mjs` to emit IDE-friendly diagnostics in `path:line:column: message` format and support explicit target roots for test harness usage while preserving production default behavior.
- Updated `scripts/check.sh` to run `node scripts/test_lint_ui_boundaries.mjs` before direct UI boundary AST lint.
- Updated `checklists/release.md` static scan debt/evidence section to record UI boundary lint diagnostic and deterministic self-test coverage.
- Updated `checklists/current-phase.md` to Phase 39 procedural truth, scoped maintenance continuation notes, deterministic self-test evidence, and validation/deferred tracking.
- Updated `CHANGELOG.md` with `v0.0.39`.

### Notes

- Phase 39 hardens the AST-aware UI boundary lint baseline with IDE-friendly diagnostics and deterministic self-tests.
- Phase 39 continues the scoped static-boundary lint maintenance deviation ahead of Phase 40.
- Phase 40 remains the scheduled roadmap/changelog alignment checkpoint for reconciling future sequencing against historical truth.
- Ripgrep scans remain advisory for broader evidence review.
- Static scan precision debt is reduced further for UI runtime/control boundaries but remains unresolved for Rust, scripts, documentation/prohibition language, and broader cross-surface scans.
- No UI behavior, Rust behavior, provider work, API server, CLI command, schema change, workflow change, governance change, architecture change, roadmap rewrite, release-candidate readiness claim, or production-readiness claim was implemented.
- No new dependencies were added unless explicitly required and documented.

## v0.0.38 - 2026-05-03

**Status:** Phase 38 - Static Boundary Lint Baseline

### Added

- Added `scripts/lint_ui_boundaries.mjs` as a deterministic AST-aware UI boundary lint baseline using existing TypeScript parser tooling.
- Added `docs/operations/static-boundary-lint-baseline-phase-38.md` as an advisory scope/enforcement/debt report for this maintenance phase.

### Changed

- Updated `scripts/check.sh` to run `node scripts/lint_ui_boundaries.mjs` as part of local deterministic validation.
- Updated `checklists/release.md` static scan debt section to record partial debt reduction from AST-aware UI linting and remaining deferred precision work.
- Updated `checklists/current-phase.md` to Phase 38 procedural scope, lint coverage checklist, scoped maintenance deviation note, and validation/findings/deferred logs.
- Updated `CHANGELOG.md` with `v0.0.38`.

### Notes

- Phase 38 adds an AST-aware UI boundary lint baseline for forbidden runtime APIs and executable UI controls.
- Phase 38 intentionally advances static-boundary linting ahead of the prior roadmap sequence to reduce scan precision debt.
- Phase 40 remains the scheduled roadmap/changelog alignment checkpoint for reconciling future sequencing against historical truth.
- Ripgrep scans remain advisory for broader evidence review.
- Static scan precision debt is partially reduced but not fully resolved.
- No UI behavior, Rust behavior, provider work, API server, CLI command, schema change, workflow change, governance change, architecture change, roadmap rewrite, release-candidate readiness claim, or production-readiness claim was implemented.
- No new dependencies were added unless explicitly required and documented.

## v0.0.37 - 2026-05-03

**Status:** Phase 37 - Release-Candidate Evidence Collection Baseline

### Added

- Added `docs/operations/release-candidate-evidence-phase-37.md` with advisory baseline release-candidate evidence collection, validation reporting, static scan classification, blockers/deferred evidence tracking, and non-readiness statement.

### Changed

- Updated `checklists/release.md` release decision record placeholder with Phase 37 evidence rows only.
- Updated `checklists/current-phase.md` to Phase 37 procedural scope, evidence checklist, findings/deferred items tables, and validation log.
- Updated `CHANGELOG.md` with `v0.0.37`.

### Notes

- Phase 37 collects baseline release-candidate evidence only.
- Release-candidate readiness is not claimed.
- Production readiness is not claimed.
- Static scan debt remains evidence debt unless converted by a future scoped linting/tooling phase.
- `CHANGELOG.md` remains the authoritative historical record.
- `docs/roadmap/phase-map.md` remains planned truth.
- No runtime harness behavior, UI behavior, provider adapter work, API server, CLI command, schema change, workflow change, governance change, architecture change, lint-tooling change, or dependency change was implemented.

## v0.0.36 - 2026-05-03

**Status:** Phase 36 - Release Candidate Boundary

### Changed

- Updated `checklists/release.md` into a procedural release-candidate boundary and evidence checklist.
- Updated `checklists/current-phase.md` to Phase 36 procedural scope, boundary/evidence framing checklists, static-scan-debt tracking, findings/deferred/validation logs, and required commands.
- Updated `CHANGELOG.md` with `v0.0.36`.

### Notes

- Phase 36 defines release-candidate boundary evidence only.
- Release-candidate readiness is not claimed.
- Production readiness is not claimed.
- Static scan debt is recorded as future release-candidate evidence work; no lint tooling, workflow change, or dependency change was implemented.
- `CHANGELOG.md` remains the authoritative historical record.
- `docs/roadmap/phase-map.md` remains planned truth.
- No runtime harness behavior, UI behavior, provider adapter work, API server, CLI command, schema change, workflow change, governance change, architecture change, lint-tooling change, or dependency change was implemented.

## v0.0.35 - 2026-05-03

**Status:** Phase 35 - Roadmap and Changelog Alignment Check + Replay Verification Idempotency Audit

### Added

- Added `docs/operations/repository-audit-phase-35.md` with an advisory Phase 35 roadmap/changelog alignment and replay-verification idempotency audit summary.

### Changed

- Updated `core/src/execution/mod.rs` with minimal replay-verification idempotency tests proving repeated verification is deterministic for valid and invalid ledgers and does not mutate or repair ledger state.
- Updated `checklists/current-phase.md` to Phase 35 procedural scope, replay-verification idempotency checklist, findings/deferred/validation-log tables, and required validation/static scan commands.
- Updated `CHANGELOG.md` with `v0.0.35`.
- Updated `docs/operations/repository-audit-phase-35.md` during Phase 35 audit finalization.

### Notes

- Phase 35 verifies roadmap/changelog alignment before release-candidate planning.
- Phase 35 audits replay verification idempotency and read-only behavior.
- Replay verification must not append, remove, reorder, mutate, repair, persist, or write audit state.
- `CHANGELOG.md` remains the authoritative historical record.
- `docs/roadmap/phase-map.md` remains planned truth.
- No replay repair, runtime behavior, UI behavior, provider adapter, schema change, workflow change, governance change, architecture change, release-candidate readiness claim, or production capability claim was implemented.

## v0.0.34 - 2026-05-03

**Status:** Phase 34 - Production Hardening

### Changed

- Updated `core/src/execution/mod.rs` with additional negative-path and boundary-regression tests for provider trust/status invariants, controlled-flow fail-closed behavior, and replay/promotion transition-history verification.
- Updated `core/src/api/mod.rs` with additional negative-path integration boundary tests proving untrusted-only handling and separation of `operator_context_summary` from provider-facing `prompt_summary`.
- Updated `ui/src/api/readModel.ts` with compile-safe exported guards that assert the UI read model remains synchronous, fixture-backed, and request-preview-only.
- Updated `checklists/current-phase.md` to Phase 34 procedural scope and required validation/static scan checklist.
- Updated `CHANGELOG.md` with `v0.0.34`.

### Notes

- Phase 34 adds negative-path and boundary-regression hardening only.
- No new runtime capability, provider call, local model invocation, IDE connection, HTTP/socket/file IO, async behavior, provider authentication, ledger persistence, replay repair, audit export, API server, CLI command, executable UI control, schema change, workflow change, or dependency was implemented.
- Provider and integration outputs remain untrusted candidate material.
- Controlled flow remains deterministic, in-memory, and fail-closed.
- Raw provider output remains untrusted.

## v0.0.33 - 2026-05-03

**Status:** Phase 33 - Local LLM and IDE Integration Boundary

### Changed

- Updated `core/src/api/mod.rs` to add typed local LLM/IDE integration boundary request/output types, untrusted-only constructors, deterministic mapping helpers into existing ProviderRequest/ProviderOutput shapes, and deterministic unit tests proving non-authoritative boundary behavior.
- Updated `checklists/current-phase.md` to Phase 33 procedural scope and required validation/static scan checklist.
- Updated `CHANGELOG.md` with `v0.0.33`.

### Notes

- Phase 33 defines typed local LLM and IDE integration boundary surfaces only.
- Integration output is mapped into existing untrusted provider-output shapes and is never authoritative.
- No local model invocation, IDE connection, file IO, file watching, socket/HTTP behavior, async behavior, provider authentication, controlled-flow execution, ledger append, replay, audit, API server, CLI command, UI behavior, schema change, or new dependency was implemented.
- Raw local/IDE integration output remains untrusted.

## v0.0.32 - 2026-05-03

**Status:** Phase 32 - End-to-End Controlled Model Run Loop

### Changed

- Updated `core/src/execution/mod.rs` to add a deterministic in-memory controlled model run loop that composes existing typed provider boundary, policy, validation, execution, promotion, ledger append, replay verification, and audit projection helpers with fail-closed behavior and deterministic unit tests.
- Updated `checklists/current-phase.md` to Phase 32 procedural scope and required validation/static scan checklist.
- Updated `CHANGELOG.md` with `v0.0.32`.

### Notes

- Phase 32 connects existing typed in-memory surfaces into a deterministic local controlled flow.
- Provider output remains untrusted candidate material.
- The controlled flow does not call real providers, perform network IO, run async behavior, persist ledgers, serve APIs, execute CLI commands, touch UI behavior, or add new dependencies.
- Accepted clean output summaries are produced only after typed policy, validation, execution, promotion, ledger append, and replay verification gates pass.
- Raw provider output remains untrusted.

## v0.0.31 - 2026-05-03

**Status:** Phase 31 - Provider Adapter Boundary

### Changed

- Updated `core/src/execution/mod.rs` to define a typed Phase 31 provider adapter boundary with deterministic constructors, typed boundary errors, explicit untrusted provider output handling, and deterministic tests that enforce non-authoritative provider output behavior.
- Updated `checklists/current-phase.md` to Phase 31 procedural scope and required validation/static scan checklist.
- Updated `CHANGELOG.md` with `v0.0.31`.

### Notes

- Phase 31 defines a provider adapter boundary only.
- Provider output is represented as untrusted candidate material and is never authoritative.
- No real provider calls, HTTP behavior, async behavior, provider authentication, model invocation, validation execution, policy execution, ledger append, replay, audit, API server, CLI behavior, UI behavior, schema change, or new dependency was implemented.
- Raw provider output remains untrusted.

## v0.0.30 - 2026-05-03

**Status:** Phase 30 - Roadmap and Changelog Alignment Check + Script Boundary Audit

### Added

- Added `docs/operations/repository-audit-phase-30.md` with an advisory Phase 30 roadmap/changelog alignment and script boundary audit summary.

### Changed

- Updated `checklists/current-phase.md` to Phase 30 procedural scope, script/workflow audit checklists, findings classification, deferred items, and validation log.
- Updated `CHANGELOG.md` with `v0.0.30`.
- Updated `docs/operations/repository-audit-phase-30.md` during Phase 30 audit finalization.

### Notes

- Phase 30 verifies roadmap/changelog alignment before provider and integration work.
- Phase 30 audits Python and Bash scripts for deterministic validation behavior and boundary compliance.
- `CHANGELOG.md` remains the authoritative historical record.
- `docs/roadmap/phase-map.md` remains planned truth.
- Audit findings are advisory unless enforced by existing code, tests, schemas, scripts, or CI.
- No runtime harness behavior, UI behavior, provider adapter, schema change, workflow change, governance change, or architecture change was implemented.

## v0.0.29 - 2026-05-03

**Status:** Phase 29 - Responsive UI and Operator Usability Hardening

### Changed

- Updated `ui/src/app/AppShell.tsx` to improve human-readable projection summary hierarchy and explicit text-visible trust/authority wording.
- Updated `ui/src/components/IntentPreviewPanel.tsx` and `ui/src/components/StatusPill.tsx` to strengthen readable request-preview, status, and authority text surfaces without introducing behavior.
- Updated `ui/src/styles/tokens.css` and `ui/src/styles/layout.css` to improve responsive shell spacing, card/list readability, wrapping behavior, and focus-visible support for desktop/tablet/mobile layouts.
- Updated `checklists/current-phase.md` to Phase 29 procedural scope.
- Updated `CHANGELOG.md` with `v0.0.29`.

### Notes

- Phase 29 improves responsive layout, readability, hierarchy, focus styling, and operator usability for existing fixture-backed UI surfaces.
- The UI remains non-authoritative, fixture-backed, read-only/request-preview only.
- No live API integration, async behavior, runtime authority, executable operator controls, provider calls, mutation behavior, Rust behavior, schema change, or new dependency was implemented.
- Raw model output remains untrusted.

## v0.0.28 - 2026-05-03

**Status:** Phase 28 - Operator Intent Controls UI

### Added

- Added `ui/src/components/IntentPreviewPanel.tsx` as a display-only typed operator intent request preview component.

### Changed

- Updated `ui/src/api/projections.ts` to add `OperatorIntentPreviewProjection` and include `operatorIntentPreviews` in `UiReadModel`.
- Updated `ui/src/api/fixtures.ts` to add static fixture-backed operator intent preview entries for approve, reject, retry, replay request, context rebuild request, and memory snapshot request.
- Updated `ui/src/api/readModel.ts` to preserve synchronous fixture-backed `getUiReadModel()` behavior.
- Updated `ui/src/screens/OverviewScreen.tsx` to render a clearly labeled request-only operator intent preview section.
- Updated `checklists/current-phase.md` to Phase 28 procedural scope.
- Updated `CHANGELOG.md` with `v0.0.28`.

### Notes

- Phase 28 adds typed operator intent preview controls as request-only UI surfaces.
- Intent previews are static, fixture-backed, non-authoritative, and non-executing in this phase.
- No live API integration, async behavior, runtime authority, provider calls, executable operator intent submission, state mutation, ledger mutation, replay repair, audit export, output publish, Rust behavior, schema change, or new dependency was implemented.
- Intent is a request. Rust decides.

## v0.0.27 - 2026-05-03

**Status:** Phase 27 - Ledger, Replay, Audit, and Clean Output UI

### Added

- Added `ui/src/components/TimelineList.tsx` as a display-only readable timeline/detail list component for projection rows.

### Changed

- Updated `ui/src/api/projections.ts` to add ledger timeline, replay detail, audit detail, and clean output projection types and include them in `UiReadModel`.
- Updated `ui/src/api/fixtures.ts` to add static fixture-backed ledger timeline entries, replay detail projection data, audit detail projection data, and clean output projection data.
- Updated `ui/src/api/readModel.ts` to preserve synchronous fixture-backed `getUiReadModel()` behavior.
- Updated `ui/src/screens/LedgerScreen.tsx` to render ledger projection metadata and readable ledger timeline entry details as a read-only display.
- Updated `ui/src/screens/ReplayScreen.tsx` to render replay projection metadata and replay detail reconstruction status as a read-only display.
- Updated `ui/src/screens/AuditScreen.tsx` to render audit projection summary and readable audit detail projection entries as a read-only display.
- Updated `ui/src/screens/OutputScreen.tsx` to render clean output projection metadata and explicit raw-output untrusted display text.
- Updated `checklists/current-phase.md` to Phase 27 procedural scope.
- Updated `CHANGELOG.md` with `v0.0.27`.

### Notes

- Phase 27 displays ledger, replay, audit, and clean output projections as read-only UI surfaces.
- The UI remains non-authoritative and fixture-backed in this phase.
- No live API integration, async behavior, runtime authority, operator intent controls, provider calls, ledger mutation, audit export, replay repair, output apply/publish, Rust behavior, schema change, or new dependency was implemented.
- Raw model output remains untrusted.

## v0.0.26 - 2026-05-03

**Status:** Phase 26 - Policy, Validation, and Execution Decision UI

### Added

- Added `ui/src/components/DecisionSummary.tsx` as a display-only readable decision detail projection component.

### Changed

- Updated `ui/src/api/projections.ts` to add `DecisionDetailProjection` and include `policyDecisions`, `validationDecisions`, and `executionDecisions` in `UiReadModel`.
- Updated `ui/src/api/fixtures.ts` to add static fixture-backed policy, validation, and execution decision detail projection data.
- Updated `ui/src/api/readModel.ts` to preserve synchronous fixture-backed `getUiReadModel()` behavior with expanded projection fields.
- Updated `ui/src/screens/PolicyScreen.tsx` to render readable policy decision projection details as a read-only surface.
- Updated `ui/src/screens/ValidationScreen.tsx` to render readable validation and execution decision projection details as read-only display sections.
- Updated `checklists/current-phase.md` to Phase 26 procedural scope.
- Updated `CHANGELOG.md` with `v0.0.26`.

### Notes

- Phase 26 displays policy, validation, and execution decision projections as read-only UI surfaces.
- The UI remains non-authoritative and fixture-backed in this phase.
- No live API integration, async behavior, runtime authority, operator intent controls, provider calls, policy evaluation in UI, validation evaluation in UI, state mutation, ledger mutation, replay repair, Rust behavior, schema change, or new dependency was implemented.
- Raw model output remains untrusted.

## v0.0.25 - 2026-05-03

**Status:** Phase 25 - Roadmap and Changelog Alignment Check + Repository Audit

### Added

- Added `docs/operations/repository-audit-phase-25.md` with an advisory Phase 25 repository audit summary.

### Changed

- Updated `checklists/current-phase.md` to Phase 25 procedural scope, validation log, and audit findings tracking.
- Updated `CHANGELOG.md` with `v0.0.25`.
- Updated `docs/operations/repository-audit-phase-25.md` during Phase 25 audit execution finalization.

### Notes

- Phase 25 verifies roadmap/changelog alignment and audits repository correctness, coding standards, truth-dimension placement, and phase creep.
- `CHANGELOG.md` remains the authoritative historical record.
- `docs/roadmap/phase-map.md` remains planned truth.
- Audit findings are advisory unless enforced by existing code, tests, schemas, or CI.
- No runtime harness behavior, UI behavior, provider adapter, schema change, workflow change, governance change, or architecture change was implemented.

## v0.0.24 - 2026-05-03

**Status:** Phase 24 - Context Packet and Memory Inspection UI

### Added

- Added `ui/src/components/ProjectionList.tsx` as a read-only readable list component for inspection projection rows.

### Changed

- Updated `ui/src/api/projections.ts` to add `ContextSliceProjection`, `MemoryEntryProjection`, and preview list fields on context and memory projections.
- Updated `ui/src/api/fixtures.ts` to add static fixture-backed context slice previews and memory entry previews for read-only inspection display.
- Updated `ui/src/api/readModel.ts` to preserve synchronous fixture-backed `getUiReadModel()` behavior.
- Updated `ui/src/screens/ContextScreen.tsx` to display context packet metadata and context slice preview inspection details as readable text.
- Updated `ui/src/screens/MemoryScreen.tsx` to display memory snapshot metadata and memory entry preview inspection details as readable text.
- Updated `checklists/current-phase.md` to Phase 24 procedural scope.
- Updated `CHANGELOG.md` with `v0.0.24`.

### Notes

- Phase 24 displays context packet and memory snapshot projections as read-only UI inspection surfaces.
- The UI remains non-authoritative and fixture-backed in this phase.
- No live API integration, async behavior, runtime authority, operator intent controls, provider calls, context mutation, memory mutation, state mutation, ledger mutation, replay repair, Rust behavior, schema change, or new dependency was implemented.
- Raw model output remains untrusted.

## v0.0.23 - 2026-05-03

**Status:** Phase 23 - State and Run Overview UI

### Added

- Added `ui/src/components/StatusPill.tsx` as a display-only status text component for read-only projection status labels.

### Changed

- Updated `ui/src/api/projections.ts` to add `RunOverviewProjection` and include `run` in `UiReadModel`.
- Updated `ui/src/api/fixtures.ts` to add static fixture-backed `run` overview projection data.
- Updated `ui/src/api/readModel.ts` to keep synchronous fixture-backed `getUiReadModel()` behavior with the extended read model shape.
- Updated `ui/src/app/AppShell.tsx` to include run overview metadata in shell projection summaries.
- Updated `ui/src/screens/OverviewScreen.tsx` to display run overview, lifecycle, decisions, replay readiness, clean output availability, and read-only summary messaging.
- Updated `ui/src/screens/StateScreen.tsx` to display lifecycle projection details and run linkage in a read-only state view.
- Updated `checklists/current-phase.md` to Phase 23 procedural scope.
- Updated `CHANGELOG.md` with `v0.0.23`.

### Notes

- Phase 23 displays state and run overview projections as read-only UI surfaces.
- The UI remains non-authoritative and fixture-backed in this phase.
- No live API integration, runtime authority, operator intent controls, provider calls, state mutation, ledger mutation, replay repair, Rust behavior, schema change, or new dependency was implemented.
- Raw model output remains untrusted.

## v0.0.22 - 2026-05-03

**Status:** Phase 22 - Read-Only API Projection Surface for UI

### Added

- Added `ui/src/api/projections.ts` with read-only UI projection type shapes.
- Added `ui/src/api/fixtures.ts` with static fixture-backed read-model projection data.
- Added `ui/src/api/readModel.ts` with synchronous fixture accessor `getUiReadModel()`.

### Changed

- Updated `ui/src/app/AppShell.tsx` to display read-only projection summary lines from `getUiReadModel()`.
- Updated `ui/src/screens/OverviewScreen.tsx` with read-only projection overview summary text.
- Updated `ui/src/screens/StateScreen.tsx` with lifecycle projection display text.
- Updated `ui/src/screens/ContextScreen.tsx` with context projection display text.
- Updated `ui/src/screens/MemoryScreen.tsx` with memory projection display text.
- Updated `ui/src/screens/LedgerScreen.tsx` with ledger projection display text.
- Updated `ui/src/screens/ReplayScreen.tsx` with replay projection display text.
- Updated `ui/src/screens/AuditScreen.tsx` with audit projection summary display text.
- Updated `ui/src/screens/OutputScreen.tsx` with output trust projection display text.
- Updated `checklists/current-phase.md` to Phase 22 procedural scope.
- Updated `CHANGELOG.md` with `v0.0.22`.

### Notes

- Phase 22 adds read-only UI projection shapes and static fixture-backed read-model access only.
- The UI read model is non-authoritative and does not redefine contract truth.
- No HTTP/API fetching, async behavior, provider calls, operator intent controls, state mutation, ledger mutation, replay repair, Rust behavior, schema change, or new dependency was implemented.
- Raw model output remains untrusted.

## v0.0.21 - 2026-05-03

**Status:** Phase 21 - Browser UI Shell Baseline

### Added

- Added `ui/src/screens/OverviewScreen.tsx`.
- Added `ui/src/screens/StateScreen.tsx`.
- Added `ui/src/screens/ContextScreen.tsx`.
- Added `ui/src/screens/MemoryScreen.tsx`.
- Added `ui/src/screens/PolicyScreen.tsx`.
- Added `ui/src/screens/ValidationScreen.tsx`.
- Added `ui/src/screens/LedgerScreen.tsx`.
- Added `ui/src/screens/ReplayScreen.tsx`.
- Added `ui/src/screens/AuditScreen.tsx`.
- Added `ui/src/screens/OutputScreen.tsx`.
- Added `ui/src/components/SectionCard.tsx`.

### Changed

- Updated `ui/src/app/AppShell.tsx` with a read-only browser UI shell baseline layout surface.
- Updated `ui/src/app/navigation.ts` with typed primary navigation display metadata.
- Updated `ui/src/app/routes.tsx` with typed route display metadata.
- Updated `ui/src/styles/tokens.css` with shared design token variables.
- Updated `ui/src/styles/layout.css` with responsive shell layout styling.
- Updated `checklists/current-phase.md` to Phase 21 procedural scope.
- Updated `CHANGELOG.md` with `v0.0.21`.

### Notes

- Phase 21 adds browser UI shell and layout scaffolding only.
- UI surfaces are read-only, non-authoritative, and intent-free in this phase.
- No API integration, runtime authority, provider adapter, operator intent controls, state mutation, ledger mutation, replay repair, or Rust behavior was implemented.
- No new dependencies were added.

## v0.0.20 - 2026-05-03

**Status:** Phase 20 - Roadmap Alignment Check and UI Entry Reset

### Changed

- Updated `docs/roadmap/phase-map.md` to add a clear planned-sequence divider that resumes future implementation planning at Phase 20 from the post-`v0.0.19` state.
- Updated `checklists/current-phase.md` for Phase 20 procedural scope and validation tracking.
- Updated `CHANGELOG.md` with `v0.0.20`.

### Notes

- Phase 20 verifies roadmap/changelog alignment after Phase 19.5.
- `CHANGELOG.md` remains the authoritative historical record.
- `docs/roadmap/phase-map.md` remains planned truth.
- Future implementation planning resumes at Phase 20 from the post-v0.0.19 repository state.
- Phase 21 remains the planned browser UI shell entry point.
- No runtime harness behavior, UI behavior, provider adapter, schema change, workflow change, governance change, or architecture change was implemented.

## v0.0.19.5 - 2026-05-03

**Status:** Phase 19.5 - Roadmap and Changelog Reconciliation

### Changed

- Updated `docs/roadmap/phase-map.md` to reconcile planned future sequencing with the conservative post-`v0.0.19` repository state while preserving roadmap planned-truth boundaries.
- Updated `checklists/current-phase.md` for Phase 19.5 roadmap/changelog reconciliation scope.
- Updated `CHANGELOG.md` with `v0.0.19.5`.

### Notes

- Phase 19.5 reconciles planned roadmap sequencing with the more granular historical implementation path recorded in `CHANGELOG.md`.
- `CHANGELOG.md` remains the authoritative historical record.
- `docs/roadmap/phase-map.md` remains planned truth and must not record completed implementation status.
- Future roadmap/changelog alignment checks are scheduled every 5 phases.
- No runtime harness behavior, UI behavior, provider adapter, schema change, workflow change, or governance change was implemented.

## v0.0.19 - 2026-05-02

**Status:** Phase 19 - Operator Intent Routing Baseline

### Changed

- Updated `core/src/api/mod.rs` to add typed operator intent routing surfaces (`OperatorRoute`, `OperatorRouteResult`, `OperatorRouteError`), deterministic route classification by `OperatorIntentType`, `OperatorIntent::new(...)`, and deterministic routing unit tests.
- Updated `checklists/current-phase.md` for Phase 19 operator intent routing scope.
- Updated `CHANGELOG.md` with `v0.0.19`.

### Notes

- Phase 19 routes caller-supplied operator intents into typed route.
- route_operator_intent(...) maps a caller-supplied OperatorIntent to a typed route classification.
- Routing does not execute actions, mutate state, append ledgers, call providers, serve APIs, parse CLI commands, or touch UI behavior.
- Route classification depends only on `OperatorIntentType` and non-empty reason validation.
- No new dependencies were added.

## v0.0.18 - 2026-05-02

**Status:** Phase 18 - Promotion Replay Verification Baseline

### Changed

- Updated `core/src/execution/mod.rs` to add read-only promotion replay verification types, stable reason code mapping, `verify_promotion_replay(...)`, and deterministic unit tests.
- Updated `checklists/current-phase.md` for Phase 18 promotion replay verification scope.
- Updated `CHANGELOG.md` with `v0.0.18`.

### Notes

- Phase 18 verifies promotion replay from an existing in-memory ledger only.
- Promotion replay verification uses the existing lifecycle transition table; the valid replay path to `PromotedTier1` requires three state-transition events (`Created -> Evaluating -> Passed -> PromotedTier1`), so the successful verification test asserts final revision `3`.
- Verification uses existing replay readiness and reconstruction functions.
- Verification does not build promotion records, append ledger events, mutate HarnessState, persist ledgers, repair replay, audit, call providers, serve APIs, execute CLI commands, or touch UI behavior.
- No new dependencies were added.

## v0.0.17 - 2026-05-02

**Status:** Phase 17 - Promotion Ledger Append Baseline

### Changed

- Updated `core/src/execution/mod.rs` to add deterministic promotion-record append delegation with stable append error code mapping and unit tests.
- Updated `checklists/current-phase.md` for Phase 17 promotion ledger append scope.
- Updated `CHANGELOG.md` with `v0.0.17`.

### Notes

- Phase 17 appends an already-built promotion record to an in-memory ledger only.
- append_promotion_record(...) appends an already-built PromotionRecord to a caller-supplied in-memory Ledger through Ledger::append.
- It does not decide, build, apply, persist, replay, audit, or execute promotion.
- `Ledger::append` remains responsible for revision sequencing.
- The append helper does not construct promotion records, classify promotion decisions, mutate `HarnessState`, apply lifecycle transitions, persist ledgers, replay, audit, call providers, serve APIs, execute CLI commands, or touch UI behavior.
- No new dependencies were added.

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

## v0.0.63 - 2026-05-04
Phase 63 - Error-Code Family and Reporting Standardization

### Added
- Added API diagnostic family/context types and reporting helpers via `core/src/api/diagnostics.rs`.
- Added advisory operations note `docs/operations/error-code-standardization-phase-63.md`.

### Changed
- Updated `core/src/api/mod.rs` to expose diagnostics module.
- Updated API modules with preserve-code diagnostic mapping tests.
- Updated `checklists/current-phase.md` for Phase 63 scope and validation tracking.
- Updated `CHANGELOG.md` with Phase 63 entry.

Notes:
- Phase 63 adds diagnostic family/context reporting around existing stable `code()` strings.
- Existing `code()` values remain unchanged and are not globally uniquified.
- Duplicate code strings remain allowed when scoped by diagnostic family/context.
- `diagnostic_key(...)` is a reporting key, not a replacement for `code()`.
- Phase 63 does not add UI/CLI reporting integration.
- Phase 63 does not change validation order, runtime behavior, persistence behavior, recovery behavior, provider execution, replay behavior, ledger behavior, CLI behavior, UI behavior, schema behavior, workflow behavior, scripts, dependencies, roadmap, governance, or architecture.
- Release-candidate readiness is not claimed.
- Production readiness is not claimed.
- Public usability is not claimed.
