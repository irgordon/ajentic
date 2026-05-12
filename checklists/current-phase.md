---
truth_dimension: procedural
phase: 152
authority_level: authoritative
mutation_path: checklist_revision
---

# Current Phase Checklist - Phase 152

## Phase name
- [x] Phase 152 - Session History and Restore UI.

## Phase goal
- [x] Make Phase 151 local session packages visible through local session history and restore preview UI.
- [x] Keep restore as local-only projection and continuity support.

## Working-tree hygiene gate
- [x] Review allowed Phase 152 surfaces before editing.
- [x] Do not modify roadmap, governance, architecture, release, installer, update-channel, or publishing surfaces.

## Allowed surfaces
- [x] `core/src/**`
- [x] `ui/src/**`
- [x] `CHANGELOG.md`
- [x] `checklists/current-phase.md`

## Code-production deliverable checklist
- [x] Add Rust-owned session history projection types.
- [x] Add Rust-owned restore candidate, read-back, status, validation, and boundary types.
- [x] Expose local shell state history and restore projections.
- [x] Add visible UI capability for history, selected package details, restore validation, and restore preview.

## Session history checklist
- [x] Initial state exposes empty history.
- [x] History projection is derived from explicit package entries only.
- [x] History projection is deterministic for identical explicit metadata.
- [x] No automatic filesystem scanning is added.

## Restore projection checklist
- [x] Restore request uses explicit package payload or caller-provided package path helper.
- [x] Valid package produces deterministic restore preview/projection.
- [x] Restore preview includes package ID, version, classifications, sections, absence markers, and boundaries.
- [x] Restore preview does not mutate current shell state.

## Read-back validation checklist
- [x] Read-back validation checks package structure only.
- [x] Read-back validation is not restore authority; read-back validation is not restore authority.
- [x] Malformed package content rejects fail-closed.
- [x] Missing required package sections or absence markers reject fail-closed.

## UI restore panel checklist
- [x] Render `Session history` panel.
- [x] Render `Local session restore` panel.
- [x] Render `Restore preview` panel.
- [x] Render selected package details, status, read-back validation, restore status, errors, sections, and absence markers.
- [x] Render required local-only and non-production wording.

## Local-only/non-production boundary checklist
- [x] Restore remains local-only and non-production.
- [x] No production persistence claim is introduced.
- [x] No readiness, release, deployment, or public-use approval is introduced.
- [x] No remote sync or background restore is active; no remote sync and no background restore are active.

## No-recovery/no-replay-repair checklist
- [x] Restore preview does not repair replay.
- [x] Restore preview does not promote recovery.
- [x] Replay repair and recovery promotion claims reject fail-closed.

## Rust test checklist
- [x] Initial empty history and restore state.
- [x] Valid explicit package appears in deterministic history.
- [x] Valid package produces deterministic restore preview/projection.
- [x] Malformed or incomplete package rejects.
- [x] Invalid classification, invalid production classification, missing marker, and authority claims reject.
- [x] Unreadable caller-provided path rejects.

## TypeScript test checklist
- [x] Initial session history and restore projections render expected state.
- [x] Explicit package details project into history.
- [x] Restore preview and rejection state render.
- [x] Local-only, non-production, no-recovery, no-replay-repair, and no-approval wording is present.
- [x] Repeated rendering/projection is deterministic.

## Phase 153 handoff checklist
- [x] Phase 153 remains next code-production phase for real local provider adapter contract.
- [x] No Phase 153 implementation is started in Phase 152.

## Validation checklist
- [x] `CARGO_TARGET_DIR=/tmp/ajentic-phase-152-target ./scripts/check.sh`
- [x] `git diff --check`
- [x] `git status --short`
- [x] `cd ui && npm run typecheck`
- [x] `cd ui && npm run lint`
- [x] `cd ui && npm run build && rm -rf dist`
- [x] `cd ui && npm run test:api`
- [x] `CARGO_TARGET_DIR=/tmp/ajentic-phase-152-target cargo test --manifest-path core/Cargo.toml --all-targets`
- [x] `cd ui && timeout 5 npm run dev`
- [x] Restore scans and boundary scans complete.

## Deferred items
- [x] Production persistence remains deferred.
- [x] Recovery promotion remains deferred.
- [x] Replay repair remains deferred.
- [x] Remote sync and background restore remain deferred.
- [x] Release/deployment/signing/publishing behavior remains deferred.

## Validation log
- Full Rust tests passed with `CARGO_TARGET_DIR=/tmp/ajentic-phase-152-target cargo test --manifest-path core/Cargo.toml --all-targets`.
- TypeScript typecheck, lint, build, and API behavior tests passed.
- Local dev smoke test printed `http://127.0.0.1:5173`; timeout stopped the long-running server.
- `git diff --check`, restore scans, filesystem scan, unsafe execution scan, readiness/release scan, changed-file guard, and no-roadmap-drift guard completed.
- `./scripts/check.sh` requires a clean worktree, so the final pass is run after committing Phase 152 changes.

## Zero-drift checklist
- [x] CHANGELOG entry matches Phase 152 changes.
- [x] Roadmap files are not modified.
- [x] Forbidden restore/readiness labels are not introduced as approvals.
- [x] Generated UI artifacts are not kept.
