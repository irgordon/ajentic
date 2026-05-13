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

33. **Phase 133 completes the usable local operator UI shell**
   - Phase 133 added a usable local browser operator shell, Rust-owned local shell projection types, deterministic stub run flow, approve/reject controls, local non-production operator intent handling, and Rust and TypeScript tests.
   - Boundary: local-only code-production phase; no provider execution, production persistence, release artifact creation, or readiness approval.

34. **Phase 134 completes the Rust-owned local transport boundary**
   - Phase 134 added typed local transport handling for initial state, deterministic stub run, approve/reject intent submission, forbidden/malformed request rejection, and capability exposure, while keeping the UI non-authoritative.
   - Boundary: local-only code-production phase; no direct Rust-to-browser runtime bridge, provider execution, production persistence, release artifacts, installer behavior, signing, publishing, deployment, or readiness approval.

35. **Phase 135 remaps the next block into code-production mode**
   - Phase 135 reconciles Phase 133 and Phase 134 implementation work, updates roadmap/changelog/checklist surfaces, and hands Phase 136 the next concrete implementation step.
   - Boundary: 0/5 alignment checkpoint only; no Phase 136 implementation, source/test/schema changes, readiness approval, release/deployment claim, or provider execution.

36. **Phase 136 records local decisions in an in-memory ledger**
   - Phase 136 must record approve/reject decisions from the Rust-owned transport boundary into an in-memory typed local decision ledger and expose the decision timeline through the UI.
   - Boundary: code-production phase; must produce usable, testable Rust and TypeScript behavior without production persistence or provider execution.

37. **Phase 137 derives replay/status projection from local decisions**
   - Phase 137 must derive a replay/status projection from the in-memory local decision ledger and render it in the UI replay/status panel.
   - Boundary: code-production phase; must produce Rust replay projection logic, UI behavior, deterministic replay tests, and UI behavior tests without recovery promotion.

38. **Phase 138 exports local-only session evidence**
   - Phase 138 must generate an explicit local-only evidence artifact for the stub run, decision record, validation projection, and replay projection.
   - Boundary: code-production phase; executable export artifact only, with tests proving export content and no release/deployment claims.

39. **Phase 139 adds a constrained provider configuration stub without execution**
   - Phase 139 must add a visible local provider configuration surface with validation and fail-closed rejection of unsafe or unsupported settings.
   - Boundary: code-production phase; must produce Rust model, UI panel, validation tests, and UI tests; no provider execution or credential activation.

40. **Phase 140 reconciles the Phase 136-139 code-production block**
   - Phase 140 confirms the in-memory decision ledger, replay/status projection, local session evidence export preview, and constrained deterministic_stub provider configuration validation are aligned with code-production mode.
   - Boundary: 0/5 alignment checkpoint only; gate decision is `proceed_with_caveats` for Phase 141 sandboxed deterministic provider execution, not general provider execution, readiness, release, deployment, local model execution, or cloud provider execution approval.

41. **Phase 141 introduces only sandboxed deterministic provider execution**
   - Phase 141 must add a Rust-owned execution path for deterministic_stub as a sandboxed provider execution boundary.
   - Boundary: code-production phase; no arbitrary local model execution, cloud calls, shell commands, network sockets, default filesystem persistence, provider trust approval, or readiness claims.

42. **Phase 142 projects deterministic provider execution results**
   - Phase 142 must expose deterministic provider execution results through local transport and UI as typed projections.
   - Boundary: code-production phase; projection only, with no broad provider enablement or persistence authority.

43. **Phase 143 validates or rejects provider output before candidate conversion**
   - Phase 143 must route provider output through typed validation/rejection before it can become candidate output.
   - Boundary: code-production phase; provider output remains untrusted and cannot bypass Rust-owned validation.

44. **Phase 144 makes provider output review visible in the UI**
   - Phase 144 must render provider output, validation result, rejection reason, candidate conversion status, and operator review path.
   - Boundary: code-production phase; UI remains non-authoritative and cannot approve trust or readiness.

