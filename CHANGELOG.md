---
truth_dimension: historical
authority_level: authoritative
mutation_path: changelog_entry
---

## v0.0.195.1 - 2026-06-11
**Status:** Phase 195.1 - Protected-Branch PR Closure and Solo-Maintainer Merge Evidence

### Added
- Add protected-branch PR closure evidence for PR #297.
- Record the solo-maintainer approval-gate deadlock and admin-bypass squash merge exception.
- Record the mainline squash merge commit and original PR branch head.

### Notes
- Mainline squash merge commit: `d154e06789e46bb6485323ef9b02020c31a992b8`.
- PR branch head before merge: `9496866300cc3e8b1c881076c3d5e8c059b320c2`.
- All required checks passed before merge.
- Branch protection remains active.
- Remote tags and releases remain limited to `v1.0.0` and `v1.0.0-rc.1`.
- No new tag or GitHub Release was created.
- No package publication, installer, update channel, deployment, OS signing, notarization, backend authority change, or UI authority change occurred.

---
truth_dimension: historical
authority_level: authoritative
mutation_path: changelog_entry
---

## v0.0.195 - 2026-06-10
**Status:** Phase 195 - Post-v1 Release Closure and Evidence Preservation

### Added
- Add post-v1 release closure documentation for the completed `v1.0.0` GitHub Release.
- Record final workflow, final release, final tag, final asset, source RC, branch protection, deployment, and attestation evidence.
- Record post-v1 package metadata policy and future-scope restrictions.

### Changed
- Update README status to reflect that the final `v1.0.0` GitHub Release is published.
- Align AGENTS, checklist, and roadmap surfaces with the post-v1 release closure boundary.
- Update the `v1.0.0` release note and Phase 194 operation note with final remote evidence.

### Notes
- Final GitHub Release `v1.0.0` is published.
- Final tag `v1.0.0` targets `6734a5b3cc223c41288b14575ad40f6fcf23fb6f`.
- Source RC `v1.0.0-rc.1` remains intact.
- Final workflow `27307966684` concluded successfully.
- Branch protection gate was closed before final dispatch.
- Final attestation evidence was recorded from the supplied link.
- Package metadata remains separate from GitHub Release identity: `core` and `ajentic-ui` remain `0.1.0` with `MIT`.
- No npm package publication occurred.
- No Cargo package publication occurred.
- No package registry publication occurred.
- No installer was created.
- No update channel was created.
- No deployment occurred.
- No OS signing or notarization occurred.
- No backend authority change occurred.
- No UI authority change occurred.

---
truth_dimension: historical
authority_level: authoritative
mutation_path: changelog_entry
---

## v0.0.194 - 2026-06-10
**Status:** Phase 194 - Final Functional Acceptance and v1.0 Release Execution

### Added
- Add a manually dispatched `final-release` GitHub Actions workflow for the guarded `v1.0.0` final GitHub Release path.
- Add GitHub REST API publication support for the final annotated tag, final release, and final asset uploads.
- Add deterministic final asset staging for the final candidate bundle, checksums, SBOM, provenance, asset manifest, release notes, and final readme.
- Add local final release preflight checks.
- Add Phase 194 operation documentation and `v1.0.0` release notes.

### Changed
- Align README, AGENTS, checklist, and roadmap surfaces with the Phase 194 final-release execution boundary.
- Extend aggregate local validation to cover final asset determinism and final release preflight checks.

### Notes
- `v1.0.0-rc.1` is the source release candidate.
- Branch protection or ruleset status must be verified, or an explicit final-release Owner exception must be recorded before final workflow dispatch.
- Package metadata remains separately governed and package versions remain unchanged.
- No npm or Cargo package publication path was created.
- No installer path was created.
- No update-channel path was created.
- No deployment path was created.
- No OS signing or notarization path was created.
- Rust, TypeScript, Python, and Bash authority boundaries remain unchanged.

---
truth_dimension: historical
authority_level: authoritative
mutation_path: changelog_entry
---

## v0.0.193 - 2026-06-10
**Status:** Phase 193 - v1 Release Candidate Publication

### Added
- Add a manually dispatched `rc-publication` GitHub Actions workflow for the bounded `v1.0.0-rc.1` public prerelease path.
- Add GitHub REST API publication support for the RC tag, prerelease, and asset uploads.
- Add deterministic public RC asset staging for the candidate bundle, checksums, SBOM, provenance, asset manifest, release notes, and RC readme.
- Add local RC publication preflight checks.
- Add Phase 193 operation documentation and `v1.0.0-rc.1` release notes.

### Changed
- Align README, AGENTS, checklist, and roadmap surfaces with the Phase 193 RC-only publication boundary.
- Extend aggregate local validation to cover RC asset determinism and RC publication preflight checks.

### Notes
- Phase 193 authorizes public RC publication only.
- Final `v1.0.0` release remains Phase 194.
- Package versions remain unchanged.
- No package or crate version was changed to `1.0.0`.
- No npm or Cargo package publication path was created.
- No installer path was created.
- No update-channel path was created.
- No deployment path was created.

---
truth_dimension: historical
authority_level: authoritative
mutation_path: changelog_entry
---

## v0.0.192 - 2026-06-10
**Status:** Phase 192 - Read-Only Browser UI/UX Clarity and Visual Polish

### Added
- Add display-only status copy mapping for plain browser UI labels.
- Add read-only browser UI glossary, evidence summary cards, accessible status badges, and technical details disclosures.
- Add Phase 192 operation documentation for UI/UX clarity and visual polish boundaries.

### Changed
- Polish the local browser shell header, help surface, release-candidate preparation panel, evidence log, and visual tokens.
- Align README, AGENTS, checklist, and roadmap surfaces with the Phase 192 read-only UI/UX boundary.
- Preserve raw technical status values in details and text snapshots while showing novice-facing labels first.

### Notes
- The UI remains local, read-only, and non-authoritative.
- Simulated local data remains predictable test data, not cloud or live database behavior.
- Blocked, missing, rejected, and unknown statuses remain visible.
- No backend or Rust authority change was created.
- No release was created.
- No Git tag was created.
- No GitHub Release was created.
- No signing path was created.
- No publishing path was created.
- No installer path was created.
- No update-channel path was created.
- No public artifact path was created.
- No deployment path was created.
- Package versions remain unchanged.

---
truth_dimension: historical
authority_level: authoritative
mutation_path: changelog_entry
---

