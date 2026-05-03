---
truth_dimension: procedural
authority_level: authoritative
mutation_path: checklist_revision
---

# Current Phase: Phase 33 - Local LLM and IDE Integration Boundary

This is the active procedural execution surface for Phase 33.

## Phase name

Phase 33 - Local LLM and IDE Integration Boundary

## Phase goal

Define typed local LLM and IDE integration boundary surfaces that package caller-supplied integration output as untrusted provider-like input for the existing controlled flow.

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

- [x] Update active checklist to Phase 33 scope and procedural sections.
- [x] Add typed local LLM and IDE integration boundary surfaces in Rust.
- [x] Support local LLM and IDE integration source types as caller-supplied metadata only.
- [x] Convert valid integration output into existing untrusted ProviderRequest and ProviderOutput shapes.
- [x] Keep integration output untrusted and never authoritative.
- [x] Add deterministic tests for integration boundary constructors, mappings, and non-authoritative behavior.
- [x] Add `CHANGELOG.md` entry `v0.0.33`.

## Validation checklist

- [x] `./scripts/check.sh`
- [x] `cd ui && npm run typecheck && npm run lint && npm run build`
- [x] `rg -n "reqwest|ureq|hyper|tokio|async|await|fetch|http|https|api key|apikey|token|Authorization|Bearer" core scripts ui`
- [x] `rg -n "trusted|authoritative|validated|approved|safe" core/src/execution/mod.rs core/src/api/mod.rs`
- [x] `git status --short`
- [x] `git log --oneline -1`
