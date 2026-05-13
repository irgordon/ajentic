---
truth_dimension: procedural
revision_mode: replace
authority_level: authoritative
mutation_path: checklist_revision
---

# Current Phase Checklist - Phase 163 Trial Session Evidence Capture

## Phase name
- [x] Phase 163 - Trial Session Evidence Capture.

## Phase goal
- [x] Capture deterministic local trial-preparation/session evidence from current local shell state.
- [x] Produce a real local trial session evidence artifact only through explicit caller-provided write/read paths.
- [x] Keep evidence local-only, non-public, non-production, and non-authoritative.

## Working-tree hygiene gate
- [x] Review current branch status before edits.
- [x] Keep changes limited to allowed Phase 163 surfaces.
- [x] Do not modify roadmap files.

## Allowed surfaces
- [x] `core/src/**`
- [x] `ui/src/**`
- [x] `CHANGELOG.md`
- [x] `checklists/current-phase.md`

## Code-production deliverable checklist
- [x] Rust-owned trial session evidence record types.
- [x] Deterministic evidence derivation, validation, serialization, parse, write, read, and read-back validation.
- [x] Local shell/transport projection for trial session evidence status.
- [x] Visible UI panels for `Trial session evidence`, `Trial evidence capture`, and `Evidence read-back validation`.

## Trial session evidence record checklist
- [x] Evidence version.
- [x] Deterministic evidence ID and content digest.
- [x] `trial_session_evidence_only` classification.
- [x] `non_production` production classification.
- [x] `local_only_non_public` distribution classification.
- [x] `non_authoritative_evidence` authority classification.
- [x] Absence markers for unavailable/non-authorized sections.

## Evidence linkage checklist
- [x] Trial package linkage.
- [x] Runbook linkage.
- [x] Failure drill linkage.
- [x] Missing linkage validation fails closed.

## Evidence snapshot checklist
- [x] Local workflow status snapshot.
- [x] Local candidate materialization snapshot.
- [x] Provider output pipeline snapshot.
- [x] Operator decision snapshot.
- [x] Replay/status snapshot.
- [x] Evidence export snapshot.
- [x] Session package snapshot.
- [x] Restore/history snapshot.
- [x] Current blocker snapshot.

## Stop-condition/escalation/failure snapshot checklist
- [x] Stop-condition snapshot.
- [x] Escalation guidance snapshot.
- [x] Failure category snapshot.
- [x] Validation rejects missing stop-condition or escalation snapshots.

## Evidence validation checklist
- [x] Missing evidence ID rejected.
- [x] Missing evidence version rejected.
- [x] Invalid classifications rejected.
- [x] Missing absence markers rejected.
- [x] Readiness, release, deployment, public-use, and production-use claims rejected.
- [x] Provider trust, action authorization, replay repair, and recovery promotion claims rejected.
- [x] Deterministic digest mismatch rejected.
- [x] Malformed evidence input rejected.

## Explicit write/read helper checklist
- [x] Write helper requires caller-provided path.
- [x] Write helper validates before write.
- [x] Read helper requires caller-provided path.
- [x] Tests use temporary paths only.
- [x] No default persistence, automatic save, background persistence, or network sync.

## Read-back validation checklist
- [x] Read-back parses written evidence.
- [x] Read-back validates structure and deterministic digest.
- [x] Read-back validation is structure-only and not authority.
- [x] Malformed read-back fails closed.

## UI evidence projection checklist
- [x] UI renders evidence status, ID, version, and classifications.
- [x] UI renders trial package linkage and runbook/failure drill linkage.
- [x] UI renders included evidence, workflow, candidate, replay, export/package/restore, stop-condition, escalation, and failure summaries.
- [x] UI renders validation/read-back status.
- [x] UI includes required local-only/non-authoritative wording.
- [x] UI does not add publish, deploy, sign, release, or start-trial controls.

