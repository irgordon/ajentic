---
truth_dimension: procedural
authority_level: authoritative
mutation_path: checklist_revision
---

# Current Phase Checklist - Phase 115

## Phase name
- [x] Phase 115 - Security Threat Model and Abuse-Case Audit.

## Phase goal
- [x] Audit current trust boundaries, abuse cases, residual attack surfaces, risk severity, risk disposition, required corrections, and required follow-ups after Phase 114 policy/governance evidence versioning.
- [x] Treat security audit evidence as risk evidence, not approval authority.
- [x] Do not implement Phase 116.

## Working-tree hygiene gate
- [x] Run `git status --short` before edits.
- [x] Classify uncommitted files before edits.
- [x] Remove/revert generated artifact drift before edits if present.
- [x] Record cleanup status: initial worktree was clean; no generated artifact cleanup was required.

## Allowed surfaces
- [x] `docs/operations/security-threat-model-abuse-case-audit-phase-115.md`.
- [x] `checklists/current-phase.md`.
- [x] `CHANGELOG.md`.

## Boundary rules
- [x] Phase 115 is security audit only.
- [x] Phase 115 adds no runtime behavior.
- [x] Phase 115 adds no new application capability.
- [x] Phase 115 adds no provider execution expansion.
- [x] Phase 115 adds no deployment automation.
- [x] Phase 115 starts no services.
- [x] Phase 115 creates no release artifacts.
- [x] Phase 115 adds no installer behavior.
- [x] Phase 115 adds no update-channel behavior.
- [x] Phase 115 adds no signing/publishing behavior.
- [x] Phase 115 adds no public release behavior.
- [x] Phase 115 adds no production deployment behavior.
- [x] Phase 115 expands no persistence authority.
- [x] Phase 115 expands no recovery behavior.
- [x] Phase 115 adds no replay repair.
- [x] Phase 115 adds no recovery promotion.
- [x] Phase 115 adds no action execution.
- [x] Phase 115 adds no provider trust.
- [x] Phase 115 promotes no provider output.
- [x] Phase 115 approves no readiness.
- [x] Phase 115 approves no Production Candidate status.
- [x] Phase 115 approves no release-candidate readiness.
- [x] Phase 115 approves no production readiness.
- [x] Phase 115 approves no public usability.
- [x] Phase 115 approves no production human use.

## Governance/architecture inspection checklist
- [x] Read `docs/governance/GOVERNANCE.md`.
- [x] Read `docs/governance/phase-execution-contract.md`.
- [x] Read `docs/architecture/ARCHITECTURE.md`.
- [x] Read architecture subdocuments in `docs/architecture/`.
- [x] Confirm Rust authority boundary.
- [x] Confirm TypeScript UI non-authority boundary.
- [x] Confirm model output remains untrusted until validated through Rust-owned paths.

## Roadmap/changelog inspection checklist
- [x] Read `docs/roadmap/phase-map.md`.
- [x] Read `docs/roadmap/phases.md`.
- [x] Read `docs/roadmap/sequencing.md`.
- [x] Confirm Phase 115 title and scope from roadmap files.
- [x] Read `CHANGELOG.md`.
- [x] Read `docs/changelog/CHANGELOG-0001-0055.md`.
- [x] Read `docs/changelog/CHANGELOG-0056-0104.md`.
- [x] Confirm roadmap remains planned truth.
- [x] Confirm changelog surfaces remain historical truth.

## Implementation-surface inspection checklist
- [x] Inspect `core/src/api/**`.
- [x] Inspect `core/src/validation/**`.
- [x] Inspect `core/src/ledger/**`.
- [x] Inspect `core/src/audit/**`.
- [x] Inspect `core/src/memory/**`.
- [x] Inspect `ui/src/api/**`.
- [x] Inspect `ui/src/app/**`.
- [x] Inspect `tests/**`.
- [x] Use implementation inspection as evidence only; do not modify source or tests.

## Validation-script inspection checklist
- [x] Inspect `scripts/check.sh`.
- [x] Inspect `scripts/validate_structure.py`.
- [x] Inspect `scripts/validate_docs.py`.
- [x] Inspect `scripts/rust_boundary_lint.mjs`.
- [x] Inspect `scripts/test_rust_boundary_lint.mjs`.
- [x] Inspect `scripts/lint_ui_boundaries.mjs`.
- [x] Inspect `scripts/test_lint_ui_boundaries.mjs`.
- [x] Inspect `.github/workflows/**`.
- [x] No workflow changes were required.
- [x] Narrow `scripts/rust_boundary_lint.mjs` and `scripts/lint_ui_boundaries.mjs` compatibility corrections were required after full clean-worktree validation exposed lint compatibility issues; no runtime behavior was affected.

