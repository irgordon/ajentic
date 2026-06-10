---
truth_dimension: orientation
authority_level: advisory
mutation_path: readme_update
---

# Phase 187 - Versioning, License, and Public Identity Alignment

## Scope

Phase 187 aligns versioning language, license posture, and public identity surfaces before artifact, checksum, provenance, signing, or release-publication work begins.

Phase 187 is not a release phase.

Phase 187 is not a production-readiness approval phase.

Phase 187 does not create public release mechanics.

## Current public status

Current status: Release Candidate stewardship / pre-v1.0 hardening.

AJENTIC is not production-ready.

AJENTIC is not public-release-ready.

AJENTIC is not v1.0-ready.

No v1.0 release has been approved.

No GitHub Release or release tag has been created by the governed release process.

GitHub Actions is the intended future release platform, but release execution remains blocked.

v1.0 execution remains blocked until Phase 200 approves and Phase 201 executes.

## Versioning policy

| Surface | Meaning | Release authority |
| --- | --- | --- |
| Phase ID | Human roadmap governance unit, such as Phase 187. | None. |
| Changelog marker | Internal evidence marker, such as `v0.0.187`. | Not a Git tag unless a later release-execution phase explicitly creates one. |
| Package/crate version | Cargo/npm/package metadata. | Not release approval, not production readiness, and not public-release approval. |
| Git tag | Release-control object. | Must not be created unless a later release-execution phase explicitly authorizes it. |
| GitHub Release | Public release/publication object. | Must not be created unless a later release-execution phase explicitly authorizes it. |
| Public product/version claim | User-facing readiness or release statement. | Must remain blocked until roadmap decision gates approve it. |

Passing validation does not approve release.

GitHub Actions validation does not approve release.

Package metadata does not approve release.

## Package version findings

| Surface | Finding | Phase 187 disposition |
| --- | --- | --- |
| Root `Cargo.toml` | Not present. | No action. |
| `core/Cargo.toml` | `ajentic-core` version is `0.1.0`. | Left unchanged; not v1.0 readiness. |
| `cli/Cargo.toml` | Not present. | No action. |
| Root `package.json` | Not present. | No action. |
| `ui/package.json` | `ajentic-ui` version is `0.1.0` and package is private. | Left unchanged; not public release approval. |

`0.1.0` package metadata is not equivalent to v1.0 readiness, public release approval, or production readiness.

Phase 187 does not normalize package versions because no public package/version strategy has been approved.

No package or crate version is set to `1.0.0`.

## License status

No `LICENSE`, `LICENSE.md`, or `COPYING` file is present.

No Owner-selected license decision exists in tracked project instructions inspected for Phase 187.

Phase 187 does not add a license file.

Phase 187 does not invent MIT, Apache-2.0, GPL, proprietary, commercial, or other licensing terms.

Without an Owner-selected license file, public reuse rights are not granted by this repository.

License selection remains a v1.0/public-release blocker.

## Workflow inspection

| Workflow | Trigger coverage | Permissions | Release authority |
| --- | --- | --- | --- |
| `.github/workflows/ci.yml` | `push` to `main`, `pull_request`, `workflow_dispatch` | `contents: read` | Validation only; no release execution. |
| `.github/workflows/memory-lint.yml` | `push` to `main`, `pull_request`, `workflow_dispatch` | `contents: read` | Validation only; no release execution. |

No `release.yml`, `publish.yml`, or `deploy.yml` workflow exists.

No release workflow was added in Phase 187.

## Remote evidence

| Evidence command | Result |
| --- | --- |
| `git ls-remote --tags origin` | No remote tags returned. |
| `gh release list --repo irgordon/ajentic` | Unknown; `gh` is not installed in the local shell. |
| `gh api repos/irgordon/ajentic/branches/main/protection` | Unknown; `gh` is not installed in the local shell. |

Phase 187 does not change remote repository settings.

## Future phase boundaries

| Phase | Responsibility |
| --- | --- |
| 188 | Reproducible artifact builds in GitHub Actions. |
| 189 | Checksums, SBOM, and provenance evidence. |
| 191 | Signing and key-custody activation boundary. |
| 200 | v1.0 release decision gate. |
| 201 | First possible v1.0 GitHub Actions release execution, only if Phase 200 approves. |

## Non-release statement

Phase 187 creates no Git tag, GitHub Release, release workflow, release artifact, public artifact, public package, license file, signing key, signing behavior, publishing behavior, installer generation, update-channel behavior, package deployment, deployment path, production readiness approval, public-release readiness approval, public-use approval, or v1.0 approval.
