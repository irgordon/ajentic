---
truth_dimension: orientation
authority_level: advisory
mutation_path: readme_update
---

# Phase 127 - Installer and Update-Channel Threat Boundary
## Scope
Phase 127 is Installer and Update-Channel Threat Boundary only.
Phase 127 is threat-model and contract-only.
Phase 127 adds no runtime behavior.
Phase 127 adds no new runtime capability.
Phase 127 creates no packages.
Phase 127 creates no release artifacts.
Phase 127 generates no checksums.
Phase 127 creates no provenance attestations.
Phase 127 creates no installers.
Phase 127 creates no updater services.
Phase 127 creates no update channels.
Phase 127 creates no update-channel metadata.
Phase 127 creates no signing keys.
Phase 127 creates no key custody behavior.
Phase 127 creates no signatures.
Phase 127 creates no installer behavior.
Phase 127 creates no update-channel behavior.
Phase 127 adds no signing behavior.
Phase 127 adds no publishing behavior.
Phase 127 creates no GitHub releases.
Phase 127 creates no release tags.
Phase 127 creates no public downloads.
Phase 127 creates no public assets.
Phase 127 adds no public release behavior.
Phase 127 adds no production deployment behavior.
Phase 127 adds no deployment automation.
Phase 127 does not expand provider execution.
Phase 127 does not expand persistence authority.
Phase 127 does not add replay repair.
Phase 127 does not add recovery promotion.
Phase 127 does not add action execution.
Phase 127 does not add provider trust.
Phase 127 does not promote provider output.
Phase 127 does not approve Release Candidate status.
Phase 127 does not approve release-candidate readiness.
Phase 127 does not approve Production Candidate status.
Phase 127 does not approve production readiness.
Phase 127 does not approve public usability.
Phase 127 does not approve public/general use.
Phase 127 does not approve production human use.
Phase 128, if recommended, is Observability and Operational Evidence Boundary only.
Phase 129 remains Release Candidate Dry Run only.
Phase 130 remains Release Candidate Decision Gate only.
Phase 130 may still decide not ready.
Public/general use remains a later final rung.
Roadmap remains planned truth.
CHANGELOG surfaces remain historical truth.

## Evidence rule
Count only committed evidence: source files, tests, UI behavior tests, validation scripts, governance docs, architecture docs, roadmap docs, operations docs, changelog surfaces, checklists, schemas, and CI/workflow files. Do not count prompt intent, prior chat summaries, uncommitted work, speculative roadmap claims, future phase descriptions as implemented behavior, passing validation as release approval, passing validation as readiness approval, Phase 127 installer spec as installer creation, Phase 127 updater spec as updater creation, Phase 127 update-channel spec as active update channel, Phase 127 rollback playbook as automated rollback, Phase 127 signing requirement as signing, Phase 127 key custody requirement as key custody creation, Phase 127 publishing requirement as publishing, Phase 127 distribution boundary as distribution, Phase 127 contract completeness as Release Candidate readiness, Phase 127 threat model completeness as Production Candidate readiness, absence of blockers as approval, or roadmap preservation as implementation.

## Phase 127 threat-boundary scope
Phase 127 may define future installer, updater, update-channel, signing, key custody, publishing, rollback, and distribution threat requirements. Phase 127 may not create or activate those mechanisms. A threat table, boundary table, or specification row is not an implemented artifact and must not be treated as Release Candidate readiness.

## Required verbatim non-approval statement
Explicit non-approval statement: Phase 127 is threat-model and contract-only; it does not create installers, update channels, signing, publishing, or approve readiness.

## Required enforcement lines
- Feedback is evidence, not authority.
- Remediation is documentation clarity, not readiness.
- Contract/spec is specification only, not artifact creation.
- No installer/update-channel spec row may imply activation, signing, publishing, or release readiness.
- Missing Phase-128/129/130 dependencies must trigger remap_phase_126_130 or defer_release_candidate_hardening.

