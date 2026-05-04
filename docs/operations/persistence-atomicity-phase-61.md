---
truth_dimension: orientation
authority_level: advisory
mutation_path: readme_update
---
# Phase 61 - Data Durability and Atomic Persistence Implementation

## Scope
Phase 61 implements physical local persistence only through `execute_local_persistence_plan(...)` in `core/src/api/persistence.rs`.

## Atomic persistence boundary
The only physical write boundary is `execute_local_persistence_plan(...)` using a typed `LocalPersistencePlan`.

## Write path
`temp_path` is opened/created for write, payload bytes are written, file is flushed and synced, then `std::fs::rename(temp_path, target_path)` is attempted.

## Validation behavior
Plan validation still runs first via `validate_local_persistence_plan(...)` and invalid plans fail closed as `InvalidPlan`. Empty payload fails closed as `EmptyPayload`.

## Failure behavior
Write, flush/sync, and rename failures fail closed with typed error codes. `CreateNew` fails with `TargetAlreadyExists` when the target exists.

## Non-capabilities
No serialization, no payload inspection, no payload-kind inference, no automatic persistence, no parent-directory creation, no path canonicalization, no hidden writes.

## Dry-run isolation
Dry-run remains deterministic and in-memory with no persistence call path.

## Local workflow isolation
Local workflow remains in-memory and does not call persistence execution.

## Deferred recovery and corruption detection
Phase 61 adds physical atomic-write semantics only. Recovery and corruption-detection behavior remain deferred to Phase 62.

## Validation evidence
Validation evidence is tracked in `checklists/current-phase.md` and command logs for check/lint/build/test and static scans.

## Confirmed vs suspected
Confirmed: physical local write boundary is explicit and typed. Suspected/deferred: cross-filesystem rename edge semantics and broader recovery policies.

## Non-readiness statement
This phase does not claim release-candidate readiness, production readiness, or public usability.
