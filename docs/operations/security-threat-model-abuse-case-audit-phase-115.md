---
truth_dimension: orientation
authority_level: advisory
mutation_path: readme_update
---

# Phase 115 - Security Threat Model and Abuse-Case Audit

## Scope
Phase 115 is security threat model and abuse-case audit only. It audits current committed trust boundaries, abuse cases, residual risks, required corrections, and required follow-ups after Phase 114 policy/governance evidence versioning.

Phase 115 adds no runtime behavior, adds no new application capability, adds no provider execution expansion, adds no deployment automation, creates no release artifacts, adds no installer behavior, adds no update-channel behavior, adds no signing or publishing behavior, adds no public release behavior, and adds no production deployment behavior.

Phase 115 expands no persistence authority, expands no recovery behavior, adds no replay repair, adds no recovery promotion, adds no action execution, adds no provider trust, promotes no provider output, approves no readiness, approves no Production Candidate status, approves no release-candidate readiness, approves no production readiness, approves no public usability, and approves no production human use.

## Evidence rule
Only committed evidence counts for this audit:

- source files
- tests
- UI behavior tests
- validation scripts
- governance docs
- architecture docs
- roadmap docs
- operations docs
- changelog surfaces
- checklists
- schemas
- CI/workflow files

The following do not count as evidence: prompt intent, prior chat summaries, uncommitted work, speculative roadmap claims, future phase descriptions as implemented behavior, passing validation as readiness approval, security audit findings as security approval, or absence of an exploit as proof of safety.

Security audit evidence is risk evidence, not approval authority.

## Security audit status model
Each major audit area uses one of these statuses:

| Status | Meaning |
| --- | --- |
| `mitigated_by_committed_evidence` | Committed implementation, tests, scripts, docs, or CI evidence directly mitigate the area within current scope. |
| `partially_mitigated` | Committed evidence mitigates part of the area, but residual risk remains material. |
| `residual_risk_documented` | The risk is documented and bounded but not fully mitigated. |
| `insufficient_evidence` | Current committed evidence is not enough to verify the area. |
| `deferred_to_later_phase` | The area is intentionally outside Phase 115 and must be addressed in a named later phase. |
| `not_applicable` | The area is not currently active behavior or not present in the audited state. |

## Risk severity model
Each confirmed residual risk uses one of these severities:

| Severity | Meaning |
| --- | --- |
| `critical` | Could directly grant unauthorized authority, cause unsafe execution, or materially compromise release/human-use safety if activated. |
| `high` | Could undermine trust boundaries or evidence integrity under plausible local or future activation conditions. |
| `medium` | Could confuse operators, obscure evidence quality, or require controls before broader use. |
| `low` | Limited practical impact in the current local, non-authoritative posture. |
| `informational` | Tracked for clarity, orientation, or future planning. |

## Risk disposition model
Each confirmed residual risk uses one of these dispositions:

| Disposition | Meaning |
| --- | --- |
| `accept_for_next_bounded_phase` | Acceptable only for Phase 116 to proceed as a bounded local deployment candidate boundary, with no readiness or release approval. |
| `requires_followup_before_release_candidate_evidence` | Must be addressed before Phase 118 release-candidate evidence assembly can be considered complete. |
| `requires_followup_before_production_candidate_reassessment` | Must be addressed before Phase 119 Production Candidate reassessment can support approval. |
| `requires_followup_before_human_use` | Must be addressed before Phase 120-or-later controlled human-use gates. |
| `deferred_to_named_phase` | Deferred to an explicit planned phase. |
| `not_accepted` | Not acceptable for proceeding without correction. |

