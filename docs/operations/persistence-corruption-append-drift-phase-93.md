---
truth_dimension: orientation
authority_level: advisory
mutation_path: readme_update
---

# Phase 93 - Persistence Corruption and Append Drift Hardening

## Scope
Phase 93 is hardening only. It adds negative-path coverage for persisted-record corruption, durable append drift, partial and failed writes, recovery candidate mismatch, and export non-authority posture.

Phase 93 adds no new persistence authority, recovery promotion, replay repair, provider execution, action execution, live transport, production-readiness claim, release-candidate-readiness claim, public-usability claim, or Production Candidate approval.

## Persistence corruption model
Persisted records remain valid only when the decoded envelope header, payload kind, revision, payload length, checksum, and payload bytes all agree. Payload length mismatch, invalid payload hex, checksum mismatch, malformed records, unknown payload kinds, and stale revisions reject fail-closed.

Phase 93 confirmed this through tests named `persisted_record_rejects_payload_length_mismatch`, `persisted_record_rejects_invalid_payload_hex`, and `persisted_record_rejects_checksum_mismatch`, alongside existing verification-path coverage.

## Durable append drift model
Durable append transactions remain complete paired audit-plus-ledger evidence envelopes. The append revision must advance by exactly one from `prior_revision` to `next_revision`. Missing audit payloads, missing ledger payloads, malformed revisions, stale revision chains, transaction-id mismatches, audit payload checksum drift, and ledger payload checksum drift reject.

Phase 93 confirmed this through tests covering checksum drift, missing payloads, malformed revisions, stale revision chains, transaction-id mismatch, prior/next revision mismatch, audit-only transactions, ledger-only transactions, and checksum drift after payload tampering.

## Partial-write and failed-write posture
Partial writes are represented as non-committed verification failures, including orphaned temp files. Failed write plans reject before commit. Verification failures return reports with `committed=false`.

Phase 93 does not add corruption repair, partial-write repair, temp-file cleanup behavior, or replay-repair behavior.

## Recovery candidate mismatch posture
Recovery candidates must match expected recovery ID, ledger record ID, and revision metadata before in-memory acceptance. Empty candidate bytes reject. Rejected candidates do not replace global state, persist, append ledger records, append audit records, or repair replay.

Phase 93 adds recovery mismatch tests for candidate recovery-id mismatch, candidate ledger-record-id mismatch, candidate revision mismatch, empty candidate bytes, non-replacement, non-persistence/non-append, and non-replay-repair.

## Export-not-ledger posture
Export bundles remain operator-readable artifacts, not ledger state, recovery input, or replay repair evidence. Export write artifacts cannot be verified as durable append transactions.

Phase 93 adds public integration smoke coverage proving export bundles are not authoritative state, are not ledger state, are not recovery input, are not replay repair evidence, and cannot be verified as durable append transactions.

## Replay drift posture
Phase 93 does not add replay repair behavior. Existing replay/evidence APIs remain diagnostic or verification surfaces only. Replay drift repair is deferred to future hardening where replay-specific abuse can be tested without adding repair or authority.

## Non-repair guarantees
Phase 93 does not repair corrupted records, durable append drift, partial writes, failed writes, mismatched recovery candidates, export artifacts, or replay drift. No corrupted or mismatched persistence artifact is silently repaired.

## Non-authority guarantees
Phase 93 adds no persistence authority. It does not make export bundles authoritative. It does not make recovery candidates global state. It does not promote recovery candidates. It does not import export bundles as ledger, recovery, or replay-repair input. It does not persist, append new authority, execute providers/actions, or mutate application state.

## Relationship to Phase 83 durable append
Phase 83 durable append success behavior is preserved: a complete audit-plus-ledger transaction that writes through the persistence boundary and verifies after write still reports a verified committed append transaction.

## Relationship to Phase 84 recovery acceptance
Phase 84 recovery acceptance success behavior is preserved: matching candidates can still be accepted for explicit in-memory use only and remain non-persistent, non-promoting, and non-mutating.

## Relationship to Phase 89 local export write
Phase 89 export-write non-authority posture is preserved. Export write remains a local verified artifact write only. It is not ledger append, recovery input, replay repair, telemetry, or authority mutation.

## Relationship to Phase 94 provider/replay abuse hardening
Phase 94 remains responsible for provider output injection and replay abuse hardening. Phase 93 does not expand provider or replay behavior and does not repair replay drift.

## Root integration-test coverage
Root integration smoke coverage was added for durable append tamper rejection, recovery acceptance mismatch rejection, and export bundle non-authority. Export-not-ledger checks are expressible through current public APIs without adding import behavior.

## AST/boundary lint parity
Phase 93 does not change lint behavior. `rg` scans are discovery and evidence only. Blocking enforcement remains in `scripts/check.sh`, Rust boundary lint, UI AST lint, compiler/type checks, and tests.

No new Rust source pattern requiring immediate lint expansion was introduced.

## Test fidelity
New hardening behavior has same-phase tests. Cross-boundary behavior that is publicly reachable has root integration coverage. Tests are deterministic except pre-existing export temp-file helpers that use unique names to avoid overwriting existing files. No network, async, provider execution, action execution, production transport, dependency change, or broad export reshaping was added.

## Validation evidence
Validation evidence for Phase 93 is the final successful run of `./scripts/check.sh`, explicit cargo tests, explicit boundary lints, explicit UI validation, CLI dry-run, evidence scans, source guard, readiness scan, lint wiring scan, staged-file review, and final working-tree hygiene checks.

## Confirmed vs suspected
Confirmed:
- Persisted record corruption rejects fail-closed.
- Durable append checksum drift, malformed revisions, stale revision chains, transaction-id mismatch, audit-only transactions, and ledger-only transactions reject.
- Failed writes and verification failures report `committed=false`.
- Recovery candidate mismatches reject without replacing global state, persisting, appending, or repairing replay.
- Export bundles are not ledger state, recovery input, or replay repair evidence.
- Phase 83, Phase 84, and Phase 89 success/non-authority postures remain intact.

Suspected:
- None within Phase 93 persistence/append/recovery/export scope.

## Deferred items
| Item | Reason |
| --- | --- |
| Replay drift repair tests beyond diagnostic non-repair posture | Phase 93 does not add replay repair behavior; Phase 94 covers provider/replay abuse hardening. |
| New lint prohibitions | No Phase 93 implementation drift required lint behavior changes. |

## Non-readiness statement
Phase 93 does not claim public usability, production readiness, Production Candidate approval, release-candidate readiness, production-candidate readiness, or production approval.
