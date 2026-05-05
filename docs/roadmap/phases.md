---
truth_dimension: planned
authority_level: advisory
mutation_path: roadmap_update
---

# Phases

This document is the active expanded catalog.

- Compact planned phase index: `docs/roadmap/phase-map.md`
- Ordering rationale: `docs/roadmap/sequencing.md`
- Historical truth: `CHANGELOG.md`

## Current planning block: Phases 81-90

### Phase 81 - Local Harness Composition Hardening
Goal: harden the bounded Phase 79 local harness composition with negative-path and mismatch tests.
Boundary: hardening only; no new runtime authority.

### Phase 82 - Provider Evidence Replay and Failure Trace Boundary
Goal: make provider execution/failure evidence replayable for local harness runs.
Boundary: replay does not trust provider output or mutate state.

### Phase 83 - Durable Audit and Ledger Append Boundary
Goal: define which audit and ledger proof records may be durably appended.
Boundary: append eligibility is explicit; append is not promotion or recovery.

### Phase 84 - Recovery Candidate Acceptance Boundary
Goal: define how a verified recovery candidate may become accepted in-memory state under Rust-owned gates.
Boundary: acceptance is explicit and fail-closed; no silent recovery.

### Phase 85 - Roadmap and Changelog Alignment Check
Goal: reconcile post-harness hardening, evidence replay, durable append, and recovery-acceptance work.
Boundary: alignment only; no readiness approval.

### Phase 86 - User-Facing Local Workflow Documentation
Goal: document supported local commands, boundaries, failure modes, and operator expectations.
Boundary: documentation only; no runtime capability.

### Phase 87 - Observability and Audit Export Boundary
Goal: define local read-only export/report surfaces for diagnostics, audit, recovery, and execution decisions.
Boundary: export is read-only and non-authoritative.

### Phase 88 - Security and Abuse-Case Hardening Pass
Goal: add negative-path tests for transport abuse, submission spoofing, authorization/action mismatch, corrupted persistence, recovery drift, and provider-output injection.
Boundary: hardening only; no broad new capability.

### Phase 89 - Packaging and Local Startup Candidate Boundary
Goal: define minimal local startup/package surfaces for operator testing.
Boundary: startup/package usability does not imply production readiness.

### Phase 90 - Production Candidate Gap Audit and Readiness Decision Gate
Goal: determine whether a production-candidate path is supported by evidence or more hardening is required.
Boundary: decision gate only; not automatic approval.

## Planning notes for this block

- This catalog is planned truth and does not duplicate completion claims from `CHANGELOG.md`.
- Phase 80 is an alignment and audit checkpoint only and does not approve Production Candidate status unless evidence unexpectedly closes all required gaps.
- Production Candidate status remains not approved while mechanical and architectural gaps remain open.
- Roadmap remains planned truth.
- `CHANGELOG.md` remains historical truth.
