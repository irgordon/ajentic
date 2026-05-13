---
truth_dimension: historical
authority_level: authoritative
mutation_path: changelog_entry
---

# CHANGELOG

Historical truth surface for the active development range.

Archived historical ranges:
- docs/changelog/CHANGELOG-0001-0055.md (v0.0.1 through v0.0.55; includes legacy v0.0.0 bootstrap entry to prevent historical omission)
- docs/changelog/CHANGELOG-0056-0104.md (v0.0.56 through v0.0.104)
- docs/changelog/CHANGELOG-0104-0115.md (v0.0.104.5 through v0.0.115; boundary adjusted for the completed v0.0.104.5 maintenance entry)
- docs/changelog/CHANGELOG-0116-0125.md (v0.0.116 through v0.0.125)
- docs/changelog/CHANGELOG-0126-0134.md (v0.0.126 through v0.0.134)

Archive guarantees:
- Historical entries are partitioned without changing their recorded wording, timestamps, ordering within each deterministic archive extraction, headings, or semantic interpretation.
- Archived entries are not duplicated in this active changelog.
- The active changelog begins with v0.0.135 and retains current entries only.
- CHANGELOG surfaces remain historical truth.
- Changelog archiving is historical maintenance, not historical rewriting.
- Archive movement must preserve historical entry content.
- Archive ranges must remain contiguous and version-bounded.
- Active CHANGELOG.md remains the current historical surface.
- Archived changelog files preserve completed historical truth.
- Phase 135.2 does not change roadmap planned truth except narrow archive-reference clarification if required.
- Phase 135.2 does not approve readiness, Release Candidate status, Production Candidate status, or public/general use.

## v0.0.170.8 - 2026-05-13
**Status:** Phase 170.8 - Out-of-Band Local Operator Shell Module Extraction

### Changed
- Extract a coherent production-code family from `core/src/api/local_operator_shell.rs` into smaller sibling Rust module files.
- Preserve public behavior, validation semantics, serialized formats, deterministic ordering, reason strings, and test coverage.
- Update checklists/current-phase.md to Phase 170.8 procedural truth.

### Notes
- Out-of-band maintenance/code-quality phase.
- Behavior-preserving extraction only.
- Production-code extraction required; test extraction alone is not sufficient.
- No Phase 171 implementation.
- No release-candidate preparation behavior.
- No runtime behavior changes.
- No TypeScript changes.
- No schema changes.
- No release artifact creation.
- No provider execution expansion.
- No persistence authority expansion.
- No replay repair.
- No recovery promotion.
- No action execution.
- No installer, update-channel, signing, publishing, deployment, public-use, or readiness approval behavior.
- Phase 171 remains the next planned code-production phase after this extraction.

## v0.0.170.7 - 2026-05-13
**Status:** Phase 170.7 - Out-of-Band Rust Module Extraction Pass

### Changed
- Split selected oversized Rust surfaces into smaller behavior-preserving modules.
- Extract `core/src/api/local_operator_shell.rs` module-local tests into `core/src/api/local_operator_shell_tests.rs` while preserving the local shell public API through the existing `core/src/api/local_operator_shell.rs` surface.
- Preserve public behavior, validation semantics, serialized formats, deterministic ordering, and test coverage.
- Update checklists/current-phase.md to Phase 170.7 procedural truth.

### Notes
- Out-of-band maintenance/code-quality phase.
- Behavior-preserving extraction only.
- No Phase 171 implementation.
- No release-candidate preparation behavior.
- No runtime behavior changes.
- No TypeScript changes.
- No schema changes.
- No release artifact creation.
- No provider execution expansion.
- No persistence authority expansion.
- No replay repair.
- No recovery promotion.
- No action execution.
- No installer, update-channel, signing, publishing, deployment, public-use, or readiness approval behavior.
- Phase 171 remains the next planned code-production phase after this extraction.

## v0.0.170.5 - 2026-05-13
**Status:** Phase 170.5 - Out-of-Band Rust Surface Maintainability Audit

### Added
- Add the Phase 170.5 Rust maintainability audit identifying oversized Rust files, monolithic functions, deep nesting, transport/projection/validation accumulation, stale surfaces, repeated patterns, and extraction candidates before Phase 171.

### Changed
- Update checklists/current-phase.md to Phase 170.5 procedural truth.

### Notes
- Out-of-band maintenance audit only.
- No Rust source changes.
- No TypeScript source changes.
- No test changes.
- No schema changes.
- No runtime behavior.
- No refactor implementation.
- No Phase 171 implementation.
- No release-candidate preparation behavior.
- No release artifact creation.
- No deployment behavior.
- No provider execution expansion.
- No persistence implementation.
- No installer, update-channel, signing, publishing, public-use, or readiness approval behavior.
- Phase 171 remains the next planned code-production phase after this audit.

## v0.0.170 - 2026-05-13
**Status:** Phase 170 - Production-Path Alignment Checkpoint

### Changed
- Reconcile the Phase 166-169 controlled-trial/local-beta block.
- Confirm the current local beta path includes controlled internal trial execution harness, trial observability/error reporting, trial evidence review surface, local beta hardening, and user-facing local HTML help.
- Verify local HTML help pages and visible UI help entry point exist as part of local beta readiness for the next block.
- Decide whether Phase 171 may proceed toward release-candidate preparation work.
- Remap Phase 171-180 toward release-candidate preparation contract, dry package assembly, checksum/provenance evidence, installer/distribution contract, signing/key-custody dry run, release-candidate evidence assembly UI, gap hardening, dry-run rehearsal, and a Release Candidate decision gate.
- Preserve the post-Phase-170 rule that non-0/5 phases must produce product-facing code, executable artifacts, release-candidate preparation artifacts, or deterministic validation improvements.
- Update checklists/current-phase.md to Phase 170 procedural truth.

### Notes
- Alignment checkpoint only.
- No Rust source changes.
- No TypeScript source changes.
- No test changes.
- No schema changes.
- No runtime behavior.
- No provider execution expansion.
- No persistence implementation.
- No trial execution behavior.
- No release artifact creation.
- No deployment behavior.
- No installer, update-channel, signing, publishing, public-use, or readiness approval behavior.
- User help is explanatory only and not authority.
- Local beta workflow completion is not production readiness.
- Release-candidate preparation is not release readiness.
- Phase 171 is expected to resume code production.

## v0.0.169 - 2026-05-13
**Status:** Phase 169 - Local Beta Hardening Pass and User Help Surface

### Added
- Add local HTML operator help pages for getting started, local workflow, provider setup, validation/review/candidates, trial package/evidence, restore/verification, errors/stop conditions, glossary, and safety boundaries.
- Add a visible UI help entry point for local operator help.
- Add deterministic validation for required help pages and required help wording.
- Add TypeScript behavior tests for visible help entry point and no-authority help wording.

### Changed
- Improve local beta usability wording by linking the shell to explanatory local help and by stating that the help entry point does not mutate state, run providers, write packages, use network, publish, deploy, sign, release, or approve actions.
- Update checklists/current-phase.md to Phase 169 procedural truth.

### Notes
- Code-production phase.
- Product-facing local help/documentation phase.
- Help pages are explanatory only.
- Help pages do not grant authority.
- Help pages do not approve controlled human use.
- Help pages do not approve readiness, release, deployment, public use, or production use.
- Help pages do not trust provider output.
- Help pages do not authorize actions.
- No trial execution expansion.
- No provider execution expansion.
- No production monitoring.
- No remote telemetry.
- No automated remediation.
- No automated escalation.
- No stop-condition automation.
- No replay repair.
- No recovery promotion.
- No production persistence.
- No release artifact creation.
- No installer, update-channel, signing, publishing, deployment, public-use, or readiness approval behavior.
- Phase 170 remains the next production-path alignment checkpoint and must verify user-facing help exists.

## v0.0.168 - 2026-05-13
**Status:** Phase 168 - Trial Evidence Review Surface

### Added
- Add Rust-owned trial evidence review projection derived from local shell state.
- Add closed review finding categories, severity, disposition, source linkage, unresolved blocker summary, and local beta hardening candidate projection.
- Add UI rendering for trial evidence review, review findings, unresolved blockers, source evidence linkage, and hardening candidates.
- Add Rust and TypeScript tests for evidence review projection, finding derivation, hardening candidate derivation, deterministic ordering, and non-authority boundaries.

### Changed
- Extend the local operator shell projection with trial evidence review state.
- Update the local UI shell to display local trial evidence review findings and hardening candidates.
- Update checklists/current-phase.md to Phase 168 procedural truth.

