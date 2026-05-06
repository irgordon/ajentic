---
truth_dimension: orientation
authority_level: advisory
mutation_path: readme_update
---

# Packaging Boundary - Phase 97

## Scope
Phase 97 is a packaging boundary candidate only.

Roadmap planned truth identifies Phase 97 as **Packaging Artifact Definition** with the goal to define which local artifacts are built, named, versioned, and excluded. This operations note uses the narrower operator wording **packaging boundary candidate** to keep the phase explicitly non-release and non-authoritative.

Phase 97 is not production readiness, release-candidate readiness, public usability approval, installer approval, distribution approval, auto-update approval, or Production Candidate approval.

Phase 97 may document a local package/build artifact boundary for operator testing only. It does not add runtime behavior.

## Packaging boundary decision
**Decision: Option B - existing build artifact boundary using existing commands only.**

The repository already has local build commands that can create local build outputs for operator testing:

- `cargo build --manifest-path core/Cargo.toml`
- `cd ui && npm run build`

Phase 97 defines only the posture of those existing local build outputs. It adds no package/build behavior, no metadata change, no dependency change, no lockfile change, no release infrastructure, and no runtime behavior.

Option A was rejected because the roadmap scope asks Phase 97 to define local artifacts that are built, named, versioned, and excluded, and existing build commands already produce local operator-test outputs without new behavior. Option C was unnecessary because current package metadata was not misleading in a way that blocked operator testing.

## Supported local build/package commands
The supported Phase 97 local build/package commands are:

```sh
cargo build --manifest-path core/Cargo.toml
```

```sh
cd ui && npm run build
```

These are local only and operator-test only. They are not signed, not installer commands, not distribution commands, not release candidate commands, not production-ready commands, not public usability approval commands, not auto-updating commands, not service or daemon registration commands, not provider execution commands, not persistence authority commands, and not action execution commands.

No other Phase 97 packaging command is approved. In particular, Phase 97 does not approve `cargo publish`, `npm publish`, Docker image creation, installer creation, signing, notarization, release upload, public download, or update-channel creation.

## What these commands do
`cargo build --manifest-path core/Cargo.toml` compiles the Rust crate through Cargo using the existing manifest. Its local build output is a Cargo build output under the local target directory. The command validates that the Rust build surface can compile for the local checkout.

`cd ui && npm run build` runs the existing UI build script from `ui/package.json`. Its local build output is the UI build output produced by the existing Vite build path. The command validates that the browser UI source can be built locally for the checkout.

Both commands are local build checks for operator testing. They may create local generated build output in ignored build directories, and that output must not be committed as Phase 97 evidence.

## What happens when they fail
If either command exits non-zero, the Phase 97 local packaging/build boundary is not satisfied for that checkout.

A failure means the local build command did not complete successfully. It does not imply provider failure, model failure, persistence failure, release failure, installer failure, distribution failure, public usability failure, auto-update failure, or Production Candidate failure, because Phase 97 does not start those surfaces.

The expected operator response is to inspect the failing command output, rerun the repository validation gates after any fix, remove generated artifact drift before commit, and keep the phase in a non-ready state until validation passes.

## What these commands do not do
These commands do not:

- approve production readiness;
- approve release-candidate readiness;
- approve public usability;
- approve installer behavior;
- approve distribution;
- approve auto-update behavior;
- create signed release artifacts;
- create installer artifacts;
- create update channels;
- publish to package registries;
- publish a GitHub Release;
- create Docker or container packaging;
- start a server;
- launch a browser;
- install services;
- register a daemon;
- modify the user environment;
- call providers or models;
- write persistence;
- append ledger or audit records;
- write exports;
- import data;
- repair replay;
- promote recovery;
- replace global state;
- execute operator actions;
- add runtime authority.

## Artifact posture
Phase 97 artifact posture is local only and operator-test only.

The Rust build output and UI build output are candidate local build artifacts only. They are not signed, not installers, not distribution artifacts, not release artifacts, not release-candidate artifacts, not production-ready artifacts, not public-usability-approved artifacts, not auto-updating artifacts, not service or daemon artifacts, not provider-execution artifacts, not persistence-authority artifacts, and not action-execution artifacts.

Generated build outputs remain untracked local artifacts. They must be cleaned or left ignored and must not be staged for Phase 97.

## Operator expectations
Operators may use the supported commands to verify that the current checkout can produce local build outputs for operator testing.

A passing local build command means only that the existing build command completed for the local checkout. It does not mean the application is packaged for users, ready for release engineering, ready for production, approved for public use, approved for installer distribution, approved for auto-update, or approved as a Production Candidate.

Operators should run the full validation suite before relying on Phase 97 evidence.

## Non-authority guarantees
Packaging must not add runtime authority.