## Trust boundary inventory
| Boundary | Status | Committed evidence | Audit finding |
| --- | --- | --- | --- |
| Human operator boundary | `partially_mitigated` | Governance intent rule, README orientation, current checklist, operations docs. | Human requests remain intents, but malicious or mistaken local operators remain residual risks. |
| UI boundary | `mitigated_by_committed_evidence` | UI source and UI boundary tests keep TypeScript non-authoritative. | UI displays and previews operator intent; it does not create authority. |
| Local transport boundary | `mitigated_by_committed_evidence` | Rust local transport validation, UI local transport code, adversarial tests, Phase 104/105 docs. | Malformed, spoofed, replay-shaped, and authority-bearing transport payloads are fail-closed within committed evidence. |
| Provider configuration boundary | `mitigated_by_committed_evidence` | Provider configuration Rust validation, tests, Phase 106 docs. | Hidden enablement, fallback, auto-selection, trust, and persistence escalation are rejected. |
| Provider execution sandbox boundary | `partially_mitigated` | Local deterministic stub sandbox, tests, Phase 107 docs. | Current sandbox is bounded and output remains untrusted; future real providers remain residual risk. |
| Timeout/resource boundary | `mitigated_by_committed_evidence` | Timeout/resource enforcement code and adversarial tests, Phase 108 docs. | Timeout/resource evidence is descriptive and rejects retry or limit escalation. |
| Decision-evidence persistence append boundary | `partially_mitigated` | Phase 109/111 docs, Rust persistence decision and append surfaces, tests. | Narrow decision-evidence append exists; manual file tampering and broader persistence remain residual. |
| Recovery lifecycle classification boundary | `partially_mitigated` | Recovery lifecycle code, tests, Phase 112 docs. | Recovery is classification and in-memory acceptance only; no repair or promotion. Local evidence tampering remains residual. |
| Deployment configuration boundary | `residual_risk_documented` | Phase 113 docs and validation code. | Deployment configuration is contract-only and no deployment automation exists; future packaging/deployment risks remain. |
| Policy/governance evidence attribution boundary | `mitigated_by_committed_evidence` | Phase 114 docs, governance evidence validation, adversarial tests. | Evidence attribution is descriptive, rejects approval claims, and does not rewrite governance. |
| Changelog/history boundary | `partially_mitigated` | CHANGELOG and archive docs validation. | Changelog is historical truth; manual edits/rollback remain residual without signed history. |
| Roadmap/planned-truth boundary | `mitigated_by_committed_evidence` | Roadmap files identify Phase 115 as security audit only and later phases as planned truth. | Roadmap remains planned truth, not implemented behavior. |
| Validation-script boundary | `partially_mitigated` | `scripts/check.sh`, docs/structure validation, Rust/UI boundary lints, CI workflows. | Validation is strong for known patterns but cannot prove security readiness or detect all semantic bypasses. |
| Filesystem/storage boundary | `residual_risk_documented` | Persistence/recovery/deployment docs and source references to storage paths. | Local filesystem tampering, path substitution, symlink risks, rollback, and environment permissions remain residual. |
| Process/network boundary | `residual_risk_documented` | Local-only transport posture, provider sandbox docs/tests, scans for process/network APIs. | Current posture is local/non-public, but future remote/cloud/process expansion requires security review. |
| Release/deployment boundary | `deferred_to_later_phase` | Roadmap phases 116, 118, 119, 120; release checklist. | Release/deployment remains deferred; no release, installer, update, signing, publishing, or production deployment is approved. |

## Human operator boundary
Human operators can request work, provide local paths, run scripts, edit files, and decide whether to proceed to later phases. Current governance treats operator input as intent unless accepted through Rust-owned paths.

Status: `partially_mitigated`.

Findings:

- The boundary is explicit in governance and architecture: external surfaces submit typed intents, and Rust decides validity.
- Phase 115 evidence cannot prevent a malicious local operator from editing repository files, persisted evidence, local storage, environment variables, or historical records.
- Operator training, review discipline, and clean-worktree validation remain required before human-use gates.

## UI boundary
The browser UI is a review and operator-intent surface only.

Status: `mitigated_by_committed_evidence`.

Findings:

- UI behavior tests cover malformed submission, authority escalation text, fake approval status, execution flag spoofing, persistence flag spoofing, provider execution flag spoofing, replay repair spoofing, live transport spoofing, and authority mutation spoofing.
- UI code may render projections and submit typed intents but does not decide policy, validation, provider trust, persistence, replay repair, recovery promotion, or action execution.
- Residual UI risk is primarily operator misunderstanding of non-authority state.

## Local transport boundary
The local transport boundary handles UI-to-Rust/local runtime communication as typed, local, bounded input.

Status: `mitigated_by_committed_evidence`.

Findings:

- Committed tests and docs reject malformed local transport input, spoofed locality, replay-shaped payloads, duplicate payloads, oversize payloads, authority-bearing operations, provider execution escalation, persistence escalation, replay repair, recovery promotion, and action execution.
- Local-only posture does not equal authenticated remote service readiness.
- Spoofed locality remains a future concern if any remote/cloud binding is introduced.

## Provider configuration boundary
Provider configuration currently describes and validates provider configuration without hidden execution or provider trust.

Status: `mitigated_by_committed_evidence`.

Findings:

- Unsupported providers, hidden enablement, unsafe fallback, auto-selection coercion, duplicate identifiers, invalid resources, malformed input, provider trust, and persistence escalation are rejected.
- Configuration contracts do not activate remote/cloud providers.
- Future provider selection must remain explicit and fail closed.

## Provider execution sandbox boundary
Provider execution evidence is limited to a bounded deterministic local stub sandbox.

Status: `partially_mitigated`.

Findings:

- Provider output is untrusted candidate material and not promoted to authority.
- Sandbox success does not approve persistence, workflow completion authority, replay repair, recovery promotion, action execution, readiness, release, or production use.
- Future nondeterministic providers, cloud providers, secrets, network I/O, process spawning, and filesystem writes are residual attack surfaces.

## Timeout/resource boundary
Timeout and resource boundaries constrain provider execution evidence and reject limit escalation.

Status: `mitigated_by_committed_evidence`.

Findings:

