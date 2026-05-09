---
truth_dimension: orientation
authority_level: advisory
mutation_path: readme_update
---

# Phase 117 operator human-trial dry-run procedure

## Scope
Phase 117 defines operator documentation and a controlled human-trial dry-run procedure for AJENTIC after the Phase 116 local deployment candidate boundary. It is a documentation and rehearsal phase only.

This procedure is local, bounded, manual-review-oriented, deterministic where validation commands are deterministic, non-public, non-releasing, non-production, non-authoritative, and explicitly non-approving.

## Evidence rule
Only committed repository evidence counts for this dry run:

- source files;
- tests;
- UI behavior tests;
- validation scripts;
- governance docs;
- architecture docs;
- roadmap docs;
- operations docs;
- changelog surfaces;
- checklists;
- schemas;
- CI/workflow files.

Do not count prompt intent, prior chat summaries, uncommitted work, speculative roadmap claims, future phase descriptions as implemented behavior, passing validation as readiness approval, local deployment candidacy as human-use approval, dry-run completion as human-use approval, security audit evidence as human-use approval, or operator documentation as operational approval.

## Human-trial dry-run boundary
The human-trial dry run is a rehearsal of the procedure that operators would use to prepare for later controlled human-use gate review. The dry run may verify that roles, preflight steps, evidence collection, session note expectations, stop conditions, escalation paths, manual-review steps, prohibited actions, and handoff requirements are understandable and complete.

The dry run must not enroll real users for production use, approve controlled human use, approve public/general use, approve production human use, create release artifacts, deploy to production, start background services, run daemons, add provider execution expansion, promote provider output, expand persistence authority, repair replay, promote recovery, execute actions, or mutate readiness status.

## Dry run is not human-use approval
Dry-run evidence is rehearsal evidence only. It is not human-use approval, public/general-use approval, production-human-use approval, controlled-human-use approval, public usability approval, or permission to operate AJENTIC with real users.

## Dry run is not release approval
Dry-run evidence is not release approval, release-candidate approval, release-candidate readiness, public release approval, production readiness, or authority to create release artifacts.

## Dry run is not Production Candidate approval
Dry-run evidence is not Production Candidate approval and does not change Production Candidate status.

## Relationship to Phase 102 human operator workflow contract
Phase 102 defined the human operator workflow contract, including operator roles, review ownership, stop conditions, handoff expectations, non-readiness language, and local-only dry-run expectations. Phase 117 reuses that role language for a rehearsal procedure only.

Phase 117 does not expand the Phase 102 contract into runtime behavior. Phase 102 remains documentation/contract evidence, and Phase 117 may cite it only as committed evidence for role and workflow boundaries.

## Relationship to Phase 116 local deployment candidate boundary
Phase 116 defined a controlled local deployment candidate boundary only. Phase 117 preserves the Phase 116 boundary by requiring the dry run to remain local-only, non-public, non-releasing, non-deploying, non-authoritative, and manual-review-gated.

Local deployment candidate evidence may be referenced during the dry run, but local deployment candidacy is not human-use approval, release approval, release-candidate readiness, public usability approval, production deployment approval, or Production Candidate approval.

## Relationship to Phase 115 residual-risk audit
Phase 115 audited security threats, abuse cases, mitigations, trust boundaries, and residual risks. Phase 117 requires explicit residual-risk acknowledgement before dry-run closure.

Residual-risk acknowledgement is risk evidence only. It does not approve human use, production use, public use, release-candidate readiness, readiness, or Production Candidate status.

## Trial roles and responsibilities
The dry run must assign the Phase 102 role language before any rehearsal steps begin:

