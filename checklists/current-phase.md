---
truth_dimension: procedural
scope: current_phase
authority_level: authoritative
mutation_path: checklist_revision
---

# Current phase checklist

Phase 157 - Real Provider Output Pipeline Integration.

## Phase goal
- [x] Route constrained local provider invocation output into the existing provider-output pipeline.
- [x] Preserve untrusted/descriptive, non-candidate, non-promoted, local-only boundaries.

## Working-tree hygiene gate
- [x] Keep changes on allowed Phase 157 code, UI, changelog, and checklist surfaces.
- [x] Do not modify roadmap, governance, architecture, release, installer, update-channel, publishing, or production persistence surfaces.

## Allowed surfaces
- [x] `core/src/**`
- [x] `ui/src/**`
- [x] `CHANGELOG.md`
- [x] `checklists/current-phase.md`

## Code-production deliverable checklist
- [x] Rust-owned provider output pipeline projection types.
- [x] Rust-owned stage status, validation status, error, boundary, and effect types.
- [x] Local shell state and transport response include pipeline integration projection.
- [x] TypeScript Rust-shaped pipeline projection and local shell model.
- [x] Visible UI panel for provider output pipeline integration.

## Invocation-to-pipeline bridge checklist
- [x] Project constrained invocation result into provider execution/result projection.
- [x] Preserve invocation result ID and provider execution/result linkage.
- [x] Preserve `untrusted_descriptive` and not-candidate/not-promoted status.
- [x] Reject missing, rejected, drifted, or claim-bearing invocation output fail-closed.

## Stage-ordering checklist
- [x] Explicit stage order starts with invocation output projection.
- [x] Provider execution/result projection precedes provider output validation.
- [x] Validation precedes review, staging, staged validation, candidate review, and operator decision.
- [x] Deterministic projection for identical shell state.

## No-skip boundary checklist
- [x] Reject stage order drift.
- [x] Reject completed downstream stages after incomplete upstream stages.
- [x] Block downstream stages when validation, review, staging, staged validation, candidate review, or operator decision is missing.

## UI pipeline panel checklist
- [x] Render `Provider output pipeline` panel.
- [x] Render source kind, invocation result ID, current stage, next required stage, status list, and blocked/rejected reasons.
- [x] Render validation, review, staged proposal, staged validation, candidate review, and operator decision status.
- [x] Show required untrusted/descriptive, no-candidate, no-skip, later materialization, and no trust/readiness/release/deployment/public-use wording.

## No-candidate/no-trust checklist
- [x] Pipeline integration does not create candidate output.
- [x] Pipeline integration does not materialize candidates.
- [x] Pipeline integration does not approve provider output or provider trust.
- [x] Operator decision remains scoped to validated staged proposals.

## No-effect boundary checklist
- [x] No additional provider execution capability.
- [x] No arbitrary command execution, shell execution, process spawn, sockets, or secret reads.
- [x] No decision ledger append outside existing operator decision boundary.
- [x] No replay repair, recovery promotion, export promotion, action execution, production persistence, release, deployment, publishing, signing, installer, update-channel, readiness, or public-use effect.

## Rust test checklist
- [x] Initial not-started projection.
- [x] Valid invocation output integration.
- [x] Missing/rejected/drifted invocation output.
- [x] Stage ordering and no-skip rejection.
- [x] Determinism and no-effect boundaries.

## TypeScript test checklist
- [x] Visible accepted pipeline rendering.
- [x] Blocked/rejected stage rendering.
- [x] Next required stage rendering.
- [x] Forbidden labels and shortcut controls absent.
- [x] Deterministic rendering.

## Phase 158 handoff checklist
- [x] Phase 158 remains the next code-production phase for local candidate materialization.
- [x] Phase 157 does not perform candidate materialization.

## Validation checklist
- [x] Run `CARGO_TARGET_DIR=/tmp/ajentic-phase-157-target ./scripts/check.sh` (blocked by dirty-tree preflight after Phase 157 edits).
- [x] Run `git diff --check`.
- [x] Run `git status --short`.
- [x] Run required direct UI and Rust validation commands because `check.sh` stopped at dirty-tree preflight.
- [x] Run required scans.

## Deferred items
- [ ] Local candidate materialization remains deferred to Phase 158.
- [ ] Readiness, release, deployment, publishing, signing, installer, update-channel, and public-use behavior remain deferred.

## Validation log
- `CARGO_TARGET_DIR=/tmp/ajentic-phase-157-target ./scripts/check.sh` stopped at the initial repository cleanliness gate because Phase 157 files were modified.
- `git diff --check` passed.
- `git status --short` showed only allowed Phase 157 surfaces.
- `CARGO_TARGET_DIR=/tmp/ajentic-phase-157-target cargo test --manifest-path core/Cargo.toml --all-targets` passed.
- `cd ui && npm run typecheck`, `npm run lint`, `npm run build && rm -rf dist`, and `npm run test:api` passed.
- `cd ui && timeout 5 npm run dev` printed the local browser URL; timeout stopped the long-running server.
- Required scans were run; forbidden-label scan matched historical/prohibition/test strings only.

## Zero-drift checklist
- [x] Roadmap files are not modified.
- [x] CHANGELOG entry matches Phase 157 scope.
- [x] Checklist describes Phase 157 procedural truth.
