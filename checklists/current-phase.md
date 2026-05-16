---
truth_dimension: procedural
authority_level: authoritative
mutation_path: checklist_revision
---

# Current Phase Checklist: Phase 171

## Phase name
- [x] Phase 171 - Release Candidate Preparation Contract.

## Phase goal
- [x] Add a Rust-owned release-candidate preparation contract that organizes committed local beta, controlled-trial, help, validation, package, restore, replay, and evidence surfaces.
- [x] Expose the projection in the local UI without approving release readiness.

## Working-tree hygiene gate
- [x] Start from `git status --short`.
- [x] Restrict edits to Phase 171 allowed code, UI, checklist, and changelog surfaces.
- [x] Keep generated artifacts cleaned after validation.

## Allowed surfaces
- [x] `core/src/api/release_candidate_preparation.rs`.
- [x] Thin local shell state/module export integration.
- [x] `ui/src/**`.
- [x] `CHANGELOG.md`.
- [x] `checklists/current-phase.md`.

## Code-production deliverable checklist
- [x] Rust-owned release-candidate preparation contract exists.
- [x] Local shell state exposes the preparation projection.
- [x] Browser UI renders a release candidate preparation panel.

## Dedicated module checklist
- [x] Implement preparation types, derivation, validation, ordering, blockers, and boundaries in `core/src/api/release_candidate_preparation.rs`.
- [x] Do not add large preparation logic to `core/src/api/local_operator_shell.rs`.

## Thin shell integration checklist
- [x] Add one projection field to local shell state.
- [x] Refresh the projection by calling the dedicated module helper.
- [x] Re-export the module through the API surface.

## Release-candidate preparation contract checklist
- [x] Add contract, projection, status, classification, capability, source linkage, input snapshot, evidence, missing evidence, blocker, validation status, validation error, and boundary status types.
- [x] Add deterministic preparation ID generation.

## Evidence category checklist
- [x] Include all required closed evidence categories.
- [x] Keep category ordering deterministic.
- [x] Do not silently omit unrepresented categories.

## Missing evidence checklist
- [x] Derive missing evidence deterministically.
- [x] Fail closed when required evidence is missing.

## Blocker checklist
- [x] Derive blocked and rejected evidence blockers deterministically.
- [x] Fail closed when required evidence is blocked or rejected.

## Source linkage checklist
- [x] Link each category back to its local shell or UI source surface.
- [x] Render source linkage in the UI.

## Validation/fail-closed checklist
- [x] Reject readiness, release, deployment, public-use, and production-use claims.
- [x] Reject signing, publishing, installer, and update-channel claims.
- [x] Reject provider-trust, action-authorization, replay-repair, and recovery-promotion claims.

## UI preparation panel checklist
- [x] Render preparation status and preparation ID.
- [x] Render evidence category counts and category statuses.
- [x] Render missing evidence, blockers, source linkage, and boundaries.
- [x] Avoid publish, sign, deploy, release, installer, update-channel, approval, or public-distribution controls.

## No-release/no-readiness boundary checklist
- [x] State that release-candidate preparation is not release readiness.
- [x] State that the contract creates no release artifacts.
- [x] State that the contract approves no Release Candidate status.
- [x] State provider output remains untrusted and no action authorization is granted.

## Rust test checklist
- [x] Cover complete evidence mapping.
- [x] Cover missing, blocked, and rejected evidence.
- [x] Cover deterministic projection and no-authority boundaries.

## TypeScript test checklist
- [x] Cover visible preparation panel.
- [x] Cover category status, missing evidence, blockers, source linkage, deterministic rendering, and boundary wording.
- [x] Cover forbidden-label absence.

## Phase 172 handoff checklist
- [x] Phase 172 remains the next code-production phase for release artifact dry package assembly.
- [x] Phase 171 creates no release artifacts.

## Validation checklist
- [x] Run full required validation after final edits.
- [x] Confirm staged files match allowed surfaces.

## Deferred items
- [x] Release artifact dry package assembly remains deferred to Phase 172.
- [x] Signing, publishing, installers, update channels, deployment, and public distribution remain deferred.

## Validation log
- [x] Record final validation commands in the agent final response.

## Zero-drift checklist
- [x] No roadmap files changed.
- [x] No Phase 170.5 audit file changed.
- [x] No release/deployment/signing/publishing behavior added.
- [x] No provider execution, action authorization, replay repair, or recovery promotion added.
