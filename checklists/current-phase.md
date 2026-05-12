---
truth_dimension: procedural
authority_level: authoritative
mutation_path: checklist_revision
---

# Current Phase Checklist - Phase 151

## Phase name
- [x] Phase 151 - Persistent Local Session Package.

## Phase goal
- [x] Produce a deterministic local session package artifact from current local operator shell state.
- [x] Support explicit caller-provided path write/read helpers.
- [x] Project package and read-back status through the local shell/UI surface.

## Working-tree hygiene gate
- [x] Start from the current branch.
- [x] Limit changes to Phase 151 code-production surfaces.
- [x] Do not modify roadmap files.
- [x] Remove generated UI build output after validation.
- [x] Confirm staged files match allowed Phase 151 surfaces before commit.

## Allowed surfaces
- [x] `core/src/**`.
- [x] `ui/src/**`.
- [x] `CHANGELOG.md`.
- [x] `checklists/current-phase.md`.

## Code-production deliverable checklist
- [x] Add executable Rust local session package derivation.
- [x] Add deterministic serialization and package identity.
- [x] Add explicit local write/read helpers.
- [x] Add read-back validation projection.
- [x] Add UI-visible package status panel.

## Local session package checklist
- [x] Include provider configuration projection when available.
- [x] Include provider execution result projection when available.
- [x] Include provider output validation and review projections.
- [x] Include staged proposal and staged validation projections.
- [x] Include candidate review and operator decision projections.
- [x] Include local decision ledger, replay/status projection, local session evidence export, and Phase 150 handoff context.
- [x] Include absence markers for unavailable or prohibited sections.

## Package validation checklist
- [x] Require package ID and package version.
- [x] Require `local_session_package_only` classification.
- [x] Require `non_production` production classification.
- [x] Validate deterministic ID/content consistency.
- [x] Reject missing absence markers.
- [x] Reject explicit release, deployment, readiness, signing, publishing, installer, update-channel, provider trust, candidate approval, action execution, or persistence-authority claims.

## Explicit write/read helper checklist
- [x] Write helper requires a caller-provided path.
- [x] Write helper validates the package before writing.
- [x] Read helper requires a caller-provided path.
- [x] Helpers do not create release, deployment, installer, update-channel, signing, publishing, or public artifacts.

## Read-back validation checklist
- [x] Parse package content from the caller-provided path.
- [x] Validate parsed content fail-closed.
- [x] Project read-back validation status.
- [x] Do not repair replay or promote recovery.

## UI package projection checklist
- [x] TypeScript shell state carries Rust-shaped local session package projection.
- [x] UI renders package status, ID/version, classification, validation, read-back status, included sections, and absence markers.
- [x] UI states local-only/non-production package boundaries.
- [x] UI does not label the package as release, deployment, or readiness evidence.

## Local-only/non-production boundary checklist
- [x] No default filesystem persistence.
- [x] No automatic save.
- [x] No background persistence or services.
- [x] No production persistence claim.
- [x] No remote sync or provider calls.
- [x] No action execution.

## Rust test checklist
- [x] Cover initial no-package projection.
- [x] Cover deterministic derivation, package ID, and serialization.
- [x] Cover included shell sections and absence markers.
- [x] Cover malformed, missing, invalid, drifted, and claim-bearing rejection.
- [x] Cover explicit temp-path write/read and read-back validation.

## TypeScript test checklist
- [x] Cover initial UI package projection.
- [x] Cover deterministic repeated package projection.
- [x] Cover local-only/non-production boundary wording.

## Phase 152 handoff checklist
- [x] Phase 152 remains the next code-production phase for session history and restore UI.
- [x] Phase 151 restore/read-back validates structure only.
- [x] Durable history and richer restore UI remain deferred.

## Validation checklist
- [x] Run `CARGO_TARGET_DIR=/tmp/ajentic-phase-151-target ./scripts/check.sh` after commit-clean state; dirty-tree preflight was observed before commit.
- [x] Run `git diff --check`.
- [x] Run `git status --short`.
- [x] Run direct UI commands.
- [x] Run direct Rust tests.
- [x] Run local UI smoke test.
- [x] Run package/filesystem/boundary scans.
- [x] Run changed-file and no-roadmap-drift guards.

## Deferred items
- [x] Session history and restore UI depth deferred to Phase 152.
- [x] Real local provider adapter contract deferred to Phase 153.
- [x] Candidate materialization deferred to Phase 158.
- [x] Complete local operator workflow deferred to Phase 159.

## Validation log
- [x] Final validation completed; check.sh requires a clean worktree and is rerun after commit.

## Zero-drift checklist
- [x] CHANGELOG entry matches Phase 151 implementation.
- [x] Checklist reflects procedural truth for Phase 151.
- [x] Roadmap files remain unchanged.
- [x] Local session package remains local-only and non-production.