### Notes
- Code-production phase.
- Trial evidence review is local-only and non-public.
- Review findings are evidence, not approval.
- Hardening candidates are inputs for Phase 169 code work, not approvals.
- Review does not approve controlled human use.
- Review does not approve readiness, release, deployment, public use, or production use.
- Review does not automate remediation.
- Review does not automate escalation.
- Review does not enforce stop conditions.
- Review does not repair replay.
- Review does not promote recovery.
- No provider trust approval.
- No action execution.
- No production monitoring.
- No remote telemetry.
- No release artifact creation.
- No installer, update-channel, signing, publishing, deployment, public-use, or readiness approval behavior.
- Phase 169 remains the next code-production phase for local beta hardening, including user-facing help documentation if still missing.

## v0.0.167 - 2026-05-13
**Status:** Phase 167 - Trial Observability and Error Reporting

### Added
- Add Rust-owned trial observability and trial error report projections derived from local shell state.
- Add closed trial error categories, severity classification, source linkage, blocked-state reporting, stop-condition observation reporting, replay/restore mismatch reporting, package/evidence read-back failure reporting, and local evidence linkage.
- Add UI rendering for trial observability status, trial error reporting, blocked-state summary, mismatch/error drilldown, and local-only/no-telemetry boundaries.
- Add Rust and TypeScript tests for happy-path observability, blocked/rejected states, stop-condition observation, verification mismatch, package/evidence failures, deterministic error reports, and non-authority boundaries.

### Changed
- Extend the local operator shell projection with trial observability and error reporting state.
- Update the local UI shell to display local trial observability and error reporting without production monitoring or telemetry.
- Update checklists/current-phase.md to Phase 167 procedural truth.

### Notes
- Code-production phase.
- Trial observability is local-only and non-public.
- Error reporting is local and descriptive only.
- No production monitoring.
- No remote telemetry.
- No network reporting.
- No background service.
- No automated remediation.
- No automated escalation.
- No stop-condition automation.
- No action execution.
- No replay repair.
- No recovery promotion.
- No provider trust approval.
- No controlled-human-use approval.
- No readiness, release, deployment, public-use, or production-use approval.
- No release artifact creation.
- No installer, update-channel, signing, publishing, deployment, public-use, or readiness approval behavior.
- Phase 168 remains the next code-production phase for trial evidence review surface.

## v0.0.166 - 2026-05-13
**Status:** Phase 166 - Controlled Internal Trial Execution Harness

### Added
- Add Rust-owned controlled internal trial execution harness projection, trial-run request, lifecycle steps, stop-condition observation, manual operator step status, evidence linkage, and boundary markers.
- Add deterministic bounded local trial-run projection gated by controlled internal trial package, runbook, failure drill, trial session evidence, replay/restore verification, and complete local workflow preconditions.
- Add fail-closed rejection for missing, blocked, rejected, authority-bearing, readiness-bearing, release-bearing, deployment-bearing, public-use-bearing, action-bearing, replay-repair-bearing, recovery-promotion-bearing, signing-bearing, or publishing-bearing trial execution requests.
- Add UI rendering for controlled trial execution harness status, trial-run lifecycle, stop-condition observation, manual operator step status, evidence linkage, blockers, and rejection reasons.
- Add Rust and TypeScript tests for valid harness execution, missing precondition rejection, stop-condition blocking, deterministic trial-run projection, no-authority boundaries, and visible harness behavior.

### Changed
- Extend the local operator shell projection with controlled internal trial execution harness status.
- Update the local UI shell to display and operate the bounded local trial execution harness.
- Update checklists/current-phase.md to Phase 166 procedural truth.

### Notes
- Code-production phase.
- Controlled internal trial execution harness is local-only and non-public.
- Harness execution does not approve controlled human use.
- Harness execution does not approve readiness, release, deployment, public use, or production use.
- Stop conditions are observed only; enforcement is not automated.
- Escalation is not automated.
- No action authorization is granted.
- No provider trust approval.
- No replay repair.
- No recovery promotion.
- No production persistence.
- No release artifact creation.
- No installer, update-channel, signing, publishing, deployment, public-use, or readiness approval behavior.
- Phase 167 remains the next code-production phase for trial observability and error reporting.

## v0.0.165 - 2026-05-13
**Status:** Phase 165 - Code-Production Alignment Checkpoint

### Changed
- Reconcile the Phase 161-164 controlled-internal-trial preparation block.
- Confirm the current controlled-trial preparation path includes controlled internal trial package, operator runbook/failure drill UI, trial session evidence capture, and trial replay/restore verification.
- Decide whether Phase 166 may proceed to a controlled internal trial execution harness.
- Confirm or adjust the Phase 166-170 block toward controlled internal trial execution, trial observability/error reporting, trial evidence review, local beta hardening, and the next production-path checkpoint.
- Preserve the post-Phase-160 rule that non-0/5 phases must produce product-facing code, executable artifacts, trial evidence artifacts, observability/error-reporting capability, or operator-visible workflow improvements.
- Update checklists/current-phase.md to Phase 165 procedural truth.

### Notes
- Alignment checkpoint only.
- No Rust source changes.
- No TypeScript source changes.
- No test changes.
- No schema changes.
- No runtime behavior.
- No provider execution expansion.
- No persistence implementation.
- No trial execution behavior.
- No release artifact creation.
- No deployment behavior.
- No installer, update-channel, signing, publishing, public-use, or readiness approval behavior.
- Controlled internal trial package is not trial approval.
- Trial operator runbook is guidance only.
- Failure drill is guidance only.
- Stop-condition drill is not automated enforcement.
- Escalation guidance does not activate authority.
- Trial session evidence is not readiness evidence.
- Trial session evidence is not release evidence.
- Trial session evidence is not deployment evidence.
- Replay/restore verification is not replay repair or recovery promotion.
- Verification passing is not approval.
- Absence of blockers is not approval.
- Phase 166 does not imply public/general use, production use, release readiness, Production Candidate status, controlled human-use approval, provider trust, or action execution.
- Phase 166 is expected to resume code production.

## v0.0.164 - 2026-05-13
**Status:** Phase 164 - Trial Replay and Restore Verification

### Added
- Add Rust-owned trial replay and restore verification projection, comparison summary, mismatch/error model, boundary markers, and deterministic verification result.
- Add package/evidence linkage verification, restore/history verification, and replay/status snapshot comparison.
- Add fail-closed rejection for missing, malformed, mismatched, authority-bearing, readiness-bearing, release-bearing, deployment-bearing, public-use-bearing, replay-repair-bearing, recovery-promotion-bearing, or action-bearing verification inputs.
- Add UI rendering for trial replay/restore verification status, package/evidence linkage, read-back state, replay/status comparison, restore/history comparison, and mismatch drilldown.
- Add Rust and TypeScript tests for valid verification, package/evidence mismatch rejection, malformed input rejection, replay/status mismatch rejection, restore/history mismatch rejection, deterministic results, and non-authority boundaries.

### Changed
- Extend the local operator shell projection with trial replay/restore verification status.
- Update the local UI shell to display trial replay and restore verification results.
- Update checklists/current-phase.md to Phase 164 procedural truth.

### Notes
- Code-production phase.
- Verification compares trial artifacts and restore/replay projections.
- Verification does not repair replay.
- Verification does not promote recovery.
- Verification does not approve controlled human use.
- Verification does not approve readiness, release, deployment, public use, or production use.
- Verification does not execute actions.
- No trial execution.
- No automated remediation.
- No provider trust approval.
- No production persistence.
- No release artifact creation.
- No installer, update-channel, signing, publishing, deployment, public-use, or readiness approval behavior.
- Phase 165 remains the next code-production alignment checkpoint.

## v0.0.163 - 2026-05-13
**Status:** Phase 163 - Trial Session Evidence Capture

### Added
- Add Rust-owned trial session evidence record derivation, validation, serialization, write, read, and read-back validation.
- Add deterministic trial session evidence metadata, evidence ID, classifications, trial package linkage, runbook/failure drill linkage, workflow snapshot, stop-condition snapshot, escalation snapshot, failure category snapshot, absence markers, and validation errors.
- Add explicit caller-provided path helpers for trial session evidence write/read.
- Add trial session evidence projection to the local shell/transport surface.
- Add UI rendering for trial session evidence status, included evidence, stop-condition snapshot, escalation summary, failure summary, and local-only/non-authoritative boundaries if implemented.
- Add Rust and TypeScript tests for deterministic evidence content, explicit write/read behavior, read-back validation, malformed evidence rejection, missing linkage rejection, and non-authority boundaries.

### Changed
- Extend the local operator shell projection with trial session evidence status.
- Update checklists/current-phase.md to Phase 163 procedural truth.

