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
