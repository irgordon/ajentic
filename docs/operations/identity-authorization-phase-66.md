---
truth_dimension: orientation
authority_level: advisory
mutation_path: readme_update
---
# Identity Authorization - Phase 66

## Scope
Phase 66 adds a Rust-owned identity/safety/context authorization boundary for operator intent requests.

## Identity-bound authorization model
`core/src/api/authorization.rs` introduces typed request and decision models and deterministic fail-closed authorization checks.

## Safety-context checks
Authorization denies unsafe runtime posture when provider network, file IO, or UI mutation permissions are enabled.

## Target-context checks
Authorization requires target kind/id to match the validated submission target context exactly.

## Relationship to operator intent ingress
Authorization composes with `OperatorIntentSubmission` and `OperatorIntentIngressReport` and requires accepted ingress plus route presence.

## Non-execution boundary
Authorization is not execution.
AuthorizedForFutureExecution does not execute an action.
Phase 66 does not write audit/ledger records.

## Structural-risk constraint
`core/src/execution/mod.rs` was not expanded.
Authorization logic is isolated to `core/src/api/authorization.rs` and re-exported by `core/src/api/mod.rs` only.

## Diagnostic coverage
Added an `OperatorAuthorization` diagnostics family and mapping with authorization reason diagnostic helper.

## Validation evidence
Phase validation commands and scans were run, including repo checks, UI checks, Rust dry-run, API scans, authorization/no-execution scans, and execution-module guard scan.

## Deferred items
Phase 67 remains responsible for intent audit record boundary.
No execution, persistence, provider/model invocation, or replay repair behavior is introduced in this phase.

## Confirmed vs suspected
### Confirmed
- Authorization decisions are typed, deterministic, and fail-closed.
- Authorization never executes actions in Phase 66.
- Authorization keeps execution disabled (`execution_enabled=false`) even when authorized.

### Suspected
- Future phases may use `IntentExecutionNotEnabled` on denial paths for variant rollout, but Phase 66 authorizes only for future execution metadata.

## Non-readiness statement
Phase 66 does not claim release-candidate readiness, production readiness, or public usability.
