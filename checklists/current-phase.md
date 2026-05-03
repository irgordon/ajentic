---
truth_dimension: procedural
authority_level: authoritative
mutation_path: checklist_revision
---

# Current Phase: Phase 21 - Browser UI Shell Baseline

This is the active procedural execution surface for Phase 21.

## Phase name

Phase 21 - Browser UI Shell Baseline

## Phase goal

Create a minimal compile-safe browser UI shell and layout scaffolding that remains read-only, non-authoritative, and intent-free.

## Allowed surfaces

- `ui/src/app/AppShell.tsx`
- `ui/src/app/routes.tsx`
- `ui/src/app/navigation.ts`
- `ui/src/styles/tokens.css`
- `ui/src/styles/layout.css`
- `ui/src/screens/*.tsx`
- `ui/src/components/*.tsx`
- `checklists/current-phase.md`
- `CHANGELOG.md`

## Boundary rules

- Keep the browser UI as a display-only shell in this phase.
- Do not add runtime authority, mutation paths, operator intent controls, API integration, or provider calls.
- Do not add runtime data fetching, browser storage usage, or state mutation behavior.
- Do not modify Rust, schemas, scripts, governance, architecture, roadmap, or package configuration surfaces.

## Task checklist

- [x] Update active checklist to Phase 21 scope.
- [x] Replace placeholder UI shell files with compile-safe shell scaffolding.
- [x] Add typed navigation display metadata for planned UI areas.
- [x] Add typed route metadata for planned screens.
- [x] Add static read-only placeholder screens for Overview, State, Context, Memory, Policy, Validation, Ledger, Replay, Audit, and Output.
- [x] Add shared layout-only `SectionCard` component.
- [x] Add design tokens and responsive layout CSS scaffolding.
- [x] Verify UI shell remains read-only and intent-free.
- [x] Add `CHANGELOG.md` entry `v0.0.21`.
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
