---
truth_dimension: orientation
authority_level: advisory
mutation_path: readme_update
---

# Phase 189 - Checksums, SBOM, and Provenance Evidence

## Scope

Phase 189 adds internal checksum, SBOM, and provenance evidence for the Phase 188 internal candidate artifact surfaces.

This is an integrity-evidence phase. It is not a release phase, signing phase, publishing phase, deployment phase, production-readiness approval phase, or public-release approval phase.

Phase 189 does not create public release artifacts.

## Evidence surfaces

Phase 189 evidence covers the same internal candidate artifact surfaces as Phase 188:

| Surface | Scope |
| --- | --- |
| Rust runtime | Release executable internal candidates from `core/Cargo.toml`. |
| Browser UI | Internal candidate files under `ui/dist`. |

The evidence files are internal review evidence only. They are not release artifacts, public distribution artifacts, signed attestations, or release approval.

## Checksum evidence

`scripts/reproducible_artifacts.py --check-evidence` generates internal checksum evidence.

Each checksum entry records:

- category
- relative path
- byte size
- SHA-256 digest

Checksum metadata records:

- evidence version
- repository
- commit SHA
- `SOURCE_DATE_EPOCH`
- generator
- artifact scope: `internal_candidate`
- release status: `not_release_artifact`

Checksums are internal integrity evidence. They are not release checksums and are not attached to a GitHub Release.

## SBOM evidence

Phase 189 generates deterministic internal SBOM evidence from:

- `core/Cargo.toml`
- `core/Cargo.lock`
- `ui/package.json`
- `ui/package-lock.json`

The SBOM evidence format is `ajentic-internal-sbom-json` with status `internal_not_standards_complete`.

It is internal review evidence, not public SBOM publication and not certified standards-complete SPDX output.

Unknown dependency metadata remains `UNKNOWN` or `null`; third-party licenses are not inferred as MIT.

Standards-complete SBOM generation may be hardened in a later release phase.

## Provenance evidence

Phase 189 generates unsigned internal provenance JSON with format `ajentic-internal-provenance` and status `internal_unsigned`.

The provenance records:

- repository
- commit SHA
- branch
- normalized GitHub Actions fields when present
- build command summary
- artifact surfaces
- source manifest digests
- `SOURCE_DATE_EPOCH`
- release status: `not_release_artifact`
- signing status: `unsigned`
- attestation status: `not_attested`
- publication status: `not_published`

The provenance is not a signed attestation, not a GitHub artifact attestation, not Sigstore signing, not SLSA compliance, and not release provenance.

## Determinism

The evidence checker builds two clean temporary source copies from the same committed source tree.

Each copy produces:

- internal checksum evidence
- internal SBOM evidence
- internal unsigned provenance evidence

The evidence is serialized canonically and compared deterministically. Differences fail closed with concise mismatch diagnostics.

Generated evidence is written only under the temporary Phase 189 evidence directory during validation and is not committed.

## Workflow contract

`.github/workflows/reproducible-artifacts.yml` remains validation-only and runs on:

- pull requests
- pushes to `main`
- manual `workflow_dispatch`

Workflow permissions remain:

```yaml
permissions:
  contents: read
```

No `id-token: write` permission is introduced.

No `attestations: write` permission is introduced.

No `contents: write` permission is introduced.

No GitHub artifact attestation is generated.

No artifact upload is used in Phase 189.

## Workflow inspection

| Workflow | Trigger coverage | Permissions | Release authority |
| --- | --- | --- | --- |
| `.github/workflows/ci.yml` | `push` to `main`, `pull_request`, `workflow_dispatch` | `contents: read` | Validation only; no release execution. |
| `.github/workflows/memory-lint.yml` | `push` to `main`, `pull_request`, `workflow_dispatch` | `contents: read` | Validation only; no release execution. |
| `.github/workflows/reproducible-artifacts.yml` | `push` to `main`, `pull_request`, `workflow_dispatch` | `contents: read` | Reproducibility and internal integrity-evidence validation only; no release execution. |

No `release.yml`, `publish.yml`, or `deploy.yml` workflow exists.

No workflow has `id-token: write`, `attestations: write`, or `contents: write`.

## Remote evidence

| Evidence command | Result |
| --- | --- |
| `git ls-remote --tags origin` | No remote tags returned. |
| `gh release list --repo irgordon/ajentic` | Unknown; `gh` is not installed in the local shell. |
| `gh api repos/irgordon/ajentic/branches/main/protection` | Unknown; `gh` is not installed in the local shell. |

Remote GitHub Actions evidence remains pending until GitHub Actions runs on GitHub for this commit.

## Future phase boundaries

| Phase | Responsibility |
| --- | --- |
| 190 | Release platform alignment checkpoint. |
| 191 | Signing and key-custody activation boundary. |
| 192 | GitHub draft release / RC publication rehearsal. |
| 200 | v1.0 release decision gate. |
| 201 | First possible v1.0 GitHub Actions release execution, only if Phase 200 approves. |

Release execution remains blocked until Phase 200 approves and Phase 201 executes.

## Non-release statement

Phase 189 creates no Git tag, GitHub Release, public artifact upload, public package, signed attestation, GitHub artifact attestation, signing key, signing behavior, publishing behavior, installer generation, update-channel behavior, package deployment, deployment path, production readiness approval, public-release readiness approval, public-use approval, or v1.0 approval.
