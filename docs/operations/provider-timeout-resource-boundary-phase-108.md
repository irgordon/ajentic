---
truth_dimension: orientation
authority_level: advisory
mutation_path: readme_update
---

# Phase 108 - Provider Timeout and Resource Limit Boundary

## Scope
Phase 108 enforces timeout/resource limits only at the deterministic provider sandbox boundary. It does not add remote provider execution, external API calls, cloud model inference, provider SDK execution, shell/process spawning, arbitrary file access, scheduler behavior, persistence authority, replay repair, recovery promotion, action execution, readiness approval, Production Candidate approval, release-candidate approval, public-usability approval, production-readiness approval, production-human-use approval, or Phase 109 implementation.

## Timeout/resource enforcement boundary
The Rust sandbox enforces declared provider limits from the Phase 106 provider configuration before or during Phase 107 deterministic local stub execution. Enforcement is fail-closed and deterministic.

## Descriptive-only evidence invariant
Resource-limit evidence is descriptive, not authoritative. Sandbox limit evidence records what was declared, observed, truncated, terminated, or rejected; it never grants trust, promotion, persistence, workflow approval, or readiness.

## Declared-limit posture
The declared snapshot records `timeout_ms`, `max_prompt_bytes`, `max_context_bytes`, retry-disabled posture, and limit-escalation-disabled posture from the accepted provider configuration.

## Sandbox enforcement posture
The sandbox remains deterministic-local-stub-only. Provider output remains untrusted candidate data even when execution succeeds within limits.

## Timeout enforcement behavior
Timeout exhaustion is represented by deterministic elapsed-work accounting. If observed deterministic elapsed milliseconds exceed the declared timeout, execution terminates with the `timeout_limit_exceeded` rejection reason and no provider output.

## Resource-limit enforcement behavior
Prompt bytes that exceed `max_prompt_bytes` reject fail-closed with `prompt_resource_limit_exceeded`. Output is bounded to `max_context_bytes`; invalid zero-sized output limits are rejected through configuration validation.

## Deterministic truncation posture
Output truncation occurs only after deterministic local stub output is built and before provider output is returned. Truncation keeps byte and character boundaries stable and records `truncated_output_to_declared_context_limit`.

## Deterministic termination posture
Timeout termination happens before output is returned. Terminated reports keep provider output absent and record a deterministic reason code.

## Sandbox limit evidence model
`SandboxLimitEvidence` is typed descriptive evidence containing declared limits, observed usage, enforcement status, decisions, bounded summaries, and explicit no-trust/no-promotion/no-persistence/no-readiness indicators.

## Declared limit snapshot model
`DeclaredProviderLimitSnapshot` records declared timeout, prompt limit, context limit, retries disabled, and limit escalation disabled.

## Observed usage model
`ObservedSandboxUsage` records input bytes, planned output bytes, emitted output bytes, deterministic elapsed milliseconds, retry attempts, and limit-escalation attempts.

## Enforcement-status model
`SandboxLimitEnforcementStatus` distinguishes within-limit execution, output truncation, timeout termination, resource-limit rejection, and unsafe retry/limit-escalation rejection.

## Provider output untrusted posture
Successful execution does not imply provider trust. Provider output remains untrusted candidate data.

## No-promotion guarantee
Phase 108 does not promote provider output or add trust-granting evidence.

## No-persistence guarantee
Phase 108 does not add persistence authority, durable append authority, export authority, or evidence persistence as authority.

## No-readiness guarantee
Phase 108 does not approve readiness, Production Candidate status, release-candidate readiness, production readiness, public usability, or production human use.

## Retry prohibition
Retry-shaped requests are rejected with deterministic retry-disabled evidence. The sandbox does not silently retry.

## Limit-escalation prohibition
Limit-escalation-shaped requests are rejected with deterministic limit-escalation-disabled evidence. The sandbox does not silently expand, auto-tune, or infer limits.

## Behavioral test coverage
Behavioral tests cover deterministic timeout enforcement, prompt resource rejection, deterministic output truncation ordering, repeated execution determinism, descriptive-only evidence, successful execution not implying trust/readiness/persistence, disabled retries, and disabled limit escalation.

## Adversarial test coverage
Adversarial tests cover oversized inputs, oversized prompt limits, timeout exhaustion payloads, resource exhaustion payloads, retry-shaped requests, limit-escalation requests, trust injection, readiness text, persistence text, replay/recovery/action text, malformed limit declarations, malformed execution requests, and hostile/noise payloads.

## Relationship to Phase 106 provider configuration contract
Phase 106 declares provider resource limits and rejects malformed declarations. Phase 108 consumes those declarations as bounded input to deterministic sandbox enforcement.

## Relationship to Phase 107 provider execution sandbox
Phase 107 introduced deterministic local stub execution with untrusted output. Phase 108 keeps that posture and adds timeout/resource enforcement and descriptive evidence.

## Relationship to Phase 109 durable persistence authority decision gate
Phase 109, if recommended, is the next planned durable persistence authority decision gate only. Phase 108 does not implement Phase 109.

## Required future implementation evidence
Future phases must provide committed source, tests, operations docs, checklist, changelog, and validation logs for any persistence authority decision or activation. Roadmap intent alone is not implementation evidence.

## Phase 109 gate decision
Phase 109 may begin next as the planned durable persistence authority decision gate only. It is not implemented here.

## Production Candidate status
Production Candidate status: not approved.

## Release-candidate readiness status
Release-candidate readiness: not approved.

## Public/general use status
Public usability, production readiness, and production human use: not approved.

## Roadmap/changelog truth posture
Roadmap remains planned truth. CHANGELOG surfaces remain historical truth.

## Required follow-ups
| Follow-up | Reason |
| --- | --- |
| Phase 109 decision gate | Decide durable persistence authority before any activation. |
| Continue no-remote scans | Preserve absence of remote/cloud/provider-SDK execution. |
| Continue no-authority scans | Preserve absence of persistence, replay, recovery, action, and readiness authority. |

## Deferred items
| Item | Deferred to |
| --- | --- |
| Durable persistence authority decision | Phase 109 |
| Persistence authority activation | Phase 110 only if Phase 109 permits |
| Remote/cloud provider execution | Not approved by Phase 108 |

## Confirmed vs suspected
| Item | Status |
| --- | --- |
| Timeout/resource enforcement code | Confirmed in committed Rust source. |
| Descriptive-only evidence | Confirmed in typed sandbox evidence and tests. |
| Provider output trust or promotion | Confirmed absent. |
| Remote/cloud provider execution | Confirmed absent from Phase 108 implementation. |
| Production readiness | Not approved. |

## Non-readiness statement
Phase 108 is provider timeout/resource hardening only. It does not approve Production Candidate status, release-candidate readiness, production readiness, public usability, production human use, or any provider-output trust/promotion/persistence authority.