### Notes
- Code-production phase.
- Produces a real local trial session evidence artifact through explicit caller-provided path helpers.
- Trial session evidence is local-only, non-public, non-production, and non-authoritative.
- Trial session evidence is not release evidence.
- Trial session evidence is not deployment evidence.
- Trial session evidence is not readiness evidence.
- Trial session evidence does not start or approve a controlled trial.
- Trial session evidence does not approve public/general use.
- Trial session evidence does not approve production use.
- No default filesystem persistence.
- No automatic save.
- No background persistence.
- No production persistence.
- No release artifact creation.
- No installer, update-channel, signing, publishing, deployment, public-use, or readiness approval behavior.
- No provider trust approval.
- No candidate approval.
- No replay repair.
- No recovery promotion.
- No action execution.
- Phase 164 remains the next code-production phase for trial replay and restore verification.

## v0.0.162 - 2026-05-13
**Status:** Phase 162 - Trial Operator Runbook and Failure Drill UI

### Added
- Add Rust-owned trial operator runbook and failure drill projections derived from local shell state.
- Add runbook step/status, failure category, stop-condition drill, current blocker, and escalation guidance projections.
- Add UI rendering for trial operator runbook, failure drilldown, stop-condition drill, current blocker guidance, and escalation guidance.
- Add Rust and TypeScript tests for runbook projection, blocked/rejected states, stop-condition drill rendering, escalation guidance rendering, deterministic rendering, and non-authority boundaries.

### Changed
- Extend the local operator shell projection with trial runbook and failure drill status.
- Update the local UI shell to display controlled-trial operator guidance and failure drill surfaces.
- Update checklists/current-phase.md to Phase 162 procedural truth.

### Notes
- Code-production phase.
- Product-facing UI/runbook phase, not document-only.
- Trial operator runbook is local-only and non-public.
- Runbook does not start a controlled trial.
- Stop conditions are guidance only; enforcement is not automated.
- Escalation guidance is descriptive only and does not activate authority.
- No controlled human-use approval.
- No public/general-use approval.
- No production-use approval.
- No readiness approval.
- No release artifact creation.
- No deployment artifact creation.
- No installer, update-channel, signing, publishing, deployment, public-use, or readiness approval behavior.
- No provider trust approval.
- No action execution.
- No replay repair.
- No recovery promotion.
- Phase 163 remains the next code-production phase for trial session evidence capture.

## v0.0.161 - 2026-05-13
**Status:** Phase 161 - Controlled Internal Trial Package

### Added
- Add Rust-owned controlled internal trial package derivation, validation, serialization, write, read, and read-back validation.
- Add deterministic trial package metadata, package ID, classification, trial scope, named internal operator/trial participant metadata, stop-condition markers, included local beta evidence summary, absence markers, and validation errors.
- Add explicit caller-provided path helpers for controlled internal trial package write/read.
- Add controlled internal trial package projection to the local shell/transport surface.
- Add UI rendering for controlled internal trial package status, scope, included evidence, stop conditions, and local-only/non-public boundaries.
- Add Rust and TypeScript tests for deterministic package content, explicit write/read behavior, read-back validation, malformed package rejection, missing trial metadata rejection, and non-public/non-production boundaries.

### Changed
- Extend the local operator shell projection with controlled internal trial package status.
- Update checklists/current-phase.md to Phase 161 procedural truth.

### Notes
- Code-production phase.
- Produces a real local controlled internal trial package artifact through explicit caller-provided path helpers.
- Controlled internal trial package is local-only and non-public.
- Controlled internal trial package is not a release artifact.
- Controlled internal trial package is not deployment evidence.
- Controlled internal trial package is not readiness evidence.
- Controlled internal trial package does not approve controlled human use.
- Controlled internal trial package does not approve public/general use.
- Controlled internal trial package does not approve production use.
- No default filesystem persistence.
- No automatic save.
- No background persistence.
- No production persistence.
- No release artifact creation.
- No installer, update-channel, signing, publishing, deployment, public-use, or readiness approval behavior.
- No provider trust approval.
- No candidate approval.
- No replay repair.
- No recovery promotion.
- No action execution.
- Phase 162 remains the next code-production phase for trial operator runbook and failure drill UI.

## v0.0.160 - 2026-05-13
**Status:** Phase 160 - Production-Path Alignment Checkpoint

### Changed
- Reconcile the Phase 151-159 local beta product-capability block.
- Confirm the current local beta workflow includes session package, restore/history UI, adapter contract, controlled adapter dry run, constrained invocation, provider output pipeline integration, local candidate materialization, and complete local operator workflow.
- Remap Phase 161-170 toward controlled internal trial packaging, trial evidence capture, restore/replay verification, trial observability, evidence review, and local beta hardening.
- Preserve the post-Phase-150 rule that non-0/5 phases must produce product-facing code, executable artifacts, or operator-visible workflow improvements.
- Update checklists/current-phase.md to Phase 160 procedural truth.

### Notes
- Alignment checkpoint only.
- No Rust source changes.
- No TypeScript source changes.
- No test changes.
- No schema changes.
- No runtime behavior.
- No provider execution expansion.
- No persistence implementation.
- No release artifact creation.
- No deployment behavior.
- No installer, update-channel, signing, publishing, public-use, or readiness approval behavior.
- Phase 160 is not release readiness approval.
- Phase 160 is not deployment approval.
- Local beta workflow completion is not production readiness.
- Local candidate materialization is not production approval.
- Provider output remains untrusted unless a later explicit bounded phase changes that.
- Evidence export is not release evidence.
- Session package is not a release artifact.
- Restore projection is not recovery promotion.
- Replay/status projection is not replay repair.
- Operator decisions are local workflow decisions, not release/deployment/public-use approvals.
- Passing validation is not readiness approval.
- Absence of blockers is not approval.
- Phase 161 is expected to resume code production.

## v0.0.159 - 2026-05-12
**Status:** Phase 159 - Complete Local Operator Workflow

### Added
- Add Rust-owned complete local operator workflow projection, step classification, current blocker detection, evidence summary, and boundary markers.
- Add UI rendering for a complete local operator workflow panel with step status, blocked/rejected state, error drilldown, local candidate materialization status, replay/status summary, evidence export summary, session package status, and restore/history status.
- Add Rust and TypeScript tests for happy-path workflow projection, blocked/rejected workflow states, deterministic rendering, and no-authority boundaries.

### Changed
- Extend the local operator shell projection with complete local workflow status.
- Update the local UI shell to show the complete local workflow from provider setup through local candidate materialization.
- Update checklists/current-phase.md to Phase 159 procedural truth.

### Notes
- Code-production phase.
- Complete local workflow is local-only and non-production.
- Workflow completion does not approve readiness, release, deployment, public use, or production use.
- Workflow completion does not grant provider trust.
- Workflow completion does not authorize actions.
- No additional provider execution capability.
- No arbitrary command execution.
- No shell command execution.
- No network sockets.
- No secret handling.
- No production persistence.
- No replay repair.
- No recovery promotion.
- No action execution.
- No release artifact creation.
- No installer, update-channel, signing, publishing, deployment, public-use, or readiness approval behavior.
- Phase 160 remains the next production-path alignment checkpoint.

## v0.0.158 - 2026-05-12
**Status:** Phase 158 - Local Candidate Materialization

### Added
- Add Rust-owned local candidate materialization request, validation, output projection, capability surface, and boundary markers.
- Add deterministic local candidate output materialization from validated staged proposals with explicit operator decision precondition.
- Add fail-closed rejection for missing, rejected, drifted, trust-bearing, readiness-bearing, release-bearing, deployment-bearing, public-use-bearing, action-bearing, execution-bearing, persistence-bearing, signing-bearing, or publishing-bearing materialization requests.
- Add UI rendering for local candidate materialization status, candidate output projection, source linkage, and boundary markers.
- Add Rust and TypeScript tests for valid materialization, rejected preconditions, deterministic candidate identity/content, no-effect boundaries, and visible materialization behavior.

### Changed
- Extend the local operator shell projection with local candidate output/materialization state.
- Update the local UI shell to expose local candidate materialization only after required validation, review, staging, staged validation, candidate review, and operator decision preconditions.
- Update checklists/current-phase.md to Phase 158 procedural truth.

### Notes
- Code-production phase.
- Local candidate output only.
- Candidate output is non-production.
- Provider output remains untrusted.
- Candidate materialization does not approve readiness, release, deployment, public use, or production use.
- Candidate materialization does not authorize actions.
- No provider trust approval.
- No action execution.
- No replay repair.
- No recovery promotion.
- No production persistence.
- No release artifact creation.
- No installer, update-channel, signing, publishing, deployment, public-use, or readiness approval behavior.
- Phase 159 remains the next code-production phase for complete local operator workflow.

