---
truth_dimension: orientation
authority_level: advisory
mutation_path: readme_update
---

# Human Operator Workflow Contract - Phase 102

## Scope
Phase 102 defines the human operator workflow contract for controlled human use preparation. It names operator roles, operator responsibilities, workflow states, review states, escalation ownership, stop conditions, evidence capture expectations, approval-language boundaries, and expected state transitions as documentation-only contract concepts.

Phase 102 is documentation/contract only. Phase 102 adds no runtime behavior. Phase 102 adds no new capability. Phase 102 does not activate UI runtime review. Phase 102 does not add live transport. Phase 102 does not add provider execution. Phase 102 does not add persistence authority. Phase 102 does not add recovery behavior. Phase 102 does not add action execution. Phase 102 does not approve readiness. Phase 102 does not approve Production Candidate status. Phase 102 does not approve release-candidate readiness. Phase 102 does not approve production readiness. Phase 102 does not approve public usability. Phase 102 does not implement Phase 103.

## Evidence rule
Count only committed evidence:

- source files;
- tests;
- UI behavioral tests;
- golden/adversarial tests;
- validation logs;
- lint gates;
- operations docs;
- checklists;
- roadmap/changelog truth surfaces.

Do not count:

- future roadmap entries as implemented workflow;
- intended UI behavior;
- intended transport behavior;
- architecture rationale alone;
- unmerged/non-history agent runs;
- speculative operator assumptions;
- prompt intent without committed files.

Roadmap remains planned truth. CHANGELOG.md remains historical truth.

## Contract status model
This contract separates documentation status from readiness status:

| Contract status | Meaning | Boundary |
| --- | --- | --- |
| Drafted for Phase 102 | The workflow contract is documented for review and handoff. | Documentation/contract only. |
| Evidence sufficient for current boundary | The committed documentation evidence is sufficient for the Phase 102 boundary. | Not readiness approval. |
| Accepted for next bounded phase | The Phase 102 documentation may be handed to the next bounded phase. | Does not implement Phase 103. |
| Not approved for readiness | Readiness is explicitly withheld. | Applies to Production Candidate, release-candidate, public usability, and production human use. |

## Operator role model
Each role may inspect evidence and make bounded workflow decisions. No role may convert Phase 102 documentation into runtime authority or readiness approval.

| Role | scope | responsibility | may decide | may not decide | evidence the role must inspect | escalation trigger |
| --- | --- | --- | --- | --- | --- | --- |
| Operator | Runs local documented commands and records observed results. | Follow local workflow instructions, capture command output, identify unexpected states, and avoid authority assumptions. | Whether a local validation attempt is complete enough to submit for review. | May not approve readiness, bypass validation gates, execute actions outside documented boundaries, or treat UI visibility as authority. | README, local workflow docs, current-phase checklist, validation output, dry-run output, UI behavioral evidence when applicable. | validation gate failure, unclear operator ownership, generated artifact drift, human-trial safety concern. |
| Reviewer | Reviews evidence for the current bounded phase. | Compare submitted evidence to required checklists, stop conditions, and non-authority guarantees. | Whether evidence is sufficient for current boundary or whether changes are requested. | May not approve Production Candidate status, release-candidate readiness, public usability, production readiness, or production human use. | Operations contract, current-phase checklist, CHANGELOG.md entry, validation logs, boundary lint output, test output. | unreviewed evidence gap, readiness language drift, source/surface drift, UI behavioral test failure. |
| Maintainer | Owns repository surface discipline and phase acceptance mechanics. | Confirm allowed surfaces, commit hygiene, artifact cleanup, and changelog/checklist fidelity. | Whether documentation changes can be accepted for next bounded phase after validation passes. | May not infer runtime capability, live transport, provider execution, persistence authority, or recovery behavior from docs. | git status, staged diff, source/script/workflow guard, CHANGELOG.md, checklist closure, validation output. | source/surface drift, generated artifact drift, release artifact creation, validation gate failure. |
| Release steward | Reviews release and packaging boundaries without approving release readiness. | Confirm release dry-run language, package-boundary expectations, and non-release posture. | Whether release-boundary language is correctly non-approving for Phase 102. | May not create release artifacts, approve release-candidate readiness, approve distribution, approve signing, approve publishing, or approve public download assets. | release checklist posture, CHANGELOG.md notes, build/package-boundary expectations, release dry-run expectations. | release artifact creation, readiness language drift, package/distribution/signing/publishing approval language. |
| Security reviewer | Owns security escalation assessment for suspected boundary violations. | Review security concerns, hostile input implications, authority drift, and human-trial safety concerns. | Whether a concern requires security review before further phase acceptance. | May not approve provider/model execution, persistence authority, transport activation, recovery promotion, replay repair, or action execution in Phase 102. | boundary lint output, adversarial/golden test evidence, no-runtime/no-authority scan output, operations stop conditions. | security concern, live transport introduced outside phase, provider/model call introduced outside phase, persistence authority introduced outside phase, action execution introduced outside phase. |
| Trial coordinator | Coordinates preparation expectations for any later controlled human trial. | Ensure human-trial preparation remains evidence-based, bounded, reversible, and not approved in Phase 102. | Whether human-trial preparation evidence is organized enough for later review. | May not start a human trial, approve production human use, approve public/general use, or treat local operator testing as production approval. | human-trial preparation expectations, operator responsibilities, handoff expectations, non-readiness statement, Phase 101 blockers. | human-trial safety concern, unclear operator ownership, evidence insufficient, escalation ownership ambiguity. |

