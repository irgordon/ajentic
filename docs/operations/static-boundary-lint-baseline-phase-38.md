---
truth_dimension: orientation
authority_level: advisory
mutation_path: readme_update
---

# Phase 38 - Static Boundary Lint Baseline

## Scope

Phase 38 adds a deterministic AST-aware UI boundary lint baseline using existing TypeScript tooling and updates procedural evidence surfaces.

This is a scoped maintenance deviation focused on reducing UI static scan precision debt.

## Enforced by AST lint

- Forbidden browser/runtime APIs in `ui/src` (`fetch`, storage, websocket/eventsource, timers).
- Forbidden JSX executable controls and event-handler attributes.
- Forbidden assignment/object-property handler names.
- Forbidden obvious runtime/network module imports.

## Still advisory

- Ripgrep scans for broad cross-surface inspection remain advisory.
- String-match scan output classification remains manual review evidence.

## Deferred enforcement

- Rust/network/provider precision checks are still deferred.
- Additional UI boundary checks not in this baseline remain deferred.

## Validation results

- `./scripts/check.sh`: pass (including AST lint).
- `cd ui && npm run typecheck && npm run lint && npm run build`: pass.
- `node scripts/lint_ui_boundaries.mjs`: pass.
- Advisory ripgrep scans executed and classified.

## Roadmap/sequencing note

Phase 38 intentionally advances static-boundary linting ahead of prior roadmap sequence as a scoped maintenance deviation.

Phase 40 remains responsible for roadmap/changelog sequencing reconciliation.

## Non-goals

- No UI behavior changes.
- No Rust behavior changes.
- No provider work.
- No API server or CLI behavior changes.
- No schema/workflow/dependency/governance/architecture/roadmap rewrites.
- No release-candidate readiness or production-readiness claim.