## Trust-boundary inventory checklist
- [x] Human operator boundary audited.
- [x] UI boundary audited.
- [x] Local transport boundary audited.
- [x] Provider configuration boundary audited.
- [x] Provider execution sandbox boundary audited.
- [x] Timeout/resource boundary audited.
- [x] Decision-evidence persistence append boundary audited.
- [x] Recovery lifecycle classification boundary audited.
- [x] Deployment configuration boundary audited.
- [x] Policy/governance evidence attribution boundary audited.
- [x] Changelog/history boundary audited.
- [x] Roadmap/planned-truth boundary audited.
- [x] Validation-script boundary audited.
- [x] Filesystem/storage boundary audited.
- [x] Process/network boundary audited.
- [x] Release/deployment boundary audited.

## Abuse-case inventory checklist
- [x] Malformed local transport input assessed.
- [x] Spoofed local transport locality assessed.
- [x] Replay-shaped local transport payloads assessed.
- [x] Authority-bearing transport payloads assessed.
- [x] Provider configuration hidden enablement assessed.
- [x] Provider fallback/auto-selection coercion assessed.
- [x] Provider-output injection assessed.
- [x] Provider-output trust/promotion attempts assessed.
- [x] Resource exhaustion assessed.
- [x] Timeout exhaustion assessed.
- [x] Retry/limit-escalation attempts assessed.
- [x] Persistence authority injection assessed.
- [x] Provider-output persistence coercion assessed.
- [x] Workflow-completion persistence coercion assessed.
- [x] Sandbox-success persistence coercion assessed.
- [x] Recovery record tampering assessed.
- [x] Recovery checksum mismatch assessed.
- [x] Recovery duplicate/conflict abuse assessed.
- [x] Recovery silent-repair attempts assessed.
- [x] Replay repair coercion assessed.
- [x] Recovery promotion coercion assessed.
- [x] Action execution coercion assessed.
- [x] Deployment automation coercion assessed.
- [x] Installer/update/signing/publishing coercion assessed.
- [x] Unsafe storage path declarations assessed.
- [x] Path traversal-shaped declarations assessed.
- [x] Symlink/path substitution risks assessed.
- [x] Local file replacement risks assessed.
- [x] Rollback/replay attack risks assessed.
- [x] Governance evidence spoofing assessed.
- [x] Policy version spoofing assessed.
- [x] Readiness approval injection assessed.
- [x] Production Candidate approval injection assessed.
- [x] Release-candidate approval injection assessed.
- [x] Public-use/human-use approval injection assessed.
- [x] UI event/action escalation assessed.
- [x] Browser storage/network misuse assessed.
- [x] Script validation bypass assessed.
- [x] Generated artifact drift assessed.
- [x] CI/local validation divergence assessed.

## Existing-mitigation checklist
- [x] Rust authority boundary documented.
- [x] Typed validation documented.
- [x] Fail-closed parsing documented.
- [x] Deterministic reason codes documented.
- [x] Adversarial tests documented.
- [x] UI non-authority posture documented.
- [x] Local-only transport posture documented.
- [x] Provider-output untrusted posture documented.
- [x] Timeout/resource enforcement documented.
- [x] Retry/limit-escalation rejection documented.
- [x] Narrow decision-evidence append documented.
- [x] Recovery classification-only behavior documented.
- [x] Deployment configuration contract-only posture documented.
- [x] Policy/governance attribution-only posture documented.
- [x] Boundary lint scripts documented.
- [x] Structure/docs validation documented.
- [x] Clean-worktree validation posture documented.
- [x] Changelog archive validation posture documented.

## Residual-risk checklist
- [x] Local filesystem tampering risk classified.
- [x] Malicious local operator risk classified.
- [x] Compromised local account risk classified.
- [x] Symlink/path traversal and filesystem substitution risk classified.
- [x] Rollback/replay attack risk classified.
- [x] Manually edited persisted evidence risk classified.
- [x] Environment-specific permissions risk classified.
- [x] Unpinned toolchain/runtime assumptions risk classified.
- [x] Generated Cargo target drift risk classified.
- [x] Validation script blind spots risk classified.
- [x] UI usability misunderstanding of non-authority state risk classified.
- [x] Future provider nondeterminism risk classified.
- [x] Future remote/cloud execution risk classified.
- [x] Future deployment packaging risk classified.
- [x] Future release artifacts risk classified.
- [x] Future installer/update channels risk classified.
- [x] Future signing/publishing risk classified.
- [x] Future human-trial operations risk classified.
- [x] Incident response/rollback procedures risk classified.
- [x] Operator training gaps risk classified.
- [x] Observability gaps risk classified.