## Top drift vectors
- Spec-to-creation drift: contract, specification, or threat rows must not be interpreted as evidence that installers, updaters, release artifacts, signing, publishing, update channels, release tags, or public assets already exist.
- Signing/publishing shortcut: signing requirements must not justify key custody creation, signing actions, publishing actions, or release approval.
- Update channel treated as live: channel metadata, channel specifications, or threat rows must not be treated as active updater behavior.
- Rollback/auto-update misread as automation: rollback plans and update-channel threat models must not imply automated rollback, updater services, background daemons, or update execution.
- Cross-category inference: installer/update-channel evidence must not imply release readiness, deployment readiness, Production Candidate readiness, production readiness, or public/general-use readiness.
- No cross-category inference: each evidence category remains distinct.

## Mechanically checkable pre-handoff checks
- Every installer/update-channel/specification row includes spec_is_spec=true and either a committed evidence pointer or deferred_to_phase marker.
- Repository scan confirms no installer binaries, updater services, update-channel metadata, signing keys, signatures, release tags, public downloads, or public assets. Active artifact presence is a stop condition unless pre-existing historical/test/prohibition context is explicitly classified.
- Signing requirements include custody_plan_pointer or deferred_to_phase and verification_process_pointer or deferred_to_phase; missing evidence requires requires_release_steward_escalation.
- Each update-channel row includes channel_type, attacker_scenario, vector, impact, mitigation, evidence_pointer, and deferred_activation=true.
- Rollback entries are procedural only, include playbook_pointer or deferred_to_phase, and carry no_automation=true, no_background_service=true, and no_update_execution=true.
- Each distribution entry is marked non_public, deferred_to_phase_130_decision, or deferred_to_post_130_public_use_phase.
- Phase 128/129/130 dependencies are enumerated; missing dependencies produce remap_phase_126_130 or defer_release_candidate_hardening.

## Phase 126 relationship
Phase 126 is complete. Phase 126 is Release Packaging Contract only. Packaging contract is not package creation. Artifact contract is not artifact creation. Checksum contract is not checksum generation. Provenance contract is not provenance attestation. Distribution contract is not distribution. Signing contract is not signing. Publishing contract is not publishing. Release packaging contract is not Release Candidate readiness. Release packaging contract is not Production Candidate readiness. Release packaging contract is not public/general use. Phase 126 created no packages, release artifacts, checksums, provenance attestations, installers, update channels, signing or publishing behavior, GitHub releases, release tags, public downloads, public assets, public release behavior, production deployment behavior, deployment automation, runtime behavior, or new runtime capability. Phase 126 changed no Rust source, TypeScript source, tests, or schemas. Phase 126 approved no readiness status. Phase 126 did not implement Phase 127 or Phase 130.

## Phase 125 relationship
Phase 125 selected preserve_with_caveats as the primary outcome and expand_post_130_plan as the secondary outcome. AJENTIC remains at constrained early-human-use candidate / usability-remediation stage. AJENTIC is not Release Candidate ready, not Production Candidate ready, and not public/general-use ready. Phase 126-130 remains valid only as caveated planned truth. Phase 130 may still decide not ready. Post-130 planning remains required for Production Candidate reassessment, production deployment contract, production-readiness evidence, public/general-use readiness audit, public/general-use decision gate, support, incident response, rollback, distribution governance, and final public/general-use gate.

## Phase 126-130 caveated plan relationship
Phase 127 remains Installer and Update-Channel Threat Boundary only. Phase 128 remains Observability and Operational Evidence Boundary only. Phase 129 remains Release Candidate Dry Run only. Phase 130 remains Release Candidate Decision Gate only. Phase 130 may still decide not ready. Public/general use remains a later final rung.

## Threat-boundary status model
threat_boundary_defined, threat_boundary_defined_with_findings, threat_boundary_partial, threat_boundary_deferred, threat_boundary_blocked, requires_release_steward_escalation, requires_security_reviewer_escalation, remap_phase_126_130, defer_release_candidate_hardening, requires_phase_128_evidence, requires_phase_129_dry_run, requires_phase_130_decision, requires_post_130_public_use_phase, not_applicable