| Role | Phase 117 responsibility |
| --- | --- |
| Trial coordinator | Owns the dry-run agenda, role assignment, evidence checklist, stop-condition review, escalation routing, closure notes, and Phase 118 handoff package. |
| Operator | Runs local-only preflight and validation commands, captures outputs, records session notes, and stops immediately on stop conditions. |
| Reviewer | Performs manual review of committed evidence references, non-approval language, dry-run notes, and closure disposition. |
| Security reviewer | Reviews security concern escalations, Phase 115 residual-risk acknowledgement, unresolved high/critical residual-risk items, and participant safety concerns. |
| Release steward | Reviews release-boundary escalations and confirms that no release artifact, public asset, release tag, GitHub release, deployment automation, installer, update-channel, signing, or publishing behavior is introduced. |

## Trial coordinator responsibilities
The Trial coordinator must:

1. confirm repository cleanliness before the dry run;
2. assign Operator, Reviewer, Security reviewer, and Release steward coverage;
3. verify that Phase 113, Phase 114, Phase 115, and Phase 116 evidence references are present;
4. verify that Phase 102 role and workflow language is used consistently;
5. confirm the local-only rehearsal posture;
6. confirm the participant boundary;
7. require preflight validation steps and evidence capture;
8. maintain the stop-condition checklist;
9. route security concern and participant safety concern escalations to the Security reviewer;
10. route release-boundary escalations to the Release steward;
11. ensure all dry-run session notes include explicit non-approval language;
12. prepare the Phase 118 handoff checklist if and only if dry-run closure is bounded and non-approving.

## Operator responsibilities
The Operator must run only local documentation, review, and validation steps. The Operator must not create release artifacts, start deployment automation, mutate runtime authority, execute actions, promote provider output, repair replay, promote recovery, or treat dry-run results as approval.

The Operator captures exact commands, pass/fail status, relevant output summaries, generated artifact cleanup if any, and stop-condition disposition.

## Reviewer responsibilities
The Reviewer must manually confirm that the dry-run report, checklist, and changelog surfaces match committed evidence and do not claim readiness approval. The Reviewer must treat validation success as evidence only, not approval.

## Security reviewer responsibilities
The Security reviewer must review any security concern, participant safety concern, unresolved Phase 115 residual-risk acknowledgement gap, or high/critical residual-risk ambiguity. Security reviewer notes are required in the Phase 118 handoff if escalation occurs.

## Release steward responsibilities
The Release steward must review any release-boundary concern, including release artifact creation, installer behavior, update-channel behavior, signing behavior, publishing behavior, GitHub release creation, release tag creation, public asset creation, public release behavior, deployment automation, or production deployment behavior. Release steward notes are required in the Phase 118 handoff if escalation occurs.

## Local-only trial rehearsal posture
The dry run is a local-only rehearsal. It may inspect committed documents, Rust-owned validation evidence, tests, UI review surfaces, scripts, schemas, roadmap entries, changelog entries, and checklists as evidence. It must not add public release behavior, production deployment behavior, deployment automation, installer behavior, update-channel behavior, signing behavior, publishing behavior, background services, daemon behavior, network service exposure, provider trust, provider output promotion, persistence authority expansion, replay repair, recovery promotion, or action execution.

## Participant boundary
No production users, public users, external participants, or real controlled-human-use participants are enrolled by Phase 117. Dry-run participants are repository operators and reviewers rehearsing procedure only.

Any claim that the dry run authorizes controlled human use, public/general use, production human use, public usability, release-candidate readiness, or Production Candidate status is a stop condition.

## Preflight validation steps
The dry-run procedure requires the following preflight checks:

1. Run `git status --short` and confirm the worktree is clean or classify every uncommitted file.
2. Remove generated artifact drift before dry-run evidence capture, including `core/target/.rustc_info.json`, UI build artifacts, test temp files, export temp files, node build artifacts, coverage output, `scripts/__pycache__`, Cargo target drift, or other generated artifact drift.
3. Confirm the current roadmap names Phase 117 as operator documentation and human-trial dry run.
4. Confirm Phase 102 human operator workflow contract evidence exists.
5. Confirm Phase 113 deployment configuration evidence exists.
6. Confirm Phase 114 policy/governance evidence exists.
7. Confirm Phase 115 security audit and residual risk evidence exists.
8. Confirm Phase 116 local deployment candidate evidence exists.
9. Run required validation commands for the phase.
10. Stop on validation failure, dirty worktree, generated artifact drift, missing evidence reference, or unclear operator ownership.

