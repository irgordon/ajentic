---
truth_dimension: procedural
authority_level: authoritative
mutation_path: checklist_revision
---

# Current Phase: Phase 27 - Ledger, Replay, Audit, and Clean Output UI

This is the active procedural execution surface for Phase 27.

## Phase name

Phase 27 - Ledger, Replay, Audit, and Clean Output UI

## Phase goal

Display ledger timeline, replay readiness/reconstruction results, audit projections, and clean output summaries as readable, non-authoritative, fixture-backed UI projections without adding runtime authority, live integration, or operator controls.

## Allowed surfaces

- `ui/src/api/projections.ts`
- `ui/src/api/fixtures.ts`
- `ui/src/api/readModel.ts`
- `ui/src/screens/LedgerScreen.tsx`
- `ui/src/screens/ReplayScreen.tsx`
- `ui/src/screens/AuditScreen.tsx`
- `ui/src/screens/OutputScreen.tsx`
- `ui/src/components/*.tsx`
- `ui/src/styles/*.css`
- `checklists/current-phase.md`
- `CHANGELOG.md`

## Task checklist

- [x] Update active checklist to Phase 27 scope.
- [x] Extend UI read-only projection model with `LedgerTimelineEntryProjection`, `ReplayDetailProjection`, `AuditDetailProjection`, and `CleanOutputProjection`.
- [x] Update `UiReadModel` to include `ledgerTimeline`, `replayDetail`, `auditDetails`, and `cleanOutput`.
- [x] Add static fixture-backed ledger timeline, replay detail, audit detail, and clean output projection data.
- [x] Keep `getUiReadModel()` synchronous and fixture-backed.
- [x] Render readable ledger timeline projections in `LedgerScreen`.
- [x] Render replay readiness and reconstruction projections in `ReplayScreen`.
- [x] Render audit summary and audit detail projections in `AuditScreen`.
- [x] Render clean output summary and raw-output trust distinction in `OutputScreen`.
- [x] Add/update a readable display-only timeline list component.
- [x] Preserve non-authoritative read-only UI boundaries with no operator intent controls.
- [x] Add `CHANGELOG.md` entry `v0.0.27`.
- [x] Run required validation commands and static scan.

## Validation checklist

- [x] `./scripts/check.sh`
- [x] `cd ui && npm run typecheck && npm run lint && npm run build`
- [x] `rg "fetch|localStorage|sessionStorage|WebSocket|EventSource|setInterval|setTimeout|approve|reject|retry|promot|execute|ledger edit|audit export|replay repair|replay rerun|output apply|clean output publish|memory mutate|context mutate|policy bypass|validation override" ui/src`
- [x] `git status --short`
- [x] `git log --oneline -1`
