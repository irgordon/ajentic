---
truth_dimension: historical
authority_level: authoritative
mutation_path: changelog_entry
---

# CHANGELOG

Historical truth surface for the active development range.

Archived historical ranges:
- docs/changelog/CHANGELOG-0001-0055.md (v0.0.1 through v0.0.55; includes legacy v0.0.0 bootstrap entry to prevent historical omission)
- docs/changelog/CHANGELOG-0056-0104.md (v0.0.56 through v0.0.104)

Archive guarantees:
- Historical entries are partitioned without changing their recorded wording, timestamps, ordering within each deterministic archive extraction, headings, or semantic interpretation.
- Archived entries are not duplicated in this active changelog.
- The active changelog begins with v0.0.104.5 and later entries only.
- CHANGELOG surfaces remain historical truth.

## v0.0.135 - 2026-05-11
**Status:** Phase 135 - Roadmap and Changelog Alignment Check

### Added
- Add the Phase 135 roadmap and changelog alignment report.
- Add the planned Phase 135.1 follow-up for artifact-chain correction before installer/update-channel work.

### Changed
- Update checklists/current-phase.md to Phase 135 procedural truth.
- Update CHANGELOG.md with v0.0.135.
- Update roadmap planned-truth surfaces only if required to record Phase 135.1 or clarify Phase 136 deferment.

### Notes
- Phase 135 is alignment only.
- Alignment is not implementation.
- Roadmap is not implementation.
- Requirements are not evidence.
- Evidence is not approval.
- Phase 130 rc_candidate_not_ready is preserved.
- Artifact creation remains blocked or deferred.
- Checksum/provenance evidence remains blocked or deferred.
- Signing/key-custody evidence remains blocked or deferred.
- Phase 136 remains mapped, but implementation must not proceed until Phase 135.1 resolves or explicitly defers the artifact-chain dependency.
- No artifact creation.
- No package creation.
- No checksum generation.
- No provenance attestation creation.
- No signing behavior.
- No key creation.
- No certificate creation.
- No signature creation.
- No publishing behavior.
- No deployment automation.
- No production deployment behavior.
- No installer/update-channel activation.
- No monitoring/logging/telemetry activation.
- No provider trust.
- No provider output promotion.
- No persistence authority expansion.
- No replay repair.
- No recovery promotion.
- No action execution.
- No readiness approval.
- No Release Candidate approval.
- No Production Candidate approval.
- No public/general-use approval.
- No production-human-use approval.
- No Phase 136 implementation.
- No Phase 139 implementation.
- No Phase 140 implementation.

## v0.0.134 - 2026-05-10
**Status:** Phase 134 - Signing and Key-Custody Implementation Boundary

### Added
- Add the Phase 134 signing and key-custody boundary report.

### Changed
- Update checklists/current-phase.md to Phase 134 procedural truth.
- Update CHANGELOG.md with v0.0.134.

### Notes
- Phase 134 is signing/key-custody boundary only.
- Signing controls are governance, not signing.
- Key custody is policy, not key creation.
- Verification requirements are not signature verification evidence.
- Missing governed artifact evidence blocks signing.
- Missing checksum evidence blocks signing.
- Missing provenance evidence blocks signing.
- Phase 134 does not create signatures, keys, attestations, release artifacts, or public assets.
- Phase 134 does not activate signing, publishing, distribution, deployment, monitoring, or readiness.
- Phase 134 does not satisfy artifact, checksum, provenance, reassembly, or decision-gate evidence.
- Phase 130 rc_candidate_not_ready is preserved.
- Phase 132 artifact_creation_deferred is preserved.
- Phase 132.1 artifact contract correction remains contract-only.
- Phase 133 checksum_provenance_blocked_by_missing_artifact is preserved.
- No package creation.
- No release artifact creation.
- No checksum generation.
- No provenance attestation creation.
- No signing behavior.
- No key creation.
- No certificate creation.
- No signature creation.
- No publishing behavior.
- No deployment automation.
- No production deployment behavior.
- No installer/update-channel activation.
- No monitoring/logging/telemetry activation.
- No provider trust.
- No provider output promotion.
- No persistence authority expansion.
- No replay repair.
- No recovery promotion.
- No action execution.
- No readiness approval.
- No Release Candidate approval.
- No Production Candidate approval.
- No public/general-use approval.
- No production-human-use approval.
- No Phase 139 implementation.
- No Phase 140 implementation.

## v0.0.133 - 2026-05-10
**Status:** Phase 133 - Checksum and Provenance Evidence Boundary

### Added
- Add the Phase 133 checksum and provenance evidence boundary report.

### Changed
- Update checklists/current-phase.md to Phase 133 procedural truth.
- Update CHANGELOG.md with v0.0.133.

### Notes
- Phase 133 is checksum/provenance evidence boundary only.
- Checksum requirements are not checksum evidence.
- Provenance requirements are not provenance evidence.
- Artifact contract correction is not artifact creation.
- Missing governed artifact evidence blocks checksum/provenance generation.
- Phase 133 must not create artifacts to satisfy checksum/provenance evidence.
- Checksum/provenance evidence is not signing, publishing, release, deployment, or readiness.
- Phase 130 rc_candidate_not_ready is preserved.
- Phase 132 artifact_creation_deferred is preserved.
- Phase 132.1 artifact contract correction remains contract-only.
- No package creation.
- No release artifact creation.
- No checksum generation unless a governed committed local/non-public artifact already exists and is explicitly documented.
- No provenance attestation creation unless a governed committed local/non-public artifact already exists and is explicitly documented.
- No signing behavior.
- No publishing behavior.
- No deployment automation.
- No production deployment behavior.
- No installer/update-channel activation.
- No monitoring/logging/telemetry activation.
- No provider trust.
- No provider output promotion.
- No persistence authority expansion.
- No replay repair.
- No recovery promotion.
- No action execution.
- No readiness approval.
- No Release Candidate approval.
- No Production Candidate approval.
- No public/general-use approval.
- No production-human-use approval.
- No Phase 134 implementation.
- No Phase 139 implementation.
- No Phase 140 implementation.



## v0.0.132.1 - 2026-05-10
**Status:** Out-of-Band Maintenance - Artifact Contract Correction

### Added
- Add the Phase 132.1 artifact contract correction report.
- Define the local/non-public artifact output directory contract.
- Define the deterministic artifact generation command contract.
- Define artifact manifest field requirements for future artifact evidence work.

### Changed
- Update checklists/current-phase.md to Phase 132.1 procedural truth.
- Update CHANGELOG.md with v0.0.132.1.

### Notes
- Out-of-band maintenance only.
- Artifact contract correction is not artifact creation.
- Output directory definition is not artifact output.
- Generation command contract is not command execution.
- Artifact manifest requirements are not artifact manifest evidence.
- Phase 132.1 does not create artifacts, packages, checksums, provenance attestations, signatures, releases, public downloads, or deployment behavior.
- Phase 132.1 does not approve Release Candidate status.
- Phase 132.1 preserves Phase 130 rc_candidate_not_ready.
- Phase 130 rc_candidate_not_ready is preserved.
- Phase 132 artifact_creation_deferred is preserved unless explicitly superseded by future evidence.
- No artifact creation.
- No package creation.
- No checksum generation.
- No provenance attestation creation.
- No signing behavior.
- No publishing behavior.
- No installer/update-channel behavior.
- No GitHub release/tag/public download asset creation.
- No public release behavior.
- No production deployment behavior.
- No monitoring/logging/telemetry activation.
- No runtime behavior.
- No new runtime capability.
- No Rust source changes.
- No TypeScript source changes.
- No test changes.
- No schema changes.
- No provider trust.
- No provider output promotion.
- No persistence authority expansion.
- No replay repair.
- No recovery promotion.
- No action execution.
- No readiness approval.
- No Release Candidate approval.
- No Production Candidate approval.
- No public/general-use approval.
- No production-human-use approval.
- No Phase 133 implementation.
- No Phase 139 implementation.
- No Phase 140 implementation.


## v0.0.132 - 2026-05-10
**Status:** Phase 132 - Release Artifact Creation Boundary

### Added
- Add the Phase 132 release artifact creation boundary report.
- Add controlled local/non-public artifact evidence only if created within the Phase 132 boundary.

