---
truth_dimension: procedural
authority_level: authoritative
mutation_path: checklist_revision
---

# Current Phase Checklist - Phase 127 Installer and Update-Channel Threat Boundary
## Phase name
- [x] Phase 127 - Installer and Update-Channel Threat Boundary.

## Phase goal
- [x] Define installer/update-channel risks, constraints, prohibited behaviors, trust boundaries, rollback/update abuse cases, signing/publishing dependencies, distribution deferrals, and future evidence requirements without creating installers, update channels, signing, publishing, release artifacts, or readiness approval.

## Working-tree hygiene gate
- [x] Run git status --short before edits.
- [x] Classify uncommitted files as empty before edits.
- [x] Remove generated artifact drift before edits if present.
- [x] Record cleanup: no generated artifact drift was present before edits.

## Allowed surfaces
- [x] docs/operations/installer-update-channel-threat-boundary-phase-127.md
- [x] checklists/current-phase.md
- [x] CHANGELOG.md

## Boundary rules
- [x] Phase 127 is Installer and Update-Channel Threat Boundary only.
- [x] Phase 127 is threat-model and contract-only.
- [x] Phase 127 adds no runtime behavior and no new runtime capability.
- [x] Phase 127 creates no packages, release artifacts, checksums, provenance attestations, installers, updater services, update channels, update-channel metadata, signing keys, key custody behavior, signatures, GitHub releases, release tags, public downloads, or public assets.
- [x] Phase 127 adds no installer behavior, update-channel behavior, signing behavior, publishing behavior, public release behavior, production deployment behavior, deployment automation, rollback automation, background services, daemon behavior, provider execution expansion, persistence authority expansion, replay repair, recovery promotion, action execution, provider trust, or provider output promotion.
- [x] Phase 127 approves no readiness, Release Candidate status, release-candidate readiness, Production Candidate status, production readiness, public usability, public/general use, or production human use.
- [x] Phase 127 does not implement Phase 128, Phase 129, Phase 130, or any post-130 phase.

## Evidence-only checklist
- [x] Count only committed evidence.
- [x] Do not count prompt intent, prior chat summaries, uncommitted work, speculative roadmap claims, future phase descriptions as implemented behavior, or validation success as approval.
- [x] Do not count Phase 127 installer/update-channel/signing/publishing/rollback/distribution specifications as created artifacts or active behavior.

## Required verbatim non-approval statement checklist
- [x] Explicit non-approval statement is present.
- [x] Phase 127 is threat-model and contract-only; it does not create installers, update channels, signing, publishing, or approve readiness.

## Required enforcement lines checklist
- [x] - Feedback is evidence, not authority.
- [x] - Remediation is documentation clarity, not readiness.
- [x] - Contract/spec is specification only, not artifact creation.
- [x] - No installer/update-channel spec row may imply activation, signing, publishing, or release readiness.
- [x] - Missing Phase-128/129/130 dependencies must trigger remap_phase_126_130 or defer_release_candidate_hardening.

## Top drift vectors checklist
- [x] Spec-to-creation drift documented.
- [x] Signing/publishing shortcut documented.
- [x] Update channel treated as live documented.
- [x] Rollback/auto-update misread as automation documented.
- [x] Cross-category inference documented.

## Mechanically checkable pre-handoff checklist
- [x] Every installer/update-channel/specification row includes spec_is_spec=true and either evidence_pointer or deferred_to_phase.
- [x] Repository artifact-presence check is defined.
- [x] Signing requirements include custody_plan_pointer or deferred_to_phase and verification_process_pointer or deferred_to_phase.
- [x] Missing custody or verification evidence requires requires_release_steward_escalation.
- [x] Each update-channel row includes channel_type, attacker_scenario, vector, impact, mitigation, evidence_pointer, and deferred_activation=true.
- [x] Rollback entries include playbook_pointer or deferred_to_phase and no_automation=true, no_background_service=true, no_update_execution=true.
- [x] Each distribution entry is non_public, deferred_to_phase_130_decision, or deferred_to_post_130_public_use_phase.
- [x] Missing Phase 128/129/130 dependencies produce remap_phase_126_130 or defer_release_candidate_hardening.

