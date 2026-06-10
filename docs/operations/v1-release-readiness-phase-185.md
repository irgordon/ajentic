---
truth_dimension: orientation
authority_level: advisory
mutation_path: readme_update
---

# Phase 185 - Release Stewardship Checkpoint and v1.0 Gap Registration

## Scope

Phase 185 records release-stewardship status after Phase 184.2 and registers the remaining v1.0 release-platform gaps.

This document is advisory operations evidence. It does not create release authority, production authority, public-use authority, signing authority, publishing authority, deployment authority, installer authority, update-channel authority, GitHub Release authority, or release-tag authority.

## Current status

Release Candidate stewardship may continue with caveats.

AJENTIC is not production-ready.

AJENTIC is not public-release-ready.

AJENTIC is not v1.0-ready.

Passing validation is necessary evidence, not release approval.

## Evidence rule

Count committed repository evidence, direct local validation output, and GitHub Actions evidence for the relevant PR head commit. Do not count prompt intent, local validation alone, clean scans alone, README status wording, absence of blockers, or roadmap continuation as release approval.

## Phase 184.2 carry-forward

Phase 184.2 recorded `release_path_can_resume_after_heuristic_triage` and found no rebuild trigger. That allows Release Candidate stewardship to continue with caveats. It does not approve production readiness, public-release readiness, v1.0 readiness, release artifact creation, signing, publishing, deployment, installer activation, update-channel activation, GitHub Release creation, release tag creation, or public distribution.

## Release-platform blocker inventory

| Blocker | Finding | Disposition |
| --- | --- | --- |
| GitHub Releases | No GitHub Releases exist. | Block v1.0 execution until release-platform phases define and approve release execution. |
| Git tags | No Git tags exist. | Block v1.0 execution until release-tag policy is defined and Phase 200 approves. |
| Main branch protection | `main` branch is not protected in repository metadata. | Repository settings requirement; do not simulate in code. |
| CI trigger model | Current workflows run on pull request and `workflow_dispatch`, not push to main. | Defer trigger policy to the release-platform block. |
| Release workflow | No release workflow exists in `.github/workflows`. | Defer to GitHub Actions Release Platform Contract and later execution phases. |
| README status | README now states Release Candidate stewardship / pre-v1.0 hardening. | Status language corrected without readiness approval. |
| Version alignment | `core/Cargo.toml` and `ui/package.json` use `0.1.0`; changelog history uses phase versions such as `v0.0.184.2`. | Defer version policy and alignment to Phase 187. |
| License | No `LICENSE`, `LICENSE.md`, or `COPYING` file exists. | Defer owner-selected license and public identity alignment to Phase 187. |
| Signing | No real signing path is active. | Defer to Phase 191 after platform contract and provenance evidence. |
| Publishing | No publishing path is active. | Defer to release-platform phases. |
| Installer/update channel | No installer or update-channel path is active. | Defer or explicitly exclude in release-platform phases. |
| Public artifacts/deployment | No public artifact or deployment release path is active. | Defer to future release decision gates. |
| Maintainability debt | Prior audits identify oversized release-adjacent modules and string-heuristic debt. | Carry into hardening and final release audit phases. |

## GitHub Actions release platform

GitHub Actions is the intended release platform.

GitHub Actions release authority is not yet formalized.

Existing workflow files:

| Workflow | Trigger model | Release authority |
| --- | --- | --- |
| `.github/workflows/ci.yml` | `pull_request` and `workflow_dispatch` | Validation only; no push-to-main trigger and no release execution. |
| `.github/workflows/memory-lint.yml` | `pull_request` and `workflow_dispatch` | Memory placement validation only; no release execution. |

No release workflow was added in Phase 185.

No workflow trigger was changed in Phase 185.

## Version inspection

| Surface | Observed version/status |
| --- | --- |
| `core/Cargo.toml` | `0.1.0` |
| `ui/package.json` | `0.1.0` |
| `CHANGELOG.md` | Phase-version entries, latest `v0.0.185` |
| Root `Cargo.toml` | Not present |
| Root `package.json` | Not present |
| `cli/Cargo.toml` | Not present |

Version alignment remains a v1.0 blocker. Phase 185 does not normalize versions because no repository versioning policy defines the exact mapping between package versions, phase versions, Release Candidate labels, and v1.0 release identifiers.

## License inspection

No `LICENSE`, `LICENSE.md`, or `COPYING` file exists.

