---
truth_dimension: orientation
authority_level: advisory
mutation_path: readme_update
---
# Phase 107 - Provider Execution Sandbox Boundary

## Scope
Phase 107 introduces only bounded deterministic local stub provider execution. It adds typed execution requests, typed execution reports, deterministic stub output, deterministic rejection reasons, bounded input/output summaries, and explicit sandbox posture indicators.

Phase 107 does not add remote provider execution, external API calls, cloud model inference, streaming execution, provider fallback, provider auto-selection, provider output trust, provider output promotion, persistence authority, durable append authority, export authority, replay repair, recovery promotion, action execution, readiness approval, Production Candidate approval, release-candidate approval, production readiness approval, public usability approval, production human-use approval, or Phase 108 implementation.

## Provider execution sandbox boundary
The sandbox boundary permits only `DeterministicLocalStub` execution requests with a Phase 106-valid local-only provider configuration. Every other execution shape fails closed before output is produced.

The boundary does not call remote APIs, invoke provider SDKs, open network transports, execute commands, invoke shells, read arbitrary files, write files, start background workers, schedule retries, persist execution reports as authority, append audit or ledger records, execute operator actions, repair replay, promote recovery, or approve workflow/readiness states.

## Deterministic local stub execution posture
The only successful execution path is deterministic local stub behavior implemented in Rust. The stub derives output from the provider identifier, bounded input summary, and deterministic input checksum. Repeating the same request returns the same report.

## Remote/cloud execution prohibition
Remote and cloud execution are explicitly rejected. Phase 107 does not add remote provider execution. Phase 107 does not call external APIs. Phase 107 does not perform cloud model inference. Remote/provider-network posture indicators remain disabled in success and rejection reports.

## Provider output untrusted posture
Provider output remains untrusted candidate data. The execution report marks output trust as `UntrustedCandidateData`; it never marks output trusted, authoritative, accepted, promoted, or clean.

## Execution report model
The execution report records status, deterministic rejection reasons, sandbox posture, output trust, optional output, deterministic metadata, bounded input summary, bounded output summary, disabled remote/network/streaming/fallback/auto-selection indicators, no-promotion/no-persistence/no-action indicators, replay/recovery isolation indicators, and authority-mutation booleans fixed to false.

## Execution request model
The execution request records execution id, typed provider configuration, execution kind, input, and explicit allow flags for remote execution, network use, streaming, fallback, auto-selection, shell/process use, file access, persistence, promotion, action execution, replay repair, and recovery promotion.

## Execution response model
The response is the typed execution report. Successful responses include deterministic local stub output as untrusted candidate data. Rejected responses include no provider output and deterministic rejection reason codes.

## Sandbox posture indicators
Reports state deterministic-local-stub-only posture, disabled remote execution, disabled provider network, disabled streaming, disabled fallback, disabled auto-selection, no promotion, no persistence, no action execution, no replay repair, and no recovery promotion.

## No-promotion guarantee
Phase 107 does not trust provider output and does not promote provider output. Reports keep `promoted_provider_output=false`, `no_promotion=true`, and `mutates_authority=false`.

## No-persistence guarantee
Phase 107 does not add persistence authority, durable append authority, export authority, audit append authority, or ledger append authority. Reports keep persistence and append indicators false.

## No-action-execution guarantee
Phase 107 does not execute operator actions. Requests that ask for action execution reject as unsafe execution requests. Reports keep `executed_action=false`.

## Replay/recovery isolation posture
Phase 107 does not add replay repair or recovery promotion. Requests that ask for replay repair or recovery promotion reject as unsafe execution requests. Reports keep replay and recovery mutation indicators false.

## Provider configuration relationship
Phase 107 preserves Phase 106 provider configuration validation by validating the supplied provider configuration through the existing Rust-owned provider configuration contract before sandbox execution. Invalid provider configuration rejects with `invalid_provider_configuration`.

## Transport hardening relationship
Phase 107 preserves Phase 105 transport hardening by adding no live transport expansion, no browser network authority, no remote ingress, no streaming channel, and no transport bypass.

## Rejection behavior
Rejections are fail-closed and deterministic. Rejected reports contain no provider output and keep all authority, persistence, promotion, action, replay, recovery, readiness, remote, network, streaming, fallback, and auto-selection indicators disabled.

## Invalid-provider-config behavior
Invalid provider identifiers, invalid isolation, invalid resources, execution-enabled configuration, transport-enabled configuration, trust-enabled configuration, readiness claims, auto-selection configuration, and fallback configuration remain rejected by the Phase 106 contract. Phase 107 maps invalid configuration to `invalid_provider_configuration` at execution time.