## v0.0.191 - 2026-06-10
**Status:** Phase 191 - Artifact, Signing, and Release Workflow Activation Boundary

### Added
- Add internal release-candidate bundle generation.
- Add manually triggered release-candidate GitHub Actions workflow.
- Add internal bundle manifest and bundle checksum evidence.
- Add GitHub artifact attestation boundary for internal candidate artifacts.
- Add Phase 191 operation documentation for artifact, signing, and release workflow boundaries.

### Changed
- Extend aggregate local validation to include deterministic internal release-candidate bundle checks.
- Align README, AGENTS, checklist, and roadmap surfaces with the Phase 191 internal-candidate workflow boundary.

### Notes
- Attestation permissions are scoped to the attestation job only.
- No `contents: write` permission was added.
- No Git tag was created.
- No GitHub Release was created.
- No package publication occurred.
- No installer path was created.
- No update-channel path was created.
- No public artifact path was created.
- No deployment path was created.
- Package versions remain unchanged.
- RC publication remains Phase 193.
- Final release execution remains Phase 194.

---
truth_dimension: historical
authority_level: authoritative
mutation_path: changelog_entry
---

## v0.0.190 - 2026-06-10
**Status:** Phase 190 - v1 Release Acceleration Plan and Functional Freeze

### Added
- Add a short v1 release acceleration and functional freeze plan.
- Define v1 must-have scope, explicit deferrals, functional acceptance checks, and UI/UX acceptance checks.
- Record that future pre-v1 work must directly support release execution, release artifacts, signing/release workflow, functional blockers, UI/UX quality, or the real user path.

### Changed
- Convert the release path into a v1 acceleration track.
- Collapse remaining pre-v1 release planning into Phases 190-194.
- Update README status language toward v1 release-candidate hardening.
- Mark the previous Phase 190-201 runway as superseded by the compressed v1 track.

### Notes
- No new governance-only or audit-only pre-v1 phase was added.
- Existing authority boundaries remain intact.
- Package versions remain unchanged.
- No release was created.
- No Git tag was created.
- No GitHub Release was created.
- No signing path was created.
- No publishing path was created.
- No installer path was created.
- No update-channel path was created.
- No public artifact path was created.
- No deployment path was created.
- No package version change was created.

---
truth_dimension: historical
authority_level: authoritative
mutation_path: changelog_entry
---

## v0.0.189 - 2026-06-10
**Status:** Phase 189 - Checksums, SBOM, and Provenance Evidence

### Added
- Add internal checksum evidence generation for internal candidate artifacts.
- Add deterministic internal SBOM evidence generation from Cargo and npm manifests.
- Add internal unsigned provenance evidence generation.
- Add deterministic integrity-evidence comparison across two clean builds.

### Changed
- Extend the reproducible artifact workflow to validate internal checksum, SBOM, and provenance evidence.
- Extend aggregate local validation to include Phase 189 internal integrity evidence checks.
- Align README, AGENTS, checklist, operations, and roadmap surfaces with Phase 189 evidence boundaries.

### Notes
- Evidence files are internal review evidence, not release artifacts.
- Checksums are internal integrity evidence, not release checksums.
- Provenance is unsigned internal provenance and not a GitHub artifact attestation.
- No `id-token: write` workflow permission was added.
- No `attestations: write` workflow permission was added.
- Package versions remain pre-v1.0 and unchanged.
- No package or crate version was changed to `1.0.0`.
- No Git tag was created.
- No GitHub Release was created.
- No public artifact was created or uploaded.
- No signing path was created.
- No publishing path was created.
- No installer path was created.
- No update-channel path was created.
- No deployment path was created.

---
truth_dimension: historical
authority_level: authoritative
mutation_path: changelog_entry
---

## v0.0.188 - 2026-06-10
**Status:** Phase 188 - Reproducible Artifact Build in Actions

### Added
- Add a validation-only GitHub Actions workflow for reproducible internal candidate artifact builds.
- Add `scripts/reproducible_artifacts.py` to run two clean builds and compare normalized manifests.
- Add Phase 188 reproducible artifact build documentation.

### Changed
- Extend aggregate local validation to include reproducible internal candidate artifact checks.
- Align README, AGENTS, checklist, and roadmap surfaces with Phase 188 artifact reproducibility boundaries.

### Notes
- The reproducibility workflow uses least-privilege `contents: read` permissions.
- The checker compares candidate category, normalized relative path, byte size, and SHA-256 digest across two clean builds.
- Internal SHA-256 digests are reproducibility comparison fields only, not formal release checksums.
- Formal checksums, SBOM, and provenance remain assigned to Phase 189.
- Package versions remain pre-v1.0 and unchanged.
- No package or crate version was changed to `1.0.0`.
- No Git tag was created.
- No GitHub Release was created.
- No public artifact was created or uploaded.
- No signing path was created.
- No publishing path was created.
- No installer path was created.
- No update-channel path was created.
- No deployment path was created.

---
truth_dimension: historical
authority_level: authoritative
mutation_path: changelog_entry
---

## v0.0.187.1 - 2026-06-10
**Status:** Phase 187.1 - MIT License Activation and License-Status Closure

### Added
- Record Owner-selected MIT license decision.
- Add root `LICENSE` using the standard MIT License text.
- Add MIT license metadata to package manifests where appropriate.
- Align UI package-lock root metadata with MIT.

### Changed
- Update README license status to `License: MIT`.
- Update Phase 187 public identity documentation to close the no-license blocker.
- Align AGENTS and roadmap surfaces with MIT license discipline.

### Notes
- `core/Cargo.toml` remains at `0.1.0`.
- `ui/package.json` remains at `0.1.0`.
- `ui/package-lock.json` remains at `0.1.0`.
- No package version was changed to `1.0.0`.
- No Git tag was created.
- No GitHub Release was created.
- No release artifact was created.
- No public artifact path was created.
- No signing path was created.
- No publishing path was created.
- No installer path was created.
- No update-channel path was created.
- No deployment path was created.

---
truth_dimension: historical
authority_level: authoritative
mutation_path: changelog_entry
---

## v0.0.187 - 2026-06-10
**Status:** Phase 187 - Versioning, License, and Public Identity Alignment

### Added
- Add versioning, license, and public identity alignment documentation.
- Clarify the distinction between roadmap phase numbers, changelog evidence markers, package/crate metadata, Git tags, and GitHub Releases.
- Document license absence as a v1.0/public-release blocker.