## Required table flags
Every installer/update-channel/specification row must include spec_is_spec=true, evidence_pointer or deferred_to_phase, activation_enabled=false, signing_enabled=false, publishing_enabled=false, and release_readiness_claim=false. Every update-channel row must include channel_type, auth_model, distribution_boundary, attacker_scenario, vector, impact, mitigation, evidence_pointer or deferred_to_phase, and deferred_activation=true. Every rollback row must include playbook_pointer or deferred_to_phase, no_automation=true, no_background_service=true, and no_update_execution=true. Every signing/key custody row must include signer_role, custody_plan_pointer or deferred_to_phase, verification_process_pointer or deferred_to_phase, signing_enabled=false, publishing_enabled=false, and requires_release_steward_escalation if custody or verification evidence is missing. Every distribution row must include non_public, deferred_to_phase_130_decision, or deferred_to_post_130_public_use_phase.

## Threat/contract category model
Categories: installer_specification, updater_specification, update_channel_metadata, update_channel_auth_model, update_channel_distribution_boundary, signing_requirement, key_custody_requirement, verification_process_requirement, publishing_requirement, rollback_procedure, rollback_automation_prohibition, background_service_prohibition, daemon_prohibition, public_distribution_deferral, non_public_distribution_boundary, GitHub_release_deferral, release_tag_deferral, public_download_deferral, public_asset_deferral, Phase_128_observability_dependency, Phase_129_dry_run_dependency, Phase_130_decision_dependency, post_130_public_use_dependency, readiness_non_approval_statement.

## Production-human-use ladder
Local operator testing → Controlled human trial → Early human-use candidate → Release candidate → Production candidate → Public/general use

## Ladder-Preservation Invariant Set
1. Ladder steps are not interchangeable; each rung is a distinct authority boundary.
2. No implicit promotion: installer specifications, updater specifications, update-channel specifications, signing requirements, key custody references, rollback playbooks, distribution deferrals, validation success, operator notes, participant feedback, provider output, absence of blockers, roadmap expansion, or changelog alignment cannot promote Release Candidate status, Production Candidate status, public/general use, production readiness, or production human use.
3. Absence of blockers is not approval.
4. Evidence assembly is not readiness.
5. Dry runs are not release.
6. Phase 127 is not a decision gate for later rungs.
7. Phase 127 cannot retroactively rewrite Phase 120, Phase 121, Phase 122, Phase 123, Phase 124, Phase 125, or Phase 126 as release, Production Candidate, or public-use approval.
8. Human use is not binary.
9. Deployment is not release.
10. Phase 127 is not the end of the roadmap.
11. Public/general use is always the final rung.
12. No trust inference: provider output, operator notes, participant notes, or feedback do not imply trust, readiness, safety, or authority.
13. No cross-category inference; installer evidence, update-channel evidence, signing evidence, publishing evidence, rollback evidence, distribution evidence, packaging evidence, artifact evidence, checksum evidence, provenance evidence, usability evidence, trial evidence, operator workflow evidence, observability evidence, security evidence, release evidence, governance evidence, roadmap evidence, changelog evidence, validation evidence, provider evidence, persistence evidence, recovery evidence, action evidence, deployment evidence, and public-use evidence remain separate.
14. No authority activates without explicit roadmap permission.
15. Every rung requires its own evidence, not inherited evidence.
16. Roadmap continuation remains required.

## Installer specification threat boundary
Category: installer_specification. Status: threat_boundary_defined. Threats include trojanized installer concepts, implicit platform package assumptions, and mistaken claims that a future installer exists. spec_is_spec=true; evidence_pointer=docs/operations/release-packaging-contract-phase-126.md; activation_enabled=false; signing_enabled=false; publishing_enabled=false; release_readiness_claim=false.

## Updater specification threat boundary
Category: updater_specification. Status: threat_boundary_defined. Threats include updater service ambiguity, background execution drift, and automatic update execution claims. spec_is_spec=true; evidence_pointer=docs/operations/release-packaging-contract-phase-126.md; activation_enabled=false; signing_enabled=false; publishing_enabled=false; release_readiness_claim=false.

