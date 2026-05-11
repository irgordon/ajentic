---
truth_dimension: procedural
authority_level: authoritative
mutation_path: checklist_revision
---

# Current Phase Checklist - Phase 136 In-Memory Local Decision Ledger

## Phase goal
- [x] Add usable local-session decision recording for approve/reject operator intents submitted through the local transport boundary.
- [x] Expose the Rust-owned in-memory decision timeline in the local UI shell.

## Working-tree hygiene gate
- [x] Keep changes limited to Phase 136 code-production, tests, changelog, and checklist surfaces.
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
- [x] Produce executable product behavior for Phase 136.
- [x] Preserve deterministic stub output for identical input.
- [x] Keep all behavior local-only and non-production.

## Rust decision ledger checklist
- [x] Add Rust-owned in-memory local decision ledger types.
- [x] Add typed approve/reject decision records.
- [x] Append exactly one decision record for a valid approve/reject intent.
- [x] Keep initial shell state and started stub run ledger/timeline empty until operator action.
- [x] Reject malformed, duplicate, wrong-run, wrong-candidate, authority-granting, readiness-claiming, and provider-execution-related requests without ledger mutation.
- [x] Preserve deterministic decision sequence ordering.

## TypeScript transport projection checklist
- [x] Extend TypeScript shell state with decision ledger/timeline projection data.
- [x] Return updated decision timeline data after approve/reject through the local transport adapter.
- [x] Keep rejected requests non-mutating and browser-usable.

## UI decision ledger checklist
- [x] Render a visible local decision ledger/timeline region.
- [x] Show empty-ledger text before approve/reject.
- [x] Show decision sequence, run ID, candidate ID, operator ID, decision kind, and decision status after approve/reject.

## Rust test checklist
- [x] Test empty initial ledger/timeline projection.
- [x] Test deterministic stub run preserves an empty ledger/timeline.
- [x] Test valid approve/reject records exactly one typed decision.
- [x] Test duplicate and invalid/forbidden requests fail closed without ledger mutation.

## TypeScript test checklist
- [x] Test visible empty local decision ledger.
- [x] Test visible approve and reject decision history updates.
- [x] Test forbidden and invalid UI actions do not add decision records.
- [x] Test transport capabilities remain disabled.

## Local-only/non-production boundary checklist
- [x] No filesystem persistence.
- [x] No durable ledger writes.
- [x] No provider execution.
- [x] No broad command execution.
- [x] No production persistence.
- [x] No replay repair or recovery promotion.
- [x] No release/deployment/signing/publishing behavior.
- [x] No readiness, Release Candidate, Production Candidate, public-use, or production-human-use approval.

## Validation checklist
- [x] Run `CARGO_TARGET_DIR=/tmp/ajentic-phase-136-target ./scripts/check.sh`.
- [x] Run `git diff --check`.
- [x] Run `git status --short`.
- [x] Run UI typecheck, lint, build, and API behavior tests directly if needed.
- [x] Run Rust tests directly if needed.
- [x] Run local dev smoke test.
- [x] Run decision ledger scan.
- [x] Run no-persistence/provider/release/deployment authority scan.
- [x] Run changed-file source guard.

## Deferred items
- [x] Filesystem persistence and durable ledger writes remain deferred.
- [x] Provider execution and broad command execution remain deferred.
- [x] Replay repair and recovery promotion remain deferred.
- [x] Release artifacts, installer/update-channel behavior, signing, publishing, deployment, and readiness approval remain deferred.

## Validation log
- [x] Validation commands completed after final edits.
- [x] No masked failures.
- [x] Generated artifacts cleaned.

## Zero-drift checklist
- [x] Changelog entry matches the Phase 136 code-production diff.
- [x] Staged files are limited to allowed Phase 136 surfaces.
- [x] Rust-owned in-memory decision ledger exists.
- [x] Transport projection includes decision ledger/timeline data.
- [x] UI visibly renders local decision history.
- [x] No local-only/non-production boundary drift introduced.