## Operator responsibilities
Operators must:

- run only documented local commands and dry-run commands;
- capture exact command output, exit status, and any generated artifact drift;
- record validation gate failure, boundary lint failure, UI behavioral test failure, golden test failure, adversarial test failure, and source/surface drift as stop conditions;
- avoid provider/model calls, live transport, persistence authority, recovery promotion, replay repair, and action execution outside their bounded phases;
- hand off evidence to a Reviewer before any acceptance language is used;
- use accepted for local operator testing only for local non-production exercises that remain inside documented boundaries;
- treat not approved for readiness as the default posture for production, release, and public/general use.

## Workflow states
| Workflow state | entry condition | required evidence | allowed action | prohibited action | exit condition | stop condition |
| --- | --- | --- | --- | --- | --- | --- |
| Intake | A bounded phase request or local operator workflow request is received. | Phase title/scope from roadmap, allowed surfaces, initial git status, evidence rule. | Classify requested work and identify required evidence. | Starting runtime work, UI activation, live transport, provider calls, persistence, or action execution. | Request is in scope and assigned for Local validation. | unclear operator ownership or source/surface drift. |
| Local validation | Intake confirms a bounded local validation path. | `./scripts/check.sh`, explicit tests/lints/builds as required, CLI dry-run output. | Run validation and record pass/fail evidence. | Masking failures, skipping required checks, creating release artifacts, or ignoring generated artifact drift. | All required local validation evidence is captured. | validation gate failure, boundary lint failure, generated artifact drift. |
| Dry-run review | Local validation evidence exists. | CLI dry-run output, local startup expectations, build/package-boundary expectations, release dry-run expectations when applicable. | Review dry-run evidence for current boundary. | Treating dry-run success as production readiness or release approval. | Dry-run evidence is sufficient for Evidence review or rejected. | release artifact creation, readiness language drift. |
| Evidence review | Dry-run review evidence is available. | Tests, lints, UI behavioral tests, golden/adversarial filters, scans, checklist closure, changelog fidelity. | Determine evidence sufficient for current boundary, evidence insufficient, Deferred, or Escalated. | Counting future roadmap entries, intended UI behavior, intended transport behavior, or speculative assumptions as implemented evidence. | Evidence is accepted for Human review, Deferred, Escalated, or Rejected. | unreviewed evidence gap, UI behavioral test failure, golden/adversarial test failure. |
| Human review | Evidence review finds the current-boundary evidence reviewable. | Reviewer notes, role evidence inspection, stop-condition disposition, approval-language check. | Human Reviewer may mark evidence sufficient for current boundary or request changes. | Approving Production Candidate status, release-candidate readiness, public usability, production readiness, or production human use. | Accepted for next bounded phase, Rejected, Deferred, or Escalated. | security concern or human-trial safety concern. |
| Rejected | A required condition fails or evidence is invalid. | Failure output, rejected evidence notes, violated stop/rejection condition. | Close without approval and document reason. | Continuing the workflow as accepted or readiness-approved. | New bounded intake may begin only with corrected evidence. | validation gate failure or rejection condition remains unresolved. |
| Deferred | Evidence is incomplete but not invalid. | Missing-evidence list and owner. | Pause pending additional bounded evidence. | Treating deferred work as accepted or implemented. | Evidence returns to Evidence review or is Rejected. | unreviewed evidence gap or unclear operator ownership. |
| Escalated | Security, release, ownership, or safety concern requires owner review. | Escalation reason, owner, affected evidence, stop condition. | Security reviewer, Release steward, Maintainer, or Trial coordinator reviews. | Bypassing escalation owner or continuing toward readiness approval. | Escalated -> Rejected or Escalated -> Accepted for next bounded phase after current-boundary evidence is sufficient. | security concern, release artifact creation, human-trial safety concern. |
| Accepted for next bounded phase | Human review determines current-boundary evidence is sufficient. | Checklist closure, changelog entry, validation log, zero-drift confirmation. | Hand off documentation evidence to the next bounded phase. | Implementing Phase 103, approving readiness, or expanding authority. | Phase 102 closes as documentation/contract accepted for current boundary. | Any state -> Not approved for readiness remains true. |
| Not approved for readiness | Default status for Phase 102 and any state with readiness uncertainty. | Non-readiness statement and non-approval language. | State explicitly that readiness is not approved. | Claiming Production Candidate, release-candidate, public usability, production readiness, or production human-use approval. | Remains until a later authorized gate changes it with committed evidence. | readiness language drift. |

