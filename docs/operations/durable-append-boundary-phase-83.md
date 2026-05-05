---
truth_dimension: orientation
authority_level: advisory
mutation_path: readme_update
---

# Phase 83 - Durable Audit and Ledger Append Boundary

## Scope
Phase 83 introduces a single durable append transaction boundary for combined audit and ledger proof bytes.

## Durable append transaction model
The append payload includes `append_transaction_id`, `audit_record_id`, `ledger_record_id`, `prior_revision`, `next_revision`, `audit_payload_hex`, `ledger_payload_hex`, and a checksum over combined content.

## Single-envelope atomicity posture
Phase 83 uses one combined append transaction envelope. Audit and ledger proof bytes are not appended as independent physical commits.

## Relationship to Phase 61 persistence
Physical write uses only `execute_local_persistence_plan(...)`.

## Relationship to Phase 62 verification
Append is considered committed only after the combined envelope verifies through persisted-record verification and transaction-byte validation.

## Relationship to Phase 82 replay evidence
Replay evidence remains separate, non-authoritative, and unchanged by append behavior.

## Audit and ledger co-commit rule
Audit-only and ledger-only transactions are rejected. A partial append is not authoritative.

## IO failure and partial-commit handling
Write failures and verification failures return rejected reports with `committed=false`; no recovery acceptance or repair behavior is introduced.

## Non-promotion and non-recovery boundary
Phase 83 does not promote, recover, repair replay, trust provider output, execute actions, or mutate application state.

## Root integration-test coverage
Durable append helpers are module-local in `core/src/api/persistence.rs`; root integration coverage remains deferred until exported through an existing boundary.

## Validation evidence
Validation includes repository checks, Rust/UI lint flows, cargo tests, dry-run, and boundary scan commands.

## Confirmed vs suspected
Confirmed: single-envelope durable append encoding and verification gate are implemented.
Suspected: none.

## Deferred items
Recovery candidate acceptance remains Phase 84. Alignment checkpoint remains Phase 85.

## Non-readiness statement
This phase does not claim public usability, release-candidate readiness, or production readiness.