### Changed
- Update checklists/current-phase.md to Phase 132 procedural truth.
- Update CHANGELOG.md with v0.0.132.
- Update only narrowly required local artifact generation or manifest surfaces if Phase 132 creates controlled non-public artifact evidence.

### Notes
- Phase 132 is local/non-public artifact evidence only.
- Local artifact creation is not release.
- Artifact evidence is not readiness.
- Artifact manifest evidence is not publication.
- Local artifacts are non-public evidence only.
- Phase 132 does not approve Release Candidate status.
- Phase 132 does not approve Production Candidate status.
- Phase 132 does not approve public/general use.
- Phase 132 does not approve production human use.
- Phase 132 does not create public assets, GitHub releases, release tags, or public downloads.
- Phase 132 does not sign, publish, deploy, or activate installer/update-channel behavior.
- Phase 132 does not satisfy checksum, provenance, signing, installer, update-channel, observability, deployment, Production Candidate, or public/general-use evidence by inference.
- No public release behavior.
- No public asset creation.
- No GitHub release creation.
- No release tag creation.
- No public download creation.
- No signing behavior.
- No publishing behavior.
- No deployment automation.
- No production deployment behavior.
- No installer/update-channel activation.
- No monitoring/logging/telemetry activation.
- No provider trust.
- No provider output promotion.
- No persistence authority expansion.
- No replay repair.
- No recovery promotion.
- No action execution.
- Phase 133 is not implemented.
- Phase 139 is not implemented.
- Phase 140 is not implemented.

## v0.0.131 - 2026-05-10
**Status:** Phase 131 - Post-130 Roadmap Expansion and Release Evidence Remap

### Added
- Add the Phase 131 post-130 release evidence remap report.

### Changed
- Update checklists/current-phase.md to Phase 131 procedural truth.
- Update CHANGELOG.md with v0.0.131.

### Notes
- Phase 131 is audit/planning only.
- Phase 131 preserves Phase 130's rc_candidate_not_ready decision as starting truth.
- Phase 131 does not rerun the Release Candidate decision gate.
- Phase 131 does not implement Phase 132.
- Phase 131-140 are pre-RC evidence-producing phases, not post-RC hardening.
- Roadmap is not implementation.
- Requirements are not evidence.
- Evidence is not approval.
- No runtime behavior.
- No new runtime capability.
- No Rust source changes.
- No TypeScript source changes.
- No test changes.
- No schema changes.
- No package creation.
- No release artifact creation.
- No checksum generation.
- No provenance attestation creation.
- No installer/update-channel behavior.
- No signing/publishing behavior.
- No monitoring/logging/telemetry activation.
- No deployment automation.
- No provider trust.
- No provider output promotion.
- No persistence authority expansion.
- No replay repair.
- No recovery promotion.
- No action execution.
- No readiness approval.
- No Release Candidate approval.
- No Production Candidate approval.
- No public/general-use approval.
- No production-human-use approval.


## v0.0.130.1 - 2026-05-10
**Status:** Out-of-Band Roadmap Expansion After Release Candidate Decision

### Added
- Add the Phase 130.1 post-130 roadmap expansion report.

### Changed
- Update roadmap planned-truth surfaces to map the Phase 131-140 evidence-producing block after the Phase 130 rc_candidate_not_ready decision.
- Update checklists/current-phase.md to Phase 130.1 procedural truth.
- Update CHANGELOG.md with v0.0.130.1.

### Notes
- Out-of-band roadmap alignment only.
- Roadmap expansion is planned truth, not implementation.
- Phase 130 decision status remains rc_candidate_not_ready.
- Phase 131-140 are planned evidence-producing and decision phases, not completed work.
- No runtime behavior.
- No new runtime capability.
- No Rust source changes.
- No TypeScript source changes.
- No test changes.
- No schema changes.
- No package creation.
- No release artifact creation.
- No checksum generation.
- No provenance attestation creation.
- No installer/update-channel behavior.
- No signing/publishing behavior.
- No monitoring/logging/telemetry activation.
- No deployment automation.
- No provider trust.
- No provider output promotion.
- No persistence authority expansion.
- No replay repair.
- No recovery promotion.
- No action execution.
- No readiness approval.
- No Release Candidate approval.
- No Production Candidate approval.
- No public/general-use approval.
- No production-human-use approval.

## v0.0.130 - 2026-05-10
**Status:** Phase 130 - Release Candidate Decision Gate

### Added
- Add the Phase 130 Release Candidate decision-gate report.

### Changed
- Update checklists/current-phase.md to Phase 130 procedural truth.
- Update CHANGELOG.md with v0.0.130.

### Notes
- Phase 130 is decision gate only.
- Dry-run completeness is not readiness.
- Evidence-map completeness is not approval.
- Specification evidence is not artifact creation.
- Operational evidence is not monitoring.
- Phase 129 did not decide Release Candidate status.
- Clean scans do not imply readiness.
- No evidence category may satisfy another category by inference.
- Phase 130 may still decide not ready.
- Release Candidate decision does not imply Production Candidate status.
- Release Candidate decision does not imply public/general use.
- Phase 130 creates no release artifact creation.
- Phase 130 creates no package creation.
- Phase 130 performs no checksum generation.
- Phase 130 performs no provenance attestation creation.
- Phase 130 creates no installer/update-channel behavior.
- Phase 130 creates no signing/publishing behavior.
- Phase 130 creates no GitHub release/tag/public download asset creation.
- Phase 130 activates no monitoring/logging/telemetry activation.
- Phase 130 creates no deployment automation.
- Phase 130 creates no production deployment behavior.
- Phase 130 changes no runtime behavior.
- Phase 130 adds no new runtime capability.
- Phase 130 makes no Rust source changes unless explicitly justified as validation compatibility.
- Phase 130 makes no TypeScript source changes unless explicitly justified as validation compatibility.
- Phase 130 makes no test assertion changes.
- Phase 130 makes no schema changes.
- Phase 130 adds no provider trust.
- Phase 130 adds no provider output promotion.
- Phase 130 adds no persistence authority expansion.
- Phase 130 adds no replay repair.
- Phase 130 adds no recovery promotion.
- Phase 130 adds no action execution.
- Phase 130 grants no Production Candidate approval.
- Phase 130 grants no public/general-use approval.
- Phase 130 grants no production-human-use approval.
- Phase 130 decision does not imply Production Candidate status or public/general use.
- Overall decision status: `rc_candidate_not_ready`.

## v0.0.129.1 - 2026-05-10
**Status:** Out-of-Band Maintenance - UI TypeScript Command Drift Fix

### Changed
- Correct the UI API behavior test command so active UI command surfaces do not use unsupported TypeScript compiler flags.
- Update bootstrap-generated UI package command surfaces if needed so bootstrap idempotence cannot restore stale TypeScript flags.
- Add the Phase 129.1 out-of-band maintenance report documenting the recurring CI failure, unsupported flag scan, bootstrap check, corrected command surface, and validation.
- Update checklists/current-phase.md to Phase 129.1 procedural truth.

### Notes
- This is out-of-band validation-compatibility maintenance only.
- Validation compatibility is not runtime capability.
- UI command correction is not UI behavior change.
- Bootstrap idempotence must not restore stale TypeScript flags.
- Unsupported TypeScript flags must not exist in active UI command surfaces.
- No UI source changes.
- No UI behavior changes.
- No UI test assertion changes.
- No Rust source changes.
- No runtime behavior changes.
- No authority behavior changes.
- No provider execution change.
- No persistence authority change.
- No replay repair.
- No recovery promotion.
- No action execution.
- No package creation.
- No release artifact creation.
- No monitoring activation.
- No logging activation.
- No telemetry collection.
- No deployment automation.
- No readiness approval.
- No Release Candidate approval.
- No Production Candidate approval.
- No public/general-use approval.
- No production-human-use approval.
- No Phase 130 implementation.

## v0.0.129 - 2026-05-10
**Status:** Phase 129 - Release Candidate Dry Run

### Added
- Add the Phase 129 Release Candidate dry-run report.

### Changed
- Update checklists/current-phase.md to Phase 129 procedural truth.
- Update CHANGELOG.md with v0.0.129.