## Required evidence collection
Dry-run evidence collection must include:

- preflight repository cleanliness check output;
- validation command names, pass/fail outcomes, and relevant output summaries;
- local-only candidate evidence check and references;
- Phase 113 deployment configuration evidence check;
- Phase 114 policy/governance evidence check;
- Phase 115 residual-risk acknowledgement check;
- Phase 116 local deployment candidate validation check;
- operator role assignment;
- Trial coordinator assignment;
- Reviewer assignment;
- Security reviewer assignment or escalation disposition;
- Release steward assignment or escalation disposition;
- evidence capture checklist completion;
- stop-condition checklist disposition;
- non-approval language checklist completion;
- dry-run closure checklist;
- Phase 118 handoff checklist if closure is successful.

## Session note expectations
Session notes are written manually as evidence; Phase 117 adds no logging runtime behavior. Session notes must record:

- date of dry run;
- repository commit or branch reference under review;
- role assignments;
- commands run;
- validation result summary;
- evidence references reviewed;
- stop conditions checked;
- escalations opened and closed;
- manual review disposition;
- explicit non-approval statement.

## Manual-review requirements
Manual review is mandatory for:

- Phase 102 relationship evidence;
- Phase 113 deployment configuration evidence;
- Phase 114 policy/governance evidence;
- Phase 115 residual-risk acknowledgement;
- Phase 116 local deployment candidate boundary evidence;
- prohibited-action checks;
- non-approval language;
- stop-condition disposition;
- Phase 118 handoff contents.

Manual review may mark evidence as present, missing, unclear, escalated, or deferred with owner. Manual review must not mark the system approved for human use, public use, production use, release-candidate readiness, Production Candidate status, or readiness.

## Stop conditions
Stop the dry run immediately if any of the following occurs:

- validation failure;
- dirty worktree or generated artifact drift;
- missing Phase 113 evidence reference;
- missing Phase 114 evidence reference;
- missing Phase 115 evidence reference;
- missing Phase 116 evidence reference;
- unresolved Phase 115 high/critical residual-risk acknowledgement;
- public-use claim;
- production-use claim;
- release-candidate approval claim;
- Production Candidate approval claim;
- release artifact creation;
- installer behavior;
- update-channel behavior;
- signing behavior;
- publishing behavior;
- GitHub release creation;
- release tag creation;
- public asset creation;
- deployment automation introduced;
- production deployment behavior introduced;
- provider trust claim;
- provider output promotion claim;
- replay repair claim;
- recovery promotion claim;
- action execution claim;
- readiness approval claim;
- unclear operator ownership;
- unreviewed evidence gap;
- security concern;
- participant safety concern.

## Escalation paths
Use these escalation paths during the dry run:

| Trigger | Escalation path | Required disposition |
| --- | --- | --- |
| Security concern, participant safety concern, or unresolved high/critical residual-risk ambiguity | Trial coordinator to Security reviewer | Security reviewer notes, risk disposition, and dry-run stop/continue decision. |
| Release artifact, installer, update-channel, signing, publishing, GitHub release, release tag, public asset, public release, deployment automation, or production deployment concern | Trial coordinator to Release steward | Release steward notes and confirmation that no release/deployment authority was added. |
| Missing Phase 113, Phase 114, Phase 115, or Phase 116 evidence reference | Trial coordinator to Reviewer | Evidence gap disposition and owner; dry run remains stopped until resolved or deferred outside approval language. |
| Unclear operator ownership | Trial coordinator | Role assignment correction before resuming. |
| Validation failure, dirty worktree, or generated artifact drift | Operator to Trial coordinator and Reviewer | Failure record, cleanup record if applicable, and rerun only after correction. |

## Prohibited actions
Phase 117 prohibits:

