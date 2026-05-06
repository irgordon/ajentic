---
truth_dimension: procedural
authority_level: authoritative
mutation_path: checklist_revision
---

# Current Phase Checklist

## phase name
Phase 91 - Transport Abuse and Submission Spoofing Hardening

## phase goal
Add negative-path hardening for UI/Rust transport contracts, submission-shaped requests, spoofed operators, and disabled execution flags while keeping the UI local, non-live, non-executing, non-persistent, and non-authoritative.

## working-tree hygiene gate
- [x] `git status --short` reviewed before edits; initial working tree was clean.
- [x] Uncommitted files classified before edits: none.
- [x] Prior generated artifact drift checked before edits; no tracked `core/target/.rustc_info.json`, UI build artifacts, test temp files, export temp files, or other generated artifact drift appeared in initial git status.
- [x] Required roadmap/history/checklist/operations inputs read or inspected.
- [x] UI contract surfaces inspected: `ui/src/api/projections.ts`, `ui/src/api/readModel.ts`, and `ui/src/api/fixtures.ts`.
- [x] Existing UI tests/config inspected; no UI unit test harness was found.
- [x] Validation tooling inspected: `scripts/check.sh`, `scripts/lint_ui_boundaries.mjs`, and `scripts/test_lint_ui_boundaries.mjs`.
- [x] Phase 91 title/scope confirmed from roadmap files.

## allowed surfaces
- [x] `ui/src/api/projections.ts`
- [x] `ui/src/api/readModel.ts`
- [x] `ui/src/api/fixtures.ts`
- [x] `checklists/current-phase.md`
- [x] `CHANGELOG.md`
- [x] `docs/operations/transport-abuse-hardening-phase-91.md`
- [x] `checklists/release.md` not changed because evidence posture did not change.
- [x] `ui/package.json` not changed because no package/test script change was required or allowed.
- [x] UI AST lint scripts not changed because no Phase 91 lint behavior gap was found.

## boundary rules
- [x] Phase 91 is hardening only.
- [x] No live transport added.
- [x] No Rust bridge added.
- [x] UI submissions were not made executable.
- [x] No runtime authority added.
- [x] No `fetch`, `XMLHttpRequest`, `WebSocket`, `EventSource`, beacon, FFI, wasm, native bridge, server endpoint, event handler, button, form, persistence, ledger/audit append, recovery acceptance, replay repair, provider/model execution, action execution, or authority mutation behavior added.
- [x] User-supplied capability and authority flags are rejected before preview acceptance.
- [x] Phase 92 remains responsible for authorization/audit/action mismatch hardening.
- [x] Public usability, production readiness, Production Candidate approval, and release-candidate readiness are not claimed.

## task checklist
- [x] Updated this checklist to Phase 91 procedural truth.
- [x] Created `docs/operations/transport-abuse-hardening-phase-91.md` with required advisory frontmatter and sections.
- [x] Added local TypeScript contract result/input/reason types for the submission boundary.
- [x] Added `buildUiSubmissionBoundaryResult` to reject malformed/spoofed submissions before bridge eligibility.
- [x] Added static malformed, risky text, spoofed capability, and accepted preview fixtures.
- [x] Preserved `UI_READ_MODEL_MUTATION_CAPABLE=false`.
- [x] Preserved `UI_INTENT_EXECUTION_ENABLED=false`.
- [x] Preserved `UI_INTENT_LEDGER_RECORDING_ENABLED=false`.
- [x] Kept Phase 77 submission shaping local and non-executing.
- [x] Added `CHANGELOG.md` v0.0.91.
- [x] Roadmap files were not changed; Phase 95 remains the next alignment checkpoint.

## validation checklist
- [x] `./scripts/check.sh`
- [x] `cargo test --manifest-path core/Cargo.toml --all-targets`
- [x] `node scripts/test_rust_boundary_lint.mjs`
- [x] `node scripts/rust_boundary_lint.mjs`
- [x] `node scripts/test_lint_ui_boundaries.mjs`
- [x] `node scripts/lint_ui_boundaries.mjs`
- [x] `cd ui && npm run typecheck && npm run lint && npm run build`
- [x] `cargo run --manifest-path core/Cargo.toml -- dry-run`
- [x] Transport hardening scan run.
- [x] No-live-transport scan run.
- [x] UI authority scan run.
- [x] Source guard run.
- [x] Readiness scan run.
- [x] Lint wiring scan run.
- [x] Final `git status --short` reviewed before commit.
- [x] `git diff --name-only --cached` reviewed before commit.

## malformed submission checklist
- [x] Non-object and array inputs reject as `malformed_submission_rejected`.
- [x] Empty operator ID rejects as `empty_operator_id`.
- [x] Empty target ID rejects as `empty_target_id`.
- [x] Empty intent kind rejects as `empty_intent_kind`.
- [x] Unknown intent kind rejects as `unknown_intent_kind`.
- [x] Risky authority-escalation text rejects as `authority_escalation_text_rejected`.
- [x] Rejected results keep all live/execution/persistence/authority flags false.
- [x] Rejected results keep `transportEligible=false`.

