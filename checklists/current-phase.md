---
truth_dimension: procedural
revision_mode: replace
authority_level: authoritative
mutation_path: checklist_revision
---

# Current Phase Checklist - Phase 164 Trial Replay and Restore Verification

## Phase name
- [x] Phase 164 - Trial Replay and Restore Verification.

## Phase goal
- [x] Compare controlled internal trial package read-back, trial session evidence read-back, restore/history projection, workflow linkage, and replay/status snapshots.
- [x] Expose deterministic Rust-owned verification status in the local shell and UI.
- [x] Keep verification local-only, non-public, non-production, and non-authoritative.

## Working-tree hygiene gate
- [x] Check branch status before final handoff.
- [x] Keep changes limited to Phase 164 code-production surfaces.
- [x] Do not modify roadmap files.

## Allowed surfaces
- [x] `core/src/**`
- [x] `ui/src/**`
- [x] `CHANGELOG.md`
- [x] `checklists/current-phase.md`

## Code-production deliverable checklist
- [x] Rust-owned `TrialReplayRestoreVerificationProjection`.
- [x] Rust-owned status, mismatch/error, boundary, and comparison summary types.
- [x] Deterministic verification result ID.
- [x] UI-visible verification panel and mismatch drilldown.

## Trial replay/restore verification checklist
- [x] Initial `not_verified` projection.
- [x] Valid package and evidence can verify.
- [x] Verification derivation is deterministic.
- [x] Verification derivation does not mutate state, write files, execute providers/actions, repair replay, promote recovery, or start a trial.

## Package/evidence linkage checklist
- [x] Compare package ID.
- [x] Compare package version.
- [x] Compare package, production, distribution, and authority classifications.
- [x] Compare workflow, local candidate materialization, provider output pipeline, and operator decision snapshots.

## Read-back verification checklist
- [x] Missing package read-back fails closed.
- [x] Missing evidence read-back fails closed.
- [x] Malformed/invalid package read-back fails closed.
- [x] Malformed/invalid evidence read-back fails closed.

## Replay/status comparison checklist
- [x] Compare trial evidence replay/status snapshot with current replay/status projection.
- [x] Reject replay/status mismatch.

## Restore/history comparison checklist
- [x] Compare trial evidence restore/history snapshot with current restore/history projection.
- [x] Reject restore/history mismatch.

## Mismatch/error model checklist
- [x] Deterministic mismatch ordering.
- [x] Descriptive mismatch codes for package/evidence, workflow, replay, restore, stop-condition, escalation, and failure category drift.
- [x] Reject readiness, release, deployment, public-use, production-use, provider-trust, action-authorization, replay-repair, and recovery-promotion claims.

## UI verification panel checklist
- [x] Render `Trial replay and restore verification`.
- [x] Render `Replay/restore verification`.
- [x] Render `Verification mismatch drilldown`.
- [x] Render verification status, ID, read-back status, linkage status, replay/status comparison, restore/history comparison, and mismatches.

## Local-only/non-authority boundary checklist
- [x] Include `local_verification_only`.
- [x] Include `non_public_verification`.
- [x] Include `verification_not_authority`.
- [x] Include no trial execution, no replay repair, no recovery promotion, and no controlled-human-use/readiness/release/deployment/public-use/production approval.

## Rust test checklist
- [x] Valid verification.
- [x] Missing/malformed package and evidence rejection.
- [x] Package/evidence mismatch rejection.
- [x] Replay/status mismatch rejection.
- [x] Restore/history mismatch rejection.
- [x] Deterministic result and non-authority boundaries.

## TypeScript test checklist
- [x] Visible verification panel.
- [x] Rejected verification state and mismatch drilldown.
- [x] Local-only/non-authority wording.
- [x] Deterministic rendering.

## Phase 165 handoff checklist
- [x] Phase 165 remains the next code-production alignment checkpoint.
- [x] No readiness, release, deployment, public-use, production-use, trial-start, replay-repair, or recovery-promotion behavior is introduced.

## Validation checklist
- [ ] `CARGO_TARGET_DIR=/tmp/ajentic-phase-164-target ./scripts/check.sh`
- [ ] `git diff --check`
- [ ] `git status --short`
- [ ] UI direct checks if needed.
- [ ] Rust direct tests if needed.
- [ ] Local dev smoke test.
- [ ] Required scans.

## Deferred items
- [x] No repair, remediation, promotion, approval, release, deployment, signing, publishing, network, cloud, background service, or daemon behavior.

## Validation log
- [ ] Fill with final command results before handoff.

## Zero-drift checklist
- [x] Roadmap files unchanged.
- [x] CHANGELOG entry matches actual Phase 164 diff.
- [x] Verification compares artifacts/projections only.
- [x] UI remains non-authoritative and display-only.
