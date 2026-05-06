---
truth_dimension: procedural
authority_level: authoritative
mutation_path: checklist_revision
---

# Current phase checklist - Phase 98 Operator Documentation and Troubleshooting Boundary

## Phase name
Phase 98 - Operator Documentation and Troubleshooting Guide / Operator Documentation and Troubleshooting Boundary.

## Phase goal
Document how an operator should run, interpret, and troubleshoot the existing local validation, dry-run, UI behavioral, golden invariant, adversarial corpus, build, and packaging-boundary commands.

Roadmap planned truth names Phase 98 **Operator Documentation and Troubleshooting Guide** and scopes it to documenting startup, validation, expected outputs, failure modes, and rollback expectations for local operators. This checklist records Phase 98 procedural truth as documentation-only, operator-facing only, and introducing no new capability.

## Working-tree hygiene gate
- [x] Ran `git status --short` before edits.
- [x] Classified the initial working tree as clean.
- [x] Found no pre-existing uncommitted generated artifact drift requiring cleanup before edits.
- [x] Recorded cleanup: no cleanup was required before edits.
- [x] Reviewed generated artifact hygiene after validation.
- [x] Removed post-validation generated artifact drift: reverted `core/target/.rustc_info.json` and removed `scripts/__pycache__`.
- [x] Confirmed no generated compiler metadata, UI build artifacts, test temp files, export temp files, node build artifacts, coverage output, `scripts/__pycache__`, or unrelated tool output were staged.

## Allowed surfaces
- [x] `docs/operations/operator-troubleshooting-phase-98.md` was created.
- [x] `checklists/current-phase.md` was updated for Phase 98 procedural truth.
- [x] `CHANGELOG.md` was updated with `v0.0.98` historical truth.
- [x] `docs/operations/local-workflow-guide.md` was not changed because the Phase 98 operations doc was sufficient.
- [x] `checklists/release.md` was not changed because release evidence posture did not change.
- [x] Rust source was not changed.
- [x] TypeScript source was not changed.
- [x] Tests were not changed.
- [x] Scripts were not changed.
- [x] Workflows were not changed.
- [x] Schemas were not changed.
- [x] Governance docs were not changed.
- [x] Architecture docs were not changed.
- [x] `README.md` was not changed.
- [x] `AGENTS.md` was not changed.
- [x] Package files, dependency files, lockfiles, UI config, and roadmap docs were not changed.

## Boundary rules
- [x] Phase 98 is documentation-only.
- [x] Phase 98 is operator-facing only.
- [x] Phase 98 introduces no new capability.
- [x] Phase 98 adds no runtime behavior.
- [x] Phase 98 adds no new CLI surface.
- [x] Phase 98 adds no packaging.
- [x] Phase 98 adds no installer.
- [x] Phase 98 adds no distribution.
- [x] Phase 98 adds no signing.
- [x] Phase 98 adds no release engineering.
- [x] Phase 98 adds no readiness claims.
- [x] Phase 98 adds no authority surfaces.
- [x] Phase 98 adds no transport behavior.
- [x] Phase 98 adds no provider behavior.
- [x] Phase 98 adds no persistence behavior.
- [x] Phase 98 adds no replay behavior.
- [x] Phase 98 adds no recovery behavior.
- [x] Phase 98 adds no export behavior.
- [x] Phase 98 adds no action behavior.
- [x] Phase 98 does not improve, fix, rename, wire, or add commands.
- [x] Phase 98 does not approve startup, packaging, public usability, release-candidate readiness, production readiness, or Production Candidate status.
- [x] Phase 99 remains responsible for release engineering dry-run boundary work if Phase 98 evidence permits it.
- [x] Roadmap remains planned truth.
- [x] `CHANGELOG.md` remains historical truth.

