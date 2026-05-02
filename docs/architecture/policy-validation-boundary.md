---
truth_dimension: structural
authority_level: authoritative
mutation_path: architecture_pr
---

# Policy and validation boundary

This document describes the structural role of the policy and validation boundary in AJENTIC.

This document is subordinate to `docs/architecture/ARCHITECTURE.md`.

This document does not define governance rules, implementation status, roadmap progress, or release history.

## Role

The Rust core owns all policy and validation decisions.

No TypeScript, Bash, workflow, model output, or documentation may create policy or validation authority.

Validation must be deterministic where it creates authority.

## Location

Policy module: `core/src/policy/`

Validation module: `core/src/validation/`

This is a placeholder subdocument. The policy and validation design will be elaborated in a future phase.
