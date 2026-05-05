---
truth_dimension: planned
authority_level: advisory
mutation_path: roadmap_update
---

# Sequencing

This document provides ordering rationale and dependency chain for the active block.

## Roadmap surface roles

- `phase-map.md`: compact planned phase index.
- `phases.md`: active expanded catalog.
- `sequencing.md`: ordering rationale and dependency chain.
- `CHANGELOG.md`: historical truth only.

## Dependency chain rationale (Phases 81-90)

1. **Phases 81-82 harden what Phase 79 connected**
   - Phase 79 provided bounded local composition evidence.
   - Phase 81 hardens negative-path and mismatch behavior for that composition seam.
   - Phase 82 then requires replayable provider execution/failure evidence so hardening outcomes are inspectable and reproducible.

2. **Phases 83-84 define durable append and recovery acceptance after proof objects exist**
   - Durable append constraints must be explicit before any acceptance path can rely on persisted proof records.
   - Phase 83 establishes append eligibility for audit/ledger proof objects only.
   - Phase 84 follows by defining explicit fail-closed acceptance from verified recovery candidates into in-memory state.

3. **Phase 85 realigns before outward-facing surfaces**
   - After hardening, replay, append, and acceptance boundaries, an alignment checkpoint is required.
   - Phase 85 validates roadmap/changelog truth-surface separation and boundary language before user-facing documentation/export/startup surfaces expand.

4. **Phases 86-87 make the system explainable and observable**
   - Phase 86 documents supported local operator workflows and failure expectations.
   - Phase 87 defines read-only observability and audit export surfaces so behavior can be inspected without authority transfer.

5. **Phase 88 attacks new composition seams**
   - Security and abuse-case hardening targets newly connected transport/submission/authorization/persistence/recovery seams.
   - This must happen before packaging/startup claims to prevent usability surfaces from outpacing hardening posture.

6. **Phase 89 handles local startup/package usability**
   - Packaging/startup boundary work is delayed until hardening and observability surfaces exist.
   - The goal is local operator usability only, not readiness approval.

7. **Phase 90 decides whether Production Candidate status is plausible**
   - Phase 90 is a decision gate informed by evidence from 81-89.
   - Approval is not automatic; unresolved mechanical or architectural gaps keep status as not approved.

## Production-candidate posture carried from Phase 80

- Phase 80 is alignment/audit/planning only and does not implement runtime behavior.
- Phase 80 does not approve Production Candidate status unless evidence unexpectedly supports closure of listed gaps.
- Roadmap remains planned truth.
- `CHANGELOG.md` remains historical truth.