## Update-channel metadata threat boundary
Category: update_channel_metadata. Status: threat_boundary_defined. Threats include treating channel metadata specifications as live appcast, repository, or feed metadata. spec_is_spec=true; evidence_pointer=docs/operations/release-packaging-contract-phase-126.md; activation_enabled=false; signing_enabled=false; publishing_enabled=false; release_readiness_claim=false.

## Update-channel auth model boundary
Category: update_channel_auth_model. Status: threat_boundary_defined. Threats include unauthenticated metadata, stale metadata replay, and confused signer/verifier roles. spec_is_spec=true; evidence_pointer=docs/operations/release-packaging-contract-phase-126.md; activation_enabled=false; signing_enabled=false; publishing_enabled=false; release_readiness_claim=false.

## Update-channel distribution boundary
Category: update_channel_distribution_boundary. Status: threat_boundary_defined. Threats include public distribution inference, mirror compromise assumptions, and live channel treatment. spec_is_spec=true; evidence_pointer=docs/operations/release-packaging-contract-phase-126.md; activation_enabled=false; signing_enabled=false; publishing_enabled=false; release_readiness_claim=false.

## Signing requirement boundary
Category: signing_requirement. Status: threat_boundary_defined. Threats include turning signing requirements into signing actions or signature creation. spec_is_spec=true; evidence_pointer=docs/operations/release-packaging-contract-phase-126.md; activation_enabled=false; signing_enabled=false; publishing_enabled=false; release_readiness_claim=false.

## Key custody requirement boundary
Category: key_custody_requirement. Status: threat_boundary_defined. Threats include key generation, custody-role creation, or undocumented custody by convenience. spec_is_spec=true; evidence_pointer=docs/operations/release-packaging-contract-phase-126.md; activation_enabled=false; signing_enabled=false; publishing_enabled=false; release_readiness_claim=false.

## Verification process requirement boundary
Category: verification_process_requirement. Status: threat_boundary_defined. Threats include assuming verification exists before committed evidence. spec_is_spec=true; evidence_pointer=docs/operations/release-packaging-contract-phase-126.md; activation_enabled=false; signing_enabled=false; publishing_enabled=false; release_readiness_claim=false.

## Publishing requirement boundary
Category: publishing_requirement. Status: threat_boundary_defined. Threats include treating publishing requirements as publishing authority. spec_is_spec=true; evidence_pointer=docs/operations/release-packaging-contract-phase-126.md; activation_enabled=false; signing_enabled=false; publishing_enabled=false; release_readiness_claim=false.

## Rollback procedure boundary
Category: rollback_procedure. Status: threat_boundary_defined. Threats include interpreting rollback playbooks as automated rollback or update execution. spec_is_spec=true; evidence_pointer=docs/operations/release-packaging-contract-phase-126.md; activation_enabled=false; signing_enabled=false; publishing_enabled=false; release_readiness_claim=false.

## Rollback automation prohibition
Category: rollback_automation_prohibition. Status: threat_boundary_defined. Rollback automation is prohibited; rollback content is procedural only. spec_is_spec=true; evidence_pointer=docs/operations/release-packaging-contract-phase-126.md; activation_enabled=false; signing_enabled=false; publishing_enabled=false; release_readiness_claim=false.

## Background service prohibition
Category: background_service_prohibition. Status: threat_boundary_defined. Background services are prohibited; no service installation, launchd, systemd, Windows service, or daemon behavior is created. spec_is_spec=true; evidence_pointer=docs/operations/release-packaging-contract-phase-126.md; activation_enabled=false; signing_enabled=false; publishing_enabled=false; release_readiness_claim=false.

## Daemon prohibition
Category: daemon_prohibition. Status: threat_boundary_defined. Daemon behavior is prohibited; no long-running update process is created. spec_is_spec=true; evidence_pointer=docs/operations/release-packaging-contract-phase-126.md; activation_enabled=false; signing_enabled=false; publishing_enabled=false; release_readiness_claim=false.

## Public distribution deferral
Category: public_distribution_deferral. Status: threat_boundary_defined. Public distribution is deferred to Phase 130 decision or post-130 public-use phases and is not approved here. spec_is_spec=true; evidence_pointer=docs/operations/release-packaging-contract-phase-126.md; activation_enabled=false; signing_enabled=false; publishing_enabled=false; release_readiness_claim=false.