## Severity classification checklist
- [x] Each confirmed residual risk has one severity from `critical`, `high`, `medium`, `low`, or `informational`.
- [x] Critical residual risks recorded for compromised local account and future remote/cloud or installer/update surfaces.
- [x] High residual risks recorded for local tampering, malicious operator, path substitution, rollback/replay, persisted evidence tampering, future providers, packaging, release artifacts, signing/publishing, and human-trial operations.
- [x] Medium and low residual risks recorded where current local audit evidence limits impact.

## Risk disposition checklist
- [x] Each confirmed residual risk has one disposition from the required model.
- [x] Next bounded Phase 116 acceptance is limited to generated target drift and audit-gated local deployment boundary work.
- [x] Release-candidate evidence follow-ups are identified.
- [x] Production Candidate reassessment follow-ups are identified.
- [x] Human-use follow-ups are identified.
- [x] Future expansion risks are deferred to named later phases.

## Phase 114 relationship checklist
- [x] Phase 114 policy/governance evidence is used as attribution evidence only.
- [x] Governance evidence does not create governance authority.
- [x] Policy version evidence does not create approval authority.
- [x] Phase 115 does not rewrite governance or policy authority.

## Phase 116 gate checklist
- [x] Phase 116 may begin only as planned local deployment candidate boundary work.
- [x] Phase 116 is not implemented by Phase 115.
- [x] Phase 116 gate does not approve readiness, release, deployment, production, public usability, or production human use.
- [x] Phase 116 must inherit Phase 115 residual risks unless corrected by committed evidence.

## Phase 118 deferral checklist
- [x] Release-candidate evidence assembly remains deferred to Phase 118.
- [x] No release artifacts are created.
- [x] Release-candidate readiness is not approved.
- [x] Release-candidate evidence follow-ups are recorded.

## Phase 119 deferral checklist
- [x] Production Candidate reassessment remains deferred to Phase 119.
- [x] Production Candidate status is not approved.
- [x] Production readiness is not approved.
- [x] Phase 119 must reassess high and critical residual risks against committed evidence.

## Phase 120-or-later deferral checklist
- [x] Controlled human-use gate work remains deferred to Phase 120 or later.
- [x] Public/general use is not approved.
- [x] Public usability is not approved.
- [x] Production human use is not approved.
- [x] Human-use follow-ups are recorded.

## Production Candidate status checklist
- [x] Production Candidate status is not approved.
- [x] Security audit findings do not grant Production Candidate approval.
- [x] Passing validation does not grant Production Candidate approval.

## Release-candidate/public-use status checklist
- [x] Release-candidate readiness is not approved.
- [x] Public/general use is not approved.
- [x] Production human use is not approved.
- [x] Phase 115 creates no release artifacts.

## Roadmap/changelog truth checklist
- [x] Roadmap remains planned truth.
- [x] Future phase descriptions do not count as implemented behavior.
- [x] CHANGELOG surfaces remain historical truth.
- [x] Operations report remains advisory orientation truth.
- [x] This checklist remains procedural truth.

## Validation checklist
- [x] `CARGO_TARGET_DIR=/tmp/ajentic-phase-115-target ./scripts/check.sh`.
- [x] `git diff --check`.
- [x] `git status --short`.
- [x] Governance/architecture scan completed.
- [x] Security boundary scan completed.
- [x] Filesystem/process/network scan completed.
- [x] No-authority scan completed.
- [x] Readiness scan completed.
- [x] Source guard completed.

## Findings table
| Finding | Status | Evidence |
| --- | --- | --- |
| Phase 115 is security audit only. | Confirmed | Roadmap title/scope and Phase 115 operations report. |
| Security audit evidence is risk evidence, not approval authority. | Confirmed | Phase 115 operations report and checklist boundary rules. |
| Rust remains the authority boundary. | Confirmed | Governance, architecture, Rust API inspection, and boundary lint posture. |
| UI remains non-authoritative. | Confirmed | UI code and UI behavior tests inspected as evidence. |
| Known transport/provider/persistence/recovery approval escalation payloads are rejected by committed evidence. | Confirmed | Rust tests, UI tests, operations docs, and scans. |
| Local filesystem/operator compromise remains residual. | Confirmed residual | Phase 115 residual-risk inventory. |
| Release/deployment/human-use approval remains deferred. | Confirmed | Roadmap and Phase 115 deferral sections. |

