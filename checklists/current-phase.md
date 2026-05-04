---
truth_dimension: procedural
authority_level: authoritative
mutation_path: checklist_revision
---

# Phase 47 - Local Persistence Boundary

## Phase name

Phase 47 - Local Persistence Boundary

## Phase goal

Add typed local persistence planning and deterministic validation boundaries with explicit atomic-write plan metadata, without wiring persistence into dry-run or any default runtime path.

## Allowed surfaces

- `core/src/api/mod.rs`
- `core/src/main.rs`
- `checklists/current-phase.md`
- `CHANGELOG.md`

## Boundary rules

- Persistence is explicit, typed, atomic-by-plan requirement, and opt-in.
- No read-only, dry-run, replay verification, projection, or UI path writes to disk.
- Local persistence paths remain caller-supplied metadata only.
- Phase 46 dry-run remains no-persistence.
- Release-candidate readiness is not claimed.
- Production readiness is not claimed.

## Task checklist

- [x] Update procedural checklist to Phase 47 scope.
- [x] Add typed local persistence plan surfaces in `core/src/api/mod.rs`.
- [x] Add deterministic validation and stable validation/error codes.
- [x] Add explicit atomic-write semantics as typed plan metadata.
- [x] Add deterministic tests for required persistence boundary behavior.
- [x] Keep CLI dry-run as no-persistence and no-write behavior.
- [x] Add `CHANGELOG.md` entry `v0.0.47`.

## Validation checklist

- [x] `./scripts/check.sh`
- [x] `cd ui && npm run typecheck && npm run lint && npm run build`
- [x] `node scripts/test_lint_ui_boundaries.mjs`
- [x] `node scripts/lint_ui_boundaries.mjs`
- [x] `cargo run --manifest-path core/Cargo.toml -- dry-run`
- [x] `rg -n "std::fs|File::|read_to_string|read_dir|canonicalize|metadata|watch|notify|walkdir|glob|write\\(|write!|writeln!|rename|sync_all|flush|serialize|serde|json|env::var|var\\(|std::net|TcpStream|UdpSocket|reqwest|ureq|hyper|tokio|async|await|fetch|http|https|Command::|std::process|thread::|sleep" core/src/main.rs core/src/api/mod.rs core/src/execution/mod.rs`
- [x] `rg -n "persist|persistence|write|rename|atomic|repair|replay repair|serialize|LocalApplicationState|dry-run|dry run|no persistence" core/src/main.rs core/src/api/mod.rs core/src/execution/mod.rs CHANGELOG.md checklists/current-phase.md checklists/release.md`
- [x] `rg -n "release candidate ready|release-candidate ready|RC ready|ready for production|production-ready|production ready" core/src/main.rs CHANGELOG.md checklists/current-phase.md checklists/release.md`
- [x] `rg -n "lint_ui_boundaries|test_lint_ui_boundaries" scripts/check.sh .github/workflows/ci.yml`
- [x] `git status --short`
- [x] `git log --oneline -1`
