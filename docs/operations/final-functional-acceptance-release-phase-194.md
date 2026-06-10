---
truth_dimension: orientation
authority_level: advisory
mutation_path: readme_update
---

# Phase 194 - Final Functional Acceptance and Release Execution

## Decision

Phase 194 is the final functional acceptance and v1.0.0 GitHub Release execution phase.

The source release candidate is `v1.0.0-rc.1`.

The final tag is `v1.0.0`.

The final GitHub Release is final, not prerelease, and not draft when publication succeeds.

## Execution Path

The final release path is the manually dispatched `.github/workflows/final-release.yml` workflow.

The workflow must run from `main`, match the supplied `expected_commit`, enforce `final_tag=v1.0.0`, enforce `source_rc_tag=v1.0.0-rc.1`, and require these confirmations:

- `FINAL_V1_RELEASE`
- `NO_PACKAGE_PUBLICATION`
- `NO_INSTALLER_UPDATE_DEPLOY`

The workflow uses the GitHub REST API from repository-scoped workflow credentials. It must create only the annotated `v1.0.0` tag, the final GitHub Release, final GitHub Release assets, and optional GitHub artifact attestations for selected final assets.

## Final Assets

Final release assets are GitHub Release assets only.

The asset set includes:

- Final candidate bundle archive.
- Final asset manifest JSON.
- Final checksum evidence JSON.
- Final SBOM evidence JSON.
- Final provenance evidence JSON.
- Final release notes markdown.
- Final release README text.

Asset names include `ajentic` and `v1.0.0`. Asset names must not include RC, prerelease, draft, unstable, installer, update-channel, or deployment markers.

## Boundaries

Phase 194 does not publish npm packages.

Phase 194 does not publish Cargo packages.

Phase 194 does not create an installer.

Phase 194 does not activate an update channel.

Phase 194 does not deploy.

Phase 194 does not add OS signing or notarization.

Release assets and attestations are evidence only. They do not become governance authority.

Rust remains authority.

TypeScript remains visibility and review.

Python remains adaptation and support scripting.

Bash remains glue.

## Branch Protection

Branch protection or ruleset status must be resolved before final workflow dispatch.

If branch protection or rulesets are disabled, Phase 194 requires an explicit final-release Owner exception. The earlier RC-only exception does not carry into final release execution.

## Package Metadata

Package metadata remains separately governed in Phase 194.

The final GitHub Release identity may be `v1.0.0` while `core/Cargo.toml` and `ui/package.json` remain at their existing package versions. Package metadata alignment does not imply package registry publication.

## Completion Evidence

Phase 194 completion requires remote final release evidence:

- Final workflow success.
- Remote `v1.0.0` tag exists.
- Remote `v1.0.0` tag targets the Phase 194 commit.
- GitHub Release `v1.0.0` exists.
- Release is not draft.
- Release is not prerelease.
- Release is latest.
- Final assets are attached.
- Checksum, SBOM, and provenance evidence assets are attached.
- Attestation behavior is recorded when enabled.
- `v1.0.0-rc.1` remains intact.
- No package publication, installer, update channel, deployment, OS signing, notarization, backend authority change, or UI authority change occurred.