## spoofed flag checklist
- [x] `liveTransportEnabled` spoofing rejects as `live_transport_flag_spoof_rejected`.
- [x] `executionEnabled` spoofing rejects as `execution_flag_spoof_rejected`.
- [x] `persistenceEnabled` spoofing rejects as `persistence_flag_spoof_rejected`.
- [x] `ledgerRecordingEnabled` spoofing rejects as `ledger_recording_flag_spoof_rejected`.
- [x] `auditAppendEnabled` spoofing rejects as `audit_append_flag_spoof_rejected`.
- [x] `providerExecutionEnabled` spoofing rejects as `provider_execution_flag_spoof_rejected`.
- [x] `replayRepairEnabled` spoofing rejects as `replay_repair_flag_spoof_rejected`.
- [x] `mutatesAuthority` and prior UI mutation spelling reject as `authority_mutation_flag_spoof_rejected`.
- [x] User-supplied capability flags are rejected, not trusted.

## bridge non-call checklist
- [x] Boundary result always reports `liveTransportCalled=false`.
- [x] Accepted preview result remains `transportEligible=false`.
- [x] Malformed/spoofed results remain `transportEligible=false`.
- [x] No helper calls or simulates live Rust transport.
- [x] No sendable transport envelope is constructed for malformed/spoofed submissions.

## UI test coverage or test-harness gap checklist
- [x] Existing UI test/config pattern inspected.
- [x] No UI unit test harness found.
- [x] No dependencies added.
- [x] No package or lock files changed.
- [x] Typecheck/lint/build are not claimed as behavioral test coverage.
- [x] Missing UI unit test harness documented as a Phase 91 finding and deferred to Phase 95 or an out-of-band testing phase.

## AST/boundary lint parity checklist
- [x] Existing UI AST lint self-tests passed.
- [x] Existing UI AST lint production scan passed.
- [x] No Phase 91 UI AST lint behavior change was required.
- [x] `rg` scans used only as discovery/evidence, not blocking enforcement.
- [x] Blocking enforcement remains `scripts/check.sh`, Rust boundary lint, UI AST lint, compiler/type checks, and tests.

## test fidelity checklist
- [x] Test names requirement could not be applied to UI unit tests because no UI unit test harness exists.
- [x] No fake UI test command was added.
- [x] Existing full validation gate was run after final edits.
- [x] Cross-boundary behavior remains not publicly reachable because Phase 91 adds no live transport or bridge.
- [x] Behavioral UI harness gap is explicitly documented.

## zero-drift checklist
- [x] `core/target/.rustc_info.json` drift checked after validation and before commit.
- [x] UI build/test temp files checked after validation and before commit.
- [x] Export temp files checked after validation and before commit.
- [x] Generated artifacts and unrelated tool output removed or absent before commit.
- [x] Source guard confirmed no disallowed source/config/roadmap/package/script diffs.

## findings table
| Finding | Evidence | Status |
| --- | --- | --- |
| Phase 91 scope | Roadmap files list transport abuse and submission spoofing hardening with no live transport. | confirmed |
| Initial working tree | `git status --short` had no output before edits. | confirmed |
| UI unit test harness | `ui/package.json` has no test script and no test files/config were found. | confirmed gap |
| Boundary helper | `buildUiSubmissionBoundaryResult` rejects malformed/spoofed submissions before transport eligibility. | implemented |
| Spoofed flags | User-supplied live/execution/persistence/ledger/audit/provider/replay/authority flags reject with dedicated reasons. | implemented |
| AST lint parity | No new UI AST lint rule was required; existing lint passed. | confirmed |

## deferred items table
| Deferred item | Owning planned phase |
| --- | --- |
| UI behavioral unit test harness for API contract helpers | Phase 95 or out-of-band testing phase |
| Authorization/audit/action mismatch hardening | Phase 92 |
| Persistence corruption and append drift hardening | Phase 93 |
| Provider output injection and replay abuse hardening | Phase 94 |
| Hardening outcome realignment | Phase 95 |

## validation log table
| Command | Result | Notes |
| --- | --- | --- |
| `./scripts/check.sh` | pass | Full local gate passed after final edits. |
| `cargo test --manifest-path core/Cargo.toml --all-targets` | pass | Explicit Rust tests passed. |
| `node scripts/test_rust_boundary_lint.mjs` | pass | Rust boundary lint self-tests passed. |
| `node scripts/rust_boundary_lint.mjs` | pass | Production Rust boundary lint passed. |
| `node scripts/test_lint_ui_boundaries.mjs` | pass | UI AST lint self-tests passed. |
| `node scripts/lint_ui_boundaries.mjs` | pass | UI AST lint passed. |
| `cd ui && npm run typecheck && npm run lint && npm run build` | pass | Existing UI validation commands passed; not behavioral test coverage. |
| `cargo run --manifest-path core/Cargo.toml -- dry-run` | pass | CLI dry-run passed. |
| Transport hardening scan | pass | Phase 91 boundary names/reasons found on intended surfaces. |
| No-live-transport scan | pass | No new UI source live transport/event bridge behavior found; documentation matches are prohibitions/evidence. |
| UI authority scan | pass | Disabled constants and risky examples found on intended surfaces. |
| Source guard | pass | No disallowed source, root test, script, workflow, README, AGENTS, package, lock, UI config, or roadmap diffs. |
| Readiness scan | pass | Matches are negative non-readiness statements only. |
| Lint wiring scan | pass | Local and CI lint wiring evidence reviewed. |
