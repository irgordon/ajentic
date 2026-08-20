---
truth_dimension: orientation
authority_level: advisory
mutation_path: readme_update
---

# Phase 197.2 Task Outcome Evidence-Binding Evidence

## Baseline and branch

- Starting commit: `73315721ce4986dfb93c0673c174cfdb6dcfd0de`.
- Starting subject: `fix(core): Derive evidence from verifiers`.
- Branch: `mothra/phase-197-2-outcome-evidence-binding`.

## Original abstraction mismatch

`ActionOutcomeInput` previously supplied `PostconditionCheck.required` and `satisfied_criterion_ids`. The action therefore decided both which observations were mandatory and which TaskContract success criteria it had satisfied. TaskContract defined criteria and postconditions independently without an explicit mapping.

## TaskContract mapping

`SuccessCriterion.required_postcondition_ids` now explicitly names the exact TaskContract postconditions required for that criterion. TaskContract construction rejects:

- duplicate criterion IDs
- duplicate postcondition IDs
- empty criterion mappings
- empty mapped IDs
- duplicate mapped IDs
- unknown postcondition references
- required criteria mapped to optional postconditions
- required postconditions not covered by a required criterion

Requiredness comes exclusively from `PostconditionRequirement.required` in TaskContract.

## Removed caller authority

- Removed caller-controlled postcondition requiredness.
- Removed caller-authored satisfied criterion IDs.
- Replaced `PostconditionCheck` with `PostconditionObservation`.
- Added no replacement boolean or public criterion-success constructor.
- `ActionOutcome`, `PostconditionResult`, and `CriterionOutcome` store derived authority fields privately.

## Derived completion path

```text
TaskContract criterion mapping
→ recorded PostconditionObservation
→ exact postcondition lookup
→ evidence-reference and digest validation
→ Rust-derived PostconditionResult
→ deterministic multi-action aggregation
→ Rust-derived CriterionOutcome
→ AuthoritativeRunResult
```

An `OBSERVED` postcondition requires a non-empty observed value, evidence references, evidence digests, inclusion in the action evidence envelope, and a SHA-256 digest matching the observed value. A bare observation state does not bypass those checks.

All postconditions mapped to a criterion must be satisfied. Optional postconditions cannot replace required postconditions. Observations from multiple actions may collectively satisfy a criterion. The highest retry index resolves earlier observations; conflicting observations at the same retry index remain `UNKNOWN`.

## Schema changes

- `schemas/tasks/task-contract.schema.json` adds required, non-empty, unique `required_postcondition_ids` mappings.
- `schemas/traces/action-outcome.schema.json` replaces caller-required checks with `postcondition_observations`, removes requiredness, and adds per-observation evidence digests.
- No generated TypeScript artifacts were present or regenerated. TypeScript remains visibility only.

## Regression evidence

- TaskContract structural validation covers all required duplicate, missing, unknown, optional, and unbound mapping failures.
- Outcome tests cover unknown/substituted IDs, duplicate observations, missing evidence/value, failed/unknown/not-checked results, derived criteria, deterministic results, prior errors, and partial side effects.
- `tests/outcome_evidence_binding.rs` proves that A without B cannot satisfy criterion C, while separate evidence-bearing actions for A and B can collectively satisfy it.
- Source-lint tripwires reject reintroduction of `satisfied_criterion_ids` and `PostconditionCheck`; Rust types and derivation remain the actual authority control.

Focused and inherited Rust validation passes `1,264` tests; the outcome evidence-binding integration suite passes `7/7`; UI API behavior validation passes `141/141`; Rust boundary lint self-tests pass `28/28`; the full clean-tree repository wrapper passes with `All checks passed.`.

## Bounded limitations

Phase 197.2 proves exact TaskContract identity, TaskContract-owned requiredness, recorded evidence binding, and deterministic criterion derivation. It does not prove arbitrary real-world truth, semantic truth of free-form observations, or external system correctness not represented by the current observation contract.

## Boundary confirmation

- Phase 197.1 validation and policy verifier behavior was not weakened.
- No real provider or external action execution was activated.
- No promotion, approval, persistence, replay, recovery, memory, context, UI, workflow, package, deployment, signing, installer, update-channel, tag, or release authority changed.
- ClaimEvidence and ClaimReport were not redesigned.