- Timeout exhaustion, oversized output, excessive operation claims, retry coercion, and resource-limit escalation are rejected or bounded by committed evidence.
- Timeout/resource success remains descriptive and does not grant trust, persistence, readiness, release, deployment, or human-use approval.
- Environment-specific runtime performance remains a residual operational variable.

## Decision-evidence persistence append boundary
Decision-evidence persistence is narrow and Rust-validated.

Status: `partially_mitigated`.

Findings:

- Phase 111 historical evidence narrowed append activation to decision evidence only.
- Provider-output persistence coercion, workflow-completion persistence coercion, sandbox-success persistence coercion, UI-authorized persistence, transport-authorized persistence, replay repair, recovery promotion, action execution, readiness promotion, and trust promotion are prohibited.
- Manual local file replacement, rollback, symlink/path substitution, and direct edits of persisted evidence remain residual risks.

## Recovery lifecycle classification boundary
Recovery lifecycle behavior is classification-oriented and does not repair replay or promote recovery into authority.

Status: `partially_mitigated`.

Findings:

- Recovery candidate preparation rejects malformed bytes, checksum mismatch, duplicate/conflict abuse, export-bundle-shaped input, replay repair, promotion, and action execution attempts.
- In-memory acceptance does not append ledger/audit records, repair replay, promote provider output, execute action, or replace global state.
- Local evidence tampering and rollback remain residual risks without stronger storage integrity controls.

## Deployment configuration boundary
Deployment configuration is a contract-only posture.

Status: `residual_risk_documented`.

Findings:

- Phase 113 documents storage paths, permissions, retention, environment assumptions, failure handling, and manual review for corrupt or unsupported recovery evidence.
- It adds no deployment automation, installer, update channel, signing, publishing, public release, or production deployment.
- Unsafe storage declarations, path traversal-shaped declarations, and environment permission assumptions require follow-up before release-candidate evidence and human-use gates.

## Policy/governance evidence attribution boundary
Policy/governance evidence attribution is descriptive and non-authoritative.

Status: `mitigated_by_committed_evidence`.

Findings:

- Phase 114 rejects governance rewrite claims, policy authority grants, duplicate evidence identifiers, contradictory labels, unsupported truth dimensions, missing source references, deployment approval, release-candidate approval, Production Candidate approval, public-use approval, production-human-use approval, provider trust, provider output promotion, persistence expansion, replay repair, recovery promotion, and action execution.
- Governance docs remain normative truth; architecture docs remain structural truth; roadmap remains planned truth; changelog remains historical truth; operations docs remain advisory orientation.
- Evidence attribution does not approve readiness.

## Changelog/history boundary
The changelog records completed accepted work as historical truth.

Status: `partially_mitigated`.

Findings:

- Current and archived changelog surfaces separate history from planned truth.
- Archive validation helps detect changelog structure drift.
- Rollback, replay attacks against repository history, and manual editing remain residual unless future release evidence adds stronger provenance, tags, or signatures.

## Roadmap/planned-truth boundary
Roadmap files define planned truth only.

Status: `mitigated_by_committed_evidence`.

Findings:

- Roadmap confirms Phase 115 as Security Threat Model and Abuse-Case Audit, security audit only.
- Phase 116, if recommended, is the next planned local deployment candidate boundary phase only.
- Phase 118 remains release-candidate evidence assembly only.
- Phase 119 remains Production Candidate reassessment only.
- Phase 120 or later remains controlled human-use gate work only.
- Planned truth does not implement behavior and does not approve readiness.

## Validation-script boundary
Validation scripts and CI workflows enforce known repository, documentation, UI, and Rust boundary constraints.

Status: `partially_mitigated`.

Findings:

- `scripts/check.sh`, structure validation, docs validation, Rust boundary lint, UI boundary lint, tests, and workflows provide defense against known drift.
- Passing validation is not readiness approval.
- Regex/lint scans may miss semantic bypasses, generated artifact drift outside known paths, environmental differences, malicious local edits between validation and commit, and future unmodeled patterns.

## Filesystem/storage boundary
Filesystem and storage boundaries include repository files, generated build output, local target directories, persisted evidence, export temp files, UI build output, node artifacts, and operator-provided paths.

Status: `residual_risk_documented`.

Findings:

- Current committed evidence documents storage assumptions but does not provide tamper-proof local storage.
- Symlink/path traversal, local file replacement, rollback/replay, direct evidence editing, filesystem permission drift, and generated artifact drift remain residual risks.
- Phase 115 cleaned no generated drift because the initial worktree was clean.

## Process/network boundary
Process and network boundaries include local scripts, Rust execution paths, UI network APIs, provider boundary code, workflows, and any future remote/cloud execution.

Status: `residual_risk_documented`.

Findings:

- Current posture is local-only/non-public and no public network service is approved.
- Process spawning, remote providers, cloud transport, live networking, and background services are not expanded by Phase 115.
- Future process/network expansion must undergo separate threat modeling before human use.

## Release/deployment boundary
Release and deployment are not approved by this audit.

Status: `deferred_to_later_phase`.

Findings:

