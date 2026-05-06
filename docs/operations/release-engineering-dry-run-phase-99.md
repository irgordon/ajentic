---
truth_dimension: orientation
authority_level: advisory
mutation_path: readme_update
---

# Release Engineering Dry-Run Boundary - Phase 99

## Scope
Phase 99 is a release-engineering dry-run boundary only.

Phase 99 defines a local, evidence-only simulation for assembling release-engineering evidence at the current checkout. Phase 99 is documentation-only unless `checklists/release.md` requires posture clarification.

Phase 99 is a simulation, not a release. It is not release-candidate approval, not Production Candidate approval, not public-usability approval, not startup/package readiness approval, not installer approval, not signing approval, not distribution approval, not publishing approval, and not auto-update approval.

Phase 99 adds no runtime behavior, no new CLI, no release tooling, no installer, no distribution path, no signing path, no publishing path, no auto-update path, and no authority.

## Dry-run model
The Phase 99 model is a local release-engineering rehearsal.

It checks whether evidence can be collected and interpreted without creating release artifacts. It reviews command results, checklist completeness, changelog/version posture, validation-gate coverage, and artifact boundaries.

The dry run must stop at evidence assembly. If evidence identifies a release-engineering defect, Phase 99 documents the defect and recommends a later repair phase. Phase 99 does not repair scripts, workflows, source, package metadata, release publishing infrastructure, lint rules, or runtime behavior.

Roadmap remains planned truth. `CHANGELOG.md` remains historical truth.

## Evidence-only rule
Phase 99 dry-run evidence consists of local command results only.

The evidence commands are non-releasing. They may compile, test, lint, scan, typecheck, or build locally, but they must not tag, sign, publish, upload, distribute, package for release, install, or create update channels.

A command result is evidence only. It is not authority, not approval, and not release readiness.

## Release dry-run evidence set
The Phase 99 release dry-run evidence set consists of these local command results only:

| Evidence command or scan | Evidence purpose | Release posture |
| --- | --- | --- |
| `./scripts/check.sh` | Full local validation gate. | Evidence-only; non-releasing. |
| `cargo test --manifest-path core/Cargo.toml --all-targets` | Explicit Rust test evidence. | Evidence-only; non-releasing. |
| `cargo test --manifest-path core/Cargo.toml golden --all-targets` | Explicit golden invariant test evidence. | Evidence-only; non-releasing. |
| `cargo test --manifest-path core/Cargo.toml adversarial --all-targets` | Explicit adversarial corpus test evidence. | Evidence-only; non-releasing. |
| `cd ui && npm run test:api` | Explicit UI behavioral test evidence. | Evidence-only; non-releasing. |
| `node scripts/test_rust_boundary_lint.mjs` | Rust boundary lint self-test evidence. | Evidence-only; non-releasing. |
| `node scripts/rust_boundary_lint.mjs` | Rust boundary lint scan evidence. | Evidence-only; non-releasing. |
| `node scripts/test_lint_ui_boundaries.mjs` | UI AST boundary lint self-test evidence. | Evidence-only; non-releasing. |
| `node scripts/lint_ui_boundaries.mjs` | UI AST boundary lint scan evidence. | Evidence-only; non-releasing. |
| `cd ui && npm run typecheck && npm run lint && npm run build` | Explicit UI validation evidence. | Evidence-only; non-releasing. |
| `cargo run --manifest-path core/Cargo.toml -- dry-run` | Local startup boundary dry-run report evidence. | Evidence-only; non-releasing. |
| `cargo build --manifest-path core/Cargo.toml` | Local Rust build evidence. | Evidence-only; non-releasing. |
| `cd ui && npm run build` | Local UI build evidence. | Evidence-only; non-releasing. |
| `git status --short` | Working-tree and artifact hygiene evidence. | Evidence-only; non-releasing. |
| roadmap/changelog truth-surface scans | Planned-vs-historical truth alignment evidence. | Evidence-only; non-releasing. |
| readiness/non-approval scans | Non-readiness language evidence. | Evidence-only; non-releasing. |
| source/script/workflow/package guard scans | Surface-discipline evidence. | Evidence-only; non-releasing. |

