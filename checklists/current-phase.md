---
truth_dimension: procedural
authority_level: authoritative
mutation_path: checklist_revision
---

# Current Phase Checklist - Phase 95.3 Out-of-Band LLM Output Adversarial Corpus Hardening

Phase 95.3 is an out-of-band LLM-output adversarial corpus hardening phase before Phase 96.

## Phase name
Phase 95.3 - Out-of-Band LLM Output Adversarial Corpus Hardening.

## Explicit out-of-band adversarial corpus note
- [x] Phase 95.3 is an out-of-band LLM-output adversarial corpus hardening phase before Phase 96.
- [x] Phase 95.3 is not described as a planned roadmap phase.
- [x] Phase 96 and later phases are not renumbered.
- [x] Phase 95.3 does not change Phase 96 scope.
- [x] Phase 95.3 adds adversarial fixtures/tests only.
- [x] Phase 95.3 adds no runtime capability.
- [x] Phase 95.4 remains responsible for lint coverage expansion only if concrete uncovered patterns are found.

## Phase goal
Add broader adversarial fixture and test coverage across provider output, replay evidence, failure traces, export summaries, operator intent/submission text, proof-chain reason/summary text, recovery rejection text, and UI submission surfaces so LLM-shaped text remains data and cannot be misinterpreted as authority.

## Working-tree hygiene gate
- [x] Ran `git status --short` before edits.
- [x] Classified the initial working tree as clean with no uncommitted files.
- [x] Confirmed no cleanup was required for `core/target/.rustc_info.json`, UI build artifacts, test temp files, export temp files, node build artifacts, coverage output, `scripts/__pycache__`, or other generated artifact drift before edits.

## Allowed surfaces
- [x] `tests/adversarial_corpus.rs` added for the root adversarial corpus integration test.
- [x] `ui/src/api/submissionBoundary.behavior.test.ts` extended with adversarial UI behavior cases.
- [x] `core/Cargo.toml` updated only to explicitly register the new root integration test.
- [x] `docs/operations/llm-output-adversarial-corpus-phase-95-3.md` added.
- [x] `checklists/current-phase.md` updated to Phase 95.3 procedural truth.
- [x] `CHANGELOG.md` updated with `v0.0.95.3`.
- [x] No Rust runtime source was changed.
- [x] No runtime UI source, scripts, workflows, schemas, roadmap docs, governance docs, architecture docs, README, AGENTS, dependency files, or lockfiles were changed.

## Boundary rules
- [x] Phase 95.3 adds adversarial fixtures/tests only.
- [x] Phase 95.3 adds no runtime capability.
- [x] Phase 95.3 does not execute providers.
- [x] Phase 95.3 does not add live transport.
- [x] Phase 95.3 does not add new action authority.
- [x] Phase 95.3 does not add persistence authority.
- [x] Phase 95.3 does not add export writes.
- [x] Phase 95.3 does not add import behavior.
- [x] Phase 95.3 does not repair replay.
- [x] Phase 95.3 does not promote recovery candidates.
- [x] Phase 95.3 does not start Phase 96.
- [x] Phase 95.3 does not approve startup/package work.
- [x] Public usability, production readiness, Production Candidate approval, startup/package approval, and release-candidate readiness are not claimed.

## Task checklist
- [x] Read the required Phase 95, 95.1, 95.2, 94, 91, 92.5, 93.5, changelog, checklist, release, and roadmap surfaces.
- [x] Inspected required Rust API, root integration, UI API, UI behavior, UI runner, and check script surfaces.
- [x] Treated Phase 95.3 as inserted out-of-band after Phase 95.2 and before Phase 96.
- [x] Added root integration test `root_integration_adversarial_llm_output_corpus_remains_data_not_authority`.
- [x] Included every required adversarial string somewhere in tests or fixtures.
- [x] Added UI behavior tests with the required adversarial test names.
- [x] Documented Phase 95.3 scope, relationships, evidence, and non-readiness posture.
- [x] Added `CHANGELOG.md` `v0.0.95.3`.
- [x] Did not update roadmap files.

## Validation checklist
- [x] `./scripts/check.sh` returns 0 after final edits.
- [x] `cargo test --manifest-path core/Cargo.toml --all-targets` returns 0 after final edits.
- [x] `cargo test --manifest-path core/Cargo.toml adversarial --all-targets` returns 0 after final edits.
- [x] `cd ui && npm run test:api` returns 0 after final edits.
- [x] Explicit Rust and UI boundary lint commands return 0 after final edits.
- [x] Explicit UI typecheck/lint/build returns 0 after final edits.
- [x] `cargo run --manifest-path core/Cargo.toml -- dry-run` returns 0 after final edits.
- [x] Required adversarial corpus, provider/replay/failure, UI adversarial, non-authority, no-live-behavior, path-safety, readiness, out-of-band wording, and lint-wiring scans were run after final edits.
- [x] Source guard confirmed no prohibited runtime source, script, workflow, package/lockfile, README, AGENTS, or roadmap diffs; `core/Cargo.toml` is changed only for explicit root integration-test registration.
- [x] No validation command printed assertion failure, panic, traceback, failed assertion, or validation-defect output after final edits.