## Review states
| Review state | Meaning | Allowed transition |
| --- | --- | --- |
| Not reviewed | Evidence has not been reviewed by a human role. | Not reviewed -> Under review. |
| Under review | A Reviewer is inspecting current-boundary evidence. | Under review -> Changes requested, Evidence sufficient for current boundary, Evidence insufficient, Escalated for security review, or Escalated for release review. |
| Changes requested | Evidence or wording needs correction within the same boundary. | Changes requested -> Under review or Closed without approval. |
| Evidence sufficient for current boundary | Evidence satisfies the documentation/contract boundary only. | Evidence sufficient for current boundary -> Accepted for next bounded phase. |
| Evidence insufficient | Required evidence is missing, invalid, or not committed. | Evidence insufficient -> Deferred, Rejected, or Changes requested. |
| Escalated for security review | Security reviewer ownership is required. | Escalated for security review -> Rejected, Changes requested, or Evidence sufficient for current boundary. |
| Escalated for release review | Release steward ownership is required. | Escalated for release review -> Rejected, Changes requested, or Evidence sufficient for current boundary. |
| Closed without approval | Review is closed without acceptance or readiness approval. | Terminal for that review record. |

## Expected state-transition model
Text-only model; no code and no schema are added for this state machine.

```text
Intake -> Local validation -> Dry-run review -> Evidence review -> Human review -> Accepted for next bounded phase
Intake -> Local validation -> Rejected
Local validation -> Dry-run review
Dry-run review -> Evidence review
Evidence review -> Human review
Human review -> Accepted for next bounded phase
Evidence review -> Deferred
Evidence review -> Escalated
Escalated -> Rejected
Escalated -> Accepted for next bounded phase
Any state -> Not approved for readiness
```

## Evidence capture expectations
Evidence capture responsibilities are assigned as follows:

- Operator captures command output, local startup observations, dry-run output, artifact drift, and any validation gate failure.
- Reviewer captures evidence sufficiency findings, evidence insufficient findings, changes requested, and closed without approval notes.
- Maintainer captures git status, staged file list, allowed-surface confirmation, changelog fidelity, and zero-drift findings.
- Release steward captures release dry-run expectations, release artifact absence, and release-candidate readiness: not approved posture.
- Security reviewer captures security concern disposition, boundary lint findings, and no-runtime/no-authority findings.
- Trial coordinator captures human-trial safety concern disposition and confirms production human use is not approved.