### Changed
- Align README public status language with Release Candidate stewardship / pre-v1.0 hardening caveats.
- Remove implied open-source licensing language from README because no license file exists.
- Align AGENTS and roadmap surfaces with version/license/public-identity discipline.

### Notes
- `core/Cargo.toml` remains at `0.1.0`.
- `ui/package.json` remains at `0.1.0`.
- Package versions were left unchanged.
- No `LICENSE`, `LICENSE.md`, or `COPYING` file is present.
- No license file was added because no Owner-selected license decision was found.
- No Git tag was created.
- No GitHub Release was created.
- No release artifact was created.
- No public artifact path was created.
- No signing path was created.
- No publishing path was created.
- No installer path was created.
- No update-channel path was created.
- No deployment path was created.

---
truth_dimension: historical
authority_level: authoritative
mutation_path: changelog_entry
---

## v0.0.186 - 2026-06-10
**Status:** Phase 186 - GitHub Actions Release Platform Contract

### Added
- Add GitHub Actions release-platform contract documentation.
- Confirm GitHub Actions is the intended future release platform.
- Document that release execution remains blocked pending later roadmap gates.

### Changed
- Add push-to-main validation triggers to existing validation workflows.
- Confirm validation workflows use least-privilege `contents: read` permissions.
- Align README, AGENTS, checklist, and roadmap surfaces with Phase 186 release-platform authority boundaries.

### Notes
- No release workflow was added.
- No tag was created.
- No GitHub Release was created.
- No public artifact path was created.
- No signing path was created.
- No publishing path was created.
- No installer path was created.
- No update-channel path was created.
- No deployment path was created.
- No license or versioning normalization was attempted.

---
truth_dimension: historical
authority_level: authoritative
mutation_path: changelog_entry
---

## v0.0.185 - 2026-06-10
**Status:** Phase 185 - Release Stewardship Checkpoint and v1.0 Gap Registration

### Added
- Add a Phase 185 release-readiness operations document recording that Release Candidate stewardship may continue with caveats while production readiness, public-release readiness, and v1.0 readiness remain blocked.
- Add an explicit post-185 v1.0 release-platform block covering Phases 186-201, with GitHub Actions kept as the intended release platform.
- Update the active checklist to Phase 185 procedural truth and add v1.0 release execution gates.
- Add release-discipline reminders to `AGENTS.md`.

### Changed
- Update README status language from Pre-Alpha to Release Candidate stewardship / pre-v1.0 hardening without claiming production, public-release, or v1.0 readiness.
- Keep `_old/` ignored as a local archive path.

### Notes
- Confirmed local validation passed for the Phase 185 documentation and release-readiness surfaces.
- Confirmed GitHub PR checks passed for the Phase 184.2 PR head commit.
- Confirmed the current main merge commit has no push-triggered Actions run because existing workflows do not trigger on push to main.
- Recorded release-platform blockers: no GitHub Releases or tags, unprotected main branch, no push-to-main CI trigger, no release workflow, no license file, package-version/changelog-version mismatch, inactive signing/publishing/installer/update-channel/public-artifact/deployment paths, and prior oversized-module/string-heuristic maintainability risks.
- No GitHub Release created.
- No release tag created.
- No release artifact created.
- No public artifact created.
- No signing path activated.
- No publishing path activated.
- No installer behavior added.
- No update-channel behavior added.
- No deployment path added.
- No production readiness claimed.
- No public-release readiness claimed.
- No v1.0 readiness claimed.

---
truth_dimension: historical
authority_level: authoritative
mutation_path: changelog_entry
---

## v0.0.184.2 - 2026-05-27
**Status:** Phase 184.2 - OOB Security Audit - Remaining Heuristic Triage

### Added
- Add a remaining-heuristic triage document classifying residual string/status heuristic scan hits by authority sensitivity, accepted non-authority usage, test-only usage, documentation/copy usage, deferred refactor status, and false-positive status.

### Fixed
- Repair authority-sensitive heuristic cases found during triage where the fix is narrow and safe.

### Notes
- Out-of-band security triage and hardening only.
- No Phase 185 implementation.
- No Release Candidate local package rehearsal feature continuation.
- No release artifact creation.
- No public artifact creation.
- No public package creation.
- No signing.
- No publishing.
- No deployment.
- No public download.
- No GitHub release.
- No release tag.
- No installer activation.
- No update-channel activation.
- No provider trust.
- No action authorization.
- No replay repair.
- No recovery promotion.
- No production/public-use approval.
- No release readiness approval.

---
truth_dimension: historical
authority_level: authoritative
mutation_path: changelog_entry
---

## v0.0.183.1 - 2026-05-24
**Status:** Phase 183.1 - OOB Release Candidate Hardening Closure Validation

### Fixed
- Apply rustfmt formatting to the Phase 183 hardening closure Rust surface so cargo fmt --check passes.
- Close the Phase 183 validation gap by running the full repository check from a clean committed tree.
- Preserve the Release Candidate hardening closure projection, UI panel, typed-hardening behavior, and no-authority boundaries.

### Notes
- Out-of-band validation closure only.
- No Phase 184 implementation.
- No release artifact creation.
- No public artifact creation.
- No signing.
- No publishing.
- No deployment.
- No public download.
- No GitHub release.
- No release tag.
- No installer activation.
- No update-channel activation.
- No provider trust.
- No action authorization.
- No replay repair.
- No recovery promotion.
- No production/public-use approval.
- No release readiness approval.
- Phase 184 remains the next code-production phase.


## v0.0.183 - 2026-05-23
**Status:** Phase 183 - Release Candidate Hardening Closure

### Added
- Add Rust-owned Release Candidate hardening closure projection with hardening item status, severity, category, linked evidence/review finding, caveats, blockers, deterministic closure ID, and no-authority boundaries.
- Add UI rendering for Release Candidate hardening closure status, hardening item list, severity, category, status, linked evidence/review finding, caveats, blockers, and no-approval/no-readiness wording.
- Add Rust and TypeScript tests for review findings, manifest caveats/blockers, targeted cleanup caveats, deterministic ordering, closure status, and non-authority boundaries.

### Changed
- Extend the local operator shell projection with Release Candidate hardening closure state using thin integration.
- Update the local UI shell to display Release Candidate hardening closure state.
- Update checklists/current-phase.md to Phase 183 procedural truth.

