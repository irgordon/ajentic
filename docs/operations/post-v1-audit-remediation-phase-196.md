---
truth_dimension: orientation
authority_level: advisory
mutation_path: readme_update
---

# Phase 196 - Post-v1 Audit Remediation and Contract Drift Guard

Phase 196 responds to the post-v1 repository audit after Phase 195.1 protected-branch closure evidence landed on mainline history.

## Scope

Phase 196 addresses audit-remediation risk only:

- stale bootstrap scaffold risk
- stale audit and migration checklist placeholder risk
- duplicated operator-intent schema/Rust/TypeScript contract drift risk
- TypeScript provider mirror wording that could be mistaken for authority
- missing validation guardrails for duplicated operator intent and target-kind models
- CLI completeness documentation without implementing full CLI behavior

## Boundary

Phase 196 preserves the repository authority model:

- Rust remains authority.
- TypeScript remains visibility and operator-intent projection.
- Python remains adaptation and validation support.
- Bash remains local operator glue.
- JSON Schema remains shared contract shape, not runtime authority outside Rust validation.

The new operator-intent contract map is review evidence only. It does not replace Rust validation, generate Rust or TypeScript code, bless drift silently, or make schema validation authoritative outside Rust-owned paths.

## Completed remediation

- Retired the obsolete bootstrap scaffold rather than updating it into a live alternate repository generator.
- Replaced Phase-0 placeholder audit and migration checklist language with current bounded procedures.
- Added `docs/contracts/operator-intent-contract-map.json` to record schema/Rust/TypeScript intent and target-kind mappings.
- Added `scripts/validate_operator_intent_contract_map.py` and wired it into `scripts/check.sh`.
- Reframed the TypeScript deterministic provider mirror as projection-only through naming, comments, and compatibility wrapping.
- Documented that the Rust CLI remains a bounded dry-run fixture and that full typed CLI completion is future-scoped.

## Known drift recorded intentionally

The contract map records lossless intent-type mappings and target-kind drift. Several schema target values are intentionally unsupported by current Rust/TypeScript operator projection surfaces, while memory/context/replay schema targets are broader lossy projections in Rust and TypeScript.

Unsupported or lossy mappings must not render as success, must not imply schema validity, and must fail closed until a later scoped phase changes the contract surface.

## Non-goals

Phase 196 does not:

- complete full schema generation
- complete full CLI behavior
- add provider or model calls
- add governance promotion behavior
- add ledger behavior
- add replay behavior
- add release or distribution mechanics
- create a tag or GitHub Release
- publish npm or Cargo packages
- create installers or update channels
- deploy anything
- add OS signing or notarization
- change backend authority
- change UI authority
- change package versions

## Remaining queued work

Future provider, CLI, schema-generation, UI, ledger, replay, and migration work must use a later explicitly scoped phase. Phase 196 only makes the drift visible and fail-closed before further expansion.
