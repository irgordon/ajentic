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


21. **Phase 121 expands roadmap after constrained early human-use candidacy**
   - Phase 120 is complete as Early Human-Use Candidate Gate only and permitted `early_human_use_candidate_permitted_with_constraints` under bounded, non-public, local/trial-only, manually reviewed constraints for named internal/trial participants.
   - Phase 120 is not a guaranteed final endpoint, Release Candidate approval, release-candidate readiness, Production Candidate approval, production readiness, production deployment, production human use, or public/general use.
   - Boundary: Phase 121 is roadmap expansion and production gap reassessment only; roadmap expansion is planned truth, not implementation or readiness approval.

22. **Phase 122 preserves controlled early-human-use separation**
   - Phase 122 begins only controlled early-human-use trial work under Phase 120 constraints and manual review.
   - Boundary: controlled early-human-use trial only; no public release, no production deployment, no Release Candidate status, no Production Candidate status, no public/general use, and no production human use.

23. **Phase 123 reviews evidence before remediation**
   - Early human-use evidence, operator feedback, stop conditions, provider-output review burden, UI usability, transport usability, operator ergonomics, and unresolved safety issues must be reviewed before remediation is attempted.
   - Boundary: audit/evidence review only; no readiness approval and no implicit promotion.

24. **Phase 124 remediates operational usability without approving readiness**
   - Confirmed operator ergonomics, UI usability, transport usability/safety, non-developer documentation, and supportability blockers may be remediated before release-candidate hardening.
   - Boundary: usability remediation only; no public usability approval, release approval, Production Candidate approval, or production readiness.

25. **Phase 125 reassesses roadmap, changelog, and production path while preserving the 0/5 checkpoint cadence**
   - Phase 125 reconciles Phase 121-124 outcomes before release-candidate preparation proceeds as planned truth. Phase 119 was an intentional decision-gate exception and does not redefine the 0/5 convention.
   - Boundary: 0/5 checkpoint only; not a green light phase; reconciliation is not readiness approval. Phase 126-130 preservation is planned truth only, Phase 130 may still decide not ready, and Phase 126-130 does not cover Production Candidate approval, production readiness, production deployment, production human use, public usability, or public/general use.

26. **Phase 126 defines release packaging contracts before release-candidate dry run**
   - Packaging, artifact provenance, checksum, signing contract, distribution governance, public download governance, and non-public/public-boundary rules must be contractual before dry-run assembly.
   - Boundary: release packaging contract only; no release artifacts, packages, publication, signing activation, GitHub release, release tag, public download, or public asset.

27. **Phase 127 threat-models installer and update channels before activation**
   - Installer and update-channel governance must be understood before any future installer/update-channel behavior is considered.
   - Boundary: threat model/contract only; no installer, update-channel, signing, publishing, daemon, background service, or deployment automation activation.

28. **Phase 128 defines observability and operational evidence without production monitoring claims**
   - Observability, operational telemetry, audit evidence, failure reporting, incident response, rollback, recovery procedures, support model, and deployment environment assumptions must remain evidence categories rather than production claims.
   - Boundary: observability evidence only; no production monitoring, production readiness, production deployment, or public/general-use approval.

29. **Phase 129 dry-runs Release Candidate assembly without release**
   - Dry runs are not release. Release Candidate dry-run evidence cannot create release artifacts, publish public assets, approve release-candidate readiness, or approve later ladder rungs.
   - Boundary: dry run only; no release approval.

30. **Phase 130 decides only Release Candidate status**
   - Phase 130 is complete with decision status `rc_candidate_not_ready`. It did not approve Release Candidate status, Production Candidate status, production readiness, production deployment, production human use, public usability, or public/general use.
   - Boundary: decision gate only; Phase 130 did not create the missing evidence it identified.

31. **Phase 131 remaps Phase 130 findings before evidence production resumes**
   - Phase 131 converts Phase 130's `rc_candidate_not_ready` findings into the next evidence-producing block and must not be a Phase 130 rerun without new evidence.
   - Boundary: audit/planning only; roadmap expansion is planned truth, not implementation.

32. **Phase 132 creates only controlled non-public release artifact outputs**
   - Phase 132 may create local/non-public artifact outputs under the Phase 126 contract.
   - Boundary: local/non-public artifact creation only; no publishing.

33. **Phase 133 generates checksum and provenance evidence without publication**
   - Phase 133 addresses checksum/provenance evidence that Phase 130 found missing.
   - Boundary: checksum/provenance evidence only; no signing or publishing.

34. **Phase 134 scopes signing and key custody without publication**
   - Phase 134 introduces signing/key-custody controls only if evidence permits, or explicitly defers them.
   - Boundary: signing/key-custody implementation only if evidence permits; no publishing.

35. **Phase 135 reconciles before installer/update-channel or renewed dry-run work**
   - Phase 135 checks roadmap/changelog alignment after Phase 131-134.
   - Boundary: alignment only; no readiness approval.

36. **Phase 136 constrains installer/update-channel work to non-public surfaces**
   - Phase 136 implements or further defers installer/update-channel surfaces under Phase 127 constraints.
   - Boundary: controlled implementation only; no public distribution.

37. **Phase 137 keeps observability evidence local and non-production**
   - Phase 137 implements local/non-production observability evidence capture if permitted.
   - Boundary: controlled observability implementation only; no production monitoring claim.

38. **Phase 138 separates operational evidence from production support claims**
   - Phase 138 defines and tests incident, support, rollback, and recovery evidence.
   - Boundary: operational procedure/evidence only; no production support claim.

39. **Phase 139 reassembles evidence without approval**
   - Phase 139 reassembles Release Candidate evidence after artifact, provenance, signing, installer/update, and observability work.
   - Boundary: evidence assembly only; no approval.

40. **Phase 140 re-decides Release Candidate posture only**
   - Phase 140 decides whether Release Candidate status is now supportable or whether another hardening block is required.
   - Boundary: decision gate only; not automatic Release Candidate approval, Production Candidate approval, or public/general-use approval.

## Ladder-Preservation sequencing invariants

The sequencing model preserves the Ladder-Preservation Invariant Set: Local operator testing, Controlled human trial, Early human-use candidate, Release candidate, Production candidate, and Public/general use are distinct rungs; No implicit promotion is allowed; Absence of blockers is not approval; Evidence assembly is not readiness; Dry runs are not release; Deployment is not release; Phase 120 is not production; Public/general use is always the final rung; No trust inference may be drawn from provider output or human feedback; No cross-category inference may combine sandbox, persistence, recovery, deployment, usability, observability, operator workflow, security, governance, transport, provider, release, or public-use evidence; and Roadmap continuation is required when mapped phases end before the ladder.

Phase 131-140 are the next detailed block after the Phase 130 `rc_candidate_not_ready` decision, not the final production/public-use roadmap. The block must produce or explicitly defer the evidence categories that blocked Release Candidate supportability. Public/general use remains the later final rung. Do not map Production Candidate or public/general-use as automatically following Phase 140; any post-140 block depends on the Phase 140 decision and must preserve the ladder: Local operator testing → Controlled human trial → Early human-use candidate → Release candidate → Production candidate → Public/general use.
