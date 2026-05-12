---
truth_dimension: procedural
authority_level: authoritative
mutation_path: checklist_revision
---

# Current Phase Checklist - Phase 147

## Phase name
- [x] Phase 147 - Candidate Conversion Validation.

## Phase goal
- [x] Add local Rust-owned validation for existing staged candidate-conversion proposals.
- [x] Validate proposal shape and source linkage only.
- [x] Keep staged proposals untrusted, not approved, not executable, not persistent, and not candidate output.

## Working-tree hygiene gate
- [x] Review existing local shell, UI, changelog, and checklist surfaces before editing.
- [x] Keep changes limited to Phase 147 allowed code-production surfaces.
- [x] Remove generated UI build output after local checks.

## Allowed surfaces
- [x] `core/src/**` for Rust validation types, projection, request handling, and tests.
- [x] `ui/src/**` for Rust-shaped TypeScript types, transport adapter, UI rendering, and tests.
- [x] `CHANGELOG.md` for the active Phase 147 entry.
- [x] `checklists/current-phase.md` for procedural truth.

## Code-production deliverable checklist
- [x] Add executable staged proposal validation.
- [x] Expose validation through typed local shell transport.
- [x] Render validation results in the browser UI.
- [x] Add Rust and TypeScript coverage for accepted and rejected validation paths.

## Rust staged proposal validation checklist
- [x] Add `StagedCandidateConversionValidationProjection`.
- [x] Add `StagedCandidateConversionValidationRequest`.
- [x] Add validation status, reason, materialization, operator-decision, and boundary status types.
- [x] Initialize local shell state with `not_validated` validation projection.

## Source-eligibility integrity checklist
- [x] Validate only existing staged proposals.
- [x] Require `reviewable_untrusted` source provider output.
- [x] Reject missing provider output validation.
- [x] Reject inconsistent provider output validation.
- [x] Reject missing or malformed provider execution result projection.

## Trust boundary preservation checklist
- [x] Preserve `untrusted_source`.
- [x] Preserve `not_trusted`.
- [x] Preserve `not_approved`.
- [x] Reject trust, approval, safety, readiness, release, deployment, public-use, action, persistence, execution, candidate-creation, and candidate-materialization claims.

## Promotion boundary preservation checklist
- [x] Do not create candidate output.
- [x] Do not replace candidate output.
- [x] Do not approve or promote provider output.
- [x] Do not expose staged proposal approve/reject controls.

## No-effect boundary preservation checklist
- [x] Validate no decision ledger effect.
- [x] Validate no replay effect.
- [x] Validate no export effect.
- [x] Validate no provider configuration effect.
- [x] Validate no provider execution effect.
- [x] Validate no action, persistence, readiness, release, or deployment effect.

## Proposal-shape integrity checklist
- [x] Require boundary flags.
- [x] Reject boundary flag drift.
- [x] Require no-effect fields.
- [x] Reject no-effect drift.
- [x] Require future-phase markers.

## Linkage and determinism integrity checklist
- [x] Validate deterministic proposal ID.
- [x] Validate provider execution result ID linkage.
- [x] Validate source validation status linkage.
- [x] Validate source reviewability status linkage.
- [x] Validate source candidate-boundary status linkage.
- [x] Keep validation deterministic for identical input.

## Pipeline integrity checklist
- [x] Preserve `reviewable_untrusted provider output -> staged proposal -> validated staged proposal` only.
- [x] State future review boundary is required.
- [x] State operator decision is unavailable in Phase 147.
- [x] State candidate materialization requires a future phase.

## UI/API semantics checklist
- [x] Add typed transport request for staged proposal validation.
- [x] Render validation status and reasons.
- [x] Render proposal/source/linkage details.
- [x] Render materialization, future review, operator-decision, trust, approval, and no-effect boundaries.
- [x] Avoid UI/API copy that implies candidate creation, approval, readiness, trust, or safety.

## Ladder and authority boundary checklist
- [x] Do not add readiness approval.
- [x] Do not add release approval.
- [x] Do not add deployment/public-use approval.
- [x] Do not add production persistence.

## TypeScript transport projection checklist
- [x] Add Rust-shaped validation projection types.
- [x] Add validation request type.
- [x] Add transport adapter method.
- [x] Preserve rejected validation response without committing rejected state to transport.

## UI staged validation checklist
- [x] Show initial `not_validated` state.
- [x] Show `staged_proposal_shape_valid` state.
- [x] Show rejected validation state in tests.
- [x] Show required Phase 147 boundary wording.

## Rust test checklist
- [x] Cover initial validation projection.
- [x] Cover valid validation.
- [x] Cover missing proposal rejection.
- [x] Cover deterministic linkage rejection.
- [x] Cover source and no-effect drift rejection.
- [x] Cover claim-bearing proposal rejection.
- [x] Cover no mutation of candidate, ledger, replay, export, provider configuration, or provider execution.

## TypeScript test checklist
- [x] Cover visible validation results.
- [x] Cover rejected validation states.
- [x] Cover forbidden labels and no authority leakage.
- [x] Cover deterministic rendering.

## Local-only/non-production boundary checklist
- [x] No arbitrary local model execution.
- [x] No cloud model execution.
- [x] No network socket behavior.
- [x] No shell command execution behavior.
- [x] No durable proposal or validation storage.

## Phase 148 handoff checklist
- [x] Phase 148 remains future candidate review surface work.
- [x] No operator candidate decision is introduced.
- [x] No candidate materialization is introduced.

## Validation checklist
- [x] Run Rust tests.
- [x] Run TypeScript tests.
- [x] Run full local check script.
- [x] Run scans requested by Phase 147.

## Deferred items
- [x] Candidate review surface remains deferred to Phase 148.
- [x] Operator candidate decision remains deferred to Phase 149.
- [x] Next 0/5 alignment checkpoint remains Phase 150.

## Validation log
- [x] `CARGO_TARGET_DIR=/tmp/ajentic-phase-147-target ./scripts/check.sh`
- [x] `git diff --check`
- [x] `git status --short`
- [x] `cd ui && npm run typecheck`
- [x] `cd ui && npm run lint`
- [x] `cd ui && npm run build && rm -rf dist`
- [x] `cd ui && npm run test:api`
- [x] `CARGO_TARGET_DIR=/tmp/ajentic-phase-147-target cargo test --manifest-path core/Cargo.toml --all-targets`
- [x] `cd ui && timeout 5 npm run dev`

## Zero-drift checklist
- [x] Changelog matches actual code changes.
- [x] Checklist matches Phase 147 procedural truth.
- [x] No generated artifacts remain staged.
- [x] No readiness, release, deployment, production, public-use, trust, approval, or candidate materialization authority is added.