## Phase 126 relationship checklist
- [x] Phase 126 is complete.
- [x] Phase 126 is Release Packaging Contract only.
- [x] Packaging contract is not package creation; artifact contract is not artifact creation; checksum contract is not checksum generation; provenance contract is not provenance attestation; distribution contract is not distribution; signing contract is not signing; publishing contract is not publishing.
- [x] Phase 126 created no packages, release artifacts, checksums, provenance attestations, installers, update channels, signing/publishing behavior, GitHub releases, release tags, public downloads, public assets, public release behavior, production deployment behavior, deployment automation, runtime behavior, or new runtime capability.
- [x] Phase 126 changed no Rust source, TypeScript source, tests, or schemas.
- [x] Phase 126 approved no readiness status and did not implement Phase 127 or Phase 130.

## Phase 125 relationship checklist
- [x] Phase 125 selected preserve_with_caveats as the primary outcome.
- [x] Phase 125 selected expand_post_130_plan as the secondary outcome.
- [x] AJENTIC remains at constrained early-human-use candidate / usability-remediation stage.
- [x] AJENTIC is not Release Candidate ready, not Production Candidate ready, and not public/general-use ready.
- [x] Phase 126-130 remains valid only as caveated planned truth.
- [x] Phase 130 may still decide not ready.
- [x] Post-130 planning remains required for later production and public/general-use gates.

## Phase 126-130 caveated plan checklist
- [x] Phase 127 remains Installer and Update-Channel Threat Boundary only.
- [x] Phase 128 remains Observability and Operational Evidence Boundary only.
- [x] Phase 129 remains Release Candidate Dry Run only.
- [x] Phase 130 remains Release Candidate Decision Gate only.
- [x] Phase 130 may still decide not ready.
- [x] Public/general use remains a later final rung.

## Threat-boundary status model checklist
- [x] threat_boundary_defined
- [x] threat_boundary_defined_with_findings
- [x] threat_boundary_partial
- [x] threat_boundary_deferred
- [x] threat_boundary_blocked
- [x] requires_release_steward_escalation
- [x] requires_security_reviewer_escalation
- [x] remap_phase_126_130
- [x] defer_release_candidate_hardening
- [x] requires_phase_128_evidence
- [x] requires_phase_129_dry_run
- [x] requires_phase_130_decision
- [x] requires_post_130_public_use_phase
- [x] not_applicable

## Required table flags checklist
- [x] spec_is_spec=true
- [x] evidence_pointer or deferred_to_phase marker
- [x] activation_enabled=false
- [x] signing_enabled=false
- [x] publishing_enabled=false
- [x] release_readiness_claim=false
- [x] deferred_activation=true for update-channel rows
- [x] no_automation=true for rollback rows
- [x] no_background_service=true for rollback rows
- [x] no_update_execution=true for rollback rows
- [x] custody_plan_pointer or deferred_to_phase for signing/key custody rows
- [x] verification_process_pointer or deferred_to_phase for signing/key custody rows
- [x] requires_release_steward_escalation when custody or verification evidence is missing
- [x] non_public, deferred_to_phase_130_decision, or deferred_to_post_130_public_use_phase for distribution rows

## Threat/contract category model checklist
- [x] installer_specification
- [x] updater_specification
- [x] update_channel_metadata
- [x] update_channel_auth_model
- [x] update_channel_distribution_boundary
- [x] signing_requirement
- [x] key_custody_requirement
- [x] verification_process_requirement
- [x] publishing_requirement
- [x] rollback_procedure
- [x] rollback_automation_prohibition
- [x] background_service_prohibition
- [x] daemon_prohibition
- [x] public_distribution_deferral
- [x] non_public_distribution_boundary
- [x] GitHub_release_deferral
- [x] release_tag_deferral
- [x] public_download_deferral
- [x] public_asset_deferral
- [x] Phase_128_observability_dependency
- [x] Phase_129_dry_run_dependency
- [x] Phase_130_decision_dependency
- [x] post_130_public_use_dependency
- [x] readiness_non_approval_statement

