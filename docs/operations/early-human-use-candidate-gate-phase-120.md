---
truth_dimension: orientation
authority_level: advisory
mutation_path: readme_update
---

# Phase 120 - Early Human-Use Candidate Gate

## Scope
Phase 120 is Early Human-Use Candidate Gate only. It decides whether committed, phase-scoped evidence supports a bounded early human-use candidate posture after Phase 119 recommended Phase 120 only as an early human-use gate.

Phase 120 does not change runtime behavior, tests, schemas, UI behavior, deployment behavior, provider behavior, persistence behavior, recovery behavior, action behavior, packaging behavior, release behavior, or public-use behavior.

## Evidence rule
Phase 120 counts only committed evidence: source files, tests, UI behavior tests, validation scripts, governance docs, architecture docs, roadmap docs, operations docs, changelog surfaces, checklists, schemas, and CI/workflow files.

Phase 120 does not count prompt intent, prior chat summaries, uncommitted work, speculative roadmap claims, future phase descriptions as implemented behavior, passing validation as release approval, passing validation as readiness approval, Phase 118 evidence assembly as release-candidate readiness, Phase 119 reassessment as early human-use approval, Phase 117 dry-run documentation as human-use approval, Phase 116 local deployment candidacy as early human-use approval, Phase 115 security audit evidence as human-use approval, operator documentation as operational approval, absence of blockers as approval, or Phase 120 placement as production endpoint.

## Phase 120 gate boundary
Phase 120 may decide only whether early human-use candidacy is permitted with constraints. Phase 120 is not a guaranteed final endpoint.

Phase 120 does not create release artifacts, packages, installers, updates, signatures, publications, GitHub releases, release tags, public downloads, or public assets. Phase 120 does not add public release behavior, production deployment behavior, deployment automation, background services, daemon behavior, provider execution expansion, persistence authority expansion, replay repair, recovery promotion, action execution, provider trust, provider output promotion, or Phase 121 implementation.

## Early human-use candidate gate is not production approval
Phase 120 is not production readiness, not production deployment, not production approval, and not production human use.

**Status:** not_approved for production approval and production readiness.

## Early human-use candidate gate is not Release Candidate approval
Phase 120 is not Release Candidate approval and is not release-candidate readiness. Phase 118 evidence assembly remains evidence assembly only.

**Status:** not_approved for Release Candidate status and release-candidate readiness.

## Early human-use candidate gate is not Production Candidate approval
Phase 120 is not Production Candidate approval. Phase 119 did not approve Production Candidate status, and Phase 120 does not upgrade that decision.

**Status:** not_approved for Production Candidate status.

## Early human-use candidate gate is not public/general use
Phase 120 is not public/general use, public usability, broad availability, public distribution, or general availability. Public/general use remains the final rung.

**Status:** not_approved for public/general use.

## Early human-use candidate gate is not production human use
Phase 120 is not production human use. Early human-use candidacy, if permitted, is bounded, non-public, and explicitly constrained.

**Status:** not_approved for production human use.

## Decision status model
Phase 120 uses only these decision statuses:

| Status | Meaning |
| --- | --- |
| not_approved | The adjacent rung, readiness claim, release claim, production claim, public-use claim, or authority claim is not approved by Phase 120. |
| insufficient_evidence | Committed evidence is missing, incomplete, or not phase-scoped for the decision. |
| blocked | A stop condition, residual risk, or missing authority prevents the decision from advancing. |
| deferred | The decision belongs to Phase 121 or later roadmap work. |
| proceed_to_phase_121_roadmap_expansion | Phase 121 must expand or explicitly defer unmapped later rungs after Phase 120. |
| early_human_use_candidate_permitted | Committed Phase 120 disposition evidence supports only a constrained early human-use candidate posture. |
| early_human_use_candidate_not_permitted | Committed Phase 120 disposition evidence does not support early human-use candidacy. |
| not_applicable | The authority or behavior is prohibited or outside Phase 120 scope. |

Phase 120 outcome class: **early_human_use_candidate_permitted_with_constraints**.