### Notes
- Code-production phase.
- Hardening closure records which Release Candidate issues are closed or still open.
- Hardening closure does not approve Release Candidate status.
- Hardening closure does not approve release readiness.
- Hardening closure does not approve production readiness.
- Hardening closure does not approve public use.
- Hardening closure does not create release artifacts.
- Hardening closure does not create public artifacts.
- Hardening closure does not sign.
- Hardening closure does not publish.
- Hardening closure does not deploy.
- Hardening closure does not release.
- Hardening closure does not distribute.
- No public download.
- No GitHub release.
- No release tag.
- No installer activation.
- No update-channel activation.
- No provider trust approval.
- No action authorization.
- No replay repair.
- No recovery promotion.
- No production persistence.
- No public-use approval.
- No readiness approval.
- Phase 184 remains the next code-production phase for Release Candidate Local Package Rehearsal.


## v0.0.182.2 - 2026-05-23
**Status:** Phase 182.2 - OOB Release Candidate Review UI Validation Closure

### Fixed
- Close the Phase 182.1 validation gap by running the full repository check from a clean committed tree.
- Preserve the Release Candidate review UI microcopy, typed-hardening, and no-authority boundaries.

### Notes
- Out-of-band validation closure only.
- No Phase 183 implementation.
- No release artifact creation.
- No public artifact creation.
- No signing.
- No publishing.
- No deployment.
- No public download.
- No GitHub release.
- No release tag.
- No installer activation.
- No update-channel activation.
- No provider trust.
- No action authorization.
- No replay repair.
- No recovery promotion.
- No production/public-use approval.
- No release readiness approval.
- Phase 183 remains the next code-production phase.


## v0.0.182.1 - 2026-05-23
**Status:** Phase 182.1 - OOB Release Candidate Review UI Completion and Microcopy Pass

### Fixed
- Complete Phase 182 Release Candidate review validation by adding focused Rust tests for manifest summary, caveat summary, blocker summary, upstream linkage, validation summary, review findings, deterministic ordering, and no-authority boundaries.
- Add TypeScript behavior tests for the Release Candidate review panel, manifest summary, caveats, blockers, upstream linkage, validation summary, review findings, empty-state copy, blocker/error-state copy, normal review-state copy, deterministic rendering, and no-authority wording.
- Add a plain-English microcopy pass for the Release Candidate review UI.
- Run and close the full Rust/UI validation stack for the Release Candidate review surface.

### Notes
- Out-of-band completion fix only.
- No Phase 183 implementation.
- No release artifact creation.
- No public artifact creation.
- No signing.
- No publishing.
- No deployment.
- No public download.
- No GitHub release.
- No release tag.
- No installer activation.
- No update-channel activation.
- No provider trust.
- No action authorization.
- No replay repair.
- No recovery promotion.
- No production/public-use approval.
- No release readiness approval.
- Phase 183 remains the next code-production phase.


## v0.0.182 - 2026-05-23
**Status:** Phase 182 - Release Candidate Review UI

### Added
- Add Rust-owned Release Candidate review projection with review sections, manifest summary, caveat summary, blocker summary, upstream linkage summary, validation summary, review findings, and no-authority boundaries.
- Add UI rendering for Release Candidate review status, manifest summary, caveats, blockers, upstream linkage, validation summary, review findings, and no-approval/no-readiness wording.
- Add Rust and TypeScript tests for valid manifest review state, blocked/incomplete manifest handling, deterministic review ordering, targeted-cleanup findings, and non-authority boundaries.

### Changed
- Extend the local operator shell projection with Release Candidate review state using thin integration.
- Update the local UI shell to display Release Candidate review state.
- Update checklists/current-phase.md to Phase 182 procedural truth.

### Notes
- Code-production phase.
- Release Candidate review UI makes the supportability evidence inspectable.
- Review does not approve Release Candidate status.
- Review does not approve release readiness.
- Review does not approve production readiness.
- Review does not approve public use.
- Review does not create release artifacts.
- Review does not create public artifacts.
- Review does not sign.
- Review does not publish.
- Review does not deploy.
- Review does not release.
- Review does not distribute.
- No public download.
- No GitHub release.
- No release tag.
- No installer activation.
- No update-channel activation.
- No provider trust approval.
- No action authorization.
- No replay repair.
- No recovery promotion.
- No production persistence.
- No public-use approval.
- No readiness approval.
- Phase 183 remains the next code-production phase for Release Candidate Hardening Closure.

---
truth_dimension: historical
authority_level: authoritative
mutation_path: changelog_entry
---

## v0.0.181.3 - 2026-05-23
**Status:** Phase 181.3 - OOB Release Candidate Evidence Manifest Validation Closure

### Fixed
- Close the Phase 181.2 validation gap by running the full repository check from a clean committed tree.
- Preserve the Phase 181.2 Release Candidate evidence manifest behavior and no-authority boundaries.

### Notes
- Out-of-band validation closure only.
- No Phase 182 implementation.
- No release artifact creation.
- No public artifact creation.
- No signing.
- No publishing.
- No deployment.
- No public download.
- No GitHub release.
- No release tag.
- No installer activation.
- No update-channel activation.
- No provider trust.
- No action authorization.
- No replay repair.
- No recovery promotion.
- No production/public-use approval.
- No release readiness approval.
- Phase 182 remains the next code-production phase.

## v0.0.181.2 - 2026-05-23
**Status:** Phase 181.2 - OOB Release Candidate Evidence Manifest Completion Fix

### Fixed
- Fix TypeScript local shell state construction by adding the missing initialReleaseCandidateEvidenceManifestProjection helper used by Release Candidate evidence manifest state initialization.
- Complete Phase 181.1 manifest validation by adding focused Rust tests for upstream normalization, missing/rejected evidence blockers, blocked rehearsal/gap blockers, deterministic ordering, targeted-cleanup caveats, and no-authority boundaries.
- Add TypeScript behavior tests for Release Candidate manifest panel rendering, supportability label, manifest ID, upstream linkage, blockers, caveats, deterministic rendering, required initial state shape, and forbidden-label absence.
- Run and close the full Rust/UI validation stack for the Release Candidate evidence manifest surface.

