---
truth_dimension: procedural
scope: current_phase
authority_level: authoritative
mutation_path: checklist_revision
---

# Current phase checklist

Phase 158 - Local Candidate Materialization.

## Phase goal
- [x] Add Rust-owned local candidate materialization after constrained invocation, provider-output validation/review, staged proposal, staged validation, candidate review, and approved validated staged-proposal operator decision.
- [x] Keep materialized output local-only, non-production, untrusted-provider-output carrying, and no-action/no-readiness/no-release/no-deployment/no-public-use.

## Working-tree hygiene gate
- [x] Start from the current branch.
- [x] Keep edits limited to Phase 158 allowed surfaces.
- [x] Run final validation after all edits.
- [x] Commit Phase 158 changes.

## Allowed surfaces
- [x] `core/src/**`
- [x] `ui/src/**`
- [x] `CHANGELOG.md`
- [x] `checklists/current-phase.md`

## Code-production deliverable checklist
- [x] Rust-owned materialization request type.
- [x] Rust-owned local candidate output projection type.
- [x] Rust-owned status and error types.
- [x] Rust-owned capability surface.
- [x] Deterministic candidate ID and content derivation.
- [x] Local shell/transport projection includes candidate materialization state.

## Candidate materialization checklist
- [x] Materializes only local candidate output.
- [x] Includes source staged proposal and operator decision linkage.
- [x] Includes `local_candidate_output_only`.
- [x] Includes `non_production_candidate`.
- [x] Carries `provider_output_remains_untrusted`.

## Preconditions checklist
- [x] Requires `reviewable_untrusted` provider output validation.
- [x] Requires provider output review.
- [x] Requires staged proposal.
- [x] Requires `staged_proposal_shape_valid`.
- [x] Requires candidate review surface.
- [x] Requires `approved_validated_staged_proposal`.
- [x] Rejects missing, rejected, invalid, incomplete, or drifted preconditions.

## Source linkage checklist
- [x] Links constrained invocation result ID.
- [x] Links provider execution result ID.
- [x] Links provider output validation/review status.
- [x] Links staged proposal ID.
- [x] Links staged validation status.
- [x] Links operator decision ID.
- [x] Rejects invocation, provider execution, staged proposal, operator decision, and source-linkage drift.

## No-provider-trust checklist
- [x] Provider output remains untrusted.
- [x] No provider trust approval is introduced.
- [x] Provider-output approval claims reject.

## No-readiness/release/deployment checklist
- [x] No production approval.
- [x] No readiness approval.
- [x] No release approval.
- [x] No deployment approval.
- [x] No public-use approval.
- [x] Readiness/release/deployment/public-use claims reject.

## UI materialization panel checklist
- [x] UI renders “Local candidate materialization” / “Local candidate output”.
- [x] UI displays status, candidate ID/content, classifications, source linkage, trust carry-forward, boundary markers, no-effect markers, and rejection reason.
- [x] UI disables materialization wording until all preconditions pass.
- [x] UI includes required local-only/non-production/untrusted/no-readiness/no-action/no-production wording.
- [x] UI avoids forbidden trust, safety, approval, readiness, action, publication, and deployment labels.

## No-effect boundary checklist
- [x] No provider execution expansion.
- [x] No arbitrary command, shell, process, or binary execution.
- [x] No network sockets or cloud calls.
- [x] No secret, token, credential, or environment reads.
- [x] No file writes or production persistence.
- [x] No provider configuration, execution, validation, staged proposal, staged validation, or operator decision mutation.
- [x] No replay repair, recovery promotion, export promotion, action execution, release, installer, update-channel, signing, publishing, deployment, public-use, or readiness approval.

## Rust test checklist
- [x] Initial not-materialized state.
- [x] Valid full pipeline materialization.
- [x] Deterministic candidate ID/content.
- [x] Missing/rejected/drifted preconditions.
- [x] Claim rejection.
- [x] Source linkage and no-effect boundaries.

## TypeScript test checklist
- [x] Visible materialization UI.
- [x] Accepted candidate projection rendering.
- [x] Rejected materialization state.
- [x] Prior accepted output preservation after rejected request.
- [x] Required no-authority wording and forbidden-label absence.

## Phase 159 handoff checklist
- [x] Phase 159 remains the next code-production phase for complete local operator workflow.
- [x] No roadmap files changed.

## Validation checklist
- [x] `CARGO_TARGET_DIR=/tmp/ajentic-phase-158-target ./scripts/check.sh`
- [x] `git diff --check`
- [x] `git status --short`
- [x] `cd ui && npm run typecheck`
- [x] `cd ui && npm run lint`
- [x] `cd ui && npm run build && rm -rf dist`
- [x] `cd ui && npm run test:api`
- [x] `CARGO_TARGET_DIR=/tmp/ajentic-phase-158-target cargo test --manifest-path core/Cargo.toml --all-targets`
- [x] `cd ui && timeout 5 npm run dev`
- [x] Materialization scan
- [x] Boundary scan
- [x] Forbidden-label scan
- [x] Pipeline/precondition scan
- [x] Unsafe execution scan
- [x] No-persistence/release scan
- [x] Changed-file source guard
- [x] No-roadmap-drift guard

## Deferred items
- [x] Complete local operator workflow remains Phase 159.
- [x] Production, readiness, release, deployment, public-use, installer, update-channel, signing, publishing, provider trust, action execution, replay repair, and recovery promotion remain deferred.

## Validation log
- `CARGO_TARGET_DIR=/tmp/ajentic-phase-158-target ./scripts/check.sh` initially stopped at the expected dirty-tree gate before commit; it is rerun after commit from a clean tree.
- `git diff --check` passed.
- `cd ui && npm run typecheck` passed.
- `cd ui && npm run lint` passed.
- `cd ui && npm run build && rm -rf dist` passed.
- `cd ui && npm run test:api` passed.
- `CARGO_TARGET_DIR=/tmp/ajentic-phase-158-target cargo test --manifest-path core/Cargo.toml --all-targets` passed.
- `cd ui && timeout 5 npm run dev` started `http://127.0.0.1:5173`; timeout stop is expected.
- Materialization, boundary, forbidden-label, pipeline/precondition, unsafe execution, no-persistence/release, changed-file, and no-roadmap-drift scans completed; matches are existing prohibition/test/historical markers or Phase 158 local-only markers.

## Zero-drift checklist
- [x] Changes match Phase 158 scope.
- [x] Local candidate materialization is Rust-owned.
- [x] TypeScript remains non-authoritative.
- [x] Roadmap files are not modified.
- [x] CHANGELOG entry matches actual scope.
- [x] Checklist describes Phase 158 procedural truth.
- [x] Phase 159 remains the next code-production phase.
