---
truth_dimension: procedural
authority_level: authoritative
mutation_path: checklist_revision
---

# Current Phase Checklist

## phase name
Phase 93 - Persistence Corruption and Append Drift Hardening

## phase goal
Add negative-path hardening for corrupted append envelopes, stale revisions, partial writes, replay drift posture, and recovery candidate mismatch without adding persistence authority or repair behavior.

## working-tree hygiene gate
- [x] `git status --short` reviewed before edits; initial working tree was clean.
- [x] Uncommitted files classified before edits: none.
- [x] Generated artifact drift checked before edits; none appeared initially.
- [x] `core/target/.rustc_info.json` drift from local cargo validation was reverted before commit.
- [x] Required roadmap, history, checklist, release, operations, workflow, persistence/recovery/replay, integration smoke, and validation surfaces read or inspected.
- [x] Phase 93 title and scope confirmed from roadmap files: hardening only; no new persistence authority.

## allowed surfaces
- [x] `core/src/api/persistence.rs`
- [x] `core/src/api/application_state.rs`
- [x] `tests/integration_smoke.rs`
- [x] `checklists/current-phase.md`
- [x] `CHANGELOG.md`
- [x] `docs/operations/persistence-corruption-append-drift-phase-93.md`
- [x] `checklists/release.md` was not changed because release evidence posture did not change.
- [x] Conditional local workflow and observability source surfaces were not changed.

## boundary rules
- [x] Phase 93 is hardening only.
- [x] No new persistence authority was added.
- [x] No recovery promotion was added.
- [x] No global state replacement was added.
- [x] No replay repair was added.
- [x] No corruption repair was added.
- [x] No export import behavior was added.
- [x] No provider/model execution was added.
- [x] No action execution was added.
- [x] No live UI/Rust transport was added.
- [x] No async/network/process/thread behavior was added by Phase 93 source changes.
- [x] No dependency changes were made.
- [x] Public usability, production readiness, Production Candidate approval, and release-candidate readiness are not claimed.

## task checklist
- [x] Updated this checklist to Phase 93 procedural truth.
- [x] Created `docs/operations/persistence-corruption-append-drift-phase-93.md`.
- [x] Added persisted-record corruption tests for payload length mismatch, invalid payload hex, and checksum mismatch.
- [x] Added durable append negative-path tests for checksum drift, missing audit payload, missing ledger payload, malformed revisions, stale revision chains, and transaction ID mismatch.
- [x] Added durable append non-commit tests for tampered bytes, partial-write simulation, failed writes, and verification failures.
- [x] Added append drift tests for revision advancement, prior/next revision mismatch, audit payload drift, ledger payload drift, audit-only transactions, ledger-only transactions, and non-promotion/non-recovery/non-repair reports.
- [x] Added recovery mismatch tests for recovery ID, ledger record ID, revision, empty bytes, non-replacement, non-persistence/non-append, and non-replay-repair.
- [x] Added root integration smoke coverage for public durable append tampering, recovery mismatch, and export non-authority behavior.
- [x] Preserved Phase 83 durable append success behavior.
- [x] Preserved Phase 84 recovery acceptance success behavior.
- [x] Preserved Phase 89 export-write non-authority posture.
- [x] Added `CHANGELOG.md` v0.0.93.
- [x] Roadmap files were not updated; Phase 95 remains the next alignment checkpoint.

## validation checklist
- [x] `./scripts/check.sh`
- [x] `cargo test --manifest-path core/Cargo.toml --all-targets`
- [x] `node scripts/test_rust_boundary_lint.mjs`
- [x] `node scripts/rust_boundary_lint.mjs`
- [x] `node scripts/test_lint_ui_boundaries.mjs`
- [x] `node scripts/lint_ui_boundaries.mjs`
- [x] `cd ui && npm run typecheck && npm run lint && npm run build`
- [x] `cargo run --manifest-path core/Cargo.toml -- dry-run`
- [x] Persistence corruption scans run and reviewed.
- [x] Append drift scans run and reviewed.
- [x] Recovery mismatch scans run and reviewed.
- [x] Export-not-ledger scans run and reviewed.
- [x] No-authority scans run and reviewed.
- [x] Source guard run and reviewed.
- [x] Readiness scan run and reviewed.
- [x] Lint wiring scan run and reviewed.
- [x] Final `git status --short` reviewed before commit.
- [x] `git diff --name-only --cached` reviewed before commit.

## persistence corruption checklist
- [x] `persisted_record_rejects_payload_length_mismatch`
- [x] `persisted_record_rejects_invalid_payload_hex`
- [x] `persisted_record_rejects_checksum_mismatch`
- [x] Corrupted persisted records reject without repair.

## append drift checklist
- [x] `durable_append_decode_rejects_checksum_drift`
- [x] `durable_append_decode_rejects_missing_audit_payload`
- [x] `durable_append_decode_rejects_missing_ledger_payload`
- [x] `durable_append_decode_rejects_malformed_revision`
- [x] `durable_append_decode_rejects_stale_revision_chain`
- [x] `durable_append_verify_rejects_transaction_id_mismatch`
- [x] `append_revision_must_advance_by_one`
- [x] `append_prior_revision_mismatch_rejects`
- [x] `append_next_revision_mismatch_rejects`
- [x] `append_audit_payload_change_after_checksum_rejects`
- [x] `append_ledger_payload_change_after_checksum_rejects`
- [x] `append_audit_only_transaction_never_verifies`
- [x] `append_ledger_only_transaction_never_verifies`
- [x] `append_report_does_not_promote_recover_or_repair`