45. **Phase 145 reconciles the Phase 141-144 code-production block**
   - Phase 145 confirms sandboxed deterministic provider execution, execution result projection, provider output validation/rejection, and provider output review UI are aligned with code-production mode.
   - Boundary: 0/5 alignment checkpoint only; gate decision is `proceed_with_caveats` for Phase 146 staged candidate-conversion proposal creation only, not direct candidate conversion, provider-output approval, provider-output trust, readiness approval, release approval, Production Candidate status, or public/general use.

46. **Phase 146 creates only staged candidate-conversion proposals**
   - Phase 146 must create a Rust-owned staged candidate-conversion proposal from `reviewable_untrusted` provider output.
   - Boundary: code-production phase; usable, testable staged-proposal code only; no direct candidate output, provider-output approval, trust elevation, operator candidate decision, or accepted-provider-output mutation.

47. **Phase 147 validates staged conversion proposals**
   - Phase 147 must validate staged conversion proposals and reject malformed, unsafe, trust-claiming, approval-claiming, action-bearing, or boundary-crossing attempts.
   - Boundary: code-production phase; provider output remains untrusted and `reviewable_untrusted` is not candidate material.

48. **Phase 148 renders validated staged proposals without approval authority**
   - Phase 148 must render validated staged candidate-conversion proposals in the UI.
   - Boundary: code-production phase; display-only review surface with no provider-output approval, trust elevation, or approval controls.

49. **Phase 149 permits operator decisions only after validated staged proposals exist**
   - Phase 149 must allow approve/reject only for validated staged candidate proposals and record decisions through Rust-owned state.
   - Boundary: code-production phase; provider output must not jump directly to approved candidate output.

50. **Phase 150 aggressively remaps the next local-beta code-production block**
   - Phase 150 uses Phase 149 executable handoff evidence to remap Phases 151-160 into larger product capability phases documented in `docs/roadmap/phase-150-code-production-remap.md`.
   - Boundary: alignment only; no implementation, runtime behavior, readiness approval, release approval, Production Candidate status, or public/general-use approval.

51. **Phase 151 persists a local session package**
   - Phase 151 must produce a Rust-owned local session package with explicit caller-path write/read helpers, validation before write and after read, feasible UI save/restore status, and deterministic package tests.
   - Boundary: local session persistence only; no production persistence claim, public release artifact, installer/update behavior, signing, or publishing.

52. **Phase 152 makes restore and history visible**
   - Phase 152 must produce UI-visible restored-session state, session history/list projection, restore integrity/error display, and tests for restored UI state and failure display.
   - Boundary: no recovery promotion, production persistence claim, background daemon, or automatic remote sync.

53. **Phase 153 defines the real local adapter contract without execution**
   - Phase 153 must produce a Rust adapter contract, adapter capability surface, non-executing adapter registry, UI provider adapter configuration panel, and allowed/unsupported declaration tests.
   - Boundary: no real model execution, arbitrary shell command field, network/cloud, secret execution path, or provider trust approval.

54. **Phase 154 dry-runs a deterministic fake adapter through the real contract**
   - Phase 154 must execute a deterministic fake adapter through the adapter contract and route output through existing provider execution, result, validation, review, and staging flow.
   - Boundary: no arbitrary local model execution, network/cloud, or production claims.

55. **Phase 155 reconciles the first remapped product block**
   - Phase 155 must reconcile Phases 151-154 and decide whether constrained real local provider invocation may proceed in Phase 156.
   - Boundary: alignment only; no implementation, runtime behavior, readiness, release, or deployment approval.

56. **Phase 156 enables exactly one constrained real local provider invocation path**
   - Phase 156 must add one allowlisted invocation path with Rust validation, UI-visible invocation result, and tests rejecting arbitrary commands, unsafe paths, network/cloud, secrets, and unsupported providers.
   - Boundary: no shell-general execution, cloud/network, public release, production readiness, or provider trust approval.

57. **Phase 157 integrates real provider output into the existing pipeline**
   - Phase 157 must route real local provider output through the existing projection, validation, review, staging, staged validation, and decision path.
   - Boundary: no direct candidate materialization, provider-output trust, action execution, or production claims.

