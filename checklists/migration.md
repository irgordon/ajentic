---
truth_dimension: procedural
authority_level: authoritative
mutation_path: checklist_revision
---

# Migration checklist

This checklist defines bounded migration steps for schema, contract, or data-shape changes.

This document does not define governance rules or architecture authority.

Migrations must preserve Rust authority and fail closed on unknown, blocked, failed, malformed, missing, or rejected states.

## Pre-migration

- [ ] Identify migration scope and affected contracts.
- [ ] Confirm whether a schema version change is required.
- [ ] Update contract maps for changed schema intent, target, event, trace, context, or memory values.
- [ ] Review Rust-owned models affected by the contract change.
- [ ] Review TypeScript projection models affected by the contract change.
- [ ] Define explicit handling for unknown, blocked, failed, malformed, missing, and rejected values.

## Migration rules

- [ ] Do not infer missing values.
- [ ] Do not convert unknown values into success.
- [ ] Do not convert blocked, failed, malformed, missing, or rejected states into pass/ready/approved states.
- [ ] Do not move authority from Rust into TypeScript, Python, Bash, schemas, docs, workflows, or generated reports.
- [ ] Preserve existing ledger/replay evidence semantics unless a later scoped phase explicitly changes them.

## Validation

- [ ] Run `python3 scripts/validate_operator_intent_contract_map.py` if operator intent contracts changed.
- [ ] Run `CARGO_TARGET_DIR=/tmp/ajentic-check-target ./scripts/check.sh` or phase-required explicit migration checks.
- [ ] Run targeted Rust, TypeScript, schema, and documentation checks for changed surfaces.
- [ ] Record migration evidence in `CHANGELOG.md` and the active phase checklist.
