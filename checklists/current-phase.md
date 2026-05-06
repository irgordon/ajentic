---
truth_dimension: procedural
authority_level: authoritative
mutation_path: checklist_revision
---

# Current Phase Checklist - Phase 95.1 Out-of-Band UI Behavioral Test Harness Baseline

Phase 95.1 is an out-of-band UI behavioral test harness baseline before Phase 96.

## Phase name
Phase 95.1 - Out-of-Band UI Behavioral Test Harness Baseline.

## Explicit out-of-band testing note
- [x] Phase 95.1 is documented as inserted after Phase 95 and before Phase 96.
- [x] Phase 95.1 is not described as a planned roadmap phase.
- [x] Phase 96 and later phases are not renumbered.
- [x] Phase 95.1 does not change Phase 96 scope.

## Phase goal
Add a real UI behavioral test harness baseline for the TypeScript submission/transport contract boundary so Phase 91 behavior is tested directly, not inferred from typecheck, lint, AST lint, or build output.

## Working-tree hygiene gate
- [x] Ran `git status --short` before edits.
- [x] Classified the initial working tree as clean with no uncommitted generated artifact drift.
- [x] Confirmed no cleanup was required for `core/target/.rustc_info.json`, UI build artifacts, test temp files, export temp files, node build artifacts, coverage output, or other generated artifact drift before edits.

## Allowed surfaces
- [x] `ui/src/api/submissionBoundary.behavior.test.ts` added for behavior tests.
- [x] `ui/run-api-behavior-tests.mjs` added as a minimal no-dependency UI behavior test runner.
- [x] `ui/package.json` updated only to add the no-dependency `test:api` script.
- [x] `scripts/check.sh` updated only to run the stable no-dependency UI behavior command during UI validation.
- [x] `docs/operations/ui-behavioral-test-harness-phase-95-1.md` added.
- [x] `checklists/current-phase.md` updated to Phase 95.1 procedural truth.
- [x] `CHANGELOG.md` updated with `v0.0.95.1`.

## Boundary rules
- [x] Phase 95.1 adds behavioral tests only.
- [x] Phase 95.1 does not add live transport.
- [x] Phase 95.1 does not add UI authority.
- [x] Phase 95.1 does not add a Rust bridge.
- [x] Phase 95.1 does not add runtime execution behavior.
- [x] Phase 95.1 does not start Phase 96.
- [x] Phase 95.1 does not approve startup/package work.
- [x] No Rust source, root tests, workflows, schemas, roadmap docs, governance docs, architecture docs, README, AGENTS, package lockfiles, or dependency lockfiles were changed.

## Task checklist
- [x] Read required Phase 95, Phase 91, Phase 76, Phase 77, roadmap, checklist, release, and changelog surfaces.
- [x] Inspected UI contract surfaces in `ui/src/api/` and UI config.
- [x] Inspected existing validation tooling and lint self-tests.
- [x] Added dependency-free UI behavioral test harness.
- [x] Added behavior tests for malformed submissions, spoofed flags, risky text, accepted preview, bridge non-call, envelope non-creation, user-supplied capability flags, and harness failure propagation.
- [x] Added `npm run test:api` without dependency or lockfile changes.
- [x] Wired `npm run test:api` into `scripts/check.sh` because the command is stable, dependency-free, and must fail the main gate if behavioral assertions fail.
- [x] Documented Phase 95.1 scope and evidence posture.
- [x] Added `CHANGELOG.md` `v0.0.95.1`.
- [x] Did not update roadmap files.

## Validation checklist
- [x] `./scripts/check.sh` returns 0 after final edits.
- [x] `cd ui && npm run test:api` returns 0 after final edits.
- [x] `cd ui && node run-api-behavior-tests.mjs --self-test-failure-propagation` returns 0 after final edits.
- [x] `cargo test --manifest-path core/Cargo.toml --all-targets` returns 0 after final edits.
- [x] Explicit Rust and UI boundary lint commands return 0 after final edits.
- [x] Explicit UI typecheck/lint/build returns 0 after final edits.
- [x] `cargo run --manifest-path core/Cargo.toml -- dry-run` returns 0 after final edits.
- [x] Required discovery/evidence scans were run after final edits.
- [x] No validation command printed panic, traceback, failed assertion, or partial pass count after final edits.

## UI behavioral harness checklist
- [x] Harness runs without Vitest, Jest, Mocha, or new dependencies.
- [x] Harness compiles TypeScript behavior tests into an OS temporary directory.
- [x] Harness removes temporary compile output after the run.
- [x] Harness exits nonzero on normal behavior test failure.
- [x] Harness prints a clear pass count for normal behavior tests.

## Malformed submission checklist
- [x] Empty operator id rejects before transport.
- [x] Empty target id rejects before transport.
- [x] Empty intent kind rejects before transport.
- [x] Unknown intent kind rejects before transport.
- [x] Non-object malformed input rejects before transport.
- [x] Malformed input does not create a sendable transport envelope.

## Spoofed flag checklist
- [x] `executionEnabled: true` rejects before transport.
- [x] `liveTransportEnabled: true` rejects before transport.
- [x] `persistenceEnabled: true` rejects before transport.
- [x] `ledgerRecordingEnabled: true` rejects before transport.
- [x] `auditAppendEnabled: true` rejects before transport.
- [x] `providerExecutionEnabled: true` rejects before transport.
- [x] `replayRepairEnabled: true` rejects before transport.
- [x] `mutatesAuthority: true` rejects before transport.
- [x] Combined user-supplied capability flags are rejected, not trusted.

