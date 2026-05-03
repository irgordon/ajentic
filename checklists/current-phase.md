---
truth_dimension: procedural
authority_level: authoritative
mutation_path: checklist_revision
---

# Current Phase: Phase 34 - Production Hardening

This is the active procedural execution surface for Phase 34.

## Phase name

Phase 34 - Production Hardening

## Phase goal

Harden failure paths, negative-path coverage, and authority-boundary regression tests without adding new product capability.

## Allowed surfaces

- `core/src/api/mod.rs`
- `core/src/execution/mod.rs`
- `core/src/lib.rs` (only if export/test cleanup requires it)
- `ui/src/**/*.ts`
- `ui/src/**/*.tsx`
- `checklists/current-phase.md`
- `CHANGELOG.md`

## Boundary rules

- Provider and integration outputs are candidate material only and are never authoritative.
- Controlled flow must not infer policy, validation, execution, promotion, replay, or clean output authority from provider/integration content.
- Controlled flow remains deterministic, in-memory, and fail-closed.
- UI remains fixture-backed, non-authoritative, and request-preview-only.
- Do not add provider calls, local model invocation, IDE connection, IO, HTTP/socket behavior, async behavior, API server behavior, CLI behavior, persistence, schema edits, workflow edits, or dependencies.

## Task checklist

- [x] Update active checklist to Phase 34 scope and procedural sections.
- [x] Add Phase 31/32/33 negative-path and boundary-regression tests in Rust surfaces.
- [x] Add compile-safe UI guard exports for synchronous fixture-backed read model and request-preview-only intent surfaces.
- [x] Confirm provider/integration output remains untrusted and non-authoritative.
- [x] Confirm integration metadata is not merged into provider prompt summary.
- [x] Confirm replay verification requires valid lifecycle transition history.
- [x] Add `CHANGELOG.md` entry `v0.0.34`.

## Validation checklist

- [x] `./scripts/check.sh`
- [x] `cd ui && npm run typecheck && npm run lint && npm run build`
- [x] `rg -n "fetch|localStorage|sessionStorage|WebSocket|EventSource|setInterval|setTimeout|onClick|onSubmit|submit|form|href=|method=|action=|append|mutate|provider|execute|ledger edit|audit export|replay repair|replay rerun|output apply|clean output publish|policy bypass|validation override" ui/src`
- [x] `rg -n "reqwest|ureq|hyper|tokio|async|await|fetch|http|https|api key|apikey|token|Authorization|Bearer|TcpStream|UdpSocket|std::net" core scripts ui`
- [x] `rg -n "std::fs|File::|read_to_string|read_dir|watch|notify|walkdir|glob|write|serialize|serde|json" core/src/api/mod.rs core/src/execution/mod.rs`
- [x] `rg -n "trusted|authoritative|validated|approved|safe|execute|promote|operator_context_summary|prompt_summary" core/src/api/mod.rs core/src/execution/mod.rs`
- [x] `git status --short`
- [x] `git log --oneline -1`
