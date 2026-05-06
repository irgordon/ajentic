---
truth_dimension: procedural
authority_level: authoritative
mutation_path: checklist_revision
---

# Current Phase Checklist

## phase name
Phase 90.1 - Out-of-Band Validation Gate Repair

## explicit out-of-band validation gate repair note
Phase 90.1 is an out-of-band validation gate repair before Phase 91.

Phase 90.1 is not a planned roadmap phase, does not change Phase 91 scope, and does not renumber Phase 91 or any later phase.

## phase goal
Repair the Rust boundary lint self-test/check.sh fidelity defect discovered during Phase 90 so the self-test count and process exit code agree and the local validation gate exits 0 only when all required self-tests, production lints, compiler/type checks, clippy, and tests pass.

## working-tree hygiene gate
- [x] `git status --short` reviewed before edits; initial working tree was clean.
- [x] Prior generated artifact drift was checked before edits; no `core/target/.rustc_info.json`, test temp files, export temp files, or other generated artifact drift was present in git status.
- [x] Required Phase 90.1 input surfaces were read or inspected: `CHANGELOG.md`, `checklists/current-phase.md`, `docs/operations/repository-audit-phase-90.md`, `scripts/check.sh`, `scripts/rust_boundary_lint.mjs`, `scripts/test_rust_boundary_lint.mjs`, `scripts/lint_ui_boundaries.mjs`, `scripts/test_lint_ui_boundaries.mjs`, `scripts/validate_structure.py`, `scripts/validate_docs.py`, and `.github/workflows/*.yml`.
- [x] Roadmap docs were not changed; absence of an explicit Phase 90.1 roadmap entry was not treated as an error.

## allowed surfaces
- [x] `scripts/test_rust_boundary_lint.mjs`
- [x] `checklists/current-phase.md`
- [x] `CHANGELOG.md`
- [x] `docs/operations/validation-gate-repair-phase-90-1.md`
- [x] `scripts/rust_boundary_lint.mjs` not changed because production lint behavior did not require repair.
- [x] `scripts/check.sh` not changed because `set -euo pipefail` already propagates nonzero node exits.

## boundary rules
- [x] Phase 90.1 is an out-of-band validation gate repair before Phase 91.
- [x] Phase 90.1 repairs validation tooling only.
- [x] Phase 90.1 does not change runtime behavior.
- [x] Phase 90.1 does not implement Phase 91 hardening.
- [x] Phase 90.1 does not weaken Rust boundary lint rules.
- [x] Phase 91 remains responsible for transport abuse and submission spoofing hardening.
- [x] Public usability, production readiness, Production Candidate approval, and release-candidate readiness are not claimed.

## task checklist
- [x] Updated this checklist to Phase 90.1 procedural truth.
- [x] Created `docs/operations/validation-gate-repair-phase-90-1.md` with required advisory frontmatter and sections.
- [x] Reproduced the direct defect before repair.
- [x] Inspected the Rust boundary lint self-test harness and identified a hard-coded expected total with only 12 incremented checks.
- [x] Converted Rust boundary lint self-tests into an explicit deterministic named test array.
- [x] Derived the expected total from the test list.
- [x] Ensured each expected self-test is executed and counted once.
- [x] Ensured failed self-tests exit nonzero and print failing test names.
- [x] Added harness-level coverage proving partial pass counts are rejected.
- [x] Preserved production Rust boundary lint behavior.
- [x] Documented that no `scripts/check.sh` change was required.
- [x] Added `CHANGELOG.md` v0.0.90.1.
- [x] Did not update roadmap files.

## validation checklist
- [x] `node scripts/test_rust_boundary_lint.mjs` run before repair and reproduced `Rust boundary lint self-tests passed (12/13).` with exit 0.
- [x] `node scripts/test_rust_boundary_lint.mjs` run after repair.
- [x] `node scripts/rust_boundary_lint.mjs`
- [x] `./scripts/check.sh`
- [x] `cargo test --manifest-path core/Cargo.toml --all-targets`
- [x] `node scripts/test_lint_ui_boundaries.mjs`
- [x] `node scripts/lint_ui_boundaries.mjs`
- [x] `cd ui && npm run typecheck && npm run lint && npm run build`
- [x] `cargo run --manifest-path core/Cargo.toml -- dry-run`
- [x] Self-test repair scan run.
- [x] Lint-rule preservation scan run.
- [x] check.sh propagation scan run.
- [x] Source guard run.
- [x] Readiness scan run.
- [x] Out-of-band wording scan run.
- [x] Lint wiring scan run.
- [x] Final `git status --short` reviewed before commit.
- [x] `git diff --name-only --cached` reviewed before commit.

## defect investigation checklist
- [x] Direct reproduction command run: `node scripts/test_rust_boundary_lint.mjs`.
- [x] Pre-repair output captured as `Rust boundary lint self-tests passed (12/13).` with exit 0.
- [x] Harness inspected for why partial pass count exited 0.
- [x] Confirmed all ad hoc self-test blocks present in the old harness were executed but only 12 checks were incremented against a hard-coded denominator of 13.
- [x] Missing test classified as a counting/harness defect rather than a broken fixture, stale Phase 89 expectation, async bug, Promise bug, or child-process handling bug.
- [x] Root cause repaired in the harness.