## Non-public distribution boundary
Category: non_public_distribution_boundary. Status: threat_boundary_defined. Distribution remains non_public unless a future explicit decision changes it. spec_is_spec=true; evidence_pointer=docs/operations/release-packaging-contract-phase-126.md; activation_enabled=false; signing_enabled=false; publishing_enabled=false; release_readiness_claim=false.

## GitHub release deferral
Category: GitHub_release_deferral. Status: threat_boundary_defined. GitHub releases are deferred and not created. spec_is_spec=true; evidence_pointer=docs/operations/release-packaging-contract-phase-126.md; activation_enabled=false; signing_enabled=false; publishing_enabled=false; release_readiness_claim=false.

## Release tag deferral
Category: release_tag_deferral. Status: threat_boundary_defined. Release tags are deferred and not created. spec_is_spec=true; evidence_pointer=docs/operations/release-packaging-contract-phase-126.md; activation_enabled=false; signing_enabled=false; publishing_enabled=false; release_readiness_claim=false.

## Public download deferral
Category: public_download_deferral. Status: threat_boundary_defined. Public downloads are deferred and not created. spec_is_spec=true; evidence_pointer=docs/operations/release-packaging-contract-phase-126.md; activation_enabled=false; signing_enabled=false; publishing_enabled=false; release_readiness_claim=false.

## Public asset deferral
Category: public_asset_deferral. Status: threat_boundary_defined. Public assets are deferred and not created. spec_is_spec=true; evidence_pointer=docs/operations/release-packaging-contract-phase-126.md; activation_enabled=false; signing_enabled=false; publishing_enabled=false; release_readiness_claim=false.

## Installer/Updater Threat Table
| category | attacker | vector | impact | mitigation | evidence_pointer | spec_is_spec | activation_enabled | signing_enabled | publishing_enabled | release_readiness_claim | status |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| installer_specification | malicious distributor | forged installer claim | operator may trust non-existent package | require Phase 130 or later explicit artifact evidence and repository artifact-presence check | docs/operations/release-packaging-contract-phase-126.md | spec_is_spec=true | activation_enabled=false | signing_enabled=false | publishing_enabled=false | release_readiness_claim=false | threat_boundary_defined |
| updater_specification | attacker posing as updater | auto-update command or service claim | background execution drift | prohibit updater service, background daemon, and update execution | docs/roadmap/phases.md | spec_is_spec=true | activation_enabled=false | signing_enabled=false | publishing_enabled=false | release_readiness_claim=false | threat_boundary_defined |
| update_channel_metadata | mirror or feed attacker | fake channel metadata/appcast | update-channel treated as live | defer metadata creation and require future verification evidence | deferred_to_phase_128 | spec_is_spec=true | activation_enabled=false | signing_enabled=false | publishing_enabled=false | release_readiness_claim=false | threat_boundary_defined_with_findings |
| signing_requirement | compromised release path | shortcut to signing without custody | false trust in artifacts | require signer role separation and release steward escalation | deferred_to_phase_130 | spec_is_spec=true | activation_enabled=false | signing_enabled=false | publishing_enabled=false | release_readiness_claim=false | requires_release_steward_escalation |
| publishing_requirement | unauthorized publisher | public asset upload claim | public release without approval | require Phase 130 decision and post-130 public-use gate | deferred_to_phase_130 | spec_is_spec=true | activation_enabled=false | signing_enabled=false | publishing_enabled=false | release_readiness_claim=false | requires_phase_130_decision |
| rollback_procedure | rollback abuser | downgrade or replay old package claim | unsafe recovery path | procedural playbook only; no update execution | deferred_to_phase_128 | spec_is_spec=true | activation_enabled=false | signing_enabled=false | publishing_enabled=false | release_readiness_claim=false | requires_phase_128_evidence |