## Production-human-use ladder checklist
- [x] Local operator testing
- [x] Controlled human trial
- [x] Early human-use candidate
- [x] Release candidate
- [x] Production candidate
- [x] Public/general use

## Ladder-Preservation Invariant checklist
- [x] Ladder-Preservation invariants applied.
- [x] No implicit promotion.
- [x] Absence of blockers is not approval.
- [x] Evidence assembly is not readiness.
- [x] Dry runs are not release.
- [x] Deployment is not release.
- [x] Public/general use is always the final rung.
- [x] No trust inference.
- [x] No cross-category inference.
- [x] Every rung requires its own evidence.

## Installer specification threat boundary checklist
- [x] installer specification threat boundary defined as threat_boundary_defined or deferred with spec_is_spec=true; evidence_pointer=docs/operations/release-packaging-contract-phase-126.md; activation_enabled=false; signing_enabled=false; publishing_enabled=false; release_readiness_claim=false.

## Updater specification threat boundary checklist
- [x] updater specification threat boundary defined as threat_boundary_defined or deferred with spec_is_spec=true; evidence_pointer=docs/operations/release-packaging-contract-phase-126.md; activation_enabled=false; signing_enabled=false; publishing_enabled=false; release_readiness_claim=false.

## Update-channel metadata threat boundary checklist
- [x] update-channel metadata threat boundary defined as threat_boundary_defined or deferred with spec_is_spec=true; evidence_pointer=docs/operations/release-packaging-contract-phase-126.md; activation_enabled=false; signing_enabled=false; publishing_enabled=false; release_readiness_claim=false.

## Update-channel auth model boundary checklist
- [x] update-channel auth model boundary defined as threat_boundary_defined or deferred with spec_is_spec=true; evidence_pointer=docs/operations/release-packaging-contract-phase-126.md; activation_enabled=false; signing_enabled=false; publishing_enabled=false; release_readiness_claim=false.

## Update-channel distribution boundary checklist
- [x] update-channel distribution boundary defined as threat_boundary_defined or deferred with spec_is_spec=true; evidence_pointer=docs/operations/release-packaging-contract-phase-126.md; activation_enabled=false; signing_enabled=false; publishing_enabled=false; release_readiness_claim=false.

## Signing requirement boundary checklist
- [x] signing requirement boundary defined as threat_boundary_defined or deferred with spec_is_spec=true; evidence_pointer=docs/operations/release-packaging-contract-phase-126.md; activation_enabled=false; signing_enabled=false; publishing_enabled=false; release_readiness_claim=false.

## Key custody requirement boundary checklist
- [x] key custody requirement boundary defined as threat_boundary_defined or deferred with spec_is_spec=true; evidence_pointer=docs/operations/release-packaging-contract-phase-126.md; activation_enabled=false; signing_enabled=false; publishing_enabled=false; release_readiness_claim=false.

## Verification process requirement boundary checklist
- [x] verification process requirement boundary defined as threat_boundary_defined or deferred with spec_is_spec=true; evidence_pointer=docs/operations/release-packaging-contract-phase-126.md; activation_enabled=false; signing_enabled=false; publishing_enabled=false; release_readiness_claim=false.

## Publishing requirement boundary checklist
- [x] publishing requirement boundary defined as threat_boundary_defined or deferred with spec_is_spec=true; evidence_pointer=docs/operations/release-packaging-contract-phase-126.md; activation_enabled=false; signing_enabled=false; publishing_enabled=false; release_readiness_claim=false.