### Notes
- Release Candidate dry run only.
- dry-run evidence assembly only.
- Phase 129 is Release Candidate dry run only; it does not create release artifacts, publish packages, activate monitoring, deploy, or approve Release Candidate readiness.
- Feedback is evidence, not authority.
- Dry run is not release.
- Evidence map is not readiness.
- Specification evidence is not artifact creation.
- Operational evidence is not release evidence unless a later decision phase explicitly classifies it.
- Release Candidate dry run does not approve Release Candidate readiness.
- Phase 129 must not decide what Phase 130 is scoped to decide.
- Missing Phase-130 dependencies must trigger remap_phase_126_130 or defer_release_candidate_hardening.
- Phase 130 may still decide not ready.
- no runtime behavior.
- no new runtime capability.
- no Rust source changes.
- no TypeScript source changes.
- no test changes.
- no schema changes.
- no package creation.
- no release artifact creation.
- no checksum generation.
- no provenance attestation creation.
- no installer behavior.
- no update-channel behavior.
- no signing/publishing behavior.
- no GitHub release/tag/public download asset creation.
- no public release behavior.
- no production deployment behavior.
- no deployment automation.
- no monitoring activation.
- no logging activation.
- no telemetry collection.
- no collector creation.
- no exporter creation.
- no dashboard creation.
- no alerting creation.
- no production telemetry endpoint creation.
- no telemetry token creation.
- no ingestion URL creation.
- no cron job creation.
- no service file creation.
- no scheduled collector creation.
- no background service.
- no daemon behavior.
- no persistence authority expansion.
- no replay repair.
- no recovery promotion.
- no action execution.
- no provider trust.
- no provider output promotion.
- no readiness approval.
- no Release Candidate approval.
- no release-candidate approval.
- no Production Candidate approval.
- no public-usability approval.
- no public/general-use approval.
- no production-human-use approval.
- no Phase 130 implementation.

## v0.0.128 - 2026-05-10
**Status:** Phase 128 - Observability and Operational Evidence Boundary

### Added
- Add the Phase 128 observability and operational-evidence boundary report.

### Changed
- Update checklists/current-phase.md to Phase 128 procedural truth.
- Update CHANGELOG.md with v0.0.128.
- Update scripts/check.sh for validation compatibility so the canonical aggregate check continues to run the existing UI API behavior test surface without changing UI source, UI behavior, or test assertions.
- Correct the UI API behavior test TypeScript invocation by preserving the CI-compatible compiler flags and not using unsupported `--ignoreConfig` or `--ignoreDeprecations 6.0` options.

### Notes
- Phase 128 is observability and operational-evidence boundary only.
- Phase 128 is observability specification only.
- Phase 128 is operational-evidence specification only.
- Phase 128 is observability and operational-evidence boundary only; it does not activate monitoring, logging, deployment, release, or approve readiness.
- Feedback is evidence, not authority.
- Observability is specification, not monitoring.
- Telemetry is not safety.
- Failure reporting is not reliability.
- Operational evidence is not release evidence.
- Audit-trail requirements are not audit authority.
- No observability row may imply readiness, deployment, or public/general use.
- No observability row may imply active collection, export, dashboarding, alerting, or production monitoring.
- Missing Phase-129/130 dependencies must trigger remap_phase_126_130 or defer_release_candidate_hardening.
- no runtime behavior
- no new runtime capability
- no Rust source changes
- no TypeScript source changes
- no test changes
- no UI behavior change; validation compatibility keeps the existing UI API behavior test surface runnable under the CI TypeScript compiler.
- no unsupported TypeScript compiler flags are required for the UI API behavior test command
- no schema changes
- no monitoring activation
- no logging activation
- no telemetry collection
- no collector creation
- no exporter creation
- no dashboard creation
- no alerting creation
- no production telemetry endpoint creation
- no telemetry token creation
- no ingestion URL creation
- no cron job creation
- no service file creation
- no scheduled collector creation
- no background service
- no daemon behavior
- no package creation
- no release artifact creation
- no checksum generation
- no provenance attestation creation
- no installer behavior
- no update-channel behavior
- no signing/publishing behavior
- no GitHub release/tag/public download asset creation
- no public release behavior
- no production deployment behavior
- no persistence authority expansion
- no replay repair
- no recovery promotion
- no action execution
- no provider trust
- no provider output promotion
- no readiness approval
- no Release Candidate approval
- no release-candidate approval
- no Production Candidate approval
- no public-usability approval
- no public/general-use approval
- no production-human-use approval
- no Phase 129 implementation
- no Phase 130 implementation

## v0.0.127.5 - 2026-05-10
**Status:** Out-of-Band Maintenance - Stale Bootstrap Artifact Sweep

### Removed

* Remove stale, unused, and misleading bootstrap artifacts identified by reference scans.

### Changed

* Update checklists/current-phase.md to record the out-of-band maintenance sweep.
* Add the out-of-band maintenance report documenting candidate classification, deletions, deferrals, and validation.
* Update active references only if needed to remove stale links to deleted artifacts.
* Update scripts/bootstrap_repo.py so bootstrap idempotence no longer recreates deleted stale bootstrap artifacts.

### Notes

* This is out-of-band maintenance only.
* No Phase 127 implementation.
* No Phase 128 implementation.
* No runtime behavior.
* No new capability.
* No UI behavior change.
* No Rust source changes unless a deleted stale Rust artifact is proven unused and explicitly justified; no Rust source was changed in this sweep.
* No TypeScript source changes unless a deleted stale TypeScript artifact is proven unused and explicitly justified; no TypeScript source was changed in this sweep.
* No test assertions changed.
* No TypeScript command-line compatibility change is recorded in this entry.
* UI API behavior test command compatibility is handled by the later corrective validation change, not by this stale-artifact sweep entry.
* No schema changes.
* No provider execution change.
* No persistence authority change.
* No replay repair.
* No recovery promotion.
* No action execution.
* No package creation.
* No release artifact creation.
* No installer behavior.
* No update-channel behavior.
* No signing/publishing behavior.
* No GitHub release/tag/public download asset creation.
* No public release behavior.
* No production deployment behavior.
* No readiness approval.
* No Release Candidate approval.
* No Production Candidate approval.
* No public/general-use approval.
* No production-human-use approval.
* Historical truth remains in CHANGELOG, not stale executable surfaces.

## v0.0.127.1 - 2026-05-10
**Status:** Out-of-Band Maintenance - CI and Validator Alignment

### Added
- Add hardened `ci.yml` coverage for repository validation, Rust validation, UI validation, schema validation, shell-script parsing, and full deterministic `scripts/check.sh` validation.
- Add CI repository-validation coverage for `scripts/validate_structure.py`, `scripts/validate_docs.py`, Rust boundary lint self-tests, and Rust boundary lint.
- Add explicit CI UI validation coverage for typecheck, lint, build, UI API behavior tests, UI boundary AST lint self-tests, and UI boundary AST lint.

### Changed
- Update `ci.yml` to remove unsupported job-level `hashFiles(...)` expressions and replace them with post-checkout shell-based surface detection.
- Update UI CI execution so UI package commands run from `ui/`, while repository boundary lint scripts run from the repository root.
- Update `ui/package.json` script commands to remove unsupported TypeScript compiler flags and keep behavior-test output under `/tmp`.
- Update `scripts/check.sh` to remain the canonical aggregate local validation surface, including clean-worktree checks, documentation validation, structure validation, schema validation, shell parsing, Rust/UI boundary linting, UI validation, Rust formatting, Rust checks, Rust tests, and Clippy.
- Update `scripts/validate_docs.py` to replace exact-path procedural operations exceptions with filename-convention handling for procedural operations templates.
- Update `scripts/validate_structure.py` to replace exact-path procedural truth exceptions with path/filename truth-dimension classification for operations templates.

### Removed
- Remove redundant `docs-gate.yml` after documentation boundary validation moved under `ci.yml` repository-validation and full-check coverage.
- Remove redundant `structure-lint.yml` after repository structure validation moved under `ci.yml` repository-validation and full-check coverage.
- Remove misleading advisory `pr-agent-review.yml` placeholder because it did not perform review, produce typed evidence, approve merges, or approve readiness.

