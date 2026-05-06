---
truth_dimension: orientation
authority_level: advisory
mutation_path: readme_update
---

# UI Behavioral Test Harness Baseline - Phase 95.1

## Scope
Phase 95.1 is an out-of-band UI behavioral test harness baseline before Phase 96.

Phase 95.1 adds behavioral tests only for the TypeScript submission boundary that was hardened in Phase 91. It adds no live transport, no UI authority, no Rust bridge, no runtime execution behavior, and no Phase 96 startup/package work.

## Why this out-of-band testing phase exists
Phase 95 found that Phase 91 had a TypeScript submission boundary, but did not have a committed UI behavioral test harness. TypeScript typecheck, UI lint, AST lint, and build are useful gates, but they are not substitutes for behavioral tests.

The inserted baseline proves the submission boundary behavior directly before Phase 96. It is not a planned roadmap phase, does not change Phase 96 scope, and does not renumber Phase 96 or later phases.

## UI behavioral test harness model
The harness is dependency-free and runs from `ui/run-api-behavior-tests.mjs` through `npm run test:api`. It compiles the TypeScript API boundary and behavior test file into an operating-system temporary directory, imports the compiled behavior tests, runs assertion-based test functions, prints per-test pass lines, and exits nonzero if any behavior test fails.

The harness removes its temporary compile output after each run. It does not write UI build output into the repository.

## Submission boundary behavior covered
The behavior tests cover malformed and rejected submissions before bridge eligibility:

- empty operator id
- empty target id
- empty intent kind
- unknown intent kind
- authority-escalation text
- malformed non-object submission input

Every rejected result is asserted to keep `status="rejected"`, `transportEligible=false`, `liveTransportCalled=false`, and all live transport, execution, persistence, ledger, audit, provider, replay repair, and authority mutation flags false.

## Spoofed capability flag coverage
The tests cover user-supplied spoof attempts for:

- `executionEnabled: true`
- `liveTransportEnabled: true`
- `persistenceEnabled: true`
- `ledgerRecordingEnabled: true`
- `auditAppendEnabled: true`
- `providerExecutionEnabled: true`
- `replayRepairEnabled: true`
- `mutatesAuthority: true`

User-supplied authority/capability flags are rejected, not trusted. The accepted-preview path also asserts all of these authority-bearing capabilities remain false.

## Risky text coverage
The behavior tests cover these risky text examples:

- `admin override`
- `skip policy`
- `execute now`
- `write ledger`
- `append audit`
- `repair replay`
- `trust provider output`
- `promote recovered state`

Each risky text example rejects before transport eligibility and does not enable execution, persistence, append, provider execution, replay repair, or authority mutation.

## Stubbed bridge non-call guarantee
The malformed-submission test creates a local stub counter and proves the stub remains uncalled when malformed input is rejected. No test invokes, simulates, or implies a live Rust bridge.

Phase 95.1 does not add a Rust bridge.

## Failure propagation proof
The runner has a self-test mode:

```sh
node run-api-behavior-tests.mjs --self-test-failure-propagation
```

That mode intentionally runs a failing internal probe, suppresses the expected probe diagnostic, verifies the harness counted the failure, and exits 0 only when failure propagation is proven. Normal behavior tests do not include an enabled failing test.

A behavioral test command that prints a failed assertion but exits 0 would be a validation defect; this harness counts failures and exits nonzero on normal test failure.

## Relationship to Phase 91
Phase 91 added the TypeScript submission boundary that rejects malformed submissions, risky authority-escalation text, and spoofed capability/authority flags before transport eligibility. Phase 95.1 tests that behavior directly instead of inferring it from typecheck, lint, or build output.

Phase 95.1 adds behavioral tests only.

Phase 95.1 does not add live transport.

Phase 95.1 does not add UI authority.

## Relationship to Phase 95 gate decision
Phase 95 identified the missing UI behavioral harness as a required out-of-band testing gap before startup/package work. Phase 95.1 closes only that testing-infrastructure and UI contract behavior gap.

Phase 95.1 does not start Phase 96.

Startup/package approval, release-candidate readiness, public usability, production readiness, and Production Candidate approval are not claimed.

## Relationship to Phase 95.2 golden invariant tests
Phase 95.2 remains responsible for cross-boundary golden invariant tests. Phase 95.1 is limited to UI behavioral test harness baseline coverage for the TypeScript submission boundary.

## AST/boundary lint parity
Phase 95.1 stays within existing UI source boundary lint rules. The behavior tests and harness do not add browser event handlers, fetch, XMLHttpRequest, WebSocket, EventSource, postMessage bridges, wasm, FFI, server endpoints, persistence, ledger/audit append, provider execution, replay repair, action execution, or authority mutation behavior.

`rg` scans are discovery/evidence only. Blocking enforcement remains with `scripts/check.sh`, Rust boundary lint, UI AST lint, compiler/type checks, clippy, and tests.

## Test fidelity
This phase exists because typecheck/lint/build is not behavioral coverage. The behavior test names describe the protected invariants, and the final validation state must include a direct run of the UI API behavior command after final edits.

## Validation evidence
Validation evidence for Phase 95.1 includes:

- `./scripts/check.sh`
- `cd ui && npm run test:api`
- `cd ui && node run-api-behavior-tests.mjs --self-test-failure-propagation`
- explicit cargo tests
- explicit Rust and UI boundary lints
- explicit UI typecheck/lint/build
- CLI dry-run
- required discovery scans for behavior names, spoof flags, risky text, no-live-transport terms, package drift, source guard, readiness wording, out-of-band wording, and lint wiring

## Confirmed vs suspected

| Item | Status | Evidence posture |
| --- | --- | --- |
| UI behavioral harness exists | confirmed | `npm run test:api` runs the no-dependency runner. |
| Phase 91 submission boundary has behavior tests | confirmed | Behavior tests exercise malformed input, spoof flags, risky text, accepted preview, bridge non-call, and envelope non-creation. |
| Typecheck/lint/build are behavioral substitutes | rejected | They remain validation gates only, not behavioral proof. |
| Live transport or Rust bridge exists | rejected | Phase 95.1 adds neither. |
| Phase 95.2 cross-boundary golden invariant tests are closed | not claimed | Phase 95.2 remains responsible. |

## Deferred items

| Deferred item | Owner |
| --- | --- |
| Cross-boundary golden invariant tests | Phase 95.2 |
| Broader adversarial LLM-output corpus depth | Phase 95.3 |
| AST/boundary lint expansion if future UI patterns require new prohibitions | Phase 95.4 |
| Local startup command boundary | Phase 96 |

## Non-readiness statement
Phase 95.1 is an out-of-band UI behavioral test harness baseline before Phase 96.

Phase 95.1 adds behavioral tests only.

Phase 95.1 does not add live transport.

Phase 95.1 does not add UI authority.

Phase 95.1 does not add a Rust bridge.

Phase 95.1 does not start Phase 96.

Phase 95.2 remains responsible for cross-boundary golden invariant tests.

Public usability, production readiness, Production Candidate approval, startup/package approval, and release-candidate readiness are not claimed.