58. **Phase 158 materializes local candidate output under Rust-owned boundaries**
   - Phase 158 must materialize validated staged proposals into local candidate output with UI projection and tests for required staged validation and decision preconditions.
   - Boundary: local candidate output only; not production approval, release approval, or public-use approval.

59. **Phase 159 completes the local operator workflow**
   - Phase 159 must make the configure-to-export operator flow usable end to end, with UI workflow improvements, error drilldowns, run history/status summary, and end-to-end tests.
   - Boundary: local beta workflow only; no production readiness, public release, installer, or deployment claim.

60. **Phase 160 is the production-path alignment checkpoint**
   - Phase 160 reconciles Phases 151-159 and decides `proceed_with_caveats_to_controlled_internal_trial_packaging`.
   - Boundary: alignment only; no implementation, public release approval, production approval, automatic Release Candidate approval, or absence-of-blockers approval.

61. **Phase 161 creates a controlled internal trial package**
   - Phase 161 must produce a local-only package/checklist bundle for named internal trial operators.
   - Boundary: no public release, installer/update channel, signing, publishing, deployment, or readiness approval.

62. **Phase 162 improves operator runbook and failure drilldown usability**
   - Phase 162 must add operator-facing runbook surfaces and visible failure/rejection drilldowns.
   - Boundary: UI/operator usability only; no provider trust, action execution, or approval authority.

63. **Phase 163 captures local trial-session evidence**
   - Phase 163 must capture workflow status, decision state, local candidate materialization, replay/status, export, package/restore, and errors in a local trial evidence artifact.
   - Boundary: trial evidence artifact only; evidence export is not release evidence.

64. **Phase 164 verifies trial replay and restore deterministically**
   - Phase 164 must produce executable restore/replay verification for trial packages and evidence comparison.
   - Boundary: restore projection is not recovery promotion; replay/status projection is not replay repair.

65. **Phase 165 reconciles the first trial-packaging block**
   - Phase 165 reconciles Phases 161-164 and decides `proceed_with_caveats_to_controlled_internal_trial_execution_harness`.
   - Boundary: alignment only; no implementation, controlled human-use approval, release readiness, production readiness, deployment, or public/general use approval.

66. **Phase 166 adds the controlled internal trial execution harness**
   - Phase 166 must run a bounded internal trial workflow using the package, runbook, evidence, restore, and verification surfaces.
   - Boundary: deterministic local trial execution only; no public release, production approval, trial authority outside explicit local harness constraints, provider trust, action execution, or public/general use.

67. **Phase 167 adds local trial observability and error reporting**
   - Phase 167 must add local surfaces for failures, blocked states, restore issues, replay status, package validation, evidence validation, and verification mismatches.
   - Boundary: local trial observability only; no production monitoring claim, background telemetry, or network telemetry.

68. **Phase 168 adds the trial evidence review surface**
   - Phase 168 must add UI/evidence review for trial run evidence, operator notes, failure categories, verification results, stop-condition outcomes, and unresolved blockers.
   - Boundary: evidence review only; no readiness approval and absence of blockers is not approval.

69. **Phase 169 hardens local beta based on trial evidence**
   - Phase 169 must fix confirmed usability, restore, package, evidence, verification, runbook, observability, or workflow defects found in controlled trial evidence.
   - Boundary: concrete local-beta hardening code only; no readiness, release, deployment, signing, publishing, provider-output trust, candidate approval, or public/general-use approval.

70. **Phase 170 reconciles the controlled-trial/local-beta block**
   - Phase 170 decides `proceed_with_caveats_to_release_candidate_preparation_block` after reconciling the controlled internal trial execution harness, trial observability, trial error reporting, trial evidence review, local beta hardening, user-facing local HTML help, `help/index.html`, and the visible UI help entry point.
   - Boundary: alignment only; release-candidate preparation is not release readiness; no Release Candidate status, Production Candidate status, release readiness, production readiness, deployment, controlled human-use approval, or public/general use approval.