### Notes
- This is out-of-band maintenance only.
- CI validation is now consolidated around canonical repository validators and `scripts/check.sh`.
- Duplicate workflow surfaces were removed to reduce governance drift and avoid misleading green checks.
- Workflow checks are diagnostic validation only and do not create authority.
- `ci.yml` does not approve readiness, Release Candidate status, Production Candidate status, public/general use, production readiness, production deployment, or production human use.
- No runtime behavior was added.
- No new runtime capability was added.
- No Rust authority behavior was changed.
- No provider execution behavior was changed.
- No persistence authority was changed.
- No replay repair was added.
- No recovery promotion was added.
- No action execution was added.
- No package creation was added.
- No release artifact creation was added.
- No checksum generation was added.
- No provenance attestation creation was added.
- No installer behavior was added.
- No update-channel behavior was added.
- No signing or publishing behavior was added.
- No GitHub release, release tag, public download, or public asset behavior was added.
- No public release behavior was added.
- No production deployment behavior was added.
- Phase 127 remains complete as installer/update-channel threat-boundary work only.
- Phase 128 was not implemented.

## v0.0.127 - 2026-05-10
**Status:** Phase 127 - Installer and Update-Channel Threat Boundary

### Added
- Add the Phase 127 installer and update-channel threat-boundary report.

### Changed
- Update checklists/current-phase.md to Phase 127 procedural truth.
- Update CHANGELOG.md with v0.0.127.

### Notes
- Phase 127 is installer and update-channel threat boundary only.
- Phase 127 is threat-model and contract-only.
- Phase 127 is threat-model and contract-only; it does not create installers, update channels, signing, publishing, or approve readiness.
- Feedback is evidence, not authority.
- Remediation is documentation clarity, not readiness.
- Contract/spec is specification only, not artifact creation.
- No installer/update-channel spec row may imply activation, signing, publishing, or release readiness.
- Missing Phase-128/129/130 dependencies must trigger remap_phase_126_130 or defer_release_candidate_hardening.
- Phase 127 adds no runtime behavior.
- Phase 127 adds no new runtime capability.
- Phase 127 makes no Rust source changes.
- Phase 127 makes no TypeScript source changes.
- Phase 127 makes no test changes.
- Phase 127 makes no schema changes.
- Phase 127 creates no packages.
- Phase 127 creates no release artifacts.
- Phase 127 generates no checksums.
- Phase 127 creates no provenance attestations.
- Phase 127 creates no installers.
- Phase 127 creates no updater services.
- Phase 127 creates no update channels.
- Phase 127 creates no update-channel metadata.
- Phase 127 creates no signing keys.
- Phase 127 creates no key custody behavior.
- Phase 127 creates no signatures.
- Phase 127 adds no deployment automation.
- Phase 127 adds no rollback automation.
- Phase 127 adds no background service.
- Phase 127 adds no daemon behavior.
- Phase 127 creates no installer behavior.
- Phase 127 creates no update-channel behavior.
- Phase 127 adds no signing/publishing behavior.
- Phase 127 creates no GitHub release/tag/public download asset.
- Phase 127 adds no public release behavior.
- Phase 127 adds no production deployment behavior.
- Phase 127 adds no persistence authority expansion.
- Phase 127 adds no replay repair.
- Phase 127 adds no recovery promotion.
- Phase 127 adds no action execution.
- Phase 127 adds no provider trust.
- Phase 127 adds no provider output promotion.
- Phase 127 grants no readiness approval.
- Phase 127 grants no Release Candidate approval.
- Phase 127 grants no release-candidate approval.
- Phase 127 grants no Production Candidate approval.
- Phase 127 grants no public-usability approval.
- Phase 127 grants no public/general-use approval.
- Phase 127 grants no production-human-use approval.
- Phase 127 implements no Phase 128.
- Phase 127 implements no Phase 129.
- Phase 127 implements no Phase 130.
- Phase 128 remains Observability and Operational Evidence Boundary only.
- Phase 129 remains Release Candidate Dry Run only.
- Phase 130 remains Release Candidate Decision Gate only.
- Phase 130 may still decide not ready.
- Public/general use remains a later final rung.
- Phase 125 preserve_with_caveats and expand_post_130_plan remain active caveated planned truth.

## v0.0.126 - 2026-05-10
**Status:** Phase 126 - Release Packaging Contract

### Added
- Add the Phase 126 release packaging contract report.

### Changed
- Update checklists/current-phase.md to Phase 126 procedural truth.
- Update CHANGELOG.md with v0.0.126.

### Notes
- Phase 126 is release packaging contract only.
- Packaging contract is not package creation.
- Artifact contract is not artifact creation.
- Checksum contract is not checksum generation.
- Provenance contract is not provenance attestation.
- Distribution contract is not distribution.
- Signing contract is not signing.
- Publishing contract is not publishing.
- Release packaging contract is not Release Candidate readiness.
- Release packaging contract is not Production Candidate readiness.
- Release packaging contract is not public/general use.
- Phase 126 adds no runtime behavior.
- Phase 126 adds no new runtime capability.
- Phase 126 makes no Rust source changes.
- Phase 126 makes no TypeScript source changes.
- Phase 126 makes no test changes.
- Phase 126 makes no schema changes.
- Phase 126 creates no packages.
- Phase 126 creates no release artifacts.
- Phase 126 generates no checksums.
- Phase 126 creates no provenance attestations.
- Phase 126 adds no deployment automation.
- Phase 126 creates no installer behavior.
- Phase 126 creates no update-channel behavior.
- Phase 126 adds no signing/publishing behavior.
- Phase 126 creates no GitHub release, release tag, or public download asset.
- Phase 126 adds no public release behavior.
- Phase 126 adds no production deployment behavior.
- Phase 126 adds no persistence authority expansion.
- Phase 126 adds no replay repair.
- Phase 126 adds no recovery promotion.
- Phase 126 adds no action execution.
- Phase 126 adds no provider trust.
- Phase 126 adds no provider output promotion.
- Phase 126 grants no readiness approval.
- Phase 126 grants no Release Candidate approval.
- Phase 126 grants no release-candidate approval.
- Phase 126 grants no Production Candidate approval.
- Phase 126 grants no public-usability approval.
- Phase 126 grants no public/general-use approval.
- Phase 126 grants no production-human-use approval.
- Phase 126 implements no Phase 127.
- Phase 126 implements no Phase 130.
- No Phase 126 contract may imply release artifact creation.
- No Phase 126 contract may imply installer or update-channel activation.
- No Phase 126 contract may imply signing, publishing, GitHub release, release tag, public download, or public asset creation.
- No Phase 126 contract may imply public release, production deployment, readiness, or production human use.

## v0.0.125 - 2026-05-10
**Status:** Phase 125 - Roadmap, Changelog, and Production-Path Reassessment

### Added
- Add the Phase 125 roadmap, changelog, and production-path reassessment report.

### Changed
- Update roadmap planned-truth surfaces to preserve Phase 126-130 with caveats and clarify post-130 planning expansion.
- Update checklists/current-phase.md to Phase 125 procedural truth.
- Update CHANGELOG.md with v0.0.125.

### Notes
- Phase 125 is roadmap, changelog, and production-path reassessment only.
- Phase 125 is a 0/5 checkpoint.
- Phase 125 is not a green light phase.
- Reconciliation is not readiness approval.
- Feedback is evidence, not authority.
- Remediation is documentation clarity, not readiness.
- Usability clarity is not safety.
- Operator workflow clarity is not release readiness.
- Evidence capture clarity is not public usability.
- No remediation item may imply runtime behavior.
- No remediation item may imply release or deployment behavior.
- No remediation item may imply authority activation.
- No remediation item may imply Release Candidate or Production Candidate readiness.
- No remediation item may imply public/general use.
- Phase 125 adds no runtime behavior.
- Phase 125 adds no new capability.
- Phase 125 makes no Rust source changes.
- Phase 125 makes no TypeScript source changes.
- Phase 125 makes no test changes.
- Phase 125 makes no schema changes.
- Phase 125 implements no Phase 126.
- Phase 125 implements no Phase 130.
- Phase 125 adds no deployment automation.
- Phase 125 creates no release artifacts.
- Phase 125 creates no packages.
- Phase 125 creates no installer behavior.
- Phase 125 creates no update-channel behavior.
- Phase 125 adds no signing/publishing behavior.
- Phase 125 creates no GitHub release, release tag, or public download asset.
- Phase 125 adds no public release behavior.
- Phase 125 adds no production deployment behavior.
- Phase 125 adds no persistence authority expansion.
- Phase 125 adds no replay repair.
- Phase 125 adds no recovery promotion.
- Phase 125 adds no action execution.
- Phase 125 adds no provider trust.
- Phase 125 adds no provider output promotion.
- Phase 125 grants no readiness approval.
- Phase 125 grants no Release Candidate approval.
- Phase 125 grants no release-candidate approval.
- Phase 125 grants no Production Candidate approval.
- Phase 125 grants no public-usability approval.
- Phase 125 grants no public/general-use approval.
- Phase 125 grants no production-human-use approval.
- Production-path estimates are non-authoritative planned-truth scenarios, not commitments.

