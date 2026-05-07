---
truth_dimension: orientation
authority_level: advisory
mutation_path: readme_update
---
# Phase 106 - Provider Configuration Contract

## Scope
Phase 106 defines deterministic provider configuration contracts only. It validates provider identity, declared capabilities, timeout/resource declarations, isolation metadata, provider selection metadata, and disabled execution indicators without executing providers.

## Provider configuration boundary
Phase 106 is configuration-contract only. Phase 106 does not execute providers, call remote APIs, perform inference, transmit prompts, transmit context, persist provider state, append ledger/audit state, repair replay, promote recovery, execute actions, or approve readiness.

## Typed configuration posture
The Rust API defines typed provider configuration structures for provider identity, provider type, capability declarations, timeout/resource limits, isolation declarations, transport posture, trust posture, readiness posture, and disabled execution posture.

## Deterministic validation posture
Provider validation uses deterministic fail-closed reason codes. Multiple rejection reasons are returned in stable code order so identical malformed input produces identical reports.

## Non-executing posture
Provider configuration validation is pure contract validation. It opens no HTTP or HTTPS sockets, invokes no provider SDK, performs no streaming, starts no background work, and schedules no retries.

## Provider identity contract
Provider identifiers must be 3-64 bytes, begin and end with lowercase ASCII letters, and contain only lowercase ASCII letters, digits, hyphens, or underscores. Invalid identifiers reject fail-closed.

## Provider capability contract
Allowed capability declarations are bounded to configuration metadata such as `configuration_review`, `text_generation_declared`, and `embedding_declared`. Unsupported capability declarations reject fail-closed and do not imply execution authority.

## Timeout/resource declaration posture
Provider configuration may declare bounded timeout and payload/resource limits. Zero, non-numeric, or excessive timeout/resource values reject fail-closed.

## Isolation declaration posture
Provider configuration must explicitly declare local-only and no-network isolation, with no background execution. Missing or unsafe isolation declarations reject fail-closed.

## Disabled-execution posture
Execution and transport indicators must be explicitly disabled. Execution-enabled or transport-enabled values reject fail-closed, and validation reports always return execution and transport as disabled.

## Untrusted-provider posture
Providers remain untrusted. Configuration that claims provider trust or readiness rejects fail-closed, and accepted configuration still reports provider trust as disabled.

## Rejection behavior
Malformed, oversized, duplicate, unsupported, authority-bearing, auto-selection, fallback, execution-enabled, transport-enabled, trust-enabled, and readiness-claiming configuration rejects deterministically.

## Malformed-configuration behavior
Malformed payload shape, missing `provider` blocks, duplicate keys, missing required fields, unknown fields, or invalid primitive values reject fail-closed.

## Duplicate-identifier behavior
Duplicate provider identifiers reject deterministically with `duplicate_provider_identifier`; validation does not select or prioritize either duplicate provider.

## Unsupported-provider behavior
Only `local_only_declared` is accepted as a configuration type. Remote, SDK-backed, cloud, HTTP, HTTPS, or otherwise unsupported provider types reject fail-closed.

## Invalid-capability behavior
Unsupported capability declarations reject fail-closed. Capability text is treated as metadata and never as permission to call a provider or model.

## Invalid-timeout/resource behavior
Timeouts must be non-zero and at most 30,000 ms. Prompt bytes must be non-zero and at most 65,536. Context bytes must be non-zero and at most 262,144.

## Behavioral test coverage
Behavioral tests cover malformed configs, missing fields, invalid identifiers, invalid capabilities, invalid timeout/resource declarations, execution-enabled flags, unsupported provider types, duplicate identifiers, deterministic reason ordering, disabled execution, disabled trust, and disabled transport.

## Adversarial test coverage
Adversarial tests cover malformed provider payloads, oversized payloads, duplicate identifiers, hidden execution flags, unsupported capabilities, invalid resource values, replay-shaped payloads, authority-bearing payloads, auto-enable payloads, fallback payloads, and hostile/noise payloads.

## Relationship to Phase 105 transport hardening
Phase 106 preserves Phase 105 transport hardening by keeping provider transport disabled and by adding no broader local bridge capability, no remote transport, and no provider execution path.

## Relationship to Phase 107 provider sandbox execution
Phase 107, if pursued, is the next planned provider execution sandbox phase only. Phase 106 does not implement Phase 107 and does not introduce live provider execution.

## Required future implementation evidence
Future provider execution work would require sandbox execution code, no-network/timeout/resource enforcement evidence, provider-output untrusted handling, no persistence authority evidence, and tests proving execution remains bounded.

## Phase 107 gate decision
Phase 107 may begin next only as the planned provider execution sandbox boundary phase if maintainers choose to proceed from the roadmap. This is not an approval of readiness or production use.

## Production Candidate status
Phase 106 does not approve Production Candidate status.

## Release-candidate readiness status
Phase 106 does not approve release-candidate readiness.

## Public/general use status
Phase 106 does not approve public usability or production human use.

## Roadmap/changelog truth posture
Roadmap files remain planned truth. CHANGELOG surfaces remain historical truth for accepted completed work.

## Required follow-ups
Future phases must supply committed sandbox execution evidence before any provider execution claim can be counted.

## Deferred items
Provider execution, inference, remote API calls, provider SDK integration, streaming, retries, persistence authority, durable append authority, export authority, replay repair, recovery promotion, and action execution are deferred.

## Confirmed vs suspected
Confirmed: deterministic configuration contracts and validation boundaries are implemented and tested. Suspected or future provider execution behavior is not counted as evidence.

## Non-readiness statement
Phase 106 defines provider configuration contracts only. It does not execute providers, perform inference, add provider runtime authority, add persistence authority, add replay repair, add recovery promotion, add action execution, approve readiness, approve Production Candidate status, approve release-candidate readiness, approve production readiness, approve public usability, approve production human use, or implement Phase 107.