- controlled human-use approval;
- public/general-use approval;
- production-human-use approval;
- public usability approval;
- readiness approval;
- release-candidate approval;
- release-candidate readiness approval;
- Production Candidate approval;
- production readiness approval;
- release artifact creation;
- installer behavior;
- update-channel behavior;
- signing behavior;
- publishing behavior;
- GitHub release creation;
- release tag creation;
- public download asset creation;
- public release behavior;
- production deployment behavior;
- deployment automation;
- background service or daemon behavior;
- provider execution expansion;
- provider trust;
- provider output promotion;
- persistence authority expansion;
- replay repair;
- recovery promotion;
- action execution.

## Failure classification
Classify failures as:

| Classification | Meaning | Required response |
| --- | --- | --- |
| Validation failure | Required command or scan failed. | Stop, record output, correct outside approval language, rerun. |
| Evidence gap | Required committed evidence is missing or unclear. | Stop, assign owner, document missing reference. |
| Boundary violation | Any prohibited action or approval claim appears. | Stop, escalate to Reviewer and Release steward or Security reviewer as applicable. |
| Security/participant safety concern | A threat, residual risk, or participant safety issue is unresolved. | Stop and escalate to Security reviewer. |
| Role ownership gap | Trial coordinator, Operator, Reviewer, Security reviewer, or Release steward ownership is unclear. | Stop until assignments are explicit. |
| Documentation inconsistency | Checklist, operations doc, changelog, roadmap, or committed evidence conflicts. | Stop and resolve through allowed documentation surfaces only. |

## Dry-run success criteria
A dry run may be considered procedurally successful only when:

- required validation passes;
- worktree hygiene is clean after generated artifact cleanup;
- Phase 102, Phase 113, Phase 114, Phase 115, and Phase 116 evidence references are present;
- Phase 115 residual-risk acknowledgement is reviewed;
- roles are assigned;
- session notes are complete;
- manual review is complete;
- stop conditions are checked and not active;
- escalation notes are present when escalation occurred;
- non-approval language is explicit;
- Phase 118 handoff evidence is bounded and non-approving.

## Dry-run failure criteria
A dry run fails if validation fails, generated artifact drift remains, evidence references are missing, residual-risk acknowledgement is unresolved, role ownership is unclear, manual review is incomplete, stop conditions remain active, escalation is unresolved, prohibited actions appear, or the record implies approval for human use, public use, production use, release-candidate readiness, Production Candidate status, readiness, release, or deployment.

## Required Phase 118 evidence handoff
Phase 117 may hand off the following evidence for Phase 118 release-candidate evidence assembly:

- Phase 117 dry-run procedure report;
- validation results;
- local candidate evidence references;
- residual-risk acknowledgement;
- stop-condition disposition;
- manual-review disposition;
- Operator and Trial coordinator notes;
- Security reviewer notes if escalated;
- Release steward notes if escalated;
- explicit non-approval statement.

Phase 118 evidence assembly is not automatic approval. It is not release-candidate approval, release-candidate readiness, public usability approval, Production Candidate approval, production readiness approval, or human-use approval.

## Release artifact prohibition
Phase 117 must not create release artifacts. Any release artifact creation is a stop condition.

## Deployment automation prohibition
Phase 117 must not add deployment automation. Any deployment automation introduced during the dry run is a stop condition.

## Installer/update-channel prohibition
Phase 117 must not add installer behavior or update-channel behavior. Any installer or update-channel behavior is a stop condition.

## Signing/publishing prohibition
Phase 117 must not add signing behavior or publishing behavior. Any signing or publishing behavior is a stop condition.

## GitHub release/tag/public asset prohibition
Phase 117 must not create a GitHub release, release tag, public asset, or public download asset. Any GitHub release, release tag, public asset, or public download asset creation is a stop condition.

## Public-release prohibition
Phase 117 must not add public release behavior and must not imply public availability.

## Production-deployment prohibition
Phase 117 must not add production deployment behavior and must not imply production deployment readiness.

