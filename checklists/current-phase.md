---
truth_dimension: procedural
authority_level: authoritative
mutation_path: checklist_revision
---
# Phase 63 - Error-Code Family and Reporting Standardization

## Phase name
Phase 63 - Error-Code Family and Reporting Standardization

## Phase goal
Add typed diagnostic family/context reporting around existing stable `code()` strings without changing runtime behavior.

## Working-tree hygiene gate
- [x] Ran `git status --short` before editing and classified uncommitted files.
- [x] Reverted generated artifacts before commit (including `core/target/.rustc_info.json`).

## Allowed surfaces
- [x] API files under `core/src/api/` for diagnostic wrappers.
- [x] `checklists/current-phase.md`
- [x] `CHANGELOG.md`
- [x] `docs/operations/error-code-standardization-phase-63.md`

## Boundary rules
- [x] Existing `code()` string values remain unchanged.
- [x] Duplicate `code()` strings remain allowed across families.
- [x] No global uniqueness registry added.
- [x] No validation order changes.
- [x] No UI/CLI reporting integration added.

## Task checklist
- [x] Added `DiagnosticFamily` and `ErrorDiagnostic`.
- [x] Added `diagnostic_family_label(...)` and `diagnostic_key(...)` helpers.
- [x] Added API-side diagnostic mapping helpers.
- [x] Added phase-63 operations document.

## Validation checklist
- [ ] `./scripts/check.sh`
- [ ] `cd ui && npm run typecheck && npm run lint && npm run build`
- [ ] `node scripts/test_lint_ui_boundaries.mjs`
- [ ] `node scripts/lint_ui_boundaries.mjs`
- [ ] `cargo run --manifest-path core/Cargo.toml -- dry-run`

## Diagnostic family checklist
- [x] OperatorIntent
- [x] Integration
- [x] RuntimeConfig
- [x] ReadProjection
- [x] ApplicationState
- [x] PersistenceValidation
- [x] PersistenceExecution
- [x] PersistenceRecovery
- [x] LocalWorkflow

## Code preservation checklist
- [x] Existing `code()` method outputs preserved.
- [x] `diagnostic_key(...)` is additive reporting key only.

## Duplicate-code handling checklist
- [x] Duplicate `code()` values can coexist across families.
- [x] Family-qualified key disambiguates duplicates.

## Coverage checklist
- [x] API-side diagnostic helpers implemented.
- [x] API-side preserve-code tests implemented.
- [x] Family/label/key deterministic tests implemented.

## Deferred items table
| Item | Reason deferred |
| --- | --- |
| Execution-owned diagnostic mappings | Deferred to keep Phase 63 scoped to API-side standardization. |

## Findings table
| Area | Finding | Status |
| --- | --- | --- |
| Roadmap title | Matches Phase 63 expected title and scope. | Confirmed |

## Validation log table
| Command | Result | Notes |
| --- | --- | --- |
| Required validation/scans | Pending | Run after edits complete. |
