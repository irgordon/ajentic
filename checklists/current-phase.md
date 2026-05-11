---
truth_dimension: procedural
authority_level: authoritative
mutation_path: checklist_revision
---

# Current Phase Checklist - Phase 138 Local Session Evidence Export

## Phase goal
- [x] Add a deterministic Rust-owned local session evidence export for the local operator shell.
- [x] Render the export preview in the local browser UI from the transport-shaped shell state.

## Working-tree hygiene gate
- [x] Keep changes limited to Phase 138 code-production, tests, changelog, and checklist surfaces.
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
- [x] Produce executable product behavior for Phase 138.
- [x] Provide default non-mutating export payload derivation and UI-visible preview.
- [x] Keep export local-session, local-only, non-production, and descriptive.

## Rust export derivation checklist
- [x] Add typed local session evidence export structures and closed status values.
- [x] Derive export payload from shell status, run projection, bounded context, candidate output, validation/policy projection, decision ledger, and replay/status projection.
- [x] Preserve deterministic output for identical shell state and ledger input.
- [x] Do not mutate local shell state or decision ledger during export derivation.
- [x] Update export projection after initial, stub-run, approve, and reject flows.

## Export validation/classification checklist
- [x] Classify exports as `local_session_evidence_only`.
- [x] Classify production status as `non-production`.
- [x] Include absence markers for provider execution, persistence, release, deployment, signing, publishing, installer, update-channel, public-use, and readiness approval.
- [x] Fail closed for missing required fields, wrong classification, wrong production classification, missing absence markers, missing run evidence, or missing decision/replay evidence.

## TypeScript transport projection checklist
- [x] Extend TypeScript local shell state with Rust-shaped export payload data.
- [x] Return export payload data after initial state, deterministic stub run, approve, and reject flows.
- [x] Keep invalid, duplicate, and forbidden requests non-mutating and browser-usable.

## UI export preview checklist
- [x] Render a visible local session evidence export panel.
- [x] Show export ID, export classification, production classification, run ID/status, candidate ID, validation/policy status, decision count, replay status, replay integrity, and absence markers summary.
- [x] Render no-completed-run, run-evidence, and decision-evidence states.

## Rust test checklist
- [x] Test export derivation for initial, stub-run, and decision states.
- [x] Test export completeness and classification.
- [x] Test deterministic and non-mutating export derivation.
- [x] Test validation fail-closed behavior.
- [x] Test forbidden requests do not promote export evidence.

## TypeScript test checklist
- [x] Test visible export preview before a run.
- [x] Test visible run/candidate/validation export evidence after stub run.
- [x] Test visible decision/replay export evidence after approve/reject.
- [x] Test forbidden actions do not produce accepted-decision export evidence.
- [x] Test export derivation determinism.

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
- [x] Run `CARGO_TARGET_DIR=/tmp/ajentic-phase-138-target ./scripts/check.sh`.
- [x] Run `git diff --check`.
- [x] Run `git status --short`.
- [x] Run UI typecheck, lint, build, and API behavior tests directly if needed.
- [x] Run Rust tests directly if needed.
- [x] Run local dev smoke test.
- [x] Run export scan.
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
- [x] Changelog entry matches the Phase 138 code-production diff.
- [x] Staged files are limited to allowed Phase 138 surfaces.
- [x] Rust-derived local session evidence export exists.
- [x] Transport projection includes local session evidence export data.
- [x] UI visibly renders local session evidence export preview.
- [x] No local-only/non-production boundary drift introduced.
