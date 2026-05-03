---
truth_dimension: procedural
authority_level: authoritative
mutation_path: checklist_revision
---

# Current Phase: Phase 38 - Static Boundary Lint Baseline

This is the active procedural execution surface for Phase 38.

## Phase name

Phase 38 - Static Boundary Lint Baseline

## Phase goal

Replace the highest-value UI boundary ripgrep checks with a deterministic AST-aware lint baseline while preserving the no-runtime-authority UI boundary.

## Roadmap deviation note

Phase 38 intentionally advances static-boundary linting ahead of the prior roadmap sequence as a scoped maintenance deviation to reduce scan precision debt.

Phase 40 remains the scheduled roadmap/changelog alignment checkpoint for reconciling future sequencing against historical truth.

## Allowed surfaces

- `scripts/lint_ui_boundaries.mjs`
- `scripts/check.sh`
- `checklists/current-phase.md`
- `checklists/release.md`
- `CHANGELOG.md`
- `docs/operations/static-boundary-lint-baseline-phase-38.md`

## Boundary rules

- UI remains non-authoritative and request-only.
- AST lint enforcement must be deterministic and parse-only.
- Ripgrep scans remain advisory/manual evidence checks.
- Do not change Rust runtime behavior.
- Do not change TypeScript UI behavior.
- Do not add dependencies unless strictly required by validation.

## Task checklist

- [x] Update this checklist to Phase 38 and include scoped roadmap deviation note.
- [x] Add `scripts/lint_ui_boundaries.mjs` AST-aware UI boundary lint script.
- [x] Update `scripts/check.sh` to run UI boundary AST lint.
- [x] Keep ripgrep scans as advisory/manual evidence only.
- [x] Update `checklists/release.md` with partial static scan debt reduction and remaining debt notes.
- [x] Add `CHANGELOG.md` entry `v0.0.38`.
- [x] Add advisory report `docs/operations/static-boundary-lint-baseline-phase-38.md`.

## Validation checklist

- [x] `./scripts/check.sh`
- [x] `cd ui && npm run typecheck && npm run lint && npm run build`
- [x] `node scripts/lint_ui_boundaries.mjs`
- [x] `rg -n "fetch|localStorage|sessionStorage|WebSocket|EventSource|setInterval|setTimeout|onClick|onSubmit|submit|form|href=|method=|action=" ui/src`
- [x] `rg -n "reqwest|ureq|hyper|tokio|async|await|fetch|http|https|api key|apikey|token|Authorization|Bearer|TcpStream|UdpSocket|std::net" core scripts ui`
- [x] `git status --short`
- [x] `git log --oneline -1`

## Lint coverage checklist

- [x] `fetch(...)` calls enforced.
- [x] `localStorage`/`sessionStorage` references enforced.
- [x] `new WebSocket(...)` and `new EventSource(...)` enforced.
- [x] `setInterval(...)` and `setTimeout(...)` enforced.
- [x] JSX event-handler attributes (`onClick`, `onSubmit`, `onChange`, `onInput`, `onKeyDown`, `onKeyUp`) enforced.
- [x] JSX `form`, `button`, `a[href]`, and `input[type=submit]` enforced.
- [x] Assignment/object-property event-handler names enforced.
- [x] Forbidden runtime/network import modules enforced.

## Findings table

| Finding | Classification | Status | Notes |
| --- | --- | --- | --- |
| AST lint baseline passes and is deterministic parse-only | now enforced by AST lint | Closed | `node scripts/lint_ui_boundaries.mjs` passed. |
| UI grep matches remain mostly textual/prose and advisory | harmless | Closed | Retained for broader review context only. |
| Broader Rust/network/provider scan terms still string-based | deferred | Open | Requires future precision work beyond this phase. |

## Deferred items table

| Item | Reason deferred | Revisit phase |
| --- | --- | --- |
| Rust/network/provider AST/static enforcement | Outside Phase 38 allowed scope | Future hardening phase |
| UI boundary checks not yet represented in AST lint | Scoped to highest-value baseline in this phase | Future static-boundary phase |

## Validation log table

| Command | Result | Notes |
| --- | --- | --- |
| `./scripts/check.sh` | Pass | Includes new UI boundary AST lint, existing checks, and Rust validations. |
| `cd ui && npm run typecheck && npm run lint && npm run build` | Pass | Typecheck + placeholder lint/build scripts pass. |
| `node scripts/lint_ui_boundaries.mjs` | Pass | Deterministic parse-only enforcement pass. |
| UI boundary advisory scan (`rg ... ui/src`) | Pass | Advisory-only scan output reviewed/classified. |
| Core/scripts/ui network advisory scan (`rg ... core scripts ui`) | Pass | Advisory-only scan output reviewed/classified. |