### Notes
- Out-of-band completion fix only.
- No Phase 182 implementation.
- No release artifact creation.
- No public artifact creation.
- No signing.
- No publishing.
- No deployment.
- No public download.
- No GitHub release.
- No release tag.
- No installer activation.
- No update-channel activation.
- No provider trust.
- No action authorization.
- No replay repair.
- No recovery promotion.
- No production/public-use approval.
- No release readiness approval.
- Phase 182 remains the next code-production phase.

## v0.0.181.1 - 2026-05-23
**Status:** Phase 181.1 - Release Candidate Label and Evidence Manifest

### Added
- Add Rust-owned Release Candidate label and evidence manifest projection with deterministic manifest ID, evidence item categories, source linkage, blockers, caveats, validation summaries, and no-authority boundaries.
- Add manifest-specific normalization helpers based on the Phase 181.0 Rust projection shape map.
- Add UI rendering for Release Candidate supportability label, evidence manifest status, upstream linkage, blockers, caveats, validation summary, and no-release/no-production/no-public-use wording.
- Add Rust and TypeScript tests for valid upstream evidence, missing/rejected upstream evidence, deterministic manifest ordering, targeted-cleanup caveats, typed normalization helpers, and non-authority boundaries.

### Changed
- Extend the local operator shell projection with Release Candidate evidence manifest state using thin integration.
- Update the local UI shell to display Release Candidate label and evidence manifest state.
- Carry Phase 180.2 targeted cleanup requirements into the Release Candidate stewardship path.
- Update checklists/current-phase.md to Phase 181.1 procedural truth.

### Notes
- Code-production phase.
- Release Candidate evidence manifest records supportability evidence.
- Release Candidate supportability is not release readiness.
- Release Candidate supportability is not production readiness.
- Release Candidate supportability is not public-use approval.
- This manifest does not create release artifacts.
- This manifest does not create public artifacts.
- This manifest does not sign.
- This manifest does not publish.
- This manifest does not deploy.
- This manifest does not release.
- This manifest does not distribute.
- No public download.
- No GitHub release.
- No release tag.
- No installer activation.
- No update-channel activation.
- No provider trust approval.
- No action authorization.
- No replay repair.
- No recovery promotion.
- No production persistence.
- No public-use approval.
- No readiness approval.
- Phase 182 remains the next code-production phase for Release Candidate Review UI.

## v0.0.181.0 - 2026-05-23
**Status:** Phase 181.0 - OOB Release Candidate Manifest Shape Reconnaissance

### Added
- Add a shape reconnaissance document mapping the current Rust and TypeScript projection types needed for Phase 181 Release Candidate label and evidence manifest implementation.

### Notes
- Out-of-band reconnaissance only.
- No Rust source changes.
- No TypeScript source changes.
- No runtime behavior.
- No tests changed.
- No schema changes.
- No roadmap changes.
- No signing.
- No publishing.
- No deployment.
- No release readiness approval.
- No Release Candidate approval.
- Phase 181 remains the next code-production phase.


## v0.0.180.2 - 2026-05-23
**Status:** Phase 180.2 - OOB Production Release Path and Stale Code Audit

### Added
- Add a production-release-path and stale-code audit covering Phase 181-185 readiness, stale code, brittle logic, unused or weak tests, module-size risk, release-boundary drift, documentation truth-dimension drift, and production-release blockers.

### Notes
- Out-of-band audit only.
- No Rust source changes.
- No TypeScript source changes.
- No test changes.
- No schema changes.
- No roadmap changes.
- No runtime behavior.
- No signing.
- No publishing.
- No deployment.
- No public distribution.
- No production/public-use approval.
- No Release Candidate decision change.
- Phase 181 remains deferred until the audit recommendation is recorded.

## v0.0.180.1 - 2026-05-23
**Status:** Phase 180.1 - OOB Release Candidate Decision Validation Closure

### Fixed
- Close the Phase 180 validation gap by running the full repository check from a clean committed tree.
- Preserve the Phase 180 decision: release_candidate_status_supportable_with_caveats.
- Preserve the Phase 181-185 stewardship block mapping.

### Notes
- Out-of-band validation closure only.
- No Rust source changes.
- No TypeScript source changes.
- No test changes.
- No schema changes.
- No runtime behavior.
- No Phase 181 implementation.
- No release artifact creation.
- No signing.
- No publishing.
- No deployment.
- No public download.
- No update-channel activation.
- No production/public-use approval.
- Phase 181 remains the next code-production phase.

## v0.0.180 - 2026-05-22
**Status:** Phase 180 - Release Candidate Decision Gate

### Changed
- Reconcile the Phase 171-179.3 release-candidate evidence chain.
- Record the Phase 180 Release Candidate decision.
- Record blocker, evidence-gap, guardrail-drift, authority-boundary, and rebuild-trigger findings.
- Update roadmap sequencing based on the Phase 180 decision.
- Update checklists/current-phase.md to Phase 180 procedural truth.

### Notes
- Decision/alignment checkpoint only.
- No Rust source changes.
- No TypeScript source changes.
- No test changes.
- No schema changes.
- No runtime behavior.
- No signing behavior.
- No publishing behavior.
- No installer behavior.
- No update-channel activation.
- No public distribution.
- No deployment behavior.
- No release artifact creation.
- No public artifact creation.
- No provider execution expansion.
- No provider trust approval.
- No action authorization.
- No replay repair.
- No recovery promotion.
- Release Candidate supportability is not production readiness.
- Release Candidate supportability is not public/general-use approval.
- Release Candidate supportability is not deployment approval.
- Release Candidate supportability does not create artifacts.


## v0.0.179.3 - 2026-05-22
**Status:** Phase 179.3 - OOB Dry-Run Rehearsal Validation Closure

### Fixed
- Close the Phase 179.2 validation gap by running the full repository check from a clean committed tree.

### Notes
- Out-of-band validation closure only.
- No Phase 180 implementation.
- No signing.
- No signature creation.
- No publishing.
- No deployment.
- No public distribution.
- No release artifact creation.
- No public artifact creation.
- No release readiness approval.
- No Release Candidate approval.
- Phase 180 remains the next decision gate.

## v0.0.179.2 - 2026-05-22
**Status:** Phase 179.2 - OOB Dry-Run Rehearsal UI Forbidden-Label Fix

### Fixed
- Fix the Phase 179.1 UI forbidden-label assertion so allowed denial markers such as no_signature_created do not collide with forbidden positive labels such as signature_created.
- Preserve forbidden-label detection for signing, signature creation, publishing, deployment, release readiness, Release Candidate approval, public distribution, provider trust, action authorization, replay repair, and recovery promotion claims.

