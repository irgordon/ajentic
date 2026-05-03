---
truth_dimension: procedural
authority_level: authoritative
mutation_path: checklist_revision
---

# Phase 39 - UI Boundary Lint Diagnostic Hardening

## Phase goal

Harden the AST-aware UI boundary lint baseline by improving diagnostic quality and adding deterministic self-tests for forbidden UI runtime/control patterns.

## Roadmap deviation note

Phase 39 continues the scoped static-boundary lint maintenance deviation introduced in Phase 38 to reduce static scan precision debt ahead of the Phase 40 roadmap/changelog sequencing reconciliation checkpoint.

## Allowed surfaces

- `scripts/lint_ui_boundaries.mjs`
- `scripts/test_lint_ui_boundaries.mjs`
- `scripts/check.sh`
- `checklists/current-phase.md`
- `checklists/release.md`
- `CHANGELOG.md`
- `docs/operations/static-boundary-lint-hardening-phase-39.md`

## Boundary rules

- Keep UI non-authoritative and request-only.
- Keep lint deterministic, read-only, parse-only, and dependency-neutral.
- Do not change Rust runtime behavior, schema contracts, workflow behavior, governance, architecture, or roadmap truth surfaces.
- Keep ripgrep scans advisory/manual evidence checks.

## Task checklist

- [x] Update checklist to Phase 39 with scoped maintenance continuation note.
- [x] Improve `scripts/lint_ui_boundaries.mjs` diagnostics to IDE-friendly `path:line:column: message`.
- [x] Add deterministic self-tests in `scripts/test_lint_ui_boundaries.mjs` using temporary files outside tracked UI source.
- [x] Update `scripts/check.sh` to run lint self-tests before direct UI boundary lint.
- [x] Update `checklists/release.md` static scan debt/evidence language for diagnostic+self-test hardening.
- [x] Add `CHANGELOG.md` entry `v0.0.39`.
- [x] Add advisory report `docs/operations/static-boundary-lint-hardening-phase-39.md`.

## Validation checklist

- [x] `./scripts/check.sh`
- [x] `cd ui && npm run typecheck && npm run lint && npm run build`
- [x] `node scripts/test_lint_ui_boundaries.mjs`
- [x] `node scripts/lint_ui_boundaries.mjs`
- [x] `rg -n "fetch|localStorage|sessionStorage|WebSocket|EventSource|setInterval|setTimeout|onClick|onSubmit|submit|form|href=|method=|action=" ui/src`
- [x] `rg -n "reqwest|ureq|hyper|tokio|async|await|fetch|http|https|api key|apikey|token|Authorization|Bearer|TcpStream|UdpSocket|std::net" core scripts ui`
- [x] `git status --short`
- [x] `git log --oneline -1`

## Lint diagnostic checklist

- [x] Diagnostics emit `path:line:column: message`.
- [x] Lint reports all violations before exit.
- [x] Exit code is nonzero for violations and zero for clean runs.
- [x] Production default target remains `ui/src`.
- [x] Optional explicit target roots supported for deterministic self-test harness use.

## Lint self-test checklist

- [x] Uses only built-in Node modules and existing lint script.
- [x] Creates temporary files under OS temp directory.
- [x] Includes allowed static text case.
- [x] Includes clean TS/TSX case.
- [x] Includes forbidden cases: `fetch`, `localStorage`, `new WebSocket`, `setTimeout`, JSX `onClick`, JSX `form`, JSX `button`, anchor `href`, submit input, object property `onSubmit`.
- [x] Asserts pass/fail expectations and diagnostic format.
- [x] Cleans temporary files/directories.
- [x] Prints short pass summary and exits nonzero on failure.

## Findings table

| Finding | Classification | Status | Notes |
| --- | --- | --- | --- |
| AST lint now provides IDE-friendly location diagnostics | enforced by AST lint | Closed | Violations now print as `path:line:column: message`. |
| Deterministic lint self-tests now guard forbidden UI runtime/control pattern detection | enforced by AST lint | Closed | Self-test harness validates clean and forbidden cases with temporary files. |
| Ripgrep UI/runtime matches remain broad evidence and not automatic failures | harmless | Closed | Continues as advisory/manual review output. |
| Rust/network/provider/script/docs static precision debt remains broader than current UI AST scope | deferred | Open | Deferred to future phases beyond current allowed scope. |

## Deferred items table

| Item | Reason deferred | Revisit phase |
| --- | --- | --- |
| Rust/network/provider AST/static enforcement | Outside Phase 39 allowed scope | Future static-boundary phase |
| Bash/Python boundary precision checks | Outside Phase 39 allowed scope | Future static-boundary phase |
| Documentation/prohibition-language scan precision | Outside Phase 39 allowed scope | Future static-boundary phase |
| Roadmap/changelog sequencing reconciliation | Assigned checkpoint | Phase 40 |

## Validation log table

| Command | Result | Notes |
| --- | --- | --- |
| `./scripts/check.sh` | Pass | Includes lint self-test and production AST lint before Rust checks. |
| `cd ui && npm run typecheck && npm run lint && npm run build` | Pass | UI validation commands pass without behavior changes. |
| `node scripts/test_lint_ui_boundaries.mjs` | Pass | Deterministic self-tests pass across allowed and forbidden cases. |
| `node scripts/lint_ui_boundaries.mjs` | Pass | Production UI boundary lint passes on `ui/src`. |
| UI boundary advisory scan (`rg ... ui/src`) | Pass | Advisory output reviewed/classified; no automatic failures. |
| Core/scripts/ui network advisory scan (`rg ... core scripts ui`) | Pass | Advisory output reviewed/classified; broad terms remain deferred evidence. |
