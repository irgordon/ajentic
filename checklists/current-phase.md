---
truth_dimension: procedural
authority_level: authoritative
mutation_path: checklist_revision
---
# Current Phase Checklist - Phase 105 Transport Abuse Hardening for Live Local Bridge

## Phase name
Phase 105 - Transport Abuse Hardening for Live Local Bridge

## Phase goal
Harden the bounded live local UI-to-Rust bridge against malformed, spoofed, replay-shaped, oversized, authority-bearing, unsupported, hostile, or state-corrupting transport input without broadening capability.

## Working-tree hygiene gate
- [x] Ran `git status --short` before edits.
- [x] Classified uncommitted files before edits.
- [x] Confirmed no generated artifact drift required cleanup before edits.
- [x] Read `CHANGELOG.md` and `docs/changelog/*` before modification.
- [x] Read roadmap, current checklist, README, AGENTS, and required operations docs before modification.
- [x] Inspected implementation surfaces only as evidence.

## Allowed surfaces
- [x] `core/src/api/**`
- [x] `ui/src/api/**`
- [x] `tests/**`
- [x] `docs/operations/transport-abuse-hardening-phase-105.md`
- [x] `checklists/current-phase.md`
- [x] `CHANGELOG.md`
- [x] Conditional validation support: `scripts/lint_ui_boundaries.mjs` updated to resolve the active Node global TypeScript path so required validation can run.

## Boundary rules
- [x] Phase 105 is transport abuse hardening only.
- [x] Phase 105 does not broaden transport capability.
- [x] Phase 105 does not add provider execution.
- [x] Phase 105 does not add persistence authority.
- [x] Phase 105 does not add durable append authority.
- [x] Phase 105 does not add export authority.
- [x] Phase 105 does not add replay repair.
- [x] Phase 105 does not add recovery promotion.
- [x] Phase 105 does not add action execution.
- [x] Phase 105 does not add deployment automation.
- [x] Phase 105 does not add remote/public transport.
- [x] Phase 105 does not approve readiness, Production Candidate status, release-candidate readiness, production readiness, public usability, or production human use.
- [x] Phase 105 does not implement Phase 106.

## Task checklist
- [x] Harden local transport parser and request handling against hostile input.
- [x] Preserve deterministic fail-closed behavior.
- [x] Preserve local-only transport posture.
- [x] Add adversarial transport abuse tests.
- [x] Add deterministic rejection tests.
- [x] Create `docs/operations/transport-abuse-hardening-phase-105.md`.
- [x] Update this checklist to Phase 105 procedural truth.
- [x] Add `CHANGELOG.md` v0.0.105.
- [x] Decide Phase 106 may begin only as the next planned provider configuration contract phase.
- [x] Avoid Phase 106 implementation.

## Validation checklist
- [x] Run `./scripts/check.sh`.
- [x] Run `cargo test --manifest-path core/Cargo.toml --all-targets`.
- [x] Run `cargo test --manifest-path core/Cargo.toml golden --all-targets`.
- [x] Run `cargo test --manifest-path core/Cargo.toml adversarial --all-targets`.
- [x] Run `cargo test --manifest-path core/Cargo.toml phase_105 --all-targets`.
- [x] Run `cd ui && npm run test:api`.
- [x] Run Rust/UI boundary lint commands.
- [x] Run `cd ui && npm run typecheck && npm run lint && npm run build`.
- [x] Run deterministic rejection validation.
- [x] Run locality validation.
- [x] Run no-authority scan.
- [x] Run readiness scan.
- [x] Run source/workflow guard.

## Transport-hardening checklist
- [x] Bridge remains local-only.
- [x] Bridge remains bounded.
- [x] Bridge remains deterministic.
- [x] Bridge remains non-authoritative.
- [x] Bridge remains fail-closed.

## Malformed-input checklist
- [x] Malformed payload rejection covered.
- [x] Truncated payload rejection covered.
- [x] Empty payload rejection covered.
- [x] Malformed structured payload rejection covered.
- [x] Malformed typed-field rejection covered.
- [x] Invalid enum rejection covered.

## Oversized-payload checklist
- [x] Oversized payload rejection occurs before parsing.
- [x] Oversized hostile payload ordering is deterministic.

## Replay-shaped-payload checklist
- [x] Replay-shaped metadata rejection covered.
- [x] Replay repair remains rejected.
- [x] Replay verification-only posture preserved.

## Duplicate-request checklist
- [x] Duplicate `request_id` field rejection covered.
- [x] Duplicate identifier rejection does not create deduplication or replay repair authority.

## Authority-escalation checklist
- [x] Authority-bearing fields rejected.
- [x] Authority escalation operations rejected.
- [x] Credentials/admin claims rejected.

## Unsupported-operation checklist
- [x] Unsupported typed operation rejected.
- [x] Unknown payload operation enum rejected deterministically.

## Invalid-state checklist
- [x] Invalid workflow state rejected.
- [x] Invalid review state rejected.
- [x] Invalid escalation state rejected.
- [x] No malformed state auto-correction added.

## Hostile-payload checklist
- [x] Hostile/noise payload rejected.
- [x] Null-containing payload rejected.
- [x] Structured injection-shaped payload rejected.

