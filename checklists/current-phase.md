---
truth_dimension: procedural
authority_level: authoritative
mutation_path: checklist_revision
---

# Current Phase Checklist - Phase 146 Candidate Conversion Staging Boundary

## Phase name
- [x] Phase 146 - Candidate Conversion Staging Boundary.

## Phase goal
- [x] Add local Rust-owned staged candidate-conversion proposal creation from `reviewable_untrusted` provider output only.
- [x] Keep staged proposals proposal-only, untrusted, not approved, not candidate material, not executable, not persistent, and not promoted.

## Working-tree hygiene gate
- [x] Start from the current Phase 145-aligned tree.
- [x] Keep changes limited to Phase 146 allowed code, UI, tests, checklist, and changelog surfaces.

## Allowed surfaces
- [x] `core/src/**` for Rust local shell types, creation, projection, transport, and tests.
- [x] `ui/src/**` for Rust-shaped TypeScript types, transport adapter, UI rendering, and tests.
- [x] `CHANGELOG.md` and `checklists/current-phase.md` for active historical/procedural truth.

## Code-production deliverable checklist
- [x] Produce usable, testable code.
- [x] Expose staged proposal creation through the local shell transport.
- [x] Render staged proposal creation and inspection in the local UI shell.

## Rust staged proposal checklist
- [x] Add `StagedCandidateConversionProposal` Rust-owned type.
- [x] Add staged proposal status, boundary, trust, effect, source eligibility, request, error, and projection types.
- [x] Add deterministic staged proposal ID generation.
- [x] Add deterministic linkage to provider execution result and validation projection.

## Source eligibility checklist
- [x] Allow staged proposal creation only from `reviewable_untrusted` provider output.
- [x] Reject rejected provider output.
- [x] Reject `not_validated` provider output.
- [x] Reject `validation_not_applicable` provider output.
- [x] Reject `invalid_validation_input` provider output.
- [x] Reject missing provider execution result.
- [x] Reject missing or inconsistent validation projection.

## Trust and approval creep checklist
- [x] Reject request claims carrying trust, approval, safety, readiness, release, deployment, public-use, action, execution, persistence, or candidate-creation authority.
- [x] Preserve `untrusted_source`, `not_trusted`, and `not_approved` staged proposal labels.

## Scope-of-staging checklist
- [x] Record `staging_only_not_candidate_material`.
- [x] Record `candidate_conversion_not_performed`.
- [x] Record `validation_required_in_future_phase`.
- [x] Record `approval_not_available_in_phase_146`.

## Data-shape and boundary encoding checklist
- [x] Encode `source_boundary: provider_output_validation_phase_143`.
- [x] Encode `proposal_boundary: staged_candidate_conversion_phase_146`.
- [x] Encode source provider kind, execution result ID, validation status, reviewability status, and candidate-boundary status.

## Pipeline integrity checklist
- [x] Provider output remains untrusted/descriptive.
- [x] `reviewable_untrusted` remains inspection-only and not candidate material.
- [x] Phase 147 remains staged proposal validation.
- [x] Phase 148 remains candidate review surface.
- [x] Phase 149 remains operator candidate decision boundary.
- [x] Phase 150 remains the next 0/5 alignment checkpoint.

## UI/API semantics checklist
- [x] TypeScript transport can request staged proposal creation.
- [x] UI does not create candidate output.
- [x] UI does not expose staged proposal approve/reject controls.
- [x] UI does not mark staged proposals trusted, safe, ready, approved, or candidate material.

## No-effect boundary checklist
- [x] No decision ledger effect.
- [x] No replay effect.
- [x] No export effect.
- [x] No provider configuration effect.
- [x] No provider execution effect.
- [x] No action effect.
- [x] No persistence effect.
- [x] No readiness, release, or deployment effect.

## TypeScript transport projection checklist
- [x] Add Rust-shaped staged proposal projection types.
- [x] Include staged proposal projection in shell state.
- [x] Return staged proposal projection in transport responses.

## UI staged proposal checklist
- [x] Render initial no-proposal state.
- [x] Render `Create staged conversion proposal`.
- [x] Render proposal ID and source linkage when present.
- [x] Render required Phase 146 wording.
- [x] Keep repeated rendering deterministic.

## Rust test checklist
- [x] Cover initial no-proposal state.
- [x] Cover valid proposal creation.
- [x] Cover rejected and missing sources.
- [x] Cover deterministic proposal identity and linkage.
- [x] Cover no-effect boundaries and projection drift rejection.

## TypeScript test checklist
- [x] Cover visible staged proposal behavior.
- [x] Cover forbidden shortcut rejection.
- [x] Cover source rejection edge states.
- [x] Cover deterministic rendering.

## Local-only/non-production boundary checklist
- [x] No provider kind expansion.
- [x] No arbitrary local model execution.
- [x] No cloud model execution.
- [x] No network sockets.
- [x] No filesystem persistence or durable proposal storage.
- [x] No release, installer, signing, publishing, deployment, public-use, or readiness approval behavior.

## Phase 147 handoff checklist
- [x] Staged proposal validation remains deferred to Phase 147.
- [x] Candidate review remains deferred to Phase 148.
- [x] Operator candidate decision remains deferred to Phase 149.

## Validation checklist
- [x] Run `CARGO_TARGET_DIR=/tmp/ajentic-phase-146-target ./scripts/check.sh`.
- [x] Run `git diff --check`.
- [x] Run `git status --short`.
- [x] Run UI direct commands when needed.
- [x] Run Rust tests directly when needed.
- [x] Run local dev smoke test.
- [x] Run required boundary scans.

## Deferred items
- [x] Candidate materialization.
- [x] Candidate replacement.
- [x] Staged proposal validation.
- [x] Staged proposal approval.
- [x] Operator candidate decision.
- [x] Provider output trust or approval.
- [x] Durable persistence and release/deployment behaviors.

## Validation log
- [x] Full validation completed after final edits.
- [x] No masked failures.
- [x] Generated artifacts cleaned.

## Zero-drift checklist
- [x] CHANGELOG entry matches intended Phase 146 diff.
- [x] Staged files remain within allowed surfaces.
- [x] Forbidden labels appear only as historical/prohibition/test strings where already present or explicitly scanned.
- [x] No readiness, release, deployment, provider-output trust, provider-output approval, candidate conversion, candidate approval, local model execution, cloud model execution, signing, publishing, or public-use approval is introduced.