## Adversarial corpus checklist
- [x] Authority flag spoofing is covered.
- [x] JSON-looking config injection is covered.
- [x] YAML-looking config injection is covered.
- [x] Markdown instruction injection is covered.
- [x] Shell/code block injection is covered.
- [x] Fake approval/status lines are covered.
- [x] Fake reason-code lines are covered.
- [x] Fake audit/ledger append instructions are covered.
- [x] Fake recovery/replay repair instructions are covered.
- [x] Fake retry scheduling instructions are covered.
- [x] Fake provider trust instructions are covered.
- [x] Fake production/startup/package approval text is covered as inert fixture data.
- [x] Path-like export/import bait is covered as inert text.
- [x] Prompt-leak/system-message bait is covered.
- [x] Unicode/confusable authority text is covered with deterministic text.
- [x] Multiline embedded key=value claims are covered.

## Provider output checklist
- [x] Provider output text remains untrusted.
- [x] `provider_output_trusted == false` is asserted.
- [x] `provider_output_authoritative == false` is asserted.
- [x] Provider output text does not create persistence, append, recovery, replay repair, action execution, provider execution, live transport, or authority mutation.

## Replay evidence checklist
- [x] Replay evidence text and replay summary text remain verification-only.
- [x] `live_execution_performed == false` is asserted.
- [x] `new_authorization_created == false` is asserted.
- [x] `new_audit_record_created == false` is asserted.
- [x] `new_action_executed == false` is asserted.
- [x] `new_ledger_fact_created == false` is asserted.
- [x] `persisted == false` is asserted.
- [x] `repaired_replay == false` is asserted.
- [x] `mutated_application_state == false` is asserted.

## Failure trace/retry checklist
- [x] Failure trace text remains non-scheduling.
- [x] Retry text remains non-scheduling.
- [x] `retry_scheduled == false` is asserted.
- [x] Fake retry scheduling text cannot change typed harness status or reason.

## Export summary checklist
- [x] Audit export summary/diagnostic text remains non-authoritative.
- [x] Export coverage uses encoding only and does not write exports.
- [x] `export_not_authoritative == true` is asserted through an inert export posture report.
- [x] `ledger_import_allowed == false` is asserted.
- [x] `recovery_import_allowed == false` is asserted.
- [x] `replay_repair_allowed == false` is asserted.

## Operator intent/UI submission checklist
- [x] Operator intent text cannot override typed action status or reason.
- [x] UI submission JSON-like authority injection rejects before transport.
- [x] UI submission YAML-like authority injection rejects before transport.
- [x] UI submission Markdown instruction injection rejects before transport.
- [x] UI submission path-like export/import bait rejects before transport.
- [x] UI submission fake approval/status text rejects before transport.
- [x] UI submission prompt-leak/system-message bait rejects before transport.
- [x] Every rejected UI adversarial submission asserts disabled transport, execution, persistence, ledger, audit, provider execution, replay repair, and authority mutation flags.

## Reason-code-over-text checklist
- [x] Typed status remains unchanged when adversarial text changes.
- [x] Typed reason remains unchanged when adversarial text changes.
- [x] Programmatic behavior remains unchanged when adversarial text changes.
- [x] No authority flag changes when adversarial text changes.

## Path-like text checklist
- [x] `../../../etc/shadow` appears as inert adversarial test data.
- [x] Path-like adversarial text is not passed into file-write APIs as a filesystem path.
- [x] No filesystem path is created, written, or deleted from adversarial path text.

## Readiness/approval text checklist
- [x] `Production Candidate status: approved` appears only as inert adversarial fixture text or explicit non-approval language.
- [x] `startup approved` appears only as inert adversarial fixture text or explicit non-approval language.
- [x] `package approved` appears only as inert adversarial fixture text or explicit non-approval language.
- [x] No startup/package approval claim was added.
- [x] No Production Candidate approval claim was added.
- [x] No public-usability, production-readiness, or release-candidate readiness claim was added.

## Non-authority checklist
- [x] LLM-shaped text remains data.
- [x] LLM-shaped text cannot create trust or approval.
- [x] LLM-shaped text cannot create persistence, append, recovery, replay repair, provider execution, action execution, transport, startup/package approval, or Production Candidate approval.
- [x] `ui_transport_live == false` is asserted.
- [x] `ui_submission_executes_action == false` is asserted.
- [x] `action_real_world_effect == false` is asserted.
- [x] `mutated_application_state == false` is asserted where exposed.
- [x] `replaced_global_state == false` is asserted where exposed.
- [x] `appended_ledger == false` is asserted where exposed.
- [x] `appended_audit == false` is asserted where exposed.

