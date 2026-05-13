---
truth_dimension: procedural
authority_level: authoritative
mutation_path: checklist_revision
---

# Current Phase Checklist - Phase 161 Controlled Internal Trial Package

## Phase name
- [x] Phase 161 - Controlled Internal Trial Package.

## Phase goal
- [x] Derive, validate, serialize, write, read, read-back validate, and display a deterministic controlled internal trial package using explicit caller-provided paths.
- [x] Keep the package local-only, non-public, non-production, and preparation-only.

## Working-tree hygiene gate
- [x] Confirm starting worktree state.
- [x] Keep changes limited to Phase 161 allowed code, UI, test, changelog, and checklist surfaces.
- [x] Do not edit roadmap, governance, architecture, release, deployment, installer, update, signing, publishing, provider execution, action execution, replay repair, or recovery promotion surfaces.

## Allowed surfaces
- [x] `core/src/**`
- [x] `ui/src/**`
- [x] `CHANGELOG.md`
- [x] `checklists/current-phase.md`

## Code-production deliverable checklist
- [x] Rust-owned controlled internal trial package types exist.
- [x] Deterministic package derivation exists.
- [x] Deterministic serialization exists.
- [x] Explicit write/read helpers exist.
- [x] UI-visible trial package panel exists.

## Controlled internal trial package checklist
- [x] Package ID is deterministic.
- [x] Package version is present.
- [x] Package classification is `controlled_internal_trial_package_only`.
- [x] Production classification is `non_production`.
- [x] Distribution classification is `local_only_non_public`.
- [x] Package status projection starts as `not_packaged`.

## Trial metadata checklist
- [x] Trial scope type is present.
- [x] Named internal operator metadata type is present.
- [x] Trial participant metadata type is present.
- [x] Validation fails closed when scope is missing.
- [x] Validation fails closed when named operator or participant metadata is missing.

## Stop-condition marker checklist
- [x] Stop-condition marker type is present.
- [x] Required stop-condition markers are included.
- [x] Validation fails closed when stop-condition markers are missing.
- [x] UI states stop conditions are metadata and do not automate enforcement in Phase 161.

## Included evidence summary checklist
- [x] Local beta workflow status is included.
- [x] Provider output pipeline status is included.
- [x] Local candidate materialization status is included.
- [x] Operator decision status is included.
- [x] Replay/status summary is included.
- [x] Local evidence export summary is included.
- [x] Session package summary is included.
- [x] Restore/history summary is included.
- [x] Phase 160 gate-decision context is included.

## Package validation checklist
- [x] Reject missing package ID.
- [x] Reject missing package version.
- [x] Reject invalid package, production, or distribution classification.
- [x] Reject missing absence markers.
- [x] Reject release, deployment, readiness, public-use, or production-use claims.
- [x] Reject provider trust, action authorization, replay repair, or recovery promotion claims.
- [x] Reject deterministic content mismatch.

## Explicit write/read helper checklist
- [x] Write helper requires a caller-provided path.
- [x] Write helper validates before writing.
- [x] Read helper requires a caller-provided path.
- [x] No default write occurs.
- [x] No automatic persistence is added.
- [x] Tests write only under `/tmp` paths.

## Read-back validation checklist
- [x] Read-back parses the caller-provided package file.
- [x] Read-back validates the parsed package.
- [x] Malformed package content is rejected fail-closed.

## UI trial package projection checklist
- [x] TypeScript local shell state includes controlled internal trial package projection.
- [x] UI renders a `Controlled internal trial package` panel.
- [x] UI renders status, ID, version, classifications, scope, named operator/participant summary, stop conditions, evidence, absence markers, validation, read-back status, and boundary notes.
- [x] UI does not expose publish, deploy, sign, release, public-use, production-use, or approval controls.

## Local-only/non-public boundary checklist
- [x] UI states: “Controlled internal trial package is local-only and non-public.”
- [x] UI states: “This package is not a release artifact.”
- [x] UI states: “This package is not deployment or readiness evidence.”
- [x] UI states: “This package does not approve public/general use or production use.”
- [x] UI states controlled human use is not approved.
- [x] Rust boundary markers include no release, deployment, readiness, public-use, production-use, provider trust, action authorization, replay repair, or recovery promotion.