## Phase 119 relationship
Phase 119 is complete. Phase 119 was a decision gate only and an intentional decision-gate exception to the usual 0/5 checkpoint cadence. Phase 119 did not approve Production Candidate status, Release Candidate status, release-candidate readiness, controlled human use, early human use, public/general use, production human use, public release, production deployment, or production readiness.

Phase 119 recommended Phase 120 only as Early Human-Use Candidate Gate, required fresh Phase 120 disposition evidence, stated Phase 120 is not a guaranteed final endpoint, required post-120 roadmap expansion because later ladder rungs remain unmapped, and recommended Phase 121 as roadmap expansion or explicit deferral of unmapped rungs after Phase 120.

## Fresh Phase 120 disposition evidence
Fresh Phase 120 disposition evidence is this report's classification of committed surfaces as of Phase 120. The classification is phase-scoped and does not inherit approval from Phase 118, Phase 119, Phase 117, Phase 116, Phase 115, or validation success.

| Evidence category | Phase 120 disposition | Status |
| --- | --- | --- |
| Governance and architecture | Rust authority, UI non-authority, model-output distrust, and typed-intent boundaries remain authoritative constraints. | early_human_use_candidate_permitted |
| Roadmap | Phase 120 is planned as Early Human-Use Candidate Gate, not final production endpoint; ladder remains explicit. | early_human_use_candidate_permitted |
| Runtime/API evidence | Local deployment, deployment configuration, policy/governance evidence, persistence decision, provider configuration, provider sandbox, and local transport surfaces reject adjacent authority claims and keep provider output untrusted. | early_human_use_candidate_permitted |
| Tests and validation scripts | Regression and boundary checks exist for transport, provider, persistence, UI spoofing, and repository structure; validation success is evidence only, not readiness. | early_human_use_candidate_permitted |
| Operations history | Phases 115-119 document security, local deployment, dry-run, evidence assembly, and reassessment without approving later rungs. | early_human_use_candidate_permitted |
| Release/public-use evidence | No committed evidence creates public release, public use, production deployment, release artifacts, installers, update channels, signing, publishing, GitHub releases, release tags, or public downloads. | not_approved |

## Ladder-Preservation Invariant Set
Phase 120 applies the Ladder-Preservation Invariant Set directly:

1. Ladder steps are not interchangeable.
2. No implicit promotion is allowed.
3. Absence of blockers is not approval.
4. Evidence assembly is not readiness.
5. Dry runs are not release.
6. Decision gates may approve only their explicitly authorized decision.
7. No phase may retroactively rewrite earlier gates.
8. Human use is not binary.
9. Deployment is not release.
10. No phase may claim to be the final gate.
11. Public/general use is always the final rung.
12. No trust inference is allowed from provider output or human feedback.
13. No cross-category inference is allowed across sandbox, persistence, recovery, deployment, usability, observability, operator workflow, security, governance, transport, provider, release, and public-use evidence.
14. No phase may activate authority without explicit roadmap permission.
15. Every rung requires fresh phase-scoped evidence, not inherited evidence.
16. Roadmap continuation is required when mapped phases end before the ladder.

## Production-human-use ladder
The staged ladder remains:

1. Local operator testing
2. Controlled human trial
3. Early human-use candidate
4. Release candidate
5. Production candidate
6. Public/general use

Phase 120 decides only the Early human-use candidate rung. It does not collapse, merge, reorder, skip, or approve later rungs.

## Ladder separation assessment
The committed roadmap and operations surfaces preserve the ladder as separate rungs. Phase 120 permits only the early human-use candidate rung with constraints and leaves Release candidate, Production candidate, and Public/general use unmapped for later roadmap expansion.

**Status:** early_human_use_candidate_permitted for the constrained rung; deferred for later rungs.

## No implicit promotion assessment
Phase 120 does not infer approval from validation, dry-run evidence, security audit evidence, local deployment candidate evidence, release-candidate evidence assembly, Production Candidate reassessment, provider output, or operator notes.

**Status:** not_approved for every implicit adjacent-rung promotion.

## Absence-of-blockers assessment
No Phase 120 finding treats the absence of blockers as approval. The positive decision rests on the fresh Phase 120 disposition evidence in this report and the explicit constraints below.

**Status:** early_human_use_candidate_permitted with constraints only.