## Update-Channel Boundary Table
| channel_type | auth_model | distribution_boundary | attacker_scenario | vector | impact | mitigation | evidence_pointer | deferred_activation | spec_is_spec | activation_enabled | signing_enabled | publishing_enabled | release_readiness_claim | status |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| appcast_specification | future signed metadata required | non_public | forged appcast feed | appcast URL claim | live update confusion | no channel metadata creation; require future verifier evidence | deferred_to_phase_128 | deferred_activation=true | spec_is_spec=true | activation_enabled=false | signing_enabled=false | publishing_enabled=false | release_readiness_claim=false | threat_boundary_deferred |
| package_repository_specification | future repository auth required | deferred_to_phase_130_decision | repository mirror compromise | apt repository/yum repository/brew tap claim | public distribution confusion | defer repository creation and distribution decision | deferred_to_phase_130 | deferred_activation=true | spec_is_spec=true | activation_enabled=false | signing_enabled=false | publishing_enabled=false | release_readiness_claim=false | requires_phase_130_decision |
| manual_download_specification | future checksum/signature verification required | deferred_to_post_130_public_use_phase | public asset substitution | public_download URL claim | public/general-use confusion | defer public asset governance to post-130 public-use phase | deferred_to_post_130_public_use_phase | deferred_activation=true | spec_is_spec=true | activation_enabled=false | signing_enabled=false | publishing_enabled=false | release_readiness_claim=false | requires_post_130_public_use_phase |

## Signing/Key Custody Table
| category | signer_role | custody_plan_pointer | verification_process_pointer | signing_enabled | publishing_enabled | escalation status | spec_is_spec | activation_enabled | release_readiness_claim |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| signing_requirement | release steward and security reviewer separation required | deferred_to_phase_130 | deferred_to_phase_129 | signing_enabled=false | publishing_enabled=false | requires_release_steward_escalation | spec_is_spec=true | activation_enabled=false | release_readiness_claim=false |
| key_custody_requirement | release steward custodian not yet assigned | deferred_to_phase_130 | deferred_to_phase_129 | signing_enabled=false | publishing_enabled=false | requires_release_steward_escalation | spec_is_spec=true | activation_enabled=false | release_readiness_claim=false |
| verification_process_requirement | security reviewer verifies future detached evidence | deferred_to_phase_130 | deferred_to_phase_128 | signing_enabled=false | publishing_enabled=false | requires_security_reviewer_escalation | spec_is_spec=true | activation_enabled=false | release_readiness_claim=false |

## Rollback Procedure Template
| rollback item | playbook_pointer | trigger condition | operator action | no_automation | no_background_service | no_update_execution | status |
| --- | --- | --- | --- | --- | --- | --- | --- |
| rollback_procedure | deferred_to_phase_128 | future release artifact failure evidence | document manual stop/defer path only | no_automation=true | no_background_service=true | no_update_execution=true | requires_phase_128_evidence |
| rollback_automation_prohibition | docs/governance/GOVERNANCE.md | any auto-update or auto-rollback proposal | reject automation and remap | no_automation=true | no_background_service=true | no_update_execution=true | threat_boundary_defined |
| downgrade_abuse_case | deferred_to_phase_129 | future dry-run stale artifact scenario | record as dry-run finding only | no_automation=true | no_background_service=true | no_update_execution=true | requires_phase_129_dry_run |

## Distribution Deferral Table
| artifact or channel | distribution marker | evidence_pointer | spec_is_spec | activation_enabled | signing_enabled | publishing_enabled | release_readiness_claim | status |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| installer artifact | non_public | docs/operations/release-packaging-contract-phase-126.md | spec_is_spec=true | activation_enabled=false | signing_enabled=false | publishing_enabled=false | release_readiness_claim=false | threat_boundary_defined |
| update channel metadata | non_public | deferred_to_phase_128 | spec_is_spec=true | activation_enabled=false | signing_enabled=false | publishing_enabled=false | release_readiness_claim=false | requires_phase_128_evidence |
| GitHub release | deferred_to_phase_130_decision | deferred_to_phase_130 | spec_is_spec=true | activation_enabled=false | signing_enabled=false | publishing_enabled=false | release_readiness_claim=false | requires_phase_130_decision |
| release tag | deferred_to_phase_130_decision | deferred_to_phase_130 | spec_is_spec=true | activation_enabled=false | signing_enabled=false | publishing_enabled=false | release_readiness_claim=false | requires_phase_130_decision |
| public download | deferred_to_post_130_public_use_phase | deferred_to_post_130_public_use_phase | spec_is_spec=true | activation_enabled=false | signing_enabled=false | publishing_enabled=false | release_readiness_claim=false | requires_post_130_public_use_phase |
| public asset | deferred_to_post_130_public_use_phase | deferred_to_post_130_public_use_phase | spec_is_spec=true | activation_enabled=false | signing_enabled=false | publishing_enabled=false | release_readiness_claim=false | requires_post_130_public_use_phase |

