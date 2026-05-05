---
truth_dimension: planned
authority_level: advisory
mutation_path: roadmap_update
---

# Sequencing

This document provides ordering rationale and dependency chain for the active production-path block.

## Roadmap surface roles

- `phase-map.md`: compact planned phase index.
- `phases.md`: active phase catalog with expanded block detail.
- `sequencing.md`: ordering rationale and dependency chain.
- `CHANGELOG.md`: historical truth only.

## Dependency chain rationale (Phases 71-80)

1. **Durability before provider execution**
   - Physical state boundaries and verification semantics must be explicit before widening live provider pathways.
   - This preserves fail-closed behavior when execution outputs eventually become storable/recoverable artifacts.

2. **Provider execution before UI transport**
   - Rust-owned provider execution boundaries must exist before UI transport wiring, so UI carries typed contracts rather than defining execution semantics.

3. **UI transport before UI submission wiring**
   - Submission wiring depends on a typed request/response transport boundary; otherwise intent submission semantics drift or become implicit.

4. **UI submission before action execution**
   - Intent ingress and authorization/audit eligibility must be wired and validated before the first executable action path is allowed.

5. **Action execution before end-to-end harness run**
   - A bounded local end-to-end workflow requires provider execution, authorization, audit, persistence, recovery, and projection to be connected in sequence.

6. **Phase 80 is a gap audit, not approval**
   - Phase 80 evaluates evidence gaps and remaining production-candidate prerequisites.
   - It does not approve release-candidate readiness, production readiness, or public usability.

## 60s-to-70s transition note

If older roadmap wording shows “Phase 70 - Roadmap and Changelog Alignment Check,” planning is now realigned to:

**Phase 70 - Roadmap Documentation Realignment and Production Candidate Gap Audit**

This retains Phase 70 as documentation/alignment scope while making documentation-role separation explicit.
