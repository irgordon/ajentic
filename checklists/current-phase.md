---
truth_dimension: procedural
authority_level: authoritative
mutation_path: checklist_revision
---

# Phase 166 - Controlled Internal Trial Execution Harness

## Phase goal
- [x] Add a bounded deterministic local controlled internal trial execution harness.
- [x] Keep the harness local-only, non-public, and non-authoritative.
- [x] Preserve Phase 167 as trial observability and error reporting.

## Working-tree hygiene gate
- [x] Changes remain limited to Phase 166 code-production surfaces, CHANGELOG.md, and this checklist.
- [x] Roadmap files are not modified.
- [x] Generated UI build artifacts are removed after validation.

## Allowed surfaces
- [x] `core/src/**`
- [x] `ui/src/**`
- [x] `CHANGELOG.md`
- [x] `checklists/current-phase.md`

## Code-production deliverable checklist
- [x] Rust-owned trial execution harness projection exists.
- [x] Trial run request and run projection exist.
- [x] Initial harness state is `not_started`.
- [x] Deterministic trial-run ID and projection derivation are implemented.

## Controlled trial execution harness checklist
- [x] Harness can start a bounded local trial run only through typed local request data.
- [x] Harness can step a local trial run to local completion when manual status is recorded.
- [x] Rejected requests preserve the prior accepted trial run projection.

## Trial precondition checklist
- [x] Controlled internal trial package is required.
- [x] Trial operator runbook is required and must not be blocked.
- [x] Failure drill projection is required.
- [x] Trial session evidence must be valid or read-back valid.
- [x] Replay/restore verification must be passed.
- [x] Complete local workflow projection is required.

## Stop-condition observation checklist
- [x] Stop-condition observation is typed.
- [x] `stop_condition_observed` blocks continuation.
- [x] Stop-condition enforcement is not automated in Phase 166.

## Manual operator step checklist
- [x] Manual operator step status is represented.
- [x] Missing manual operator step produces `manual_action_required` or missing status.
- [x] Manual status is not human-use approval.

## Evidence linkage checklist
- [x] Trial package linkage is represented.
- [x] Runbook linkage is represented.
- [x] Failure drill linkage is represented.
- [x] Trial session evidence linkage is represented.
- [x] Replay/restore verification linkage is represented.
- [x] Complete local workflow linkage is represented.

## UI harness panel checklist
- [x] UI renders “Controlled internal trial execution harness”.
- [x] UI renders “Trial run status”.
- [x] UI renders “Trial stop-condition observation”.
- [x] UI renders “Trial evidence linkage”.
- [x] UI renders blockers, rejection reasons, preconditions, steps, and boundary markers.
- [x] UI renders start/step controls as enabled or disabled from projection state.

## Local-only/non-public boundary checklist
- [x] `controlled_internal_trial_harness_only`
- [x] `local_trial_execution_only`
- [x] `non_public_trial_execution`
- [x] No network/cloud/background service behavior added.

## No-authority checklist
- [x] No controlled human-use approval.
- [x] No readiness, release, deployment, public-use, or production-use approval.
- [x] No provider trust.
- [x] No action authorization.
- [x] No replay repair.
- [x] No recovery promotion.
- [x] No signing, publishing, deployment, release artifact, or installer behavior.
- [x] No automated stop-condition enforcement.
- [x] No automated escalation.

## Rust test checklist
- [x] Accepted harness run.
- [x] Missing precondition rejection.
- [x] Stop-condition blocking.
- [x] Deterministic run projection and ID.
- [x] No-authority boundary rejection.

## TypeScript test checklist
- [x] Visible harness behavior.
- [x] Blocked/rejected rendering.
- [x] Stop-condition observation rendering.
- [x] Evidence linkage rendering.
- [x] No-authority wording and forbidden-label checks.

## Phase 167 handoff checklist
- [x] Phase 167 remains trial observability and error reporting.
- [x] Phase 166 does not add observability/error-reporting doctrine beyond harness display.

## Validation checklist
- [x] `CARGO_TARGET_DIR=/tmp/ajentic-phase-166-target ./scripts/check.sh` (run after commit for clean-worktree gate)
- [x] `git diff --check`
- [x] `git status --short`
- [x] UI typecheck, lint, build cleanup, and API behavior tests.
- [x] Rust all-target tests.
- [x] Local dev smoke test.
- [x] Harness, boundary, forbidden-label, unsafe-execution, release/deployment, changed-file, and roadmap-drift scans.

## Deferred items
- [x] Trial observability and error reporting are deferred to Phase 167.
- [x] Controlled human use, public use, production use, readiness, release, deployment, signing, publishing, replay repair, recovery promotion, and action authorization remain out of scope.

## Validation log
- Direct validation passed before commit except `scripts/check.sh` clean-worktree gate, which is run after commit.
- UI dev smoke printed `http://127.0.0.1:5173`; timeout stopped the long-running local server.
- Scans showed required harness and boundary markers; forbidden/unsafe/release scan matches are historical/test/prohibition strings or required `no_...` boundary markers, not new executable behavior.

## Zero-drift checklist
- [x] CHANGELOG entry matches Phase 166 scope.
- [x] Checklist reflects procedural truth for Phase 166.
- [x] Governance, architecture, and roadmap documents are unchanged.