## Phase 128/129/130 Dependency Table
| phase | required evidence | current evidence status | missing dependency | decision outcome candidate |
| --- | --- | --- | --- | --- |
| Phase 128 | observability evidence for installer/update-channel threat findings, artifact-presence scans, verification process evidence, rollback evidence | requires_phase_128_evidence | update-channel metadata verification and rollback evidence are not implemented | remap_phase_126_130 |
| Phase 129 | Release Candidate dry run evidence that no packages, signatures, public assets, update channels, or rollback automation are produced | requires_phase_129_dry_run | dry run not performed in Phase 127 | defer_release_candidate_hardening |
| Phase 130 | Release Candidate Decision Gate evidence and release steward decision | requires_phase_130_decision | Phase 128/129 evidence not yet assembled | remap_phase_126_130 |

## Missing-dependency decision candidate table
| missing dependency | affected future phase | decision outcome candidate | escalation owner |
| --- | --- | --- | --- |
| custody plan evidence | Phase 130 | remap_phase_126_130 | release steward |
| verification process evidence | Phase 128 | remap_phase_126_130 | security reviewer |
| Release Candidate dry-run evidence | Phase 129 | defer_release_candidate_hardening | release steward |
| observability evidence for artifact-presence scan | Phase 128 | remap_phase_126_130 | security reviewer |
| public distribution governance | post-130 public/general use phase | defer_release_candidate_hardening | release steward |

## Repository artifact-presence check
Phase 127 defines the repository artifact-presence check as an evidence requirement and ran it as a documentation-scope guard. Expected result: no signing keys, certificates, signatures, installer binaries/packages, appcast/update-channel metadata, GitHub releases, release tags, public downloads, or public release assets introduced by Phase 127. Existing mentions are classified as docs, tests, planned-truth, historical-truth, threat requirements, or explicit prohibitions unless active artifacts are discovered.

## Phase 128 observability evidence expectation
Phase 128 must remain Observability and Operational Evidence Boundary only and provide evidence requirements for artifact-presence scans, verification observations, rollback observations, incident evidence, and operator evidence capture without production monitoring or update-channel activation.

## Phase 129 Release Candidate dry-run expectation
Phase 129 remains Release Candidate Dry Run only. It must rehearse release-candidate assembly without publishing, approving release-candidate readiness, creating installers, activating update channels, signing artifacts, or creating public downloads.

## Phase 130 Release Candidate decision-gate expectation
Phase 130 remains Release Candidate Decision Gate only. Phase 130 may still decide not ready and cannot approve Production Candidate status, production readiness, public usability, public/general use, or production human use.

## Post-130 production/public-use deferrals
Production Candidate reassessment, production deployment contract, production-readiness evidence, public/general-use readiness audit, public/general-use decision gate, support, incident response, rollback, distribution governance, and final public/general-use gate remain post-130 deferrals unless explicitly mapped later.

## Installer creation prohibition
Phase 127 creates no installers and adds no installer behavior.

## Updater service prohibition
Phase 127 creates no updater services and adds no updater behavior.

## Update-channel activation prohibition
Phase 127 creates no update channels and activation_enabled=false for every specification row.

## Update-channel metadata creation prohibition
Phase 127 creates no update-channel metadata.

## Signing key creation prohibition
Phase 127 creates no signing keys.

## Key custody behavior prohibition
Phase 127 creates no key custody behavior.

## Signature creation prohibition
Phase 127 creates no signatures and signing_enabled=false.

