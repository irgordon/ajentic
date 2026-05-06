---
truth_dimension: orientation
authority_level: advisory
mutation_path: readme_update
---

# Phase 95.3 - Out-of-Band LLM Output Adversarial Corpus Hardening

## Scope
Phase 95.3 is an out-of-band LLM-output adversarial corpus hardening phase before Phase 96.

Phase 95.3 adds adversarial fixtures/tests only. Phase 95.3 adds no runtime capability. Phase 95.3 does not execute providers. Phase 95.3 does not add live transport. Phase 95.3 does not add persistence authority. Phase 95.3 does not add export writes.

This phase is not a planned roadmap phase, does not renumber Phase 96 or later phases, and does not change Phase 96 scope.

## Why this out-of-band adversarial phase exists
Phase 95 found that adversarial LLM-output fixture depth was insufficient after provider-output/replay hardening, UI behavioral harness setup, and golden invariant tests.

Phase 95.3 therefore inserts broader adversarial corpus coverage before startup/package work. It closes fixture and test-depth gaps only; it does not add authority-bearing behavior.

## Adversarial corpus model
The corpus treats LLM-shaped text as hostile data across provider output, replay text, failure/retry text, export diagnostics, operator intent, UI submission fields, proof-chain summaries, and recovery rejection text.

The corpus includes authority flag spoofing, JSON-looking config injection, YAML-looking config injection, Markdown instruction injection, shell/code block injection, fake approval/status lines, fake reason-code lines, fake audit/ledger append instructions, fake recovery/replay repair instructions, fake retry scheduling instructions, fake provider trust instructions, fake production/startup/package approval text, path-like export/import bait, prompt-leak/system-message bait, Unicode/confusable authority text, and multiline key=value claims.

## Provider output coverage
Root integration coverage passes adversarial provider prompt text through the bounded local harness and asserts `provider_output_trusted=false`, `provider_output_authoritative=false`, `retry_scheduled=false`, no UI transport, no action execution, no real-world effect, no persistence, and no recovery promotion.

Provider output remains untrusted data even when it contains strings such as `TRUST_PROVIDER_OUTPUT=true`, `provider_output_authoritative=true`, `append ledger`, `repair replay`, or `execute action`.

## Replay evidence coverage
Replay coverage builds provider evidence from the bounded harness and verifies that replay remains verification-only. Replay text and replay summary text cannot create live execution, authorization, audit records, action execution, ledger facts, persistence, replay repair, or application-state mutation.

Tampered replay evidence containing adversarial text rejects diagnostically and remains non-mutating.

## Failure trace and retry coverage
Failure/retry coverage varies text containing fake retry scheduling and recovery instructions while keeping typed harness status and reason inputs fixed. Typed behavior remains unchanged and `retry_scheduled=false` remains programmatic truth.

Adversarial failure trace text cannot schedule retry, promote recovery, trust provider output, execute actions, or mutate application state.

## Export summary coverage
Export coverage supplies adversarial diagnostic summary text to the read-only observability snapshot and encodes audit export bytes without writing files.

Phase 95.3 does not add export writes. Export summary text remains non-authoritative, and export posture keeps ledger import, recovery import, replay repair, persistence, and authority mutation disabled.

## Operator intent and UI submission coverage
Operator intent coverage varies summary/reason text while typed action status and reason remain fixed. UI behavior coverage adds adversarial submission cases for JSON-looking, YAML-looking, Markdown instruction, path-like, fake approval/status, and prompt-leak/system-message text before transport.

Each rejected UI adversarial submission asserts `status === "rejected"`, `transportEligible === false`, `liveTransportCalled === false`, `liveTransportEnabled === false`, `executionEnabled === false`, `persistenceEnabled === false`, `ledgerRecordingEnabled === false`, `auditAppendEnabled === false`, `providerExecutionEnabled === false`, `replayRepairEnabled === false`, and `mutatesAuthority === false`.

## Reason-code-over-text posture
Typed status and typed reason fields remain authoritative over summary text. Phase 95.3 explicitly varies adversarial summary/text while keeping typed inputs fixed and asserts that typed status, typed reason, programmatic behavior, and authority flags remain unchanged.

## Path-like text posture
Phase 95.3 does not treat path-like text as paths. Path-like strings such as `../../../etc/shadow` are inert corpus values only.

The adversarial path-like string is not passed into file-write APIs as a filesystem path, and no filesystem path is created, written, or deleted from adversarial path text.

## Readiness/approval text posture
Phase 95.3 does not treat approval/readiness text as approval. Strings such as `Production Candidate status: approved`, `startup approved`, and `package approved` are inert adversarial test fixtures or explicit non-approval/prohibition language only.