## v0.0.157.1 - 2026-05-12
**Status:** Phase 157.1 - Out-of-Band Rustfmt Maintenance Fix

### Changed
- Apply rustfmt to the Phase 157 local provider output pipeline implementation in `core/src/api/local_operator_shell.rs`.
- Update checklists/current-phase.md to Phase 157.1 procedural truth.

### Notes
- Out-of-band maintenance fix.
- Formatting-only Rust source update.
- No runtime behavior changes.
- No runtime Rust logic changes.
- No TypeScript changes.
- Inline Rust test assertion style update only to satisfy `cargo clippy -- -D warnings`.
- No schema changes.
- No roadmap changes.
- No provider execution expansion.
- No candidate materialization.
- No persistence expansion.
- No release artifact creation.
- No installer, update-channel, signing, publishing, deployment, public-use, or readiness approval behavior.
- Phase 158 remains the next planned code-production phase.

## v0.0.157 - 2026-05-12
**Status:** Phase 157 - Real Provider Output Pipeline Integration

### Added
- Add Rust-owned provider output pipeline integration projection for constrained local invocation output.
- Add invocation-to-provider-output projection linkage while preserving untrusted/descriptive, non-candidate, and non-promoted output boundaries.
- Add stage-by-stage pipeline status for provider output validation, provider output review, staged proposal creation, staged proposal validation, candidate review, and operator decision.
- Add fail-closed rejection for missing/rejected/drifted invocation output and boundary skip attempts.
- Add UI rendering for provider output pipeline integration status, next required stage, stage list, and blocked/rejected reasons.
- Add Rust and TypeScript tests for valid integration, blocked/rejected stages, stage ordering, no-skip boundaries, and no-effect behavior.

### Changed
- Extend the local operator shell projection with provider output pipeline integration state.
- Update the local UI shell to display constrained invocation output entering the existing provider-output pipeline.
- Update checklists/current-phase.md to Phase 157 procedural truth.

### Notes
- Code-production phase.
- Integrates constrained invocation output into the existing provider-output pipeline.
- Invocation output remains untrusted/descriptive.
- Pipeline integration does not create candidate output.
- Pipeline integration does not materialize candidates.
- Validation, review, staging, staged validation, candidate review, and operator decision boundaries cannot be skipped.
- No additional provider execution capability.
- No arbitrary command execution.
- No shell command execution.
- No network sockets.
- No secret handling.
- No provider trust approval.
- No candidate materialization.
- No production persistence.
- No replay repair.
- No recovery promotion.
- No action execution.
- No release artifact creation.
- No installer, update-channel, signing, publishing, deployment, public-use, or readiness approval behavior.
- Phase 158 remains the next code-production phase for local candidate materialization.

## v0.0.156 - 2026-05-12
**Status:** Phase 156 - Constrained Real Local Provider Invocation

### Added
- Add Rust-owned constrained local provider invocation request, result, projection, capability surface, and validation.
- Add exactly one allowlisted local provider invocation path.
- Add fail-closed rejection for non-allowlisted, cloud, network, shell, command, endpoint, secret-bearing, trust-bearing, readiness-bearing, release-bearing, deployment-bearing, public-use-bearing, candidate-materialization-bearing, action-bearing, or persistence-bearing invocation requests.
- Add UI rendering for constrained local provider invocation status, result, capability surface, and boundary markers.
- Add Rust and TypeScript tests for accepted allowlisted invocation, rejected unsafe invocation, deterministic projection, and no-effect boundaries.

### Changed
- Extend the local operator shell projection with constrained local provider invocation status.
- Update the local UI shell to expose one constrained local provider invocation path without arbitrary command execution.
- Update checklists/current-phase.md to Phase 156 procedural truth.

### Notes
- Code-production phase.
- Exactly one allowlisted local provider invocation path.
- No arbitrary provider execution.
- No arbitrary command string field.
- No shell command execution.
- No shell expansion.
- No cloud model calls.
- No network sockets.
- No secret handling.
- No provider trust approval.
- No candidate materialization.
- No candidate approval.
- No production persistence.
- No replay repair.
- No recovery promotion.
- No action execution.
- No release artifact creation.
- No installer, update-channel, signing, publishing, deployment, public-use, or readiness approval behavior.
- Phase 157 remains the next code-production phase for real provider output pipeline integration.

## v0.0.154 - 2026-05-12
**Status:** Phase 154 - Controlled Adapter Dry-Run Harness

### Added
- Add Rust-owned controlled adapter dry-run request, result, projection, capability surface, and validation.
- Add deterministic fake adapter dry-run execution through the Phase 153 adapter contract.
- Add fail-closed rejection for missing, rejected, unsupported, local-model, cloud, network, shell, filesystem, executable-path, secret-bearing, execution-bearing, trust-bearing, readiness-bearing, release-bearing, deployment-bearing, public-use-bearing, action-bearing, or persistence-bearing adapter dry-run requests.
- Add UI rendering for controlled adapter dry-run status, result, linkage, and boundary markers.
- Add Rust and TypeScript tests for deterministic dry-run execution, rejected dry-run preconditions, no-effect boundaries, and visible dry-run behavior.

### Changed
- Extend the local operator shell projection with controlled adapter dry-run status.
- Update the local UI shell to expose controlled adapter dry-run behavior for deterministic_fake_adapter only.
- Update checklists/current-phase.md to Phase 154 procedural truth.

### Notes
- Code-production phase.
- Controlled adapter dry run only.
- deterministic_fake_adapter is the only executable adapter in Phase 154.
- No real model execution.
- No provider execution expansion beyond deterministic fake adapter dry run.
- No arbitrary local model execution.
- No cloud model calls.
- No shell command execution.
- No local binary invocation.
- No process spawning.
- No network sockets.
- No filesystem persistence.
- No environment variable reads.
- No secret reads.
- No provider trust approval.
- No candidate materialization.
- No candidate approval.
- No production persistence.
- No replay repair.
- No recovery promotion.
- No action execution.
- No release artifact creation.
- No installer, update-channel, signing, publishing, deployment, public-use, or readiness approval behavior.
- Phase 155 remains the next code-production alignment checkpoint.

## v0.0.153 - 2026-05-12
**Status:** Phase 153 - Real Local Provider Adapter Contract

### Added
- Add Rust-owned local provider adapter contract, registry projection, capability surface, and validation.
- Add non-executing adapter declaration support for deterministic fake/local adapter contract surfaces.
- Add fail-closed rejection for unsupported, cloud, network, shell, filesystem, executable-path, secret-bearing, execution-bearing, trust-bearing, readiness-bearing, release-bearing, deployment-bearing, public-use-bearing, signing-bearing, or publishing-bearing adapter declarations.
- Add UI rendering for adapter registry/configuration, validation status, and capability surface.
- Add Rust and TypeScript tests for adapter declaration validation, rejected unsafe declarations, deterministic registry projection, and no-execution boundaries.

### Changed
- Extend the local operator shell projection with local provider adapter registry/configuration state.
- Update the local UI shell to display adapter contract and registry state.
- Update checklists/current-phase.md to Phase 153 procedural truth.

### Notes
- Code-production phase.
- Adapter contract only.
- No model execution.
- No provider execution expansion.
- No shell command execution.
- No local binary invocation.
- No network sockets.
- No cloud calls.
- No secret handling.
- No provider trust approval.
- No production persistence.
- No replay repair.
- No recovery promotion.
- No action execution.
- No release artifact creation.
- No installer, update-channel, signing, publishing, deployment, public-use, or readiness approval behavior.
- Phase 154 remains the next code-production phase for controlled adapter dry-run harness.

## v0.0.152 - 2026-05-12
**Status:** Phase 152 - Session History and Restore UI

### Added
- Add Rust-owned local session history and restore projections for local session packages.
- Add restore candidate/read-back validation and restore preview projection for explicit local packages.
- Add UI rendering for session history, selected package details, restore status, read-back validation, and restore preview.
- Add Rust and TypeScript tests for restore projection, malformed package rejection, deterministic rendering, and local-only/non-production boundaries.

### Changed
- Extend the local operator shell projection with session history and restore status.
- Update the local UI shell to display local session restore and read-back validation state.
- Update checklists/current-phase.md to Phase 152 procedural truth.