## Rollback procedure boundary checklist
- [x] rollback procedure boundary defined as threat_boundary_defined or deferred with spec_is_spec=true; evidence_pointer=docs/operations/release-packaging-contract-phase-126.md; activation_enabled=false; signing_enabled=false; publishing_enabled=false; release_readiness_claim=false.

## Rollback automation prohibition checklist
- [x] rollback automation prohibition defined as threat_boundary_defined or deferred with spec_is_spec=true; evidence_pointer=docs/operations/release-packaging-contract-phase-126.md; activation_enabled=false; signing_enabled=false; publishing_enabled=false; release_readiness_claim=false.

## Background service prohibition checklist
- [x] background service prohibition defined as threat_boundary_defined or deferred with spec_is_spec=true; evidence_pointer=docs/operations/release-packaging-contract-phase-126.md; activation_enabled=false; signing_enabled=false; publishing_enabled=false; release_readiness_claim=false.

## Daemon prohibition checklist
- [x] daemon prohibition defined as threat_boundary_defined or deferred with spec_is_spec=true; evidence_pointer=docs/operations/release-packaging-contract-phase-126.md; activation_enabled=false; signing_enabled=false; publishing_enabled=false; release_readiness_claim=false.

## Public distribution deferral checklist
- [x] public distribution deferral defined as threat_boundary_defined or deferred with spec_is_spec=true; evidence_pointer=docs/operations/release-packaging-contract-phase-126.md; activation_enabled=false; signing_enabled=false; publishing_enabled=false; release_readiness_claim=false.

## Non-public distribution boundary checklist
- [x] non-public distribution boundary defined as threat_boundary_defined or deferred with spec_is_spec=true; evidence_pointer=docs/operations/release-packaging-contract-phase-126.md; activation_enabled=false; signing_enabled=false; publishing_enabled=false; release_readiness_claim=false.

## Github release deferral checklist
- [x] GitHub release deferral defined as threat_boundary_defined or deferred with spec_is_spec=true; evidence_pointer=docs/operations/release-packaging-contract-phase-126.md; activation_enabled=false; signing_enabled=false; publishing_enabled=false; release_readiness_claim=false.

## Release tag deferral checklist
- [x] release tag deferral defined as threat_boundary_defined or deferred with spec_is_spec=true; evidence_pointer=docs/operations/release-packaging-contract-phase-126.md; activation_enabled=false; signing_enabled=false; publishing_enabled=false; release_readiness_claim=false.

## Public download deferral checklist
- [x] public download deferral defined as threat_boundary_defined or deferred with spec_is_spec=true; evidence_pointer=docs/operations/release-packaging-contract-phase-126.md; activation_enabled=false; signing_enabled=false; publishing_enabled=false; release_readiness_claim=false.

## Public asset deferral checklist
- [x] public asset deferral defined as threat_boundary_defined or deferred with spec_is_spec=true; evidence_pointer=docs/operations/release-packaging-contract-phase-126.md; activation_enabled=false; signing_enabled=false; publishing_enabled=false; release_readiness_claim=false.

## Installer/Updater Threat Table checklist
- [x] Installer/Updater Threat Table completed in docs/operations/installer-update-channel-threat-boundary-phase-127.md.

## Update-Channel Boundary Table checklist
- [x] Update-Channel Boundary Table completed in docs/operations/installer-update-channel-threat-boundary-phase-127.md.

## Signing/Key Custody Table checklist
- [x] Signing/Key Custody Table completed in docs/operations/installer-update-channel-threat-boundary-phase-127.md.

## Rollback Procedure Template checklist
- [x] Rollback Procedure Template completed in docs/operations/installer-update-channel-threat-boundary-phase-127.md.

## Distribution Deferral Table checklist
- [x] Distribution Deferral Table completed in docs/operations/installer-update-channel-threat-boundary-phase-127.md.

## Phase 128/129/130 Dependency Table checklist
- [x] Phase 128/129/130 Dependency Table completed in docs/operations/installer-update-channel-threat-boundary-phase-127.md.

