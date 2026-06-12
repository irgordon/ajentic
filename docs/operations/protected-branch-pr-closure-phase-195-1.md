---
truth_dimension: orientation
authority_level: advisory
mutation_path: readme_update
---

# Phase 195.1 - Protected-Branch PR Closure and Solo-Maintainer Merge Evidence

## Decision

Phase 195.1 records protected-branch PR closure evidence for PR #297.

This is a process-evidence closure phase. It is not a feature phase and not a release execution phase.

## PR Closure Evidence

- PR: https://github.com/irgordon/ajentic/pull/297
- PR branch: `mothra/phase-195-release-closure`
- PR branch head before merge: `9496866300cc3e8b1c881076c3d5e8c059b320c2`
- Mainline squash merge commit: `d154e06789e46bb6485323ef9b02020c31a992b8`
- Mainline commit message: `docs(release): Close v1 release evidence (#297)`
- Merge method: squash merge

The PR branch head may not appear directly in `main` history because squash merge was used.

## Required Checks

All required checks passed before merge:

- Full deterministic check
- TypeScript UI
- Schemas
- Scripts
- Rust
- Repository validation
- Internal candidate artifact reproducibility
- Memory Placement Gate

Direct push protection worked. Direct push to `main` was blocked, and Phase 195 entered the protected PR flow.

## Solo-Maintainer Exception

Phase 195.1 solo-maintainer admin-bypass exception:

PR #297 was documentation/status closure work after the v1.0.0 final release.
The repository is solo-maintained.
The branch protection required one approving review from a reviewer with write access.
The PR author could not approve their own PR.
All required checks passed.
No release mechanics or authority changes were included.
Admin bypass was used to complete the squash merge.
This exception is recorded as a process exception only and does not weaken AJENTIC release authority, runtime authority, or post-v1 restrictions.

This was not a final-release execution exception.

## Branch Protection

Branch protection remains active on `main`.

Required checks remain configured:

- Full deterministic check
- TypeScript UI
- Schemas
- Scripts
- Rust
- Repository validation
- Internal candidate artifact reproducibility
- Memory Placement Gate

Future solo-maintainer branch protection should avoid mandatory independent approval unless a second eligible reviewer exists.

A post-v1 governance task should decide whether to keep admin bypass available for documentation/status PRs, remove required approvals for solo-maintainer operation, add a second eligible maintainer/reviewer, or keep the current stricter rule and accept future deadlocks.

## Preserved Release Boundaries

Phase 195.1 did not create any new tag.

Phase 195.1 did not create any new GitHub Release.

Phase 195.1 did not publish packages.

Phase 195.1 did not create an installer.

Phase 195.1 did not create an update channel.

Phase 195.1 did not deploy anything.

Phase 195.1 did not add OS signing.

Phase 195.1 did not add notarization.

Phase 195.1 did not change backend authority.

Phase 195.1 did not change UI authority.

Remote tags and releases remain limited to:

- `v1.0.0`
- `v1.0.0-rc.1`

Release evidence, docs, workflows, UI, and scripts do not become authority.

Rust remains the authoritative control layer.