### Notes
- Code-production phase.
- Local session restore UI only.
- Restore is local-only and non-production.
- Read-back validation is not restore authority.
- Restore preview does not repair replay.
- Restore preview does not promote recovery.
- Restore projection does not imply readiness, release, deployment, or public use.
- No automatic filesystem scanning.
- No remote sync.
- No background restore.
- No provider execution.
- No command execution.
- No network sockets.
- No production persistence.
- No release artifact creation.
- No installer, update-channel, signing, publishing, deployment, public-use, or readiness approval behavior.
- No replay repair.
- No recovery promotion.
- No action execution.

## v0.0.151 - 2026-05-12
**Status:** Phase 151 - Persistent Local Session Package

### Added
- Add Rust-owned local session package derivation, validation, serialization, write, read, and read-back validation.
- Add deterministic local session package metadata, classification, payload, absence markers, and validation errors.
- Add explicit caller-provided path helpers for local session package write/read.
- Add local session package projection to the local shell/transport surface.
- Add UI rendering for local session package status and validation.
- Add Rust and TypeScript tests for deterministic package content, explicit write/read behavior, read-back validation, malformed package rejection, and local-only/non-production boundaries.

### Changed
- Extend the local operator shell projection with local session package status.
- Update checklists/current-phase.md to Phase 151 procedural truth.

### Notes
- Code-production phase.
- Produces a real local session package artifact through explicit caller-provided path helpers.
- Local session package is local-only and non-production.
- Local session package is not a release artifact.
- Local session package is not deployment evidence.
- Local session package is not readiness evidence.
- No default filesystem persistence.
- No automatic save.
- No background persistence.
- No production persistence.
- No release artifact creation.
- No installer, update-channel, signing, publishing, deployment, public-use, or readiness approval behavior.
- No provider trust approval.
- No candidate approval.
- No replay repair.
- No recovery promotion.
- No action execution.

## v0.0.150 - 2026-05-12
**Status:** Phase 150 - Aggressive Code-Production Roadmap Remap

### Added
- Add the Phase 150 code-production remap document for Phases 151-160.

### Changed
- Remap Phases 151-160 into larger product-capability phases toward a usable local beta.
- Collapse overly granular safety-only sequencing into implementation phases with embedded safety checks.
- Update roadmap sequencing to prioritize visible UI capability, executable Rust capability, persisted local artifacts, restore/replay/export capability, real adapter integration, and end-to-end operator workflow improvements.
- Update checklists/current-phase.md to Phase 150 procedural truth.

### Notes
- Alignment checkpoint only.
- No Rust source changes.
- No TypeScript source changes.
- No test changes.
- No schema changes.
- No runtime behavior.
- No provider execution expansion.
- No persistence implementation.
- No candidate materialization implementation.
- No release artifact creation.
- No installer, update-channel, signing, publishing, deployment, public-use, or readiness approval behavior.
- Phase 151 is expected to resume code production.

## v0.0.149 - 2026-05-12
**Status:** Phase 149 - Operator Candidate Decision Boundary and Phase 150 Handoff Projection

### Added
- Add Rust-owned operator decision boundary for validated staged candidate-conversion proposals.
- Add approve/reject decision records scoped only to validated staged proposals.
- Add deterministic operator decision projection with explicit no candidate materialization, no provider-output trust, and no readiness/release/deployment effects.
- Add Phase150CodeProductionHandoff projection generated from executable local shell state.
- Add UI rendering for validated staged proposal decisions and Phase 150 code-production handoff.
- Add Rust and TypeScript tests for operator decision recording, rejected preconditions, no-effect boundaries, and handoff projection.

### Changed
- Extend the local operator shell transport projection with operator candidate decision and Phase 150 handoff data.
- Update the local UI shell to allow approve/reject decisions only for validated staged proposals without creating candidate output.
- Update checklists/current-phase.md to Phase 149 procedural truth.

### Notes
- Code-production phase.
- Operator decision applies only to validated staged proposals.
- No candidate output is created.
- Candidate materialization is not performed.
- Provider output remains untrusted.
- Provider output is not approved.
- Operator decision does not imply readiness, release, deployment, or public-use approval.
- Phase 150 handoff is generated from executable state.
- Phase 149 does not edit roadmap files.
- No provider-output trust approval.
- No candidate materialization.
- No durable decision storage.
- No durable ledger writes.
- No replay repair.
- No recovery promotion.
- No action execution.
- No additional provider execution capability.
- No arbitrary local model execution.
- No cloud model calls.
- No shell command execution.
- No local binary invocation.
- No network sockets.
- No filesystem persistence.
- No production persistence.
- No release artifact creation.
- No installer, update-channel, signing, publishing, deployment, public-use, or readiness approval behavior.

## v0.0.148 - 2026-05-12
**Status:** Phase 148 - Candidate Review Surface

### Added
- Add a visible candidate review surface for validated staged candidate-conversion proposals.
- Add UI rendering for proposal linkage, staged validation status, validation reasons, materialization boundary, operator-decision boundary, trust/approval status, and no-effect summary.
- Add explicit UI boundary text that validated staged proposals are not candidate output, candidate materialization was not performed, and operator decision is unavailable in Phase 148.
- Add TypeScript behavior tests for candidate review rendering, unavailable/rejected states, no-authority UI copy, and display-only non-mutation boundaries.

### Changed
- Refine the local UI shell to make validated staged proposals easier to inspect without adding candidate materialization or approval controls.
- Update checklists/current-phase.md to Phase 148 procedural truth.

### Notes
- Code-production phase.
- Candidate review surface is display-only.
- Validated staged proposal is not candidate output.
- Candidate materialization is not performed.
- Operator decision is not available in Phase 148.
- Provider output remains untrusted.
- Staged proposal remains not approved.
- No candidate conversion.
- No candidate replacement.
- No candidate materialization.
- No candidate approval.
- No operator candidate decision.
- No decision ledger mutation.
- No replay mutation.
- No export promotion.
- No provider configuration mutation.
- No provider execution trigger.
- No additional provider execution capability.
- No arbitrary local model execution.
- No cloud model calls.
- No shell command execution.
- No local binary invocation.
- No network sockets.
- No filesystem persistence.
- No durable proposal storage.
- No durable validation storage.
- No durable review storage.
- No durable ledger writes.
- No provider trust approval.
- No production persistence.
- No replay repair.
- No recovery promotion.
- No action execution.
- No release artifact creation.
- No installer, update-channel, signing, publishing, deployment, public-use, or readiness approval behavior.
- Phase 149 remains the next code-production phase for operator candidate decision boundary and Phase 150 handoff evidence.

## v0.0.147 - 2026-05-12
**Status:** Phase 147 - Candidate Conversion Validation

### Added
- Add Rust-owned staged candidate-conversion validation projection and request handling.
- Add staged proposal shape, source eligibility, deterministic linkage, no-effect, boundary, and future-phase marker validation.
- Add fail-closed rejection for missing, inconsistent, drifted, claim-bearing, or malformed staged proposals.
- Add UI rendering for staged proposal validation status, reasons, linkage, materialization boundary, future review boundary, operator-decision unavailability, and no-effect summary.
- Add Rust and TypeScript tests for valid staged proposal validation, rejected validation, deterministic linkage, non-candidate boundaries, and no-effect guarantees.

### Changed
- Extend the local operator shell transport projection with staged proposal validation data.
- Update the local UI shell to allow validating staged candidate-conversion proposals without creating candidate output.
- Update checklists/current-phase.md to Phase 147 procedural truth.

### Notes
- Code-production phase.
- Validation checks staged proposal shape and source linkage only.
- Validated staged proposal is not candidate output.
- Candidate materialization is not performed.
- Candidate materialization is not available in Phase 147.
- Operator decision is not available in Phase 147.
- Provider output remains untrusted.
- Staged proposal remains not approved.
- No candidate conversion.
- No candidate replacement.
- No candidate approval.
- No operator candidate decision.
- No decision ledger mutation.
- No replay mutation.
- No export promotion.
- No provider configuration mutation.
- No provider execution trigger.
- No additional provider execution capability.
- No arbitrary local model execution.
- No cloud model calls.
- No shell command execution.
- No local binary invocation.
- No network sockets.
- No filesystem persistence.
- No durable proposal storage.
- No durable validation storage.
- No durable ledger writes.
- No provider trust approval.
- No production persistence.
- No replay repair.
- No recovery promotion.
- No action execution.
- No release artifact creation.
- No installer, update-channel, signing, publishing, deployment, public-use, or readiness approval behavior.

## v0.0.146 - 2026-05-11
**Status:** Phase 146 - Candidate Conversion Staging Boundary

