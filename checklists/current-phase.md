---
truth_dimension: procedural
authority_level: authoritative
mutation_path: checklist_revision
---

# Current Phase Checklist - Phase 162 Trial Operator Runbook and Failure Drill UI

## Phase name
- [x] Phase 162 - Trial Operator Runbook and Failure Drill UI.

## Phase goal
- [x] Add Rust-owned local trial operator runbook and failure drill projections.
- [x] Display controlled-trial preparation guidance, blockers, stop-condition drills, and escalation guidance in the local UI without starting a trial or granting authority.

## Working-tree hygiene gate
- [x] Confirm starting worktree state.
- [x] Keep changes limited to Phase 162 code, UI, test, changelog, and checklist surfaces.
- [x] Do not edit roadmap, governance, architecture, release, deployment, installer, update, signing, publishing, provider execution, action execution, replay repair, or recovery promotion surfaces.

## Allowed surfaces
- [x] `core/src/**`
- [x] `ui/src/**`
- [x] `CHANGELOG.md`
- [x] `checklists/current-phase.md`

## Code-production deliverable checklist
- [x] Rust-owned runbook projection.
- [x] Rust-owned failure drill projection.
- [x] Visible UI panels for trial operator runbook, trial failure drill, stop-condition drill, and escalation guidance.
- [x] Behavior tests for Rust projections and TypeScript rendering.

## Trial operator runbook checklist
- [x] Include runbook status, current step, ordered steps, package status/ID, validation/read-back status, trial scope, named operator, named participant, workflow blocker, candidate materialization, provider pipeline, replay, evidence export, and restore/history summaries.
- [x] Derive deterministically from local shell state.

## Failure drill checklist
- [x] Classify missing package, package validation/read-back failure, workflow blocked/rejected, provider pipeline, provider validation, staged validation, operator decision, materialization, restore, replay, evidence export, and stop-condition categories.
- [x] Include severity, manual action guidance, and rejection summary.

## Stop-condition drill checklist
- [x] Render stop-condition drill markers from the controlled internal trial package projection.
- [x] State that enforcement is not automated in Phase 162.

## Escalation guidance checklist
- [x] Include trial coordinator, security reviewer, release steward, and operator manual review guidance where applicable.
- [x] State that escalation guidance is descriptive only and does not activate authority.

## UI runbook/failure drill checklist
- [x] Render `Trial operator runbook`.
- [x] Render `Trial failure drill`.
- [x] Render `Stop-condition drill`.
- [x] Render `Escalation guidance`.
- [x] Render current blocker guidance.
- [x] Avoid start-trial, publish, deploy, sign, release, and approval controls.

## Local-only/non-public boundary checklist
- [x] Include `local_trial_guidance_only`.
- [x] Include `non_public_trial_guidance`.
- [x] UI states: “Trial operator runbook is local-only and non-public.”
- [x] UI states: “This runbook does not start a controlled trial.”

## No-authority checklist
- [x] Include no trial execution.
- [x] Include no stop-condition automation.
- [x] Include no authority activation.
- [x] Include no readiness, release, deployment, public-use, or production approval.
- [x] Include no action execution.
- [x] Include no replay repair or recovery promotion.

## Rust test checklist
- [x] Initial runbook and failure drill projection.
- [x] Determinism.
- [x] Valid package state.
- [x] Missing package/scope/operator/participant/stop-condition blockers.
- [x] Validation/read-back, workflow/provider/restore classifications.
- [x] Stop-condition, escalation guidance, and no-authority boundaries.

## TypeScript test checklist
- [x] Visible runbook panel.
- [x] Valid and blocked states.
- [x] Failure drill rendering.
- [x] Stop-condition drill rendering.
- [x] Escalation guidance rendering.
- [x] Forbidden control absence and deterministic rendering.

## Phase 163 handoff checklist
- [x] Phase 163 remains the next code-production phase for trial session evidence capture.
- [x] Phase 162 does not start trial execution or approve controlled human use.

## Validation checklist
- [x] `CARGO_TARGET_DIR=/tmp/ajentic-phase-162-target ./scripts/check.sh`
- [x] `git diff --check`
- [x] `git status --short`
- [x] `cd ui && npm run typecheck`
- [x] `cd ui && npm run lint`
- [x] `cd ui && npm run build && rm -rf dist`
- [x] `cd ui && npm run test:api`
- [x] `CARGO_TARGET_DIR=/tmp/ajentic-phase-162-target cargo test --manifest-path core/Cargo.toml --all-targets`
- [x] `cd ui && timeout 5 npm run dev`
- [x] Runbook scan.
- [x] Boundary scan.
- [x] Forbidden label scan.
- [x] Unsafe execution scan.
- [x] Release/deployment scan.
- [x] Changed-file source guard.
- [x] No-roadmap-drift guard.

## Deferred items
- [x] Trial execution remains deferred.
- [x] Automated stop-condition enforcement remains deferred.
- [x] Trial session evidence capture remains Phase 163.
- [x] Replay repair, recovery promotion, action execution, release, deployment, signing, publishing, installer, update-channel, public-use, production-use, readiness, and provider trust remain deferred.

## Validation log
- `CARGO_TARGET_DIR=/tmp/ajentic-phase-162-target ./scripts/check.sh` was attempted before commit and correctly stopped on dirty-worktree preflight, then passed after committing the Phase 162 patch.
- `git diff --check` passed.
- `cd ui && npm run typecheck` passed.
- `cd ui && npm run lint` passed.
- `cd ui && npm run build && rm -rf dist` passed and generated `dist` was removed.
- `cd ui && npm run test:api` passed.
- `CARGO_TARGET_DIR=/tmp/ajentic-phase-162-target cargo test --manifest-path core/Cargo.toml --all-targets` passed.
- `cd ui && timeout 5 npm run dev` printed the local browser URL; timeout stopped the long-running dev server as expected.
- Runbook, boundary, forbidden-label, unsafe-execution, release/deployment, changed-file, and no-roadmap-drift scans completed; broad scans reported historical/prohibition matches only, with no Phase 162 runtime execution, network, release, deployment, or authority behavior added.
- `git status --short` showed only allowed Phase 162 files changed before commit.

## Zero-drift checklist
- [x] Rust-owned trial operator runbook projection exists.
- [x] Rust-owned trial failure drill projection exists.
- [x] Transport/local shell state carries projections.
- [x] UI renders required runbook and drill surfaces.
- [x] Local-only/non-public and no-authority markers are present.
- [x] Roadmap files are not modified.
- [x] CHANGELOG entry matches actual diff.
- [x] Phase 163 remains the next code-production phase for trial session evidence capture.
