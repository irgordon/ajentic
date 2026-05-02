---
truth_dimension: procedural
authority_level: authoritative
mutation_path: checklist_revision
---

# Current Phase: Phase 4 — Contract Schema Baseline

This is the active procedural execution surface for Phase 4.

This document does not define governance rules or architecture authority.

This document does not record completed history. Completed accepted work moves to `CHANGELOG.md`.

## Phase

Phase 4 — Contract Schema Baseline

## Phase goal

Define strict shared JSON Schema Draft 2020-12 contract baselines under `schemas/` for future validation, memory, intent, ledger, replay, and documentation work without implementing runtime behavior.

## Allowed surfaces

- `schemas/docs/artifact-frontmatter.schema.json`
- `schemas/context/context-packet.schema.json`
- `schemas/context/context-slice.schema.json`
- `schemas/context/context-budget.schema.json`
- `schemas/memory/memory-entry.schema.json`
- `schemas/memory/memory-snapshot.schema.json`
- `schemas/memory/memory-provenance.schema.json`
- `schemas/events/ledger-event.schema.json`
- `schemas/events/run-event.schema.json`
- `schemas/events/audit-event.schema.json`
- `schemas/intents/operator-intent.schema.json`
- `schemas/intents/approve-intent.schema.json`
- `schemas/intents/reject-intent.schema.json`
- `schemas/intents/retry-intent.schema.json`
- `schemas/intents/memory-intent.schema.json`
- `schemas/traces/run-trace.schema.json`
- `schemas/traces/replay-report.schema.json`
- `schemas/traces/validation-report.schema.json`
- `checklists/current-phase.md`
- `CHANGELOG.md`
- `.github/workflows/ci.yml`
- `.github/workflows/structure-lint.yml`
- `.github/workflows/docs-gate.yml`
- `.github/workflows/memory-lint.yml`

## Boundary rules for this checklist

- This checklist is procedural truth only for active Phase 4 execution.
- Governance authority remains in `docs/governance/GOVERNANCE.md` and subordinate governance documents.
- Architecture authority remains in `docs/architecture/ARCHITECTURE.md`.
- Contract truth remains rooted in `schemas/`; no Rust or TypeScript contract duplication is introduced in this phase.
- Completed accepted work must be recorded in `CHANGELOG.md`, not in this checklist.

## Task checklist

- [ ] Normalize `checklists/current-phase.md` for Phase 4 procedural scope.
- [ ] Replace placeholder Phase 4 schema files with strict Draft 2020-12 contract schemas.
- [ ] Ensure schema contracts include required versioning and boundary fields.
- [ ] Ensure schema files remain JSON-only contract truth under `schemas/`.
- [ ] Review CI and lint workflows for Phase 4 compatibility and edit only if required.
- [ ] Add `v0.0.4` changelog entry.
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
| Runtime harness behavior | Out of scope for Phase 4 | Later phases |
| TypeScript bindings and generated validators | Out of scope for Phase 4 | Later phases |
| Rust schema consumer/runtime validation wiring | Out of scope for Phase 4 | Later phases |
| Provider adapters and policy lifecycle logic | Out of scope for Phase 4 | Later phases |
| Replay/audit runtime behavior | Out of scope for Phase 4 | Later phases |

## Validation log

| Date | Command | Result | Notes |
| --- | --- | --- | --- |