## partial-write checklist
- [x] `durable_append_tampered_bytes_are_not_committed`
- [x] `durable_append_partial_write_is_not_committed_if_simulatable`
- [x] `durable_append_failed_write_is_not_committed`
- [x] `durable_append_verification_failure_is_not_committed`
- [x] Rejected append paths report `committed=false`.

## recovery mismatch checklist
- [x] `recovery_acceptance_rejects_candidate_recovery_id_mismatch`
- [x] `recovery_acceptance_rejects_candidate_ledger_record_id_mismatch`
- [x] `recovery_acceptance_rejects_candidate_revision_mismatch`
- [x] `recovery_acceptance_rejects_empty_candidate_bytes`
- [x] `recovery_candidate_mismatch_does_not_replace_global_state`
- [x] `recovery_candidate_mismatch_does_not_persist_or_append`
- [x] `recovery_candidate_mismatch_does_not_repair_replay`

## export-not-ledger checklist
- [x] `export_bundle_is_not_ledger_state`
- [x] `export_bundle_is_not_recovery_input`
- [x] `export_bundle_is_not_replay_repair_evidence`
- [x] `export_write_artifact_cannot_be_verified_as_durable_append_transaction`
- [x] Export bundles remain non-authoritative and not importable as ledger, recovery, or replay repair state.

## replay drift posture checklist
- [x] No replay repair behavior was added.
- [x] Existing replay posture remains diagnostic/verification-only.
- [x] Replay abuse hardening remains assigned to Phase 94.

## non-repair checklist
- [x] No corrupted persisted record repair was added.
- [x] No durable append drift repair was added.
- [x] No partial-write repair was added.
- [x] No recovery mismatch repair was added.
- [x] No export-as-repair behavior was added.
- [x] No replay drift repair was added.

## non-authority checklist
- [x] No new persistence authority was added.
- [x] Export bundles were not made authoritative.
- [x] Recovery candidates were not made global state.
- [x] No provider output was trusted.
- [x] No action execution was added.
- [x] No LocalApplicationState mutation was added.

## root integration-test checklist
- [x] `root_integration_durable_append_rejects_tampered_transaction`
- [x] `root_integration_recovery_acceptance_rejects_mismatched_candidate`
- [x] `root_integration_export_bundle_is_not_authoritative_state`
- [x] Export-not-ledger public smoke tests were expressible without broad export reshaping.

## AST/boundary lint parity checklist
- [x] No lint behavior changed.
- [x] `rg` scans were treated as discovery/evidence only.
- [x] Blocking enforcement remained with `scripts/check.sh`, Rust boundary lint, UI AST lint, compiler/type checks, and tests.
- [x] No new Rust source pattern requiring a lint maintenance phase was introduced.

## test fidelity checklist
- [x] New hardening behavior has same-phase tests.
- [x] Public cross-boundary behavior has root integration coverage.
- [x] Test names describe the invariant protected.
- [x] Tests were not skipped after final edits.
- [x] No network, async, provider execution, action execution, import behavior, or authority mutation was introduced.

## zero-drift checklist
- [x] `git diff --name-only` reviewed for allowed surfaces only.
- [x] Source guard returned no disallowed diffs.
- [x] Generated `core/target/.rustc_info.json` drift was reverted.
- [x] No UI build artifacts, test temp files, export temp files, package drift, lockfile drift, roadmap drift, script drift, workflow drift, README drift, AGENTS drift, or unrelated output is staged.

## findings table
| Area | Finding | Evidence |
| --- | --- | --- |
| Persistence corruption | Payload length, invalid hex, and checksum mismatch reject. | Module tests and persistence scans. |
| Append drift | Checksum drift, stale revision chains, malformed revisions, audit-only, and ledger-only transactions reject. | Module tests and append scans. |
| Partial/failed write | Failed and verification-failed append paths report `committed=false`; orphaned temp is diagnostic and not repaired. | Module tests. |
| Recovery mismatch | Candidate metadata mismatches reject and remain side-effect-free. | Module tests and root integration test. |
| Export non-authority | Export artifacts are not ledger/recovery/replay repair input and do not verify as append transactions. | Root integration tests. |
| Replay posture | Replay repair remains out of scope for Phase 93. | Operations doc and no-authority scans. |

## deferred items table
| Item | Disposition | Reason |
| --- | --- | --- |
| Replay drift repair behavior | Deferred | Phase 93 does not add repair; Phase 94 covers provider/replay abuse hardening. |
| Lint expansion | Deferred | No new prohibited Rust source pattern was introduced. |

## validation log table
| Command | Status | Notes |
| --- | --- | --- |
| `./scripts/check.sh` | Passed | Full repository gate. |
| `cargo test --manifest-path core/Cargo.toml --all-targets` | Passed | Rust unit, binary, and root integration coverage. |
| `node scripts/test_rust_boundary_lint.mjs` | Passed | Rust boundary lint self-test. |
| `node scripts/rust_boundary_lint.mjs` | Passed | Rust boundary lint. |
| `node scripts/test_lint_ui_boundaries.mjs` | Passed | UI boundary lint self-test. |
| `node scripts/lint_ui_boundaries.mjs` | Passed | UI boundary lint. |
| `cd ui && npm run typecheck && npm run lint && npm run build` | Passed | Explicit UI validation. |
| `cargo run --manifest-path core/Cargo.toml -- dry-run` | Passed | CLI dry-run remains non-executing. |
| Required evidence scans | Passed | Scans reviewed as evidence only. |
| Source guard | Passed | No disallowed diffs. |