## Evidence-category separation assessment
Sandbox, persistence, recovery, deployment, usability, observability, operator workflow, security, governance, transport, provider, release, and public-use evidence remain separate. No category is used to satisfy another category's approval requirement.

**Status:** early_human_use_candidate_permitted with category separation maintained.

## Provider-output and human-feedback trust assessment
Provider output remains untrusted. Human feedback and operator notes remain review evidence only. Phase 120 grants no provider trust and no provider output promotion.

**Status:** not_approved for trust and promotion authority.

## Deployment-is-not-release assessment
Deployment configuration and local deployment candidate evidence do not imply release, public usability, production deployment, production readiness, or production status.

**Status:** not_approved for release and production deployment claims.

## Phase 120 is not production assessment
Phase 120 is not production, not production readiness, not production deployment, not production human use, and not Production Candidate approval.

**Status:** not_approved.

## Public/general use final-rung assessment
Public/general use is always the final rung. Phase 120 does not approve public/general use and does not shorten the ladder.

**Status:** deferred and not_approved.

## Early human-use candidate posture assessment
Committed evidence supports a bounded early human-use candidate posture only because the repository preserves Rust-owned authority, rejects provider trust and adjacent approvals, documents local-only and manual-review boundaries, preserves non-public constraints, and keeps release, production, deployment, and public-use authorities inactive.

The posture remains advisory and constrained. It is not readiness, release, production, public use, or production human use.

## Early human-use candidate decision
**Decision:** early_human_use_candidate_permitted.

**Outcome class:** early_human_use_candidate_permitted_with_constraints.

This decision permits only a bounded early human-use candidate posture under the constraints in the next section. Any violation of those constraints stops the posture and requires future disposition.

## Early human-use candidate constraints if permitted
The Phase 120 early human-use candidate posture is constrained to:

- local-only or explicitly bounded non-public use
- named internal/trial participants only
- manual review required
- Trial coordinator ownership required
- Security reviewer escalation required for security issues
- Release steward escalation required for release, public-use, or deployment claims
- no public/general availability
- no production deployment
- no release artifacts
- no packages
- no installers, update channels, signing, or publishing
- no GitHub release, release tag, public downloads, or public assets
- no provider trust
- no provider output promotion
- no action execution
- no replay repair
- no recovery promotion
- no persistence expansion
- no readiness upgrade
- stop-on-validation-failure
- stop-on-residual-risk escalation
- stop-on-boundary drift
- stop-on-public-use or production-use claim
- evidence capture required
- Phase 121 roadmap expansion required after Phase 120

## Early human-use candidate denial rationale if not permitted
Not applicable. Phase 120 does not deny the early human-use candidate posture because fresh committed disposition evidence supports the constrained posture. This non-denial does not approve Release Candidate status, Production Candidate status, public/general use, production deployment, production readiness, production human use, or any release behavior.

**Status:** not_applicable.

## Evidence supporting early human-use candidacy
| Evidence | Phase 120 finding | Constraint retained |
| --- | --- | --- |
| Governance and architecture authority model | Rust owns runtime authority; UI, Bash, docs, and model output do not create authority. | Manual review and Rust-owned validation remain required. |
| Local transport and UI surfaces | Local review and transport surfaces are bounded and non-authoritative. | Local-only and non-public use only. |
| Provider configuration and sandbox surfaces | Provider output is untrusted and no provider trust is granted. | No provider trust or output promotion. |
| Persistence and recovery surfaces | Narrow persistence evidence and recovery hardening do not expand authority, repair replay, or promote recovery. | No persistence expansion, replay repair, or recovery promotion. |
| Deployment and local deployment candidate surfaces | Deployment configuration and local deployment candidate evidence remain local, non-public, and non-release. | No deployment automation or production deployment. |
| Security, dry-run, evidence assembly, and reassessment operations docs | Prior phases document risks, dry-run boundaries, evidence assembly, and reassessment without approving adjacent rungs. | Phase 120 uses these as context only, not inherited approval. |

## Evidence blocking early human-use candidacy
No committed Phase 120 evidence blocks the constrained early human-use candidate posture. The posture stops if validation fails, residual risk escalates, boundary drift appears, public/general-use claims appear, production-use claims appear, or any prohibited authority is requested.