## v0.0.124 - 2026-05-10
**Status:** Phase 124 - Operational Usability Remediation Boundary

### Added
- Add the Phase 124 operational usability remediation report.
- Add the Phase 124 early human-use evidence-capture template.

### Changed
- Update checklists/current-phase.md to Phase 124 procedural truth.
- Update CHANGELOG.md with v0.0.124.

### Notes
- Phase 124 is operational usability remediation only.
- Usability remediation is not readiness approval.
- Feedback is evidence, not authority.
- Trial evidence is not readiness.
- The evidence-capture template is procedural guidance only.
- Phase 124 adds no runtime behavior.
- Phase 124 adds no new runtime capability.
- Phase 124 makes no Rust source changes.
- Phase 124 makes no TypeScript source changes.
- Phase 124 makes no test changes.
- Phase 124 makes no schema changes.
- Phase 124 implements no Phase 125.
- Phase 124 implements no Phase 130.
- Phase 124 adds no deployment automation.
- Phase 124 creates no release artifacts.
- Phase 124 creates no packages.
- Phase 124 creates no installer behavior.
- Phase 124 creates no update-channel behavior.
- Phase 124 adds no signing/publishing behavior.
- Phase 124 creates no GitHub release, release tag, or public download asset.
- Phase 124 adds no public release behavior.
- Phase 124 adds no production deployment behavior.
- Phase 124 adds no persistence authority expansion.
- Phase 124 adds no replay repair.
- Phase 124 adds no recovery promotion.
- Phase 124 adds no action execution.
- Phase 124 adds no provider trust.
- Phase 124 adds no provider output promotion.
- Phase 124 grants no readiness approval.
- Phase 124 grants no Release Candidate approval.
- Phase 124 grants no release-candidate approval.
- Phase 124 grants no Production Candidate approval.
- Phase 124 grants no public-usability approval.
- Phase 124 grants no public/general-use approval.
- Phase 124 grants no production-human-use approval.

## v0.0.123 - 2026-05-09
**Status:** Phase 123 - Early Human-Use Evidence Review and Operator Feedback Audit

### Added
- Add the Phase 123 early human-use evidence review and operator feedback audit report.

### Changed
- Update checklists/current-phase.md to Phase 123 procedural truth.
- Update CHANGELOG.md with v0.0.123.

### Notes
- Phase 123 is evidence review and operator feedback audit only.
- Feedback is evidence, not authority.
- Trial evidence is not readiness.
- Trial evidence is not Release Candidate approval.
- Trial evidence is not Production Candidate approval.
- Trial evidence is not public/general-use approval.
- Trial evidence is not production-human-use approval.
- Phase 123 implements no Phase 124 remediation.
- Phase 123 implements no Phase 125 alignment.
- Phase 123 implements no Phase 130 Release Candidate Decision Gate.
- Phase 123 adds no runtime behavior.
- Phase 123 adds no new capability.
- Phase 123 makes no Rust source changes.
- Phase 123 makes no TypeScript source changes.
- Phase 123 makes no test changes.
- Phase 123 makes no schema changes.
- Phase 123 adds no deployment automation.
- Phase 123 creates no release artifacts.
- Phase 123 creates no packages.
- Phase 123 creates no installer behavior.
- Phase 123 creates no update-channel behavior.
- Phase 123 adds no signing/publishing behavior.
- Phase 123 creates no GitHub release, release tag, or public download asset.
- Phase 123 adds no public release behavior.
- Phase 123 adds no production deployment behavior.
- Phase 123 adds no persistence authority expansion.
- Phase 123 adds no replay repair.
- Phase 123 adds no recovery promotion.
- Phase 123 adds no action execution.
- Phase 123 adds no provider trust.
- Phase 123 adds no provider output promotion.
- Phase 123 grants no readiness approval.
- Phase 123 grants no Release Candidate approval.
- Phase 123 grants no release-candidate approval.
- Phase 123 grants no Production Candidate approval.
- Phase 123 grants no public-usability approval.
- Phase 123 grants no public/general-use approval.
- Phase 123 grants no production-human-use approval.
- Phase 124, if recommended, is operational usability remediation only.
- Phase 125 remains the next 0/5 checkpoint.
- Phase 130 remains Release Candidate Decision Gate only.
- Public/general use remains a later final rung.

## v0.0.122 - 2026-05-09
**Status:** Phase 122 - Controlled Early Human-Use Trial Boundary

### Added
- Add the Phase 122 controlled early-human-use trial boundary report.

### Changed
- Update checklists/current-phase.md to Phase 122 procedural truth.
- Update CHANGELOG.md with v0.0.122.

### Notes
- Phase 122 is controlled early-human-use trial boundary only.
- Trial evidence is not Release Candidate approval.
- Trial evidence is not Production Candidate approval.
- Trial evidence is not public/general-use approval.
- Trial evidence is not production-human-use approval.
- Phase 122 adds no runtime behavior unless limited to existing validation/reporting surfaces already permitted by the phase.
- Phase 122 adds no new capability beyond controlled trial boundary documentation/evidence.
- Phase 122 makes no Rust source changes unless an existing trial evidence/reporting surface already requires typed support and the change is explicitly justified.
- Phase 122 makes no TypeScript source changes unless descriptive projection support is explicitly justified.
- Phase 122 makes no schema changes unless an existing schema already governs this exact trial evidence surface.
- Phase 122 adds no deployment automation.
- Phase 122 creates no release artifacts.
- Phase 122 creates no packages.
- Phase 122 creates no installer behavior.
- Phase 122 creates no update-channel behavior.
- Phase 122 adds no signing/publishing behavior.
- Phase 122 creates no GitHub release, release tag, or public download asset.
- Phase 122 adds no public release behavior.
- Phase 122 adds no production deployment behavior.
- Phase 122 adds no persistence authority expansion.
- Phase 122 adds no replay repair.
- Phase 122 adds no recovery promotion.
- Phase 122 adds no action execution.
- Phase 122 adds no provider trust.
- Phase 122 adds no provider output promotion.
- Phase 122 grants no readiness approval.
- Phase 122 grants no Release Candidate approval.
- Phase 122 grants no release-candidate approval.
- Phase 122 grants no Production Candidate approval.
- Phase 122 grants no public-usability approval.
- Phase 122 grants no public/general-use approval.
- Phase 122 grants no production-human-use approval.
- Phase 122 does not implement Phase 123.
- Phase 123, if recommended later, is early human-use evidence review and operator feedback audit only.
- Phase 125 remains the next 0/5 checkpoint.
- Phase 130 remains Release Candidate Decision Gate only.
- Public/general use remains a later final rung.

## v0.0.121 - 2026-05-09
**Status:** Phase 121 - Post-120 Roadmap Expansion and Production Gap Reassessment

### Added
- Add the Phase 121 post-120 roadmap expansion and production gap reassessment report.

### Changed
- Update roadmap planned-truth surfaces to extend the roadmap beyond Phase 120.
- Update checklists/current-phase.md to Phase 121 procedural truth.
- Update CHANGELOG.md with v0.0.121.

