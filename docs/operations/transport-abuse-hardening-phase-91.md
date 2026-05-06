---
truth_dimension: orientation
authority_level: advisory
mutation_path: readme_update
---

# Phase 91 - Transport Abuse and Submission Spoofing Hardening

## Scope

Phase 91 is hardening only. It adds a local TypeScript submission boundary helper and static fixture payloads for malformed submissions, risky operator text, and spoofed capability flags.

Phase 91 does not add live transport. Phase 91 does not add a Rust bridge. Phase 91 does not make submissions executable. Phase 91 does not add runtime authority.

## Transport abuse model

The abuse model is a UI-origin object that attempts to present itself as transport-capable, live, persistent, executable, audit-appending, ledger-recording, provider-executing, replay-repairing, or authority-mutating.

The hardened boundary treats all user-supplied capability and authority flags as untrusted. A payload containing such flags is rejected before any bridge eligibility can be represented.

## Submission spoofing model

The spoofing model includes malformed submission-shaped objects, empty operator identity, empty target identity, empty intent kind, unknown intent kind, risky authority-escalation text, and explicit capability flag spoofing.

Risky text examples covered by static fixtures include `admin override`, `skip policy`, `execute now`, `write ledger`, `append audit`, `repair replay`, `trust provider output`, and `promote recovered state`.

## Contract gate before bridge

`buildUiSubmissionBoundaryResult` is the local TypeScript contract gate. It accepts unknown input and returns either `accepted_for_preview` or `rejected`.

The gate never creates a sendable envelope. Both accepted preview and rejected results report `transportEligible=false`, `liveTransportCalled=false`, `liveTransportEnabled=false`, `executionEnabled=false`, `persistenceEnabled=false`, `ledgerRecordingEnabled=false`, `auditAppendEnabled=false`, `providerExecutionEnabled=false`, `replayRepairEnabled=false`, and `mutatesAuthority=false`.

## Malformed submission handling

Malformed input is rejected with `malformed_submission_rejected`. Empty `operatorId`, empty `targetId`, empty intent kind, and unknown intent kind are rejected with dedicated reason codes.

Malformed or rejected results are not bridge-eligible and do not imply live Rust transport.

## Spoofed capability flag handling

User-supplied authority/capability flags are not trusted. Payloads that include `liveTransportEnabled`, `executionEnabled`, `persistenceEnabled`, `ledgerRecordingEnabled`, `auditAppendEnabled`, `providerExecutionEnabled`, `replayRepairEnabled`, `mutatesAuthority`, or the prior UI state mutation spelling are rejected before preview acceptance.

The required spoof payload fixture attempts `executionEnabled: true`, `liveTransportEnabled: true`, `persistenceEnabled: true`, `ledgerRecordingEnabled: true`, `auditAppendEnabled: true`, `providerExecutionEnabled: true`, `replayRepairEnabled: true`, and `mutatesAuthority: true`.

## Stubbed bridge non-call guarantee

There is no stubbed bridge call in Phase 91. The boundary result always reports `liveTransportCalled=false` and no helper invokes transport, Rust, persistence, provider execution, replay repair, ledger recording, audit append, or action execution.

## UI test coverage or test-harness gap

No UI unit test harness exists in the repository: `ui/package.json` only defines placeholder lint/build commands and a TypeScript no-emit typecheck. Phase 91 therefore does not add a fake UI test command, does not add dependencies, and does not modify package or lock files.

Phase 91 behavioral intent is represented by the local contract helper and static fixtures, and validation uses the existing UI typecheck, UI AST lint, placeholder lint/build commands, full repository check, Rust tests, boundary lints, scans, and CLI dry-run. Typecheck/lint/build are not claimed as behavioral test coverage.

## Relationship to Phase 76 transport contract

Phase 76 established UI/Rust transport-shaped surfaces as display-only and non-live. Phase 91 preserves that contract by keeping transport capabilities disabled and ensuring malformed or spoofed submission input cannot become transport-eligible.

## Relationship to Phase 77 submission shaping

Phase 77 established local submission shaping for operator intent preview. Phase 91 keeps that shaping local and non-executing while adding a stricter gate for malformed objects and spoofed flags.

## Relationship to Phase 92 authorization/audit/action hardening

Phase 92 remains responsible for authorization/audit/action mismatch hardening. Phase 91 does not resolve stale proof reuse, identity mismatch beyond local empty operator/target checks, action-kind escalation, or audit/action proof mismatch beyond rejecting spoofed capability and authority flags before bridge eligibility.

## AST/boundary lint parity

Phase 91 did not require changes to `scripts/lint_ui_boundaries.mjs` or `scripts/test_lint_ui_boundaries.mjs`. Existing UI AST lint rules continue to block live transport/event surfaces such as network calls, browser storage, timers, event handlers, forms, buttons, and forbidden modules.

`rg` scans are discovery/evidence only. Blocking enforcement remains `scripts/check.sh`, Rust boundary lint, UI AST lint, compiler/type checks, and available tests.

## Test fidelity

New TypeScript behavior would normally require UI behavioral tests in the same phase. Because no UI unit test harness exists and dependency/package changes were prohibited, Phase 91 documents the gap instead of pretending typecheck/lint/build are behavioral tests.

The test-harness gap is deferred to Phase 95 or an out-of-band testing phase. Phase 91 still runs the existing full validation gate after final edits.

## Validation evidence

Final validation evidence is recorded in `checklists/current-phase.md` and in the Phase 91 changelog entry. Required validation includes `./scripts/check.sh`, explicit Rust tests, explicit boundary lints, explicit UI validation, CLI dry-run, transport hardening scans, no-live-transport scans, UI authority scans, source guard, readiness scan, and lint wiring scan.

## Confirmed vs suspected

| Item | Status | Evidence |
| --- | --- | --- |
| Phase 91 title and scope | confirmed | Roadmap files list Phase 91 as transport abuse and submission spoofing hardening with no live transport. |
| UI unit test harness absent | confirmed | `ui/package.json` has no test script and no test config/files are present. |
| TypeScript boundary rejects malformed input before bridge eligibility | confirmed by contract implementation | The helper returns rejected results with `transportEligible=false` and all live/execution/persistence flags false. |
| Spoofed capability flags are rejected, not trusted | confirmed by contract implementation | The helper rejects user-supplied capability/authority flag properties with dedicated reason codes. |
| AST/boundary lint behavior change needed | not found | Existing UI AST lint passed and no new source pattern required lint changes. |

## Deferred items

| Deferred item | Owner |
| --- | --- |
| UI behavioral unit test harness for contract helpers | Phase 95 or out-of-band testing phase |
| Authorization/audit/action mismatch hardening | Phase 92 |
| Persistence corruption and append drift hardening | Phase 93 |
| Provider output injection and replay abuse hardening | Phase 94 |
| Hardening outcome realignment | Phase 95 |

## Non-readiness statement

Public usability, production readiness, Production Candidate approval, and release-candidate readiness are not claimed. Phase 91 is hardening-only evidence and does not approve release, production, public operation, live transport, executable submissions, or runtime authority.
