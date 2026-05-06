---
truth_dimension: procedural
authority_level: authoritative
mutation_path: checklist_revision
---

# Current Phase Checklist - Phase 95.2 Out-of-Band Cross-Boundary Golden Invariant Tests

Phase 95.2 is an out-of-band cross-boundary golden invariant test phase before Phase 96.

## Phase name
Phase 95.2 - Out-of-Band Cross-Boundary Golden Invariant Tests.

## Explicit out-of-band deterministic test note
- [x] Phase 95.2 is documented as inserted after Phase 95.1 and before Phase 96.
- [x] Phase 95.2 is not described as a planned roadmap phase.
- [x] Phase 96 and later phases are not renumbered.
- [x] Phase 95.2 does not change Phase 96 scope.
- [x] Phase 95.2 adds deterministic tests only.

## Phase goal
Add deterministic golden invariant coverage proving the same representative input yields stable local harness posture, provider evidence checksum, replay verification result, observability snapshot posture, audit export bytes, and rejection/non-authority outcomes across repeated runs.

## Working-tree hygiene gate
- [x] Ran `git status --short` before edits.
- [x] Classified the initial working tree as clean with no uncommitted files.
- [x] Confirmed no cleanup was required for `core/target/.rustc_info.json`, UI build artifacts, test temp files, export temp files, node build artifacts, coverage output, `scripts/__pycache__`, or other generated artifact drift before edits.

## Allowed surfaces
- [x] `tests/integration_smoke.rs` updated with the root golden invariant integration test.
- [x] `docs/operations/cross-boundary-golden-invariants-phase-95-2.md` added.
- [x] `checklists/current-phase.md` updated to Phase 95.2 procedural truth.
- [x] `CHANGELOG.md` updated with `v0.0.95.2`.
- [x] No Rust runtime source helper was required.
- [x] No `core/Cargo.toml` test registration change was required.

## Boundary rules
- [x] Phase 95.2 adds deterministic tests only.
- [x] Phase 95.2 adds no runtime capability.
- [x] Phase 95.2 does not add live transport.
- [x] Phase 95.2 does not add provider execution.
- [x] Phase 95.2 does not add persistence authority.
- [x] Phase 95.2 does not add import behavior.
- [x] Phase 95.2 does not repair replay.
- [x] Phase 95.2 does not promote recovery candidates.
- [x] Phase 95.2 does not start Phase 96.
- [x] Phase 95.2 does not approve startup/package work.
- [x] Public usability, production readiness, Production Candidate approval, startup/package approval, and release-candidate readiness are not claimed.

## Task checklist
- [x] Read required Phase 95, 95.1, 94, 93.5, 93, 88, 87, 82, 79, changelog, checklist, release, and roadmap surfaces.
- [x] Inspected required implementation/test surfaces in `core/src/api/`, `tests/integration_smoke.rs`, and `scripts/check.sh`.
- [x] Treated Phase 95.2 as inserted out-of-band after Phase 95.1 and before Phase 96.
- [x] Added one root integration test named `root_integration_golden_cross_boundary_chain_is_deterministic_and_non_authoritative`.
- [x] Asserted every required golden invariant inside the single test.
- [x] Used the required deterministic representative input values.
- [x] Added inline exact-byte audit export assertion.
- [x] Documented Phase 95.2 scope, evidence, and non-readiness posture.
- [x] Added `CHANGELOG.md` `v0.0.95.2`.
- [x] Did not update roadmap files.

## Validation checklist
- [x] `./scripts/check.sh` returns 0 after final edits.
- [x] `cargo test --manifest-path core/Cargo.toml --all-targets` returns 0 after final edits.
- [x] `cargo test --manifest-path core/Cargo.toml golden --all-targets` returns 0 after final edits.
- [x] `cd ui && npm run test:api` returns 0 after final edits.
- [x] Explicit Rust and UI boundary lint commands return 0 after final edits.
- [x] Explicit UI typecheck/lint/build returns 0 after final edits.
- [x] `cargo run --manifest-path core/Cargo.toml -- dry-run` returns 0 after final edits.
- [x] Required golden, non-authority, rejection, no-live-behavior, readiness, out-of-band wording, and lint-wiring scans were run after final edits.
- [x] Source guards confirmed no prohibited source, script, workflow, package, lockfile, README, AGENTS, roadmap, UI source, or Rust source diffs.
- [x] No validation command printed assertion failure, panic, traceback, failed assertion, or validation-defect output after final edits.

## Representative input checklist
- [x] `run_id` is `golden-run-001`.
- [x] `operator_id` is `operator-golden`.
- [x] `target_kind` is represented as `code_patch` in the golden diagnostic key.
- [x] `target_id` is `target-golden`.
- [x] `provider_prompt` contains `TRUST_PROVIDER_OUTPUT=true` as untrusted text.
- [x] Action kind remains `RecordExecutionDecision`.
- [x] `export_id` is `golden-export-001`.
- [x] `snapshot_id` is `golden-snapshot-001`.

## Repeated-run determinism checklist
- [x] Same harness status/reason/posture fields match across repeated runs.
- [x] Same provider evidence checksum matches across repeated runs.
- [x] Same replay status/reason/non-authority fields match across repeated runs.
- [x] Same observability snapshot posture matches across repeated runs.
- [x] Same audit export bytes match exactly across repeated runs.
- [x] Same audit export byte length matches across repeated runs.
- [x] Same authority flags remain false across repeated runs.