## Task checklist
- [x] Ran initial `git status --short` and classified uncommitted files.
- [x] Cleaned or reverted generated artifact drift before edits where required; no cleanup was required.
- [x] Read `docs/roadmap/phase-map.md`, `docs/roadmap/phases.md`, and `docs/roadmap/sequencing.md`.
- [x] Read `CHANGELOG.md`, `checklists/current-phase.md`, `checklists/release.md`, `README.md`, and `AGENTS.md`.
- [x] Read required operations docs for Phases 86, 95.1, 95.2, 95.3, 95.4, 96, 97, and the local workflow guide.
- [x] Inspected current command surfaces for documentation accuracy only: `scripts/check.sh`, `core/Cargo.toml`, `core/src/main.rs`, `ui/package.json`, and `ui/run-api-behavior-tests.mjs`.
- [x] Confirmed Phase 98 title and scope from roadmap files.
- [x] Created `docs/operations/operator-troubleshooting-phase-98.md` with required frontmatter.
- [x] Documented validation gate troubleshooting.
- [x] Documented dry-run troubleshooting.
- [x] Documented Rust test troubleshooting.
- [x] Documented golden invariant troubleshooting.
- [x] Documented adversarial corpus troubleshooting.
- [x] Documented UI behavioral test troubleshooting.
- [x] Documented UI typecheck/lint/build troubleshooting.
- [x] Documented local build artifact troubleshooting.
- [x] Documented generated artifact drift cleanup.
- [x] Documented frontmatter/checklist validation failures.
- [x] Documented boundary lint failures.
- [x] Documented npm non-failing environment warning posture.
- [x] Documented Cargo-generated metadata drift.
- [x] Documented what not to infer from passing commands.
- [x] Updated `CHANGELOG.md` with `v0.0.98`.
- [x] Did not update roadmap files.
- [x] Did not start Phase 99.

## Validation checklist
- [x] `./scripts/check.sh` passed after final edits.
- [x] `cargo test --manifest-path core/Cargo.toml --all-targets` passed after final edits.
- [x] `cargo test --manifest-path core/Cargo.toml golden --all-targets` passed after final edits.
- [x] `cargo test --manifest-path core/Cargo.toml adversarial --all-targets` passed after final edits.
- [x] `cd ui && npm run test:api` passed after final edits.
- [x] `node scripts/test_rust_boundary_lint.mjs` passed after final edits.
- [x] `node scripts/rust_boundary_lint.mjs` passed after final edits.
- [x] `node scripts/test_lint_ui_boundaries.mjs` passed after final edits.
- [x] `node scripts/lint_ui_boundaries.mjs` passed after final edits.
- [x] `cd ui && npm run typecheck && npm run lint && npm run build` passed after final edits.
- [x] `cargo run --manifest-path core/Cargo.toml -- dry-run` passed after final edits.
- [x] `cargo build --manifest-path core/Cargo.toml` passed after final edits.
- [x] `cd ui && npm run build` passed after final edits.
- [x] Command coverage scan passed.
- [x] Troubleshooting topic scan passed.
- [x] No-capability scan passed.
- [x] Source/script/workflow guard showed no prohibited diffs.
- [x] Readiness scan showed no approval claim; matches were explicit non-approval/prohibition language.
- [x] Roadmap/changelog scan passed.
- [x] Lint wiring scan passed.
- [x] Final `git status --short` was reviewed before commit.
- [x] Final staged file list matched allowed Phase 98 surfaces.

## Operator documentation checklist
- [x] Required Scope section included.
- [x] Required Operator troubleshooting model section included.
- [x] Required Command interpretation rule section included.
- [x] Required Validation gate troubleshooting section included.
- [x] Required Dry-run troubleshooting section included.
- [x] Required Rust test troubleshooting section included.
- [x] Required Golden invariant troubleshooting section included.
- [x] Required Adversarial corpus troubleshooting section included.
- [x] Required UI behavioral test troubleshooting section included.
- [x] Required UI typecheck/lint/build troubleshooting section included.
- [x] Required Local build artifact troubleshooting section included.
- [x] Required Generated artifact drift cleanup section included.
- [x] Required Frontmatter and checklist validation failures section included.
- [x] Required Boundary lint failures section included.
- [x] Required Non-failing npm warning posture section included.
- [x] Required What passing checks do not mean section included.
- [x] Required What operators should not do section included.
- [x] Required Phase 96 relationship section included.
- [x] Required Phase 97 relationship section included.
- [x] Required Phase 99 relationship section included.
- [x] Required AST/boundary lint parity section included.
- [x] Required Test fidelity section included.
- [x] Required Validation evidence section included.
- [x] Required Confirmed vs suspected section included.
- [x] Required Deferred items section included.
- [x] Required Non-readiness statement section included.

