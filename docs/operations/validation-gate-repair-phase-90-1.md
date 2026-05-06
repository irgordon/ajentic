---
truth_dimension: orientation
authority_level: advisory
mutation_path: readme_update
---

# Phase 90.1 - Out-of-Band Validation Gate Repair

## Scope
Phase 90.1 is an out-of-band validation gate repair before Phase 91.

Phase 90.1 repairs validation tooling only. It does not change runtime behavior, Rust runtime source, TypeScript app source, UI source, schemas, workflows, roadmap files, governance files, architecture files, dependency files, or package files.

Phase 90.1 is not a planned roadmap phase, does not change Phase 91 scope, and does not renumber Phase 91 or any later phase.

## Why this out-of-band repair exists
Phase 90 found a validation-gate fidelity defect that had to be repaired before Phase 91 could begin. The defect was limited to Rust boundary lint self-test reporting and local gate fidelity.

Phase 90.1 is an out-of-band validation gate repair before Phase 91.

## Defect discovered in Phase 90
Direct reproduction before repair confirmed that `node scripts/test_rust_boundary_lint.mjs` printed `Rust boundary lint self-tests passed (12/13).` and exited 0.

Because `./scripts/check.sh` invokes that command, the local gate could also exit 0 while carrying a partial Rust boundary lint self-test count.

## Rust boundary lint self-test repair
`scripts/test_rust_boundary_lint.mjs` now uses an explicit deterministic array of named self-tests. The expected total is derived from that array, each test is executed exactly once, and the final result requires the passed count, executed count, and expected total to agree.

The harness now exits 0 only after printing `Rust boundary lint self-tests passed (N/N)`. It exits nonzero and prints failing test names when any expected-pass fixture fails, any expected-fail fixture unexpectedly passes, a partial pass count occurs, an assertion fails, or a promise rejects.

## check.sh propagation assessment
`scripts/check.sh` already uses `set -euo pipefail` and directly invokes `node scripts/test_rust_boundary_lint.mjs` before the production Rust boundary lint. No `check.sh` change was required because the repaired self-test harness now exits nonzero on partial counts or failed self-tests, and `set -euo pipefail` stops the local gate on that failure.

## Lint rule preservation
Phase 90.1 does not weaken Rust boundary lint rules. Production lint behavior in `scripts/rust_boundary_lint.mjs` was preserved; the repair is limited to self-test harness fidelity.

## Failure-propagation proof
The harness includes a self-contained invariant test named `harness-fail: partial pass count is rejected by result summary`. It proves that a partial pass count is not considered successful by the result summarizer.

The command-level propagation proof is that the repaired harness calls `process.exit(1)` when the result summary is not successful. Since `scripts/check.sh` runs with `set -euo pipefail`, a deliberately failing Rust boundary lint self-test would make `node scripts/test_rust_boundary_lint.mjs` exit nonzero and `./scripts/check.sh` would stop at that command. No deliberately failing fixture remains enabled in the committed state.

## AST/boundary lint parity
`rg` scans are discovery and evidence only. Blocking enforcement comes from `scripts/check.sh`, the Rust boundary lint, the UI AST lint, compiler/type checks, clippy, and tests.

This phase repairs the Rust boundary lint self-test harness only, so production Rust lint behavior was preserved. UI AST lint behavior and self-tests were inspected and validated without changes.

## Test fidelity
New validation behavior is covered by named self-tests in the same phase. Test names describe the invariant being protected and identify expected-pass, expected-fail, and harness-fail behavior.

No phase is complete if validation commands are skipped after final edits. A command that prints assertion failure, panic, traceback, failed assertion, or a partial pass count is not acceptable even if the shell exit code is 0.

## Validation evidence
Phase 90.1 validation evidence includes direct repaired Rust boundary lint self-test execution, production Rust boundary lint execution, full local `./scripts/check.sh`, explicit Cargo tests, UI AST lint self-tests and production lint, explicit UI validation, CLI dry-run, source guard scans, readiness scans, out-of-band wording scans, lint wiring scans, and zero-drift checks.

The expected repaired Rust boundary lint self-test output is `Rust boundary lint self-tests passed (N/N)` with a matching numerator and denominator and exit code 0.

## Confirmed vs suspected
### Confirmed
- Phase 90.1 is an out-of-band validation gate repair before Phase 91.
- Phase 90.1 repairs validation tooling only.
- Phase 90.1 does not implement Phase 91 hardening.
- Phase 90.1 does not change runtime behavior.
- Phase 90.1 does not weaken Rust boundary lint rules.
- The pre-repair defect was a harness count/exit-code fidelity defect.
- `scripts/check.sh` already propagates the repaired harness failure through `set -euo pipefail`.
- Phase 91 remains responsible for transport abuse and submission spoofing hardening.

### Suspected
- No additional validator, Rust boundary lint production rule, UI AST lint, workflow, or Python validator defect was confirmed in Phase 90.1.

## Deferred items
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

## Non-readiness statement
Public usability, production readiness, Production Candidate approval, and release-candidate readiness are not claimed.

Production Candidate status: not approved.

Phase 90.1 does not implement Phase 91 hardening and does not change readiness posture.