Phase 97 does not add runtime behavior, provider calls, model calls, persistence writes, export writes, replay repair, recovery promotion, action execution, server behavior, daemon behavior, live transport, browser launch, service registration, user-environment mutation, installer behavior, distribution behavior, signing behavior, release publishing, or auto-update behavior.

A package/build artifact is not evidence of production readiness, release-candidate readiness, public usability, installer approval, distribution approval, auto-update approval, or Production Candidate approval.

## Relationship to Phase 96 local startup boundary
Phase 96 defined the local startup command boundary around the existing dry-run command and did not approve packaging.

Phase 97 follows that boundary by defining only local build artifact posture for operator testing. It does not alter the Phase 96 dry-run command, does not start a server, does not launch a UI, does not create live UI/Rust transport, and does not convert startup evidence into package, release, public-usability, or production approval.

## Relationship to Phase 98 release dry-run boundary
The task boundary names Phase 98 as responsible for release dry-run boundary work if Phase 97 evidence permits it. Roadmap planned truth currently names Phase 98 as the Operator Documentation and Troubleshooting Guide and Phase 99 as the Release Engineering Dry Run. Phase 97 does not start either later phase.

Phase 97 does not perform release dry-run mechanics, artifact inventory approval, signing, installer generation, release upload, registry publishing, public distribution, operator documentation expansion for Phase 98, or Production Candidate approval.

Roadmap remains planned truth. `CHANGELOG.md` remains historical truth.

## Validation evidence
Phase 97 validation requires the full repository gate, explicit Rust tests, explicit golden and adversarial filters, UI behavioral tests, boundary lint self-tests and lints, UI validation, CLI dry-run, the documented local build commands, package-boundary scans, no-release-publishing scans, no-live-behavior scans, no-authority scans, source guard, readiness scan, roadmap/changelog scans, lint wiring scan, clean working-tree review, staged-file review, artifact cleanup, and final commit review.

Because Phase 97 changes documentation only and reuses existing build commands, no new runtime tests were required.

## AST/boundary lint parity
`rg` scans are discovery/evidence only and are not enforcement.

Blocking enforcement remains in `scripts/check.sh`, Rust boundary lint, UI AST lint, compiler/type checks, clippy, UI behavioral tests, and Rust tests.

Phase 97 did not identify a new source pattern requiring lint maintenance. No lint behavior changed, so no lint self-tests were added or updated.

## Test fidelity
No runtime or build behavior changed in Phase 97, so no new tests were required.

The existing full validation suite still had to pass after final edits. No phase is complete if validation is skipped after final edits. If a future phase adds a new packaging/build command, that phase must validate the command and add tests if behavior changes.

## Confirmed vs suspected
| Item | Status | Evidence |
| --- | --- | --- |
| Phase 97 title/scope | Confirmed | Roadmap planned truth identifies Phase 97 as Packaging Artifact Definition with local artifacts built, named, versioned, and excluded; boundary is packaging definition only with no distribution/release approval. |
| Packaging decision | Confirmed | Option B selected using existing commands only. |
| Build behavior changes | Confirmed absent | No package/build behavior, dependency, lockfile, runtime, script, workflow, README, Rust source, TypeScript source, test, schema, governance, architecture, roadmap, installer, signing, publishing, or auto-update changes were made. |
| Artifact posture | Confirmed | Local build outputs are operator-test only and not signed, not installers, not distribution artifacts, not release-candidate artifacts, not production-ready artifacts, and not public-usability-approved artifacts. |
| Runtime authority | Confirmed absent | Phase 97 documentation adds no provider/model calls, persistence writes, durable appends, export writes, replay repair, recovery promotion, action execution, server behavior, daemon behavior, browser launch, live transport, service registration, or user-environment mutation. |
| Roadmap/changelog roles | Confirmed | Roadmap remains planned truth. `CHANGELOG.md` remains historical truth. |

### Suspected
None.

## Deferred items
| Item | Reason |
| --- | --- |
| Installer, signing, notarization, release upload, registry publishing, public download, and update channels | Out of scope; Phase 97 does not approve installer/distribution/release/update behavior. |
| Release dry-run mechanics | Deferred to later planned release dry-run boundary work if evidence permits it. |
| Production Candidate readiness decision | Deferred to the planned decision gate; Phase 97 is not a readiness approval. |
| New package metadata or script naming | Not required because existing commands are sufficient for local operator-test build posture. |
| Lint maintenance phase | Not required because Phase 97 found no new source pattern requiring lint changes. |

## Non-readiness statement
Phase 97 is a packaging boundary candidate only.

Phase 97 does not approve production readiness, release-candidate readiness, public usability, installer approval, distribution approval, auto-update approval, package approval, release approval, startup approval, or Production Candidate approval.