## Unsafe-execution-request behavior
Requests that ask for streaming, shell/process execution, arbitrary file access, persistence, promotion, action execution, replay repair, or recovery promotion reject with `unsafe_execution_request`.

## Remote-provider rejection behavior
Requests that select remote/cloud execution, allow remote execution, or allow provider-network execution reject with `remote_provider_rejected`.

## Fallback/auto-selection rejection behavior
Requests that allow provider fallback reject with `fallback_rejected`. Requests that allow provider auto-selection or request an auto-selected provider reject with `auto_selection_rejected`. Phase 107 does not silently fall back to another provider.

## Provider-output injection behavior
Provider-output text that asks to approve readiness, trust output, promote output, execute actions, repair replay, promote recovery, persist state, append audit/ledger records, export data, stream output, read files, write files, run shells, or use remote providers remains inert candidate data.

## Behavioral test coverage
Behavioral tests cover successful deterministic local stub execution as untrusted candidate output, repeated execution determinism, invalid provider configuration rejection, execution-enabled configuration remaining rejected/untrusted, remote provider rejection, fallback rejection, auto-selection rejection, unsafe request rejection, and provider output inability to mutate authority.

## Adversarial test coverage
Adversarial tests cover remote-provider execution payloads, cloud-provider execution payloads, provider fallback payloads, provider auto-selection payloads, provider-output authority injection, readiness approval text, action execution text, replay repair text, recovery promotion text, persistence/export text, shell/process execution payloads, file read/write payloads, streaming execution payloads, oversized provider input, malformed execution requests, and hostile/noise execution requests.

## Relationship to Phase 106 provider configuration contract
Phase 106 remains the provider configuration contract. Phase 107 consumes that contract as a precondition for the deterministic local stub sandbox and does not loosen Phase 106 validation.

## Relationship to Phase 108 provider timeout and resource limits
Phase 108, if recommended, is the next planned provider timeout and resource-limit boundary only. Phase 107 does not implement Phase 108 timeout enforcement, resource enforcement, scheduling, background execution, streaming, provider hardening beyond this sandbox boundary, or authority decisions.

## Required future implementation evidence
Future phases need committed timeout/resource-limit enforcement evidence, additional sandbox hardening evidence, persistence authority decision evidence, deployment evidence, security review evidence, and controlled human-use evidence before any production-readiness claim can be considered.

## Phase 108 gate decision
Phase 108 may begin next only as the planned provider timeout and resource-limit boundary. Phase 107 does not implement Phase 108.

## Production Candidate status
Production Candidate status is not approved.

## Release-candidate readiness status
Release-candidate readiness is not approved.

## Public/general use status
Public usability, production readiness, and production human use are not approved.

## Roadmap/changelog truth posture
Roadmap remains planned truth. CHANGELOG surfaces remain historical truth. This operations document is advisory orientation and does not supersede governance, roadmap, changelog, source, tests, schemas, or validation evidence.

## Required follow-ups
| Follow-up | Reason |
| --- | --- |
| Phase 108 timeout/resource-limit boundary | Planned next provider hardening boundary only. |
| Continued no-remote-execution validation | Needed to keep provider execution local and deterministic. |
| Continued no-authority validation | Needed to prove output remains untrusted and unpromoted. |

## Deferred items
| Item | Deferred to |
| --- | --- |
| Remote provider execution | Not approved; no phase approved here. |
| Cloud model inference | Not approved; no phase approved here. |
| Streaming provider output | Not approved in Phase 107. |
| Provider fallback or auto-selection | Not approved in Phase 107. |
| Timeout/resource runtime enforcement | Phase 108 if started. |
| Persistence, replay repair, recovery promotion, action execution | Not approved in Phase 107. |

## Confirmed vs suspected
| Classification | Finding |
| --- | --- |
| Confirmed | Rust exposes typed provider execution request/report structures for deterministic local stub execution only. |
| Confirmed | Provider output remains untrusted candidate data and cannot mutate authority. |
| Confirmed | Remote/cloud, fallback, auto-selection, unsafe, malformed, oversized, and invalid-configuration requests reject fail-closed. |
| Suspected | Additional timeout/resource runtime limits will be needed in Phase 108 before provider-adjacent authority decisions proceed. |

## Non-readiness statement
Phase 107 does not approve readiness. Phase 107 does not approve Production Candidate status. Phase 107 does not approve release-candidate readiness. Phase 107 does not approve production readiness. Phase 107 does not approve public usability. Phase 107 does not approve production human use.
