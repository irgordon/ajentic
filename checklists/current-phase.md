---
truth_dimension: procedural
authority_level: authoritative
mutation_path: checklist_revision
---
# Phase 64 - Rust/TypeScript Contract Synchronization Boundary

## Phase name
Phase 64 - Rust/TypeScript Contract Synchronization Boundary

## Phase goal
Synchronize TypeScript mirror contract shapes with Rust-owned API semantics for read projections, intent submission previews, diagnostics, and persistence verification reports.

## Working-tree hygiene gate
- [x] Ran `git status --short` before editing and classified uncommitted files.
- [x] Reverted generated artifacts before commit (including `core/target/.rustc_info.json`).

## Allowed surfaces
- [x] `ui/src/api/projections.ts`
- [x] `ui/src/api/readModel.ts`
- [x] `ui/src/api/fixtures.ts`
- [x] `checklists/current-phase.md`
- [x] `CHANGELOG.md`
- [x] `docs/operations/contract-sync-phase-64.md`

## Boundary rules
- [x] Rust remains authoritative.
- [x] TypeScript mirrors are compile-time/UI display shapes only.
- [x] No transport/API client/fetch/server wiring added.
- [x] No runtime validation/parsing libraries added.
- [x] No Rust behavior changed.
- [x] No readiness/public-usability claim added.

## Task checklist
- [x] Added TypeScript diagnostic family/diagnostic mirror shapes.
- [x] Added TypeScript persisted record verification mirror shapes.
- [x] Tightened TypeScript intent submission preview shape alignment.
- [x] Kept intent/read-model capability constants disabled.
- [x] Updated fixtures to exercise diagnostic and persistence-verification mirror shapes.
- [x] Added Phase 64 operations document.

## Validation checklist
- [x] `./scripts/check.sh`
- [x] `cd ui && npm run typecheck && npm run lint && npm run build`
- [x] `node scripts/test_lint_ui_boundaries.mjs`
- [x] `node scripts/lint_ui_boundaries.mjs`
- [x] `cargo run --manifest-path core/Cargo.toml -- dry-run`
- [x] API decomposition checks
- [x] UI forbidden-pattern scan
- [x] Contract-shape scan
- [x] Non-transport scan
- [x] Readiness scan
- [x] Lint wiring scan

## Deferred items table
| Item | Reason deferred |
| --- | --- |
| Execution-path transport wiring | Out of scope for Phase 64 boundary; compile-time shape sync only. |

## Findings table
| Area | Finding | Status |
| --- | --- | --- |
| Phase map title/scope | Phase 64 title and contract-sync scope confirmed from roadmap. | Confirmed |
| Rust authority boundary | Rust-owned code/diagnostic semantics preserved. | Confirmed |

## Non-readiness statement
Phase 64 aligns contract mirrors only. It does not approve release-candidate readiness, production readiness, or public usability.