- Phase 115 creates no release artifacts.
- Phase 115 adds no deployment automation, installer, update-channel behavior, signing/publishing behavior, public release behavior, or production deployment behavior.
- Release/deployment questions remain deferred to later named phases and require separate evidence.

## Abuse-case inventory
| Abuse case | Status | Current mitigation or residual finding | Required follow-up |
| --- | --- | --- | --- |
| Malformed local transport input | `mitigated_by_committed_evidence` | Fail-closed parsing and adversarial transport tests. | Continue regression coverage. |
| Spoofed local transport locality | `partially_mitigated` | Local bind/loopback posture rejects public/remote claims. | Reassess before any remote/cloud binding. |
| Replay-shaped local transport payloads | `mitigated_by_committed_evidence` | Replay-shaped and duplicate payloads are rejected as authority. | Keep deterministic reason codes. |
| Authority-bearing transport payloads | `mitigated_by_committed_evidence` | Authority and execution claims reject. | Continue negative tests. |
| Provider configuration hidden enablement | `mitigated_by_committed_evidence` | Hidden execution/secrets activation rejects. | Reassess if real providers added. |
| Provider fallback/auto-selection coercion | `mitigated_by_committed_evidence` | Unsafe fallback/auto-selection rejects. | Reassess before provider expansion. |
| Provider-output injection | `partially_mitigated` | Provider output remains untrusted data. | More corpus coverage before release-candidate evidence. |
| Provider-output trust/promotion attempts | `mitigated_by_committed_evidence` | Trust and promotion claims reject. | Continue no-authority scans. |
| Resource exhaustion | `partially_mitigated` | Resource declarations bounded in current stub context. | Environment and OS-level limits before human use. |
| Timeout exhaustion | `mitigated_by_committed_evidence` | Timeout exhaustion is descriptive and bounded. | Continue tests. |
| Retry/limit-escalation attempts | `mitigated_by_committed_evidence` | Retry/limit escalation rejects. | Continue tests. |
| Persistence authority injection | `mitigated_by_committed_evidence` | Persistence expansion claims reject. | Continue no-authority scans. |
| Provider-output persistence coercion | `mitigated_by_committed_evidence` | Provider output cannot be persisted as authority. | Continue tests. |
| Workflow-completion persistence coercion | `mitigated_by_committed_evidence` | Workflow completion does not grant persistence. | Continue tests. |
| Sandbox-success persistence coercion | `mitigated_by_committed_evidence` | Sandbox success remains descriptive. | Continue tests. |
| Recovery record tampering | `partially_mitigated` | Checksums and classification reject mismatch. | Stronger storage integrity before human use. |
| Recovery checksum mismatch | `mitigated_by_committed_evidence` | Mismatch rejects. | Continue tests. |
| Recovery duplicate/conflict abuse | `mitigated_by_committed_evidence` | Duplicate/conflict abuse rejects. | Continue tests. |
| Recovery silent-repair attempts | `mitigated_by_committed_evidence` | Silent repair is prohibited. | Continue tests. |
| Replay repair coercion | `mitigated_by_committed_evidence` | Replay repair claims reject. | Continue no-authority scans. |
| Recovery promotion coercion | `mitigated_by_committed_evidence` | Recovery promotion claims reject. | Continue no-authority scans. |
| Action execution coercion | `mitigated_by_committed_evidence` | Action execution claims reject. | Continue tests. |
| Deployment automation coercion | `mitigated_by_committed_evidence` | Deployment automation claims reject. | Reassess in Phase 116. |
| Installer/update/signing/publishing coercion | `residual_risk_documented` | No current behavior; future release surfaces deferred. | Required before release-candidate evidence. |
| Unsafe storage path declarations | `partially_mitigated` | Deployment configuration is contract-only. | Path policy before human use. |
| Path traversal-shaped declarations | `partially_mitigated` | Contract docs identify manual review needs. | Add explicit validation if paths become active. |
| Symlink/path substitution risks | `residual_risk_documented` | No tamper-proof storage. | Required before human use. |
| Local file replacement risks | `residual_risk_documented` | Clean-worktree validation helps repository drift only. | Integrity/provenance before human use. |
| Rollback/replay attack risks | `residual_risk_documented` | Changelog/history are not signed. | Required before release-candidate evidence. |
| Governance evidence spoofing | `mitigated_by_committed_evidence` | Phase 114 evidence validation rejects missing/duplicate/contradictory references. | Continue tests. |
| Policy version spoofing | `mitigated_by_committed_evidence` | Policy labels and source references validated. | Continue tests. |
| Readiness approval injection | `mitigated_by_committed_evidence` | Readiness approval claims reject. | Continue scans. |
| Production Candidate approval injection | `mitigated_by_committed_evidence` | Production Candidate approval claims reject. | Continue scans. |
| Release-candidate approval injection | `mitigated_by_committed_evidence` | Release-candidate approval claims reject. | Continue scans. |
| Public-use/human-use approval injection | `mitigated_by_committed_evidence` | Public-use and production-human-use claims reject. | Continue scans. |
| UI event/action escalation | `mitigated_by_committed_evidence` | UI behavior tests reject action/execution escalation. | Continue tests. |
| Browser storage/network misuse | `residual_risk_documented` | UI is non-authoritative; browser misuse still requires future review. | Required before human use. |
| Script validation bypass | `partially_mitigated` | Lints and docs validation exist. | Review blind spots before release-candidate evidence. |
| Generated artifact drift | `partially_mitigated` | Clean-worktree and target-dir hygiene reduce drift. | Continue final status and source guard. |
| CI/local validation divergence | `partially_mitigated` | Workflows mirror major checks. | Confirm parity before release-candidate evidence. |