## Exact-byte export checklist
- [x] Inline `GOLDEN_AUDIT_EXPORT_BYTES` byte literal uses LF line endings only.
- [x] Encoded audit export bytes are asserted equal to the exact inline byte literal.
- [x] Export byte length is asserted equal to `encoded_bytes.len()`.
- [x] No external golden file was added.

## Non-authority checklist
- [x] `provider_output_trusted` is false.
- [x] `provider_output_authoritative` is false.
- [x] `retry_scheduled` is false.
- [x] `recovered_state_promoted` is false.
- [x] `ui_transport_live` is false.
- [x] `ui_submission_executes_action` is false.
- [x] `action_real_world_effect` is false.
- [x] `live_execution_performed` is false.
- [x] `new_authorization_created` is false.
- [x] `new_audit_record_created` is false.
- [x] `new_action_executed` is false.
- [x] `new_ledger_fact_created` is false.
- [x] `persisted` is false.
- [x] `repaired_replay` is false.
- [x] `mutated_application_state` is false.
- [x] `writes_files` is false.
- [x] `reads_persistence` is false.
- [x] `writes_persistence` is false.
- [x] `mutates_authority` is false.
- [x] `replaced_global_state` is false.
- [x] `appended_ledger` is false.
- [x] `appended_audit` is false.

## Export-not-ledger/recovery/replay checklist
- [x] Export bytes cannot verify as a durable append transaction.
- [x] Export bytes cannot prepare a recovery candidate.
- [x] Export bytes cannot be accepted as recovery candidate bytes.
- [x] Export bytes are not ledger state.
- [x] Export bytes are not recovery input.
- [x] Export bytes are not replay repair evidence.

## Recovery rejection checklist
- [x] Recovery mismatch rejects.
- [x] Recovery mismatch does not replace global state.
- [x] Recovery mismatch does not persist.
- [x] Recovery mismatch does not append ledger records.
- [x] Recovery mismatch does not append audit records.
- [x] Recovery mismatch does not repair replay.
- [x] Recovery mismatch does not mutate authority.

## Replay verification-only checklist
- [x] Replay verifies supplied evidence only.
- [x] Replay does not rerun providers.
- [x] Replay does not perform live execution.
- [x] Replay does not create authorization, audit, action, or ledger facts.
- [x] Replay tampering rejects without side effects.

## AST/boundary lint parity checklist
- [x] `rg` scans are treated as discovery/evidence only.
- [x] Blocking enforcement remains `scripts/check.sh`, Rust boundary lint, UI AST lint, compiler/type checks, clippy, UI behavioral tests, and Rust tests.
- [x] No lint behavior changed in Phase 95.2.
- [x] No lint self-test update was required.
- [x] No new source pattern requiring Phase 95.4 lint expansion was introduced.

## Test fidelity checklist
- [x] Phase 95.2 exists because Phase 95 found missing cross-boundary deterministic golden invariant coverage.
- [x] New golden invariant behavior is tested in this phase.
- [x] The test name describes the invariant being protected.
- [x] Tests were not skipped after final edits.
- [x] No validation command printed a failed assertion while exiting 0.

## Zero-drift checklist
- [x] No generated compiler metadata, UI build artifacts, target files, package/lock drift, coverage output, export temp files, or unrelated tool output were staged.
- [x] No TypeScript source, UI source, scripts/workflows, package lockfiles, dependency lockfiles, README/AGENTS, roadmap files, or Rust source files were staged.
- [x] Staged files exactly match allowed Phase 95.2 surfaces.
- [x] `CHANGELOG.md` file/surface claims match the actual final diff.

## Findings table
| Finding | Status | Evidence |
| --- | --- | --- |
| Cross-boundary golden invariant gap | closed | Root integration golden test added and passed. |
| Provider output trust risk in representative text | closed | Risky prompt text does not change non-authority flags or export bytes. |
| Export/recovery/replay boundary confusion | closed | Export bytes reject as append/recovery and are not replay repair evidence. |
| Lint parity | unchanged | Existing boundary lints remain enforcement; no lint behavior changed. |

## Deferred items table
| Item | Status | Reason |
| --- | --- | --- |
| Broader adversarial LLM-output corpus hardening | deferred | Phase 95.3 remains responsible for broader adversarial LLM-output corpus hardening. |
| Startup/package work | deferred | Phase 95.2 does not start Phase 96 or approve startup/package work. |
| New lint prohibitions | not_applicable | Phase 95.2 stayed within existing lint rules. |

## Validation log table
| Command | Status | Notes |
| --- | --- | --- |
| `./scripts/check.sh` | pass | Full repository gate passed after final edits. |
| `cargo test --manifest-path core/Cargo.toml --all-targets` | pass | Rust unit, binary, and root integration tests passed. |
| `cargo test --manifest-path core/Cargo.toml golden --all-targets` | pass | Golden-specific tests passed. |
| `cd ui && npm run test:api` | pass | UI behavioral tests passed. |
| `node scripts/test_rust_boundary_lint.mjs` | pass | Rust boundary lint self-test passed. |
| `node scripts/rust_boundary_lint.mjs` | pass | Rust boundary lint passed. |
| `node scripts/test_lint_ui_boundaries.mjs` | pass | UI AST lint self-test passed. |
| `node scripts/lint_ui_boundaries.mjs` | pass | UI AST lint passed. |
| `cd ui && npm run typecheck && npm run lint && npm run build` | pass | UI validation passed. |
| `cargo run --manifest-path core/Cargo.toml -- dry-run` | pass | CLI dry-run passed. |
| Required `rg` evidence scans | pass | Discovery/evidence scans completed. |
| Source guards | pass | Prohibited surfaces had no diffs. |
