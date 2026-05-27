---
truth_dimension: orientation
authority_level: advisory
mutation_path: readme_update
---

# Phase 184.2 - OOB Security Audit - Remaining Heuristic Triage

## Scope
Classify remaining heuristic scan hits and repair only authority-sensitive cases where repair is narrow and safe.

## Evidence rule
Repository truth only: all conclusions are based on current repository code, tests, and command output executed in this phase.

## Phase 184.S carry-forward
- Phase 184.S reported no critical finding and no rebuild trigger.
- Phase 184.S recommended release-path pause pending security repairs.

## Phase 184.1 carry-forward
- Exact-match authority classification helper remains active.
- Positive authority markers remain rejected by exact token.
- Denial markers remain allowed by exact token.
- Unknown authority-shaped tokens remain fail-closed in authority-sensitive contexts.

## Triage method
1. Execute required heuristic, authority-surface, positive-marker, and denial-marker scans.
2. Review remaining hits on authority-adjacent surfaces first.
3. Classify each reviewed hit into required categories.
4. Apply only narrow repairs when authority-sensitive and not already hardened.

## Remaining heuristic inventory
Primary remaining heuristic hits cluster in:
- `core/src/api/release_candidate_dry_run_rehearsal.rs`
- `core/src/api/local_operator_shell.rs`
- `ui/src/api/submissionBoundary.behavior.test.ts`
- Typed status/reason `String` fields across review/projection structs.

## Authority-sensitive repair findings
No additional authority-sensitive repair-required hits were found in the allowed repair surface. Authority-sensitive token decisions are routed through exact-match classifier logic from Phase 184.1.

## Already-hardened authority findings
- `core/src/api/authority_classification.rs` exact-match allow/reject/fail-closed classifier remains authoritative for authority-shaped tokens.

## Accepted non-authority findings
- Non-authority rendering/status projection `String` fields in rehearsal/review/local-shell projections.
- Deterministic summary parsing and fixture formatting helpers in local-shell support code.

## Test-only accepted findings
- `ui/src/api/submissionBoundary.behavior.test.ts` token splitting helper used only by adversarial tests.

## Documentation/copy accepted findings
- Positive/denial marker strings in `CHANGELOG.md`, `checklists/current-phase.md`, and prior audit docs are copy/audit artifacts, not runtime authority logic.

## Deferred refactor candidates
- `core/src/api/local_operator_shell.rs`: broad lowercase/contains heuristics in provider-output linting and error tagging are non-authority today but should be migrated to typed rule sets in a future refactor.
- `core/src/api/release_candidate_dry_run_rehearsal.rs`: blocker reason prefix check (`starts_with("rejected_")`) is deterministic but stringly; could be replaced with typed blocker kind in a future refactor.

## False positives
- `status: String`/`reason: String` hits on plain data carriers that do not grant authority.

## Repairs performed
- No code-path repair was required after triage; authority-sensitive decisions are already hardened.

## Required future repairs
- Optional maintainability refactor: replace non-authority string heuristics with typed enums in large projection modules.

## Release-path decision
`release_path_can_resume_after_heuristic_triage`

## Rebuild trigger assessment
No rebuild trigger found.

## Validation output
- Required validation stack executed (fmt, check, tests, clippy, UI typecheck/lint/build/test/dev smoke, diff check).
- Post-repair scans executed for heuristic, authority helper, positive markers, denial markers, no-Phase-185 guard, and roadmap drift guard.

## Zero-drift statement
Phase 184.2 performed out-of-band security triage documentation/checklist/changelog updates and did not introduce release/public/deployment/signing/publishing/provider-trust/action/replay/recovery authority behavior.

## Triage table
| File | Line or pattern | Heuristic type | Surface | Classification | Reason | Action |
| --- | --- | --- | --- | --- | --- | --- |
| `core/src/api/authority_classification.rs` | `token.contains('_')` | contains | Authority classifier | authority_sensitive_already_hardened | Unknown authority-shaped token handling is explicit fail-closed logic from Phase 184.1. | Keep as-is. |
| `core/src/api/release_candidate_dry_run_rehearsal.rs` | `b.reason.starts_with("rejected_")` | starts_with | Rehearsal status derivation | deferred_refactor_candidate | String prefix drives local status wording only; no authority grant path. | Defer typed blocker-kind refactor. |
| `core/src/api/release_candidate_dry_run_rehearsal.rs` | `status: String`, `reason: String` | string status fields | Projection DTO | non_authority_accepted | Projection transport fields do not authorize release actions. | Keep as-is. |
| `core/src/api/local_operator_shell.rs` | `to_ascii_lowercase` + `contains` clusters | lowercase/contains | Provider output and note linting | deferred_refactor_candidate | Heuristics classify messaging/reviewability boundaries, not approval authority. | Defer targeted typed refactor. |
| `core/src/api/local_operator_shell.rs` | many `status: String` / `reason: String` fields | string status fields | UI projection data | non_authority_accepted | Human-readable state reporting only; no authority transitions granted. | Keep as-is. |
| `ui/src/api/submissionBoundary.behavior.test.ts` | `split(/[^a-zA-Z0-9_]+/u)` | split | Test helper | test_only_accepted | Exists only in behavior tests for token boundary adversarial coverage. | Keep as-is. |
| `CHANGELOG.md` | positive/denial marker phrases | keyword scan hit | Documentation | documentation_or_copy_accepted | Changelog text only. | Keep as-is. |
| `checklists/current-phase.md` | positive/denial marker phrases | keyword scan hit | Checklist copy | documentation_or_copy_accepted | Procedural checklist text only. | Keep as-is. |
