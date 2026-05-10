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
| 111 | Narrow Persistence Activation Boundary | Milestone 10 | Reconciled as completed historical outcome: narrow Rust-validated decision-evidence append activation. | Not broad persistence authority; no provider-output authority, replay repair, recovery promotion, action execution, or readiness approval. |
| 112 | Recovery Lifecycle Hardening | Milestone 10 | Reconciled as completed historical outcome: fail-closed recovery lifecycle classification for Phase 111 decision-evidence records. | Recovery lifecycle hardening only; no replay repair, recovery promotion, action execution, readiness approval, or Phase 113 implementation. |
| 113 | Deployment Configuration Contract | Milestone 10 | Define deployment configuration contracts that consume Phase 112 recovery handoff gaps without deployment automation. | Deployment config only; no deployment automation; must account for recovery handoff gaps. |
| 114 | Policy Versioning and Governance Evidence Boundary | Milestone 10 | Add policy versioning and governance evidence traceability as planned future work. | Policy/governance versioning only; not completed Phase 112 work. |
| 115 | Security Threat Model and Abuse-Case Audit | Milestone 10 | Audit threat model, abuse cases, trust boundaries, and residual attack surfaces. | Security audit only. |
| 116 | Local Deployment Candidate Boundary | Milestone 10 | Define a local deployment candidate boundary for controlled testing after deployment configuration and security audit evidence. | Local deployment candidate only; no public release. |
| 117 | Operator Documentation and Human Trial Dry Run | Milestone 10 | Prepare and rehearse controlled human-trial procedures without public availability. | Operator docs and dry run only; no readiness approval and no public availability. |
| 118 | Release Candidate Evidence Assembly | Milestone 10 | Assemble release-candidate evidence without approval. | Evidence assembly only; no automatic approval. |
| 119 | Production Candidate Reassessment | Milestone 10 | Reassess Production Candidate posture after controlled evidence assembly. | Decision gate only. |
| 120 | Early Human-Use Candidate Gate | Milestone 10 | Phase 120 is complete as Early Human-Use Candidate Gate only and permitted `early_human_use_candidate_permitted_with_constraints`. | Complete gate only; not a guaranteed final endpoint, not Release Candidate approval, not Production Candidate approval, not production readiness, not production deployment, not production human use, and not public/general use. |
| 121 | Post-120 Roadmap Expansion and Production Gap Reassessment | Milestone 11 | Expand the roadmap beyond Phase 120 based on Phase 118-120 evidence and remaining usability, deployment, observability, release, and public-use gaps. | Audit/planning only; no runtime behavior, readiness approval, release approval, production approval, or public-use approval. |
| 122 | Controlled Early Human-Use Trial Boundary | Milestone 11 | Conduct or rehearse bounded early human-use under Phase 120 constraints and manual review. | Controlled early-human-use trial only; no public release, no authority expansion, and no production human use. |
| 123 | Early Human-Use Evidence Review and Operator Feedback Audit | Milestone 11 | Review early human-use evidence, operator notes, stop conditions, usability findings, and unresolved safety issues. | Audit/evidence review only; no readiness approval and no implicit rung promotion. |
| 124 | Operational Usability Remediation Boundary | Milestone 11 | Address confirmed usability blockers for local operators and bounded early human-use participants. | Usability remediation only; no readiness approval and no public/general use. |
| 125 | Roadmap, Changelog, and Production-Path Reassessment | Milestone 11 | Reconcile Phase 121-124 outcomes and reassess whether Phase 126-130 should be preserved, caveated, remapped, deferred, or expanded beyond Phase 130. | 0/5 checkpoint only; not a green light phase, not readiness approval, and not release, Production Candidate, production, or public/general-use approval. |
| 126 | Release Packaging Contract | Milestone 12 | Define packaging, artifact, checksum, provenance, distribution, and non-public/public-boundary contracts. | Release packaging contract only; no package creation, publication, or release artifact publication. |
| 127 | Installer and Update-Channel Threat Boundary | Milestone 12 | Define installer/update-channel risks, constraints, prohibited behaviors, and future evidence requirements. | Threat model/contract only; no installer or update-channel activation. |
| 128 | Observability and Operational Evidence Boundary | Milestone 12 | Define operational telemetry, audit, failure reporting, incident evidence, and operator evidence-capture requirements. | Observability evidence only; no production monitoring claim. |
| 129 | Release Candidate Dry Run | Milestone 12 | Rehearse release-candidate assembly without publishing or approving release-candidate readiness. | Dry run only; no release approval, release artifact publication, or public download. |
| 130 | Release Candidate Decision Gate | Milestone 12 | Decide whether evidence supports Release Candidate status or requires another hardening block. | Decision gate only; not Production Candidate status, production readiness, production deployment, or public/general use. |
| 131 | Post-130 Roadmap Expansion and Release Evidence Remap | Milestone 13 | Convert Phase 130's `rc_candidate_not_ready` findings into the next evidence-producing block. | Audit/planning only; no Phase 131+ implementation and not a Phase 130 rerun without new evidence. |
| 132 | Release Artifact Creation Boundary | Milestone 13 | Create controlled release artifact outputs under the Phase 126 contract. | Local/non-public artifact creation only; no publishing. |
| 133 | Checksum and Provenance Evidence Boundary | Milestone 13 | Generate and validate checksum/provenance evidence for controlled artifacts. | Checksum/provenance evidence only; no signing or publishing. |
| 134 | Signing and Key-Custody Implementation Boundary | Milestone 13 | Introduce signing/key-custody controls or explicitly defer them. | Signing/key-custody implementation only if evidence permits; no publishing. |
| 135 | Roadmap and Changelog Alignment Check | Milestone 13 | Reconcile Phase 131-134 and decide whether installer/update-channel or release dry-run work may proceed. | Alignment only; no readiness approval. |
| 136 | Installer/Update-Channel Implementation Boundary | Milestone 14 | Implement or further defer installer/update-channel surfaces under Phase 127 constraints. | Controlled implementation only; no public distribution. |
| 137 | Operational Observability Implementation Boundary | Milestone 14 | Implement local/non-production observability evidence capture if permitted. | Controlled observability implementation only; no production monitoring claim. |
| 138 | Incident, Support, and Rollback Evidence Boundary | Milestone 14 | Define and test incident, support, rollback, and recovery evidence. | Operational procedure/evidence only; no production support claim. |
| 139 | Release Candidate Evidence Reassembly | Milestone 14 | Reassemble Release Candidate evidence after artifact, provenance, signing, installer/update, and observability work. | Evidence assembly only; no approval. |
| 140 | Release Candidate Re-Decision Gate | Milestone 14 | Decide whether Release Candidate status is now supportable or whether another hardening block is required. | Decision gate only; not automatic Release Candidate approval, Production Candidate approval, or public/general-use approval. |

