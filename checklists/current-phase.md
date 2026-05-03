---
truth_dimension: procedural
authority_level: authoritative
mutation_path: checklist_revision
---

# Phase 40 - Roadmap/Changelog Reconciliation + AST Lint CI Alignment

## Phase goal

Reconcile planned roadmap sequencing against historical changelog truth after the Phase 38/39 maintenance deviation and verify CI alignment with the AST-aware UI boundary lint baseline.

## Allowed surfaces

- `docs/roadmap/phase-map.md`
- `checklists/current-phase.md`
- `checklists/release.md`
- `CHANGELOG.md`
- `.github/workflows/*.yml` only if AST lint CI alignment is missing
- `docs/operations/repository-audit-phase-40.md` if needed

## Boundary rules

- Treat `CHANGELOG.md` as historical truth and `docs/roadmap/phase-map.md` as planned truth.
- Preserve Phase 38/39 as scoped maintenance deviations, not retroactive planned feature phases.
- Correct only future planned sequencing drift.
- Do not modify runtime/UI behavior, schemas, scripts, governance, architecture, or dependencies.
- Do not claim release-candidate readiness or production readiness.

## Task checklist

- [x] Update current checklist to Phase 40 scope.
- [x] Compare roadmap planned sequence against changelog history.
- [x] Reconcile planned future sequencing for Phase 38/39 maintenance deviation context.
- [x] Confirm Phase 41 is next planned implementation phase after Phase 40.
- [x] Audit CI workflows for AST-aware UI boundary lint baseline execution.
- [x] Update CI workflow minimally to run AST-aware UI boundary lint baseline on pull requests.
- [x] Update release checklist AST lint CI evidence language.
- [x] Add `CHANGELOG.md` entry `v0.0.40`.

## Validation checklist

- [x] `./scripts/check.sh`
- [x] `cd ui && npm run typecheck && npm run lint && npm run build`
- [x] `node scripts/test_lint_ui_boundaries.mjs`
- [x] `node scripts/lint_ui_boundaries.mjs`
- [x] `rg -n "scripts/check.sh|test_lint_ui_boundaries|lint_ui_boundaries" .github/workflows scripts/check.sh`
- [x] `rg -n "production-ready|production ready|release candidate ready|release-candidate ready|RC ready|ready for production" docs checklists CHANGELOG.md README.md AGENTS.md`
- [x] `rg -n "fetch|localStorage|sessionStorage|WebSocket|EventSource|setInterval|setTimeout|onClick|onSubmit|submit|form|href=|method=|action=" ui/src`
- [x] `rg -n "reqwest|ureq|hyper|tokio|async|await|fetch|http|https|api key|apikey|token|Authorization|Bearer|TcpStream|UdpSocket|std::net" core scripts ui`
- [x] `git status --short`
- [x] `git log --oneline -1`

## Roadmap/changelog alignment checklist

- [x] Roadmap contains Phase 19.5 historical implementation note.
- [x] Roadmap contains "Planned sequence from current state" divider.
- [x] Roadmap contains recurring alignment checkpoint language.
- [x] Future planned sequencing reconciled for the Phase 38/39 maintenance deviation context.
- [x] Roadmap remains planned-truth only and does not record completion history.

## AST lint CI alignment checklist

- [x] CI pull-request workflow includes AST lint self-test command.
- [x] CI pull-request workflow includes AST UI boundary lint command.
- [x] CI lint alignment added with minimal workflow change and no new dependencies.

## Findings table

| Finding | Classification | Status | Notes |
| --- | --- | --- | --- |
| Roadmap/changelog alignment required Phase 40 planned-sequence reconciliation after maintenance deviation | required follow-up | Closed | Planned sequence updated; historical completion remains in changelog. |
| AST-aware UI boundary lint baseline was local-only before workflow audit | required follow-up | Closed | CI now executes self-test + lint commands in PR workflow UI job. |
| Broad advisory ripgrep scans still return context-rich matches | harmless | Closed | Matches classified as advisory evidence, not automatic failures. |
| Wider static scan precision debt outside UI AST scope remains | deferred | Open | Deferred to future scoped static-boundary phases. |

## Deferred items table

| Item | Reason deferred | Revisit phase |
| --- | --- | --- |
| Rust/network/provider AST static enforcement expansion | Outside Phase 40 scope | Future static-boundary phase |
| Bash/Python boundary precision automation | Outside Phase 40 scope | Future static-boundary phase |
| Documentation/prohibition-language scan precision automation | Outside Phase 40 scope | Future static-boundary phase |

## Validation log table

| Command | Result | Notes |
| --- | --- | --- |
| `./scripts/check.sh` | Pass | Includes AST lint self-test + lint, docs/structure/schema gates, and Rust checks. |
| `cd ui && npm run typecheck && npm run lint && npm run build` | Pass | UI validation passes with no behavior changes. |
| `node scripts/test_lint_ui_boundaries.mjs` | Pass | Deterministic AST lint self-tests pass. |
| `node scripts/lint_ui_boundaries.mjs` | Pass | Production UI AST boundary lint passes on `ui/src`. |
| `rg -n "scripts/check.sh|test_lint_ui_boundaries|lint_ui_boundaries" .github/workflows scripts/check.sh` | Pass | Verified CI + local wiring evidence for AST lint baseline. |
| Advisory scans (`rg ...`) | Pass | Classified matches as enforced by AST lint / harmless / deferred. |