## Existing mitigation inventory
| Mitigation | Status | Evidence tie |
| --- | --- | --- |
| Rust authority boundary | `mitigated_by_committed_evidence` | Governance, architecture, Rust API ownership, Rust boundary lint. |
| Typed validation | `mitigated_by_committed_evidence` | Rust API validation modules and tests. |
| Fail-closed parsing | `mitigated_by_committed_evidence` | Local transport, provider configuration, deployment configuration, governance evidence, recovery, persistence, and adversarial tests. |
| Deterministic reason codes | `mitigated_by_committed_evidence` | Validation reports expose stable reason codes in Rust tests. |
| Adversarial tests | `mitigated_by_committed_evidence` | `tests/adversarial_corpus.rs` and UI behavior tests. |
| UI non-authority posture | `mitigated_by_committed_evidence` | UI code/tests and governance UI rule. |
| Local-only transport posture | `mitigated_by_committed_evidence` | Phase 104/105 operations docs and transport tests. |
| Provider-output untrusted posture | `mitigated_by_committed_evidence` | Provider sandbox, timeout/resource, governance evidence tests. |
| Timeout/resource enforcement | `mitigated_by_committed_evidence` | Phase 108 evidence and tests. |
| Retry/limit-escalation rejection | `mitigated_by_committed_evidence` | Timeout/resource adversarial tests. |
| Narrow decision-evidence append | `partially_mitigated` | Phase 109/111 docs and persistence tests. |
| Recovery classification-only behavior | `partially_mitigated` | Phase 112 docs and recovery tests. |
| Deployment configuration contract-only posture | `residual_risk_documented` | Phase 113 docs and deployment configuration validation. |
| Policy/governance attribution-only posture | `mitigated_by_committed_evidence` | Phase 114 docs/tests. |
| Boundary lint scripts | `partially_mitigated` | Rust/UI boundary lints. |
| Structure/docs validation | `partially_mitigated` | `scripts/validate_structure.py` and `scripts/validate_docs.py`. |
| Clean-worktree validation posture | `partially_mitigated` | Required final `git status --short` and source guard. |
| Changelog archive validation posture | `partially_mitigated` | Changelog archive files and docs validation. |

## Residual risk inventory
| Risk | Severity | Disposition | Status | Rationale |
| --- | --- | --- | --- | --- |
| Local filesystem tampering | `high` | `requires_followup_before_human_use` | `residual_risk_documented` | A local user can alter repository files or persisted evidence outside Rust authority. |
| Malicious local operator | `high` | `requires_followup_before_human_use` | `residual_risk_documented` | Governance cannot prevent deliberate misuse by an operator with local write access. |
| Compromised local account | `critical` | `requires_followup_before_human_use` | `residual_risk_documented` | Account compromise can bypass local-only assumptions and alter files, env, tools, or evidence. |
| Symlink/path traversal and filesystem substitution | `high` | `requires_followup_before_human_use` | `residual_risk_documented` | Current contract docs do not provide active tamper-proof path handling for all future surfaces. |
| Rollback/replay attacks | `high` | `requires_followup_before_release_candidate_evidence` | `residual_risk_documented` | Repository/persisted evidence history can be rolled back without stronger provenance. |
| Manually edited persisted evidence | `high` | `requires_followup_before_human_use` | `residual_risk_documented` | Narrow append does not prevent manual file modification outside Rust. |
| Environment-specific permissions | `medium` | `requires_followup_before_human_use` | `residual_risk_documented` | Local OS permissions and umasks vary. |
| Unpinned toolchain/runtime assumptions | `medium` | `requires_followup_before_release_candidate_evidence` | `residual_risk_documented` | Validation may depend on installed Rust, Node, npm, Python, and platform behavior. |
| Generated Cargo target drift | `low` | `accept_for_next_bounded_phase` | `partially_mitigated` | `CARGO_TARGET_DIR=/tmp/...` reduces repo drift; external temp targets still require cleanup discipline. |
| Validation script blind spots | `medium` | `requires_followup_before_release_candidate_evidence` | `partially_mitigated` | Regex and structural validation cannot prove all semantic boundaries. |
| UI usability misunderstanding of non-authority state | `medium` | `requires_followup_before_human_use` | `residual_risk_documented` | Operators may mistake displayed evidence for approval. |
| Future provider nondeterminism | `high` | `deferred_to_named_phase` | `deferred_to_later_phase` | Real providers are not currently trusted; future expansion needs dedicated controls. |
| Future remote/cloud execution | `critical` | `deferred_to_named_phase` | `deferred_to_later_phase` | Remote/cloud execution changes locality, secrets, network, and provider trust surfaces. |
| Future deployment packaging | `high` | `deferred_to_named_phase` | `deferred_to_later_phase` | Packaging can introduce install, path, dependency, and update risks. |
| Future release artifacts | `high` | `requires_followup_before_release_candidate_evidence` | `deferred_to_later_phase` | Release artifacts need provenance, reproducibility, signing decisions, and artifact inventory. |
| Future installer/update channels | `critical` | `requires_followup_before_human_use` | `deferred_to_later_phase` | Install/update channels can execute code and distribute compromise. |
| Future signing/publishing | `high` | `requires_followup_before_release_candidate_evidence` | `deferred_to_later_phase` | Signing/publishing requires key management and provenance controls. |
| Future human-trial operations | `high` | `requires_followup_before_human_use` | `deferred_to_later_phase` | Human trials require incident, rollback, consent, support, and operator training controls. |
| Incident response/rollback procedures | `medium` | `requires_followup_before_human_use` | `insufficient_evidence` | Procedures are not complete enough for human use. |
| Operator training gaps | `medium` | `requires_followup_before_human_use` | `insufficient_evidence` | Non-authority posture requires operator training and review expectations. |
| Observability gaps | `medium` | `requires_followup_before_release_candidate_evidence` | `partially_mitigated` | Read-only evidence exists, but release evidence needs stronger observability coverage. |