Evidence must include exact command names where practical and must distinguish discovery scans from blocking enforcement. `rg` scans are discovery/evidence only. Blocking enforcement remains `scripts/check.sh`, Rust boundary lint, UI AST lint, compiler/type checks, clippy, UI behavioral tests, and Rust tests. Phase 102 must not change lint behavior.

## Escalation ownership
| Escalation | Owner | Required evidence | Exit |
| --- | --- | --- | --- |
| validation gate failure | Maintainer | failing command output and affected surface | Rejected or Changes requested. |
| boundary lint failure | Maintainer with Security reviewer if authority-related | lint output and boundary rule | Rejected or Escalated for security review. |
| UI behavioral test failure | Reviewer with Maintainer | UI test output | Changes requested or Rejected. |
| generated artifact drift | Maintainer | git status and cleanup record | Rejected until cleaned or Changes requested. |
| readiness language drift | Reviewer with Release steward | offending language and corrected non-approval wording | Changes requested or Rejected. |
| release artifact creation | Release steward | artifact list and removal record | Rejected until removed. |
| live transport introduced outside phase | Security reviewer | diff evidence | Rejected. |
| provider/model call introduced outside phase | Security reviewer | diff evidence | Rejected. |
| persistence authority introduced outside phase | Security reviewer | diff evidence | Rejected. |
| recovery promotion introduced outside phase | Security reviewer | diff evidence | Rejected. |
| replay repair introduced outside phase | Security reviewer | diff evidence | Rejected. |
| action execution introduced outside phase | Security reviewer | diff evidence | Rejected. |
| unclear operator ownership | Trial coordinator with Maintainer | role gap notes | Deferred or Changes requested. |
| unreviewed evidence gap | Reviewer | missing committed evidence list | Deferred or Rejected. |
| security concern | Security reviewer | concern description and affected boundary | Escalated for security review. |
| human-trial safety concern | Trial coordinator with Security reviewer | safety concern and trial impact | Escalated, Deferred, or Rejected. |

## Stop conditions
The workflow must stop on:

- validation gate failure;
- boundary lint failure;
- UI behavioral test failure;
- golden test failure;
- adversarial test failure;
- generated artifact drift;
- source/surface drift;
- readiness language drift;
- release artifact creation;
- live transport introduced outside phase;
- provider/model call introduced outside phase;
- persistence authority introduced outside phase;
- recovery promotion introduced outside phase;
- replay repair introduced outside phase;
- action execution introduced outside phase;
- unclear operator ownership;
- unreviewed evidence gap;
- security concern;
- human-trial safety concern.

## Rejection conditions
Reject current-boundary acceptance when any of the following are present:

- required validation fails or is skipped;
- failure output is masked by a zero exit code without being documented as a validation-gate defect;
- generated artifacts, Cargo target drift, node build artifacts, coverage output, test temp files, export temp files, or `scripts/__pycache__` remain in the final diff;
- source, test, script, workflow, schema, package, dependency, lockfile, README, AGENTS, UI config, release infrastructure, or roadmap surfaces change outside the allowed Phase 102 scope;
- approval language claims readiness, Production Candidate status, release-candidate readiness, public usability, production readiness, or production human use;
- live UI-to-Rust transport, provider/model calls, persistence authority, recovery promotion, replay repair, or action execution is introduced;
- evidence relies on future roadmap intent or speculative assumptions instead of committed files.

## Approval language boundaries
Allowed bounded phrases:

- accepted for next bounded phase: documentation evidence can be handed off, but this is not readiness approval.
- accepted for local operator testing: local non-production operator exercises may proceed only within documented boundaries and only when later phases authorize them.
- evidence sufficient for current boundary: Phase 102 documentation/contract evidence is sufficient for Phase 102 only.

Required non-approval phrases:

- not approved for readiness;
- not approved for Production Candidate status;
- not approved for release-candidate readiness;
- not approved for public usability;
- not approved for production human use.

