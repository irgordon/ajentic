---
truth_dimension: planned
authority_level: authoritative
mutation_path: roadmap_update
---
# Phase map

This document is the compact planned phase index.

It is planned truth only.

It does not record completed work.

Completed accepted work is historical truth in `CHANGELOG.md`.

Expanded planning detail for active phases is in `docs/roadmap/phases.md`.

Sequencing rationale is in `docs/roadmap/sequencing.md`.

## Phase index

| Phase | Title | Milestone group | One-sentence goal | Boundary note |
| --- | --- | --- | --- | --- |
| 85 | Roadmap and Changelog Alignment Check | Milestone 6 | Reconcile 81-84 outcomes and planning/history posture before outward-facing surfaces. | Alignment only; no readiness approval. |
| 86 | User-Facing Local Workflow Documentation | Milestone 6 | Document supported local workflows, commands, failure states, non-authority limits, and operator expectations. | Documentation only; no runtime capability change. |
| 87 | Read-Only Observability Snapshot Boundary | Milestone 6 | Define typed, local, read-only observability snapshots for diagnostics, recovery, append, replay, and action decision status. | Snapshot is read-only and non-authoritative; no export files, no transport, no persistence. |
| 88 | Audit Export Encoding Boundary | Milestone 6 | Define deterministic export encoding for audit/replay/append/recovery evidence. | Encoding only; no filesystem export, no network export, no authority mutation. |
| 89 | Local Export Write Boundary | Milestone 6 | Write verified export bundles through the existing persistence boundary. | Export write is not ledger append, not recovery, not promotion, and not live telemetry. |
| 90 | Roadmap and Production Candidate Gap Audit | Milestone 6 | Reconcile Phases 85-89 and determine whether the next block can pursue startup/package usability. | Gap audit only; not approval. |
| 91 | Transport Abuse and Submission Spoofing Hardening | Milestone 7 | Add negative-path tests for transport contracts, spoofed submission requests, and disabled execution flags. | Hardening only; no live transport. |
| 92 | Authorization/Audit/Action Mismatch Hardening | Milestone 7 | Add negative-path tests for authorization/audit/action mismatch, stale proof reuse, identity mismatch, and escalation attempts. | Hardening only; no new action authority. |
| 93 | Persistence Corruption and Append Drift Hardening | Milestone 7 | Add negative-path tests for corrupted append envelopes, stale revisions, partial writes, replay drift, and recovery mismatch. | Hardening only; no new persistence authority. |
| 94 | Provider Output Injection and Replay Abuse Hardening | Milestone 7 | Add negative-path tests for provider-output injection, replay tampering, failure trace spoofing, and retry escalation attempts. | Hardening only; provider output remains untrusted. |
| 95 | Roadmap and Changelog Alignment Check | Milestone 7 | Reconcile 91-94 hardening outcomes and decide whether startup/package work is safe. | Alignment only; no readiness approval. |
| 96 | Local Startup Command Boundary | Milestone 8 | Define the minimal local startup command surface for operator testing. | Startup command does not imply production service, daemon, network, or public usability. |
| 97 | Packaging Artifact Definition | Milestone 8 | Define which local artifacts are built, named, versioned, and excluded. | Packaging definition only; no distribution/release approval. |
| 98 | Operator Documentation and Troubleshooting Guide | Milestone 8 | Document startup, validation, expected outputs, failure modes, and rollback expectations for local operators. | Documentation only; no runtime capability. |
| 99 | Release Engineering Dry Run | Milestone 8 | Validate release checklist mechanics, artifact inventory, version/changelog consistency, and CI gate completeness without publishing. | Dry run only; no release and no production-candidate approval. |
| 100 | Production Candidate Readiness Decision Gate | Milestone 8 | Determine whether evidence supports a production-candidate branch/tag or whether another hardening block is required. | Decision gate only; approval only if evidence is complete. |
| 101 | Production Use Gap Decomposition | Milestone 9 | Decompose remaining blockers for human use. | Audit/planning only. |
| 102 | Human Operator Workflow Contract | Milestone 9 | Define operator workflows, roles, and expected states. | Documentation/contract only. |
| 103 | UI Runtime Review Surface Activation Boundary | Milestone 9 | Activate the browser review surface for human operator visibility. | UI usability only; no Rust authority and no live mutation. |
| 104 | UI-to-Rust Local Transport Prototype Boundary | Milestone 9 | Prototype local UI-to-Rust communication under non-authoritative constraints. | Local transport prototype only; no provider execution and no persistence authority. |
| 105 | Transport Abuse Hardening for Live Local Bridge | Milestone 9 | Harden the live local bridge against malformed, spoofed, replayed, or hostile transport input. | Hardening only; no broad capability. |
| 106 | Provider Configuration Contract | Milestone 9 | Reconciled as completed historical outcome: provider configuration contracts without provider execution. | Configuration contract only; no live provider execution; historical completion remains in changelog. |
| 107 | Provider Execution Sandbox Boundary | Milestone 9 | Reconciled as completed historical outcome: bounded deterministic local stub provider execution. | Bounded local stub execution only; provider output remains untrusted. |
| 108 | Provider Timeout and Resource Limit Boundary | Milestone 9 | Reconciled as completed historical outcome: deterministic timeout/resource enforcement with descriptive-only evidence. | Provider hardening only; no promotion authority. |
| 109 | Durable Persistence Authority Decision Gate | Milestone 9 | Reconciled as completed historical outcome: durable persistence authority decision evidence only. | Decision/audit only; no persistence activation. |
| 110 | Roadmap and Changelog Alignment Check | Milestone 9 | Reconcile Phase 106-109 outcomes, roadmap planned truth, changelog archive posture, and Phase 111 gate constraints. | Alignment/check only; no runtime behavior, no new capability, no persistence authority. |
| 111 | Narrow Persistence Activation Boundary | Milestone 10 | Earliest possible narrow Rust-validated decision-evidence append activation, only if Phase 109/110 constraints remain valid. | Not broad persistence authority; no provider-output authority, replay repair, recovery promotion, action execution, or readiness approval. |
| 112 | Policy Versioning and Governance Evidence Boundary | Milestone 10 | Add policy versioning and governance evidence traceability. | Policy/governance versioning only. |
| 113 | Deployment Configuration Contract | Milestone 10 | Define deployment configuration contracts without deployment automation. | Deployment config only; no deployment automation. |
| 114 | Local Deployment Candidate Boundary | Milestone 10 | Define a local deployment candidate boundary for controlled testing. | Local deployment candidate only; no public release. |
| 115 | Security Threat Model and Abuse-Case Audit | Milestone 10 | Audit threat model, abuse cases, trust boundaries, and residual attack surfaces. | Security audit only. |
| 116 | Operator Documentation for Human Trial | Milestone 10 | Prepare operator documentation for controlled human-trial use. | Operator docs only; no readiness approval. |
| 117 | Human Trial Dry Run | Milestone 10 | Rehearse controlled human-trial procedures without public availability. | Dry run only; no public availability. |
| 118 | Release Candidate Evidence Assembly | Milestone 10 | Assemble release-candidate evidence without approval. | Evidence assembly only; no automatic approval. |
| 119 | Production Candidate Reassessment | Milestone 10 | Reassess Production Candidate posture after controlled evidence assembly. | Decision gate only. |
| 120 | Early Human-Use Candidate Gate | Milestone 10 | Decide whether controlled early human use is permitted if intervening evidence supports review. | Current planned gate only; not a guaranteed final endpoint and not general public release. |

## Boundary reminder

Roadmap entries are planned truth and boundaries only.

Phase 110 is the current alignment/check gate after Phase 109; it is not persistence activation and does not approve readiness.

Phase 111 is the earliest possible narrow persistence activation phase, limited to the Rust-validated decision-evidence append candidate permitted by Phase 109 and confirmed by Phase 110.

Phase 120 is a current planned gate, not a guaranteed final production endpoint.

Phases 101-120 are planned truth only and do not imply implementation, readiness, public usability, release-candidate status, Production Candidate status, or production approval.

The staged production-human-use ladder is: Local operator testing; Controlled human trial; Early human-use candidate; Release candidate; Production candidate; Public/general use.

Roadmap remains planned truth.

Historical completion remains in `CHANGELOG.md`.

`CHANGELOG.md` remains historical truth.
