---
truth_dimension: orientation
authority_level: advisory
mutation_path: readme_update
---

# Phase 93.5 - Persistence Semantics Edge-Case Hardening

## Scope
Phase 93.5 is an out-of-band persistence semantics edge-case hardening pass before Phase 94.

Phase 93.5 is hardening and documentation only. Phase 93.5 adds no new persistence authority, no import behavior, no repair behavior, no continuous monitoring, no concurrent writer support, and no production-readiness claim.

## Why this out-of-band hardening pass exists
Phase 93 completed persistence corruption and append drift hardening. Phase 93.5 records and tests edge-case semantics discovered after that work, especially unsupported payload posture, paired audit and ledger append assumptions, write-time verification limits, export non-authority, recovery context matching, and non-repair behavior.

Phase 93.5 is not a planned roadmap phase and does not renumber Phase 94 or later phases.

## Version and unsupported-format posture
Persisted records carry an `AJENTIC_RECORD_V1` envelope marker and a payload kind. Phase 93.5 confirms unknown payload kinds reject with an explicit unknown-payload posture and unsupported envelope markers reject fail-closed as malformed records.

Durable append transaction bytes do not currently carry a transaction kind or format version field. Unsupported transaction-kind input is therefore representable only as an unexpected field and rejects as a malformed append transaction.

Explicit persistence format versioning remains deferred. Current Phase 93.5 hardening covers known envelope/payload kind and checksum/length drift, not legacy migration.

## Write-time verification posture
Successful append/write verification is write-time verification only. Success does not prove continuous integrity after later external tampering.

Phase 93.5 tests show that a verified durable append report can be followed by external file tampering and later verification must surface the corruption instead of relying on earlier write-time success.

## Paired audit and ledger append model
Current durable append requires paired audit + ledger payloads in one combined transaction. Audit-only append remains unsupported. Ledger-only append remains unsupported.

Phase 93.5 preserves the Phase 83 durable append success behavior for complete paired transactions and rejects single-sided payload attempts.

## Single-writer revision assumption
Current revision hardening assumes a single writer. Concurrent writers remain unsupported.

Phase 93.5 documents and tests revision-chain drift as fail-closed. It does not add locks, compare-and-swap semantics, merge behavior, background coordination, or concurrent writer machinery.

## Export-not-ledger posture
Export bundles are operator-readable artifacts only. Export bundles are not ledger state and cannot verify as durable append transactions.

Phase 93.5 does not add ledger import behavior and does not classify export bytes as ledger payloads.

## Export-not-recovery posture
Export bundles are not recovery input. Recovery candidates must originate from the expected ledger/recovery context.

Phase 93.5 adds fail-closed rejection for export-shaped bytes at the recovery acceptance boundary while preserving Phase 84 in-memory acceptance for valid non-export candidates.

## Export-not-replay-repair posture
Export bundles are not replay repair evidence. Phase 93.5 does not treat export bundles as replay repair input and does not add replay repair behavior.

## Recovery candidate context posture
Recovery candidates remain tied to expected recovery id, ledger record id, and revision context. Candidate mismatches reject without replacing global state, persisting, appending ledger/audit records, repairing replay, trusting provider output, or executing actions.

Phase 93.5 does not promote recovery candidates to global state.

## Replay verification-only posture
Replay verifies evidence; it does not repair persistence drift. Phase 93.5 does not repair replay drift.

Phase 94 remains responsible for provider output injection and replay abuse hardening.

## Corrupted read posture
Corrupted reads must fail or surface corruption explicitly. Phase 93.5 confirms malformed persisted records and malformed append transactions do not silently skip unexpected corrupted content.

## Non-repair guarantees
Phase 93.5 does not repair corrupted records. Phase 93.5 does not add continuous integrity monitoring. Phase 93.5 does not add replay repair, ledger repair, recovery repair, or corrupted record repair.

## Non-authority guarantees
Phase 93.5 adds no new persistence authority. It does not add import behavior, recovery import behavior, ledger import behavior, export import behavior, recovery promotion, global state replacement, provider/action execution, live transport, production readiness, release-candidate readiness, public usability, or Production Candidate approval.

## Relationship to Phase 93
Phase 93 established corruption and append drift hardening. Phase 93.5 is an out-of-band persistence semantics edge-case hardening pass before Phase 94 that tightens edge-case tests and documentation without changing Phase 93 historical scope.

## Relationship to Phase 94 provider/replay abuse hardening
Phase 94 remains responsible for provider output injection and replay abuse hardening. Phase 93.5 does not imply any change to Phase 94 scope and does not renumber Phase 94 or later phases.

## Root integration-test coverage
Root integration coverage is present for export bytes not becoming recovery candidates, export bytes not verifying as durable append transactions, append verification being write-time-only rather than continuous integrity proof, and the paired append model remaining required.

## AST/boundary lint parity
Phase 93.5 does not rely on `rg` scans as enforcement. `rg` scans are discovery/evidence only.

Blocking enforcement remains in `scripts/check.sh`, Rust boundary lint, UI AST lint, compiler/type checks, and tests. Phase 93.5 does not change lint behavior and does not add lint self-tests.

## Test fidelity
New hardening behavior has same-phase Rust tests. Cross-boundary behavior reachable through current public APIs has root integration coverage. Tests are deterministic and do not add time, random, environment-dependent behavior, network, async, processes, threads, dependencies, import paths, repair paths, continuous monitoring, or concurrent writer machinery.

## Validation evidence
Validation evidence for Phase 93.5 is the final successful run of `./scripts/check.sh`, explicit cargo tests, explicit boundary lints, explicit UI validation, CLI dry-run, required evidence scans, source guard, readiness scan, lint wiring scan, staged-file review, and final working-tree hygiene checks.

## Confirmed vs suspected
Confirmed:
- Unknown persisted payload kinds reject explicitly.
- Unsupported persisted envelope markers reject fail-closed.
- Durable append has no transaction-kind/version field; unexpected transaction-kind fields reject as malformed.
- Audit-only and ledger-only durable append remain unsupported.
- Out-of-order revision chains reject as drift.
- Append success is write-time verification only and does not prove continuous integrity after external tampering.
- Export bundle bytes are not ledger state, recovery input, or replay repair evidence.
- Recovery context mismatches reject without global state replacement or authority mutation.
- Replay verification remains verification-only and does not repair persistence drift.
- Corrupted reads do not silently skip malformed content.

Suspected:
- None within the Phase 93.5 edge-case hardening scope.

## Deferred items
| Item | Reason |
| --- | --- |
| Explicit broad persistence format versioning | Deferred; Phase 93.5 covers current envelope marker, payload kind, checksum, length, and transaction field drift only. |
| Legacy migration framework | Deferred; Phase 93.5 does not add migration support. |
| Concurrent writer support | Deferred; Phase 93.5 documents the current single-writer assumption only. |
| Continuous integrity monitoring | Deferred; successful writes remain write-time verification only. |
| Replay repair | Deferred; replay remains verification-only and Phase 94 covers provider/replay abuse hardening. |

## Non-readiness statement
Public usability, production readiness, Production Candidate approval, and release-candidate readiness are not claimed.