71. **Phase 171 defines the Release Candidate Preparation Contract**
   - Phase 171 must produce product-facing code, executable validation, or a concrete preparation contract surface from local beta evidence.
   - Boundary: no release artifacts and no release readiness approval.

72. **Phase 172 assembles a local dry package**
   - Phase 172 must produce a local, non-public dry package artifact through explicit caller-provided paths.
   - Boundary: no publishing, signing, installer, public download, or release approval.

73. **Phase 173 generates checksum and provenance evidence**
   - Phase 173 must produce deterministic checksum/provenance evidence for the dry package.
   - Boundary: no signing or publishing.

74. **Phase 174 exposes installer and distribution contract status**
   - Phase 174 must add a local installer/distribution contract projection and UI visibility.
   - Boundary: no public distribution and no update-channel activation.

75. **Phase 175 is a code-production alignment checkpoint**
   - Phase 175 must reconcile Phases 171-174 and decide whether signing/key-custody dry-run work or another hardening block may proceed.
   - Boundary: alignment only; no release approval, readiness approval, signing activation, publishing, deployment, or public/general use approval.

76. **Phase 176 performs signing and key-custody dry run work**
   - Phase 176 must add a local dry-run signing/key-custody evidence surface using test-only or placeholder metadata.
   - Boundary: no real signing keys, public signing, publishing, or release approval.

77. **Phase 177 assembles Release Candidate evidence in the UI**
   - Phase 177 must add a UI surface for dry-package, checksum/provenance, installer/distribution contract, signing dry run, help docs, local beta evidence, and validation results.
   - Boundary: evidence assembly only; no approval.

78. **Phase 178 hardens confirmed Release Candidate preparation gaps**
   - Phase 178 must ship concrete code or validation hardening for confirmed evidence/documentation/packaging gaps.
   - Boundary: no release approval.

79. **Phase 179 rehearses the Release Candidate dry run**
   - Phase 179 must produce a deterministic local dry-run evidence artifact.
   - Boundary: no release approval and no public artifact.

80. **Phase 180 is the Release Candidate Decision Gate**
   - Phase 180 must decide whether Release Candidate status is supportable or whether another hardening block is required.
   - Boundary: decision gate only; no release, production, deployment, public/general use, signing, publishing, or provider-output trust approval by implication.


Phase 170 remaps Phases 171-180 toward release-candidate preparation while preserving code-production requirements. Phases 171-174 and 176-179 must produce product-facing code, executable artifacts, release-candidate preparation artifacts, or deterministic validation improvements. Phase 175 remains alignment-only and Phase 180 remains a decision gate. User help is explanatory only, not authority, and not release documentation approval. Local HTML help pages are not production readiness evidence. Local beta workflow completion is not production readiness. Provider output remains untrusted, session package is not a release artifact, restore projection is not recovery promotion, replay/status projection is not replay repair, and absence of blockers is not approval.

## Ladder-Preservation sequencing invariants

The sequencing model preserves the Ladder-Preservation Invariant Set: Local operator testing, Controlled human trial, Early human-use candidate, Release candidate, Production candidate, and Public/general use are distinct rungs; No implicit promotion is allowed; Absence of blockers is not approval; Evidence assembly is not readiness; Dry runs are not release; Deployment is not release; Phase 120 is not production; Public/general use is always the final rung; No trust inference may be drawn from provider output or human feedback; No cross-category inference may combine sandbox, persistence, recovery, deployment, usability, observability, operator workflow, security, governance, transport, provider, release, or public-use evidence; and Roadmap continuation is required when mapped phases end before the ladder.

Phase 141-145 are reconciled as a completed provider-output code-production block. Phase 146-149 completed staged proposal and operator decision work, and Phase 150 remaps Phases 151-160 as larger product capability phases toward a usable local beta. Every non-0/5 phase in the remapped block must produce visible UI capability, executable Rust capability, persisted local artifact, restore/replay/export capability, real adapter integration step, or end-to-end operator workflow improvement. Safety checks remain embedded in implementation phases. Public/general use remains a later final rung. Do not map Production Candidate or public/general-use as automatically following Phase 145 or Phase 150.