## self-test count/exit-code checklist
- [x] Expected total is derived deterministically from the explicit test list.
- [x] Harness fails if `passed_count !== expected_total`.
- [x] Harness fails if any individual self-test fails.
- [x] Harness fails if an expected-fail fixture unexpectedly passes.
- [x] Harness fails if an expected-pass fixture unexpectedly fails.
- [x] Harness fails on uncaught assertion errors.
- [x] Harness fails on rejected promises.
- [x] Harness prints `Rust boundary lint self-tests passed (N/N)` only on success.
- [x] Harness prints failing test names and exits nonzero on failure.

## check.sh propagation checklist
- [x] `scripts/check.sh` contains `set -euo pipefail`.
- [x] `scripts/check.sh` directly invokes `node scripts/test_rust_boundary_lint.mjs`.
- [x] `scripts/check.sh` directly invokes `node scripts/rust_boundary_lint.mjs`.
- [x] Repaired nonzero self-test exits would stop `./scripts/check.sh` at the Rust boundary lint self-test command.
- [x] No `scripts/check.sh` change was required.

## lint-rule preservation checklist
- [x] `scripts/rust_boundary_lint.mjs` was inspected.
- [x] Production Rust boundary lint rules were not weakened.
- [x] No suppression-only fix was added.
- [x] No negative-path self-test was removed.
- [x] No failing test was changed into a skipped test.
- [x] No expected coverage was reduced to make counts pass.

## AST/boundary lint parity checklist
- [x] `rg` scans treated as discovery/evidence only, not enforcement.
- [x] Blocking enforcement remains `scripts/check.sh`, Rust boundary lint, UI AST lint, compiler/type checks, clippy, and tests.
- [x] UI AST lint self-tests and production lint were run after final edits.
- [x] Production Rust lint behavior was preserved because this phase only repaired the self-test harness.

## test fidelity checklist
- [x] New validation behavior has self-tests in the same phase.
- [x] Test names describe the invariant being protected.
- [x] No validation command was skipped after final edits.
- [x] No final validation command printed assertion failure, panic, traceback, failed assertion, or partial pass count.

## zero-drift checklist
- [x] No Rust source diffs.
- [x] No TypeScript source diffs.
- [x] No test diffs outside the allowed self-test harness.
- [x] No UI source diffs.
- [x] No workflow diffs.
- [x] No schema diffs.
- [x] No roadmap diffs.
- [x] No governance doc diffs.
- [x] No architecture doc diffs.
- [x] No README or AGENTS diffs.
- [x] No dependency, package, lockfile, or UI config diffs.
- [x] Generated artifacts removed before commit.

## findings table
| Area | Finding | Status |
| --- | --- | --- |
| Defect reproduction | `node scripts/test_rust_boundary_lint.mjs` printed `Rust boundary lint self-tests passed (12/13).` and exited 0 before repair. | confirmed |
| Root cause | The old harness used ad hoc check increments and a hard-coded denominator; only 12 checks were incremented for a printed expected total of 13. | confirmed |
| Missing test classification | Counting/harness defect; not a broken fixture, stale Phase 89 expectation, async/Promise bug, or child-process exit-code handling bug. | confirmed |
| Repair | The harness now uses an explicit named test array, deterministic expected total, failure aggregation, failing test names, and explicit process exits. | complete |
| check.sh propagation | `set -euo pipefail` already propagates the repaired nonzero harness exit. | confirmed |
| Lint rules | Production Rust boundary lint behavior was preserved. | confirmed |

## deferred items table
| Deferred item | Owning planned phase |
| --- | --- |
| Transport abuse and submission spoofing hardening | Phase 91 |
| Authorization/audit/action mismatch hardening | Phase 92 |
| Persistence corruption and append drift hardening | Phase 93 |
| Provider output injection and replay abuse hardening | Phase 94 |
| Evidence realignment after hardening | Phase 95 |
| Local startup command boundary | Phase 96 |
| Packaging artifact definition | Phase 97 |
| Operator troubleshooting documentation | Phase 98 |
| Release engineering dry run | Phase 99 |
| Final Production Candidate readiness decision evidence | Phase 100 |

## validation log table
| Command | Result | Notes |
| --- | --- | --- |
| `node scripts/test_rust_boundary_lint.mjs` | pre-repair pass with fidelity defect | Printed `Rust boundary lint self-tests passed (12/13).` and exited 0. |
| `node scripts/test_rust_boundary_lint.mjs` | pass | Repaired output printed full `N/N` with exit 0. |
| `node scripts/rust_boundary_lint.mjs` | pass | Production Rust boundary lint passed. |
| `./scripts/check.sh` | pass | Full local gate passed after repair. |
| `cargo test --manifest-path core/Cargo.toml --all-targets` | pass | Explicit Rust tests passed. |
| `node scripts/test_lint_ui_boundaries.mjs` | pass | UI AST lint self-tests passed. |
| `node scripts/lint_ui_boundaries.mjs` | pass | UI AST lint production scan passed. |
| `cd ui && npm run typecheck && npm run lint && npm run build` | pass | Explicit UI validation passed. |
| `cargo run --manifest-path core/Cargo.toml -- dry-run` | pass | CLI dry-run passed. |
| Self-test repair scan | pass | Harness and docs evidence present. |
| Lint-rule preservation scan | pass | Production Rust lint preserved; evidence reviewed. |
| check.sh propagation scan | pass | `set -euo pipefail` and lint commands documented. |
| Source guard | pass | No disallowed source, workflow, package, config, README, AGENTS, or roadmap diffs. |
| Readiness scan | pass | No readiness approval claim added. |
| Out-of-band wording scan | pass | Required Phase 90.1 wording present on affected documentation surfaces. |
| Lint wiring scan | pass | Local and CI lint wiring evidence reviewed. |