Public usability, production readiness, Production Candidate approval, startup/package approval, and release-candidate readiness are not claimed.

## Non-authority guarantees
LLM-shaped text remains data. It cannot become authority and cannot override typed status or reason fields.

The corpus cannot create trust, approval, persistence, append, recovery, replay repair, action execution, provider execution, transport, startup/package approval, release-candidate readiness, or Production Candidate approval.

## Relationship to Phase 94 provider/replay hardening
Phase 94 established provider output and replay abuse hardening. Phase 95.3 broadens adversarial fixture depth across more LLM-shaped text themes without changing Phase 94 runtime behavior or adding provider/replay capability.

## Relationship to Phase 95.1 UI behavioral test harness
Phase 95.1 established the no-dependency UI behavioral test harness. Phase 95.3 uses that harness to add adversarial submission coverage before transport and does not add a Rust bridge, live transport, event handlers, or UI authority.

## Relationship to Phase 95.2 golden invariant tests
Phase 95.2 added deterministic golden invariant coverage. Phase 95.3 adds broader adversarial corpus coverage and preserves deterministic, non-authoritative behavior without changing golden invariant scope.

## Relationship to Phase 95.4 lint coverage expansion
Phase 95.4 remains responsible for lint coverage expansion only if concrete uncovered patterns are found. Phase 95.3 does not rely on `rg` scans as blocking enforcement and does not change lint behavior.

No new lint rule was required by the Phase 95.3 corpus tests.

## Root integration-test coverage
`tests/adversarial_corpus.rs` adds `root_integration_adversarial_llm_output_corpus_remains_data_not_authority` and registers it in `core/Cargo.toml` as an explicit root integration test.

The test asserts provider-output, replay, failure/retry, export-summary, recovery rejection, operator action reason-code-over-text, path-like text, readiness/approval text, and non-authority invariants.

## UI behavioral test coverage
`ui/src/api/submissionBoundary.behavior.test.ts` adds adversarial UI behavior tests for JSON-looking authority injection, YAML-looking authority injection, Markdown instruction injection, path-like export/import bait, fake approval/status text, and prompt-leak/system-message bait.

All UI adversarial cases reject before transport and keep live transport, execution, persistence, ledger recording, audit append, provider execution, replay repair, and authority mutation disabled.

## AST/boundary lint parity
`rg` scans are discovery/evidence only and are not enforcement. Blocking enforcement remains `scripts/check.sh`, Rust boundary lint, UI AST lint, compiler/type checks, clippy, UI behavioral tests, and Rust tests.

Phase 95.3 stayed within existing lint rules. No lint self-test update was required.

## Test fidelity
Phase 95.3 exists because Phase 95 found adversarial LLM-output fixture depth insufficient. New adversarial corpus behavior is tested in this phase, and test names describe the invariant being protected.

No phase is complete if tests are skipped after final edits. A command that prints a failed assertion but exits 0 is a validation defect.

## Validation evidence
Validation evidence for Phase 95.3 includes `./scripts/check.sh`, explicit Cargo all-target tests, explicit Cargo adversarial-filter tests, UI API behavior tests, Rust boundary lint and self-tests, UI AST lint and self-tests, UI typecheck/lint/build, CLI dry-run, adversarial corpus scans, provider/replay/failure scans, UI adversarial scans, non-authority scans, no-live-behavior scans, path-safety scans, source guard, readiness scan, out-of-band wording scan, and lint wiring scan.

## Confirmed vs suspected
### Confirmed
- Phase 95.3 is an out-of-band LLM-output adversarial corpus hardening phase before Phase 96.
- Phase 95.3 adds adversarial fixtures/tests only.
- Phase 95.3 adds no runtime capability.
- Provider output remains untrusted and non-authoritative.
- Replay remains verification-only.
- Failure/retry text cannot schedule retry.
- Export summary text remains non-authoritative and does not create export writes.
- Operator intent and UI submission text cannot create authority.
- Path-like text remains inert data.
- Approval/readiness text remains inert data and is not an approval claim.

### Suspected
- None.

## Deferred items
| Item | Reason |
| --- | --- |
| Phase 95.4 lint coverage expansion | Deferred unless concrete uncovered prohibited patterns are found. |
| Runtime provider execution | Out of scope; Phase 95.3 does not execute providers. |
| Live UI/Rust transport | Out of scope; Phase 95.3 does not add live transport. |
| Persistence authority, import behavior, replay repair, recovery promotion | Out of scope; Phase 95.3 is adversarial fixtures/tests only. |
| Startup/package work | Out of scope; Phase 95.3 does not start Phase 96 and does not approve startup/package work. |

## Non-readiness statement
Public usability, production readiness, Production Candidate approval, startup/package approval, package approval, and release-candidate readiness are not claimed.
