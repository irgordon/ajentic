---
truth_dimension: historical
authority_level: authoritative
mutation_path: changelog_entry
---
# CHANGELOG

Historical truth surface for the active development range.

Archived historical ranges:
- docs/changelog/CHANGELOG-0001-0055.md (v0.0.1 through v0.0.55; includes legacy v0.0.0 bootstrap entry to prevent historical omission)
- docs/changelog/CHANGELOG-0056-0104.md (v0.0.56 through v0.0.104)

Archive guarantees:
- Historical entries are partitioned without changing their recorded wording, timestamps, ordering within each deterministic archive extraction, headings, or semantic interpretation.
- Archived entries are not duplicated in this active changelog.
- The active changelog begins with v0.0.104.5 and later entries only.
- CHANGELOG surfaces remain historical truth.

## v0.0.111 - 2026-05-08
**Status:** Phase 111 - Narrow Persistence Authority Activation Boundary

### Added
- Add the narrow Rust-validated decision-evidence append path.
- Add tests for accepted decision-evidence append and prohibited authority categories.
- Add the Phase 111 operations report.

### Changed
- Update checklists/current-phase.md to Phase 111 procedural truth.
- Update CHANGELOG.md with v0.0.111.

### Notes
- Narrow persistence authority only.
- Rust-validated decision-evidence append only.
- No broad persistence authority.
- No provider-output authority.
- No workflow-completion authority.
- No sandbox-success authority.
- No UI/transport persistence authority.
- No replay repair.
- No recovery promotion.
- No action execution.
- No provider trust.
- No provider output promotion.
- No readiness approval.
- No Production Candidate approval.
- No release-candidate approval.
- No public-usability approval.
- No production-human-use approval.
- No Phase 112 implementation.

## v0.0.110 - 2026-05-08
**Status:** Phase 110 - Roadmap and Changelog Alignment Check

### Added
- Added the Phase 110 roadmap/changelog alignment report.

### Changed
- Updated roadmap planned-truth surfaces to reconcile Phase 106-109 outcomes and correct stale current-block/immediate-gate language.
- Added an archive annotation outside historical entries in docs/changelog/CHANGELOG-0056-0104.md for the known v0.0.63 ordering anomaly.
- Updated checklists/current-phase.md to Phase 110 procedural truth.
- Updated CHANGELOG.md with v0.0.110.

### Notes
- Alignment/check only.
- No runtime behavior.
- No new capability.
- No Rust source changes.
- No TypeScript source changes.
- No test changes.
- No schema changes.
- No governance doc changes.
- No persistence authority.
- No provider trust.
- No provider output promotion.
- No replay repair.
- No recovery promotion.
- No action execution.
- No readiness approval.
- No Production Candidate approval.
- No release-candidate approval.
- No public-usability approval.
- No production-human-use approval.
- No Phase 111 implementation.
- Historical entries were not rewritten.
- Phase 120 remains a planned gate, not a guaranteed final endpoint.

## v0.0.109.5 - 2026-05-08
**Status:** Out-of-Band Repository Governance Audit - post-Phase 109 audit/alignment only

### Added
- Add the audit report.
- Update current checklist to the audit procedural truth.

### Changed
- Update CHANGELOG.md with v0.0.109.5.

### Notes
- Audit/alignment only.
- No runtime behavior.
- No new capability.
- No Rust source changes.
- No TypeScript source changes.
- No test changes.
- No schema changes.
- No governance doc changes unless explicitly justified.
- No persistence authority.
- No provider trust.
- No provider output promotion.
- No replay repair.
- No recovery promotion.
- No action execution.
- No readiness approval.
- No Production Candidate approval.
- No release-candidate approval.
- No public-usability approval.
- No production-human-use approval.
- No Phase 110 implementation.

## v0.0.109 - 2026-05-08
**Status:** Phase 109 - Durable Persistence Authority Decision Gate

### Added
- Added deterministic durable persistence authority decision evidence structures, proposed persistence-boundary classification, negative-authority evidence, decision statuses, constraint sets, prohibited persistence categories, and reason codes.
- Added behavioral and adversarial coverage for descriptive-only decision evidence, sandbox-success non-authority, workflow-completion non-authority, provider-output non-authority, prohibited persistence categories, malformed evidence, hostile/noise authority payloads, and no Phase 109 persistence activation.
- Added Phase 109 operations documentation.

### Changed
- Updated the active phase checklist to Phase 109 procedural truth.

### Notes
- Persistence-boundary decision evidence only.
- No durable persistence authority.
- No provider-output authority.
- No replay repair.
- No recovery promotion.
- No action execution.
- No readiness approval.
- No Production Candidate approval.
- No release-candidate approval.
- No public-usability approval.
- No production-human-use approval.
- No Phase 110 implementation.

## v0.0.108 - 2026-05-07
**Status:** Phase 108 - Provider Timeout and Resource Limit Boundary

### Added
- Added deterministic sandbox timeout/resource enforcement with typed declared limit snapshots, observed usage summaries, enforcement statuses, decisions, deterministic termination reasons, and descriptive-only sandbox limit evidence.
- Added behavioral and adversarial coverage for timeout exhaustion, resource-limit enforcement, deterministic truncation, repeated execution determinism, retry prohibition, limit-escalation prohibition, no-authority posture, and provider-output-untrusted posture.
- Added Phase 108 operations documentation.

### Changed
- Updated provider sandbox reports and UI API projections to expose descriptive timeout/resource evidence without granting authority.
- Updated the active phase checklist to Phase 108 procedural truth.