### Notes
- Phase 121 is roadmap expansion and production gap reassessment only.
- Roadmap expansion is not implementation.
- Roadmap expansion is not readiness approval.
- Phase 121 adds no runtime behavior and no new capability.
- Phase 121 makes no Rust source changes, no TypeScript source changes, no test changes, and no schema changes.
- Phase 120 remains Early Human-Use Candidate Gate only and is not a guaranteed final endpoint.
- Constrained early-human-use candidacy is not Release Candidate approval, not Production Candidate approval, not public/general use, and not production human use.
- Phase 121 adds no deployment automation, no release artifact creation, no package creation, no installer behavior, no update-channel behavior, no signing/publishing behavior, no GitHub release/tag/public download asset creation, no public release behavior, and no production deployment behavior.
- Phase 121 adds no persistence authority expansion, no replay repair, no recovery promotion, no action execution, no provider trust, and no provider output promotion.
- Phase 121 grants no readiness approval, no Release Candidate approval, no release-candidate approval, no Production Candidate approval, no public-usability approval, no public/general-use approval, and no production-human-use approval.
- Phase 121 does not implement Phase 122.
- Phase 119 remains an intentional decision-gate exception and does not redefine the 0/5 checkpoint cadence.
- Public/general use remains a later final rung.

## v0.0.120 - 2026-05-09
**Status:** Phase 120 - Early Human-Use Candidate Gate

### Added
- Add the Phase 120 early human-use candidate gate report.

### Changed
- Update checklists/current-phase.md to Phase 120 procedural truth.
- Update CHANGELOG.md with v0.0.120.

### Notes
- Phase 120 is Early Human-Use Candidate Gate only.
- Phase 120 is not a guaranteed final endpoint.
- Phase 120 is not Release Candidate approval.
- Phase 120 is not Production Candidate approval.
- Phase 120 is not public/general use.
- Phase 120 is not production readiness.
- Phase 120 is not production human use.
- Early human-use candidacy, if permitted, is bounded and constrained.
- No runtime behavior.
- No new capability.
- No Rust source changes.
- No TypeScript source changes.
- No test changes.
- No schema changes.
- No deployment automation.
- No release artifact creation.
- No package creation.
- No installer behavior.
- No update-channel behavior.
- No signing/publishing behavior.
- No GitHub release/tag/public download asset creation.
- No public release behavior.
- No production deployment behavior.
- No persistence authority expansion.
- No replay repair.
- No recovery promotion.
- No action execution.
- No provider trust.
- No provider output promotion.
- No readiness approval.
- No Release Candidate approval.
- No release-candidate approval.
- No Production Candidate approval.
- No public-usability approval.
- No public/general-use approval.
- No production-human-use approval.
- No Phase 121 implementation.
- Phase 121 roadmap expansion is required or deferred explicitly.
- Public/general use remains the final rung.

## v0.0.119 - 2026-05-09
**Status:** Phase 119 - Production Candidate Reassessment

### Added
- Add the Phase 119 Production Candidate reassessment report.

### Changed
- Update checklists/current-phase.md to Phase 119 procedural truth.
- Update CHANGELOG.md with v0.0.119.

### Notes
- Phase 119 is a decision gate only.
- Phase 119 is an intentional decision-gate exception to the usual 0/5 checkpoint cadence.
- The exception exists because Phase 118 evidence assembly requires reassessment before Phase 120 can consider early human-use candidacy.
- This exception does not redefine the 0/5 checkpoint convention for future roadmap planning.
- Production Candidate reassessment is not automatic Production Candidate approval.
- Phase 118 evidence assembly is not readiness.
- Absence of blockers is not approval.
- No runtime behavior.
- No new capability.
- No Rust source changes.
- No TypeScript source changes.
- No test changes.
- No schema changes.
- No deployment automation.
- No release artifact creation.
- No package creation.
- No installer behavior.
- No update-channel behavior.
- No signing/publishing behavior.
- No GitHub release/tag/public download asset creation.
- No public release behavior.
- No production deployment behavior.
- No persistence authority expansion.
- No replay repair.
- No recovery promotion.
- No action execution.
- No provider trust.
- No provider output promotion.
- No readiness approval.
- No Release Candidate approval.
- No release-candidate approval.
- No Production Candidate approval unless explicitly and narrowly supported by this phase’s decision criteria.
- No public-usability approval.
- No public/general-use approval.
- No controlled-human-use approval.
- No early-human-use approval.
- No production-human-use approval.
- No Phase 120 implementation.
- Phase 120 remains a current planned early-human-use gate, not a guaranteed final endpoint.

## v0.0.118 - 2026-05-09
**Status:** Phase 118 - Release Candidate Evidence Assembly

### Added
- Add the Phase 118 release-candidate evidence assembly report.

### Changed
- Update checklists/current-phase.md to Phase 118 procedural truth.
- Update CHANGELOG.md with v0.0.118.

### Notes
- Release-candidate evidence assembly only.
- Evidence assembly is not release-candidate approval.
- Evidence assembly is not Release Candidate approval.
- Evidence assembly is not Production Candidate approval.
- Evidence assembly is not controlled human-use approval.
- Evidence assembly is not public/general-use approval.
- Evidence assembly is not production-human-use approval.
- No runtime behavior.
- No new capability.
- No Rust source changes.
- No TypeScript source changes.
- No test changes.
- No schema changes.
- No deployment automation.
- No release artifact creation.
- No package creation.
- No installer behavior.
- No update-channel behavior.
- No signing/publishing behavior.
- No GitHub release/tag/public download asset creation.
- No public release behavior.
- No production deployment behavior.
- No persistence authority expansion.
- No replay repair.
- No recovery promotion.
- No action execution.
- No provider trust.
- No provider output promotion.
- No readiness approval.
- No Production Candidate approval.
- No release-candidate approval.
- No public-usability approval.
- No public/general-use approval.
- No production-human-use approval.
- No Phase 119 implementation.
- Phase 120 remains a current planned gate, not a guaranteed final endpoint.

## v0.0.117 - 2026-05-09
**Status:** Phase 117 - Operator Documentation and Human-Trial Dry Run

### Added
- Add the Phase 117 operator human-trial dry-run procedure and evidence handoff documentation.

### Changed
- Update checklists/current-phase.md to Phase 117 procedural truth.
- Update CHANGELOG.md with v0.0.117.

### Notes
- Operator documentation and human-trial dry run only.
- Dry-run evidence is not human-use approval.
- Dry-run evidence is not release approval.
- Dry-run evidence is not Production Candidate approval.
- No runtime behavior.
- No new capability.
- No Rust source changes.
- No TypeScript source changes.
- No test changes.
- No schema changes.
- No deployment automation.
- No release artifact creation.
- No installer behavior.
- No update-channel behavior.
- No signing/publishing behavior.
- No GitHub release/tag/public download asset creation.
- No public release behavior.
- No production deployment behavior.
- No persistence authority expansion.
- No replay repair.
- No recovery promotion.
- No action execution.
- No provider trust.
- No provider output promotion.
- No readiness approval.
- No Production Candidate approval.
- No release-candidate approval.
- No public-usability approval.
- No public/general-use approval.
- No production-human-use approval.
- No Phase 118 implementation.

## v0.0.116 - 2026-05-09
**Status:** Phase 116 - Local Deployment Candidate Boundary

### Added
- Add deterministic local deployment candidate boundary and validation structures.
- Add tests for local deployment candidate evidence requirements and non-authority guarantees.
- Add the Phase 116 operations report.

### Changed
- Update checklists/current-phase.md to Phase 116 procedural truth.
- Update CHANGELOG.md with v0.0.116.

### Notes
- Phase 116 is local deployment candidate boundary only.
- Local deployment candidate status is not deployment authority.
- Local deployment candidate status is not release authority.
- Local deployment candidate status is not human-use approval.
- No deployment automation.
- No release artifact creation.
- No installer behavior.
- No update-channel behavior.
- No signing/publishing behavior.
- No GitHub release/tag/public download asset creation.
- No public release behavior.
- No production deployment behavior.
- No persistence authority expansion.
- No replay repair.
- No recovery promotion.
- No action execution.
- No provider trust.
- No provider output promotion.
- No readiness approval.
- No Production Candidate approval.
- No release-candidate approval.
- No public-usability approval.
- No public/general-use approval.
- No production-human-use approval.
- No Phase 117 implementation.

## v0.0.115 - 2026-05-09
**Status:** Phase 115 - Security Threat Model and Abuse-Case Audit

### Added
- Add the Phase 115 security threat model and abuse-case audit report.
- Add security audit procedural truth to the current checklist.

### Changed
- Update CHANGELOG.md with v0.0.115.
- Narrow Rust boundary lint matching to avoid validation false positives for ordinary `copy` identifiers and safe `std::process::id()` test temp-directory usage, and restore UI boundary lint detection for `Promise.resolve(...).then(...)` under the local TypeScript AST shape.

