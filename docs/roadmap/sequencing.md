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

## Post-Phase-100 staged production human-use rationale

Phase 99.5 is planning and alignment only. It adds no runtime behavior, adds no new capability, does not approve production human use, does not approve Production Candidate status, does not approve release-candidate readiness, does not approve public usability, does not start Phase 100, and does not implement Phases 101-120.

Phase 100 remains the immediate Production Candidate gap audit and readiness decision gate. Phase 100 does not equal production because a Production Candidate gap audit can only decide whether the evidence posture supports a candidate branch/tag or requires more hardening; it does not approve production human use, release-candidate readiness, or public/general use.

Phases 101-120 are planned truth only. They do not imply implementation, readiness, public usability, release-candidate status, Production Candidate status, or production approval.

Production human use is staged in this ladder:

1. Local operator testing
2. Controlled human trial
3. Early human-use candidate
4. Release candidate
5. Production candidate
6. Public/general use

The post-100 roadmap separates UI activation, live local transport, provider execution, authoritative persistence, deployment, security, human trial, release-candidate evidence, Production Candidate reassessment, and controlled human-use gate work.

## Dependency chain rationale (Phases 101-120)

1. **Phase 101 decomposes production-use gaps before contracts are written**
   - Remaining blockers for human use must be named before operator workflow contracts can distinguish expected states from missing capability.
   - Boundary: audit/planning only.

2. **Phase 102 defines human operator workflows before UI activation**
   - Operator roles, workflows, and expected states provide the usability contract that Phase 103 can expose.
   - Boundary: documentation/contract only.

3. **Phase 103 activates review visibility before live local transport**
   - UI activation precedes live transport so humans can inspect review surfaces before any local communication prototype exists.
   - Boundary: UI usability only; no Rust authority and no live mutation.

4. **Phase 104 prototypes local transport before transport hardening**
   - The local prototype identifies the exact live bridge shape to harden.
   - Boundary: local transport prototype only; no provider execution and no persistence authority.

5. **Phase 105 hardens the live local bridge after the prototype exists**
   - Live transport hardening follows live transport prototype because malformed, spoofed, replayed, or hostile input tests need a concrete local bridge boundary.
   - Boundary: hardening only; no broad capability.

6. **Phase 106 defines provider configuration before provider execution**
   - Provider configuration precedes provider execution so secrets, provider selection, and intent remain contractual before any provider can run.
   - Boundary: configuration contract only; no live provider execution.

7. **Phase 107 introduces sandboxed provider execution before provider hardening**
   - Provider sandboxing keeps provider output untrusted and bounded before timeout/resource evidence is added.
   - Boundary: bounded provider execution only; provider output remains untrusted.

8. **Phase 108 adds timeout and resource limit enforcement before authority decisions**
   - Provider sandboxing and resource limit enforcement precede provider authority decisions because execution evidence must remain bounded before any adjacent authority question is considered.
   - Boundary: provider hardening only; no promotion authority.

9. **Phase 109 decides persistence authority before activation**
   - Persistence authority requires Phase 109 decision evidence before Phase 110 activation because local persistence must not become authoritative by implementation drift.
   - Boundary: decision/audit only.

10. **Phase 110 activates only the persistence authority permitted by Phase 109**
    - Authoritative persistence, if permitted, must remain narrow and traceable to Phase 109 evidence constraints.
    - Boundary: narrow persistence authority only if Phase 109 permits it.

11. **Phase 111 hardens recovery lifecycle after persistence posture is known**
    - Recovery lifecycle hardening must prevent silent recovery, silent repair, replay repair authority, and implicit promotion under the persistence posture selected by Phase 109/110.
    - Boundary: recovery lifecycle only; no silent recovery.

12. **Phase 112 adds policy/governance evidence traceability before deployment contracts**
    - Policy versioning and governance evidence make later deployment and trial evidence attributable without rewriting governance authority.
    - Boundary: policy/governance versioning only.

13. **Phase 113 defines deployment configuration before local deployment candidacy**
    - Deployment configuration must be contractual before any local deployment candidate boundary is named.
    - Boundary: deployment config only; no deployment automation.

14. **Phase 114 defines a local deployment candidate before security and trial gates**
    - A local deployment candidate gives the security audit and trial dry run a controlled, non-public target.
    - Boundary: local deployment candidate only; no public release.

15. **Phase 115 separates security audit from deployment and trial work**
    - Security audit remains a separate gate because threat modeling, abuse cases, trust boundaries, and residual attack surfaces must be reviewed before human-trial documentation.
    - Boundary: security audit only.

16. **Phase 116 prepares operator documentation before human trial dry run**
    - Human trial documentation must exist before any controlled trial rehearsal can assess procedure fidelity.
    - Boundary: operator docs only; no readiness approval.

17. **Phase 117 rehearses controlled human-trial procedures before release evidence assembly**
    - Human trial dry run remains non-public and does not grant public availability.
    - Boundary: dry run only; no public availability.

18. **Phase 118 assembles release-candidate evidence without approval**
    - Release-candidate evidence is separate from release-candidate approval; Phase 118 is Release Candidate Evidence Assembly, not Release Candidate approval.
    - Boundary: evidence assembly only; no automatic approval.

19. **Phase 119 reassesses Production Candidate posture after evidence assembly**
    - Production Candidate reassessment is separate from automatic approval; Phase 119 is Production Candidate Reassessment, not automatic approval.
    - Boundary: decision gate only.

20. **Phase 120 decides controlled early human-use candidacy without public/general use**
    - The early human-use candidate gate is a final gate for controlled human use; it is not general public release.
    - Boundary: final gate for controlled human use; not general public release.

Deployment, security audit, human trial, release-candidate evidence, and Production Candidate reassessment are separate gates because each answers a different evidence question: configuration shape, abuse and trust posture, human procedure fidelity, candidate evidence assembly, and production-candidate posture.
