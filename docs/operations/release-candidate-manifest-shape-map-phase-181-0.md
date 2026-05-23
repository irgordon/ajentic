---
truth_dimension: orientation
authority_level: advisory
mutation_path: readme_update
---

# Phase 181.0 - OOB Release Candidate Manifest Shape Reconnaissance

## Scope
Phase 181.0 is a shape-reconnaissance pass only. It inventories current Rust and TypeScript projection shapes used by the release-candidate evidence chain so Phase 181 can target real repository types.

## Evidence rule
Only current committed repository files were used. Prior chat summaries and speculative target shapes were not used as evidence.

## Failure summary
The prior Phase 181 attempt was rolled back after projection wiring failed due to type-shape mismatches across Rust and TypeScript release-candidate surfaces. No code was committed. This phase documents exact present shapes before retrying implementation.

## Rust projection shape inventory
### ReleaseCandidatePreparationProjection
- Source: `core/src/api/release_candidate_preparation.rs`
- Status field: `status: ReleaseCandidatePreparationStatus` (`code()` helper on enum).
- ID field: `preparation_id: String`.
- Linkage fields: `evidence_items[*].source_linkage`.
- Missing/blocker fields: `missing_evidence`, `blockers`.
- Boundary/capability fields: `boundary_statuses`, `capability_surface`.
- Initial helper: `initial_release_candidate_preparation_projection()`.
- Derivation helper: `derive_release_candidate_preparation_projection(...)`.

### ReleaseArtifactDryPackageProjection
- Source: `core/src/api/release_artifact_dry_package.rs`
- Status field: `status: ReleaseArtifactDryPackageStatus` (`code()` helper on enum).
- ID field: `dry_package_id: Option<String>`.
- Linkage fields: `preparation_id`, `included_evidence[*]` (category + source status/linkage).
- Missing/blocker fields: `missing_evidence`, `blockers`.
- Boundary/capability fields: `boundary_statuses`.
- Initial helper: `initial_release_artifact_dry_package_projection()`.
- Derivation helper: `derive_release_artifact_dry_package(...)`; projection helper also exists: `project_release_artifact_dry_package(...)`.

### ReleaseDryPackageChecksumProvenanceProjection
- Source: `core/src/api/release_dry_package_checksum_provenance.rs`
- Status field: `status: ReleaseDryPackageChecksumProvenanceStatus` (`code()` helper on enum).
- ID field: `provenance_id: Option<String>`.
- Linkage fields: `dry_package_id`, `preparation_id`, `provenance_linkage_summary`.
- Missing/blocker fields: `missing_evidence`, `blockers`.
- Boundary/capability fields: `boundary_statuses`.
- Initial helper: `initial_release_dry_package_checksum_provenance_projection()`.
- Derivation helper: `derive_release_dry_package_checksum_provenance(...)`; projection helper also exists: `project_release_dry_package_checksum_provenance(...)`.

### InstallerDistributionContractProjection
- Source: `core/src/api/installer_distribution_contract.rs`
- Status field: `status: InstallerDistributionContractStatus` (`code()` helper on enum).
- ID field: `contract_id: Option<String>`.
- Linkage fields: `dry_package_linkage`, `checksum_provenance_linkage`.
- Missing/blocker fields: `missing_evidence`, `blockers`.
- Boundary/capability fields: `boundary_statuses`.
- Initial helper: `initial_installer_distribution_contract_projection()`.
- Derivation helper: projection helper `project_installer_distribution_contract(...)` (no `derive_...` helper in current module).

### SigningKeyCustodyDryRunProjection
- Source: `core/src/api/signing_key_custody_dry_run.rs`
- Status field: `status: SigningKeyCustodyDryRunStatus` (`code()` helper on enum).
- ID field: `evidence_id: Option<String>`.
- Linkage fields: `upstream_evidence_linkage`.
- Missing/blocker fields: `missing_evidence`, `blockers`.
- Boundary/capability fields: `boundary_statuses`.
- Initial helper: `initial_signing_key_custody_dry_run_projection()`.
- Derivation helper: `derive_signing_key_custody_dry_run(...)`.

