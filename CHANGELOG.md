---
truth_dimension: historical
authority_level: authoritative
mutation_path: changelog_entry
---

# CHANGELOG.md

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