## Local-only/non-authoritative boundary checklist
- [x] Include `local_trial_evidence_only`.
- [x] Include `non_public_evidence`.
- [x] Include `evidence_not_authority`.
- [x] Include no trial execution.
- [x] Include no controlled human-use, readiness, release, deployment, public-use, or production-use approval.
- [x] Include no provider trust, action authorization, replay repair, or recovery promotion.

## Rust test checklist
- [x] Initial not-captured projection.
- [x] Deterministic evidence derivation, evidence ID, and serialized content.
- [x] Valid linkage and snapshot coverage.
- [x] Missing metadata/linkage/snapshot/marker rejection.
- [x] Non-authority claim rejection.
- [x] Explicit write/read with temp path.
- [x] Read-back validation and malformed/digest-drift rejection.

## TypeScript test checklist
- [x] Visible evidence capture status.
- [x] Included evidence summary.
- [x] Stop-condition, escalation, and failure summaries.
- [x] Read-back status.
- [x] Required no-authority wording.
- [x] Deterministic repeated rendering.

## Phase 164 handoff checklist
- [x] Phase 164 remains the next code-production phase for trial replay and restore verification.
- [x] Trial session evidence does not start trials, approve use, repair replay, or promote recovery.

## Validation checklist
- [x] `CARGO_TARGET_DIR=/tmp/ajentic-phase-163-target ./scripts/check.sh`
- [x] `git diff --check`
- [x] `git status --short`
- [x] `cd ui && npm run typecheck`
- [x] `cd ui && npm run lint`
- [x] `cd ui && npm run build && rm -rf dist`
- [x] `cd ui && npm run test:api`
- [x] `CARGO_TARGET_DIR=/tmp/ajentic-phase-163-target cargo test --manifest-path core/Cargo.toml --all-targets`
- [x] `cd ui && timeout 5 npm run dev`
- [x] Evidence scan.
- [x] Evidence boundary scan.
- [x] Filesystem scan.
- [x] Forbidden label scan.
- [x] Unsafe execution scan.
- [x] No-release/deployment/readiness scan.
- [x] Changed-file source guard.
- [x] No-roadmap-drift guard.

## Deferred items
- [x] Controlled trial start remains deferred.
- [x] Controlled human-use, public-use, and production-use approval remain deferred.
- [x] Automated stop-condition enforcement and escalation remain deferred.
- [x] Replay repair, restore promotion, recovery promotion, action execution, release, deployment, signing, publishing, installer, update-channel, network/cloud behavior, and background services remain deferred.

## Validation log
- `CARGO_TARGET_DIR=/tmp/ajentic-phase-163-target ./scripts/check.sh` first stopped at the dirty-working-tree preflight before commit, then passed after the Phase 163 commit.
- `git diff --check` passed.
- `git status --short` showed only allowed Phase 163 files changed.
- `cd ui && npm run typecheck` passed.
- `cd ui && npm run lint` passed.
- `cd ui && npm run build && rm -rf dist` passed and generated `dist` was removed.
- `cd ui && npm run test:api` passed.
- `CARGO_TARGET_DIR=/tmp/ajentic-phase-163-target cargo test --manifest-path core/Cargo.toml --all-targets` passed.
- `cd ui && timeout 5 npm run dev` printed the local browser URL and exited by timeout while stopping the long-running dev server.
- Evidence, boundary, filesystem, forbidden-label, unsafe-execution, release/deployment, changed-file, and no-roadmap-drift scans completed; broad scans include historical/prohibition matches, with new filesystem operations limited to explicit trial session evidence helpers and temp-path tests.

## Zero-drift checklist
- [x] Rust owns trial session evidence derivation, validation, serialization, write/read, and read-back validation.
- [x] TypeScript remains display/non-authoritative.
- [x] Evidence write/read uses explicit caller-provided paths only.
- [x] Evidence derivation/write/read does not mutate shell workflow state.
- [x] No default filesystem persistence is added.
- [x] No release, deployment, readiness, public-use, production-use, provider trust, candidate approval, replay repair, recovery promotion, or action execution behavior is added.
- [x] CHANGELOG entry matches the Phase 163 diff.
- [x] Roadmap files are not modified.
