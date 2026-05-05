---
truth_dimension: orientation
authority_level: advisory
mutation_path: readme_update
---

# Phase 84 - Recovery Candidate Acceptance Boundary

## Scope
Phase 84 defines explicit acceptance of a verified recovery candidate for in-memory use only.

## Recovery acceptance model
Acceptance is explicit and fail-closed through a typed Rust request/report gate.

## Relationship to Phase 74 recovery candidates
Phase 74 prepared candidates with `recovers_application_state=false`; Phase 84 accepts only verified candidates for in-memory use without state replacement.

## Relationship to Phase 83 durable append
Phase 83 governs durable append eligibility. Phase 84 does not append ledger/audit records.

## Explicit acceptance gate
`accept_recovery_candidate_for_in_memory_use(...)` requires acceptance id, expected recovery id, expected ledger record id, expected revision (optional), and candidate bytes.

## In-memory-only acceptance posture
Accepted reports may set `accepted_for_in_memory_use=true` only.

## Non-replacement boundary
Phase 84 does not replace global application state.

## Non-persistence and non-append boundary
Phase 84 does not persist and does not append ledger/audit.

## Non-replay-repair boundary
Phase 84 does not repair replay.

## Non-provider-trust boundary
Phase 84 does not trust/promote provider output and does not execute actions.

## Root integration-test coverage
Root integration smoke includes acceptance in-memory-only and non-authority checks through existing public API surfaces.

## Validation evidence
Validation executed with `./scripts/check.sh`, Rust all-target tests, lints, UI checks, dry-run, and boundary scans.

## Confirmed vs suspected
Confirmed: explicit acceptance gate and side-effect flags remain false.
Suspected: none.

## Deferred items
Phase 85 remains the planned roadmap/changelog alignment checkpoint.

## Non-readiness statement
Phase 84 does not approve public usability, release-candidate readiness, or production readiness.
