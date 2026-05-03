---
truth_dimension: procedural
authority_level: authoritative
mutation_path: checklist_revision
---

# Current Phase: Phase 24 - Context Packet and Memory Inspection UI

This is the active procedural execution surface for Phase 24.

## Phase name

Phase 24 - Context Packet and Memory Inspection UI

## Phase goal

Display context packet and memory snapshot projections in the browser UI as read-only inspection surfaces using static fixture-backed data.

## Allowed surfaces

- `ui/src/api/projections.ts`
- `ui/src/api/fixtures.ts`
- `ui/src/api/readModel.ts`
- `ui/src/screens/ContextScreen.tsx`
- `ui/src/screens/MemoryScreen.tsx`
- `ui/src/components/*.tsx`
- `ui/src/styles/*.css`
- `checklists/current-phase.md`
- `CHANGELOG.md`

## Boundary rules

- Keep browser UI projections display-only, fixture-backed, synchronous, and non-authoritative.
- Do not add API fetching, async behavior, browser storage, timers, provider calls, or mutation paths.
- Do not add runtime authority, operator intent controls, context rebuild controls, memory edit/disable/delete controls, ledger edits, replay repair controls, or execution triggers.
- Do not modify Rust, schemas, scripts, workflows, governance, architecture, roadmap, README, AGENTS.md, or package configuration surfaces.

## Task checklist

- [x] Update active checklist to Phase 24 scope.
- [x] Extend UI read-model projection types with context slice and memory entry inspection projections.
- [x] Add static context slice and memory entry preview fixture data.
- [x] Keep `getUiReadModel()` synchronous and fixture-backed.
- [x] Update Context screen to render packet metadata and context slice inspection details as display-only text.
- [x] Update Memory screen to render snapshot metadata and memory entry inspection details as display-only text.
- [x] Add a read-only projection list component for readable inspection list output.
- [x] Add `CHANGELOG.md` entry `v0.0.24`.
- [x] Run required validation and static scan commands.

## Validation checklist

- [x] `./scripts/check.sh`
- [x] `cd ui && npm run typecheck && npm run lint && npm run build`
- [x] `rg "fetch|localStorage|sessionStorage|WebSocket|EventSource|setInterval|setTimeout|approve|reject|retry|promot|execute|ledger edit|memory mutate|memory edit|context mutate|context edit|replay repair|delete|disable|rebuild" ui/src`
- [x] `git status --short`
- [x] `git log --oneline -1`