### Notes
- Deterministic timeout/resource enforcement only.
- Resource-limit evidence remains descriptive-only.
- Provider output remains untrusted candidate data.
- No remote/cloud provider execution.
- No provider output promotion.
- No persistence authority.
- No replay repair.
- No recovery promotion.
- No action execution.
- No readiness approval.
- No Production Candidate approval.
- No release-candidate approval.
- No public-usability approval.
- No production-human-use approval.
- No Phase 109 implementation.

## v0.0.107 - 2026-05-07
**Status:** Phase 107 - Provider Execution Sandbox Boundary

### Added
- Added bounded deterministic local stub provider execution through a typed Rust execution request/report sandbox.
- Added deterministic execution metadata, bounded input/output summaries, sandbox posture indicators, untrusted-output posture, disabled remote/provider-network posture, no-promotion posture, no-persistence posture, and no-action/replay/recovery posture.
- Added behavioral and adversarial coverage for deterministic local stub execution, invalid provider configuration rejection, unsafe request rejection, remote/cloud rejection, fallback rejection, auto-selection rejection, provider-output injection, oversized input, malformed requests, and no-authority guarantees.
- Added Phase 107 operations documentation.

### Changed
- Updated the active phase checklist to Phase 107 procedural truth.

### Notes
- Bounded deterministic local stub provider execution only.
- No remote provider execution.
- No external API calls.
- No cloud model inference.
- Provider output remains untrusted candidate data.
- No provider output promotion.
- No persistence authority.
- No durable append authority.
- No export authority.
- No replay repair.
- No recovery promotion.
- No action execution.
- No readiness approval.
- No Production Candidate approval.
- No release-candidate approval.
- No public-usability approval.
- No production-human-use approval.
- No Phase 108 implementation.

## v0.0.106 - 2026-05-07
**Status:** Phase 106 - Provider Configuration Contract

### Added
- Deterministic provider configuration contracts, parser, validation reason codes, and disabled/untrusted/non-ready posture reports.
- Behavioral and adversarial coverage for malformed, duplicate, unsupported, authority-bearing, execution-enabled, transport-enabled, trust-enabled, auto-selection, fallback, and invalid resource provider configuration payloads.
- Phase 106 operations report and current-phase checklist evidence.

### Changed
- UI API projections now include descriptive provider configuration validation types while remaining non-authoritative.
- Current-phase procedural truth now tracks Phase 106 provider configuration contract work.

### Notes
- Deterministic provider configuration contracts only.
- No provider execution.
- No inference execution.
- No persistence authority.
- No replay repair.
- No recovery promotion.
- No action execution.
- No readiness approval.
- No Production Candidate approval.
- No release-candidate approval.
- No public-usability approval.
- No production-human-use approval.
- No Phase 107 implementation.

## v0.0.105 - 2026-05-07
**Status:** Phase 105 - Transport Abuse Hardening for Live Local Bridge

### Added
- Added `docs/operations/transport-abuse-hardening-phase-105.md` as the Phase 105 operations report.
- Added Rust and UI adversarial transport tests for malformed, truncated, oversized, replay-shaped, duplicate-identifier, authority-bearing, unsupported, invalid-state, invalid enum, invalid typed-field, non-local, and hostile/noise inputs.

### Changed
- Hardened the bounded local UI-to-Rust transport parser and request handling with deterministic fail-closed rejection for hostile transport input.
- Updated `checklists/current-phase.md` to Phase 105 procedural truth.
- Updated `scripts/lint_ui_boundaries.mjs` to resolve TypeScript from the active Node global module path before the legacy fallback so required validation can run.

### Notes
- Phase 105 is transport abuse hardening only.
- Phase 105 includes no transport capability expansion.
- Phase 105 adds no provider execution.
- Phase 105 adds no persistence authority.
- Phase 105 adds no durable append authority.
- Phase 105 adds no export authority.
- Phase 105 adds no replay repair.
- Phase 105 adds no recovery promotion.
- Phase 105 adds no action execution.
- Phase 105 grants no readiness approval.
- Phase 105 grants no Production Candidate approval.
- Phase 105 grants no release-candidate approval.
- Phase 105 grants no public-usability approval.
- Phase 105 grants no production-human-use approval.
- Phase 105 includes no Phase 106 implementation.

## v0.0.104.5 - 2026-05-07
**Status:** Phase 104.5 - Historical Truth Partitioning

### Added
- Added deterministic archived changelog ranges under `docs/changelog/`.
- Added `docs/operations/historical-truth-partitioning-phase-104-5.md` as the Phase 104.5 operations report.

### Changed
- Changed `CHANGELOG.md` into the rolling active historical-truth surface beginning with `v0.0.104.5` and later entries only.
- Updated `checklists/current-phase.md` to Phase 104.5 procedural truth.

### Notes
- Phase 104.5 performs deterministic changelog archival partitioning only.
- Phase 104.5 adds no runtime behavior.
- Phase 104.5 adds no transport behavior.
- Phase 104.5 adds no provider execution.
- Phase 104.5 adds no persistence authority.
- Phase 104.5 adds no replay repair.
- Phase 104.5 adds no recovery promotion.
- Phase 104.5 adds no action execution.
- Phase 104.5 grants no readiness approval.
- Phase 104.5 grants no Production Candidate approval.
- Phase 104.5 grants no release-candidate approval.
- Phase 104.5 grants no public-usability approval.
- Phase 104.5 grants no production-human-use approval.
- Phase 104.5 does not implement Phase 105.