Production Candidate status: not approved. release-candidate readiness: not approved. production readiness: not approved. public usability: not approved. production human use is not approved. public/general use is not approved.

## Handoff expectations
Handoffs must include:

- the Phase 102 contract document;
- current-phase checklist closure;
- CHANGELOG.md historical entry;
- validation commands and pass/fail evidence;
- role ownership for unresolved issues;
- stop-condition disposition;
- explicit non-readiness language;
- confirmation that Phase 102 does not implement Phase 103.

Handoff language may say accepted for next bounded phase only when evidence is sufficient for current boundary and all stop conditions are cleared or deferred with ownership. It may not say that any release, production, public-use, or Production Candidate gate is approved.

## Dry-run expectations
Dry-run expectations are limited to local command evidence. A successful dry run may support evidence sufficient for current boundary, but it does not imply release-candidate readiness, production readiness, public usability, production human use, provider execution, persistence authority, action execution, or recovery behavior.

## UI review expectations
Phase 102 does not activate UI runtime review. UI review expectations are contract language only: later UI work must make human-visible review states understandable, non-authoritative, and consistent with this contract. Intended UI behavior is not counted as implemented workflow evidence in Phase 102.

## Local startup expectations
Local startup expectations remain bounded by committed local workflow and startup documentation. Operators may inspect documented startup commands and outputs as evidence, but Phase 102 does not approve startup readiness, daemon behavior, server behavior, public network access, live transport, or production human use.

## Build and packaging-boundary expectations
Build and packaging-boundary expectations are evidence capture expectations only. Cargo and UI builds may be run as validation, but Phase 102 does not change package files, lockfiles, packaging behavior, installer behavior, distribution behavior, signing behavior, publishing behavior, or auto-update behavior.

## Release dry-run expectations
Release dry-run expectations are limited to confirming that no release artifacts are created and no release approval language is introduced. Phase 102 creates no tags, release branches, uploaded artifacts, signed artifacts, installer artifacts, distribution archives, package-registry artifacts, Docker/container images, update manifests, or public download assets.

## Human-trial preparation expectations
Human-trial preparation expectations require role ownership, stop conditions, evidence capture, escalation paths, and non-readiness language. Phase 102 does not start a human trial, approve controlled human use, approve production human use, approve public/general use, or close Phase 101 blockers for UI activation, live transport, provider execution, persistence authority, deployment, recovery, action execution, release-candidate evidence, or production-candidate reassessment.

## Non-authority guarantees
Phase 102 guarantees only a documentation contract. It does not mutate authority and does not add runtime behavior, CLI surface, release tooling, packaging behavior, installer behavior, distribution behavior, signing behavior, publishing behavior, auto-update behavior, authority surface, transport, provider/model call, persistence, durable append, export write, replay repair, recovery acceptance, recovery behavior, action behavior, or action execution.

## Prohibited inferences
Do not infer that:

- roadmap planned truth is implemented workflow;
- CHANGELOG.md entries approve future phases;
- Phase 102 activates UI runtime review;
- Phase 102 adds live UI-to-Rust transport;
- Phase 102 adds provider execution;
- Phase 102 adds persistence authority;
- Phase 102 adds recovery behavior;
- Phase 102 adds action execution;
- local validation success means release approval;
- dry-run success means Production Candidate approval;
- accepted for next bounded phase means accepted for production human use;
- accepted for local operator testing means public usability;
- evidence sufficient for current boundary means production readiness;
- Phase 103 is implemented or approved for readiness by this contract.

## Relationship to Phase 101 gap decomposition
Phase 101 identified remaining production-use blockers and recommended Phase 102 as the next planned documentation/contract phase only. This contract responds to the operator-workflow blocker by defining roles, states, evidence capture, escalation ownership, stop conditions, and approval-language boundaries. It does not claim closure for runtime, UI activation, live transport, provider execution, persistence authority, deployment, security audit, human trial, release-candidate evidence, Production Candidate reassessment, public usability, or production human use.