## Missing-dependency decision candidate table checklist
- [x] Missing-dependency decision candidate table completed in docs/operations/installer-update-channel-threat-boundary-phase-127.md.

## Repository artifact-presence check checklist
- [x] Repository artifact-presence check defined and validation scan planned.

## Phase 128 observability evidence expectation checklist
- [x] Phase 128 observability evidence expectation recorded as requires_phase_128_evidence.

## Phase 129 dry-run expectation checklist
- [x] Phase 129 Release Candidate dry-run expectation recorded as requires_phase_129_dry_run.

## Phase 130 decision-gate expectation checklist
- [x] Phase 130 Release Candidate decision-gate expectation recorded as requires_phase_130_decision and may still decide not ready.

## Post-130 production/public-use deferral checklist
- [x] Post-130 production/public-use deferrals recorded and public/general use remains final rung.

## Installer creation prohibition checklist
- [x] installer creation prohibition preserved.

## Updater service prohibition checklist
- [x] updater service prohibition preserved.

## Update-channel activation prohibition checklist
- [x] update-channel activation prohibition preserved.

## Update-channel metadata creation prohibition checklist
- [x] update-channel metadata creation prohibition preserved.

## Signing key creation prohibition checklist
- [x] signing key creation prohibition preserved.

## Key custody behavior prohibition checklist
- [x] key custody behavior prohibition preserved.

## Signature creation prohibition checklist
- [x] signature creation prohibition preserved.

## Publishing prohibition checklist
- [x] publishing prohibition preserved.

## Release artifact prohibition checklist
- [x] release artifact prohibition preserved.

## Package creation prohibition checklist
- [x] package creation prohibition preserved.

## Checksum generation prohibition checklist
- [x] checksum generation prohibition preserved.

## Provenance attestation prohibition checklist
- [x] provenance attestation prohibition preserved.

## Distribution prohibition checklist
- [x] distribution prohibition preserved.

## Deployment automation prohibition checklist
- [x] deployment automation prohibition preserved.

## Rollback automation prohibition checklist
- [x] rollback automation prohibition preserved.

## Background service/daemon prohibition checklist
- [x] background service/daemon prohibition preserved.

## Github release/tag/public asset prohibition checklist
- [x] GitHub release/tag/public asset prohibition preserved.

## Public-release prohibition checklist
- [x] public-release prohibition preserved.

## Production-deployment prohibition checklist
- [x] production-deployment prohibition preserved.

## Public/general-use approval prohibition checklist
- [x] public/general-use approval prohibition preserved.

## Production-human-use approval prohibition checklist
- [x] production-human-use approval prohibition preserved.

## Production candidate approval prohibition checklist
- [x] Production Candidate approval prohibition preserved.

## Release-candidate approval prohibition checklist
- [x] release-candidate approval prohibition preserved.

## Provider trust/output promotion prohibition checklist
- [x] provider trust/output promotion prohibition preserved.

## Replay-repair prohibition checklist
- [x] replay-repair prohibition preserved.

## Recovery-promotion prohibition checklist
- [x] recovery-promotion prohibition preserved.

## Action-execution prohibition checklist
- [x] action-execution prohibition preserved.

## Readiness prohibition checklist
- [x] readiness prohibition preserved.

## Production Candidate status checklist
- [x] Production Candidate status remains not approved.

## Release-candidate/public-use status checklist
- [x] Release-candidate readiness remains not approved.
- [x] Public/general use remains not approved.

## Roadmap/changelog truth checklist
- [x] Roadmap remains planned truth.
- [x] CHANGELOG remains historical truth.

## Validation checklist
- [x] CARGO_TARGET_DIR=/tmp/ajentic-phase-127-target ./scripts/check.sh
- [x] git diff --check
- [x] git status --short
- [x] Phase 127 threat-boundary scan
- [x] Required enforcement lines scan
- [x] Drift-vector scan
- [x] Table flag scan
- [x] Required output scan
- [x] Phase relationship scan
- [x] Ladder invariant scan
- [x] No-installer/updater/release-artifact scan
- [x] Key/signature/public asset presence scan
- [x] No-deployment/release authority scan
- [x] No-authority scan
- [x] Readiness scan
- [x] Source guard
- [x] Roadmap guard

