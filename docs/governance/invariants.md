---
truth_dimension: normative
authority_level: authoritative
mutation_path: governance_pr
---

# Invariants

This document defines the invariants that must hold at all times in the AJENTIC system.

This document is subordinate to `docs/governance/GOVERNANCE.md`.

This document does not redefine system identity, runtime authority, roadmap progress, release history, or operational procedure.

## 1. Purpose

Invariants are conditions that must always be true regardless of implementation state or phase.

If an invariant is violated, the violation must be identified and resolved before the relevant artifact or behavior becomes authoritative.

## 2. System invariants

- Rust owns runtime authority at all times.
- Model output is untrusted until validated through Rust-owned paths.
- No TypeScript, Bash, workflow, documentation, memory entry, example, or model output may create runtime authority.
- Ephemeral memory must not be committed.
- Schema files must live under `schemas/`.
- Markdown documentation must not live under `memory/`.
- Truth dimensions must not collapse into each other.
- Each artifact type must use the correct mutation path.
- The active phase checklist is `checklists/current-phase.md`.
- Completed accepted work is recorded in `CHANGELOG.md` only.
- Future plans must not appear in `CHANGELOG.md`.
- Completed work must not appear in roadmap documents.

## 3. Validation

Invariants are enforced through CI workflows where automation is possible.

Invariants that cannot be automated must be checked during phase execution.