### ReleaseCandidateEvidenceAssemblyProjection
- Source: `core/src/api/release_candidate_evidence_assembly.rs`
- Status field: `status: ReleaseCandidateEvidenceAssemblyStatus` (`code()` helper on enum).
- ID field: `assembly_id: Option<String>`.
- Linkage fields: `evidence_items[*].source_linkage`, `validation_summaries`.
- Missing/blocker fields: `missing_evidence`, `blockers`.
- Boundary/capability fields: `boundary_statuses`.
- Initial helper: `initial_release_candidate_evidence_assembly_projection()`.
- Derivation helper: `derive_release_candidate_evidence_assembly(...)`.

### ReleaseCandidateGapReviewProjection
- Source: `core/src/api/release_candidate_gap_review.rs`
- Status field: `status: ReleaseCandidateGapReviewStatus` (`code()` helper on enum).
- ID field: `gap_review_id: Option<String>`.
- Linkage fields: `source_linkage`.
- Missing/blocker fields: `missing_evidence`, `blockers`.
- Boundary/capability fields: `boundary_statuses`.
- Initial helper: `initial_release_candidate_gap_review_projection()`.
- Derivation helper: `derive_release_candidate_gap_review(...)`.

### ReleaseCandidateDryRunRehearsalProjection
- Source: `core/src/api/release_candidate_dry_run_rehearsal.rs`
- Status field: `status: ReleaseCandidateDryRunRehearsalStatus` (`code()` helper on enum).
- ID field: `rehearsal_id: Option<String>`.
- Linkage fields: `upstream_linkage`, `gap_review_summary`, `rehearsal_artifact_summary`.
- Missing/blocker fields: `missing_evidence`, `blockers`.
- Boundary/capability fields: `boundary_statuses`.
- Initial helper: `initial_release_candidate_dry_run_rehearsal_projection()`.
- Derivation helper: `derive_release_candidate_dry_run_rehearsal(...)`.

### LocalOperatorShellState
- Source: `core/src/api/local_operator_shell_state.rs`
- Relevant linkage fields for Phase 181: `release_candidate_preparation`, `release_artifact_dry_package`, `release_dry_package_checksum_provenance`, `installer_distribution_contract`, `signing_key_custody_dry_run`, `release_candidate_evidence_assembly`, `release_candidate_gap_review`, `release_candidate_dry_run_rehearsal`.
- Initial helper: `initial_local_operator_shell_state()`.
- Derivation/integration points: assignment/refresh logic in `initial_local_operator_shell_state()` and `attach_local_session_evidence_export(...)`.

## TypeScript projection shape inventory
Source for all listed projection types: `ui/src/api/localOperatorShell.ts`.

### release candidate preparation projection
- Type: `ReleaseCandidatePreparationProjection`.
- Status field + literals: `status: ReleaseCandidatePreparationStatus` (`"not_prepared" | "preparation_projected" | "preparation_validated" | "preparation_blocked" | "preparation_rejected" | "invalid_preparation_input"`).
- ID field: `preparationId`.
- Missing/blocker/linkage fields: `missingEvidence`, `blockers`, `evidenceItems[*].sourceLinkage`.
- Initial helper: none exported in current TS shape.
- Derivation helper: `deriveReleaseCandidatePreparationProjection(state)`.
- Render helper: consumed by `renderLocalOperatorShellSnapshot(...)` via view helpers.

### release artifact dry package projection
- Type: `ReleaseArtifactDryPackageProjection`.
- Status field + literals: `status: ReleaseArtifactDryPackageStatus` (literal union).
- ID field: `dryPackageId: string | null`.
- Missing/blocker/linkage fields: `missingEvidence`, `blockers`, `preparationId`, `includedEvidence`.
- Initial helper: `initialReleaseArtifactDryPackageProjection()`.
- Derivation helper: `deriveReleaseArtifactDryPackageProjection(preparation)`.
- Render helper: consumed by `renderLocalOperatorShellSnapshot(...)`.

