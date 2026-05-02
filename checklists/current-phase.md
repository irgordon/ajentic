---
truth_dimension: procedural
authority_level: authoritative
mutation_path: checklist_revision
---

# Current Phase: Phase 3 — CI and Structure Drift Gates

This is the active procedural execution surface for Phase 3.

This document does not define governance rules or architecture authority.

This document does not record completed history. Completed accepted work moves to `CHANGELOG.md`.

## Phase

Phase 3 — CI and Structure Drift Gates

## Phase goal

Harden deterministic CI and repository validation gates so structure, documentation placement, schema placement, memory placement, script syntax, Rust compile shape, and UI placeholder checks fail early before runtime code expands.

## Allowed surfaces

- `.github/workflows/ci.yml`
- `.github/workflows/structure-lint.yml`
- `.github/workflows/docs-gate.yml`
- `.github/workflows/memory-lint.yml`
- `.github/workflows/pr-agent-review.yml`
- `.github/instructions/*.md`
- `.gitignore`
- `checklists/current-phase.md`
- `CHANGELOG.md`
- `docs/roadmap/phase-map.md`
- `docs/governance/phase-execution-contract.md`

## Boundary rules for this checklist

- This checklist is procedural truth only for active Phase 3 execution.
- Governance authority remains in `docs/governance/GOVERNANCE.md` and `docs/governance/phase-execution-contract.md`.
- Architecture authority remains in `docs/architecture/ARCHITECTURE.md`.
- Workflows are enforcement wiring only and must stay traceable to governed docs and repository contracts.
- Completed accepted work must be recorded in `CHANGELOG.md`, not in this checklist.

## Task checklist

- [ ] Normalize `checklists/current-phase.md` for Phase 3 procedural scope.
- [ ] Harden `.github/workflows/ci.yml` with conditional Rust/UI/schema/script gates.
- [ ] Harden `.github/workflows/structure-lint.yml` for canonical structure and conservative frontmatter checks.
- [ ] Harden `.github/workflows/docs-gate.yml` for conservative truth-dimension boundary checks.
- [ ] Harden `.github/workflows/memory-lint.yml` for memory placement and JSON syntax checks.
- [ ] Verify `.github/workflows/pr-agent-review.yml` remains advisory and least-privilege.
- [ ] Verify `.github/instructions/*.md` remain Copilot metadata and non-governance roots.
- [ ] Verify `.gitignore` contains required build/cache/environment/ephemeral exclusions.
- [ ] Add `v0.0.3` changelog entry.
- [ ] Run required validation commands and record results.

## Validation checklist

- [ ] `python3 scripts/bootstrap_repo.py`
- [ ] `python3 -m compileall scripts/bootstrap_repo.py`
- [ ] `bash -n scripts/*.sh`
- [ ] `find schemas -type f -name "*.json" -print0 | xargs -0 -n1 python3 -m json.tool > /dev/null`
- [ ] `cargo fmt --manifest-path core/Cargo.toml -- --check`
- [ ] `cargo check --manifest-path core/Cargo.toml --all-targets`
- [ ] `cargo test --manifest-path core/Cargo.toml --all-targets`
- [ ] `cargo clippy --manifest-path core/Cargo.toml --all-targets -- -D warnings`
- [ ] `cd ui && npm run typecheck && npm run lint && npm run build`

## Deferred items

| Item | Reason deferred | Target phase |
| --- | --- | --- |
| Runtime harness behavior | Out of scope for Phase 3 | Later phases |
| Model provider adapters | Out of scope for Phase 3 | Later phases |
| Policy/validation runtime logic | Out of scope for Phase 3 | Later phases |
| Memory, ledger, replay, audit runtime behavior | Out of scope for Phase 3 | Later phases |
| Browser UI runtime functionality | Out of scope for Phase 3 | Later phases |

## Validation log

| Date | Command | Result | Notes |
| --- | --- | --- | --- |
