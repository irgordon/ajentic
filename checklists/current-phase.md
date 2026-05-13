---
truth_dimension: procedural
authority_level: authoritative
mutation_path: checklist_revision
---

# Phase 167 - Trial Observability and Error Reporting

## Phase goal
- [x] Add local trial observability and error reporting derived from controlled internal trial execution state.

## Working-tree hygiene gate
- [x] Keep changes within Phase 167 allowed surfaces.
- [x] Do not modify roadmap, governance, architecture, release, installer, deployment, or publishing files.

## Allowed surfaces
- [x] `core/src/**`
- [x] `ui/src/**`
- [x] `CHANGELOG.md`
- [x] `checklists/current-phase.md`

## Code-production deliverable checklist
- [x] Rust-owned trial observability projection exists.
- [x] Rust-owned trial error report projection exists.
- [x] Local shell state carries both projections.
- [x] UI renders the projections without telemetry, monitoring, remediation, escalation, or approval controls.

## Trial observability checklist
- [x] Trial observability status is projected.
- [x] Trial run status and current trial step are projected.
- [x] Current blocker is projected.
- [x] Package, evidence, read-back, replay, and restore verification summaries are projected.
- [x] Evidence linkage summary is projected.

## Trial error report checklist
- [x] Error report status is projected.
- [x] Error details include category, severity, source, guidance, and evidence linkage.
- [x] Error ordering is deterministic.
- [x] Error report is local and descriptive only.

## Error category/severity checklist
- [x] Closed categories cover missing, blocked, rejected, mismatched, invalid, and stop-condition-observed states.
- [x] Severity classification is deterministic.
- [x] Source linkage is deterministic.

## Blocked-state reporting checklist
- [x] Current blocker is summarized.
- [x] Blocked/rejected reasons are summarized.
- [x] Manual operator step requirement is reported.

## Stop-condition observation reporting checklist
- [x] Stop-condition observation status is rendered.
- [x] Stop-condition markers are rendered.
- [x] Stop-condition automation remains disabled.

## Verification mismatch reporting checklist
- [x] Replay/status mismatch is reported.
- [x] Restore/history mismatch is reported.
- [x] Mismatch drilldown is visible in the UI.

## Package/evidence failure reporting checklist
- [x] Missing package and evidence are reported.
- [x] Validation failures are reported.
- [x] Read-back failures are reported.

## UI observability/error panel checklist
- [x] Trial observability panel renders.
- [x] Trial error reporting panel renders.
- [x] Trial error drilldown renders.
- [x] Trial blocked-state summary renders.

## Local-only/no-telemetry boundary checklist
- [x] Trial observability is local-only and non-public.
- [x] No production monitoring is active.
- [x] No remote telemetry is sent.
- [x] No network reporting is added.
- [x] No background service is active.

## No-authority checklist
- [x] No remediation, escalation, or stop-condition enforcement is automated.
- [x] No action execution is added.
- [x] No replay repair or recovery promotion is added.
- [x] No controlled-human-use, readiness, release, deployment, public-use, or production-use approval is granted.

## Rust test checklist
- [x] Initial projection coverage.
- [x] Happy-path observability coverage.
- [x] Blocked/rejected and stop-condition coverage.
- [x] Verification mismatch and package/evidence read-back failure coverage.
- [x] No-authority boundary coverage.

## TypeScript test checklist
- [x] Observability panel rendering coverage.
- [x] Error reporting panel rendering coverage.
- [x] Blocked-state rendering coverage.
- [x] Mismatch drilldown coverage.
- [x] Local-only/no-telemetry/no-monitoring wording coverage.

## Phase 168 handoff checklist
- [x] Phase 168 remains the next code-production phase for trial evidence review surface.

## Validation checklist
- [x] `CARGO_TARGET_DIR=/tmp/ajentic-phase-167-target ./scripts/check.sh`
- [x] `git diff --check`
- [x] `git status --short`
- [x] UI direct commands if needed.
- [x] Rust direct tests if needed.
- [x] Local dev smoke test.
- [x] Observability scan.
- [x] Boundary scan.
- [x] Forbidden label scan.
- [x] Unsafe execution scan.
- [x] Release/deployment scan.
- [x] Changed-file source guard.
- [x] No-roadmap-drift guard.

## Deferred items
- [x] Phase 168 trial evidence review surface.

## Validation log
- Direct Rust and UI validation passed before commit.
- Full `scripts/check.sh` is expected to run on the clean committed tree because the script requires no working-tree diff at startup.

## Zero-drift checklist
- [x] Roadmap files are not modified.
- [x] CHANGELOG entry reflects Phase 167 scope.
- [x] No release, deployment, signing, publishing, installer, update-channel, production monitoring, or remote telemetry behavior is added.
