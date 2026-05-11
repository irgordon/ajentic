---
truth_dimension: procedural
authority_level: authoritative
mutation_path: checklist_revision
---

# Current Phase Checklist - Phase 137 Replay Projection for Local Decisions

## Phase goal
- [x] Derive a deterministic Rust-owned local replay/status projection from the in-memory local decision ledger.
- [x] Render the replay/status projection in the local browser UI after initial, stub-run, approve, and reject flows.

## Working-tree hygiene gate
- [x] Keep changes limited to Phase 137 code-production, tests, changelog, and checklist surfaces.
- [x] Do not modify governance, architecture, roadmap, release, installer, update-channel, signing, publishing, deployment, archived changelog, or AGENTS surfaces.

## Allowed surfaces
- [x] `core/src/**`
- [x] `core/tests/**` or `tests/**`
- [x] `ui/src/**`
- [x] `ui/index.html` only if needed
- [x] `ui/package.json` only if needed for existing script correction
- [x] `ui/tsconfig.json` only if needed for source inclusion
- [x] `scripts/check.sh` only if validation compatibility is required
- [x] `CHANGELOG.md`
- [x] `checklists/current-phase.md`

## Code-production deliverable checklist
- [x] Produce executable product behavior for Phase 137.
- [x] Preserve deterministic replay projection output for identical local shell and ledger input.
- [x] Keep replay descriptive, local-session, in-memory, local-only, and non-production.

## Rust replay projection checklist
- [x] Add typed Rust replay/status projection structures and closed status values.
- [x] Expose explicit no-decision, approved-decision, rejected-decision, and inconsistent-ledger projection states.
- [x] Derive replay projection from current local shell state and decision ledger without mutating either input.
- [x] Update replay projection after accepted approve/reject decisions.
- [x] Leave replay projection unchanged for invalid, duplicate, or forbidden requests.
- [x] Fail closed with an inconsistent projection for malformed representable ledger state.

## TypeScript transport projection checklist
- [x] Extend TypeScript local shell state with Rust-shaped replay/status projection data.
- [x] Return replay projection data after initial state, deterministic stub run, approve, and reject flows.
- [x] Keep rejected requests non-mutating and browser-usable.

## UI replay/status panel checklist
- [x] Replace the replay/status placeholder with a visible local replay/status projection panel.
- [x] Show replay status, decision count, latest decision ID, latest run ID, latest candidate ID, latest operator ID, latest decision kind, and integrity summary.
- [x] Render no-decision, approved-decision, and rejected-decision states.

## Rust test checklist
- [x] Test initial no-decision replay projection.
- [x] Test deterministic stub run preserves no-decision replay projection.
- [x] Test valid approve/reject decisions update replay projection.
- [x] Test invalid/forbidden/duplicate requests do not update replay projection as accepted.
- [x] Test deterministic and non-mutating replay derivation.
- [x] Test malformed ledger state fails closed with inconsistent replay projection.

## TypeScript test checklist
- [x] Test visible no-decision replay/status output.
- [x] Test visible approved and rejected replay/status updates.
- [x] Test forbidden and duplicate actions do not update replay projection as accepted.
- [x] Test replay derivation determinism.

## Local-only/non-production boundary checklist
- [x] No filesystem persistence.
- [x] No durable ledger writes.
- [x] No provider execution.
- [x] No broad command execution.
- [x] No production persistence.
- [x] No replay repair or recovery promotion.
- [x] No action execution.
- [x] No release/deployment/signing/publishing behavior.
- [x] No readiness, Release Candidate, Production Candidate, public-use, or production-human-use approval.

## Validation checklist
- [x] Run `CARGO_TARGET_DIR=/tmp/ajentic-phase-137-target ./scripts/check.sh`.
- [x] Run `git diff --check`.
- [x] Run `git status --short`.
- [x] Run UI typecheck, lint, build, and API behavior tests directly if needed.
- [x] Run Rust tests directly if needed.
- [x] Run local dev smoke test.
- [x] Run replay projection scan.
- [x] Run no-persistence/provider/release/deployment authority scan.
- [x] Run changed-file source guard.

## Deferred items
- [x] Filesystem persistence and durable ledger writes remain deferred.
- [x] Provider execution and broad command execution remain deferred.
- [x] Replay repair and recovery promotion remain deferred.
- [x] Action execution remains deferred.
- [x] Release artifacts, installer/update-channel behavior, signing, publishing, deployment, and readiness approval remain deferred.

## Validation log
- [x] Validation commands completed after final edits.
- [x] No masked failures.
- [x] Generated artifacts cleaned.

## Zero-drift checklist
- [x] Changelog entry matches the Phase 137 code-production diff.
- [x] Staged files are limited to allowed Phase 137 surfaces.
- [x] Rust-derived replay/status projection exists.
- [x] Transport projection includes replay/status data.
- [x] UI visibly renders replay/status projection.
- [x] No local-only/non-production boundary drift introduced.
