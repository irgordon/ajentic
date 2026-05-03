---
truth_dimension: procedural
authority_level: authoritative
mutation_path: checklist_revision
---

# Current Phase: Phase 28 - Operator Intent Controls UI

This is the active procedural execution surface for Phase 28.

## Phase name

Phase 28 - Operator Intent Controls UI

## Phase goal

Add typed, display-only operator intent preview controls that render request-shaped data without execution, submission, mutation, or authority bypass.

## Allowed surfaces

- `ui/src/api/projections.ts`
- `ui/src/api/fixtures.ts`
- `ui/src/api/readModel.ts`
- `ui/src/screens/OverviewScreen.tsx`
- `ui/src/components/*.tsx`
- `ui/src/styles/*.css`
- `checklists/current-phase.md`
- `CHANGELOG.md`

## Task checklist

- [x] Update active checklist to Phase 28 scope.
- [x] Extend UI read-only projection model with `OperatorIntentPreviewProjection`.
- [x] Update `UiReadModel` to include `operatorIntentPreviews`.
- [x] Add static fixture-backed operator intent preview projection data for approve/reject/retry/replay/context rebuild/memory snapshot request shapes.
- [x] Keep `getUiReadModel()` synchronous and fixture-backed.
- [x] Add display-only `IntentPreviewPanel` component for typed intent request preview display.
- [x] Render request-only operator intent previews on `OverviewScreen` with explicit trust-boundary text.
- [x] Preserve non-authoritative read-only UI boundaries with no submit, mutation, network, provider, replay, audit, or execution behavior.
- [x] Add `CHANGELOG.md` entry `v0.0.28`.
- [x] Run required validation commands and static scan.

## Validation checklist

- [x] `./scripts/check.sh`
- [x] `cd ui && npm run typecheck && npm run lint && npm run build`
- [x] `rg "fetch|localStorage|sessionStorage|WebSocket|EventSource|setInterval|setTimeout|onClick|onSubmit|submit|form|href=|method=|action=|append|mutate|provider|execute|ledger edit|audit export|replay repair|replay rerun|output apply|clean output publish|policy bypass|validation override" ui/src`
- [x] `git status --short`
- [x] `git log --oneline -1`
