---
truth_dimension: procedural
authority_level: authoritative
mutation_path: checklist_revision
---

# Current Phase Checklist - Phase 120

## Phase name
- [x] Phase 120 - Early Human-Use Candidate Gate.

## Phase goal
- [x] Decide whether early human-use candidacy is permitted.
- [x] Use fresh Phase 120 disposition evidence.
- [x] Apply the Ladder-Preservation Invariant Set.
- [x] Preserve Phase 120 as a gate, not final production endpoint.
- [x] Do not implement Phase 121.
- [x] Do not change runtime/source/test behavior.
- [x] Do not create release artifacts.

## Working-tree hygiene gate
- [x] Run `git status --short` before edits.
- [x] Classify uncommitted files before edits.
- [x] Remove generated artifact drift before edits if present.
- [x] Keep staged files limited to allowed Phase 120 surfaces.

## Allowed surfaces
- [x] `docs/operations/early-human-use-candidate-gate-phase-120.md`.
- [x] `checklists/current-phase.md`.
- [x] `CHANGELOG.md`.

## Boundary rules
- [x] Phase 120 is Early Human-Use Candidate Gate only.
- [x] Phase 120 is not a guaranteed final endpoint.
- [x] Phase 120 is not Release Candidate approval.
- [x] Phase 120 is not Production Candidate approval.
- [x] Phase 120 is not public/general use.
- [x] Phase 120 is not production readiness.
- [x] Phase 120 is not production human use.
- [x] Phase 120 creates no release artifacts, packages, installers, update channels, signatures, publications, GitHub releases, release tags, public downloads, or public assets.

## Evidence-only checklist
- [x] Count only committed source, tests, UI behavior tests, validation scripts, governance docs, architecture docs, roadmap docs, operations docs, changelog surfaces, checklists, schemas, and CI/workflow files.
- [x] Do not count prompt intent, prior chat summaries, uncommitted work, speculative roadmap claims, future phase descriptions as implemented behavior, validation success as release approval, validation success as readiness approval, Phase 118 evidence assembly as readiness, Phase 119 reassessment as early human-use approval, Phase 117 dry-run documentation as human-use approval, Phase 116 local deployment candidacy as early human-use approval, Phase 115 security audit evidence as human-use approval, operator documentation as operational approval, absence of blockers as approval, or Phase 120 placement as production endpoint.

## Phase 119 relationship checklist
- [x] State Phase 119 is complete.
- [x] State Phase 119 is a decision gate only.
- [x] State Phase 119 is an intentional decision-gate exception to the usual 0/5 checkpoint cadence.
- [x] State Phase 119 did not approve Production Candidate status, Release Candidate status, release-candidate readiness, controlled human use, early human use, public/general use, production human use, or production readiness.
- [x] State Phase 119 recommended Phase 120 only as Early Human-Use Candidate Gate.
- [x] State Phase 120 requires fresh phase-scoped disposition evidence.
- [x] State post-120 roadmap expansion is required.

## Fresh Phase 120 disposition evidence checklist
- [x] Identify fresh Phase 120 disposition evidence.
- [x] Keep prior phases as context only.
- [x] Do not inherit approval from Phase 118, Phase 119, Phase 117, Phase 116, Phase 115, or validation success.

## Ladder-Preservation Invariant checklist
- [x] Ladder-Preservation invariant 1: ladder steps are not interchangeable.
- [x] Ladder-Preservation invariant 2: No implicit promotion.
- [x] Ladder-Preservation invariant 3: Absence of blockers is not approval.
- [x] Ladder-Preservation invariant 4: Evidence assembly is not readiness.
- [x] Ladder-Preservation invariant 5: Dry runs are not release.
- [x] Ladder-Preservation invariant 6: decision gates may approve only their explicitly authorized decision.
- [x] Ladder-Preservation invariant 7: no retroactive rewrite of earlier gates.
- [x] Ladder-Preservation invariant 8: human use is not binary.
- [x] Ladder-Preservation invariant 9: Deployment is not release.
- [x] Ladder-Preservation invariant 10: Phase 120 is not production and not final gate.
- [x] Ladder-Preservation invariant 11: Public/general use is always the final rung.
- [x] Ladder-Preservation invariant 12: No trust inference from provider output or human feedback.
- [x] Ladder-Preservation invariant 13: No cross-category inference.
- [x] Ladder-Preservation invariant 14: no authority activation without roadmap permission.
- [x] Ladder-Preservation invariant 15: every rung requires own evidence.
- [x] Ladder-Preservation invariant 16: Roadmap continuation is required.

## Production-human-use ladder checklist
- [x] Preserve Local operator testing.
- [x] Preserve Controlled human trial.
- [x] Preserve Early human-use candidate.
- [x] Preserve Release candidate.
- [x] Preserve Production candidate.
- [x] Preserve Public/general use.

