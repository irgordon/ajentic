---
truth_dimension: orientation
authority_level: advisory
mutation_path: readme_update
---

# Phase 94 - Provider Output Injection and Replay Abuse Hardening

## Scope
Phase 94 is hardening only. It adds negative-path coverage for provider-output injection, replay tampering, failure trace spoofing, retry escalation attempts, and reason-code-over-text invariants.

Phase 94 adds no new provider authority, provider execution, replay repair, retry scheduler, persistence write, ledger append, audit append, recovery promotion, action execution, live transport, lifecycle transition, global state replacement, or authority mutation.

## Provider output injection model
Provider output text is treated as untrusted input. Risky strings such as `TRUST_PROVIDER_OUTPUT=true`, `provider_output_authoritative=true`, `schedule retry now`, `append ledger`, `append audit`, `persist this`, `recover global state`, `repair replay`, `execute action`, `mutate application state`, `new authorization created`, `new audit record created`, `new ledger fact created`, and `production approved` remain inert when supplied through local harness provider-prompt text.

## Replay tampering model
Replay verifies a supplied `ProviderEvidenceSnapshot` and recomputes checksum evidence before producing a report. Tampered checksum, source run mismatch, action kind mismatch, authority-flag mismatch, and missing evidence identifiers reject or mismatch diagnostically.

Replay does not rerun provider execution and does not create new authorization, audit records, action execution, ledger facts, persistence, replay repair, or application-state mutation.

## Failure trace spoofing model
Failure and reason text remain descriptive input only. Strings such as `retry eligible override`, `schedule retry now`, `recover global state`, and `TRUST_PROVIDER_OUTPUT=true` cannot change typed harness status, typed harness reason, provider trust flags, lifecycle posture, or retry scheduling flags.

## Retry escalation posture
Retry classification remains report-only and non-scheduling. Phase 94 does not add a retry scheduler, retry execution path, lifecycle transition, async worker, thread, process, network transport, or action execution path.

## Reason-code-over-text posture
Programmatic behavior depends on typed status and reason fields. Provider summary text, replay summary text, failure summary text, and risky text cannot override typed status/reason fields or non-authority flags.

## Provider output remains untrusted
Provider output remains untrusted and non-authoritative. Provider output text cannot trigger persistence, recovery promotion, replay repair, audit append, ledger append, action execution, provider execution, or authority mutation.

## Replay remains verification-only
Replay remains verification of supplied evidence only. Replay evidence cannot become live execution, cannot repair itself, and cannot create authorization, audit, action, ledger, persistence, recovery, or application-state mutation.

## Retry classification remains non-scheduling
Retry classification remains non-scheduling. Phase 94 proves spoof text cannot schedule retries through current public harness and replay reports.

## Non-authority guarantees
Phase 94 does not persist, append ledger or audit records, recover global state, execute actions, mutate application state, repair replay, run providers, create live UI/Rust transport, or create authority from text.

## Relationship to Phase 81 local harness hardening
Phase 94 preserves Phase 81 local harness hardening by keeping risky prompt/reason text neutralized and by preserving bounded, deterministic, side-effect-free local harness behavior.

## Relationship to Phase 82 provider evidence replay
Phase 94 preserves Phase 82 provider evidence replay success behavior while adding focused abuse tests proving replay tampering remains diagnostic-only and non-mutating.

## Relationship to Phase 93/93.5 persistence hardening
Phase 94 preserves Phase 93 and Phase 93.5 persistence/export non-authority posture. Provider output and replay text do not trigger durable append, export-as-ledger behavior, recovery acceptance, persistence repair, or replay repair.

## Relationship to Phase 95 roadmap/changelog alignment
Phase 95 remains responsible for roadmap/changelog alignment after the Phase 91-94 hardening block. Phase 94 does not update roadmap files.

## Root integration-test coverage
Root integration coverage was added for provider output injection remaining non-authoritative, replay tampering rejecting without side effects, and failure trace spoofing not scheduling retry. The current public APIs express these abuse cases without broad export reshaping.

## AST/boundary lint parity
Phase 94 does not rely on `rg` scans as enforcement. The scans are discovery/evidence only. Blocking enforcement remains `scripts/check.sh`, Rust boundary lint, UI AST lint, compiler/type checks, and Rust tests.

No lint behavior changed in Phase 94, so no lint self-test changes were required.

## Test fidelity
Hardening behavior added in Phase 94 is covered by tests in the same phase. Test names describe the invariants being protected. Cross-boundary behavior has root integration coverage through current public APIs.

## Validation evidence
Validation evidence includes `./scripts/check.sh`, explicit Cargo tests, explicit Rust/UI boundary lints, UI type/lint/build validation, CLI dry-run, provider injection scans, replay tampering scans, failure/retry spoofing scans, reason-code-over-text scans, no-authority scans, source guard, readiness scan, and lint wiring scan.

## Confirmed vs suspected
### Confirmed
- Phase 94 is hardening only.
- Provider output remains untrusted and non-authoritative.
- Replay remains verification-only and non-mutating.
- Tampered replay evidence rejects or mismatches diagnostically.
- Failure trace text cannot schedule retry or change typed lifecycle status.
- Retry classification remains non-scheduling.
- Risky text cannot override typed status/reason fields or non-authority flags.

### Suspected
- None.

## Deferred items
| Item | Reason |
| --- | --- |
| Retry runtime or scheduler | Out of scope; Phase 94 is hardening only and retry classification remains non-scheduling. |
| Replay repair | Out of scope; replay remains verification-only. |
| Provider status field expansion inside replay snapshots | Out of scope; current replay types cover source run, evidence id, checksum, action kind, reason code, and authority flags without broad replay reshaping. |
| New lint rules | Not required; no new prohibited Rust source pattern was needed for Phase 94 coverage. |

## Non-readiness statement
Public usability, production readiness, Production Candidate approval, and release-candidate readiness are not claimed.