### checksum/provenance projection
- Type: `ReleaseDryPackageChecksumProvenanceProjection`.
- Status field + literals: `status: ReleaseDryPackageChecksumProvenanceStatus` (literal union).
- ID field: `provenanceId: string | null`.
- Missing/blocker/linkage fields: `missingEvidence`, `blockers`, `dryPackageId`, `preparationId`, `provenanceLinkageSummary`.
- Initial helper: `initialReleaseDryPackageChecksumProvenanceProjection()`.
- Derivation helper: `deriveReleaseDryPackageChecksumProvenanceProjection(dryPackage, preparation)`.
- Render helper: consumed by `renderLocalOperatorShellSnapshot(...)`.

### installer/distribution contract projection
- Type: `InstallerDistributionContractProjection`.
- Status field + literals: `status: "not_defined" | "contract_validated" | "contract_rejected" | "invalid_contract_input"`.
- ID field: `contractId: string | null`.
- Missing/blocker/linkage fields: `missingEvidence`, `blockers`, `dryPackageLinkage`, `checksumProvenanceLinkage`.
- Initial helper: `initialInstallerDistributionContractProjection()`.
- Derivation helper: no `derive...` helper exported in current TS shape.
- Render helper: consumed by `renderLocalOperatorShellSnapshot(...)`.

### signing/key-custody dry-run projection
- Type: `SigningKeyCustodyDryRunProjection`.
- Status field + literals: `status: string` (not a constrained status union in current TS shape).
- ID field: `evidenceId: string | null`.
- Missing/blocker/linkage fields: `missingEvidence`, `blockers`, `upstreamEvidenceLinkage`.
- Initial helper: `initialSigningKeyCustodyDryRunProjection()`.
- Derivation helper: no `derive...` helper exported in current TS shape.
- Render helper: consumed by `renderLocalOperatorShellSnapshot(...)`.

### release candidate evidence assembly projection
- Type: `ReleaseCandidateEvidenceAssemblyProjection`.
- Status field + literals: `status: string`.
- ID field: `assemblyId: string | null`.
- Missing/blocker/linkage fields: `missingEvidence`, `blockers`, `evidenceItems[*].sourceLinkage`.
- Initial helper: `initialReleaseCandidateEvidenceAssemblyProjection()`.
- Derivation helper: no `derive...` helper exported in current TS shape.
- Render helper: consumed by `renderLocalOperatorShellSnapshot(...)`.

### release candidate gap review projection
- Type: `ReleaseCandidateGapReviewProjection`.
- Status field + literals: `status: string`.
- ID field: `gapReviewId: string | null`.
- Missing/blocker/linkage fields: `missingEvidence`, `blockers`, `sourceLinkage`.
- Initial helper: `initialReleaseCandidateGapReviewProjection()`.
- Derivation helper: no `derive...` helper exported in current TS shape.
- Render helper: consumed by `renderLocalOperatorShellSnapshot(...)`.

### release candidate dry-run rehearsal projection
- Type: `ReleaseCandidateDryRunRehearsalProjection`.
- Status field + literals: `status: string`.
- ID field: `rehearsalId: string | null`.
- Missing/blocker/linkage fields: `missingEvidence`, `blockers`, `upstreamLinkage`, `gapReviewSummary`, `rehearsalArtifactSummary`.
- Initial helper: `initialReleaseCandidateDryRunRehearsalProjection()`.
- Derivation helper: no `derive...` helper exported in current TS shape.
- Render helper: consumed by `renderLocalOperatorShellSnapshot(...)`.

### LocalOperatorShellState
- Type: `LocalOperatorShellState`.
- Relevant linkage fields for Phase 181: `releaseCandidatePreparation`, `releaseArtifactDryPackage`, `releaseDryPackageChecksumProvenance`, `installerDistributionContract`, `signingKeyCustodyDryRun`, `releaseCandidateEvidenceAssembly`, `releaseCandidateGapReview`, `releaseCandidateDryRunRehearsal`.
- Initial helper: `initialLocalOperatorShellState()`.