**Status:** early_human_use_candidate_permitted with active stop conditions.

## Evidence gaps blocking Release Candidate status
Release Candidate status remains blocked by missing phase-scoped release-candidate approval evidence, missing release artifact governance, missing release process authority, missing distribution constraints, missing public asset controls, and missing Phase 121-or-later roadmap mapping.

**Status:** not_approved and deferred.

## Evidence gaps blocking Production Candidate status
Production Candidate status remains blocked by Phase 119 non-approval, missing release-candidate decision evidence, missing production readiness authority, missing production deployment evidence, unresolved residual risks, and missing later-rung roadmap mapping.

**Status:** not_approved and deferred.

## Evidence gaps blocking public/general use
Public/general use remains blocked by absence of public usability approval, public distribution authority, release artifacts, public download controls, production operations evidence, public support posture, and later-rung roadmap mapping.

**Status:** not_approved and deferred.

## Evidence gaps blocking production human use
Production human use remains blocked by absence of production approval, production readiness approval, production deployment authority, public/general-use gate disposition, operational support evidence, incident response disposition, and later-rung roadmap mapping.

**Status:** not_approved and deferred.

## Residual-risk carry-forward
| Source | Residual risk | Phase 120 disposition |
| --- | --- | --- |
| Phase 115 | Security threat model and abuse-case residual risks require escalation if security issues arise. | Carry forward; Security reviewer escalation required. |
| Phase 118 | Release-candidate evidence assembly did not approve readiness or release status. | Carry forward; release steward handles any release claim. |
| Phase 119 | Production Candidate status was not approved and post-120 roadmap expansion was required. | Carry forward; Phase 121 roadmap expansion required. |
| Provider surfaces | Provider output remains untrusted and cannot create authority. | Carry forward; no trust or promotion. |
| Deployment surfaces | Deployment configuration is not release or production deployment. | Carry forward; no deployment automation. |

## Stop-condition disposition
| Stop condition | Disposition |
| --- | --- |
| validation failure | Stop early human-use candidate posture and require disposition. |
| residual-risk escalation | Stop or pause trial posture until Security reviewer disposition. |
| boundary drift | Stop and require governance/roadmap review. |
| public-use claim | Stop and escalate to Release steward. |
| production-use claim | Stop and escalate to Release steward. |
| provider trust or output promotion request | Reject and require future roadmap authority. |
| replay repair, recovery promotion, or action execution request | Reject and require future roadmap authority. |
| persistence expansion request | Reject and require future roadmap authority. |

## Manual-review disposition
Manual review is mandatory. Named internal/trial participants may use the bounded posture only under Trial coordinator ownership. Security issues escalate to the Security reviewer. Release, public-use, deployment, readiness, or production claims escalate to the Release steward.

## Phase 121 roadmap expansion requirement
**Status:** proceed_to_phase_121_roadmap_expansion.

Phase 121 roadmap expansion is required after Phase 120 because Release candidate, Production candidate, public/general use, and production human-use rungs remain unmapped. Phase 121, if recommended, is roadmap expansion and production gap reassessment only.

## Phase 121 planning recommendation
Phase 121 should expand the roadmap or explicitly defer unmapped rungs before any Release Candidate, Production Candidate, public/general-use, production-readiness, production-deployment, or production-human-use gate. Phase 121 must not be implemented by Phase 120.

## Release artifact prohibition
Phase 120 creates no release artifacts and no release_artifact_created state.

## Deployment automation prohibition
Phase 120 adds no deployment automation and no deployment_automation behavior.

## Installer/update-channel prohibition
Phase 120 adds no installer_enabled behavior and no update_channel_enabled behavior.

## Signing/publishing prohibition
Phase 120 adds no signing_enabled behavior and no publishing_enabled behavior.

## GitHub release/tag/public asset prohibition
Phase 120 creates no github_release_created state, no release_tag_created state, and no public_download or public asset behavior.

## Public-release prohibition
Phase 120 adds no public_release_enabled behavior and grants no public release authority.

## Production-deployment prohibition
Phase 120 adds no production_deployment_enabled behavior and grants no production deployment authority.