## Local filesystem tampering risk
Severity: `high`.
Disposition: `requires_followup_before_human_use`.
Status: `residual_risk_documented`.

A local user with filesystem access can modify repository files, persisted evidence, generated outputs, local target directories, or configuration outside Rust-owned paths. Current clean-worktree checks reduce accidental drift but do not provide tamper-proof storage.

## Malicious local operator risk
Severity: `high`.
Disposition: `requires_followup_before_human_use`.
Status: `residual_risk_documented`.

A malicious operator can intentionally bypass procedures, edit files, rerun tools with altered environments, or misrepresent validation. Phase 115 does not create identity, authorization, or attestation controls for local operators.

## Compromised local account risk
Severity: `critical`.
Disposition: `requires_followup_before_human_use`.
Status: `residual_risk_documented`.

A compromised local account can tamper with source, evidence, shell configuration, local dependencies, Node/Rust/Python tools, environment variables, and generated outputs. This is not acceptable for human-use gates without stronger operational controls.

## Symlink/path traversal risk
Severity: `high`.
Disposition: `requires_followup_before_human_use`.
Status: `residual_risk_documented`.

Deployment configuration currently documents path assumptions and manual review rather than enforcing comprehensive filesystem isolation. If future phases activate storage or deployment paths, symlink substitution and traversal-shaped declarations require explicit validation.

## Rollback/replay attack risk
Severity: `high`.
Disposition: `requires_followup_before_release_candidate_evidence`.
Status: `residual_risk_documented`.

Repository history, changelog entries, operations reports, and persisted evidence can be rolled back or replayed without release-grade provenance. Phase 115 does not add signatures, immutable logs, trusted timestamps, or release artifact attestation.

## Persisted evidence tampering risk
Severity: `high`.
Disposition: `requires_followup_before_human_use`.
Status: `residual_risk_documented`.

Narrow decision-evidence append is Rust-validated, but manual edits to persisted evidence remain possible outside Rust. Future human-use work needs tamper detection and operational review controls.

## Environment permission risk
Severity: `medium`.
Disposition: `requires_followup_before_human_use`.
Status: `residual_risk_documented`.

Local permissions, umasks, user/group configuration, and filesystem behavior vary by environment. Current docs identify assumptions but do not enforce a hardened runtime environment.

## Toolchain/runtime reproducibility risk
Severity: `medium`.
Disposition: `requires_followup_before_release_candidate_evidence`.
Status: `residual_risk_documented`.

Validation depends on locally installed Rust, Cargo, Node/npm, Python, shell behavior, and OS tools. Phase 115 does not pin or attest a full toolchain/runtime environment.

## Generated artifact drift risk
Severity: `low`.
Disposition: `accept_for_next_bounded_phase`.
Status: `partially_mitigated`.

Using `CARGO_TARGET_DIR=/tmp/ajentic-phase-115-target` keeps Cargo target output out of the repository, and final status/source-guard checks reduce artifact drift. External temporary targets still require operator cleanup discipline.

## Validation-script blind spot risk
Severity: `medium`.
Disposition: `requires_followup_before_release_candidate_evidence`.
Status: `partially_mitigated`.

Validation scripts catch known forbidden patterns and structure drift but cannot prove complete semantic security. Future release-candidate evidence should include broader security-specific validation.

