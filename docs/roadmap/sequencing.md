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

Phase 110 is now the immediate roadmap/changelog alignment gate after Phase 109. Phase 110 does not equal persistence activation, readiness approval, Production Candidate approval, release-candidate approval, public-use approval, or production approval.

Phases 101-120 are planned truth only. They do not imply implementation, readiness, public usability, release-candidate status, Production Candidate status, or production approval.

Production human use is staged in this ladder:

1. Local operator testing
2. Controlled human trial
3. Early human-use candidate
4. Release candidate
5. Production candidate
6. Public/general use

The post-100 roadmap separates UI activation, live local transport, provider execution, alignment/decision checkpoints, narrow persistence activation, recovery lifecycle hardening, deployment configuration, policy/governance versioning, security, local deployment candidacy, human trial, release-candidate evidence, Production Candidate reassessment, and controlled human-use gate work.

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

9. **Phase 109 decides persistence authority before any activation**
   - Persistence authority requires Phase 109 decision evidence before any activation because local persistence must not become authoritative by implementation drift.
   - Boundary: decision/audit only; the only permitted later candidate is Rust-validated decision-evidence append under explicit exclusions.

10. **Phase 110 aligns roadmap and changelog truth before activation**
    - Phase 110 reconciles Phase 106-109 outcomes, corrects stale current-block/immediate-gate language, preserves historical changelog entries, and confirms whether Phase 109 constraints remain valid.
    - Boundary: alignment/check only; no runtime behavior, no new capability, no persistence authority, and no Phase 111 implementation.

11. **Phase 111 is the earliest possible narrow persistence activation boundary**
    - Phase 111 may begin only if Phase 109/110 constraints remain valid and may implement only Rust-validated decision-evidence append under Phase 109 exclusions.
    - Boundary: not broad persistence authority; no provider-output authority, replay repair, recovery promotion, action execution, or readiness approval.

12. **Phase 112 preserves recovery lifecycle hardening before deployment contracts**
    - Recovery lifecycle hardening follows narrow decision-evidence append activation so corrupt, unsupported, conflicting, duplicate, and hostile recovery inputs remain fail-closed before deployment configuration names storage assumptions.
    - Boundary: recovery lifecycle hardening only; no replay repair, recovery promotion, action execution, readiness approval, or Phase 113 implementation.

13. **Phase 113 defines deployment configuration before local deployment candidacy**
    - Deployment configuration must be contractual before any local deployment candidate boundary is named, must not add deployment automation, and must consume Phase 112 recovery handoff gaps for storage paths, permissions, retention, environment assumptions, failure handling, manual review, no background repair, no automatic replay patching, no continue-anyway behavior, no migration/version upgrade authority, no production recovery guarantee, and no release evidence guarantee.
    - Boundary: deployment config only; no deployment automation.

14. **Phase 114 adds policy/governance evidence traceability as planned future work**
    - Policy Versioning and Governance Evidence Boundary was moved out of completed Phase 112 and remains planned future work so later deployment, security, and trial evidence can be attributable without rewriting governance authority.
    - Boundary: policy/governance versioning only; not completed Phase 112 work.

15. **Phase 115 separates security audit from deployment and trial work**
    - Security audit remains a separate gate because threat modeling, abuse cases, trust boundaries, and residual attack surfaces must be reviewed before local deployment candidacy and human-trial procedure work.
    - Boundary: security audit only.

16. **Phase 116 defines a local deployment candidate after deployment configuration and security audit evidence**
    - A local deployment candidate gives later trial work a controlled, non-public target after configuration and security posture have been reviewed.
    - Boundary: local deployment candidate only; no public release.

17. **Phase 117 prepares and rehearses controlled human-trial procedures before release evidence assembly**
    - Human trial documentation and dry run remain non-public and do not grant public availability.
    - Boundary: operator docs and dry run only; no readiness approval and no public availability.

18. **Phase 118 assembles release-candidate evidence without approval**
    - Release-candidate evidence is separate from release-candidate approval; Phase 118 is Release Candidate Evidence Assembly, not Release Candidate approval.
    - Boundary: evidence assembly only; no automatic approval.

19. **Phase 119 reassesses Production Candidate posture after evidence assembly**
    - Production Candidate reassessment is separate from automatic approval; Phase 119 is Production Candidate Reassessment, not automatic approval.
    - Boundary: decision gate only.

20. **Phase 120 decides controlled early human-use candidacy without public/general use**
    - The early human-use candidate gate is a current planned gate, not a guaranteed final endpoint; it is not general public release.
    - Boundary: current planned gate for controlled human use; not a guaranteed final endpoint and not general public release.

Deployment configuration, policy/governance versioning, security audit, local deployment candidacy, human trial, release-candidate evidence, and Production Candidate reassessment are separate gates because each answers a different evidence question: configuration shape, evidence attribution, abuse and trust posture, controlled target definition, human procedure fidelity, candidate evidence assembly, and production-candidate posture.