## Residual risks table
| Risk | Severity | Disposition | Status |
| --- | --- | --- | --- |
| Local filesystem tampering | `high` | `requires_followup_before_human_use` | `residual_risk_documented` |
| Malicious local operator | `high` | `requires_followup_before_human_use` | `residual_risk_documented` |
| Compromised local account | `critical` | `requires_followup_before_human_use` | `residual_risk_documented` |
| Symlink/path traversal and filesystem substitution | `high` | `requires_followup_before_human_use` | `residual_risk_documented` |
| Rollback/replay attacks | `high` | `requires_followup_before_release_candidate_evidence` | `residual_risk_documented` |
| Manually edited persisted evidence | `high` | `requires_followup_before_human_use` | `residual_risk_documented` |
| Environment-specific permissions | `medium` | `requires_followup_before_human_use` | `residual_risk_documented` |
| Unpinned toolchain/runtime assumptions | `medium` | `requires_followup_before_release_candidate_evidence` | `residual_risk_documented` |
| Generated Cargo target drift | `low` | `accept_for_next_bounded_phase` | `partially_mitigated` |
| Validation script blind spots | `medium` | `requires_followup_before_release_candidate_evidence` | `partially_mitigated` |
| UI non-authority misunderstanding | `medium` | `requires_followup_before_human_use` | `residual_risk_documented` |
| Future provider nondeterminism | `high` | `deferred_to_named_phase` | `deferred_to_later_phase` |
| Future remote/cloud execution | `critical` | `deferred_to_named_phase` | `deferred_to_later_phase` |
| Future deployment packaging | `high` | `deferred_to_named_phase` | `deferred_to_later_phase` |
| Future release artifacts | `high` | `requires_followup_before_release_candidate_evidence` | `deferred_to_later_phase` |
| Future human-trial operations | `high` | `requires_followup_before_human_use` | `deferred_to_later_phase` |

## Required corrections table
| Correction | Status | Notes |
| --- | --- | --- |
| Roadmap planned-truth correction | Not required | Phase 115 title and scope are consistent. |
| Governance correction | Not required | Governance docs not modified. |
| Architecture correction | Not required | Architecture docs not modified. |
| Runtime/source correction | Not required | Phase 115 is audit only. |
| Validation-script compatibility correction | Required and completed | Narrow Rust and UI boundary lint compatibility fixes only; no runtime behavior affected. |

## Deferred items table
| Item | Deferral |
| --- | --- |
| Local deployment candidate boundary | Phase 116 only. |
| Operator documentation and human-trial dry run | Phase 117 only. |
| Release-candidate evidence assembly | Phase 118 only. |
| Production Candidate reassessment | Phase 119 only. |
| Controlled human-use gate | Phase 120 or later only. |
| Provider/remote/cloud expansion | Later named phase only. |
| Installer/update/signing/publishing/public release/production deployment | Later named phase only; not approved. |

## Validation log table
| Command | Status | Notes |
| --- | --- | --- |
| `git status --short` | Passed | Initial status clean before edits; final status showed only Phase 115 allowed surfaces before staging/commit. |
| `CARGO_TARGET_DIR=/tmp/ajentic-phase-115-target ./scripts/check.sh` | Passed after narrow validation-script compatibility fix | First clean-worktree runs exposed Rust boundary lint false positives and a UI Promise microtask lint compatibility issue; final run passed. |
| `git diff --check` | Passed | No whitespace errors. |
| Governance/architecture scan | Passed | Boundary/truth terminology reviewed. |
| Security boundary scan | Passed | Mentions reviewed as boundary, prohibition, test, history, planned, or security-risk context. |
| Filesystem/process/network scan | Passed | Current mentions reviewed as evidence; no Phase 115 behavior added. |
| No-authority scan | Passed | No active unauthorized authority claims introduced. |
| Readiness scan | Passed | No readiness approval claims introduced. |
| Source guard | Passed | No source/test/schema/script/workflow/README/AGENTS/archive/governance/architecture drift. |

## Zero-drift checklist
- [x] No Rust source modified.
- [x] No TypeScript source modified.
- [x] No tests modified.
- [x] No schemas modified.
- [x] No operator/runtime scripts modified.
- [x] `scripts/rust_boundary_lint.mjs` and `scripts/lint_ui_boundaries.mjs` received narrow validation compatibility corrections after full checks exposed lint compatibility issues.
- [x] No workflows modified.
- [x] No governance docs modified.
- [x] No architecture docs modified.
- [x] No README modified.
- [x] No AGENTS modified.
- [x] No archived changelog files modified.
- [x] No package or lockfile modified.
- [x] No deployment or release infrastructure modified.
- [x] Generated artifacts cleaned/not introduced.