## UI non-authority misunderstanding risk
Severity: `medium`.
Disposition: `requires_followup_before_human_use`.
Status: `residual_risk_documented`.

Operators may misread a UI-rendered report, successful validation, or audit finding as readiness approval. UI copy, documentation, and training must continue to distinguish evidence from approval.

## Future provider nondeterminism risk
Severity: `high`.
Disposition: `deferred_to_named_phase`.
Status: `deferred_to_later_phase`.

Current provider execution is a deterministic local stub and provider output is untrusted. Any future nondeterministic provider requires new controls for replay, provenance, output handling, timeout/resource behavior, and trust denial.

## Future remote/cloud execution risk
Severity: `critical`.
Disposition: `deferred_to_named_phase`.
Status: `deferred_to_later_phase`.

Remote/cloud execution would add secrets, network exposure, provider account controls, remote logs, remote data retention, and locality spoofing concerns. Phase 115 grants no such expansion.

## Future deployment packaging risk
Severity: `high`.
Disposition: `deferred_to_named_phase`.
Status: `deferred_to_later_phase`.

Packaging can introduce dependency, installer, path, permissions, update, signing, and distribution risks. Phase 116 may define a local deployment candidate boundary only; public release remains out of scope.

## Future release artifact risk
Severity: `high`.
Disposition: `requires_followup_before_release_candidate_evidence`.
Status: `deferred_to_later_phase`.

Release artifacts require inventory, reproducibility, provenance, hash/signature decisions, and distribution controls. Phase 115 creates no release artifacts.

## Future human-trial operations risk
Severity: `high`.
Disposition: `requires_followup_before_human_use`.
Status: `deferred_to_later_phase`.

Controlled human trials require operator training, incident response, rollback procedures, consent/support expectations, and clear non-public boundaries. Phase 115 does not approve human use.

## Security findings table
| Finding | Status | Severity | Disposition | Confirmed/suspected |
| --- | --- | --- | --- | --- |
| Phase 115 audit evidence is non-authoritative risk evidence. | `mitigated_by_committed_evidence` | `informational` | `accept_for_next_bounded_phase` | Confirmed. |
| Current Rust/UI/provider/transport boundaries reject known authority escalation payloads. | `mitigated_by_committed_evidence` | `informational` | `accept_for_next_bounded_phase` | Confirmed. |
| Local filesystem and operator compromise can bypass procedural controls. | `residual_risk_documented` | `critical` | `requires_followup_before_human_use` | Confirmed residual. |
| Release artifact provenance is not yet evidenced. | `deferred_to_later_phase` | `high` | `requires_followup_before_release_candidate_evidence` | Confirmed residual. |
| Validation scripts may miss semantic security bypasses. | `partially_mitigated` | `medium` | `requires_followup_before_release_candidate_evidence` | Confirmed residual. |
| Future remote/cloud/provider expansion changes the threat model. | `deferred_to_later_phase` | `critical` | `deferred_to_named_phase` | Confirmed residual. |

## Required corrections
No required correction to roadmap planned truth, governance docs, architecture docs, Rust source, TypeScript source, tests, schemas, scripts, workflows, README, AGENTS, package files, lockfiles, deployment infrastructure, release infrastructure, or archived changelogs was identified during Phase 115.

A narrow validation-script compatibility correction was required after the first full clean-worktree validation run: `scripts/rust_boundary_lint.mjs` now treats `copy(` as the filesystem copy token instead of flagging the ordinary identifier `copy`, and it flags `std::process::Command` rather than safe `std::process::id()` test temp-directory usage. A second compatibility correction restored UI boundary lint detection for `Promise.resolve(...).then(...)` with the TypeScript AST shape in this environment. These changes affect validation only and add no runtime behavior.

## Required follow-ups
- Before Phase 118 release-candidate evidence assembly can be considered complete, add or cite stronger evidence for rollback/replay resistance, release artifact provenance, validation blind-spot review, toolchain/runtime reproducibility, and observability completeness.
- Before Phase 119 Production Candidate reassessment can support approval, re-evaluate all high and critical residual risks against committed evidence, not roadmap intent.
- Before Phase 120-or-later controlled human-use gates, address local filesystem tampering, compromised account assumptions, malicious local operator controls, path/symlink substitution, persisted evidence tampering, operator training, incident response, rollback procedures, and UI non-authority comprehension.
- Before any future provider, remote/cloud, process, network, installer, update-channel, signing, publishing, or public release expansion, run a separate threat model and abuse-case audit for that expansion.

## Deferred items
| Item | Deferred to | Reason |
| --- | --- | --- |
| Local deployment candidate boundary | Phase 116 | May proceed only as local deployment candidate boundary, not public release or readiness approval. |
| Operator documentation and human-trial dry run | Phase 117 | Requires Phase 116 boundary and does not approve public availability. |
| Release-candidate evidence assembly | Phase 118 | Evidence assembly only; no approval. |
| Production Candidate reassessment | Phase 119 | Reassessment only; no automatic approval. |
| Controlled human-use gate | Phase 120 or later | Requires intervening evidence and explicit future gate. |
| Provider/remote/cloud expansion | Later named phase | Not part of Phase 115 or Phase 116. |
| Installer/update/signing/publishing/public release | Later named phase | Not approved and no artifacts created. |