## Deterministic-rejection checklist
- [x] Identical hostile payloads produce identical rejection results.
- [x] Reason codes remain deterministic.
- [x] Rejection ordering remains deterministic.
- [x] No partial authority path activation occurs.
- [x] No silent fallback path occurs.

## Non-authority checklist
- [x] No authoritative state mutation.
- [x] No durable ledger append.
- [x] No audit append.
- [x] No export write.
- [x] No silent recovery or repair.

## Provider-isolation checklist
- [x] Provider execution attempts rejected.
- [x] Provider execution remains disabled.
- [x] Provider output remains untrusted.

## Persistence-isolation checklist
- [x] Persistence attempts rejected.
- [x] Persistence remains disabled.
- [x] Durable append/export authority remains disabled.

## Recovery/replay-isolation checklist
- [x] Replay repair attempts rejected.
- [x] Recovery promotion attempts rejected.
- [x] No replay repair or recovery promotion authority added.

## Action-execution-isolation checklist
- [x] Action execution attempts rejected.
- [x] Action execution remains disabled.

## Adversarial-test checklist
- [x] Malformed payloads covered.
- [x] Replay-shaped payloads covered.
- [x] Duplicate request identifiers covered.
- [x] Malformed enum values covered.
- [x] Oversized payloads covered.
- [x] Authority escalation attempts covered.
- [x] Provider, persistence, replay repair, recovery promotion, and action execution attempts covered.
- [x] Hostile/noise payloads covered.
- [x] Invalid transport metadata and locality claims covered.
- [x] Invalid review/workflow/escalation values covered.
- [x] Malformed structured request bodies covered.

## Behavioral-test checklist
- [x] UI API behavior tests cover Phase 105 hostile payloads.
- [x] Rust integration behavior tests cover Phase 105 hostile payloads.
- [x] Deterministic failure ordering is covered.

## Roadmap/changelog truth checklist
- [x] Confirmed Phase 105 title/scope from roadmap files.
- [x] Roadmap remains planned truth.
- [x] CHANGELOG surfaces remain historical truth.
- [x] Phase 105 changelog entry reflects final behavior.

## Phase 106 gate checklist
- [x] Phase 106 may begin next only as the planned provider configuration contract phase.
- [x] Phase 106 is not implemented.
- [x] Phase 106 is not provider execution.

## Production Candidate status checklist
- [x] Production Candidate status is not approved.
- [x] Phase 105 does not approve Production Candidate status.

## Release-candidate/public-use status checklist
- [x] Release-candidate readiness is not approved.
- [x] Public usability is not approved.
- [x] Production readiness is not approved.
- [x] Production human use is not approved.

## Zero-drift checklist
- [x] Staged files are limited to allowed surfaces.
- [x] No generated artifacts remain staged.
- [x] No package or lockfile drift.
- [x] No roadmap/governance/README/AGENTS drift.
- [ ] No scripts drift.
- [x] Script drift is limited to validation support for the required UI boundary lint gate under the active Node runtime.
- [x] Source/workflow guard checked.

## Findings table
| Finding | Evidence | Status |
| --- | --- | --- |
| Initial working tree had no uncommitted files. | `git status --short` before edits produced no paths. | Confirmed |
| No generated artifact cleanup was required before edits. | Initial status showed no generated artifact paths. | Confirmed |
| Phase 105 title/scope is planned. | Roadmap lists Phase 105 as transport abuse hardening for live local bridge. | Confirmed |
| Transport hardening rejects malformed, oversized, replay-shaped, duplicate identifier, authority-bearing, invalid enum, invalid typed-field, invalid state, non-local, unsupported, and hostile inputs. | Rust and UI tests plus implementation changes. | Confirmed |
| No new authority behavior was added. | Authority flags remain disabled and no-authority scan was reviewed. | Confirmed |
| UI boundary lint needed runtime-compatible TypeScript resolution. | `scripts/lint_ui_boundaries.mjs` now checks the active Node global module path before the legacy fallback. | Confirmed validation-support drift |

## Deferred items table
| Item | Reason |
| --- | --- |
| Provider execution | Deferred to later provider execution phases; not Phase 105. |
| Persistence authority | Deferred to future persistence authority decision/activation phases. |
| Durable append authority | Outside Phase 105 boundary. |
| Export authority | Outside Phase 105 boundary. |
| Replay repair | Outside Phase 105 boundary. |
| Recovery promotion | Outside Phase 105 boundary. |
| Action execution | Outside Phase 105 boundary. |
| Deployment automation | Outside Phase 105 boundary. |
| Remote/public transport | Outside Phase 105 boundary. |
| Readiness approvals | Not approved by Phase 105. |
| Phase 106 implementation | Deferred to Phase 106. |

## Validation log table
| Command | Result | Notes |
| --- | --- | --- |
| `git status --short` | Passed | Initial status had no paths. |
| `cargo test --manifest-path core/Cargo.toml phase_105 --all-targets` | Passed | Phase 105 Rust tests passed. |
| `cargo test --manifest-path core/Cargo.toml adversarial --all-targets` | Passed | Adversarial Rust tests passed. |
| `cd ui && npm run test:api` | Passed | UI API behavior tests passed. |
| `./scripts/check.sh` | Passed | Full validation passed after final edits. |
| Full required validation command set | Passed | See final response for exact commands. |