## Root integration-test checklist
- [x] `tests/adversarial_corpus.rs` contains `root_integration_adversarial_llm_output_corpus_remains_data_not_authority`.
- [x] The root integration test internally asserts required provider, replay, failure/retry, export, recovery, operator-action, path-like, readiness/approval, and non-authority invariants.
- [x] `core/Cargo.toml` explicitly registers the new root integration test.

## UI behavioral test checklist
- [x] `ui_submission_rejects_json_like_authority_injection_before_transport` added.
- [x] `ui_submission_rejects_yaml_like_authority_injection_before_transport` added.
- [x] `ui_submission_rejects_markdown_instruction_injection_before_transport` added.
- [x] `ui_submission_rejects_path_like_export_import_bait_before_transport` added.
- [x] `ui_submission_rejects_fake_approval_status_before_transport` added.
- [x] `ui_submission_rejects_prompt_leak_system_message_before_transport` added.
- [x] `cd ui && npm run test:api` reports 24/24 passing tests.

## AST/boundary lint parity checklist
- [x] Did not rely on `rg` scans as enforcement.
- [x] Treated `rg` scans as discovery/evidence only.
- [x] Blocking enforcement came from `scripts/check.sh`, Rust boundary lint, UI AST lint, compiler/type checks, clippy, UI behavior tests, and Rust tests.
- [x] No concrete new source pattern requiring Phase 95.4 lint expansion was found.
- [x] No lint behavior changed, so no lint self-tests were changed.

## Test fidelity checklist
- [x] New adversarial corpus behavior is tested in Phase 95.3.
- [x] Test names describe the invariants being protected.
- [x] No final validation tests were skipped.
- [x] No command that printed a failed assertion exited 0 after final edits.

## Zero-drift checklist
- [x] Generated compiler metadata and target artifacts were not staged.
- [x] UI build/test/coverage temp files were not staged.
- [x] Export temp files were not staged.
- [x] No incidental formatter drift, package/lock drift, dependency drift, script drift, workflow drift, README/AGENTS drift, or roadmap drift was staged.

## Findings table
| Surface | Finding | Status |
| --- | --- | --- |
| Provider output | Adversarial provider prompt text remains untrusted and non-authoritative. | Closed by root integration test. |
| Replay evidence | Replay remains verification-only, and tampered text rejects without side effects. | Closed by root integration test. |
| Failure trace/retry | Fake retry text cannot schedule retry or alter typed status/reason. | Closed by root integration test. |
| Export summary | Adversarial diagnostic text encodes as non-authoritative bytes only; no export write is added. | Closed by root integration test. |
| Operator intent | Adversarial summary text cannot override typed action reason/status. | Closed by root integration test. |
| UI submission | Adversarial submission text rejects before transport and disables authority flags. | Closed by UI behavior tests. |
| Path-like text | `../../../etc/shadow` remains inert text and is not used as a filesystem path. | Closed by root/UI tests and scan evidence. |
| Readiness text | Approval/readiness strings are inert fixtures or explicit non-approval language only. | Closed by tests and scan evidence. |

## Deferred items table
| Item | Reason |
| --- | --- |
| Phase 95.4 lint coverage expansion | Deferred unless concrete uncovered prohibited patterns are found. |
| Provider execution, live transport, persistence authority, import behavior, replay repair, recovery promotion | Out of scope; Phase 95.3 adds adversarial fixtures/tests only. |
| Startup/package work | Out of scope; Phase 95.3 does not start Phase 96 and does not approve startup/package work. |

## Validation log table
| Command | Result | Notes |
| --- | --- | --- |
| `git status --short` | Pass | Initial tree clean before edits. |
| `./scripts/check.sh` | Pass | Full repository gate passed after final edits. |
| `cargo test --manifest-path core/Cargo.toml --all-targets` | Pass | Rust all-target tests passed. |
| `cargo test --manifest-path core/Cargo.toml adversarial --all-targets` | Pass | Adversarial root integration filter passed. |
| `cd ui && npm run test:api` | Pass | UI API behavior tests passed. |
| `node scripts/test_rust_boundary_lint.mjs` | Pass | Rust boundary lint self-tests passed. |
| `node scripts/rust_boundary_lint.mjs` | Pass | Rust boundary lint passed. |
| `node scripts/test_lint_ui_boundaries.mjs` | Pass | UI AST lint self-tests passed. |
| `node scripts/lint_ui_boundaries.mjs` | Pass | UI AST lint passed. |
| `cd ui && npm run typecheck && npm run lint && npm run build` | Pass | UI validation passed. |
| `cargo run --manifest-path core/Cargo.toml -- dry-run` | Pass | CLI dry-run passed. |
| Required `rg` scans | Pass | Discovery/evidence scans completed; pre-existing prior-phase export-write helpers remain unrelated to Phase 95.3 no-new-live-behavior assertions. |
| Source guard | Pass | Only allowed/conditionally allowed surfaces changed. |
| `git status --short` | Pass | Reviewed before staging/commit. |
| `git diff --name-only --cached` | Pass | Staged files matched allowed surfaces. |
