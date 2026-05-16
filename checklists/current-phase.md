---
truth_dimension: procedural
authority_level: authoritative
mutation_path: checklist_revision
---

# Current Phase Checklist: Phase 172

## Phase name
- [x] Phase 172 - Release Artifact Dry Package Assembly.

## Phase goal
- [x] Add a deterministic local-only dry package path from the Phase 171 release-candidate preparation contract.
- [x] Keep the package rehearsal evidence only: not a release artifact, not public distribution, and not approval.

## Working-tree hygiene gate
- [x] Start from repaired Phase 171.1 baseline.
- [x] Keep edits within core API, UI source/tests, changelog, and this checklist.
- [x] Do not edit roadmap, governance, architecture, release workflows, installer, signing, publishing, deployment, provider execution, or recovery behavior.

## Allowed surfaces
- [x] `core/src/api/release_artifact_dry_package.rs` dedicated module.
- [x] Thin exports/state integration in `core/src/api/mod.rs` and `core/src/api/local_operator_shell_state.rs`.
- [x] `ui/src/**` TypeScript model, rendering, and behavior tests.
- [x] `CHANGELOG.md` and `checklists/current-phase.md`.

## Code-production deliverable checklist
- [x] Rust-owned dry package type, metadata, payload, projection, read-back projection, statuses, classifications, validation errors, and boundary statuses.
- [x] Deterministic dry package ID and content digest.
- [x] Deterministic validation, serialization, parsing, write, read, and read-back validation helpers.
- [x] Visible UI dry package panel.

## Dedicated module checklist
- [x] Keep dry package derivation, validation, serialization, parsing, write/read, and read-back validation in `release_artifact_dry_package.rs`.
- [x] Avoid placing dry package behavior in `local_operator_shell.rs`.

## Thin shell integration checklist
- [x] Add dry package projection field to local shell state.
- [x] Initialize the field as `not_assembled` only.
- [x] Do not automatically derive, write, read, persist, sign, publish, deploy, install, update, execute providers/actions, repair replay, or promote recovery from shell initialization.

## Dry package assembly checklist
- [x] Derive from Phase 171 preparation projection.
- [x] Require projected, valid, unblocked, unrejected preparation with no missing required evidence.
- [x] Classify dry package as `dry_run_package_only`.
- [x] Classify production as `non_production`.
- [x] Classify distribution as `local_only_non_public`.
- [x] Classify authority as `non_authoritative_rehearsal_evidence`.
- [x] Classify release as `release_not_approved`.

## Included evidence checklist
- [x] Include deterministic evidence category, status, source surface, source status, and source summary.
- [x] Include deterministic included evidence count.
- [x] Link included evidence to the source preparation ID.

## Validation/fail-closed checklist
- [x] Reject missing preparation.
- [x] Reject blocked preparation.
- [x] Reject rejected preparation.
- [x] Reject preparation with missing required evidence.
- [x] Reject readiness, release, deployment, public-use, production-use, signing, publishing, installer, update-channel, public-download, GitHub-release, release-tag, provider-trust, action-authorization, replay-repair, and recovery-promotion claims.
- [x] Reject malformed dry package input.
- [x] Reject digest mismatch.

## Explicit write/read helper checklist
- [x] Write helper requires caller-provided path.
- [x] Write helper validates before write.
- [x] Read helper requires caller-provided path.
- [x] No default filesystem persistence, automatic save, background persistence, arbitrary scan, remote sync, or public artifact path.

## Read-back validation checklist
- [x] Parse dry package content deterministically.
- [x] Validate structure and digest after read.
- [x] Report read-back validation status as structure-only validation.

## UI dry package panel checklist
- [x] Render panel labeled `Release artifact dry package`.
- [x] Render package status and dry package ID.
- [x] Render dry, production, distribution, authority, and release classifications.
- [x] Render included evidence count and summary.
- [x] Render validation status, read-back validation status, and rejection reason.
- [x] Keep UI display-only with no publish, sign, deploy, release, installer, update-channel, public-download, GitHub-release, release-tag, approval, or public-distribution controls.

## No-release/no-public-distribution boundary checklist
- [x] State: “A dry package is rehearsal evidence, not a release artifact.”
- [x] State: “This package does not approve release readiness or Release Candidate status.”
- [x] State: “This package is local-only and non-public.”
- [x] State: “No signing, publishing, installer, update-channel, public download, GitHub release, release tag, deployment, or public distribution occurs.”
- [x] State: “Read-back validation checks dry package structure only.”

## Rust test checklist
- [x] Deterministic dry package assembly, ID, digest, serialization, classifications, and included evidence.
- [x] Missing, blocked, rejected, and missing-evidence preparation rejection.
- [x] Explicit write/read with temp path.
- [x] Malformed read-back rejection.
- [x] Digest mismatch rejection.
- [x] No-authority boundaries and claim rejection.

## TypeScript test checklist
- [x] Visible dry package panel.
- [x] Package status, package ID, included evidence summary, and read-back status.
- [x] Rejected dry package projection.
- [x] Deterministic rendering.
- [x] No-authority wording and forbidden-label absence.

## Phase 173 handoff checklist
- [x] Phase 173 remains the next code-production phase for checksum and provenance evidence for the dry package.
- [x] Phase 172 does not create release artifacts, public artifacts, tags, releases, installers, update channels, signing, publishing, deployment, or approval behavior.

## Validation checklist
- [x] Run `CARGO_TARGET_DIR=/tmp/ajentic-phase-172-target ./scripts/check.sh`.
- [x] Run `git diff --check`.
- [x] Run `git status --short`.
- [x] Run direct UI commands if needed.
- [x] Run direct Rust tests if needed.
- [x] Run local dev smoke test because UI changed.
- [x] Run dry package, dedicated module, boundary, forbidden label, filesystem, unsafe execution, release/deployment, changed-file, and no-roadmap-drift scans.

## Deferred items
- [x] Public artifacts, release artifacts, signing, publishing, installer activation, update-channel activation, public downloads, GitHub releases, release tags, deployment behavior, readiness approval, Release Candidate approval, Production Candidate approval, public-use approval, production-use approval, provider trust, action authorization, replay repair, and recovery promotion remain deferred.

## Validation log
- [x] Record final command outcomes in the agent final response.

## Zero-drift checklist
- [x] Roadmap files not modified.
- [x] Governance and architecture files not modified.
- [x] No long governance report added.
- [x] No release matrix, readiness ladder, or artifact sequencing table added.
