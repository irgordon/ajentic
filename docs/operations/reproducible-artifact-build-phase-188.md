---
truth_dimension: orientation
authority_level: advisory
mutation_path: readme_update
---

# Phase 188 - Reproducible Artifact Build in Actions

## Scope

Phase 188 adds internal reproducible artifact build validation in GitHub Actions.

This is a build-evidence phase. It is not a release phase, not a production-readiness approval phase, and not a public-release approval phase.

Phase 188 does not create release authority, signing authority, publishing authority, deployment authority, installer authority, update-channel authority, GitHub Release authority, or release-tag authority.

## Workflow contract

`.github/workflows/reproducible-artifacts.yml` runs on:

- pull requests
- pushes to `main`
- manual `workflow_dispatch`

The workflow uses least-privilege permissions:

```yaml
permissions:
  contents: read
```

The workflow performs internal validation only. It does not upload artifacts, publish packages, sign outputs, create tags, create GitHub Releases, deploy, create installers, or create update channels.

## Candidate artifact surfaces

Phase 188 validates reproducibility for internal candidate build outputs:

| Surface | Build command | Candidate output |
| --- | --- | --- |
| Rust runtime | `cargo build --manifest-path core/Cargo.toml --release --locked` | Release executable files from the Rust release target directory. |
| Browser UI | `npm ci` and `npm run build` in `ui/` | Files under `ui/dist`. |

These outputs are internal reproducibility candidates only. They are not release artifacts and are not public artifacts.

## Reproducibility method

`scripts/reproducible_artifacts.py --check` creates two clean temporary source copies from the same committed source tree.

Each copy is built with controlled reproducibility environment values:

- `SOURCE_DATE_EPOCH`
- `TZ=UTC`
- `LC_ALL=C`
- `CARGO_INCREMENTAL=0`

The checker records a normalized manifest for each build. Each manifest entry contains:

- candidate category
- normalized relative path
- byte size
- SHA-256 digest

The SHA-256 digests are internal reproducibility comparison fields only. They are not formal release checksums, release provenance, or publication evidence.

The check passes only when the two normalized manifests are identical.

## Checksums, SBOM, and provenance boundary

Phase 188 does not add formal release checksums.

Phase 188 does not add SBOM generation.

Phase 188 does not add provenance publication.

Checksums, SBOM, and provenance evidence remain assigned to Phase 189.

## Workflow inspection

| Workflow | Trigger coverage | Permissions | Release authority |
| --- | --- | --- | --- |
| `.github/workflows/ci.yml` | `push` to `main`, `pull_request`, `workflow_dispatch` | `contents: read` | Validation only; no release execution. |
| `.github/workflows/memory-lint.yml` | `push` to `main`, `pull_request`, `workflow_dispatch` | `contents: read` | Validation only; no release execution. |
| `.github/workflows/reproducible-artifacts.yml` | `push` to `main`, `pull_request`, `workflow_dispatch` | `contents: read` | Reproducibility validation only; no release execution. |

No `release.yml`, `publish.yml`, or `deploy.yml` workflow exists.

No workflow has write permissions.

## Remote evidence

| Evidence command | Result |
| --- | --- |
| `git ls-remote --tags origin` | No remote tags returned. |
| `gh release list --repo irgordon/ajentic` | Unknown; `gh` is not installed in the local shell. |
| `gh api repos/irgordon/ajentic/branches/main/protection` | Unknown; `gh` is not installed in the local shell. |

Remote GitHub Actions evidence remains pending until this branch is pushed and Actions runs on GitHub.

## Future phase boundaries

| Phase | Responsibility |
| --- | --- |
| 189 | Checksums, SBOM, and provenance evidence. |
| 191 | Signing and key-custody activation boundary. |
| 200 | v1.0 release decision gate. |
| 201 | First possible v1.0 GitHub Actions release execution, only if Phase 200 approves. |

## Non-release statement

Phase 188 creates no Git tag, GitHub Release, public artifact upload, public package, formal release checksum, SBOM, provenance publication, signing key, signing behavior, publishing behavior, installer generation, update-channel behavior, package deployment, deployment path, production readiness approval, public-release readiness approval, public-use approval, or v1.0 approval.
