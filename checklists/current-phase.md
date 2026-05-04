---
truth_dimension: procedural
authority_level: authoritative
mutation_path: checklist_revision
---

# Phase 52 - UI Live Read Projection Integration

## Phase name

Phase 52 - UI Live Read Projection Integration

## Phase goal

Replace fixture-only UI read data with a typed read-model boundary that consumes Rust-owned `ApplicationReadProjection`-shaped data while keeping the UI non-authoritative and side-effect-free.

## Allowed surfaces

- `ui/src/api/projections.ts`
- `ui/src/api/fixtures.ts`
- `ui/src/api/readModel.ts`
- `ui/src/app/AppShell.tsx`
- `ui/src/screens/*.tsx`
- `ui/src/components/*.tsx`
- `checklists/current-phase.md`
- `checklists/release.md` (only if evidence posture changes)
- `CHANGELOG.md`

## Boundary rules

- UI consumption boundary only; no runtime Rust bridge, fetch, API client, or server integration.
- No UI mutation, no event handlers, no forms, no buttons, no anchor href wiring.
- Operator intent preview remains request-preview-only and non-functional.
- Provider and integration outputs remain untrusted and non-authoritative.
- Raw provider/model output remains distinct from clean output.
- `getUiReadModel` remains synchronous and side-effect-free.
- Release-candidate readiness is not claimed.
- Production readiness is not claimed.

## Task checklist

- [x] Update checklist to Phase 52 scope.
- [x] Add/align `ApplicationUiProjection` and `RuntimeSafetyUiProjection` typed surfaces.
- [x] Add pure synchronous `buildUiReadModelFromApplicationProjection(...)` helper.
- [x] Keep explicit fixture fallback path.
- [x] Keep `getUiReadModel` synchronous and side-effect-free.
- [x] Update app/screens to consume typed projection-backed read model.
- [x] Display runtime safety posture and trust posture in UI summaries.
- [x] Preserve non-functional operator intent preview surface.
- [x] Add `CHANGELOG.md` entry `v0.0.52`.

## Validation checklist

- [x] `./scripts/check.sh`
- [x] `cd ui && npm run typecheck && npm run lint && npm run build`
- [x] `node scripts/test_lint_ui_boundaries.mjs`
- [x] `node scripts/lint_ui_boundaries.mjs`
- [x] `cargo run --manifest-path core/Cargo.toml -- dry-run`
- [x] `rg -n "fetch|localStorage|sessionStorage|WebSocket|EventSource|setInterval|setTimeout|onClick|onSubmit|onChange|onInput|onKeyDown|onKeyUp|<form|<button|href=|type=\"submit\"|submit\\(|addEventListener" ui/src`
- [x] `rg -n "intent submission|submit_operator_intent|operator_intent_ingress|ledger append|persist|provider call|run_controlled_model_flow|execute_local_persistence_plan|fetch|api client|server" ui/src CHANGELOG.md checklists/current-phase.md checklists/release.md`
- [x] `rg -n "rawOutputTrusted|provider output remains untrusted|integration output remains untrusted|allowProviderNetwork|allowFileIo|allowUiMutation|release-candidate readiness is not claimed|production readiness is not claimed" ui/src CHANGELOG.md checklists/current-phase.md`
- [x] `rg -n "lint_ui_boundaries|test_lint_ui_boundaries" scripts/check.sh .github/workflows/ci.yml`
