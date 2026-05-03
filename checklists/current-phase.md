---
truth_dimension: procedural
authority_level: authoritative
mutation_path: checklist_revision
---

# Current Phase: Phase 31 - Provider Adapter Boundary

This is the active procedural execution surface for Phase 31.

## Phase name

Phase 31 - Provider Adapter Boundary

## Phase goal

Define and implement the first typed provider adapter boundary where provider output enters the harness as untrusted candidate material only.

## Allowed surfaces

- `core/src/execution/mod.rs`
- `core/src/api/mod.rs` (only if minimal read-only boundary typing is required)
- `core/src/lib.rs` (only if export/test cleanup requires it)
- `checklists/current-phase.md`
- `CHANGELOG.md`

## Boundary rules

- Provider output is candidate material only and is never authoritative.
- Do not add trusted provider output states.
- Do not infer validation, policy, execution, promotion, ledger, replay, or audit outcomes from provider output content.
- Do not add real provider calls, HTTP/network behavior, async behavior, provider authentication, model invocation, API server behavior, CLI behavior, or UI behavior.
- Do not add dependencies.
- Do not modify files outside allowed surfaces.

## Task checklist

- [x] Update active checklist to Phase 31 scope and procedural sections.
- [x] Add typed provider adapter boundary types in Rust.
- [x] Represent provider output as untrusted candidate material only.
- [x] Add deterministic constructors and typed provider-boundary errors.
- [x] Add helper proving provider output is never authoritative.
- [x] Add deterministic tests for provider boundary inputs, trust, and non-authoritative behavior.
- [x] Add `CHANGELOG.md` entry `v0.0.31`.

## Validation checklist

- [x] `./scripts/check.sh`
- [x] `cd ui && npm run typecheck && npm run lint && npm run build`
- [x] `rg -n "reqwest|ureq|hyper|tokio|async|await|fetch|http|https|api key|apikey|token|Authorization|Bearer" core scripts ui`
- [x] `rg -n "trusted|authoritative|validated|approved|safe" core/src/execution/mod.rs core/src/api/mod.rs`
- [x] `git status --short`
- [x] `git log --oneline -1`