## Ladder separation checklist
- [x] Keep early human-use candidate separate from Release candidate.
- [x] Keep Release candidate separate from Production candidate.
- [x] Keep Production candidate separate from Public/general use.
- [x] Do not collapse, merge, reorder, skip, or approve later rungs.

## No implicit promotion checklist
- [x] Do not promote validation success.
- [x] Do not promote security audit evidence.
- [x] Do not promote dry-run evidence.
- [x] Do not promote release-candidate evidence assembly.
- [x] Do not promote Production Candidate reassessment.

## Absence-of-blockers checklist
- [x] State absence of blockers is not approval.
- [x] Require positive, explicit, phase-scoped authority for the Phase 120 decision.

## Evidence-category separation checklist
- [x] Keep sandbox, persistence, recovery, deployment, usability, observability, operator workflow, security, governance, transport, provider, release, and public-use evidence separate.
- [x] Do not infer across categories.

## Provider-output/human-feedback trust checklist
- [x] State provider output remains untrusted.
- [x] State human feedback and operator notes do not imply trust, readiness, safety, or authority.

## Deployment-is-not-release checklist
- [x] State deployment configuration is not release.
- [x] State local deployment candidate evidence is not production deployment, release readiness, public usability, or production status.

## Phase 120 is not production checklist
- [x] State Phase 120 is not production.
- [x] State Phase 120 is not production readiness.
- [x] State Phase 120 is not production deployment.
- [x] State Phase 120 is not production human use.

## Public/general-use final-rung checklist
- [x] State Public/general use remains final rung.
- [x] Do not approve public/general use.

## Early human-use candidate posture checklist
- [x] Assess bounded early human-use candidate posture.
- [x] Preserve non-readiness language.
- [x] Preserve non-public language.

## Early human-use candidate decision checklist
- [x] Use required decision status model.
- [x] Record decision as `early_human_use_candidate_permitted`.
- [x] Record outcome class as `early_human_use_candidate_permitted_with_constraints`.
- [x] Do not use banned approval statuses.

## Early human-use candidate constraints checklist
- [x] Constrain to local-only or explicitly bounded non-public use.
- [x] Constrain to named internal/trial participants only.
- [x] Require manual review.
- [x] Require Trial coordinator ownership.
- [x] Require Security reviewer escalation for security issues.
- [x] Require Release steward escalation for release/public-use/deployment claims.
- [x] Prohibit public/general availability.
- [x] Prohibit production deployment.
- [x] Prohibit release artifacts, packages, installers, update channels, signing, publishing, GitHub release, release tag, public downloads, and public assets.
- [x] Prohibit provider trust, provider output promotion, action execution, replay repair, recovery promotion, persistence expansion, and readiness upgrade.
- [x] Require stop-on-validation-failure, stop-on-residual-risk escalation, stop-on-boundary drift, and stop-on-public-use or production-use claim.
- [x] Require evidence capture.
- [x] Require Phase 121 roadmap expansion after Phase 120.

## Evidence supporting early human-use checklist
- [x] Identify evidence supporting early human-use candidate posture.
- [x] Keep evidence constrained to Phase 120 decision.

## Evidence blocking early human-use checklist
- [x] Identify no committed blocker to constrained early human-use candidate posture.
- [x] Preserve active stop conditions.

## Evidence gap checklist
- [x] Identify evidence gaps blocking Release Candidate status.
- [x] Identify evidence gaps blocking Production Candidate status.
- [x] Identify evidence gaps blocking public/general use.
- [x] Identify evidence gaps blocking production human use.

## Residual-risk checklist
- [x] Carry residual risks from Phase 115.
- [x] Carry residual risks from Phase 118.
- [x] Carry residual risks from Phase 119.

## Stop-condition checklist
- [x] Document validation failure stop condition.
- [x] Document residual-risk escalation stop condition.
- [x] Document boundary drift stop condition.
- [x] Document public-use or production-use claim stop condition.

## Manual-review checklist
- [x] Require manual review.
- [x] Require named ownership and escalation paths.

## Phase 121 roadmap expansion checklist
- [x] Record `proceed_to_phase_121_roadmap_expansion`.
- [x] State Phase 121 should expand roadmap or explicitly defer unmapped rungs.
- [x] Do not implement Phase 121.

## Release artifact prohibition checklist
- [x] Confirm no release_artifact_created behavior.

## Deployment automation prohibition checklist
- [x] Confirm no deployment_automation behavior.

## Installer/update-channel prohibition checklist
- [x] Confirm no installer_enabled behavior.
- [x] Confirm no update_channel_enabled behavior.

## Signing/publishing prohibition checklist
- [x] Confirm no signing_enabled behavior.
- [x] Confirm no publishing_enabled behavior.

