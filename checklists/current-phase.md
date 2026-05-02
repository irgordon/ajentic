---
truth_dimension: procedural
authority_level: authoritative
mutation_path: checklist_revision
---

# Current Phase: Phase 8.5 — CI Validation Script Extraction

This is the active procedural execution surface for Phase 8.5.

This document does not define governance rules or architecture authority.

This document does not record completed history. Completed accepted work moves to `CHANGELOG.md`.

## Phase

Phase 8.5 — CI Validation Script Extraction

## Phase goal

Eliminate workflow-embedded Python validation logic by extracting structure and documentation checks into first-class scripts under `scripts/` that run identically for local users, agents, and GitHub Actions.

## Allowed surfaces

- `scripts/validate_structure.py`
- `scripts/validate_docs.py`
- `.github/workflows/structure-lint.yml`
- `.github/workflows/docs-gate.yml`
- `checklists/current-phase.md`
- `CHANGELOG.md`

## Boundary rules for this checklist

- This checklist is procedural truth only for active Phase 8.5 execution.
- Governance authority remains in `docs/governance/GOVERNANCE.md` and subordinate governance documents.
- Architecture authority remains in `docs/architecture/ARCHITECTURE.md` and subordinate architecture documents.
- This phase is CI/CD maintenance only.
- This phase must preserve existing validation behavior unless an existing false positive or false negative is identified and documented.
- This phase must not implement runtime harness behavior.
- Completed accepted work must be recorded in `CHANGELOG.md`, not in this checklist.

## Task checklist

- [x] Normalize `checklists/current-phase.md` for Phase 8.5 procedural scope.
- [x] Create `scripts/validate_structure.py` with extracted structure validation logic.
- [x] Create `scripts/validate_docs.py` with extracted documentation boundary logic.
- [x] Update `.github/workflows/structure-lint.yml` to call `python3 scripts/validate_structure.py`.
- [x] Update `.github/workflows/docs-gate.yml` to call `python3 scripts/validate_docs.py`.
- [x] Preserve anchor checks, frontmatter checks, path constraints, and conservative pattern checks.
- [x] Run required validation commands and record results.

## Validation checklist

- [x] `python3 scripts/bootstrap_repo.py`
- [x] `python3 -m compileall scripts/bootstrap_repo.py scripts/validate_structure.py scripts/validate_docs.py`
- [x] `python3 scripts/validate_structure.py`
- [x] `python3 scripts/validate_docs.py`
- [x] `bash -n scripts/*.sh`
- [x] `find schemas -type f -name "*.json" -print0 | xargs -0 -n1 python3 -m json.tool > /dev/null`
- [x] `cargo fmt --manifest-path core/Cargo.toml -- --check`
- [x] `cargo check --manifest-path core/Cargo.toml --all-targets`
- [x] `cargo test --manifest-path core/Cargo.toml --all-targets`
- [x] `cargo clippy --manifest-path core/Cargo.toml --all-targets -- -D warnings`
- [x] `cd ui && npm run typecheck && npm run lint && npm run build`

## Deferred items

| Item | Reason deferred | Target phase |
| --- | --- | --- |
| Runtime harness behavior | Out of scope for Phase 8.5 maintenance | Later phases |
| Policy or validation engine behavior changes | Out of scope for Phase 8.5 maintenance | Later phases |
| Lifecycle, memory, ledger, replay, audit, API, CLI, or UI implementation changes | Out of scope for Phase 8.5 maintenance | Later phases |

## Validation log

| Date | Command | Result | Notes |
| --- | --- | --- | --- |
| 2026-05-02 | `python3 scripts/bootstrap_repo.py` | Pass | Repository already initialized; no new skeleton changes required. |
| 2026-05-02 | `python3 -m compileall scripts/bootstrap_repo.py scripts/validate_structure.py scripts/validate_docs.py` | Pass | Compiled all listed Python scripts successfully. |
| 2026-05-02 | `python3 scripts/validate_structure.py` | Pass | Extracted structure checks pass locally. |
| 2026-05-02 | `python3 scripts/validate_docs.py` | Pass | Extracted docs boundary checks pass locally. |
| 2026-05-02 | `bash -n scripts/*.sh` | Pass | Shell syntax checks passed. |
| 2026-05-02 | `find schemas -type f -name "*.json" -print0 | xargs -0 -n1 python3 -m json.tool > /dev/null` | Pass | Schema JSON syntax checks passed. |
| 2026-05-02 | `cargo fmt --manifest-path core/Cargo.toml -- --check` | Pass | Formatting checks passed. |
| 2026-05-02 | `cargo check --manifest-path core/Cargo.toml --all-targets` | Pass | Rust compile checks passed. |
| 2026-05-02 | `cargo test --manifest-path core/Cargo.toml --all-targets` | Pass | Rust tests passed. |
| 2026-05-02 | `cargo clippy --manifest-path core/Cargo.toml --all-targets -- -D warnings` | Pass | No clippy warnings. |
| 2026-05-02 | `cd ui && npm run typecheck && npm run lint && npm run build` | Pass | UI validation checks passed in current environment. |