### Notes
- Security audit only.
- Security audit evidence is risk evidence, not approval authority.
- No runtime behavior.
- No new application capability.
- No Rust source changes.
- No TypeScript source changes.
- No test changes.
- No schema changes.
- No governance doc changes.
- No deployment automation.
- No release artifact creation.
- No installer behavior.
- No update-channel behavior.
- No signing/publishing behavior.
- No public release behavior.
- No production deployment behavior.
- No persistence authority expansion.
- No replay repair.
- No recovery promotion.
- No action execution.
- No provider trust.
- No provider output promotion.
- No readiness approval.
- No Production Candidate approval.
- No release-candidate approval.
- No public-usability approval.
- No production-human-use approval.
- No Phase 116 implementation.
- Narrow validation-script compatibility only; no runtime behavior affected.

## v0.0.114.1 - 2026-05-08
**Status:** Post-114 Maintenance Script Fix - validation-script hardening only

### Changed
- Hardened Rust boundary lint coverage for filesystem mutation, process execution, raw-string handling, and char-literal handling.
- Expanded Rust boundary lint self-tests to cover updated filesystem, process, raw-string, and harness-integrity behavior.
- Expanded UI boundary lint coverage for post-111/post-114 authority-shaped UI calls and approval-shaped fields.
- Expanded UI boundary lint self-tests to cover authority-shaped UI calls, deployment/release-shaped calls, and harness-integrity behavior.
- Hardened repository structure validation to ignore generated/build/cache directories during recursive scans while preserving truth-dimension, archive naming, and frontmatter enforcement.

### Notes
- Validation-script hardening only.
- No runtime behavior.
- No new application capability.
- No Rust source behavior changes.
- No TypeScript application behavior changes.
- No test-surface authority expansion.
- No governance doc changes.
- No roadmap changes.
- No archived changelog changes.
- No deployment automation.
- No release artifact creation.
- No installer, update-channel, signing, or publishing behavior.
- No persistence authority expansion.
- No replay repair.
- No recovery promotion.
- No action execution.
- No provider trust.
- No provider output promotion.
- No readiness approval.
- No Production Candidate approval.
- No release-candidate approval.
- No public-usability approval.
- No production-human-use approval.
- No Phase 115 implementation.

## v0.0.114 - 2026-05-09
**Status:** Phase 114 - Policy Versioning and Governance Evidence Boundary

### Added
- Add deterministic policy/governance versioning and evidence attribution structures.
- Add tests for governance evidence attribution and non-authority guarantees.
- Add the Phase 114 operations report.

### Changed
- Update checklists/current-phase.md to Phase 114 procedural truth.
- Update CHANGELOG.md with v0.0.114.

### Notes
- Policy/governance versioning only.
- Governance version evidence is attribution evidence, not governance authority.
- Policy version evidence is not approval authority.
- No governance rule rewrite.
- No deployment automation.
- No release artifact creation.
- No installer behavior.
- No update-channel behavior.
- No signing/publishing behavior.
- No public release behavior.
- No production deployment behavior.
- No persistence authority expansion.
- No replay repair.
- No recovery promotion.
- No action execution.
- No provider trust.
- No provider output promotion.
- No readiness approval.
- No Production Candidate approval.
- No release-candidate approval.
- No public-usability approval.
- No production-human-use approval.
- No Phase 115 implementation.

## v0.0.113 - 2026-05-08
**Status:** Phase 113 - Deployment Configuration Contract

### Added
- Add deterministic deployment configuration contract and validation structures.
- Add tests for deployment configuration rejection and non-authority guarantees.
- Add the Phase 113 operations report.

### Changed
- Update checklists/current-phase.md to Phase 113 procedural truth.
- Update CHANGELOG.md with v0.0.113.

### Notes
- Deployment configuration contract only.
- Deployment configuration is not deployment authority.
- No deployment automation.
- No release artifact creation.
- No installer behavior.
- No update-channel behavior.
- No signing/publishing behavior.
- No public release behavior.
- No production deployment behavior.
- No persistence authority expansion.
- No replay repair.
- No recovery promotion.
- No action execution.
- No provider trust.
- No provider output promotion.
- No readiness approval.
- No Production Candidate approval.
- No release-candidate approval.
- No public-usability approval.
- No production-human-use approval.
- No Phase 114 implementation.

## v0.0.112.5 - 2026-05-08
**Status:** Phase 112.5 - Out-of-Band Roadmap Alignment and Recovery Handoff Correction

### Added
- Add the Phase 112.5 roadmap/recovery handoff alignment report.

### Changed
- Update roadmap planned-truth surfaces to record Phase 112 as Recovery Lifecycle Hardening and Phase 113 as Deployment Configuration Contract.
- Reposition Policy Versioning and Governance Evidence Boundary as planned future work rather than completed Phase 112 work.
- Update checklists/current-phase.md to Phase 112.5 procedural truth.
- Update CHANGELOG.md with v0.0.112.5.

### Notes
- Alignment/correction only.
- No runtime behavior.
- No new capability.
- No Rust source changes.
- No TypeScript source changes.
- No test changes.
- No schema changes.
- No governance doc changes.
- No archived changelog changes.
- No persistence authority expansion.
- No replay repair.
- No recovery promotion.
- No action execution.
- No provider trust.
- No provider output promotion.
- No readiness approval.
- No Production Candidate approval.
- No release-candidate approval.
- No public-usability approval.
- No production-human-use approval.
- No Phase 113 implementation.
- Phase 113 remains deployment configuration only.
- Phase 120 remains a planned gate, not a guaranteed final endpoint.

## v0.0.112 - 2026-05-08
**Status:** Phase 112 - Recovery Lifecycle Hardening

### Added
- Add typed recovery lifecycle classification for Phase 111 decision-evidence records.
- Add tests for fail-closed recovery classification and prohibited recovery authority paths.
- Add the Phase 112 operations report.

### Changed
- Update checklists/current-phase.md to Phase 112 procedural truth.
- Update CHANGELOG.md with v0.0.112.

### Notes
- Recovery lifecycle hardening only.
- Recovery reads are not recovery authority.
- No silent recovery.
- No replay repair.
- No recovery promotion.
- No action execution.
- No provider trust.
- No provider output promotion.
- No broad persistence authority.
- No Phase 111 append-boundary expansion.
- No readiness approval.
- No Production Candidate approval.
- No release-candidate approval.
- No public-usability approval.
- No production-human-use approval.
- No Phase 113 implementation.

## v0.0.111 - 2026-05-08
**Status:** Phase 111 - Narrow Persistence Authority Activation Boundary

### Added
- Add the narrow Rust-validated decision-evidence append path.
- Add tests for accepted decision-evidence append and prohibited authority categories.
- Add the Phase 111 operations report.

### Changed
- Update checklists/current-phase.md to Phase 111 procedural truth.
- Update CHANGELOG.md with v0.0.111.

### Notes
- Narrow persistence authority only.
- Rust-validated decision-evidence append only.
- No broad persistence authority.
- No provider-output authority.
- No workflow-completion authority.
- No sandbox-success authority.
- No UI/transport persistence authority.
- No replay repair.
- No recovery promotion.
- No action execution.
- No provider trust.
- No provider output promotion.
- No readiness approval.
- No Production Candidate approval.
- No release-candidate approval.
- No public-usability approval.
- No production-human-use approval.
- No Phase 112 implementation.

## v0.0.110 - 2026-05-08
**Status:** Phase 110 - Roadmap and Changelog Alignment Check

### Added
- Added the Phase 110 roadmap/changelog alignment report.

### Changed
- Updated roadmap planned-truth surfaces to reconcile Phase 106-109 outcomes and correct stale current-block/immediate-gate language.
- Added an archive annotation outside historical entries in docs/changelog/CHANGELOG-0056-0104.md for the known v0.0.63 ordering anomaly.
- Updated checklists/current-phase.md to Phase 110 procedural truth.
- Updated CHANGELOG.md with v0.0.110.

### Notes
- Alignment/check only.
- No runtime behavior.
- No new capability.
- No Rust source changes.
- No TypeScript source changes.
- No test changes.
- No schema changes.
- No governance doc changes.
- No persistence authority.
- No provider trust.
- No provider output promotion.
- No replay repair.
- No recovery promotion.
- No action execution.
- No readiness approval.
- No Production Candidate approval.
- No release-candidate approval.
- No public-usability approval.
- No production-human-use approval.
- No Phase 111 implementation.
- Historical entries were not rewritten.
- Phase 120 remains a planned gate, not a guaranteed final endpoint.

