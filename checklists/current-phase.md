---
truth_dimension: procedural
authority_level: authoritative
mutation_path: checklist_revision
---

# Current Phase Checklist - Phase 136.2 Local Artifact Manifest Validation

## Working-tree hygiene
- [x] Inspect repository state before changes.
- [x] Limit changes to Phase 136.2 requested surfaces.
- [x] Preserve Phase 130 `rc_candidate_not_ready` starting truth.
- [x] Commit Phase 136.2 changes after validation.

## Allowed surfaces
- [x] Rust API implementation under `core/src/api/`.
- [x] Rust integration/adversarial tests under `tests/`.
- [x] Phase 136.2 operations report under `docs/operations/`.
- [x] Active checklist update.
- [x] Active changelog update.
- [x] No TypeScript, schema, workflow, governance, architecture, roadmap, package, or lockfile drift.

## Rust implementation checklist
- [x] Add `LocalArtifactManifest`.
- [x] Add `LocalArtifactManifestValidationReport`.
- [x] Add `ArtifactManifestValidationStatus`.
- [x] Add `LocalArtifactManifestValidationReason`.
- [x] Add `validate_local_artifact_manifest`.
- [x] Export the validation surface from the Rust API module.
- [x] Keep validation pure and non-mutating.

## Manifest field checklist
- [x] Require or explicitly evaluate `artifact_id`.
- [x] Require or explicitly evaluate `artifact_name`.
- [x] Require or explicitly evaluate `artifact_kind`.
- [x] Require or explicitly evaluate `source_revision`.
- [x] Require or explicitly evaluate `build_command`.
- [x] Require or explicitly evaluate `output_path`.
- [x] Require or explicitly evaluate `created_by_phase`.
- [x] Require or explicitly evaluate `non_public`.
- [x] Require or explicitly evaluate `release_artifact_claim`.
- [x] Require or explicitly evaluate `checksum_status`.
- [x] Require or explicitly evaluate `provenance_status`.
- [x] Require or explicitly evaluate `signing_status`.
- [x] Require or explicitly evaluate `publishing_status`.
- [x] Require or explicitly evaluate `deployment_status`.
- [x] Require or explicitly evaluate `readiness_claim`.
- [x] Require or explicitly evaluate `deferred_to_phase`.

## Path safety checklist
- [x] Accept only relative output paths.
- [x] Accept only paths under `artifacts/local/`.
- [x] Reject absolute paths.
- [x] Reject traversal paths.
- [x] Reject home-directory paths.
- [x] Reject system paths.
- [x] Reject public/download/release-shaped paths.
- [x] Reject `.git` paths.
- [x] Reject `target` and `node_modules` paths.

## Rejection reason checklist
- [x] Include `missing_required_field`.
- [x] Include `invalid_artifact_id`.
- [x] Include `invalid_artifact_kind`.
- [x] Include `unsafe_output_path`.
- [x] Include `output_path_not_local_artifacts`.
- [x] Include `non_public_not_true`.
- [x] Include `release_artifact_claim_present`.
- [x] Include `readiness_claim_present`.
- [x] Include `publishing_claim_present`.
- [x] Include `deployment_claim_present`.
- [x] Include `signing_claim_present`.
- [x] Include `checksum_claim_not_deferred`.
- [x] Include `provenance_claim_not_deferred`.
- [x] Include `installer_or_update_channel_claim_present`.
- [x] Include `invalid_created_by_phase`.
- [x] Include `invalid_deferred_to_phase`.
- [x] Include `malformed_manifest`.

## Adversarial test checklist
- [x] Public release claim rejects.
- [x] Production deployment claim rejects.
- [x] Release Candidate readiness claim rejects.
- [x] Production Candidate readiness claim rejects.
- [x] Signing complete claim rejects.
- [x] Provenance complete claim rejects.
- [x] Checksum complete claim rejects.
- [x] Path escaping `artifacts/local/` rejects.
- [x] Path targeting release/download/dist/public directories rejects.
- [x] Path targeting system directories rejects.

## Non-mutation checklist
- [x] Validation mutates no filesystem state.
- [x] Validation creates no artifacts.
- [x] Validation creates no checksums.
- [x] Validation creates no provenance attestations.
- [x] Validation signs nothing.
- [x] Validation publishes nothing.
- [x] Validation deploys nothing.
- [x] Validation does not mutate input state.

## Release/deployment/signing/publishing prohibition checklist
- [x] Reject release artifact claims.
- [x] Reject publishing claims.
- [x] Reject deployment claims.
- [x] Reject signing claims.
- [x] Reject installer/update-channel shaped claims.
- [x] Do not create release artifacts.
- [x] Do not create publishing behavior.
- [x] Do not create deployment automation.
- [x] Do not create signing behavior.
- [x] Do not create installer/update-channel behavior.

## Readiness prohibition checklist
- [x] Reject readiness claims.
- [x] Do not approve Release Candidate status.
- [x] Do not approve Production Candidate status.
- [x] Do not approve public/general use.
- [x] Do not approve production-human use.

## Validation log
- [x] Run `CARGO_TARGET_DIR=/tmp/ajentic-phase-136-2-target ./scripts/check.sh`.
- [x] Run `cargo test --manifest-path core/Cargo.toml --all-targets`.
- [x] Run `cargo test --manifest-path core/Cargo.toml local_artifact_manifest --all-targets`.
- [x] Run `cargo test --manifest-path core/Cargo.toml adversarial --all-targets`.
- [x] Run `git diff --check`.
- [x] Run `git status --short`.
- [x] Run artifact scan.
- [x] Run authority scan.
- [x] Run guarded diff scan.

## Zero-drift checklist
- [x] No TypeScript drift.
- [x] No schema drift.
- [x] No workflow drift.
- [x] No governance-doc drift.
- [x] No architecture-doc drift.
- [x] No roadmap drift.
- [x] No package or lockfile drift.
- [x] No artifact, checksum, provenance, signature, installer, update-channel, public asset, or deployment artifact creation.
- [x] No readiness claims introduced.
- [x] CHANGELOG entry matches actual diff.
- [x] Final git status clean after commit.
