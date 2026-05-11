---
truth_dimension: orientation
authority_level: advisory
mutation_path: readme_update
---

# Local Artifact Manifest Producer - Phase 132.3

## Scope

Phase 132.3 adds the smallest deterministic Rust producer for a local, non-public artifact manifest candidate. The producer creates manifest evidence in memory only and validates the candidate through the existing local artifact manifest validator.

## Evidence rule

A produced manifest candidate is evidence about a local artifact boundary only. It is not evidence that a payload artifact, checksum, provenance attestation, signature, installer, update channel, public asset, deployment artifact, or readiness approval exists.

## Phase 132.3 implementation boundary

The implementation boundary is the Rust API surface in `core/src/api/local_artifact_manifest.rs`. The phase accepts explicit input fields, derives a bounded default manifest path under `artifacts/local/`, constructs a manifest candidate, and validates it before returning a report.

## Manifest production is not artifact release

Manifest production does not create or release an artifact. The producer does not package binaries, publish public files, sign outputs, deploy outputs, install outputs, or activate update metadata.

## Phase 130 carry-forward

Phase 130 decision status remains `rc_candidate_not_ready`. Phase 132.3 does not change Release Candidate status, Production Candidate status, public/general-use status, or production-human-use status.

## Phase 132 / 132.1 / 133 / 134 / 135 / 135.1 / 136 / 136.2 relationship

Phase 132 deferred artifact creation. Phase 132.1 corrected the future artifact contract only. Phase 133 checksum/provenance remains blocked by missing governed artifact evidence. Phase 134 signing/key-custody remains blocked by missing artifact, checksum, provenance, manifest, and key-custody decision evidence. Phase 135 deferred Phase 136 implementation. Phase 135.1 kept the artifact chain deferred because no deterministic artifact command existed. Phase 136 deferred installer/update-channel implementation. Phase 136.2 validates local artifact manifest evidence but does not produce it. Phase 132.3 fills only the manifest-candidate production gap and keeps all downstream evidence categories deferred or absent.

## Rust producer surface

Phase 132.3 introduces:

- `LocalArtifactManifestProducerInput`
- `LocalArtifactManifestProducerReport`
- `LocalArtifactManifestProducerStatus`
- `LocalArtifactManifestProducerReason`
- `produce_local_artifact_manifest_candidate`

## Manifest validation integration

The producer calls `validate_local_artifact_manifest` before returning a successful report. A produced candidate is successful only when validation returns the accepted local non-public status.

## Determinism rule

Identical producer inputs must return identical reports. The producer does not read wall-clock time, environment variables, filesystem state, network state, provider output, or mutable global state.

## Filesystem non-write default

The default producer function does not write files or create directories. Successful reports include the bounded reason that filesystem writing was not requested.

## Optional write boundary if implemented

Phase 132.3 does not implement a manifest write function. If a later phase adds writing, that function must be separate, fail closed on unsafe paths, and write only manifest JSON under `artifacts/local/`.

## Path safety

Producer output paths must be relative, must remain under `artifacts/local/`, and must not contain traversal, home-directory, absolute, system-directory, `.git`, `target`, `node_modules`, public, download, release, or dist-shaped segments.

## Non-public boundary

The produced manifest sets `non_public` to true and keeps `release_artifact_claim` and `readiness_claim` false.

## Cross-category inference rejection

Manifest production does not infer checksum evidence, provenance evidence, signing evidence, publishing evidence, deployment evidence, installer/update-channel evidence, or readiness evidence. Checksum, provenance, and signing statuses remain deferred; publishing and deployment statuses remain absent.

## Release/deployment/signing/publishing prohibition

Phase 132.3 adds no release, deployment, signing, publishing, installer, update-channel, monitoring, telemetry, provider execution, persistence authority, replay repair, recovery promotion, or action execution behavior.

## Readiness prohibition

Phase 132.3 does not approve readiness. Readiness-shaped artifact names or text do not become readiness claims.

## Phase 133 handoff

The produced manifest can defer checksum and provenance evidence to Phase 133-compatible work, but it does not generate checksums or provenance attestations.

## Phase 139 handoff

The produced manifest can defer artifact-chain reassembly to Phase 139-compatible work, but it does not create artifacts, public assets, installers, update channels, or deployment behavior.

## Validation log

Phase 132.3 validation covers Rust producer tests, local artifact manifest validation tests, adversarial unsafe-path tests, prohibited-claim tests, deterministic repeat production, non-write default behavior, artifact scans, authority scans, guarded diff scans, and final git hygiene.
