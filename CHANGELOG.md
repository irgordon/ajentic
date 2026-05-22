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
