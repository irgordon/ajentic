---
truth_dimension: procedural
authority_level: authoritative
mutation_path: checklist_revision
---

# Current Phase: Phase 1 — Governance and Architecture Baseline

This is the active execution surface for Phase 1.

This document does not record completed history. Completed work moves to `CHANGELOG.md`.

This document does not define governance rules or architecture authority.

## Phase

Phase 1 — Governance and Architecture Baseline

## Goal

Establish the smallest durable governance and architecture baseline needed for agents and humans to continue work without boundary drift.

## Allowed surfaces

- `docs/governance/GOVERNANCE.md`
- `docs/governance/*.md`
- `docs/architecture/ARCHITECTURE.md`
- `docs/architecture/*.md`
- `AGENTS.md`
- `README.md`
- `checklists/current-phase.md`
- `CHANGELOG.md`
- `.github/workflows/docs-gate.yml`
- `.github/workflows/structure-lint.yml`
- `.github/instructions/*.md`

## Task checklist

- [ ] Governance anchor audit (`docs/governance/GOVERNANCE.md`)
- [ ] Governance subdocument audit (`docs/governance/*.md`)
- [ ] Architecture anchor audit (`docs/architecture/ARCHITECTURE.md`)
- [ ] Architecture subdocument audit (`docs/architecture/*.md`)
- [ ] AGENTS.md navigation-only check
- [ ] README.md orientation-only check
- [ ] Validation commands executed and results captured
- [ ] `CHANGELOG.md` updated with Phase 1 (`v0.0.1`) entry

## Validation checklist

- [ ] `python3 scripts/bootstrap_repo.py`
- [ ] `python3 -m compileall scripts/bootstrap_repo.py`
- [ ] `bash -n scripts/*.sh`
- [ ] `find schemas -type f -name "*.json" -print0 | xargs -0 -n1 python3 -m json.tool > /dev/null`
- [ ] `cargo check --manifest-path core/Cargo.toml`
- [ ] `cd ui && npm run typecheck && npm run lint && npm run build`

## Deferred items

| Item | Reason deferred | Target phase |
| --- | --- | --- |
| Runtime harness behavior | Out of scope for Phase 1 | Later phases |
| Model provider adapters | Out of scope for Phase 1 | Later phases |
| Policy execution logic | Out of scope for Phase 1 | Later phases |
| Validation execution logic | Out of scope for Phase 1 | Later phases |
| Memory behavior implementation | Out of scope for Phase 1 | Later phases |
| Ledger/replay runtime implementation | Out of scope for Phase 1 | Later phases |
| Browser UI runtime functionality | Out of scope for Phase 1 | Later phases |

## Validation log

| Date | Command | Result | Notes |
| --- | --- | --- | --- |