### Added
- Add Rust-owned staged candidate-conversion proposal types and projection.
- Add staged proposal creation from reviewable_untrusted provider output only.
- Add fail-closed rejection for rejected, not_validated, validation_not_applicable, invalid, missing, inconsistent, trust-bearing, approval-bearing, readiness-bearing, release-bearing, deployment-bearing, public-use-bearing, action-bearing, execution-bearing, persistence-bearing, or candidate-creation-bearing proposal requests.
- Add UI rendering for staged candidate-conversion proposals.
- Add Rust and TypeScript tests for staged proposal creation, source rejection, deterministic proposal identity, non-candidate boundaries, and no-effect guarantees.

### Changed
- Extend the local operator shell transport projection with staged candidate-conversion proposal data.
- Update the local UI shell to allow creating and inspecting staged candidate-conversion proposals without creating candidate output.
- Update checklists/current-phase.md to Phase 146 procedural truth.

### Notes
- Code-production phase.
- Staged proposal creation only.
- Staged proposal is not candidate output.
- Staged proposal is not approved output.
- Staged proposal is not trusted output.
- Provider output remains untrusted.
- Candidate conversion is not performed.
- Candidate approval is not available in Phase 146.
- Validation of staged proposals remains deferred to Phase 147.
- No candidate materialization.
- No candidate replacement.
- No operator candidate decision.
- No decision ledger mutation.
- No replay mutation.
- No export promotion.
- No provider configuration mutation.
- No provider execution trigger.
- No additional provider execution capability.
- No arbitrary local model execution.
- No cloud model calls.
- No shell command execution.
- No local binary invocation.
- No network sockets.
- No filesystem persistence.
- No durable proposal storage.
- No durable ledger writes.
- No provider trust approval.
- No production persistence.
- No replay repair.
- No recovery promotion.
- No action execution.
- No release artifact creation.
- No installer, update-channel, signing, publishing, deployment, public-use, or readiness approval behavior.

## v0.0.145 - 2026-05-11
**Status:** Phase 145 - Code-Production Alignment Checkpoint

### Changed
- Reconcile the Phase 141-144 code-production block.
- Confirm the current provider-output pipeline includes sandboxed deterministic provider execution, execution result projection, provider output validation/rejection, and provider output review UI.
- Remap Phase 146-150 as the next code-production block focused on staged candidate-conversion proposal work.
- Preserve the rule that every non-0/5 phase must produce usable, testable code or a concrete executable artifact.
- Update checklists/current-phase.md to Phase 145 procedural truth.

### Notes
- Alignment checkpoint only.
- No Rust source changes.
- No TypeScript source changes.
- No test changes.
- No schema changes.
- No candidate conversion.
- No candidate approval.
- No provider-output approval.
- No provider-output trust.
- No trust elevation.
- No readiness approval.
- No Release Candidate approval.
- No Production Candidate approval.
- No local model execution.
- No cloud model calls.
- No shell command execution.
- No network sockets.
- No production persistence.
- No release artifact creation.
- No installer, update-channel, signing, publishing, deployment, public-use, or readiness approval behavior.
- Phase 146 is expected to resume code production.

## v0.0.144 - 2026-05-11
**Status:** Phase 144 - Provider Output Review in UI

### Added
- Add a visible provider output review UI surface for provider execution and validation results.
- Add UI rendering for reviewability status, candidate-boundary status, validation reasons, absence markers, and no-effect summaries.
- Add explicit UI boundary text that reviewable_untrusted is inspection-only, not candidate material, and cannot be approved in Phase 144.
- Add explicit UI boundary text that absence markers are not safety or readiness.
- Add TypeScript behavior tests for provider output review UI, rejection reason display, absence marker rendering, and non-mutation boundaries.

### Changed
- Refine the local UI shell to make provider output validation results easier to inspect without adding candidate conversion or approval controls.
- Update checklists/current-phase.md to Phase 144 procedural truth.

### Notes
- Code-production phase.
- UI/provider-output review phase only.
- Provider output review is inspection-only.
- reviewable_untrusted is not candidate material.
- reviewable_untrusted is not approved output.
- reviewable_untrusted is not trusted output.
- reviewable_untrusted is not promoted output.
- Absence markers are not safety.
- Absence markers are not readiness.
- No candidate conversion.
- No candidate replacement.
- No approve/reject controls for provider output.
- No decision ledger mutation.
- No replay mutation.
- No export promotion.
- No provider configuration mutation from review UI.
- No additional provider execution capability.
- No arbitrary local model execution.
- No cloud model calls.
- No shell command execution.
- No local binary invocation.
- No network sockets.
- No filesystem persistence.
- No durable provider execution storage.
- No durable ledger writes.
- No provider trust approval.
- No production persistence.
- No replay repair.
- No recovery promotion.
- No action execution.
- No release artifact creation.
- No installer, update-channel, signing, publishing, deployment, public-use, or readiness approval behavior.
- Phase 145 remains the next code-production alignment checkpoint.

## v0.0.143 - 2026-05-11
**Status:** Phase 143 - Provider Output Validation and Rejection Flow

### Added
- Add Rust-owned provider output validation and rejection projection for sandboxed deterministic provider output.
- Add closed validation, reviewability, candidate-boundary, and rejection-reason statuses.
- Add validation behavior that classifies deterministic_stub output as reviewable_untrusted or rejected without converting it into candidate material.
- Add explicit no-effect boundaries for trust, candidate conversion, decision ledger, replay, export, action, readiness, release, and deployment effects.
- Add UI rendering for provider output validation status, reviewability, candidate-boundary status, and rejection reasons.
- Add Rust and TypeScript tests for reviewable-untrusted classification, rejection reasons, determinism, and non-promotion boundaries.

### Changed
- Extend the local operator shell transport projection with provider output validation data.
- Update the local UI shell to display provider output validation and rejection details.
- Update checklists/current-phase.md to Phase 143 procedural truth.

### Notes
- Code-production phase.
- Provider output validation is a reviewability filter only.
- reviewable_untrusted is not candidate material.
- reviewable_untrusted is not approved output.
- reviewable_untrusted is not trusted output.
- Provider output is not promoted.
- Provider output is not eligible for approve/reject in Phase 143.
- No candidate conversion.
- No candidate replacement.
- No additional provider execution capability.
- No arbitrary local model execution.
- No cloud model calls.
- No shell command execution.
- No local binary invocation.
- No network sockets.
- No filesystem persistence.
- No durable provider execution storage.
- No durable ledger writes.
- No provider trust approval.
- No production persistence.
- No replay repair.
- No recovery promotion.
- No action execution.
- No release artifact creation.
- No installer, update-channel, signing, publishing, deployment, public-use, or readiness approval behavior.

## v0.0.142 - 2026-05-11
**Status:** Phase 142 - Provider Execution Result Projection

### Added
- Add refined Rust-owned provider execution result projection for sandboxed deterministic provider execution.
- Add explicit provider output trust, materialization, promotion, linkage, and absence-marker projection fields.
- Add projection validation proving provider output remains untrusted, descriptive, not candidate material, and not promoted.
- Add UI rendering for provider execution result projection details.
- Add Rust and TypeScript tests for projection determinism, visible projection behavior, and non-promotion boundaries.

### Changed
- Extend the local operator shell transport projection with refined provider execution result data.
- Update the local UI shell to display provider execution result projection details and non-candidate status.
- Update checklists/current-phase.md to Phase 142 procedural truth.

### Notes
- Code-production phase.
- Local-session provider execution result projection only.
- Rust remains authoritative for provider execution result projection.
- UI remains non-authoritative.
- Provider output remains untrusted/descriptive.
- Provider output is not candidate material.
- Provider output is not promoted.
- Provider output is not eligible for approve/reject in Phase 142.
- No additional provider execution capability.
- No arbitrary local model execution.
- No cloud model calls.
- No shell command execution.
- No local binary invocation.
- No network sockets.
- No filesystem persistence.
- No durable provider execution storage.
- No durable ledger writes.
- No provider trust approval.
- No production persistence.
- No replay repair.
- No recovery promotion.
- No action execution.
- No release artifact creation.
- No installer, update-channel, signing, publishing, deployment, public-use, or readiness approval behavior.

## v0.0.141 - 2026-05-11
**Status:** Phase 141 - Sandboxed Deterministic Provider Execution Boundary

### Added
- Add Rust-owned sandboxed deterministic provider execution for deterministic_stub only.
- Add provider execution request validation, execution result projection, and capability surface.
- Add local transport support for deterministic provider execution requests and results.
- Add UI rendering for sandboxed deterministic provider execution status and result.
- Add Rust and TypeScript tests for accepted deterministic_stub execution and rejected unsafe/unsupported execution attempts.