## Risky text checklist
- [x] `admin override` rejects before transport.
- [x] `skip policy` rejects before transport.
- [x] `execute now` rejects before transport.
- [x] `write ledger` rejects before transport.
- [x] `append audit` rejects before transport.
- [x] `repair replay` rejects before transport.
- [x] `trust provider output` rejects before transport.
- [x] `promote recovered state` rejects before transport.

## Bridge non-call checklist
- [x] Stubbed bridge counter remains zero for malformed submission rejection.
- [x] No test invokes, simulates, or implies a live Rust bridge.
- [x] Accepted preview submissions remain non-live and non-executing.

## Failure propagation checklist
- [x] Normal behavior runner exits nonzero on counted failures.
- [x] Self-test mode intentionally runs a failing probe internally and exits 0 only after proving the failure was counted.
- [x] Normal behavior run has no enabled failing test.

## check.sh integration checklist
- [x] `scripts/check.sh` retains `set -euo pipefail`.
- [x] `scripts/check.sh` runs `npm run test:api` inside existing UI validation.
- [x] The check integration is limited to the stable no-dependency behavior command.
- [x] Failure propagation remains enforced by shell exit status.

## AST/boundary lint parity checklist
- [x] `rg` scans are treated as discovery/evidence only, not enforcement.
- [x] Blocking enforcement remains with `scripts/check.sh`, Rust boundary lint, UI AST lint, compiler/type checks, clippy, and tests.
- [x] No new UI source pattern requires lint expansion in this phase.
- [x] No lint behavior changed, so no lint self-test update was required.

## Test fidelity checklist
- [x] This phase records that typecheck/lint/build is not behavioral coverage.
- [x] Behavior test names describe the invariant being protected.
- [x] Tests were not skipped after final edits.
- [x] The behavioral command cannot print a normal failed assertion and exit 0.

## Zero-drift checklist
- [x] No UI build output was left in the repository.
- [x] No core target metadata was staged.
- [x] No lockfile drift was introduced.
- [x] No test temp, export temp, node build, or coverage output was left in the repository.
- [x] Staged files match the Phase 95.1 allowed surfaces plus the documented `scripts/check.sh` integration.

## Findings table

| Finding | Status | Evidence |
| --- | --- | --- |
| UI behavioral harness gap from Phase 95 | closed for Phase 91 UI boundary behavior | `npm run test:api` runs 18 behavior tests. |
| Malformed submissions | covered | Empty fields, unknown kind, and non-object input reject before transport. |
| Spoofed capability flags | covered | Individual and combined spoof attempts reject before transport. |
| Risky authority text | covered | All required risky text examples reject before transport. |
| Bridge non-call | covered | Stub counter remains zero and no live bridge is added. |
| Failure propagation | covered | Self-test mode proves the runner counts failures. |
| Cross-boundary golden invariants | deferred | Phase 95.2 remains responsible. |

## Deferred items table

| Deferred item | Owner | Notes |
| --- | --- | --- |
| Cross-boundary golden invariant tests | Phase 95.2 | Phase 95.1 does not claim this closure. |
| Broader adversarial LLM-output corpus depth | Phase 95.3 | Not part of UI behavior harness baseline. |
| Future AST/boundary lint expansion | Phase 95.4 if needed | No lint expansion needed for this diff. |
| Local startup command boundary | Phase 96 | Phase 95.1 does not start Phase 96. |

## Validation log table

| Command | Status | Notes |
| --- | --- | --- |
| `git status --short` | pass | Initial tree was clean; final tree contained only intended edits before staging. |
| `./scripts/check.sh` | pass | Includes `npm run test:api` through UI validation. |
| `cd ui && npm run test:api` | pass | 18/18 behavior tests passed. |
| `cd ui && node run-api-behavior-tests.mjs --self-test-failure-propagation` | pass | Failure propagation proof passed without leaving a normal failing test enabled. |
| `cargo test --manifest-path core/Cargo.toml --all-targets` | pass | Explicit cargo test gate passed. |
| `node scripts/test_rust_boundary_lint.mjs` | pass | Rust boundary lint self-tests passed. |
| `node scripts/rust_boundary_lint.mjs` | pass | Rust boundary lint passed. |
| `node scripts/test_lint_ui_boundaries.mjs` | pass | UI AST lint self-tests passed. |
| `node scripts/lint_ui_boundaries.mjs` | pass | UI AST lint passed. |
| `cd ui && npm run typecheck && npm run lint && npm run build` | pass | Explicit UI validation passed. |
| `cargo run --manifest-path core/Cargo.toml -- dry-run` | pass | CLI dry-run passed. |
| Required discovery/evidence scans | pass | Scans matched expected test/docs/checklist evidence only. |
| Package drift scan | pass | Only `ui/package.json` changed; no lockfile drift. |
| Source guard | pass | Only documented `scripts/check.sh` integration appeared in guarded surfaces. |

## Non-readiness statement
Phase 95.1 is an out-of-band UI behavioral test harness baseline before Phase 96.

Phase 95.1 adds behavioral tests only.

Phase 95.1 does not add live transport.

Phase 95.1 does not add UI authority.

Phase 95.1 does not add a Rust bridge.

Phase 95.1 does not start Phase 96.

Phase 95.2 remains responsible for cross-boundary golden invariant tests.

Public usability, production readiness, Production Candidate approval, startup/package approval, and release-candidate readiness are not claimed.