### Notes
- Out-of-band validation fix only.
- No Phase 180 implementation.
- No release-candidate dry-run rehearsal behavior expansion.
- No signing behavior.
- No signature creation.
- No publishing.
- No deployment.
- No public distribution.
- No release artifact creation.
- No public artifact creation.
- No release readiness approval.
- No Release Candidate approval.
- Phase 180 remains the next decision gate.

## v0.0.179.1 - 2026-05-22
**Status:** Phase 179.1 - OOB Release Candidate Dry-Run Rehearsal Completion Fix

### Fixed
- Complete Phase 179 dry-run rehearsal validation by replacing loose status-string inference with explicit upstream rehearsal classification helpers where practical.
- Add Rust tests for missing/rejected upstream evidence, blocking/informational gap review states, deterministic ordering, deterministic rehearsal ID, and no-authority boundaries.
- Add TypeScript behavior tests for the Release Candidate dry-run rehearsal panel, upstream linkage, missing/blocker summaries, gap review summary, artifact summary, deterministic rendering, and no-authority wording.

### Notes
- Out-of-band completion fix only.
- No Phase 180 implementation.
- No Release Candidate approval.
- No release readiness approval.
- No signing.
- No publishing.
- No deployment.
- No release artifact creation.
- No public artifact creation.
- No public download.
- No GitHub release.
- No release tag.
- No installer activation.
- No update-channel activation.
- No public distribution.
- No provider trust.
- No action authorization.
- No replay repair.
- No recovery promotion.

## v0.0.179 - 2026-05-22
**Status:** Phase 179 - Release Candidate Dry-Run Rehearsal

### Added
- Add Rust-owned Release Candidate dry-run rehearsal projection with deterministic rehearsal ID, upstream evidence linkage, missing evidence, blockers, gap review summary, rehearsal evidence artifact summary, and no-approval boundaries.
- Add UI rendering for Release Candidate dry-run rehearsal status, rehearsal ID, upstream linkage, missing evidence, blockers, gap review summary, rehearsal evidence artifact summary, and no-release/no-public-artifact wording.
- Add Rust and TypeScript tests for valid upstream evidence, missing/rejected upstream evidence, blocking gaps, deterministic ordering, and non-authority boundaries.

### Changed
- Extend the local operator shell projection with Release Candidate dry-run rehearsal state using thin integration.
- Update the local UI shell to display Release Candidate dry-run rehearsal state.
- Update checklists/current-phase.md to Phase 179 procedural truth.

### Notes
- Code-production phase.
- Release Candidate dry-run rehearsal exercises the evidence chain.
- Dry-run rehearsal does not create or approve a Release Candidate.
- Dry-run rehearsal does not approve release readiness.
- Dry-run rehearsal does not create release artifacts.
- Dry-run rehearsal does not create public artifacts.
- Dry-run rehearsal does not sign.
- Dry-run rehearsal does not publish.
- Dry-run rehearsal does not deploy.
- Dry-run rehearsal does not release.
- Dry-run rehearsal does not distribute.
- No public download.
- No GitHub release.
- No release tag.
- No installer activation.
- No update-channel activation.
- No provider trust approval.
- No action authorization.
- No replay repair.
- No recovery promotion.
- No production persistence.
- No public-use approval.
- No readiness approval.
- Phase 180 remains the next Release Candidate decision gate.

---
truth_dimension: historical
authority_level: authoritative
mutation_path: changelog_entry
---

## v0.0.178.3 - 2026-05-22
**Status:** Phase 178.3 - OOB Core Purpose Audit Validation Closure

### Fixed
- Close the Phase 178.2 core-purpose drift audit validation gap by running the full repository check from a clean committed tree.
- Preserve the Phase 178.2 audit recommendation: phase_179_can_proceed_with_caveats.
- Confirm no rebuild trigger was found in the Phase 178.2 audit record.

### Notes
- Out-of-band validation closure only.
- No Rust source changes.
- No TypeScript source changes.
- No test changes.
- No schema changes.
- No runtime behavior.
- No roadmap changes.
- No signing behavior.
- No publishing behavior.
- No deployment behavior.
- No public distribution.
- No release approval.
- No Release Candidate approval.
- No production/public-use approval.
- Phase 179 remains the next code-production phase.

## v0.0.178.2 - 2026-05-22
**Status:** Phase 178.2 - OOB Core Purpose and Drift Audit

### Added
- Add a top-down core purpose and drift audit covering determinism, bounded control, non-authority boundaries, provider/execution guardrails, release-path guardrails, UI honesty, documentation truth dimensions, architecture/modularity, and validation integrity.

### Notes
- Out-of-band audit only.
- No Rust source changes.
- No TypeScript source changes.
- No runtime behavior changes.
- No tests changed.
- No schema changes.
- No roadmap changes.
- No signing.
- No publishing.
- No deployment.
- No public distribution.
- No release readiness approval.
- No Release Candidate approval.
- Phase 179 remains deferred until the audit recommendation is recorded.

## v0.0.178.1 - 2026-05-22
**Status:** Phase 178.1 - OOB Release Candidate Gap Review Validation Closure

### Fixed
- Close Phase 178 validation by running the full repository check from a clean committed tree.
- Fix only validation failures discovered in the Phase 178 release-candidate gap review surface.

### Notes
- Out-of-band validation closure only.
- No Phase 179 implementation.
- No Release Candidate approval.
- No release readiness approval.
- No signing.
- No publishing.
- No deployment.
- No public distribution.
- No public download.
- No GitHub release.
- No release tag.
- No installer activation.
- No update-channel activation.
- No provider trust.
- No action authorization.
- No replay repair.
- No recovery promotion.
- Phase 179 remains the next code-production phase.

## v0.0.178 - 2026-05-22
**Status:** Phase 178 - Release Candidate Gap Review and Hardening

### Added
- Add Rust-owned release-candidate gap review projection with deterministic gap categories, severities, affected surfaces, upstream linkage, missing evidence, blockers, hardening candidates, and no-approval boundaries.
- Add UI rendering for Release Candidate gap review status, gap list, severity, affected surface, hardening candidate summary, upstream linkage, missing evidence, blockers, and no-release-readiness/no-approval wording.
- Add Rust and TypeScript tests for complete upstream evidence, missing/rejected upstream evidence, deterministic gap ordering, deterministic hardening candidate ordering, and non-authority boundaries.