## Command coverage checklist
- [x] `./scripts/check.sh` documented.
- [x] `cargo run --manifest-path core/Cargo.toml -- dry-run` documented.
- [x] `cargo test --manifest-path core/Cargo.toml --all-targets` documented.
- [x] `cargo test --manifest-path core/Cargo.toml golden --all-targets` documented.
- [x] `cargo test --manifest-path core/Cargo.toml adversarial --all-targets` documented.
- [x] `cd ui && npm run test:api` documented.
- [x] `cd ui && npm run typecheck && npm run lint && npm run build` documented.
- [x] `cargo build --manifest-path core/Cargo.toml` documented.
- [x] `cd ui && npm run build` documented as part of local build artifact troubleshooting.
- [x] Each documented command includes what it checks.
- [x] Each documented command includes what a pass means.
- [x] Each documented command includes what a pass does not mean.
- [x] Each documented command includes common failure signals.
- [x] Each documented command includes what to inspect next.
- [x] Each documented command includes what not to infer.

## Non-claim checklist
- [x] Documentation states that a passing command means only that the checked local boundary passed at that moment.
- [x] Documentation states that passing commands do not approve production readiness.
- [x] Documentation states that passing commands do not approve release-candidate readiness.
- [x] Documentation states that passing commands do not approve public usability.
- [x] Documentation states that passing commands do not approve packaging.
- [x] Documentation states that passing commands do not approve distribution.
- [x] Documentation states that passing commands do not approve installer use.
- [x] Documentation states that passing commands do not approve provider execution.
- [x] Documentation states that passing commands do not approve persistence authority.
- [x] Documentation states that passing commands do not approve replay repair.
- [x] Documentation states that passing commands do not approve recovery promotion.
- [x] Documentation states that passing commands do not approve action execution.
- [x] Documentation states Phase 98 has no runtime behavior, no new CLI, no packaging, no installer, no distribution, no signing, no release engineering, no readiness claims, no authority surfaces, no transport, no provider, no persistence, no replay, no recovery, no export, and no action behavior.

## Generated artifact cleanup checklist
- [x] `git status --short` was used to classify initial uncommitted files.
- [x] No initial generated artifact drift was present.
- [x] Post-validation generated artifact drift was inspected.
- [x] Post-validation `core/target/.rustc_info.json` drift was reverted.
- [x] Post-validation `scripts/__pycache__` drift was removed.
- [x] Cargo-generated metadata drift, including `core/target/.rustc_info.json`, was not staged.
- [x] UI build artifacts were not staged.
- [x] Node build artifacts were not staged.
- [x] Coverage output was not staged.
- [x] Test temp files were not staged.
- [x] Export temp files were not staged.
- [x] `scripts/__pycache__` was not staged.
- [x] Package/lockfile drift was not staged.

## Frontmatter/checklist failure checklist
- [x] Phase 98 operations doc has required frontmatter.
- [x] Checklist frontmatter remains procedural/authoritative/checklist_revision.
- [x] Changelog frontmatter remains historical/authoritative/changelog_entry.
- [x] Current-phase checklist has no stale Phase 97 unchecked completion items.
- [x] Completed Phase 98 task and validation items are checked.
- [x] Changelog claims match actual changed surfaces.

## Boundary lint troubleshooting checklist
- [x] Rust boundary lint commands are documented as blocking enforcement surfaces.
- [x] UI AST boundary lint commands are documented as blocking enforcement surfaces.
- [x] Documentation says not to patch lint behavior in Phase 98.
- [x] Documentation says new source/tooling patterns should be deferred rather than fixed in Phase 98.
- [x] Lint self-test changes are deferred to a separate maintenance phase if needed.

## AST/boundary lint parity checklist
- [x] Documentation says not to rely on `rg` scans as enforcement.
- [x] Documentation says `rg` scans are discovery/evidence only.
- [x] Blocking enforcement is assigned to `./scripts/check.sh`, Rust boundary lint, UI AST lint, compiler/type checks, clippy, UI behavioral tests, and Rust tests.
- [x] No lint behavior was changed.
- [x] No lint self-tests were added or changed.

## Test fidelity checklist
- [x] Phase 98 is documentation-only, so no new Rust tests were added.
- [x] Phase 98 is documentation-only, so no new TypeScript tests were added.
- [x] Full existing test/lint/check suite passed after final documentation edits.
- [x] No validation command was skipped after final edits.
- [x] No validation command printed assertion failure, panic, traceback, failed assertion, partial pass count, or masked failure while returning zero.

