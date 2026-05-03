---
truth_dimension: procedural
authority_level: authoritative
mutation_path: checklist_revision
---

# Current Phase: Phase 22 - Read-Only API Projection Surface for UI

This is the active procedural execution surface for Phase 22.

## Phase name

Phase 22 - Read-Only API Projection Surface for UI

## Phase goal

Add read-only TypeScript UI projection shapes and static fixture-backed read-model access for display-only browser UI surfaces.

## Allowed surfaces

- `ui/src/app/AppShell.tsx`
- `ui/src/api/*.ts`
- `ui/src/app/AppShell.tsx`
- `ui/src/screens/*.tsx`
- `ui/src/components/*.tsx`
- `ui/src/styles/*.css`
- `checklists/current-phase.md`
- `CHANGELOG.md`

## Boundary rules

- Keep the browser UI as a display-only shell in this phase.
- Do not add runtime authority, mutation paths, operator intent controls, API integration, or provider calls.
- Do not add runtime data fetching, browser storage usage, or state mutation behavior.
- Do not modify Rust, schemas, scripts, governance, architecture, roadmap, or package configuration surfaces.

## Task checklist

- [x] Update active checklist to Phase 22 scope.
- [x] Add read-only UI projection types under `ui/src/api/projections.ts`.
- [x] Add static fixture-backed read model data under `ui/src/api/fixtures.ts`.
- [x] Add synchronous read-model accessor under `ui/src/api/readModel.ts`.
- [x] Wire AppShell and read-only screens to display fixture projection summaries.
- [x] Keep projection display non-authoritative and read-only with no intent controls.
- [x] Verify UI shell remains read-only and intent-free.
- [x] Add `CHANGELOG.md` entry `v0.0.22`.
- [x] Run required validation commands.

## Validation checklist

- [x] `./scripts/check.sh`
- [x] `cd ui && npm run typecheck && npm run lint && npm run build`
- [x] `git status --short`
- [x] `git log --oneline -1`

## Deferred items

| Item | Reason | Planned phase |
| --- | --- | --- |
| Operator intent controls | Explicitly out of scope for shell baseline. | Phase 28 |
| API-backed read-only projections | Explicitly out of scope for shell baseline. | Phase 22 |

## Validation log

| Command | Result | Notes |
| --- | --- | --- |
| `./scripts/check.sh` | Pass | Repository checks passed after Phase 21 shell scaffolding updates. |
| `cd ui && npm run typecheck && npm run lint && npm run build` | Pass | UI checks passed with no new dependencies. |
| `git status --short` | Pass | Verified scoped Phase 21 changes only. |
| `git log --oneline -1` | Pass | Captured commit pointer before Phase 21 commit. |
