---
truth_dimension: orientation
authority_level: advisory
mutation_path: readme_update
---

# Phase 191 - Artifact, Signing, and Release Workflow Activation Boundary

## Decision

Phase 191 activates internal release-candidate artifact packaging and the manually triggered release-candidate workflow.

GitHub Actions is release-platform infrastructure. A workflow file alone does not prove repository environment protection, branch protection, required reviewers, or release approval. Branch protection remains a repository settings requirement. Required reviewer environments may be used later, and if GitHub environment required reviewers are configured manually, that must be recorded as remote evidence.

Phase 191 does not simulate branch protection in code and does not claim branch protection unless it can be verified.

## Activated Surfaces

- Internal release-candidate bundle generation.
- Deterministic internal candidate bundle manifest.
- Internal bundle checksum evidence.
- Internal checksum, SBOM, and provenance evidence reuse from Phase 189.
- Manually triggered `release-candidate` GitHub Actions workflow.
- Optional short-retention internal workflow artifact upload.
- GitHub artifact attestation boundary for internal candidate artifacts.

## Internal Candidate Bundle

The internal candidate bundle includes:

- Rust release executable candidates.
- UI `dist` files.
- Internal checksum evidence.
- Internal SBOM evidence.
- Internal provenance evidence.
- `release-candidate-manifest.json`.
- `bundle-checksums.json`.
- `README-INTERNAL-CANDIDATE.txt`.

The bundle is an internal candidate bundle. It is not a public release artifact, not v1.0 release execution, not production approval, not an installer, and not an update-channel artifact.

## Workflow Boundary

The `release-candidate` workflow is `workflow_dispatch` only.

It requires:

- `candidate_label`.
- `expected_commit`.
- `upload_internal_artifact`.
- `attest_internal_artifact`.
- `confirm_no_public_release=NO_PUBLIC_RELEASE`.

The workflow must fail closed if the checked-out commit does not match the expected commit or if the no-public-release confirmation is absent.

Top-level and build-job permissions are `contents: read`. Attestation permissions are scoped to the attestation job only:

- `contents: read`
- `id-token: write`
- `attestations: write`

No workflow has `contents: write`, `packages: write`, `deployments: write`, `actions: write`, or `artifact-metadata: write`.

## Attestation Boundary

Phase 191 allows GitHub artifact attestations for internal candidate artifacts only.

Allowed subjects:

- Internal candidate bundle archive.
- Internal SBOM evidence.

Attestation evidence is internal release-candidate evidence. It is not release approval, public release publication, production signing, installer signing, OS code signing, notarization, package signing, final release provenance, or final v1 provenance.

If attestation is skipped by workflow input, signing/attestation status remains skipped and no signing success is inferred.

## Non-Release Statement

Phase 191 does not create Git tags.

Phase 191 does not create GitHub Releases.

Phase 191 does not publish packages.

Phase 191 does not publish public artifacts.

Phase 191 does not create installers.

Phase 191 does not create update channels.

Phase 191 does not deploy.

Phase 191 does not approve production use.

Phase 191 does not mark v1.0 released.

Workflow artifacts are short-retention CI review artifacts, not distribution artifacts.

RC publication remains Phase 193.

Final functional acceptance and release execution remain Phase 194.

## Local Evidence Notes

Root npm commands are not applicable while the repository has no root `package.json`; UI package validation runs from `ui/`.

Local GitHub Actions emulation with `act` is optional and was not required for Phase 191 acceptance.

If `gh` is unavailable locally, GitHub Releases, branch protection, and remote workflow conclusions must be recorded as unknown from local tooling rather than inferred.
