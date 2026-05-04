---
truth_dimension: procedural
authority_level: authoritative
mutation_path: checklist_revision
---

# Phase 48 - Provider Adapter Trait and Deterministic Stub

## Phase name

Phase 48 - Provider Adapter Trait and Deterministic Stub

## Phase goal

Add a Rust-owned provider adapter trait and deterministic stub provider that produces untrusted provider output only, with no provider/model/network/file/persistence execution.

## Allowed surfaces

- `core/src/execution/mod.rs`
- `core/src/api/mod.rs` (not required this phase)
- `core/src/main.rs` (dry-run assertion only)
- `core/src/lib.rs` (not required this phase)
- `checklists/current-phase.md`
- `checklists/release.md` (not changed; posture unchanged)
- `CHANGELOG.md`

## Boundary rules

- Stub provider output remains untrusted and non-authoritative.
- No real provider/model invocation.
- No network, file IO, environment reads, async, process/thread/timer use, ledger append, controlled-flow execution, persistence, or serialization.
- Phase 46 dry-run remains no-provider-call and no-persistence.
- Phase 47 persistence remains validation/stub-only and not physically implemented.
- Release-candidate readiness is not claimed.
- Production readiness is not claimed.

## Task checklist

- [x] Update procedural checklist to Phase 48 scope.
- [x] Add typed provider adapter trait and adapter invocation/result types.
- [x] Add deterministic stub provider implementation using `ProviderOutput::new_untrusted`.
- [x] Add deterministic tests for adapter/stub behavior and non-authoritative boundaries.
- [x] Keep CLI dry-run no-provider-call behavior and assert it does not use stub output.
- [x] Add `CHANGELOG.md` entry `v0.0.48`.

## Validation checklist

- [x] `./scripts/check.sh`
- [x] `cd ui && npm run typecheck && npm run lint && npm run build`
- [x] `node scripts/test_lint_ui_boundaries.mjs`
- [x] `node scripts/lint_ui_boundaries.mjs`
- [x] `cargo run --manifest-path core/Cargo.toml -- dry-run`
- [x] `rg -n "std::fs|File::|read_to_string|read_dir|canonicalize|metadata|watch|notify|walkdir|glob|write\\(|write!|writeln!|rename|sync_all|flush|serialize|serde|json|env::var|var\\(|std::net|TcpStream|UdpSocket|reqwest|ureq|hyper|tokio|async|await|fetch|http|https|Command::|std::process|thread::|sleep" core/src/main.rs core/src/api/mod.rs core/src/execution/mod.rs`
- [x] `rg -n "provider|adapter|stub|real provider|model|invoke|trusted|authoritative|validated|approved|safe|execute|promote|persist|write|ledger append|run_controlled_model_flow|execute_local_persistence_plan" core/src/main.rs core/src/api/mod.rs core/src/execution/mod.rs CHANGELOG.md checklists/current-phase.md checklists/release.md`
- [x] `rg -n "release candidate ready|release-candidate ready|RC ready|ready for production|production-ready|production ready" core/src/main.rs CHANGELOG.md checklists/current-phase.md checklists/release.md`
- [x] `rg -n "lint_ui_boundaries|test_lint_ui_boundaries" scripts/check.sh .github/workflows/ci.yml`