## Command interpretation
Evidence commands are interpreted narrowly.

A passing validation, test, lint, typecheck, dry-run, build, or scan command means only that the specific local command returned acceptable evidence for its stated boundary at that moment.

A passing `cargo build --manifest-path core/Cargo.toml` or `cd ui && npm run build` may create local build output, but that output is not a release artifact, not an archive intended for distribution, not an installer, not a package-registry artifact, and not a public download asset. Generated build output must be treated as local tool output and removed or left untracked according to repository hygiene before commit.

`rg` scans are discovery/evidence only. Phase 99 does not rely on `rg` scans as enforcement. Blocking enforcement must come from `./scripts/check.sh`, Rust boundary lint, UI AST lint, compiler/type checks, clippy, UI behavioral tests, and Rust tests.

If any validation command prints assertion failure, panic, traceback, failed assertion, partial pass count, or masked failure while still returning 0, Phase 99 classifies that as a validation-gate defect and documents a required follow-up. Phase 99 does not repair that defect.

## Prohibited artifacts and actions
Phase 99 must not create:

- git tags;
- GitHub releases;
- release branches;
- uploaded artifacts;
- signed artifacts;
- checksums for release distribution;
- SBOMs for release distribution;
- installers;
- archives intended for distribution;
- Docker/container images;
- package-registry artifacts;
- npm packages;
- cargo packages;
- update channels;
- auto-update manifests;
- public download assets.

Phase 99 must not run or add behavior for `cargo publish`, `npm publish`, `docker build`, `docker push`, `gh release`, create-release actions, upload-artifact actions, release upload actions, `git tag`, `git push --tags`, codesign, notarization, package installer builders, SBOM release generation, cosign, SLSA publishing, auto-update behavior, or update channel management.

## What a dry-run pass means
A release dry-run pass means only that the release-evidence checklist can be assembled locally at that moment.

## What a dry-run pass does not mean
A release dry-run pass:

- does not create a release;
- does not approve a release candidate;
- does not approve Production Candidate status;
- does not approve public usability;
- does not approve startup/package readiness;
- does not approve installer behavior;
- does not approve signing behavior;
- does not approve distribution behavior;
- does not approve publishing behavior;
- does not approve auto-update behavior.

## Version and changelog posture
Phase 99 records completed accepted documentation work in `CHANGELOG.md` as historical truth.

The dry-run does not change package versions, dependency versions, lockfiles, release branches, tags, registry metadata, download metadata, update manifests, or public version channels.

Version/changelog review in Phase 99 means only that the changelog entry matches the final documentation diff and does not claim readiness or release approval.

## Release checklist posture
`checklists/release.md` remains a future release-candidate evidence checklist. Phase 99 dry-run evidence is not readiness approval.

Phase 99 does not mark release readiness, release-candidate readiness, production readiness, public usability, Production Candidate status, installer approval, signing approval, distribution approval, publishing approval, or auto-update approval as complete.

No change to `checklists/release.md` is required when the checklist already avoids readiness claims and does not imply that Phase 99 evidence approves release readiness.

## Validation-gate integrity posture
Validation-gate integrity requires the final post-edit validation commands to return 0 and to avoid masked failure output.

For Phase 99, the validation gate is integrity evidence only. It does not grant release authority and does not approve any release or Production Candidate state.

If validation-gate integrity fails, the dry run is incomplete and the failure must be recorded rather than converted into a release-engineering repair inside Phase 99.

## Artifact cleanup posture
Phase 99 starts and ends with working-tree hygiene.

Generated compiler metadata, UI build artifacts, Cargo build artifacts if tracked, target drift, test temp files, export temp files, coverage output, `scripts/__pycache__`, node build artifacts, and other generated artifact drift from prior or current tool runs must be removed or left untracked outside the commit.

The final staged files must match the allowed surfaces for Phase 99.

## Non-authority guarantees
Phase 99 does not add provider/model calls, persistence, durable append, export writes, replay repair, recovery acceptance, action execution, live transport, or authority.

