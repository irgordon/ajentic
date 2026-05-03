---
truth_dimension: procedural
authority_level: authoritative
mutation_path: checklist_revision
---

# Current Phase: Phase 23 - State and Run Overview UI

This is the active procedural execution surface for Phase 23.

## Phase name

Phase 23 - State and Run Overview UI

## Phase goal

Display lifecycle, execution decision, promotion decision, and run summary projections in the browser UI as read-only views using static fixture-backed data.

## Allowed surfaces

- `ui/src/api/projections.ts`
- `ui/src/api/fixtures.ts`
- `ui/src/api/readModel.ts`
- `ui/src/app/AppShell.tsx`
- `ui/src/screens/OverviewScreen.tsx`
- `ui/src/screens/StateScreen.tsx`
- `ui/src/components/*.tsx`
- `ui/src/styles/*.css`
- `checklists/current-phase.md`
- `CHANGELOG.md`

## Boundary rules

- Keep browser UI projections display-only, fixture-backed, and non-authoritative.
- Do not add API fetching, async behavior, browser storage, timers, provider calls, or mutation paths.
- Do not add runtime authority, operator intent controls, promotion execution, execution trigger, ledger edits, memory edits, or replay repair controls.
- Do not modify Rust, schemas, scripts, workflows, governance, architecture, roadmap, README, AGENTS.md, or package configuration surfaces.

## Task checklist

- [x] Update active checklist to Phase 23 scope.
- [x] Extend UI read-model projection types with run overview projection data.
- [x] Add static run overview fixture data with read-only values only.
- [x] Keep `getUiReadModel()` synchronous and fixture-backed.
- [x] Update Overview screen to render run overview projection fields as display-only text.
- [x] Update State screen to render lifecycle details and run linkage as display-only text.
- [x] Update AppShell summary to include run overview metadata.
- [x] Add display-only `StatusPill` component for explicit status text.
- [x] Add `CHANGELOG.md` entry `v0.0.23`.
- [x] Run required validation and static scan commands.

## Validation checklist

- [x] `./scripts/check.sh`
- [x] `cd ui && npm run typecheck && npm run lint && npm run build`
- [x] `rg "fetch|localStorage|sessionStorage|WebSocket|EventSource|setInterval|setTimeout|approve|reject|retry|promot|execute|ledger edit|memory mutate|replay repair" ui/src`
- [x] `git status --short`
- [x] `git log --oneline -1`