## Relationship to Phase 103 UI runtime review surface
Phase 103, if recommended, is the next planned UI usability phase only. Phase 103 is named in planned truth as UI Runtime Review Surface Activation Boundary. Phase 102 may hand off this contract to Phase 103 only as documentation evidence. This is not readiness approval, not Production Candidate approval, not release-candidate readiness approval, and not public usability approval. Phase 102 does not implement Phase 103.

## Required future implementation evidence
Future phases would need committed evidence before any capability can be treated as implemented, including:

- UI runtime review surface evidence and UI behavioral tests;
- live local UI-to-Rust transport implementation and transport hardening evidence;
- provider configuration and provider execution sandbox evidence;
- provider timeout and resource limit evidence;
- persistence authority decision evidence and durable persistence tests;
- deployment/local service evidence;
- security review evidence;
- controlled human trial evidence;
- release-candidate evidence;
- Production Candidate reassessment evidence.

These are required future implementation evidence categories only; Phase 102 does not implement them.

## Phase 103 gate decision
Phase 103 may begin as the next planned UI usability phase only if Phase 102 is accepted for next bounded phase. That gate is not readiness approval. Phase 103 remains UI Runtime Review Surface Activation Boundary, and Phase 102 does not implement Phase 103.

## Production Candidate status
Production Candidate status: not approved. Phase 102 does not approve Production Candidate status and does not create a Production Candidate branch, tag, artifact, or release.

## Release-candidate readiness status
release-candidate readiness: not approved. Phase 102 does not approve release-candidate readiness, does not approve release artifacts, and does not approve release publishing.

## Public/general use status
public usability: not approved. public/general use is not approved. production human use is not approved. Phase 102 does not approve production readiness or production human use.

## Roadmap/changelog truth posture
Roadmap remains planned truth. CHANGELOG.md remains historical truth. Phase 102 documentation may cite roadmap planned truth only to confirm title, scope, and sequencing; it may not treat future roadmap entries as completed work.

## Required follow-ups
Required follow-ups are deferred to later bounded phases and must be supported by committed implementation evidence before being treated as complete:

- Phase 103 UI usability work for the runtime review surface;
- later live local transport work;
- later provider execution and sandbox work;
- later persistence authority decision and implementation work;
- later deployment, security review, human trial, release-candidate, and Production Candidate reassessment work.

## Deferred items
| Deferred item | Reason | Owner for later phase |
| --- | --- | --- |
| UI runtime review activation | Phase 102 is documentation/contract only. | Maintainer and Reviewer. |
| Live local transport | Phase 102 does not add live transport. | Maintainer and Security reviewer. |
| Provider/model execution | Phase 102 does not add provider execution. | Security reviewer. |
| Persistence authority | Phase 102 does not add persistence authority. | Maintainer and Security reviewer. |
| Recovery behavior and replay repair | Phase 102 does not add recovery behavior or replay repair. | Maintainer and Security reviewer. |
| Action execution | Phase 102 does not add action execution. | Maintainer and Security reviewer. |
| Controlled human trial | Phase 102 does not approve production human use. | Trial coordinator. |
| Release-candidate readiness | Phase 102 does not approve release-candidate readiness. | Release steward. |
| Production Candidate status | Phase 102 does not approve Production Candidate status. | Release steward and Maintainer. |

## Confirmed vs suspected
Confirmed:

- Phase 102 title/scope are present in roadmap planned truth.
- Phase 102 is documentation/contract only.
- Phase 103 is planned after Phase 102 as UI Runtime Review Surface Activation Boundary.
- Phase 101 blockers are not closed by runtime implementation in Phase 102.
- The Phase 102 evidence rule counts committed files and validation evidence only.

Suspected but not counted as implemented evidence:

- later UI behavior;
- later transport behavior;
- later provider/model behavior;
- later persistence authority;
- later release readiness;
- later human-trial behavior.

## Non-readiness statement
Phase 102 is documentation/contract only. Phase 102 adds no runtime behavior, no new capability, no UI runtime review activation, no live transport, no provider execution, no persistence authority, no recovery behavior, and no action execution. Phase 102 does not approve readiness, Production Candidate status, release-candidate readiness, production readiness, public usability, production human use, or public/general use. Phase 102 does not implement Phase 103.