## Public/general-use approval prohibition
Phase 120 does not approve public/general use.

**Status:** not_approved.

## Production-human-use approval prohibition
Phase 120 does not approve production human use.

**Status:** not_approved.

## Production Candidate approval prohibition
Phase 120 does not approve Production Candidate status.

**Status:** not_approved.

## Release-candidate approval prohibition
Phase 120 does not approve Release Candidate status or release-candidate readiness.

**Status:** not_approved.

## Provider trust/output promotion prohibition
Phase 120 grants no trust_granted state, no provider trust, and no provider output promotion.

**Status:** not_approved.

## Replay-repair prohibition
Phase 120 adds no replay_repaired behavior and no replay repair authority.

**Status:** not_approved.

## Recovery-promotion prohibition
Phase 120 adds no recovery_promoted behavior and no recovery promotion authority.

**Status:** not_approved.

## Action-execution prohibition
Phase 120 adds no action_executed behavior and no action execution authority.

**Status:** not_approved.

## Readiness approval prohibition
Phase 120 grants no readiness_approved state and no readiness upgrade.

**Status:** not_approved.

## Required future implementation evidence
Future rungs require their own phase-scoped evidence, explicit roadmap permission, source/test/schema validation where applicable, separate release and public-use governance, production operations evidence, and changelog entries that match actual behavior.

## Phase 121 gate decision
Phase 120 decision for Phase 121: **proceed_to_phase_121_roadmap_expansion**.

Phase 121 must be roadmap expansion or explicit deferral of unmapped rungs after Phase 120.

## Phase 121-or-later deferrals
Release Candidate status, release-candidate readiness, Production Candidate status, public/general use, public usability, production readiness, production deployment, and production human use are deferred to Phase 121-or-later roadmap work.

## Production Candidate status
Production Candidate status: not_approved.

## Release-candidate readiness status
Release-candidate readiness status: not_approved.

## Public/general use status
Public/general use status: not_approved and final rung remains preserved.

## Roadmap/changelog truth posture
Roadmap remains planned truth. CHANGELOG surfaces remain historical truth. Phase 120 adds an advisory operations report and procedural checklist update only.

## Required follow-ups
- Perform Phase 121 roadmap expansion or explicit deferral of unmapped later rungs.
- Preserve ladder separation in all future planning.
- Keep release, production, deployment, public-use, and production-human-use approvals out of Phase 120.
- Capture evidence for any bounded early human-use activity under the constraints above.

## Deferred items
| Item | Disposition |
| --- | --- |
| Release Candidate status | deferred; not_approved |
| release-candidate readiness | deferred; not_approved |
| Production Candidate status | deferred; not_approved |
| public/general use | deferred; not_approved |
| production readiness | deferred; not_approved |
| production deployment | deferred; not_approved |
| production human use | deferred; not_approved |
| provider trust/output promotion | deferred; not_approved |
| replay repair/recovery promotion/action execution | deferred; not_approved |

## Confirmed vs suspected
| Classification | Finding |
| --- | --- |
| Confirmed | Phase 120 is Early Human-Use Candidate Gate only. |
| Confirmed | Phase 120 is not Release Candidate approval, not Production Candidate approval, not public/general use, not production readiness, and not production human use. |
| Confirmed | Phase 120 does not create release artifacts, packages, installers, update channels, signing, publishing, GitHub releases, release tags, public downloads, public assets, public release behavior, production deployment behavior, or deployment automation. |
| Confirmed | Phase 120 does not expand provider execution, persistence authority, replay repair, recovery promotion, action execution, provider trust, or provider output promotion. |
| Confirmed | Public/general use remains the final rung. |
| Confirmed | Phase 121 roadmap expansion is required after Phase 120. |
| Suspected | None recorded as approval evidence; suspected future needs remain deferred until phase-scoped evidence exists. |

## Non-readiness statement
Phase 120 is Early Human-Use Candidate Gate only. It permits a bounded early human-use candidate posture with constraints, but it is not a guaranteed final endpoint, not Release Candidate approval, not Production Candidate approval, not public/general use, not production readiness, not production deployment, not production human use, not release-candidate readiness, and not public usability.