## v0.0.109.5 - 2026-05-08
**Status:** Out-of-Band Repository Governance Audit - post-Phase 109 audit/alignment only

### Added
- Add the audit report.
- Update current checklist to the audit procedural truth.

### Changed
- Update CHANGELOG.md with v0.0.109.5.

### Notes
- Audit/alignment only.
- No runtime behavior.
- No new capability.
- No Rust source changes.
- No TypeScript source changes.
- No test changes.
- No schema changes.
- No governance doc changes unless explicitly justified.
- No persistence authority.
- No provider trust.
- No provider output promotion.
- No replay repair.
- No recovery promotion.
- No action execution.
- No readiness approval.
- No Production Candidate approval.
- No release-candidate approval.
- No public-usability approval.
- No production-human-use approval.
- No Phase 110 implementation.

## v0.0.109 - 2026-05-08
**Status:** Phase 109 - Durable Persistence Authority Decision Gate

### Added
- Added deterministic durable persistence authority decision evidence structures, proposed persistence-boundary classification, negative-authority evidence, decision statuses, constraint sets, prohibited persistence categories, and reason codes.
- Added behavioral and adversarial coverage for descriptive-only decision evidence, sandbox-success non-authority, workflow-completion non-authority, provider-output non-authority, prohibited persistence categories, malformed evidence, hostile/noise authority payloads, and no Phase 109 persistence activation.
- Added Phase 109 operations documentation.

### Changed
- Updated the active phase checklist to Phase 109 procedural truth.

### Notes
- Persistence-boundary decision evidence only.
- No durable persistence authority.
- No provider-output authority.
- No replay repair.
- No recovery promotion.
- No action execution.
- No readiness approval.
- No Production Candidate approval.
- No release-candidate approval.
- No public-usability approval.
- No production-human-use approval.
- No Phase 110 implementation.

## v0.0.108 - 2026-05-07
**Status:** Phase 108 - Provider Timeout and Resource Limit Boundary

### Added
- Added deterministic sandbox timeout/resource enforcement with typed declared limit snapshots, observed usage summaries, enforcement statuses, decisions, deterministic termination reasons, and descriptive-only sandbox limit evidence.
- Added behavioral and adversarial coverage for timeout exhaustion, resource-limit enforcement, deterministic truncation, repeated execution determinism, retry prohibition, limit-escalation prohibition, no-authority posture, and provider-output-untrusted posture.
- Added Phase 108 operations documentation.

### Changed
- Updated provider sandbox reports and UI API projections to expose descriptive timeout/resource evidence without granting authority.
- Updated the active phase checklist to Phase 108 procedural truth.

### Notes
- Deterministic timeout/resource enforcement only.
- Resource-limit evidence remains descriptive-only.
- Provider output remains untrusted candidate data.
- No remote/cloud provider execution.
- No provider output promotion.
- No persistence authority.
- No replay repair.
- No recovery promotion.
- No action execution.
- No readiness approval.
- No Production Candidate approval.
- No release-candidate approval.
- No public-usability approval.
- No production-human-use approval.
- No Phase 109 implementation.

## v0.0.107 - 2026-05-07
**Status:** Phase 107 - Provider Execution Sandbox Boundary

### Added
- Added bounded deterministic local stub provider execution through a typed Rust execution request/report sandbox.
- Added deterministic execution metadata, bounded input/output summaries, sandbox posture indicators, untrusted-output posture, disabled remote/provider-network posture, no-promotion posture, no-persistence posture, and no-action/replay/recovery posture.
- Added behavioral and adversarial coverage for deterministic local stub execution, invalid provider configuration rejection, unsafe request rejection, remote/cloud rejection, fallback rejection, auto-selection rejection, provider-output injection, oversized input, malformed requests, and no-authority guarantees.
- Added Phase 107 operations documentation.

### Changed
- Updated the active phase checklist to Phase 107 procedural truth.

### Notes
- Bounded deterministic local stub provider execution only.
- No remote provider execution.
- No external API calls.
- No cloud model inference.
- Provider output remains untrusted candidate data.
- No provider output promotion.
- No persistence authority.
- No durable append authority.
- No export authority.
- No replay repair.
- No recovery promotion.
- No action execution.
- No readiness approval.
- No Production Candidate approval.
- No release-candidate approval.
- No public-usability approval.
- No production-human-use approval.
- No Phase 108 implementation.

## v0.0.106 - 2026-05-07
**Status:** Phase 106 - Provider Configuration Contract

### Added
- Deterministic provider configuration contracts, parser, validation reason codes, and disabled/untrusted/non-ready posture reports.
- Behavioral and adversarial coverage for malformed, duplicate, unsupported, authority-bearing, execution-enabled, transport-enabled, trust-enabled, auto-selection, fallback, and invalid resource provider configuration payloads.
- Phase 106 operations report and current-phase checklist evidence.

### Changed
- UI API projections now include descriptive provider configuration validation types while remaining non-authoritative.
- Current-phase procedural truth now tracks Phase 106 provider configuration contract work.

### Notes
- Deterministic provider configuration contracts only.
- No provider execution.
- No inference execution.
- No persistence authority.
- No replay repair.
- No recovery promotion.
- No action execution.
- No readiness approval.
- No Production Candidate approval.
- No release-candidate approval.
- No public-usability approval.
- No production-human-use approval.
- No Phase 107 implementation.

## v0.0.105 - 2026-05-07
**Status:** Phase 105 - Transport Abuse Hardening for Live Local Bridge

### Added
- Added `docs/operations/transport-abuse-hardening-phase-105.md` as the Phase 105 operations report.
- Added Rust and UI adversarial transport tests for malformed, truncated, oversized, replay-shaped, duplicate-identifier, authority-bearing, unsupported, invalid-state, invalid enum, invalid typed-field, non-local, and hostile/noise inputs.

### Changed
- Hardened the bounded local UI-to-Rust transport parser and request handling with deterministic fail-closed rejection for hostile transport input.
- Updated `checklists/current-phase.md` to Phase 105 procedural truth.
- Updated `scripts/lint_ui_boundaries.mjs` to resolve TypeScript from the active Node global module path before the legacy fallback so required validation can run.

### Notes
- Phase 105 is transport abuse hardening only.
- Phase 105 includes no transport capability expansion.
- Phase 105 adds no provider execution.
- Phase 105 adds no persistence authority.
- Phase 105 adds no durable append authority.
- Phase 105 adds no export authority.
- Phase 105 adds no replay repair.
- Phase 105 adds no recovery promotion.
- Phase 105 adds no action execution.
- Phase 105 grants no readiness approval.
- Phase 105 grants no Production Candidate approval.
- Phase 105 grants no release-candidate approval.
- Phase 105 grants no public-usability approval.
- Phase 105 grants no production-human-use approval.
- Phase 105 includes no Phase 106 implementation.

## v0.0.104.5 - 2026-05-07
**Status:** Phase 104.5 - Historical Truth Partitioning

### Added
- Added deterministic archived changelog ranges under `docs/changelog/`.
- Added `docs/operations/historical-truth-partitioning-phase-104-5.md` as the Phase 104.5 operations report.

### Changed
- Changed `CHANGELOG.md` into the rolling active historical-truth surface beginning with `v0.0.104.5` and later entries only.
- Updated `checklists/current-phase.md` to Phase 104.5 procedural truth.

### Notes
- Phase 104.5 performs deterministic changelog archival partitioning only.
- Phase 104.5 adds no runtime behavior.
- Phase 104.5 adds no transport behavior.
- Phase 104.5 adds no provider execution.
- Phase 104.5 adds no persistence authority.
- Phase 104.5 adds no replay repair.
- Phase 104.5 adds no recovery promotion.
- Phase 104.5 adds no action execution.
- Phase 104.5 grants no readiness approval.
- Phase 104.5 grants no Production Candidate approval.
- Phase 104.5 grants no release-candidate approval.
- Phase 104.5 grants no public-usability approval.
- Phase 104.5 grants no production-human-use approval.
- Phase 104.5 does not implement Phase 105.