Phase 99 does not mutate runtime authority. It does not start a server, daemon, watcher, socket, UI bridge, live transport, async runtime, background process, browser launch, provider adapter execution, operator action execution, persistence plan execution, local export write, recovery promotion, or replay repair.

Model output remains untrusted until validated through Rust-owned paths.

## Relationship to Phase 96 local startup boundary
Phase 96 defines the existing local startup command boundary using `cargo run --manifest-path core/Cargo.toml -- dry-run`.

Phase 99 may collect that command result as release dry-run evidence, but Phase 99 does not change the Phase 96 startup boundary and does not approve startup/package readiness.

## Relationship to Phase 97 packaging boundary
Phase 97 defines local packaging artifact boundaries and exclusions.

Phase 99 may review artifact posture as evidence, but it does not create packages, installers, distribution archives, signed artifacts, checksums for release distribution, SBOMs for release distribution, package-registry artifacts, or public download assets.

## Relationship to Phase 98 operator troubleshooting
Phase 98 documents local operator troubleshooting and command interpretation.

Phase 99 uses that posture for evidence interpretation. It does not add troubleshooting runtime behavior, release tooling, scripts, workflows, or public usability approval.

## Relationship to Phase 100 Production Candidate decision gate
Phase 100 remains responsible for the final Production Candidate gap audit and readiness decision gate.

Phase 99 does not make the Phase 100 decision. Phase 99 does not approve a production-candidate branch, production-candidate tag, release candidate, public usability, production readiness, release readiness, or Production Candidate status.

## AST/boundary lint parity
Phase 99 keeps AST/boundary lint parity as an evidence boundary.

Do not rely on `rg` scans as enforcement. `rg` scans are discovery/evidence only.

Blocking enforcement must come from `./scripts/check.sh`, Rust boundary lint, UI AST lint, compiler/type checks, clippy, UI behavioral tests, and Rust tests.

If Phase 99 finds a new source/tooling pattern that should be prohibited later, it must document the pattern as a deferred maintenance item. It must not patch lint behavior inside Phase 99. If lint behavior must change, lint self-tests must be added or updated in a separate maintenance phase, not Phase 99.

## Test fidelity
Phase 99 is dry-run/documentation-only, so no new Rust or TypeScript tests are expected.

Full existing test/lint/check suite must pass after final documentation edits. If tests are skipped after final edits, Phase 99 is not complete.

If any validation command prints failure output while still returning 0, Phase 99 treats that as a validation-gate defect and records a follow-up instead of repairing it.

## Validation evidence
Phase 99 validation evidence is the local command result set listed in this document and in `checklists/current-phase.md`.

The final validation pass must include `./scripts/check.sh`, explicit Rust tests, explicit golden/adversarial filters, explicit UI behavior tests, explicit boundary lints, explicit UI validation, CLI dry-run, local builds, evidence scans, interpretation scans, prohibited-artifact scans, no-release-action scans, no-runtime/no-authority scans, source/script/workflow/package guard scans, readiness scans, roadmap/changelog scans, roadmap completion contamination scans, changelog future-planning contamination scans, lint wiring scans, `git status --short`, and staged-file review.

## Confirmed vs suspected
Confirmed findings are limited to observed local command output, final diffs, and final working-tree state.

Suspected release-engineering defects, release-readiness gaps, or tooling improvements are not fixed in Phase 99. They are documented as deferred items only when observed evidence supports them.

## Deferred items
No release-engineering repair is implemented in Phase 99.

Any future release tooling, release checklist restructuring, CI gate changes, package metadata changes, signing setup, distribution setup, installer setup, update-channel setup, SBOM release generation, checksum distribution, or lint-rule expansion remains outside Phase 99.

## Non-readiness statement
Phase 99 is not a release.

Phase 99 does not approve release-candidate readiness, Production Candidate status, production readiness, public usability, startup/package readiness, installer behavior, signing behavior, distribution behavior, publishing behavior, auto-update behavior, provider/model execution, persistence authority, replay repair, recovery promotion, export writes, action execution, live transport, or any new authority.
