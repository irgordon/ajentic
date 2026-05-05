---
truth_dimension: orientation
authority_level: advisory
mutation_path: readme_update
---

# Validation Gate Alignment - Phase 76.5

## Scope
Phase 76.5 is an out-of-band maintenance fix before Phase 77.

Phase 76.5 changes validation tooling only. It does not implement UI submission wiring, and it does not modify Rust or TypeScript behavior.

## Why this out-of-band maintenance phase exists
The local validation gate needed maintenance alignment so that the local whole-repo check posture matches expected repository validation semantics before Phase 77 submission wiring work begins.

## check.sh non-mutation correction
Phase 76.5 removes mutating Rust auto-format execution from `scripts/check.sh` and keeps formatting verification only (`cargo fmt --manifest-path core/Cargo.toml -- --check`).

This makes `scripts/check.sh` a non-mutating whole-repo validation gate and ensures formatting drift fails validation instead of being auto-corrected.

## UI validation coverage alignment
Phase 76.5 adds UI validation commands to `scripts/check.sh`:
- `npm run typecheck`
- `npm run lint`
- `npm run build`

This aligns local validation coverage with expected whole-repo validation semantics.

## Structure/docs validator split
`validate_structure.py` and `validate_docs.py` intentionally overlap and serve different enforcement layers:
- `validate_structure.py` handles repository shape, required placement, required frontmatter presence, and truth-dimension placement compatibility.
- `validate_docs.py` handles exact frontmatter values and truth-surface contamination checks.

## Operations frontmatter convention
`docs/operations/*` uses `mutation_path: readme_update` as the enforced location convention, and `validate_docs.py` expects that convention.

## Relationship to Phase 77
Phase 76.5 is an out-of-band maintenance fix before Phase 77.

Phase 77 remains responsible for UI operator intent submission wiring.

Phase 76.5 does not change Phase 77 scope and does not renumber roadmap phases.

## Validation evidence
Validation evidence is recorded via:
- `./scripts/check.sh`
- explicit UI parity rerun (`npm run typecheck`, `npm run lint`, `npm run build`)
- explicit Rust/UI boundary lint self-tests and production lint runs
- content scans confirming non-mutation and coverage alignment

## Confirmed vs suspected
### Confirmed
- `scripts/check.sh` is non-mutating with respect to Rust formatting.
- UI typecheck/lint/build are included in local gate execution.
- Rust boundary lint self-tests run before production Rust boundary lint.
- UI AST lint self-tests run before production UI AST lint.
- Validator split and operations frontmatter convention are documented.
- Phase 76.5 is an out-of-band maintenance fix before Phase 77.

### Suspected
- No additional suspected findings were introduced beyond normal future maintenance needs.

## Non-readiness statement
Phase 76.5 is maintenance-only and does not claim release-candidate readiness, production readiness, or public usability.