## Zero-drift checklist
- [x] Final source/script/workflow guard showed no Rust source diffs.
- [x] Final source/script/workflow guard showed no TypeScript source diffs.
- [x] Final source/script/workflow guard showed no test diffs.
- [x] Final source/script/workflow guard showed no script diffs.
- [x] Final source/script/workflow guard showed no workflow diffs.
- [x] Final source/script/workflow guard showed no README or AGENTS diffs.
- [x] Final source/script/workflow guard showed no package, lockfile, UI config, or roadmap diffs.
- [x] Final staged files exactly matched allowed Phase 98 surfaces.

## Findings table
| Finding | Evidence | Classification |
| --- | --- | --- |
| Phase 98 roadmap title is Operator Documentation and Troubleshooting Guide. | `docs/roadmap/phase-map.md`, `docs/roadmap/phases.md` | Confirmed planned truth. |
| Phase 98 is documentation-only and operator-facing only. | Phase 98 task boundary and operations doc | Confirmed boundary. |
| Existing commands were documented without command changes. | Final diff | Confirmed no capability change. |
| No generated artifact cleanup was needed before edits. | Initial `git status --short` | Confirmed hygiene finding. |
| Post-validation generated artifact drift was cleaned. | Final hygiene review | Reverted `core/target/.rustc_info.json`; removed `scripts/__pycache__`. |
| Roadmap remains planned truth. | Roadmap files and operations doc | Confirmed truth posture. |
| `CHANGELOG.md` remains historical truth. | Changelog and operations doc | Confirmed truth posture. |

## Deferred items table
| Item | Reason deferred | Target posture |
| --- | --- | --- |
| Broken validation gate repair, if ever observed later | Phase 98 must document, not fix, gates. | Later out-of-band maintenance phase. |
| New lint source/tooling pattern, if discovered later | Phase 98 must not patch lint behavior. | Later maintenance phase with lint self-tests. |
| New Rust or TypeScript tests for troubleshooting behavior | Phase 98 is documentation-only. | Later implementation or maintenance phase if approved. |
| Release engineering dry-run boundary work | Phase 99 owns release engineering dry-run work if Phase 98 evidence permits it. | Phase 99. |

## Validation log table
| Command | Result | Notes |
| --- | --- | --- |
| `./scripts/check.sh` | Passed | Main validation gate completed after final edits. |
| `cargo test --manifest-path core/Cargo.toml --all-targets` | Passed | Explicit Rust all-target tests completed. |
| `cargo test --manifest-path core/Cargo.toml golden --all-targets` | Passed | Explicit golden filter completed. |
| `cargo test --manifest-path core/Cargo.toml adversarial --all-targets` | Passed | Explicit adversarial filter completed. |
| `cd ui && npm run test:api` | Passed | Explicit UI behavioral harness completed. |
| `node scripts/test_rust_boundary_lint.mjs` | Passed | Rust boundary lint self-test completed. |
| `node scripts/rust_boundary_lint.mjs` | Passed | Rust boundary lint completed. |
| `node scripts/test_lint_ui_boundaries.mjs` | Passed | UI AST lint self-test completed. |
| `node scripts/lint_ui_boundaries.mjs` | Passed | UI AST lint completed. |
| `cd ui && npm run typecheck && npm run lint && npm run build` | Passed | Explicit UI validation completed. |
| `cargo run --manifest-path core/Cargo.toml -- dry-run` | Passed | CLI dry-run completed. |
| `cargo build --manifest-path core/Cargo.toml` | Passed | Rust local build completed. |
| `cd ui && npm run build` | Passed | UI local build completed. |
| Command coverage scan | Passed | Required commands present in Phase 98 documentation/checklist/changelog. |
| Troubleshooting topic scan | Passed | Required troubleshooting topics present. |
| No-capability scan | Passed | Required non-capability language present. |
| Source/script/workflow guard | Passed | No prohibited diffs. |
| Readiness scan | Passed | Matches are explicit non-approval/prohibition language only. |
| Roadmap/changelog scan | Passed | Planned/historical truth language present. |
| Lint wiring scan | Passed | Existing lint wiring references found. |
