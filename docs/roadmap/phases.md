---
truth_dimension: planned
authority_level: advisory
mutation_path: roadmap_update
---

# Phases

This document is the active phase catalog.

- Compact index: `docs/roadmap/phase-map.md`
- Ordering rationale: `docs/roadmap/sequencing.md`
- Historical truth: `CHANGELOG.md`

## Current planning block: Phases 70-80

### Phase 70 - Roadmap Documentation Realignment and Production Candidate Gap Audit
Goal: realign roadmap planning surfaces (`phase-map.md`, `phases.md`, `sequencing.md`), refresh current-phase procedural truth, and publish an advisory production-candidate gap audit.
Boundary: documentation realignment and governance hygiene only; no runtime behavior implementation.

### Phase 71 - Provider Execution Adapter Implementation
Goal: implement a real local/provider execution path behind the Phase 69 transport boundary.
Boundary: provider output remains untrusted; no direct promotion, persistence, or ledger authority.

### Phase 72 - Provider Failure, Timeout, and Retry Boundary
Goal: add deterministic handling for provider timeout, cancellation, malformed output, and retry classification.
Boundary: retries do not mutate authoritative state unless recorded through Rust-owned sequencing.

### Phase 73 - Durable Ledger Persistence Lifecycle
Goal: define and implement which ledger records are persisted, verified, and loaded.
Boundary: persisted ledger records become usable only after Phase 62-style verification.

### Phase 74 - Application State Recovery Boundary
Goal: rehydrate in-memory state from verified persisted records.
Boundary: recovery is explicit, typed, and fail-closed; no silent repair.

### Phase 75 - Roadmap and Changelog Alignment Check
Goal: reconcile provider execution and durable recovery work.
Boundary: alignment checkpoint only; no readiness approval.

### Phase 76 - UI/Rust Transport Boundary
Goal: add the transport boundary between UI and Rust-owned read/intent surfaces.
Boundary: UI remains non-authoritative; transport carries typed requests/responses only.

### Phase 77 - UI Operator Intent Submission Wiring
Goal: wire UI intent submission to Rust ingress/authorization without action execution.
Boundary: submission does not execute action; Rust owns validation, authorization, and audit eligibility.

### Phase 78 - Authorized Operator Action Execution Boundary
Goal: define the first narrow executable operator action path.
Boundary: only explicitly authorized, audit-eligible actions can execute; no broad “apply any output” behavior.

### Phase 79 - End-to-End Local Harness Run
Goal: connect provider execution, authorization, audit record, persistence, recovery, and read projection into one bounded local workflow.
Boundary: still local-first; no public production claim.

### Phase 80 - Roadmap and Changelog Alignment Check + Production Candidate Gap Audit
Goal: determine what remains before public usability or release-candidate status can be considered.
Boundary: gap audit only; not approval.

## Planning notes for this block

- The 60s block is completed historical work and remains recorded in `CHANGELOG.md` only.
- This catalog intentionally avoids duplicating changelog completion claims.
- Production-candidate posture is deferred pending successful delivery/validation of phases 71-79 plus follow-on hardening/documentation/security/observability work.