## Relationship to Phase 114 policy/governance evidence
Phase 114 provides committed attribution evidence for policy/governance versions, source references, roadmap references, changelog references, validation-run references, and authority-denial snapshots. Phase 115 treats that evidence as descriptive input to the security audit.

Phase 115 does not convert Phase 114 governance or policy evidence into governance authority, policy authority, readiness approval, deployment approval, release-candidate approval, Production Candidate approval, public-use approval, production-human-use approval, provider trust, provider output promotion, persistence expansion, replay repair, recovery promotion, or action execution.

## Relationship to Phase 116 local deployment candidate boundary
Phase 116, if recommended, is the next planned local deployment candidate boundary phase only. Phase 115 allows Phase 116 to begin only because current trust boundaries, abuse cases, residual risks, and required follow-ups are explicitly documented without granting readiness, deployment approval, release approval, production approval, public usability, or production human use.

## Relationship to Phase 118 release-candidate evidence assembly
Phase 118 remains release-candidate evidence assembly only. Phase 115 identifies follow-ups that must be addressed or explicitly carried before Phase 118 evidence can be considered complete, including rollback/replay resistance, release artifact provenance, validation blind spots, reproducibility, and observability.

## Relationship to Phase 119 Production Candidate reassessment
Phase 119 remains Production Candidate reassessment only. Phase 115 does not approve Production Candidate status. Phase 119 must reassess the residual risk inventory against committed evidence available at that time.

## Relationship to Phase 120-or-later controlled human-use gate
Phase 120 or later remains controlled human-use gate work only. Phase 115 does not approve public/general use, production human use, or public usability. Human-use gates require follow-up controls for local tampering, compromised accounts, operator training, incident response, UI misunderstanding, path safety, and persisted evidence integrity.

## Phase 116 gate decision
Phase 116 may begin as the next planned local deployment candidate boundary phase only.

This gate decision is not readiness approval, release approval, deployment approval, Production Candidate approval, release-candidate approval, public-use approval, production-human-use approval, public usability approval, or production readiness approval.

The Phase 116 gate is accepted only under these constraints:

- no runtime behavior expansion unless Phase 116 explicitly and validly scopes it under committed governance;
- no provider trust;
- no provider output promotion;
- no persistence authority expansion;
- no replay repair;
- no recovery promotion;
- no action execution;
- no installer, update-channel, signing, publishing, public release, or production deployment behavior;
- residual risks from Phase 115 remain open unless corrected by committed evidence.

## Phase 118 deferrals
Release-candidate evidence assembly remains deferred to Phase 118. Phase 115 creates no release artifacts and approves no release-candidate readiness.

## Phase 119 deferrals
Production Candidate reassessment remains deferred to Phase 119. Phase 115 approves no Production Candidate status and no production readiness.

## Phase 120-or-later deferrals
Controlled human-use gate work remains deferred to Phase 120 or later. Phase 115 approves no public usability, public/general use, production human use, or production deployment.

## Production Candidate status
Production Candidate status is not approved.

Security audit evidence is risk evidence, not approval authority. Passing validation is not Production Candidate approval.

## Release-candidate readiness status
Release-candidate readiness is not approved.

Phase 115 creates no release artifacts and does not assemble release-candidate evidence.

## Public/general use status
Public/general use, public usability, and production human use are not approved.

Phase 115 adds no public release behavior and no production deployment behavior.

## Roadmap/changelog truth posture
Roadmap remains planned truth. It may identify future phases, but future phase descriptions do not count as implemented behavior.

CHANGELOG surfaces remain historical truth. They record completed accepted work and do not approve future readiness by themselves.

Operations docs remain advisory orientation unless another authoritative source says otherwise. Checklists remain procedural truth for active phase execution.

## Confirmed vs suspected
Confirmed findings are based on committed evidence reviewed during Phase 115: governance docs, architecture docs, roadmap docs, changelog surfaces, current checklist, operations docs, Rust source, UI source, tests, validation scripts, schemas, and CI/workflow files.

Suspected or future risks are not counted as current behavior. They are recorded as residual or deferred only where future expansion would change the threat model.

## Non-readiness statement
Phase 115 is security audit only. Security audit evidence is risk evidence, not approval authority.

Phase 115 adds no runtime behavior, no new application capability, no deployment automation, no release artifacts, no installer behavior, no update-channel behavior, no signing or publishing behavior, no public release behavior, no production deployment behavior, no persistence authority expansion, no replay repair, no recovery promotion, no action execution, no provider trust, and no provider output promotion.

Phase 115 approves no readiness, no Production Candidate status, no release-candidate readiness, no production readiness, no public usability, and no production human use. Phase 115 does not implement Phase 116.