## Publishing prohibition
Phase 127 adds no publishing behavior and publishing_enabled=false.

## Release artifact prohibition
Phase 127 creates no release artifacts.

## Package creation prohibition
Phase 127 creates no packages.

## Checksum generation prohibition
Phase 127 generates no checksums.

## Provenance attestation prohibition
Phase 127 creates no provenance attestations.

## Distribution prohibition
Phase 127 performs no distribution; distribution entries remain non_public, deferred_to_phase_130_decision, or deferred_to_post_130_public_use_phase.

## Deployment automation prohibition
Phase 127 adds no deployment automation.

## Rollback automation prohibition
Phase 127 adds no automated rollback; rollback entries carry no_automation=true.

## Background service/daemon prohibition
Phase 127 adds no background service and no daemon behavior; rollback entries carry no_background_service=true and no_update_execution=true.

## GitHub release/tag/public asset prohibition
Phase 127 creates no GitHub releases, release tags, public downloads, or public assets.

## Public-release prohibition
Phase 127 adds no public release behavior.

## Production-deployment prohibition
Phase 127 adds no production deployment behavior.

## Public/general-use approval prohibition
Phase 127 does not approve public usability or public/general use.

## Production-human-use approval prohibition
Phase 127 does not approve production human use.

## Production Candidate approval prohibition
Phase 127 does not approve Production Candidate status.

## Release-candidate approval prohibition
Phase 127 does not approve Release Candidate status or release-candidate readiness.

## Provider trust/output promotion prohibition
Phase 127 does not add provider trust and does not promote provider output.

## Replay-repair prohibition
Phase 127 does not add replay repair.

## Recovery-promotion prohibition
Phase 127 does not add recovery promotion.

## Action-execution prohibition
Phase 127 does not add action execution.

## Readiness approval prohibition
Phase 127 does not approve readiness; absence of blockers is not approval.

## Required future implementation evidence
Future implementation requires committed phase-scoped evidence before any installer, updater, update-channel metadata, signing, key custody, verification, publishing, rollback, distribution, release artifact, package, checksum, provenance attestation, deployment automation, public release, or production deployment behavior can be considered. Phase 127 evidence is descriptive and cannot be inherited as approval.

## Phase 128 gate decision
Decision candidate: requires_phase_128_evidence. Missing evidence must trigger remap_phase_126_130 or defer_release_candidate_hardening; Phase 128 is not implemented by Phase 127.

## Phase 130-or-later deferrals
Phase 130 or later must decide release-candidate status and later production/public-use rungs separately. Phase 127 does not implement Phase 128, Phase 129, Phase 130, or any post-130 phase.

## Production Candidate status
Production Candidate status remains not approved.

## Release-candidate readiness status
Release-candidate readiness remains not approved.

## Public/general use status
Public/general use remains not approved and remains a later final rung.

## Roadmap/changelog truth posture
Roadmap remains planned truth. CHANGELOG surfaces remain historical truth. This operations report is advisory orientation evidence only.

## Required follow-ups
Required follow-ups are Phase 128 observability evidence, Phase 129 dry-run evidence, Phase 130 decision-gate evidence, and post-130 public/general-use evidence if later authorized.

## Deferred items
Deferred items include installers, updater services, update channels, update-channel metadata, signing keys, key custody behavior, signatures, signing, publishing, GitHub releases, release tags, public downloads, public assets, release artifacts, packages, checksums, provenance attestations, distribution, deployment automation, rollback automation, background services, daemon behavior, public release, production deployment, provider trust, provider output promotion, replay repair, recovery promotion, action execution, readiness approval, Phase 128 implementation, Phase 129 implementation, Phase 130 implementation, and post-130 public/general-use approval.

## Confirmed vs suspected
Confirmed: Phase 127 adds only documentation-scope installer and update-channel threat-boundary evidence. Suspected claims are not counted without committed source, test, doc, checklist, changelog, schema, script, or workflow proof.

## Non-readiness statement
Phase 127 is threat-model and contract-only; it does not create installers, update channels, signing, publishing, or approve readiness. Release Candidate status, Production Candidate status, production readiness, public usability, public/general use, and production human use remain not approved.
