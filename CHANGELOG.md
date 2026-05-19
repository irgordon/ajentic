---
truth_dimension: historical
authority_level: authoritative
mutation_path: changelog_entry
---

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
