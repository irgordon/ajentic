---
truth_dimension: planned
authority_level: advisory
mutation_path: roadmap_update
---

# Phases

This document is the active expanded planning catalog.

- Compact planned phase index: `docs/roadmap/phase-map.md`
- Ordering rationale: `docs/roadmap/sequencing.md`
- Historical truth: `CHANGELOG.md`

## Current planning block: Phases 85-100

### Phase 85 - Roadmap and Changelog Alignment Check
Goal: reconcile Phase 81-84 outcomes and planning/history posture before outward-facing surfaces.
Boundary: alignment only; no readiness approval.

### Phase 86 - User-Facing Local Workflow Documentation
Goal: document supported local workflows, commands, failure states, non-authority limits, and operator expectations.
Boundary: documentation only; no runtime capability change.

### Phase 87 - Read-Only Observability Snapshot Boundary
Goal: define typed, local, read-only observability snapshots for diagnostics, recovery, append, replay, and action decision status.
Boundary: snapshot is read-only and non-authoritative; no export files, no transport, no persistence.

### Phase 88 - Audit Export Encoding Boundary
Goal: define deterministic export encoding for audit/replay/append/recovery evidence.
Boundary: encoding only; no filesystem export, no network export, no authority mutation.

### Phase 89 - Local Export Write Boundary
Goal: write verified export bundles through the existing persistence boundary.
Boundary: export write is not ledger append, not recovery, not promotion, and not live telemetry.

### Phase 90 - Roadmap and Production Candidate Gap Audit
Goal: reconcile Phases 85-89 and determine whether the next block can pursue startup/package usability.
Boundary: gap audit only; not approval.

### Phase 91 - Transport Abuse and Submission Spoofing Hardening
Goal: add negative-path tests for UI/Rust transport contracts, submission-shaped requests, spoofed operators, and disabled execution flags.
Boundary: hardening only; no live transport.

### Phase 92 - Authorization/Audit/Action Mismatch Hardening
Goal: add negative-path tests for authorization/audit/action proof mismatch, stale proof reuse, identity mismatch, and action-kind escalation.
Boundary: hardening only; no new action authority.

### Phase 93 - Persistence Corruption and Append Drift Hardening
Goal: add negative-path tests for corrupted append envelopes, stale revisions, partial writes, replay drift, and recovery candidate mismatch.
Boundary: hardening only; no new persistence authority.

### Phase 94 - Provider Output Injection and Replay Abuse Hardening
Goal: add negative-path tests for provider-output injection, replay tampering, failure trace spoofing, and retry escalation attempts.
Boundary: hardening only; provider output remains untrusted.

### Phase 95 - Roadmap and Changelog Alignment Check
Goal: reconcile Phase 91-94 hardening outcomes and decide whether local startup/package work is safe.
Boundary: alignment only; no readiness approval.

### Phase 96 - Local Startup Command Boundary
Goal: define the minimal local startup command surface for operator testing.
Boundary: startup command does not imply production service, daemon, network, or public usability.

### Phase 97 - Packaging Artifact Definition
Goal: define what local artifacts are built, named, versioned, and excluded.
Boundary: packaging definition only; no distribution/release approval.

### Phase 98 - Operator Documentation and Troubleshooting Guide
Goal: document startup, validation, expected outputs, failure modes, and rollback expectations for local operators.
Boundary: documentation only; no runtime capability.

### Phase 99 - Release Engineering Dry Run
Goal: validate release checklist mechanics, artifact inventory, changelog/version consistency, and CI gate completeness without publishing.
Boundary: dry run only; no release, no production-candidate approval.

### Phase 100 - Production Candidate Readiness Decision Gate
Goal: determine whether evidence supports a production-candidate branch/tag or whether another hardening block is required.
Boundary: decision gate only; approval only if evidence is complete.

## Planning notes for this block

- This catalog is planned truth and does not duplicate completion claims from `CHANGELOG.md`.
- Roadmap remains planned truth.
- `CHANGELOG.md` remains historical truth.
- This expansion reduces composition density before outward-facing surfaces.
- Production Candidate status remains not approved unless a future decision gate explicitly approves it with complete evidence.
## Post-Phase-100 planned production human-use expansion: Phases 101-120

