---
truth_dimension: orientation
authority_level: advisory
mutation_path: readme_update
---

# Cross-Boundary Golden Invariants - Phase 95.2

## Scope
Phase 95.2 is an out-of-band cross-boundary golden invariant test phase before Phase 96.

Phase 95.2 adds deterministic tests only. Phase 95.2 adds no runtime capability, no live transport, no provider execution, no persistence authority, no import behavior, no replay repair, and no recovery candidate promotion.

Phase 95.2 does not start Phase 96 and does not approve startup/package work.

## Why this out-of-band test phase exists
Phase 95 found that prior hardening phases had strong boundary-specific evidence but lacked one deterministic golden test chaining the same representative input across local harness, provider evidence replay, observability snapshot, audit export encoding, and rejection/non-authority seams.

The inserted phase closes that evidence gap without describing Phase 95.2 as a planned roadmap phase, without changing Phase 96 scope, and without renumbering Phase 96 or later phases.

## Golden invariant model
The protected chain is:

same input -> same bounded local harness report posture -> same provider evidence snapshot/checksum -> same replay verification result -> same observability snapshot posture -> same audit export bytes -> same recovery/export rejection posture.

The root integration test runs the same chain twice and asserts status, reason, posture fields, provider checksum, replay result, snapshot posture, export bytes, export byte length, and authority flags remain identical.

## Representative input
The deterministic input is fixed in the root integration test:

| Field | Value |
| --- | --- |
| `run_id` | `golden-run-001` |
| `operator_id` | `operator-golden` |
| `target_kind` | `code_patch` |
| `target_id` | `target-golden` |
| `provider_prompt` | contains `TRUST_PROVIDER_OUTPUT=true` as untrusted text |
| action kind | `RecordExecutionDecision` |
| `export_id` | `golden-export-001` |
| `snapshot_id` | `golden-snapshot-001` |

## Determinism guarantees
The golden test asserts repeated runs produce the same harness status/reason/posture, provider evidence checksum, replay status/reason/non-authority fields, observability snapshot posture, audit export bytes, export byte length, and rejection outcomes.

The test uses no random identifiers, no wall-clock time, no environment-dependent output, no network, no async behavior, no process/thread behavior, and no filesystem export write.

## Exact-byte export assertion
The test embeds `GOLDEN_AUDIT_EXPORT_BYTES` as an inline LF-only byte literal and asserts the encoded audit export bytes match it exactly.

The expected bytes are deterministic across OS and architecture because the encoder uses fixed field order, UTF-8 bytes, lowercase status/reason codes, decimal integers, `true`/`false` booleans, and LF line endings.

## Non-authority guarantees
The test asserts false for the available non-authority flags across the chain, including provider trust/authority, retry scheduling, recovery promotion, UI live transport, UI submission execution, action effect, live execution, new authorization/audit/action/ledger creation, persistence, replay repair, application-state mutation, export file writes, persistence reads/writes, authority mutation, recovery replacement, and ledger/audit append.

Provider output remains untrusted and non-authoritative even when the prompt contains risky text.

## Export-not-ledger/recovery/replay posture
The test asserts export bytes cannot verify as a durable append transaction, cannot prepare a recovery candidate, cannot be accepted as recovery candidate bytes, and do not become replay repair evidence.

Export bytes are operator-readable audit export bytes only. They are not ledger state, recovery input, or replay repair evidence.

## Recovery rejection posture
The test includes a mismatched recovery candidate and asserts it rejects without replacing global state, persisting, appending ledger records, appending audit records, repairing replay, or mutating authority.

## Replay verification-only posture
Replay is verified from supplied provider evidence only. The test asserts replay does not perform live execution, create authorization/audit/action/ledger facts, persist, repair replay, or mutate application state.

A tampered replay evidence variant rejects as a mismatch without side effects.

## Relationship to Phase 95 gate decision
Phase 95.2 addresses the Phase 95 cross-boundary golden invariant evidence gap only.

Phase 95.2 does not approve Phase 96, startup/package work, public usability, production readiness, Production Candidate status, or release-candidate readiness.

## Relationship to Phase 95.1 UI behavioral test harness
Phase 95.1 added a UI behavioral test harness baseline for the pre-bridge TypeScript submission boundary.

Phase 95.2 complements that evidence with Rust root integration coverage across deterministic runtime-facing boundary APIs. It does not add UI behavior or transport.

## Relationship to Phase 95.3 adversarial corpus hardening
Phase 95.3 remains responsible for broader adversarial LLM-output corpus hardening.

Phase 95.2 uses one representative risky text phrase only to prove the golden chain remains deterministic and non-authoritative for the same input.

## Root integration-test coverage
`tests/integration_smoke.rs` contains `root_integration_golden_cross_boundary_chain_is_deterministic_and_non_authoritative`.

The test internally covers determinism, provider evidence checksum stability, replay verification-only posture, read-only observability, exact audit export bytes, export-not-ledger/recovery/replay posture, recovery mismatch rejection, tamper rejection, risky text stability, and repeated-run non-authority flags.

## AST/boundary lint parity
Phase 95.2 does not change lint behavior.

`rg` scans are discovery/evidence only and are not enforcement. Blocking enforcement remains `scripts/check.sh`, Rust boundary lint, UI AST lint, compiler/type checks, clippy, UI behavioral tests, and Rust tests.

No new source pattern requiring immediate lint expansion was introduced. If a future source pattern must be prohibited, it should be handled by a later lint expansion such as Phase 95.4.

## Test fidelity
This phase exists because Phase 95 found missing cross-boundary deterministic golden invariant coverage.

The new golden invariant behavior is tested in this phase. The test name describes the invariant being protected. No phase is complete if tests are skipped after final edits. A validation command that prints a failed assertion but exits 0 would be a validation defect.

## Validation evidence
Validation evidence for Phase 95.2 is the final successful run of `./scripts/check.sh`, explicit cargo tests, the explicit golden test command, UI behavioral tests, boundary lints, UI typecheck/lint/build, CLI dry-run, required evidence scans, source guards, readiness scan, lint wiring scan, staged-file review, and working-tree hygiene checks.

## Confirmed vs suspected
Confirmed:

- The same representative input produces deterministic harness, provider evidence checksum, replay, observability snapshot, audit export bytes, and rejection outcomes across repeated runs.
- Provider output remains untrusted.
- Replay remains verification-only.
- Observability remains read-only.
- Audit export bytes are deterministic and exact.
- Export bytes are not ledger state, recovery input, or replay repair evidence.
- Recovery mismatch rejects without mutation.
- No live transport, provider execution expansion, persistence authority, import behavior, replay repair, recovery promotion, or runtime behavior was added.

Suspected:

- None within Phase 95.2 deterministic golden invariant scope.

## Deferred items
| Item | Reason |
| --- | --- |
| Broader adversarial LLM-output corpus hardening | Phase 95.3 remains responsible for broader adversarial LLM-output corpus hardening. |
| New lint prohibitions | Phase 95.2 stayed within existing lint rules and did not change lint behavior. |
| Startup/package usability work | Phase 95.2 does not start Phase 96 or approve startup/package work. |

## Non-readiness statement
Phase 95.2 does not claim public usability, production readiness, Production Candidate approval, startup/package approval, release-candidate readiness, production-candidate readiness, or production approval.