## GitHub release/tag/public asset prohibition checklist
- [x] Confirm no github_release_created behavior.
- [x] Confirm no release_tag_created behavior.
- [x] Confirm no public_download/public asset behavior.

## Public-release prohibition checklist
- [x] Confirm no public_release_enabled behavior.

## Production-deployment prohibition checklist
- [x] Confirm no production_deployment_enabled behavior.

## Public/general-use approval prohibition checklist
- [x] Record public/general use as `not_approved`.

## Production-human-use approval prohibition checklist
- [x] Record production human use as `not_approved`.

## Production Candidate approval prohibition checklist
- [x] Record Production Candidate status as `not_approved`.

## Release-candidate approval prohibition checklist
- [x] Record Release Candidate status as `not_approved`.
- [x] Record release-candidate readiness as `not_approved`.

## Provider trust/output promotion prohibition checklist
- [x] Confirm no provider trust.
- [x] Confirm no provider output promotion.

## Replay-repair prohibition checklist
- [x] Confirm no replay repair.

## Recovery-promotion prohibition checklist
- [x] Confirm no recovery promotion.

## Action-execution prohibition checklist
- [x] Confirm no action execution.

## Readiness prohibition checklist
- [x] Confirm no readiness_approved state.
- [x] Confirm no readiness upgrade.

## Production Candidate status checklist
- [x] Production Candidate status: `not_approved`.

## Release-candidate/public-use status checklist
- [x] Release Candidate status: `not_approved`.
- [x] Release-candidate readiness: `not_approved`.
- [x] Public/general use: `not_approved`.

## Roadmap/changelog truth checklist
- [x] Roadmap remains planned truth.
- [x] CHANGELOG surfaces remain historical truth.

## Validation checklist
- [x] Run `CARGO_TARGET_DIR=/tmp/ajentic-phase-120-target ./scripts/check.sh`.
- [x] Run `git diff --check`.
- [x] Run `git status --short`.
- [x] Run Phase 120 gate scan.
- [x] Run ladder invariant scan.
- [x] Run phase relationship scan.
- [x] Run decision status scan.
- [x] Run evidence and constraint scan.
- [x] Run no-deployment/release authority scan.
- [x] Run no-authority scan.
- [x] Run readiness scan.
- [x] Run source guard.
- [x] Run roadmap guard.

## Findings table
| Finding | Status | Notes |
| --- | --- | --- |
| Phase 120 gate boundary | early_human_use_candidate_permitted | Constrained only. |
| Adjacent approvals | not_approved | Release, Production Candidate, public/general use, production readiness, production human use remain prohibited. |
| Phase 121 | proceed_to_phase_121_roadmap_expansion | Required after Phase 120. |

## Decision table
| Decision | Status |
| --- | --- |
| Early human-use candidate posture | early_human_use_candidate_permitted |
| Outcome class | early_human_use_candidate_permitted_with_constraints |
| Release Candidate status | not_approved |
| Production Candidate status | not_approved |
| Public/general use | not_approved |
| Production human use | not_approved |
| Phase 121 roadmap expansion | proceed_to_phase_121_roadmap_expansion |

## Evidence gaps table
| Gap | Status |
| --- | --- |
| Evidence gaps blocking Release Candidate status | deferred |
| Evidence gaps blocking Production Candidate status | deferred |
| Evidence gaps blocking public/general use | deferred |
| Evidence gaps blocking production human use | deferred |

## Residual risks table
| Risk | Disposition |
| --- | --- |
| Phase 115 security residual risks | Carry forward; Security reviewer escalation. |
| Phase 118 release-evidence non-approval | Carry forward; Release steward escalation. |
| Phase 119 Production Candidate non-approval | Carry forward; Phase 121 roadmap expansion. |

## Deferred items table
| Item | Status |
| --- | --- |
| Release Candidate status | not_approved; deferred |
| Production Candidate status | not_approved; deferred |
| Public/general use | not_approved; deferred |
| Production human use | not_approved; deferred |

## Validation log table
| Command | Result |
| --- | --- |
| `CARGO_TARGET_DIR=/tmp/ajentic-phase-120-target ./scripts/check.sh` | pass |
| `git diff --check` | pass |
| `git status --short` | pass with expected Phase 120 files before commit |
| required `rg` scans | pass with expected prohibition/evidence context |
| source guard | pass; no source/test/schema/script/workflow drift |
| roadmap guard | pass; no roadmap drift |

## Zero-drift checklist
- [x] No Rust source changes.
- [x] No TypeScript source changes.
- [x] No test changes.
- [x] No schema changes.
- [x] No deployment infrastructure changes.
- [x] No release infrastructure changes.
- [x] No package or lockfile changes.
- [x] No governance or architecture changes.
- [x] No README or AGENTS changes.
- [x] No archived changelog changes.
- [x] No roadmap changes.
