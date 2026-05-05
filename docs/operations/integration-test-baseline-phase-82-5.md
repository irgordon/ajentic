---
truth_dimension: orientation
authority_level: advisory
mutation_path: readme_update
---

# Phase 82.5 - Out-of-Band Root Integration Test Harness Baseline

## Scope
Phase 82.5 is an out-of-band maintenance/testing fix before Phase 83. This phase adds a root integration-test baseline for existing cross-boundary local harness and replay invariants only.

## Why this out-of-band maintenance/testing phase exists
Phase 82 completed bounded harness and replay evidence behavior, but the repository root `tests/` surface did not yet contain a compile/run integration baseline. Phase 82.5 inserts that baseline before Phase 83 so executable cross-boundary coverage is not deferred.

## Root integration-test role
Root `tests/` is for cross-boundary integration tests only. These tests validate composed behavior across existing boundaries without introducing new authority or mutating behavior.

## Boundary between unit tests and integration tests
Module-local unit tests remain the place for focused enum/helper behavior. Root integration tests are for cross-boundary behavior that composes existing module-owned surfaces.

## Test coverage added
Added root integration checks for bounded local harness completion, non-authority flags, deterministic action-kind/effect boundary, and replay-mode distinction from live execution.

## Library/export surface assessment
No new runtime module semantics were added. Existing `ajentic_core` exports already exposed required API functions. `core/Cargo.toml` adds only a minimal test target path so root `tests/integration_smoke.rs` is compiled and run by existing Rust test commands.

## Relationship to Phase 83 durable append
Phase 82.5 is an out-of-band maintenance/testing fix before Phase 83. Phase 82.5 does not implement durable append. Phase 83 remains responsible for durable audit/ledger append.

## Non-authority guarantees
Phase 82.5 does not add runtime authority. No provider network execution, persistence append/write behavior, recovery acceptance, live UI transport, or action side effects are introduced.

## Validation evidence
Validation executed `./scripts/check.sh`, explicit Cargo all-target tests, root test discovery scan, integration content scan, no-append/no-authority scan, guarded-source diff scan, readiness wording scan, out-of-band wording scan, and lint wiring scan.

## Confirmed vs suspected
Confirmed: root integration baseline compiles/runs, preserves bounded non-authority behavior, and preserves replay-vs-live distinction.
Suspected: none.

## Deferred items
Durable audit/ledger append remains deferred to Phase 83. Recovery candidate acceptance remains deferred to Phase 84.

## Non-readiness statement
Public usability, production readiness, and release-candidate readiness are not claimed.
