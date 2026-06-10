---
truth_dimension: orientation
authority_level: advisory
mutation_path: readme_update
---

# Phase 195 - Post-v1 Release Closure and Evidence Preservation

## Decision

AJENTIC v1.0.0 is published as a final GitHub Release.

Phase 195 preserves final release evidence, updates repository status surfaces, records post-v1 restrictions, and defines the next maintenance boundary. It is not a feature phase and not another release execution phase.

## Final Release Evidence

- Final GitHub Release: https://github.com/irgordon/ajentic/releases/tag/v1.0.0
- Final workflow: https://github.com/irgordon/ajentic/actions/runs/27307966684
- Final workflow event: `workflow_dispatch`
- Final workflow conclusion: `success`
- Final tag: `v1.0.0`
- Final tag type: annotated
- Final tag target commit: `6734a5b3cc223c41288b14575ad40f6fcf23fb6f`
- Source RC: `v1.0.0-rc.1`
- Source RC remains intact: yes
- Deployments API evidence: `[]`

The final GitHub Release is not draft, not prerelease, and is the latest release.

## Final Assets

- `README-FINAL-v1.0.0.txt`
- `ajentic-v1.0.0-final-asset-manifest.json`
- `ajentic-v1.0.0-final-candidate-bundle.tar.gz`
- `ajentic-v1.0.0-final-checksums.json`
- `ajentic-v1.0.0-final-provenance.json`
- `ajentic-v1.0.0-final-sbom.json`
- `ajentic-v1.0.0-release-notes.md`

## Attestation Evidence

Recorded final attestation evidence:

https://github.com/irgordon/ajentic/attestations/30684159

Local curl re-verification was blocked by local DNS failure for `github.com` during the final evidence pass. Re-check the attestation with GitHub's official attestation verification tooling when network/DNS is available.

## Branch Protection

The branch protection gate was closed before final dispatch through classic branch protection on `main`.

Recorded settings:

- Pull request required.
- One approval required.
- Required status checks enabled.
- Branches required to be up to date.
- Conversation resolution required.
- Linear history required.
- Admin bypass disabled.
- Force pushes disabled.
- Branch deletions disabled.

## Package Metadata Policy

GitHub Release identity and package registry identity remain separate.

Current package metadata:

- `core/Cargo.toml`: `ajentic-core` version `0.1.0`, license `MIT`.
- `ui/package.json`: `ajentic-ui` version `0.1.0`, private package, license `MIT`.
- `ui/package-lock.json`: `ajentic-ui` version `0.1.0`, license `MIT`.

No npm package publication occurred. No Cargo package publication occurred. No package registry publication occurred. Package metadata was not changed to `1.0.0` in Phase 195.

## Workflow Boundary

Workflow inspection records:

- `.github/workflows/final-release.yml` remains `workflow_dispatch` only.
- `.github/workflows/rc-publication.yml` remains `workflow_dispatch` only.
- `.github/workflows/ci.yml` remains validation-only.
- `.github/workflows/memory-lint.yml` remains validation-only.
- `.github/workflows/reproducible-artifacts.yml` remains validation/evidence-only.
- `contents: write` appears only in release publication jobs.
- `id-token: write` and `attestations: write` appear only in attestation jobs.
- No workflow has `packages: write`.
- No workflow has `deployments: write`.
- No workflow has `artifact-metadata: write`.
- No workflow publishes npm or Cargo packages.
- No workflow deploys.
- No workflow creates installers or update channels.

## Post-v1 Restrictions

Phase 195 created no new tag and no new GitHub Release.

No package publication occurred.

No installer exists.

No update channel exists.

No deployment exists.

No OS signing or notarization occurred.

No backend authority change occurred.

No UI authority change occurred.

Release artifacts, checksums, SBOM/provenance files, attestations, workflows, scripts, UI surfaces, and documentation are release/evidence surfaces. They do not become governance authority.

Rust remains authority.

TypeScript remains visibility.

Python remains adaptation.

Bash remains glue.

Post-v1 work must be scoped through explicit future phases. Allowed next-phase categories include maintenance, defect repair, package-publication planning, installer planning, update-channel planning, deployment planning, signing/notarization planning, documentation correction, and evidence verification.