### Changed
- Extend the local operator shell projection with provider execution status/result data.
- Update the local UI shell to display provider execution controls, status, and untrusted/descriptive output.
- Update checklists/current-phase.md to Phase 141 procedural truth.

### Notes
- Code-production phase.
- Local-session sandboxed deterministic provider execution only.
- deterministic_stub is the only executable provider.
- Rust remains authoritative for provider execution.
- UI remains non-authoritative.
- Provider output remains untrusted/descriptive.
- No arbitrary local model execution.
- No cloud model calls.
- No shell command execution.
- No local binary invocation.
- No network sockets.
- No filesystem persistence.
- No durable provider configuration storage.
- No durable ledger writes.
- No provider trust approval.
- No production persistence.
- No replay repair.
- No recovery promotion.
- No action execution.
- No release artifact creation.
- No installer, update-channel, signing, publishing, deployment, public-use, or readiness approval behavior.

## v0.0.140 - 2026-05-11
**Status:** Phase 140 - Code-Production Alignment Checkpoint

### Changed
- Reconcile the Phase 136-139 code-production block.
- Confirm the current local product loop includes decision ledger, replay/status projection, local session evidence export preview, and constrained provider configuration validation.
- Remap Phase 141-145 as the next code-production block focused on sandboxed deterministic provider execution.
- Preserve the rule that every non-0/5 phase must produce usable, testable code or a concrete executable artifact.
- Update checklists/current-phase.md to Phase 140 procedural truth.

### Notes
- Alignment checkpoint only.
- No Rust source changes.
- No TypeScript source changes.
- No test changes.
- No schema changes.
- No provider execution.
- No local model execution.
- No cloud model calls.
- No shell command execution.
- No network sockets.
- No production persistence.
- No release artifact creation.
- No installer, update-channel, signing, publishing, deployment, public-use, or readiness approval behavior.
- Phase 141 is expected to resume code production.

## v0.0.139 - 2026-05-11
**Status:** Phase 139 - Constrained Local Provider Configuration Stub

### Added
- Add Rust-owned local provider configuration types and validation for a deterministic_stub configuration surface.
- Add fail-closed validation for unsupported provider kinds, endpoints, commands, file paths, secrets, execution flags, trust flags, readiness claims, release claims, and deployment claims.
- Add local transport support for provider configuration submission and validation projection.
- Add UI rendering for local provider configuration and validation results.
- Add Rust and TypeScript tests for accepted deterministic_stub configuration and rejected unsafe/unsupported provider configuration candidates.

### Changed
- Extend the local operator shell projection with provider configuration state.
- Update the local UI shell to display provider configuration status and validation results.
- Update checklists/current-phase.md to Phase 139 procedural truth.

### Notes
- Code-production phase.
- Local-session in-memory provider configuration stub only.
- deterministic_stub configuration only.
- Rust remains authoritative for provider configuration validation.
- UI remains non-authoritative.
- No provider execution.
- No local binary invocation.
- No cloud calls.
- No network sockets.
- No shell command execution.
- No filesystem persistence.
- No durable provider configuration storage.
- No provider trust approval.
- No production persistence.
- No replay repair.
- No recovery promotion.
- No action execution.
- No release artifact creation.
- No installer, update-channel, signing, publishing, deployment, public-use, or readiness approval behavior.
- Phase 140 remains the next roadmap and changelog alignment checkpoint.

## v0.0.138 - 2026-05-11
**Status:** Phase 138 - Local Session Evidence Export

### Added
- Add a Rust-derived deterministic local session evidence export for the local operator shell flow.
- Add export content covering shell status, run projection, bounded context, candidate output, validation/policy projection, decision ledger, and replay/status projection.
- Add explicit local-only and non-production export classification and absence markers.
- Add UI rendering for the local session evidence export preview.
- Add Rust and TypeScript tests for export determinism, completeness, classification, and visible export behavior.

### Changed
- Extend the local operator shell transport projection to include local session evidence export data.
- Update the local UI shell to display local session evidence export content.
- Update checklists/current-phase.md to Phase 138 procedural truth.

### Notes
- Code-production phase.
- Local-session evidence export only.
- Rust remains authoritative for export derivation.
- UI remains non-authoritative.
- Export is not release evidence.
- Export is not deployment evidence.
- Export is not production readiness evidence.
- Export does not authorize provider output.
- Default export behavior is non-mutating.
- No filesystem persistence unless an explicit optional helper is added and tested with temp paths only.
- No durable ledger writes.
- No provider execution.
- No production persistence.
- No replay repair.
- No recovery promotion.
- No release artifact creation.
- No installer, update-channel, signing, publishing, deployment, public-use, or readiness approval behavior.

## v0.0.137 - 2026-05-11
**Status:** Phase 137 - Replay Projection for Local Decisions

### Added
- Add a Rust-derived local replay/status projection for in-memory local decision ledger records.
- Add replay projection states for no decision, approved decision, and rejected decision local flows.
- Add UI rendering for the local replay/status projection.
- Add Rust and TypeScript tests for replay projection determinism and visible replay/status updates.

### Changed
- Extend the local operator shell transport projection to include decision replay/status data.
- Replace the UI replay/status placeholder with a real local replay/status projection panel.
- Update checklists/current-phase.md to Phase 137 procedural truth.

### Notes
- Code-production phase.
- Local-session in-memory replay/status projection only.
- Rust remains authoritative for replay projection derivation.
- UI remains non-authoritative.
- Replay is descriptive only.
- No replay repair.
- No recovery promotion.
- No filesystem persistence.
- No durable ledger writes.
- No provider execution.
- No production persistence.
- No release artifact creation.
- No installer, update-channel, signing, publishing, deployment, public-use, or readiness approval behavior.

## v0.0.136 - 2026-05-11
**Status:** Phase 136 - In-Memory Local Decision Ledger

### Added
- Add a Rust-owned in-memory local decision ledger for approve/reject operator decisions.
- Add typed local decision records and deterministic decision timeline projection.
- Add UI rendering for the local decision ledger/timeline.
- Add Rust and TypeScript tests for decision ledger behavior and visible decision history updates.

### Changed
- Extend the local operator shell transport projection to include decision ledger/timeline data.
- Update the local UI shell so approve/reject interactions display recorded local decision history.
- Update checklists/current-phase.md to Phase 136 procedural truth.

### Notes
- Code-production phase.
- Local-session in-memory decision ledger only.
- Rust remains authoritative for decision recording.
- UI remains non-authoritative.
- No filesystem persistence.
- No durable ledger writes.
- No provider execution.
- No production persistence.
- No replay repair.
- No recovery promotion.
- No release artifact creation.
- No installer, update-channel, signing, publishing, deployment, public-use, or readiness approval behavior.

## v0.0.134 - 2026-05-11
**Status:** Phase 134 - Rust-to-UI Local Transport Boundary

### Added
- Add a local Rust-owned transport/API boundary for the local operator shell.
- Add local transport behavior for initial shell state, deterministic stub run start, and approve/reject operator intent submission.
- Add Rust and TypeScript tests for the local transport boundary.

### Changed
- Update the local UI shell to use a typed local transport abstraction for shell state and operator intent flow.
- Update checklists/current-phase.md to Phase 134 procedural truth.

### Notes
- Code-production phase.
- Local-only non-production transport boundary.
- Rust remains authoritative for local shell state transitions.
- UI remains non-authoritative.
- Deterministic stub workflow only.
- No external provider execution.
- No cloud model calls.
- No broad command execution.
- No production persistence.
- No release artifact creation.
- No installer, update-channel, signing, publishing, deployment, public-use, or readiness approval behavior.


## v0.0.133 - 2026-05-11
**Status:** Phase 133 - Usable Local Operator UI Shell

### Added
- Add a usable local operator UI shell.
- Add a deterministic local stub run workflow visible in the UI.
- Add candidate output, validation/policy result, run timeline, and operator approve/reject controls.
- Add Rust and TypeScript tests for the local UI shell and typed local operator workflow.

### Changed
- Replace placeholder local UI behavior with a browser-usable development shell.
- Update local validation as needed for the new Rust/UI capability.

### Notes
- Code-production phase.
- Local-only non-production operator shell.
- Deterministic stub provider/workflow only.
- No external provider execution.
- No cloud model calls.
- No production readiness approval.
- No Release Candidate approval.
- No Production Candidate approval.
- No public/general-use approval.
- No release artifact creation.
- No installer, update-channel, signing, publishing, or deployment behavior.

## v0.0.132.3 - 2026-05-11
**Status:** Phase 132.3 - Local Artifact Manifest Producer

