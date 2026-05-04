---
truth_dimension: procedural
authority_level: authoritative
mutation_path: checklist_revision
---

# Phase 43 - Rust Read Projection Boundary

## Phase goal

Add Rust-owned read projection boundary structs for UI/API consumption using caller-supplied typed inputs only, without controlled-flow execution, provider calls, replay verification, file/network/API/CLI/UI behavior, persistence, or serialization.

## Allowed surfaces

- `core/src/api/mod.rs`
- `core/src/execution/mod.rs` only if required for provider-mode alignment
- `core/src/lib.rs` only if export/test cleanup is required
- `checklists/current-phase.md`
- `checklists/release.md` only if release evidence posture changes
- `CHANGELOG.md`

## Task checklist

- [x] Update checklist to Phase 43 scope.
- [x] Add Rust-owned typed read projection surfaces in Rust.
- [x] Add deterministic constructor validation for required read projection identifiers.
- [x] Keep projection metadata caller-supplied typed inputs only.
- [x] Expose runtime safety level/defaults in read projection and reject unsafe runtime config bypass.
- [x] Add deterministic tests for required Phase 43 behaviors.
- [x] Add `CHANGELOG.md` entry `v0.0.43`.

## Validation checklist

- [x] `./scripts/check.sh`
- [x] `cd ui && npm run typecheck && npm run lint && npm run build`
- [x] `node scripts/test_lint_ui_boundaries.mjs`
- [x] `node scripts/lint_ui_boundaries.mjs`
- [x] `rg -n "reqwest|ureq|hyper|tokio|async|await|fetch|http|https|api key|apikey|token|Authorization|Bearer|TcpStream|UdpSocket|std::net" core scripts ui`
- [x] `rg -n "std::fs|File::|read_to_string|read_dir|canonicalize|metadata|watch|notify|walkdir|glob|write|serialize|serde|json|env::|var\\(" core/src/api/mod.rs core/src/execution/mod.rs`
- [x] `rg -n "trusted|authoritative|validated|approved|safe|execute|promote|provider_network|file_io|ui_mutation|secret|token|password|bearer|api_key|apikey" core/src/api/mod.rs core/src/execution/mod.rs`
- [x] `rg -n "lint_ui_boundaries|test_lint_ui_boundaries" scripts/check.sh .github/workflows/ci.yml`
- [x] `git status --short`
- [x] `git log --oneline -1`
