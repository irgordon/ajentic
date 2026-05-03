---
truth_dimension: orientation
authority_level: advisory
mutation_path: readme_update
---

# Phase 39 - UI Boundary Lint Diagnostic Hardening

## Scope

Phase 39 hardens the existing AST-aware UI boundary lint baseline by improving diagnostics and adding deterministic self-tests only.

This phase is a scoped maintenance continuation of the static-boundary lint deviation started in Phase 38.

## Diagnostic format

UI boundary violations are emitted in IDE-friendly format:

- `path:line:column: message`

The lint still reports all violations before nonzero exit.

## Self-test coverage

`scripts/test_lint_ui_boundaries.mjs` validates deterministic behavior for:

- allowed static text and clean TS/TSX examples
- forbidden `fetch(...)`
- forbidden `localStorage`
- forbidden `new WebSocket(...)`
- forbidden `setTimeout(...)`
- forbidden JSX `onClick`
- forbidden JSX `form`
- forbidden JSX `button`
- forbidden anchor `href`
- forbidden submit input
- forbidden object property `onSubmit`

Self-tests create temporary files outside tracked `ui/src`, run lint, validate results, and clean up.

## Still advisory

Ripgrep scans remain advisory evidence for broad cross-surface review.

String-match output remains classification input, not automatic failure.

## Deferred enforcement

Remaining static scan precision debt includes:

- Rust/network/provider checks
- Bash/Python boundary checks
- documentation/prohibition-language scans
- additional UI checks not yet represented in AST lint

## Validation results

- `./scripts/check.sh`: pass
- `cd ui && npm run typecheck && npm run lint && npm run build`: pass
- `node scripts/test_lint_ui_boundaries.mjs`: pass
- `node scripts/lint_ui_boundaries.mjs`: pass
- advisory ripgrep scans: executed and classified

## Roadmap/sequencing note

Phase 39 is a scoped maintenance continuation ahead of Phase 40.

Phase 40 remains responsible for roadmap/changelog sequencing reconciliation.

## Non-goals

- No UI behavior changes
- No runtime behavior changes
- No Rust behavior changes
- No provider work
- No API server changes
- No CLI behavior changes
- No schema/workflow/dependency changes
- No governance/architecture/roadmap rewrites
- No release-candidate readiness or production-readiness claims
