---
truth_dimension: procedural
authority_level: authoritative
mutation_path: checklist_revision
---

# Current Phase: Phase 0 — Initial Repo Setup

This is the active execution surface for Phase 0.

This document does not record completed history. Completed work moves to `CHANGELOG.md`.

This document does not define governance rules or architecture authority.

## Phase

Phase 0 — Initial Repo Setup

## Goal

Create the minimum governed repository skeleton required for Phase 0 bootstrap as described in `docs/roadmap/phase-map.md`.

## Allowed surfaces

- `docs/governance/GOVERNANCE.md`
- `docs/architecture/ARCHITECTURE.md`
- `AGENTS.md`
- `README.md`
- `CHANGELOG.md`
- Canonical directories
- `scripts/bootstrap_repo.py`
- Placeholder Rust modules (`core/`)
- Placeholder UI files (`ui/`)
- Placeholder schemas (`schemas/`)
- Governed docs (`docs/governance/`, `docs/architecture/`, `docs/roadmap/`, `docs/operations/`, `docs/examples/`)
- Checklists (`checklists/`)
- Memory structure (`memory/`)
- Test directories (`tests/`)

## Task checklist

- [ ] Top-level anchors created (`docs/governance/GOVERNANCE.md`, `docs/architecture/ARCHITECTURE.md`, `AGENTS.md`, `README.md`, `CHANGELOG.md`)
- [ ] Canonical directories created
- [ ] Rust core skeleton compiles (`core/Cargo.toml`, `core/src/lib.rs`, module stubs)
- [ ] TypeScript UI placeholder structure created (`ui/package.json`, `ui/tsconfig.json`, source dirs)
- [ ] Scripts created and executable (`scripts/*.sh`, `scripts/bootstrap_repo.py`)
- [ ] Test directories created (`tests/`)
- [ ] Memory structure created (`memory/persistent/`, `memory/ephemeral/.gitignore`)
- [ ] Checklists created (`checklists/`)
- [ ] Governed docs created (`docs/architecture/`, `docs/roadmap/`, `docs/operations/`, `docs/examples/`)
- [ ] Schema placeholder files created (`schemas/`)

## Validation checklist

- [ ] `python3 scripts/bootstrap_repo.py` runs without error (idempotent — run twice)
- [ ] `python3 -m compileall scripts/bootstrap_repo.py` passes
- [ ] `bash -n scripts/*.sh` passes
- [ ] `find schemas -type f -name "*.json" -print0 | xargs -0 -n1 python3 -m json.tool > /dev/null` passes
- [ ] `cargo check --manifest-path core/Cargo.toml` passes
- [ ] `cd ui && npm install && npm run typecheck && npm run lint && npm run build` passes (when Node is available)

## Deferred items

| Item | Reason deferred | Target phase |
| --- | --- | --- |
| Runtime harness behavior | Out of scope for Phase 0 | Phase 1+ |
| Model provider adapters | Out of scope for Phase 0 | Phase 3+ |
| Real policy logic | Out of scope for Phase 0 | Phase 1+ |
| Real validation logic | Out of scope for Phase 0 | Phase 1+ |
| Ledger persistence | Out of scope for Phase 0 | Phase 2+ |
| Replay logic | Out of scope for Phase 0 | Phase 2+ |
| Generated TypeScript bindings | Out of scope for Phase 0 | Phase 4+ |
| Browser UI functionality | Out of scope for Phase 0 | Phase 4+ |

## Validation log

| Date | Command | Result | Notes |
| --- | --- | --- | --- |