### Changed
- Extend the local operator shell projection with release-candidate gap review state using thin integration.
- Update the local UI shell to display release-candidate gap review and hardening candidate state.
- Split root CHANGELOG.md so it stays under 900 lines while preserving archived historical changelog entries.
- Update checklists/current-phase.md to Phase 178 procedural truth.

### Notes
- Code-production phase.
- Gap review turns assembled evidence into hardening work.
- Gap review does not approve Release Candidate status.
- Gap review does not approve release readiness.
- Gap review does not sign.
- Gap review does not publish.
- Gap review does not deploy.
- Gap review does not release.
- Gap review does not distribute.
- No public download.
- No GitHub release.
- No release tag.
- No installer activation.
- No update-channel activation.
- No provider trust approval.
- No action authorization.
- No replay repair.
- No recovery promotion.
- No production persistence.
- No public-use approval.
- No readiness approval.
- Phase 179 remains the next code-production phase for Release Candidate Dry-Run Rehearsal.

---

## v0.0.178.2 - 2026-05-22
**Status:** Phase 178.2 - OOB Core Purpose and Drift Audit

### Added
- Add a top-down core purpose and drift audit covering determinism, bounded control, non-authority boundaries, provider/execution guardrails, release-path guardrails, UI honesty, documentation truth dimensions, architecture/modularity, and validation integrity.

### Notes
- Out-of-band audit only.
- No Rust source changes.
- No TypeScript source changes.
- No runtime behavior changes.
- No tests changed.
- No schema changes.
- No roadmap changes.
- No signing.
- No publishing.
- No deployment.
- No public distribution.
- No release readiness approval.
- No Release Candidate approval.
- Phase 179 remains deferred until the audit recommendation is recorded.

truth_dimension: historical
authority_level: authoritative
mutation_path: changelog_entry
---

## v0.0.177.1 - 2026-05-21
**Status:** Phase 177.1 - OOB Release Candidate Evidence Assembly Formatting Fix

### Fixed
- Apply rustfmt formatting to the Phase 177 release-candidate evidence assembly Rust surfaces so cargo fmt --check passes.

### Notes
- Out-of-band formatting fix only.
- No runtime behavior changes.
- No evidence assembly semantic changes.
- No TypeScript behavior changes.
- No schema changes.
- No signing.
- No publishing.
- No deployment.
- No public distribution.
- No readiness approval.
- No Release Candidate approval.
- Phase 178 remains the next code-production phase after Phase 177 validation closure.

## v0.0.176 - 2026-05-19
**Status:** Phase 176 - Signing and Key-Custody Dry Run

### Added
- Add Rust-owned signing/key-custody dry-run evidence derivation and validation from release-candidate preparation, dry package, checksum/provenance, and installer/distribution contract evidence.
- Add placeholder key metadata, upstream evidence linkage, missing evidence, blockers, validation errors, disabled capability surface, and no-real-signing/no-release boundary markers.
- Add UI rendering for signing/key-custody dry-run status, upstream evidence linkage, placeholder key metadata, missing/blocked evidence, and no-real-signing/no-release boundaries.
- Add Rust and TypeScript tests for valid upstream mapping, missing/rejected evidence, real-key/signature/public-signing claim rejection, deterministic dry-run evidence, and non-authority boundaries.

### Changed
- Extend the local operator shell projection with signing/key-custody dry-run status using thin integration.
- Update the local UI shell to display signing/key-custody dry-run evidence state.
- Update checklists/current-phase.md to Phase 176 procedural truth.

### Notes
- Code-production phase.
- Signing/key-custody dry run uses placeholder key metadata only.
- No real signing keys.
- No private keys.
- No certificate material.
- No KMS binding.
- No secret material.
- No signature creation.
- No public signing.
- No signing.
- No publishing.
- No release artifact creation.
- No public artifact creation.
- No installer activation.
- No update-channel activation.
- No public download.
- No GitHub release.
- No release tag.
- No deployment behavior.
- No provider trust approval.
- No action authorization.
- No replay repair.
- No recovery promotion.
- No production persistence.
- No public-use approval.
- No readiness approval.
- Phase 177 remains the next code-production phase for Release Candidate Evidence Assembly UI.

## v0.0.175.1 - 2026-05-19
**Status:** Phase 175.1 - OOB Alignment Checkpoint Validation Closure

### Fixed
- Close the Phase 175 validation gap by rerunning the full repository check from a clean committed tree.
- Record the successful clean-tree validation result in checklists/current-phase.md.

### Notes
- Out-of-band validation closure only.
- No Rust source changes.
- No TypeScript source changes.
- No test changes.
- No schema changes.
- No runtime behavior.
- No signing behavior.
- No publishing behavior.
- No installer behavior.
- No update-channel activation.
- No deployment behavior.
- No public distribution.
- No release artifact creation.
- No readiness approval.
- No Release Candidate approval.
- Phase 176 remains the next code-production phase.

## v0.0.175 - 2026-05-19
**Status:** Phase 175 - Code-Production Alignment Checkpoint

### Changed
- Reconcile the Phase 171-174.2 release-candidate-preparation block.
- Confirm the current release-candidate-preparation path includes release-candidate preparation contract, dry package rehearsal evidence, checksum/provenance evidence, installer/distribution contract surface, and OOB validation closure fixes.
- Decide whether Phase 176 may proceed toward signing and key-custody dry-run work.
- Confirm Phase 176-180 toward signing/key-custody dry run, release-candidate evidence assembly UI, gap hardening, release-candidate dry-run rehearsal, and Release Candidate decision gate.
- Preserve the post-Phase-175 rule that non-0/5 phases must produce product-facing code, executable artifacts, release-candidate preparation artifacts, signing/key-custody dry-run surfaces, release evidence review UI, concrete hardening code, or deterministic validation improvements.
- Update checklists/current-phase.md to Phase 175 procedural truth.

### Notes
- Alignment checkpoint only.
- No Rust source changes.
- No TypeScript source changes.
- No test changes.
- No schema changes.
- No runtime behavior.
- No provider execution expansion.
- No persistence implementation.
- No signing behavior.
- No publishing behavior.
- No installer behavior.
- No update-channel activation.
- No public distribution.
- No release artifact creation.
- No deployment behavior.
- No public-use or readiness approval.
- Release-candidate preparation is not release readiness.
- Dry package is rehearsal evidence, not a release artifact.
- Checksum/provenance evidence is not signing.
- Installer/distribution contract describes constraints only.
- Phase 176 is expected to resume code production.


