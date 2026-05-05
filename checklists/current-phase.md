---
truth_dimension: procedural
authority_level: authoritative
mutation_path: checklist_revision
---
# Current Phase Checklist

## phase name
Phase 76 - UI/Rust Transport Boundary

## phase goal
Define typed transport-shaped request/response contract surfaces between the non-authoritative UI and Rust-owned read/intent surfaces while preserving display-only behavior.

## working-tree hygiene gate
- [x] `git status --short` run before edits and classified.
- [x] Generated artifacts reviewed/reverted before staging (including `core/target/.rustc_info.json` if present).

## allowed surfaces
- [x] `ui/src/api/projections.ts`
- [x] `ui/src/api/readModel.ts`
- [x] `ui/src/api/fixtures.ts`
- [x] `docs/operations/ui-rust-transport-boundary-phase-76.md`
- [x] `checklists/current-phase.md`
- [x] `CHANGELOG.md`
- [ ] Rust source
- [ ] scripts/workflows/dependency files

## boundary rules
- [x] Transport is typed contract shape only; no live transport implementation.
- [x] UI remains non-authoritative and display-only.
- [x] `UI_READ_MODEL_MUTATION_CAPABLE = false` preserved.
- [x] `UI_INTENT_SUBMISSION_ENABLED = false` preserved.
- [x] `UI_INTENT_EXECUTION_ENABLED = false` preserved.
- [x] `UI_INTENT_LEDGER_RECORDING_ENABLED = false` preserved.
- [x] Phase 77 remains responsible for submission wiring.
- [x] Phase 78 remains responsible for authorized action execution boundary.

## validation log table
| Command | Result | Notes |
| --- | --- | --- |
| `git status --short` | pass | Clean start. |
