---
truth_dimension: orientation
authority_level: advisory
mutation_path: readme_update
---

# Phase 186 - GitHub Actions Release Platform Contract

## Scope

Phase 186 defines GitHub Actions as AJENTIC's intended future release execution platform.

This is a release-platform contract phase. It is not a release-execution phase.

Phase 186 does not create release authority, production authority, public-use authority, signing authority, publishing authority, deployment authority, installer authority, update-channel authority, GitHub Release authority, or release-tag authority.

## Platform decision

GitHub Actions is the intended future release platform for AJENTIC.

GitHub Actions is not yet release-authoritative in the sense of executing releases.

Local developer machines may build, validate, and produce local evidence. Local machines must not be treated as authoritative release publishers.

Release execution requires explicit approval from later roadmap phases. Phase 201 is the first possible v1.0 release execution phase, and only if Phase 200 approves.

## Validation workflow contract

Validation workflows produce evidence. They do not approve releases.

Validation workflows must not create tags, GitHub Releases, public artifacts, signatures, packages, installers, update channels, deployments, or release branches.

Validation workflows should run on:

- pull requests
- manual `workflow_dispatch`
- pushes to `main`

Validation workflows should use least-privilege permissions, preferably:

```yaml
permissions:
  contents: read
```

## Existing workflow inspection

| Workflow | Trigger coverage after Phase 186 | Permissions | Release authority |
| --- | --- | --- | --- |
| `.github/workflows/ci.yml` | `pull_request`, `workflow_dispatch`, `push` to `main` | `contents: read` | Validation only; no release execution. |
| `.github/workflows/memory-lint.yml` | `pull_request`, `workflow_dispatch`, `push` to `main` | `contents: read` | Memory placement validation only; no release execution. |

No release workflow exists in `.github/workflows`.

No `release.yml`, `publish.yml`, or `deploy.yml` workflow exists.

## Future release workflow rules

Any future release workflow must fail closed.

Any future release workflow must not silently continue on missing evidence.

Any future release workflow must not infer approval from passing tests alone.

Any future release workflow must not publish from unapproved branches or unapproved commits.

Any future release workflow must not create tags as a side effect of validation.

Any future release workflow must not use broad write permissions unless a later release phase explicitly requires them.

Generated output, adapters, UI, scripts, and infrastructure do not become governance authority.

## Prohibited in Phase 186

Phase 186 does not add:

- release workflow execution
- public release artifact uploads
- GitHub Release creation
- Git tag creation
- `npm publish`
- `cargo publish`
- package registry publishing
- code signing
- notarization
- signing keys or secrets
- environment approvals
- installer builds
- update-channel generation
- deployment behavior
- branch-protection simulation in code
- license or version normalization

## Branch protection

Branch protection is required before production/public release.

Branch protection must be configured in GitHub repository settings.

Branch protection must not be simulated in repository code.

Phase 186 did not change remote repository settings.

## Remote evidence

| Evidence command | Result |
| --- | --- |
| `gh release list --repo irgordon/ajentic` | Unknown; `gh` is not installed in the local shell. |
| `git ls-remote --tags origin` | No remote tags returned. |
| `gh api repos/irgordon/ajentic/branches/main/protection` | Unknown; `gh` is not installed in the local shell. |

## Future phase boundaries

| Phase | Responsibility |
| --- | --- |
| 187 | Versioning, license, and public identity alignment. |
| 188 | Reproducible artifact builds in GitHub Actions. |
| 189 | Checksums, SBOM, and provenance evidence. |
| 191 | Signing and key-custody activation boundary. |
| 200 | v1.0 release decision gate. |
| 201 | First possible v1.0 GitHub Actions release execution, only if Phase 200 approves. |

## Non-release statement

Phase 186 creates no GitHub tag, GitHub Release, release workflow, release artifact, public artifact, public package, signing key, signing behavior, publishing behavior, installer generation, update-channel behavior, package deployment, deployment path, production readiness approval, public-release readiness approval, public-use approval, or v1.0 approval.