Phase 185 does not add a license because the owner has not selected one in repository evidence. License and public identity alignment are deferred to Phase 187.

## Branch protection

Main branch protection is a repository settings requirement.

Phase 185 does not change branch protection and does not simulate branch protection in code.

## Required future release-platform block

The v1.0 release-platform block is:

| Phase | Title | Purpose |
| --- | --- | --- |
| 186 | GitHub Actions Release Platform Contract | Define GitHub Actions release authority, triggers, permissions, environments, and prohibited release bypasses. |
| 187 | Versioning, License, and Public Identity Alignment | Align package versions, changelog versioning, license, README status, and public identity without release execution. |
| 188 | Reproducible Artifact Build in Actions | Add reproducible build evidence in Actions without public release publication. |
| 189 | Checksums, SBOM, and Provenance Evidence | Add checksum, SBOM, and provenance evidence boundaries without signing or publishing. |
| 190 | Release Platform Alignment Checkpoint | Reconcile platform contract, version/license status, reproducible builds, and provenance evidence. |
| 191 | Signing and Key-Custody Activation Boundary | Define or activate bounded signing/key-custody behavior only after prerequisite evidence. |
| 192 | GitHub Draft Release / RC Publication Rehearsal | Rehearse draft release mechanics without v1.0 approval or public final release. |
| 193 | Support, Incident, Rollback, and Security Closure | Close support, incident, rollback, and security evidence before production/public-use gates. |
| 194 | Production/Public-Use Final Hardening | Resolve confirmed production and public-use blockers. |
| 195 | Production Candidate Decision Gate | Decide whether Production Candidate status is supportable. |
| 196 | v1.0 Readiness Evidence Consolidation | Consolidate final v1.0 evidence without approval. |
| 197 | v1.0 Documentation and Public Identity Closure | Close public documentation and identity evidence. |
| 198 | Final Release Audit | Audit release, security, support, versioning, provenance, and public-use evidence. |
| 199 | Final Release Blocker Resolution | Resolve final blockers identified by the final audit. |
| 200 | v1.0 Release Decision Gate | Decide whether v1.0 execution is approved. |
| 201 | v1.0 GitHub Actions Release Execution | Execute v1.0 GitHub Actions release only if Phase 200 approves. |

## Release execution gates

v1.0 execution is blocked until Phase 200 approves.

Phase 201 may execute the v1.0 GitHub Actions release only if Phase 200 approves.

No earlier phase may infer v1.0 approval from validation success, clean scans, absence of blockers, Release Candidate stewardship, GitHub Actions availability, package version strings, README status, or draft release rehearsal.

## Non-release statement

Phase 185 creates no GitHub tag, GitHub Release, release artifact, public artifact, public package, signing key, signing behavior, publishing behavior, installer generation, update-channel behavior, package deployment, deployment path, production readiness approval, public-release readiness approval, public-use approval, or v1.0 approval.

## Validation output

Phase 185 validation executed:

- `git status --short`
- `cd ui && npm run build`
- `cd ui && rm -rf dist`
- `cd ui && npm run typecheck`
- `cd ui && npm run lint`
- `cd ui && npm run test:api`
- `cargo fmt --manifest-path core/Cargo.toml -- --check`
- `CARGO_TARGET_DIR=/tmp/ajentic-phase-185-target cargo test --manifest-path core/Cargo.toml --all-targets`
- `CARGO_TARGET_DIR=/tmp/ajentic-phase-185-target cargo clippy --manifest-path core/Cargo.toml --all-targets -- -D warnings`
- `PYTHONDONTWRITEBYTECODE=1 python3 scripts/validate_structure.py`
- `PYTHONDONTWRITEBYTECODE=1 python3 scripts/validate_docs.py`
- `node scripts/test_rust_boundary_lint.mjs`
- `node scripts/rust_boundary_lint.mjs`
- `PYTHONDONTWRITEBYTECODE=1 python3 scripts/check_help_pages.py`
- `git diff --check`
- `CARGO_TARGET_DIR=/tmp/ajentic-phase-185-target ./scripts/check.sh`

## Zero-drift statement

Phase 185 changes documentation, roadmap, checklist, README, AGENTS navigation, changelog, and `.gitignore` surfaces only. It does not change Rust runtime behavior, TypeScript UI behavior, schemas, workflows, release infrastructure, signing behavior, publishing behavior, installer behavior, update-channel behavior, deployment behavior, provider execution behavior, replay repair behavior, recovery promotion behavior, or action authorization behavior.