## Boundary reminder

Roadmap entries are planned truth and boundaries only.

Phase 112.5 corrected planned-truth drift after Phase 112 and before Phase 113; it is alignment/correction only, adds no runtime behavior, adds no new capability, and does not implement Phase 113.

Phase 111 remains recorded in historical truth as completed narrow Rust-validated decision-evidence append activation under Phase 109/110 constraints.

Phase 112 is Recovery Lifecycle Hardening in planned truth and historical truth; Policy Versioning and Governance Evidence Boundary is planned future work at Phase 114, not completed Phase 112 work.

Phase 113 is Deployment Configuration Contract only. It must not add deployment automation and must consume Phase 112 recovery handoff gaps for storage paths, permissions, retention, environment assumptions, failure handling, manual review, no background repair, no automatic replay patching, no continue-anyway behavior, no migration/version upgrade authority, no production recovery guarantee, and no release evidence guarantee.

Phase 120 is complete as Early Human-Use Candidate Gate only. Phase 120 permitted `early_human_use_candidate_permitted_with_constraints`: bounded, non-public, local/trial-only, manually reviewed, and constrained to named internal/trial participants. Phase 120 is not a guaranteed final endpoint, Release Candidate approval, release-candidate readiness, Production Candidate approval, production readiness, production deployment, production human use, or public/general use. Phase 120 created no release artifacts, packages, installer behavior, update-channel behavior, signing/publishing behavior, GitHub release, release tag, public download, public asset, public release behavior, production deployment behavior, or deployment automation; it did not expand provider execution, persistence authority, replay repair, recovery promotion, action execution, provider trust, provider output promotion, or readiness.

