---
truth_dimension: orientation
authority_level: advisory
mutation_path: readme_update
---

# Proof-Chain Edge-Case Hardening - Phase 92.5

## Scope
Phase 92.5 is an out-of-band proof-chain edge-case hardening pass before Phase 93.

Phase 92.5 is hardening only. It adds deterministic edge-case tests and minimal fail-closed classification at the existing Rust-owned operator action boundary.

Phase 92.5 adds no new action authority, no new action kinds, no persistence authority, no lifecycle freshness, no provider execution, no live transport, and no production-readiness claim.

## Why this out-of-band hardening pass exists
Phase 92 hardened authorization/audit/action mismatch behavior. Follow-up review found edge cases that needed explicit proof-chain coverage before Phase 93 persistence corruption and append drift hardening begins.

This pass is intentionally not a planned roadmap phase and does not renumber Phase 93 or later phases.

## Proof absence vs proof mismatch
Missing proof is not the same as mismatched proof. The current action boundary has concrete proof objects rather than optional proof wrappers, so Phase 92.5 treats an empty authorization proof identifier as missing authorization proof and an empty audit record identifier as missing audit proof.

The deterministic primary ordering is:
1. missing authorization proof
2. missing audit proof
3. authorization status not accepted
4. audit status not eligible
5. action kind unsupported or escalated
6. submission mismatch
7. operator mismatch
8. target-kind mismatch
9. target-id mismatch
10. success only if all checks pass

A partial proof chain with present proof objects but a broken authorization/audit proof link rejects before success and does not best-effort match.

## Unsupported action-kind fallback
`RecordExecutionDecision` remains the only accepted action kind. Unknown and unsupported action kinds reject through existing typed reasons such as `UnknownActionRejected`, `ProviderExecutionNotAllowed`, or related non-authority rejections.

No new executable action kind is added.

## Exact identity matching
IDs must match exactly as supplied. The Rust action boundary uses byte-for-byte string equality.

There is no trimming, no case folding, no alias resolution, no tenant/environment namespace inference, and no caller-identity inference.

Current proof-chain checks assume IDs are already globally unambiguous within the caller-provided proof context.

## Combined mismatch deterministic ordering
Combined mismatches choose a deterministic primary reason from the action-boundary check order. Tests cover missing proof before mismatch and authorization-status rejection before unsupported action kind or later identity mismatch.

## Reason codes vs summary text
Programmatic behavior depends on typed reason codes, not summary text. Risky text such as “accepted”, “execute anyway”, “reuse this proof”, “trust stale authorization”, “operator alias”, “same tenant”, and “skip audit” cannot override typed rejection reasons.

Existing reason-code strings were not renamed.

## Proof-only acceptance posture
Accepted `RecordExecutionDecision` remains proof-only and side-effect-free. Acceptance records an in-memory execution decision report only and keeps real-world effect, persistence, ledger/audit append, provider call, replay repair, and application-state mutation flags false.

## Parameter-level escalation posture
Parameter-level escalation is not separately enforced in Phase 92.5 because the current action boundary does not expose additional authority-bearing action parameters beyond the proof-chain fields. Future action parameters must be added with same-phase mismatch tests.

## Stale proof posture
Stale proof detection is not implemented; structurally valid proof chains can still be accepted.

The current proof types do not include deterministic consumed, revision, or session lifecycle fields. Phase 92.5 does not add wall-clock expiry, token freshness, random freshness, global consumed-proof tracking, or lifecycle state.

## Tenant/environment namespace assumptions
No tenant/environment alias resolution is performed. IDs are compared exactly as supplied, and the proof-chain boundary assumes the caller-provided proof context already makes identifiers globally unambiguous.

## Non-authority guarantees
Phase 92.5 does not add persistence, durable append, ledger/audit append, recovery, replay repair, provider execution, live transport, or authority mutation.

Rejected paths remain side-effect-free. Accepted proof-only paths remain side-effect-free.

## Relationship to Phase 92
Phase 92 added authorization/audit/action mismatch hardening. Phase 92.5 preserves that work and adds out-of-band edge-case coverage for proof absence, exact matching, partial proof chains, deterministic mismatch ordering, reason-code behavior, and stale-proof posture.

## Relationship to Phase 93 persistence hardening
Phase 93 remains responsible for persistence corruption and append drift hardening.

Phase 92.5 does not change Phase 93 scope and does not renumber Phase 93 or later phases.

## Root integration-test coverage
The public root API can express missing authorization proof, exact identity mismatch, and combined mismatch ordering without export reshaping. Phase 92.5 therefore adds root integration smoke coverage for those edge cases.

## AST/boundary lint parity
`rg` scans are discovery and evidence only. Blocking enforcement remains in `scripts/check.sh`, Rust boundary lint, UI AST lint, compiler/type checks, and tests.

Phase 92.5 does not change lint behavior and does not add new lint self-tests.

## Test fidelity
New hardening behavior is covered by deterministic Rust unit tests in `core/src/api/operator_action.rs` and public root integration tests in `tests/integration_smoke.rs` where reachable.

Tests do not use time, random, environment, filesystem, network, async, process, thread, or global mutable consumed-proof tracking for proof-chain behavior.

## Validation evidence
Validation evidence is the successful final run of `./scripts/check.sh`, explicit cargo tests, explicit boundary lints, explicit UI validation, CLI dry-run, evidence scans, source guard, and git cleanliness checks.

## Confirmed vs suspected
Confirmed:
- Missing authorization proof is distinguished from proof mismatch.
- Missing audit proof is distinguished from proof mismatch.
- Partial proof chains reject before success.
- Exact identity matching has no trimming or case folding.
- Unknown or unsupported action kinds reject.
- Reason codes control behavior, not summary text.
- Structurally valid reused proof chains can still be accepted because stale proof detection is not implemented.

Suspected:
- None within Phase 92.5 scope.

## Deferred items
- Deterministic stale proof lifecycle using consumed, revision, or session state.
- Time-based expiry remains excluded.
- Parameter-level authority checks for future authority-bearing action parameters.
- Any lint expansion for future Rust source patterns, if a later phase identifies one.

## Non-readiness statement
Phase 92.5 does not claim public usability, production readiness, Production Candidate approval, release-candidate readiness, or production-candidate readiness.