## v0.0.174.2 - 2026-05-19
**Status:** Phase 174.2 - OOB Frontmatter Validation Fix

### Fixed
- Restore required frontmatter metadata for CHANGELOG.md and checklists/current-phase.md so structure/documentation validation passes.

### Notes
- Out-of-band validation repair only.
- No Rust changes.
- No TypeScript changes.
- No runtime behavior changes.
- No tests changed.
- No roadmap changes.
- No installer behavior.
- No update-channel activation.
- No signing.
- No publishing.
- No deployment.
- No public distribution.
- No readiness approval.
- No Release Candidate approval.
- Phase 175 remains the next alignment checkpoint after Phase 174.2 validation closure.

## v0.0.174.0 - 2026-05-18
**Status:** Phase 174.0 - OOB Installer Distribution Formatting Fix

### Fixed
- Apply rustfmt formatting to the Phase 174 installer/distribution contract Rust surfaces so cargo fmt --check passes.

### Notes
- Out-of-band formatting fix only.
- No runtime behavior changes.
- No validation behavior changes.
- No TypeScript changes.
- No schema changes.
- No installer behavior.
- No update-channel activation.
- No signing.
- No publishing.
- No deployment.
- No public distribution.
- No readiness approval.
- No Release Candidate approval.
- Phase 174.1 remains required to complete validation, missing evidence, blocker, Rust test, and TypeScript test coverage.

## v0.0.174 - 2026-05-18
**Status:** Phase 174 - Installer and Distribution Contract Surface

### Added
- Add Rust-owned installer/distribution contract derivation and validation from dry package and checksum/provenance evidence.
- Add contract metadata, projection, status, classification, dry package linkage, checksum/provenance linkage, missing evidence, blockers, validation errors, capability surface, and boundary markers.
- Add UI rendering for installer/distribution contract status, dry package linkage, checksum/provenance linkage, blocked/missing evidence, and no-installer/no-distribution/no-update-channel boundaries.
- Add Rust and TypeScript tests for valid linkage, missing dry package rejection, missing checksum/provenance rejection, installer/update/public-distribution claim rejection, and non-authority boundaries.

### Changed
- Extend the local operator shell projection with installer/distribution contract status using thin integration.
- Update the local UI shell to display installer/distribution contract state and evidence linkage.
- Update checklists/current-phase.md to Phase 174 procedural truth.

### Notes
- Code-production phase.
- The installer/distribution contract describes constraints only.
- The contract does not create an installer.
- The contract does not activate an update channel.
- The contract does not sign, publish, deploy, release, or distribute anything.
- No public download.
- No GitHub release.
- No release tag.
- No public distribution.
- No release readiness approval.
- No Release Candidate status approval.
- No Production Candidate approval.
- No production-use approval.
- No provider trust approval.
- No action authorization.
- No replay repair.
- No recovery promotion.
- No production persistence.
- No public-use approval.
- No readiness approval.
- Phase 175 remains the next alignment checkpoint.

---


## v0.0.177 - 2026-05-20
**Status:** Phase 177 - Release Candidate Evidence Assembly UI

### Added
- Add Rust-owned release-candidate evidence assembly projection with deterministic evidence categories, source linkage, missing evidence, blockers, validation summary, and no-approval boundaries.
- Add UI rendering for Release Candidate evidence assembly status, category sections, upstream linkage, missing evidence, blockers, validation summaries, and no-release-readiness/no-approval wording.
- Add Rust and TypeScript tests for complete upstream evidence rendering, missing/rejected upstream evidence handling, deterministic ordering/rendering, and non-authority boundaries.

### Changed
- Extend the local operator shell projection with release-candidate evidence assembly state using thin integration.
- Update the local UI shell to display release-candidate evidence assembly review state.
- Update checklists/current-phase.md to Phase 177 procedural truth.

### Notes
- Code-production phase.
- Evidence assembly organizes release-candidate evidence for review.
- Evidence assembly does not approve Release Candidate status.
- Evidence assembly does not approve release readiness.
- Evidence assembly does not sign.
- Evidence assembly does not publish.
- Evidence assembly does not deploy.
- Evidence assembly does not release.
- Evidence assembly does not distribute.
- No public download.
- No GitHub release.
- No release tag.
- No installer activation.
- No update-channel activation.
- No provider trust approval.
- No action authorization.
- No replay repair.
- No recovery promotion.
- No production persistence.
- No public-use approval.
- No readiness approval.
- Phase 178 remains the next code-production phase for Release Candidate Gap Review and Hardening.

## v0.0.184.S - 2026-05-27
**Status:** Phase 184.S - OOB Security Audit and Release-Authority Boundary Review

### Added
- Add a security audit covering authority boundaries, execution boundaries, secret handling, release boundaries, replay/persistence behavior, UI honesty, brittle string/status heuristics, test integrity, documentation drift, and supply-chain surfaces.

### Notes
- Out-of-band audit only.
- No Rust source changes.
- No TypeScript source changes.
- No runtime behavior changes.
- No release artifact creation.
- No signing.
- No publishing.
- No deployment.
- No public distribution.
- No production/public-use approval.
- Phase 184 remains paused until this audit recommendation is reviewed.

## v0.0.184.1 - 2026-05-27
**Status:** Phase 184.1 - OOB Security Audit - Authority Classification Hardening

### Fixed
- Replace or isolate brittle authority-adjacent string/status heuristics with exact-match, typed, fail-closed authority classification helpers.
- Add adversarial Rust and TypeScript tests for denial-marker handling, positive authority claims, token-boundary collisions, casing mutations, provider trust claims, action authorization claims, replay repair claims, and recovery promotion claims.
- Preserve the Phase 184.S security-audit decision that the release path remains paused until security repairs are reviewed.

### Notes
- Out-of-band security-hardening phase.
- No Phase 185 implementation.
- No Release Candidate local package rehearsal feature continuation.
- No release artifact creation.
- No public artifact creation.
- No public package creation.
- No signing.
- No publishing.
- No deployment.
- No public download.
- No GitHub release.
- No release tag.
- No installer activation.
- No update-channel activation.
- No provider trust.
- No action authorization.
- No replay repair.
- No recovery promotion.
- No production/public-use approval.
- No release readiness approval.
- Release path remains paused pending hardening review.