Phase 121 expands roadmap/planned truth beyond Phase 120 and is audit/planning only. Roadmap expansion is planned truth, not implementation, release approval, production approval, public-use approval, or readiness approval.

Phase 122 begins only controlled early-human-use trial work under Phase 120 constraints; it is not public release, Release Candidate status, Production Candidate status, production deployment, production readiness, public/general use, or production human use.

Phase 125 is Roadmap, Changelog, and Production-Path Reassessment. It preserves the 0/5 alignment/checkpoint cadence and is not a green light phase. Phase 125 may preserve Phase 126-130 with caveats, remap, defer, or expand post-130 planned truth, but reconciliation is not readiness approval. Phase 119 was an intentional decision-gate exception and does not redefine the 0/5 convention.

Phase 130 is complete. Its decision status is `rc_candidate_not_ready`. Phase 130 did not approve Release Candidate status, did not approve Production Candidate status, did not approve public/general use, and did not create the missing evidence it identified.

Phase 131 is audit/planning only and must not be a rerun of Phase 130 without new evidence. Phase 131 converts Phase 130's `rc_candidate_not_ready` findings into the Phase 132-140 planned evidence-producing and decision block. The next block must produce or explicitly defer the release artifacts, package outputs, checksum/provenance evidence, signing/key-custody controls, installer/update-channel evidence, operational observability evidence, and incident/support/rollback evidence that blocked Release Candidate supportability.

Phase 140 is a Release Candidate Re-Decision Gate only. It is not automatic Release Candidate approval, Production Candidate approval, production readiness, production deployment, production human use, or public/general use. Public/general use remains a later final rung. Do not map Production Candidate or public/general-use as automatically following Phase 140; any post-140 block depends on the Phase 140 decision and must preserve the ladder: Local operator testing → Controlled human trial → Early human-use candidate → Release candidate → Production candidate → Public/general use.

Phases 101-140 are planned truth only and do not imply implementation, readiness, public usability, Release Candidate status, Production Candidate status, or production approval.

The staged production-human-use ladder is: Local operator testing; Controlled human trial; Early human-use candidate; Release candidate; Production candidate; Public/general use.

Roadmap remains planned truth.

Historical completion remains in `CHANGELOG.md`.

`CHANGELOG.md` remains historical truth.


## Phase 121 roadmap expansion guardrails

Phase 121 applies the Ladder-Preservation Invariant Set: ladder steps are distinct; No implicit promotion is allowed; Absence of blockers is not approval; Evidence assembly is not readiness; Dry runs are not release; Deployment is not release; no phase retroactively rewrites earlier gates; human use is not binary; Public/general use is always the final rung; No trust inference may be drawn from provider output or human feedback; No cross-category inference may collapse sandbox, persistence, recovery, deployment, usability, observability, operator workflow, security, governance, transport, provider, release, and public-use evidence; and Roadmap continuation is required when mapped phases end before the ladder.

Phase 121 reassesses production/public-use gaps as planned truth: early human-use constraints, controlled participant management, operator ergonomics, UI usability, transport usability and safety, provider execution and provider-output review burden, observability, operational evidence, operational telemetry, incident response, rollback, recovery procedures, release packaging, artifact provenance, checksum/signing contract, installer/update-channel governance, public download governance, deployment environment assumptions, support model, non-developer documentation, public/general-use threat model, production readiness evidence, Release Candidate evidence, Production Candidate evidence, and public/general-use gate evidence.

Phase 131-140 planning is binding planned truth for the next mapped block but remains non-implementation. It carries forward Phase 130's `rc_candidate_not_ready` status and maps category-specific evidence production or explicit deferral before any Release Candidate re-decision. Public/general use remains a later final rung. Production Candidate and public/general-use planning must not be mapped as automatic successors to Phase 140; any post-140 block depends on the Phase 140 decision and must preserve the ladder.
