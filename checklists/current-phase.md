---
truth_dimension: procedural
authority_level: authoritative
mutation_path: checklist_revision
---

# Phase 42 - Local Runtime Configuration Boundary

## Phase goal

Add typed Rust local runtime configuration boundaries and deterministic validation only, without adding provider execution, filesystem access, controlled-flow execution, state mutation, or UI behavior.

## Allowed surfaces

- `core/src/api/mod.rs`
- `core/src/execution/mod.rs` only if required for provider-mode alignment
- `core/src/lib.rs` only if export/test cleanup is required
- `checklists/current-phase.md`
- `checklists/release.md` only if release evidence posture changes
- `CHANGELOG.md`

## Task checklist

- [x] Update checklist to Phase 42 scope.
- [x] Add typed local runtime configuration surfaces in Rust.
- [x] Add deterministic validation for empty or malformed local runtime configuration metadata fields.
- [x] Keep workspace path handling caller-supplied metadata only.
- [x] Keep runtime safety defaults closed for provider network, file IO, and UI mutation.
- [x] Add deterministic tests for required Phase 42 behaviors.
- [x] Add `CHANGELOG.md` entry `v0.0.42`.

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