Phase 99.5 is planning and alignment only. It adds no runtime behavior, adds no new capability, does not approve production human use, does not approve Production Candidate status, does not approve release-candidate readiness, does not approve public usability, does not start Phase 100, and does not implement Phases 101-120.

Phase 100 remains the immediate Production Candidate gap audit and readiness decision gate. Phase 100 does not equal production. Phases 101-120 are planned truth only and do not imply implementation, readiness, public usability, release-candidate status, Production Candidate status, or production approval.

Production human use is staged in this ladder:

1. Local operator testing
2. Controlled human trial
3. Early human-use candidate
4. Release candidate
5. Production candidate
6. Public/general use

The post-100 roadmap separates UI activation, live local transport, provider execution, authoritative persistence, deployment, security, human trial, release-candidate evidence, production-candidate reassessment, and controlled human-use gate work.

### Phase 101 - Production Use Gap Decomposition
Goal: Decompose remaining blockers for human use.
Boundary: audit/planning only.
Non-goals: no source changes, no runtime behavior, no UI activation, no transport, no provider execution, no persistence authority, and no readiness approval.
Evidence gate: blocker inventory reconciled against Phase 100 findings without claiming human-use readiness.

### Phase 102 - Human Operator Workflow Contract
Goal: Define operator workflows, roles, and expected states.
Boundary: documentation/contract only.
Non-goals: no workflow automation, no authority mutation, no public usability approval, and no Production Candidate approval.
Evidence gate: operator workflow states are documented as contracts before usability surfaces are activated.

### Phase 103 - UI Runtime Review Surface Activation Boundary
Goal: Activate the browser review surface for human operator visibility.
Boundary: UI usability only; no Rust authority and no live mutation.
Non-goals: no live transport, no provider execution, no persistence authority, no action authority, and no production human-use approval.
Evidence gate: UI activation remains visible/review-only and cannot mutate Rust-owned authority.

### Phase 104 - UI-to-Rust Local Transport Prototype Boundary
Goal: Prototype local UI-to-Rust communication under non-authoritative constraints.
Boundary: local transport prototype only; no provider execution and no persistence authority.
Non-goals: no broad capability, no public transport, no provider/model authority, no persistence authority, and no readiness approval.
Evidence gate: prototype evidence shows the transport remains local and non-authoritative before hardening begins.

### Phase 105 - Transport Abuse Hardening for Live Local Bridge
Goal: Harden the live local bridge against malformed, spoofed, replayed, or hostile transport input.
Boundary: hardening only; no broad capability.
Non-goals: no new operator capability, no provider execution, no persistence authority, and no release-candidate readiness.
Evidence gate: negative-path evidence covers malformed, spoofed, replayed, and hostile input without expanding authority.

### Phase 106 - Provider Configuration Contract
Goal: Define provider configuration contracts without executing providers.
Boundary: configuration contract only; no live provider execution.
Non-goals: no provider calls, no model calls, no network execution, no secret activation, and no production approval.
Evidence gate: provider configuration is defined and validated as intent/configuration only.

### Phase 107 - Provider Execution Sandbox Boundary
Goal: Introduce bounded provider execution under sandboxed constraints.
Boundary: bounded provider execution only; provider output remains untrusted.
Non-goals: no provider authority, no model-output authority, no persistence promotion, no action execution, and no Production Candidate approval.
Evidence gate: sandbox evidence demonstrates bounded execution and untrusted-output handling.

### Phase 108 - Provider Timeout and Resource Limit Boundary
Goal: Add provider timeout and resource-limit enforcement.
Boundary: provider hardening only; no promotion authority.
Non-goals: no promotion of provider output, no persistence authority, no recovery acceptance, and no release approval.
Evidence gate: timeout and resource-limit behavior is enforced before provider-adjacent authority decisions proceed.