## Public/general-use approval prohibition
Phase 117 must not approve public/general use or public usability.

## Production-human-use approval prohibition
Phase 117 must not approve production human use.

## Production Candidate approval prohibition
Phase 117 must not approve Production Candidate status.

## Release-candidate approval prohibition
Phase 117 must not approve release-candidate readiness or release-candidate approval.

## Provider trust/output promotion prohibition
Phase 117 must not grant provider trust or promote provider output.

## Replay-repair prohibition
Phase 117 must not add replay repair.

## Recovery-promotion prohibition
Phase 117 must not add recovery promotion.

## Action-execution prohibition
Phase 117 must not add action execution.

## Readiness approval prohibition
Phase 117 must not approve readiness, production readiness, public usability, release-candidate readiness, Production Candidate status, public/general use, controlled human use, or production human use.

## Relationship to Phase 118 release-candidate evidence assembly
Phase 118 may assemble Phase 117 evidence for review. Phase 118 remains evidence assembly only and is not automatic approval.

## Relationship to Phase 119 Production Candidate reassessment
Phase 119 may reassess Production Candidate posture only after evidence assembly. Phase 117 does not decide or pre-approve the Phase 119 Production Candidate reassessment.

## Relationship to Phase 120-or-later controlled human-use gate
Phase 120 or later may evaluate a controlled human-use gate if intervening committed evidence supports review. Phase 117 does not approve controlled human use and does not guarantee that Phase 120 or any later gate will approve it.

## Required future implementation evidence
Future implementation evidence would be required before any human-use, public-use, release-candidate, Production Candidate, production deployment, provider trust, provider output promotion, persistence authority expansion, replay repair, recovery promotion, or action execution claim could be reviewed. Such future evidence must be committed and validated through the appropriate Rust-owned authority paths and governance boundaries.

## Phase 118 gate decision
Phase 117 may allow Phase 118 to begin only if operator documentation and dry-run procedure evidence is bounded, local, manual-review-oriented, deterministic where applicable, non-public, non-releasing, non-production, non-authoritative, and explicitly non-approving.

If any stop condition remains active, Phase 118 must not begin from Phase 117 evidence.

## Phase 119 deferrals
Production Candidate reassessment is deferred to Phase 119. Phase 117 does not approve, deny, or implement Phase 119.

## Phase 120-or-later deferrals
Controlled human-use gate decisions are deferred to Phase 120 or later. Phase 117 does not approve controlled human use.

## Production Candidate status
Production Candidate status: not approved.

## Release-candidate readiness status
Release-candidate readiness: not approved.

## Public/general use status
Public/general use: not approved. Public usability: not approved. Production human use: not approved.

## Roadmap/changelog truth posture
Roadmap files remain planned truth. `CHANGELOG.md` remains historical truth for completed accepted work. This operations document is advisory orientation evidence and does not override governance, architecture, roadmap, checklist, or changelog authority.

## Required follow-ups
Required follow-ups for later phases are limited to evidence assembly, review, and gate decisions named in roadmap truth. They do not create approval in Phase 117.

## Deferred items
The following remain deferred beyond Phase 117:

- release-candidate evidence assembly to Phase 118;
- Production Candidate reassessment to Phase 119;
- controlled human-use gate decision to Phase 120 or later;
- any production deployment, public release, installer, update-channel, signing, publishing, provider trust, provider output promotion, replay repair, recovery promotion, persistence authority expansion, or action execution behavior to future committed evidence if ever authorized.

## Confirmed vs suspected
Confirmed evidence must be committed repository evidence. Suspected behavior, prompt intent, future roadmap language, uncommitted work, or local dry-run notes without committed references must not be counted as implemented behavior.

## Non-readiness statement
Phase 117 is operator documentation and human-trial dry run only. Dry-run evidence is not human-use approval, release approval, Production Candidate approval, release-candidate readiness, public usability approval, public/general-use approval, production-human-use approval, production readiness, public release approval, production deployment approval, or operational approval.
