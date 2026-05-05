---
truth_dimension: planned
authority_level: advisory
mutation_path: roadmap_update
---

# Sequencing

This document provides ordering rationale and dependency chain for the active block.

## Roadmap surface roles

- `phase-map.md`: compact planned phase index.
- `phases.md`: active expanded planning catalog.
- `sequencing.md`: ordering rationale and dependency chain.
- `CHANGELOG.md`: historical truth only.

## Why the previous 85-90 block was too compressed

The previous block coupled observability, export, hardening families, startup/package surfaces, and readiness-gate concerns too tightly.

That compression increased composition density before outward-facing surfaces were sufficiently separated by trust and mutation boundaries.

The Phase 85-100 split reduces composition density by separating scopes that look adjacent but are not equivalent:

- Observability snapshot != export encoding
- Export encoding != export write
- Export write != ledger append
- Hardening is split by attack surface
- Startup != packaging
- Packaging != release
- Release dry run != readiness approval
- Readiness decision != automatic approval

## Dependency chain rationale (Phases 85-100)

1. **Phase 85 aligns planning/historical truth before new outward-facing planning**
   - Reconciles Phase 81-84 outcomes against `CHANGELOG.md` historical truth.
   - Confirms roadmap surfaces remain planned truth.

2. **Phase 86 documents supported local operator workflows before expanding diagnostics surfaces**
   - User/operator expectations and failure boundaries become explicit.
   - Prevents ambiguity when snapshot/export boundaries are added as planned work.

3. **Phases 87-89 split observability/export into three non-equivalent boundaries**
   - Phase 87 defines read-only snapshot semantics only.
   - Phase 88 defines deterministic encoding only.
   - Phase 89 writes verified bundles through existing persistence boundaries only.

4. **Phase 90 performs an alignment/gap checkpoint before hardening block two**
   - Confirms whether planned observability/export sequencing is coherent.
   - Does not approve readiness.

5. **Phases 91-94 split hardening by attack surface**
   - Phase 91: transport abuse and submission spoofing.
   - Phase 92: authorization/audit/action mismatch and escalation.
   - Phase 93: persistence corruption, append drift, partial write, replay/recovery mismatch.
   - Phase 94: provider-output injection, replay tamper, failure-trace spoofing, retry escalation.

6. **Phase 95 realigns hardening outcomes before startup/package block**
   - Ensures hardening evidence is reconciled before usability-surface planning advances.

7. **Phases 96-99 split usability/release mechanics into four non-equivalent boundaries**
   - Phase 96 defines local startup command boundary.
   - Phase 97 defines packaging artifact boundary.
   - Phase 98 defines operator documentation/troubleshooting boundary.
   - Phase 99 executes release-engineering dry run mechanics only.

8. **Phase 100 is a decision gate and not automatic approval**
   - Decides whether evidence supports production-candidate branch/tag.
   - May require another hardening block instead of approval.

## Truth-surface reminder

Roadmap remains planned truth.

`CHANGELOG.md` remains historical truth.