## Rust test checklist
- [x] Initial `not_packaged` projection.
- [x] Deterministic derivation, package ID, content digest, and serialization.
- [x] Required metadata, classifications, scope, operators, participants, stop conditions, included evidence, and absence markers.
- [x] Missing metadata/scope/operator/participant/stop-condition rejection.
- [x] No-authority boundary rejection.
- [x] Explicit write/read and read-back validation using temp paths.
- [x] Malformed read-back rejection.

## TypeScript test checklist
- [x] Initial controlled internal trial package projection.
- [x] Visible UI package panel and required boundary wording.
- [x] Deterministic rendering.
- [x] No publish/deploy/sign controls.

## Phase 162 handoff checklist
- [x] Phase 162 remains the next code-production phase for trial operator runbook and failure drill UI.
- [x] Phase 161 does not start trial execution.
- [x] Phase 161 does not approve controlled human use, public/general use, production use, release readiness, deployment readiness, or production readiness.

## Validation checklist
- [x] `CARGO_TARGET_DIR=/tmp/ajentic-phase-161-target ./scripts/check.sh`
- [x] `git diff --check`
- [x] `git status --short`
- [x] `cd ui && npm run typecheck`
- [x] `cd ui && npm run lint`
- [x] `cd ui && npm run build && rm -rf dist`
- [x] `cd ui && npm run test:api`
- [x] `CARGO_TARGET_DIR=/tmp/ajentic-phase-161-target cargo test --manifest-path core/Cargo.toml --all-targets`
- [x] `cd ui && timeout 5 npm run dev`
- [x] Trial package scan.
- [x] Trial boundary scan.
- [x] Filesystem scan.
- [x] Forbidden label scan.
- [x] Unsafe execution scan.
- [x] No-release/deployment/readiness scan.
- [x] Changed-file source guard.
- [x] No-roadmap-drift guard.

## Deferred items
- [x] Trial execution harness remains deferred.
- [x] Trial operator runbook/failure drill UI remains Phase 162.
- [x] Trial session evidence capture remains Phase 163.
- [x] Trial replay/restore verification remains Phase 164.
- [x] Release, deployment, installer, update-channel, signing, publishing, public-use, production-use, readiness, provider trust, action authorization, replay repair, and recovery promotion remain deferred.

## Validation log
- `CARGO_TARGET_DIR=/tmp/ajentic-phase-161-target cargo test --manifest-path core/Cargo.toml --all-targets` passed.
- `cd ui && npm run typecheck` passed.
- `cd ui && npm run lint` passed.
- `cd ui && npm run build && rm -rf dist` passed and generated `dist` was removed.
- `cd ui && npm run test:api` passed.
- `cd ui && timeout 5 npm run dev` printed the local browser URL; timeout stopped the long-running dev server as expected.
- `git diff --check` passed.
- `git status --short` showed only allowed Phase 161 files changed before commit.
- Trial package, trial boundary, filesystem, forbidden label, unsafe execution, no-release/deployment/readiness, changed-file source guard, and no-roadmap-drift scans completed.
- `CARGO_TARGET_DIR=/tmp/ajentic-phase-161-target ./scripts/check.sh` requires a clean worktree and is run after committing the Phase 161 patch.

## Zero-drift checklist
- [x] Full validation passes after final edits.
- [x] Generated artifacts are cleaned.
- [x] Staged files match allowed Phase 161 surfaces.
- [x] Rust-owned controlled internal trial package exists.
- [x] Package metadata includes deterministic package ID, version, classifications, scope, named internal operator/trial participant metadata, stop-condition markers, included evidence summary, and absence markers.
- [x] Validation fails closed for missing/malformed/drifted required fields and authority claims.
- [x] Write/read helpers require explicit caller-provided paths.
- [x] No default persistence, automatic save, background persistence, release artifact, deployment artifact, installer/update/signing/publishing behavior, production persistence, or approval behavior is introduced.
- [x] Package derivation/write/read does not mutate shell workflow state or execute providers/actions/replay repair/recovery promotion.
- [x] UI renders controlled internal trial package status.
- [x] Roadmap files are not modified.
- [x] CHANGELOG entry matches actual diff.
- [x] Phase 162 remains the next code-production phase for trial operator runbook and failure drill UI.
