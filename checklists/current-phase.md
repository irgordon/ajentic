---
truth_dimension: procedural
scope: current_phase
authority_level: authoritative
mutation_path: checklist_revision
---

# Current phase checklist

Phase 159 - Complete Local Operator Workflow.

## Phase goal
- [x] Consolidate the implemented local provider, pipeline, validation, review, staging, decision, materialization, replay, export, package, history, and restore surfaces into one complete local operator workflow projection and UI panel.
- [x] Keep the workflow local-only, non-production, non-authoritative, and bounded to existing typed local actions.

## Working-tree hygiene gate
- [x] Start from the current branch.
- [x] Keep edits limited to Phase 159 allowed surfaces.
- [x] Run final validation after all edits.
- [x] Commit Phase 159 changes.

## Allowed surfaces
- [x] `core/src/**`
- [x] `ui/src/**`
- [x] `CHANGELOG.md`
- [x] `checklists/current-phase.md`

## Code-production deliverable checklist
- [x] Rust-owned complete local workflow projection.
- [x] Rust-owned workflow step, status, error, evidence, boundary, and capability types.
- [x] Transport/local shell state carries the complete workflow projection.
- [x] TypeScript model carries Rust-shaped workflow projection.
- [x] Visible local operator workflow panel added.

## Complete workflow projection checklist
- [x] Deterministic projection for identical shell state.
- [x] Projection derived from `LocalOperatorShellState`.
- [x] Projection includes classification `local_beta_workflow_only`.
- [x] Projection covers provider setup through local candidate materialization and local evidence surfaces.

## Workflow step/status checklist
- [x] Provider adapter configured.
- [x] Adapter dry-run available/executed.
- [x] Constrained invocation completed.
- [x] Provider output pipeline projected.
- [x] Provider output validated and reviewed.
- [x] Staged proposal created and validated.
- [x] Candidate review projected.
- [x] Operator decision recorded.
- [x] Local candidate materialized.
- [x] Replay, evidence export, session package, history, and restore projected.

## Current blocker checklist
- [x] Missing adapter configuration blocks initial workflow.
- [x] Missing/rejected invocation blocks or rejects workflow.
- [x] Missing/rejected validation, review, staged proposal, staged validation, candidate review, operator decision, and materialization are surfaced.
- [x] Rejection reasons are summarized for drilldown.

## Evidence/package/replay/export summary checklist
- [x] Provider output pipeline summary.
- [x] Local candidate materialization summary.
- [x] Replay/status summary.
- [x] Local evidence export summary.
- [x] Session package summary.
- [x] Session history and restore summary.

## UI workflow panel checklist
- [x] Panel labeled “Complete local operator workflow”.
- [x] Workflow status, current step, next required step, and current blocking step render.
- [x] Ordered step list and statuses render.
- [x] Rejection/error drilldown renders.
- [x] Evidence/package/replay/export summaries render.
- [x] UI remains usable when workflow is blocked or rejected.

## Local-only/non-production boundary checklist
- [x] UI states: “Complete local workflow is local-only and non-production.”
- [x] UI states workflow completion does not approve readiness, release, deployment, public use, or production use.
- [x] UI states provider output remains untrusted unless a later bounded phase explicitly changes that.
- [x] UI states workflow status does not authorize actions.
- [x] UI states replay is not repaired and recovery is not promoted.

## No-authority checklist
- [x] No provider trust.
- [x] No production, readiness, release, deployment, or public-use approval.
- [x] No action execution.
- [x] No replay repair or recovery promotion.
- [x] No provider execution expansion, arbitrary command execution, shell execution, network/cloud execution, secret handling, production persistence, release artifact, installer, update-channel, signing, publishing, or deployment behavior.

## Rust test checklist
- [x] Initial workflow projection.
- [x] Determinism and non-mutation.
- [x] Complete happy path after local candidate materialization.
- [x] Blocked/rejected workflow states.
- [x] Transport carries workflow projection.
- [x] No-authority boundaries.

## TypeScript test checklist
- [x] Visible workflow panel.
- [x] Ordered step rendering.
- [x] Blocked and rejected state rendering.
- [x] Happy-path deterministic rendering.
- [x] Required no-authority wording.

## Phase 160 handoff checklist
- [x] Phase 160 remains the next production-path alignment checkpoint.
- [x] No roadmap files changed.
- [x] Production-path alignment, readiness, release, deployment, signing, publishing, installer, update-channel, public-use, provider trust, action execution, replay repair, and recovery promotion remain deferred.

## Validation checklist
- [x] `CARGO_TARGET_DIR=/tmp/ajentic-phase-159-target ./scripts/check.sh` (dirty-tree gate reported expected uncommitted Phase 159 changes before commit)
- [x] `git diff --check`
- [x] `git status --short`
- [x] `cd ui && npm run typecheck`
- [x] `cd ui && npm run lint`
- [x] `cd ui && npm run build && rm -rf dist`
- [x] `cd ui && npm run test:api`
- [x] `CARGO_TARGET_DIR=/tmp/ajentic-phase-159-target cargo test --manifest-path core/Cargo.toml --all-targets`
- [x] `cd ui && timeout 5 npm run dev`
- [x] Workflow scan.
- [x] Workflow boundary scan.
- [x] Forbidden-label scan.
- [x] Unsafe execution scan.
- [x] No-persistence/release scan.
- [x] Changed-file source guard.
- [x] No-roadmap-drift guard.

## Deferred items
- [x] New provider execution capability.
- [x] Arbitrary provider execution, shell command execution, network/cloud execution, secret handling, production persistence, durable workflow/candidate/provider storage, action execution, replay repair, recovery promotion, release/deployment/signing/publishing/installer/update behavior, and readiness/public-use/production approval.

## Validation log
- `CARGO_TARGET_DIR=/tmp/ajentic-phase-159-target ./scripts/check.sh` stopped at the expected dirty-tree gate before commit.
- `git diff --check` passed.
- `git status --short` showed only Phase 159 allowed files changed before commit.
- `cd ui && npm run typecheck` passed.
- `cd ui && npm run lint` passed.
- `cd ui && npm run build && rm -rf dist` passed.
- `cd ui && npm run test:api` passed.
- `CARGO_TARGET_DIR=/tmp/ajentic-phase-159-target cargo test --manifest-path core/Cargo.toml --all-targets` passed.
- `cd ui && timeout 5 npm run dev` started `http://127.0.0.1:5173`; timeout stop is expected.
- Workflow, boundary, forbidden-label, unsafe execution, no-persistence/release, changed-file, and no-roadmap-drift scans completed; matches are Phase 159 markers or existing prohibition/test/historical markers.

## Zero-drift checklist
- [x] Changes match Phase 159 workflow consolidation scope.
- [x] Rust owns workflow classification/projection.
- [x] TypeScript remains non-authoritative.
- [x] Roadmap files are not modified.
- [x] CHANGELOG entry matches actual scope.
- [x] Checklist describes Phase 159 procedural truth.
- [x] Phase 160 remains the next production-path alignment checkpoint.
