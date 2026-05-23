---
truth_dimension: procedural
authority_level: authoritative
mutation_path: checklist_revision
---

# Phase 181.0 - OOB Release Candidate Manifest Shape Reconnaissance

- Phase name: Phase 181.0 - OOB Release Candidate Manifest Shape Reconnaissance
- Phase goal: map actual Rust/TypeScript projection shapes for Phase 181 manifest wiring before any code-production retry.
- Working-tree hygiene gate: start clean, only touch allowed files, run required validation, end with zero unintended drift.
- Allowed surfaces: `docs/operations/release-candidate-manifest-shape-map-phase-181-0.md`, `CHANGELOG.md`, `checklists/current-phase.md`.

## Shape inventory checklist
- [x] Required reconnaissance commands executed against `core/src/api` and `ui/src/api`.
- [x] Shape map document created with required sections.

## Rust projection inventory checklist
- [x] `ReleaseCandidatePreparationProjection` mapped (status/id/linkage/missing/blocker/boundary/helpers).
- [x] `ReleaseArtifactDryPackageProjection` mapped.
- [x] `ReleaseDryPackageChecksumProvenanceProjection` mapped.
- [x] `InstallerDistributionContractProjection` mapped.
- [x] `SigningKeyCustodyDryRunProjection` mapped.
- [x] `ReleaseCandidateEvidenceAssemblyProjection` mapped.
- [x] `ReleaseCandidateGapReviewProjection` mapped.
- [x] `ReleaseCandidateDryRunRehearsalProjection` mapped.
- [x] `LocalOperatorShellState` RC linkage fields + initializer mapped.

## TypeScript projection inventory checklist
- [x] release candidate preparation projection mapped.
- [x] release artifact dry package projection mapped.
- [x] checksum/provenance projection mapped.
- [x] installer/distribution contract projection mapped.
- [x] signing/key-custody dry-run projection mapped.
- [x] release candidate evidence assembly projection mapped.
- [x] release candidate gap review projection mapped.
- [x] release candidate dry-run rehearsal projection mapped.
- [x] `LocalOperatorShellState` RC linkage fields + initializer mapped.

## Local shell integration checklist
- [x] Rust integration order documented.
- [x] TypeScript projection composition path documented.

## Type-shape risk checklist
- [x] Rust enum vs TypeScript literal/string status mismatch risk documented.
- [x] ID/nullability and naming-shape mismatch risks documented.
- [x] helper naming inconsistency risk (`derive_*` vs `project_*`) documented.

## Phase 181 retry checklist
- [x] Required adapter/helper function surfaces identified.
- [x] Recommended implementation sequence documented.
- [x] Phase 181 remains next code-production phase.

## Validation checklist
- [x] `git status --short` run before edits.
- [x] Required `rg` commands run.
- [x] `CARGO_TARGET_DIR=/tmp/ajentic-phase-181-0-target ./scripts/check.sh` passed.
- [x] `git diff --check` passed.
- [x] `git status --short` run after edits.

## Zero-drift checklist
- [x] Shape map document exists.
- [x] All required Rust projection shapes documented.
- [x] All required TypeScript projection shapes documented.
- [x] Local shell integration points documented.
- [x] Type-shape mismatch risks documented.
- [x] Phase 181 retry strategy documented.
- [x] No Rust source changes.
- [x] No TypeScript source changes.
- [x] No tests changed.
- [x] No schema changes.
- [x] No roadmap/governance/architecture/scripts/package/lockfile/CI drift.
