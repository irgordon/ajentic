---
truth_dimension: orientation
authority_level: advisory
mutation_path: readme_update
---

# Local Artifact Manifest Validation - Phase 136.2

## Scope

Phase 136.2 adds executable Rust validation for governed local artifact manifest evidence. The scope is intentionally narrow: validate manifest shape, deterministic local fields, non-public boundaries, unsafe paths, prohibited claims, and artifact-chain separation.

## Evidence rule

A local artifact manifest is evidence about a local artifact boundary only. It is not evidence that an artifact was created, checksummed, attested, signed, published, installed, updated, deployed, or approved for any readiness category.

## Phase 136.2 implementation boundary

The implementation boundary is the Rust API surface exposed by `validate_local_artifact_manifest`. The function is pure validation over caller-supplied manifest data and returns a closed validation report.

## Manifest validation is evidence validation, not release approval

Accepted manifest evidence means only that the supplied manifest conforms to the local non-public manifest boundary. It does not change Phase 130 decision status and does not grant Release Candidate, Production Candidate, public/general-use, or production-human-use readiness.

## Rust validation surface

Phase 136.2 introduces these Rust types:

- `LocalArtifactManifest`
- `LocalArtifactManifestValidationReport`
- `ArtifactManifestValidationStatus`
- `LocalArtifactManifestValidationReason`
- `LocalArtifactKind`
- `LocalArtifactKindField`
- `LocalArtifactEvidenceStatus`
- `validate_local_artifact_manifest`

## Accepted manifest fields

The validator requires or explicitly evaluates:

- `artifact_id`
- `artifact_name`
- `artifact_kind`
- `source_revision`
- `build_command`
- `output_path`
- `created_by_phase`
- `non_public`
- `release_artifact_claim`
- `checksum_status`
- `provenance_status`
- `signing_status`
- `publishing_status`
- `deployment_status`
- `readiness_claim`
- `deferred_to_phase`
- `extra_fields`

## Rejection reasons

The closed rejection reason vocabulary includes:

- `missing_required_field`
- `invalid_artifact_id`
- `invalid_artifact_kind`
- `unsafe_output_path`
- `output_path_not_local_artifacts`
- `non_public_not_true`
- `release_artifact_claim_present`
- `readiness_claim_present`
- `publishing_claim_present`
- `deployment_claim_present`
- `signing_claim_present`
- `checksum_claim_not_deferred`
- `provenance_claim_not_deferred`
- `installer_or_update_channel_claim_present`
- `invalid_created_by_phase`
- `invalid_deferred_to_phase`
- `malformed_manifest`

## Path safety rules

Accepted output paths must be relative paths under `artifacts/local/`. The validator rejects absolute paths, home-directory paths, traversal paths, `.git`, `target`, `node_modules`, system-shaped directories, and public/download/release/dist shaped paths.

## Non-public boundary

The manifest must explicitly set `non_public` to true. Any missing or false value rejects the manifest.

## Cross-category inference rejection

Manifest validation does not infer checksum, provenance, signing, publishing, installer/update-channel, deployment, or readiness evidence. Checksum and provenance statuses must be absent or deferred; completed claims reject unless future governed evidence introduces a separate validation boundary.

## Filesystem non-mutation statement

The validator does not write files, create directories, create artifacts, generate checksums, create provenance attestations, sign outputs, publish outputs, deploy outputs, or mutate readiness state.

## Release/deployment/signing/publishing prohibition

Release artifact claims, deployment claims, signing claims, publishing claims, installer claims, and update-channel claims reject. Phase 136.2 creates no release, deployment, signing, publishing, installer, or update-channel behavior.

## Readiness prohibition

Readiness claims reject. Phase 136.2 does not approve Release Candidate status, Production Candidate status, public/general use, or production-human use.

## Phase 133 handoff

Checksum and provenance remain separate from manifest validation. Phase 136.2 can record that checksum and provenance evidence is deferred to Phase 133-compatible evidence work, but it does not create that evidence.

## Phase 139 handoff

Artifact-chain reassembly remains a downstream boundary. Phase 136.2 produces a deterministic manifest validation input that Phase 139 can later evaluate without treating manifest evidence as checksum, provenance, signing, publishing, or deployment evidence.

## Validation log

Phase 136.2 validation runs Rust tests, adversarial tests, diff checks, artifact scans, authority scans, guarded diff scans, and final git hygiene checks. Expected output is no artifact, checksum, provenance, signing, installer, update-channel, public release, deployment, or readiness file creation.