### Phase 109 - Durable Persistence Authority Decision Gate
Goal: Determine whether local persistence can become authoritative.
Boundary: decision/audit only.
Non-goals: no persistence activation, no durable append expansion, no silent migration, no recovery promotion, and no readiness approval.
Evidence gate: decision evidence explicitly permits or blocks any Phase 110 authoritative persistence activation.

### Phase 110 - Authoritative Persistence Activation Boundary
Goal: Activate authoritative persistence only under Phase 109 evidence constraints.
Boundary: narrow persistence authority only if Phase 109 permits it.
Non-goals: no persistence activation without Phase 109 permission, no broad storage capability, no silent repair, and no public usability approval.
Evidence gate: activation evidence maps directly to Phase 109 constraints and remains narrow.

### Phase 111 - Recovery Lifecycle Hardening
Goal: Harden recovery lifecycle behavior without silent repair or implicit promotion.
Boundary: recovery lifecycle only; no silent recovery.
Non-goals: no silent recovery, no implicit promotion, no replay repair authority, and no production human-use approval.
Evidence gate: recovery lifecycle evidence demonstrates explicit operator-visible states and no silent repair.

### Phase 112 - Policy Versioning and Governance Evidence Boundary
Goal: Add policy versioning and governance evidence traceability.
Boundary: policy/governance versioning only.
Non-goals: no governance rule rewrite, no architecture authority change, no source authority expansion, and no release-candidate approval.
Evidence gate: policy version evidence is traceable and does not change runtime authority by itself.

### Phase 113 - Deployment Configuration Contract
Goal: Define deployment configuration contracts without deployment automation.
Boundary: deployment config only; no deployment automation.
Non-goals: no deployment automation, no installer, no distribution, no publishing, and no startup/package readiness approval.
Evidence gate: deployment configuration is documented as contract evidence only.

### Phase 114 - Local Deployment Candidate Boundary
Goal: Define a local deployment candidate boundary for controlled testing.
Boundary: local deployment candidate only; no public release.
Non-goals: no public release, no distribution approval, no installer approval, no production approval, and no public/general use.
Evidence gate: local deployment candidate evidence remains controlled and non-public.

### Phase 115 - Security Threat Model and Abuse-Case Audit
Goal: Audit threat model, abuse cases, trust boundaries, and residual attack surfaces.
Boundary: security audit only.
Non-goals: no runtime repair, no new security tooling, no authority expansion, and no readiness approval.
Evidence gate: threat model and abuse-case findings are recorded before human-trial documentation proceeds.

### Phase 116 - Operator Documentation for Human Trial
Goal: Prepare operator documentation for controlled human-trial use.
Boundary: operator docs only; no readiness approval.
Non-goals: no human trial start, no public availability, no release-candidate readiness, and no Production Candidate approval.
Evidence gate: operator documentation is complete enough for a later dry run but does not approve the trial.

### Phase 117 - Human Trial Dry Run
Goal: Rehearse controlled human-trial procedures without public availability.
Boundary: dry run only; no public availability.
Non-goals: no public/general use, no production human-use approval, no release-candidate approval, and no Production Candidate approval.
Evidence gate: dry-run evidence shows procedures can be rehearsed without making the system publicly available.

### Phase 118 - Release Candidate Evidence Assembly
Goal: Assemble release-candidate evidence without approval.
Boundary: evidence assembly only; no automatic approval.
Non-goals: no Release Candidate approval, no release-candidate readiness claim, no production approval, and no public usability approval.
Evidence gate: release-candidate evidence is assembled for review and remains non-approving.

### Phase 119 - Production Candidate Reassessment
Goal: Reassess Production Candidate posture after controlled evidence assembly.
Boundary: decision gate only.
Non-goals: no automatic approval, no public release, no general-use approval, and no bypass of evidence gates.
Evidence gate: reassessment records whether the posture remains blocked or can proceed under explicit evidence constraints.

### Phase 120 - Early Human-Use Candidate Gate
Goal: Decide whether controlled early human use is permitted.
Boundary: final gate for controlled human use; not general public release.
Non-goals: no public/general use, no automatic production approval, no release-candidate shortcut, and no broad availability.
Evidence gate: controlled early human-use permission, if granted by future evidence, remains separate from public/general use.
