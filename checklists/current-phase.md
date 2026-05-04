---
truth_dimension: procedural
authority_level: authoritative
mutation_path: checklist_revision
---

# Phase 46 - Local CLI Dry-Run Entry

## Phase name

Phase 46 - Local CLI Dry-Run Entry

## Phase goal

Add a minimal deterministic local CLI dry-run entry that composes in-memory typed fixtures and prints a safe summary without provider/model calls, file IO, persistence, replay repair, network/socket behavior, environment-variable reads, workspace scanning, API server startup, or background services.

## Allowed surfaces

- `core/src/main.rs`
- `checklists/current-phase.md`
- `CHANGELOG.md`

## Boundary rules

- Dry-run is deterministic local command composition only.
- Provider output remains untrusted and non-authoritative.
- No file reads/writes, persistence, network/socket behavior, environment-variable reads, replay repair, workspace scanning, API server startup, background services, process spawning, or UI behavior.
- `LocalApplicationState` remains in-memory only.
- Phase 47 remains the explicit local persistence boundary.
- Release-candidate readiness is not claimed.
- Production readiness is not claimed.

## Task checklist

- [x] Update procedural checklist to Phase 46 scope.
- [x] Add minimal CLI dry-run path in `core/src/main.rs`.
- [x] Support `dry-run` command and default no-arg dry-run summary output.
- [x] Handle unknown commands with a safe usage summary.
- [x] Compose deterministic in-memory typed fixtures from Rust-owned surfaces.
- [x] Print explicit non-readiness and non-side-effect statements in dry-run output.
- [x] Add deterministic tests for dry-run summary and unknown command usage.
- [x] Add `CHANGELOG.md` entry `v0.0.46`.

## Validation checklist

- [x] `./scripts/check.sh`
- [x] `cd ui && npm run typecheck && npm run lint && npm run build`
- [x] `node scripts/test_lint_ui_boundaries.mjs`
- [x] `node scripts/lint_ui_boundaries.mjs`
- [x] `cargo run --manifest-path core/Cargo.toml -- dry-run`
- [x] `cargo run --manifest-path core/Cargo.toml`
- [x] `rg -n "std::fs|File::|read_to_string|read_dir|canonicalize|metadata|watch|notify|walkdir|glob|write\\(|write!|writeln!|serialize|serde|json|env::var|var\\(|std::net|TcpStream|UdpSocket|reqwest|ureq|hyper|tokio|async|await|fetch|http|https|Command::|std::process|thread::|sleep" core/src/main.rs core/src/api/mod.rs core/src/execution/mod.rs`
- [x] `rg -n "release candidate ready|release-candidate ready|RC ready|ready for production|production-ready|production ready" core/src/main.rs CHANGELOG.md checklists/current-phase.md checklists/release.md`
- [x] `rg -n "lint_ui_boundaries|test_lint_ui_boundaries" scripts/check.sh .github/workflows/ci.yml`
- [x] `git status --short`
- [x] `git log --oneline -1`