### Added
- Add Rust local artifact manifest producer support.
- Add deterministic producer statuses and reason codes.
- Add tests proving produced manifest candidates validate through the local artifact manifest validator.
- Add adversarial coverage for unsafe paths and prohibited claims.
- Add the Phase 132.3 operations report.

### Changed
- Export or extend the local artifact manifest API as needed.
- Update checklists/current-phase.md to Phase 132.3 procedural truth.
- Update CHANGELOG.md with v0.0.132.3.

### Notes
- Manifest production is not artifact release.
- Manifest evidence is not checksum evidence.
- Manifest evidence is not provenance evidence.
- Manifest evidence is not signing evidence.
- Manifest evidence is not publishing evidence.
- Manifest evidence is not deployment evidence.
- Manifest evidence is not readiness approval.
- No release artifact creation.
- No public asset creation.
- No checksum generation.
- No provenance attestation creation.
- No signing behavior.
- No key creation.
- No publishing behavior.
- No deployment automation.
- No production deployment behavior.
- No installer/update-channel activation.
- No monitoring/logging/telemetry activation.
- No provider trust.
- No provider output promotion.
- No persistence authority expansion.
- No replay repair.
- No recovery promotion.
- No action execution.
- No Release Candidate approval.
- No Production Candidate approval.
- No public/general-use approval.
- No production-human-use approval.

## v0.0.136.2 - 2026-05-11
**Status:** Phase 136.2 - Local Artifact Manifest Validation

### Added
- Add Rust local artifact manifest validation.
- Add deterministic validation statuses and rejection reasons.
- Add tests for local/non-public manifest acceptance, unsafe path rejection, claim rejection, and deterministic non-mutating validation.
- Add the Phase 136.2 operations report.

### Changed
- Export the local artifact manifest validation surface from the Rust API module.
- Update checklists/current-phase.md to Phase 136.2 procedural truth.
- Update CHANGELOG.md with v0.0.136.2.

### Notes
- Manifest validation is evidence validation, not release approval.
- No artifact creation.
- No checksum generation.
- No provenance attestation creation.
- No signing behavior.
- No publishing behavior.
- No installer/update-channel behavior.
- No deployment automation.
- No monitoring/logging/telemetry activation.
- No readiness approval.
- No Release Candidate approval.
- No Production Candidate approval.
- No public/general-use approval.
- No production-human-use approval.

## v0.0.136 - 2026-05-11
**Status:** Phase 136 - Installer/Update-Channel Dependency Reassessment

### Added
- Add the Phase 136 installer/update-channel dependency reassessment report.

### Changed
- Update checklists/current-phase.md to Phase 136 procedural truth.
- Update CHANGELOG.md with v0.0.136.
- Update roadmap planned-truth surfaces only if a narrow clarification is required to preserve Phase 136 deferment.

### Notes
- Phase 136 is installer/update-channel dependency reassessment only.
- Installer/update-channel dependency reassessment is not installer/update-channel activation.
- Installer requirements are not installer evidence.
- Update-channel requirements are not update-channel evidence.
- Missing governed artifact evidence blocks installer/update-channel implementation.
- Missing checksum/provenance evidence blocks installer/update-channel implementation.
- Missing signing/key-custody decision evidence blocks installer/update-channel implementation.
- Phase 136 does not create installers, update channels, updater services, daemons, background services, public distribution, deployment automation, or readiness.
- Phase 136 does not approve Release Candidate status.
- Phase 136 does not implement Phase 139 or Phase 140.
- Phase 130 rc_candidate_not_ready is preserved.
- Phase 136 implementation remains deferred unless committed evidence proves otherwise.
- No package creation.
- No release artifact creation.
- No checksum generation.
- No provenance attestation creation.
- No signing behavior.
- No key creation.
- No certificate creation.
- No signature creation.
- No publishing behavior.
- No deployment automation.
- No production deployment behavior.
- No installer/update-channel activation.
- No daemon behavior.
- No background service behavior.
- No public distribution.
- No monitoring/logging/telemetry activation.
- No provider trust.
- No provider output promotion.
- No persistence authority expansion.
- No replay repair.
- No recovery promotion.
- No action execution.
- No readiness approval.
- No Release Candidate approval.
- No Production Candidate approval.
- No public/general-use approval.
- No production-human-use approval.
- No Phase 137 implementation.
- No Phase 139 implementation.
- No Phase 140 implementation.

## v0.0.135.2 - 2026-05-11
**Status:** Out-of-Band Maintenance - Changelog Archive Split

### Added
- Add the Phase 135.2 changelog archive split report.
- Add or update archived changelog files for completed contiguous version ranges selected by line-count and version-boundary inspection.

### Changed
- Move completed historical entries from CHANGELOG.md into docs/changelog archive files without rewriting entry content.
- Update CHANGELOG.md to preserve the active changelog surface and archive pointers.
- Update checklists/current-phase.md to Phase 135.2 procedural truth.
- Update changelog validation only if required for archive naming compatibility.

### Notes
- Out-of-band changelog maintenance only.
- Changelog archiving is historical maintenance, not historical rewriting.
- Archive movement must preserve historical entry content.
- Archive movement preserves historical entry content.
- Archive ranges must remain contiguous and version-bounded.
- Archive ranges remain contiguous and version-bounded.
- Active CHANGELOG.md remains the current historical surface.
- Archived changelog files preserve completed historical truth.
- Phase 135.2 does not change roadmap planned truth except narrow archive-reference clarification if required.
- Phase 135.2 does not approve readiness, Release Candidate status, Production Candidate status, or public/general use.
- No roadmap implementation.
- No runtime behavior.
- No new runtime capability.
- No Rust source changes.
- No TypeScript source changes.
- No test changes.
- No schema changes.
- No package creation.
- No release artifact creation.
- No checksum generation.
- No provenance attestation creation.
- No signing/publishing behavior.
- No installer/update-channel behavior.
- No monitoring/logging/telemetry activation.
- No deployment automation.
- No provider trust.
- No provider output promotion.
- No persistence authority expansion.
- No replay repair.
- No recovery promotion.
- No action execution.
- No readiness approval.
- No Release Candidate approval.
- No Production Candidate approval.
- No public/general-use approval.
- No production-human-use approval.

## v0.0.135.1 - 2026-05-11
**Status:** Out-of-Band Maintenance - Artifact Chain Correction Before Installer/Update-Channel Work

### Added
- Add the Phase 135.1 artifact-chain correction report.
- Add governed local/non-public artifact or manifest evidence only if the existing repository contract and deterministic command evidence support it.

### Changed
- Update checklists/current-phase.md to Phase 135.1 procedural truth.
- Update CHANGELOG.md with v0.0.135.1.
- Update artifact-chain planning surfaces only if needed to preserve Phase 136 deferment or record Phase 133 readiness-to-proceed status.

### Notes
- Phase 135.1 is artifact-chain correction only.
- Artifact-chain correction is not release.
- Local artifact evidence is not Release Candidate readiness.
- Artifact manifest evidence is not checksum evidence.
- Artifact manifest evidence is not provenance evidence.
- Artifact creation does not imply signing, publishing, installer/update-channel activation, deployment, monitoring, or readiness.
- Phase 130 rc_candidate_not_ready is preserved.
- Phase 136 remains deferred unless the artifact chain is explicitly ready for Phase 133 and downstream dependencies remain separated.
- No signing behavior.
- No key creation.
- No certificate creation.
- No signature creation.
- No publishing behavior.
- No deployment automation.
- No production deployment behavior.
- No installer/update-channel activation.
- No monitoring/logging/telemetry activation.
- No provider trust.
- No provider output promotion.
- No persistence authority expansion.
- No replay repair.
- No recovery promotion.
- No action execution.
- No readiness approval.
- No Release Candidate approval.
- No Production Candidate approval.
- No public/general-use approval.
- No production-human-use approval.
- No Phase 136 implementation.
- No Phase 139 implementation.
- No Phase 140 implementation.

## v0.0.135 - 2026-05-11
**Status:** Phase 135 - Code-Production Roadmap and Changelog Alignment

### Changed
- Remap the next roadmap block into code-production mode after the Phase 133 and Phase 134 local operator shell implementation work.
- Establish that every non-0/5 phase must produce usable, testable code or a concrete executable artifact.
- Preserve 0/5 phases as alignment checkpoints for reconciling implementation progress and remapping the next block.
- Update checklists/current-phase.md to Phase 135 procedural truth.

### Notes
- Alignment phase only.
- No Rust source changes.
- No TypeScript source changes.
- No test changes.
- No schema changes.
- No provider execution.
- No production persistence.
- No release artifact creation.
- No installer, update-channel, signing, publishing, deployment, public-use, or readiness approval behavior.
- Phase 136 is expected to resume code production.