## Findings table
| finding | status | evidence_pointer |
| --- | --- | --- |
| Phase 127 scope | threat_boundary_defined | docs/operations/installer-update-channel-threat-boundary-phase-127.md |
| Missing future evidence | remap_phase_126_130 | deferred_to_phase_128/deferred_to_phase_129/deferred_to_phase_130 |


## Threat table
| category | status | spec_is_spec | evidence_pointer |
| --- | --- | --- | --- |
| installer_specification | threat_boundary_defined | spec_is_spec=true | docs/operations/release-packaging-contract-phase-126.md |
| updater_specification | threat_boundary_defined | spec_is_spec=true | docs/roadmap/phases.md |
| update_channel_metadata | threat_boundary_defined_with_findings | spec_is_spec=true | deferred_to_phase_128 |


## Update-channel boundary table
| channel_type | auth_model | distribution_boundary | attacker_scenario | vector | impact | mitigation | evidence_pointer | deferred_activation |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| appcast_specification | future signed metadata required | non_public | forged appcast | appcast claim | live update confusion | defer metadata | deferred_to_phase_128 | deferred_activation=true |
| package_repository_specification | future repository auth required | deferred_to_phase_130_decision | mirror compromise | repository claim | distribution confusion | defer repository | deferred_to_phase_130 | deferred_activation=true |


## Signing/key custody table
| signer_role | custody_plan_pointer | verification_process_pointer | signing_enabled | publishing_enabled | escalation status |
| --- | --- | --- | --- | --- | --- |
| release steward and security reviewer separation required | deferred_to_phase_130 | deferred_to_phase_129 | signing_enabled=false | publishing_enabled=false | requires_release_steward_escalation |


## Rollback procedure table
| playbook_pointer | no_automation | no_background_service | no_update_execution |
| --- | --- | --- | --- |
| deferred_to_phase_128 | no_automation=true | no_background_service=true | no_update_execution=true |


## Distribution deferral table
| artifact or channel | distribution marker | evidence_pointer |
| --- | --- | --- |
| installer artifact | non_public | docs/operations/release-packaging-contract-phase-126.md |
| GitHub release | deferred_to_phase_130_decision | deferred_to_phase_130 |
| public download | deferred_to_post_130_public_use_phase | deferred_to_post_130_public_use_phase |


## Dependency table
| phase | required evidence | current evidence status | missing dependency | decision outcome candidate |
| --- | --- | --- | --- | --- |
| Phase 128 | observability evidence | requires_phase_128_evidence | not implemented in Phase 127 | remap_phase_126_130 |
| Phase 129 | dry-run evidence | requires_phase_129_dry_run | not implemented in Phase 127 | defer_release_candidate_hardening |
| Phase 130 | decision evidence | requires_phase_130_decision | Phase 128/129 evidence missing | remap_phase_126_130 |


## Deferred items table
| deferred item | marker |
| --- | --- |
| installer creation | deferred_to_phase_130_decision |
| update-channel activation | deferred_to_phase_130_decision |
| public/general use | deferred_to_post_130_public_use_phase |


## Validation log table
| command | status |
| --- | --- |
| CARGO_TARGET_DIR=/tmp/ajentic-phase-127-target ./scripts/check.sh | passed |
| git diff --check | passed |
| git status --short | clean after commit |


## Zero-drift checklist
- [x] No Rust source changes.
- [x] No TypeScript source changes.
- [x] No test changes.
- [x] No schema changes.
- [x] No scripts changed.
- [x] No release artifacts created.
- [x] No generated artifacts retained.
- [x] Allowed Phase 127 surfaces only.
