---
truth_dimension: orientation
authority_level: advisory
mutation_path: readme_update
---

# Phase 193 - Release Candidate Publication

## Decision

Phase 193 authorizes publication of the bounded `v1.0.0-rc.1` Release Candidate for final acceptance review.

Publication is performed by the manually dispatched `rc-publication` GitHub Actions workflow. The workflow uses the GitHub REST API from repository-scoped workflow credentials instead of the `gh` CLI.

## Publication Scope

The RC workflow may:

- Validate the exact commit selected for publication.
- Build deterministic RC assets from the existing internal candidate bundle path.
- Produce public RC checksums, SBOM, provenance, release notes, and asset manifest files.
- Generate GitHub artifact attestations for selected RC assets.
- Create the `v1.0.0-rc.1` tag.
- Create a GitHub prerelease for `v1.0.0-rc.1`.
- Upload the bounded RC assets to that prerelease.

## Fail-Closed Inputs

The workflow requires:

- `rc_tag` matching `v1.0.0-rc.<number>`.
- `expected_commit` matching the checked-out commit.
- `candidate_label` matching the RC candidate label.
- `confirm_public_rc` set to `PUBLIC_RC_ONLY`.
- `confirm_no_final_release` set to `NO_FINAL_V1_RELEASE`.
- `upload_public_rc_assets` set to `true`.
- `attest_public_rc_assets` selected explicitly.

It must fail before publication if the tag or release already exists, if the branch is not `main`, if package metadata is final `1.0.0`, if release notes or assets are missing, or if validation fails.

## Workflow Permissions

The workflow keeps default permissions at `contents: read`.

Only the attestation job may use:

- `id-token: write`
- `attestations: write`

Only the publication job may use:

- `contents: write`

The workflow does not use `packages: write`, `deployments: write`, `actions: write`, or `artifact-metadata: write`.

## Non-Final Boundary

Phase 193 does not create final `v1.0.0`.

Phase 193 does not approve production readiness.

Phase 193 does not publish npm or Cargo packages.

Phase 193 does not create installers.

Phase 193 does not activate update channels.

Phase 193 does not deploy.

Phase 193 does not change package versions.

Phase 193 does not let TypeScript compute release authority.

Final v1 acceptance and final release execution remain Phase 194.