## Local shell integration points
- Rust integration pipeline currently refreshes release-candidate projections inside `initial_local_operator_shell_state()` and `attach_local_session_evidence_export(...)` in a fixed order (preparation -> dry package -> checksum/provenance -> installer contract -> signing dry run -> gap review -> dry-run rehearsal).
- TypeScript integration pipeline composes initial + derived projection state inside `projectLocalOperatorShellState(...)` and `initialLocalOperatorShellState()` in `ui/src/api/localOperatorShell.ts`.

## Upstream evidence surfaces
- Rust upstream evidence source for release-candidate preparation derives from workflow, trial, provider, candidate, replay, export, package, and restore projections via `derive_release_candidate_preparation_input_snapshot(...)`.
- Downstream RC chain currently depends on `release_candidate_preparation` as primary seed, then threads through dry package/checksum/installer/signing/gap/rehearsal projections.

## Required adapter/helper functions for Phase 181
- Rust: new Phase 181 manifest adapter should consume existing helpers rather than bypassing them: `derive_release_candidate_preparation_projection`, `project_release_artifact_dry_package`, `project_release_dry_package_checksum_provenance`, `project_installer_distribution_contract`, `derive_signing_key_custody_dry_run`, `derive_release_candidate_evidence_assembly`, `derive_release_candidate_gap_review`, `derive_release_candidate_dry_run_rehearsal`.
- TypeScript: Phase 181 UI adapter should consume existing projection fields and current render surface path via `renderLocalOperatorShellSnapshot(...)` to avoid duplicating status logic.

## Type-shape mismatch risks
- Rust uses strongly typed enums with `code()` helpers, while TypeScript mixes strict literal unions and broad `string` status fields (notably signing/evidence-assembly/gap-review/rehearsal).
- Rust uses `Option<String>` IDs; TypeScript uses `string | null` IDs; adapter boundaries must preserve nullability semantics.
- Field naming differs (`snake_case` in Rust vs `camelCase` in TypeScript).
- Some modules expose `derive_*`; others expose `project_*` only (installer contract). Assuming uniform helper naming will fail.
- Evidence linkage element shapes differ by projection; a one-size-fits-all manifest item mapping will likely mismatch.

## Recommended Phase 181 implementation strategy
1. Implement Rust manifest-shape production from already-derived `LocalOperatorShellState` projection fields only.
2. Add dedicated normalization helpers for status/id/linkage extraction per projection instead of generic reflection.
3. Treat TypeScript as display/transport surface: mirror Rust manifest payload shape explicitly with constrained mapping helpers.
4. Gate status mapping with projection-specific tests before wiring into broader release-candidate flow.
5. Integrate in smallest vertical slice: preparation + dry package + checksum first, then installer/signing/assembly/gap/rehearsal.

## Files Phase 181 should touch
- `core/src/api/` files that implement new manifest adapter/projection wiring for existing RC chain types.
- `ui/src/api/` files that map/render the manifest surface.
- corresponding Rust and TypeScript tests for new adapter behavior.

## Files Phase 181 should not touch
- governance/roadmap architecture authority documents for behavior changes.
- unrelated provider/runtime transport modules.
- non-RC projection modules outside required adapter path.

## Validation output
- Required reconnaissance commands were run exactly as requested.
- `CARGO_TARGET_DIR=/tmp/ajentic-phase-181-0-target ./scripts/check.sh` passed.
- `git diff --check` passed.
- Final `git status --short` confirmed only allowed files changed.

## Phase 181 retry guidance
- Start from clean tree and re-run the same shape inventory greps as a preflight.
- Use this Phase 181.0 map as the authoritative implementation baseline for field names, status surfaces, helper names, and projection-linkage order.
- Do not assume missing `derive_*` helpers; use `project_*` where that is the module’s current contract.
- Keep release boundaries unchanged: no signing/publishing/deployment/release/public-approval behavior in manifest-shape implementation.
