---
truth_dimension: structural
authority_level: authoritative
mutation_path: architecture_pr
---

# State machine

This document describes the structural role of the state machine in AJENTIC.

This document is subordinate to `docs/architecture/ARCHITECTURE.md`.

This document does not define governance rules, implementation status, roadmap progress, or release history.

## Role

The Rust core owns all authoritative state transitions.

State transitions are recorded as typed ledger events where required.

## Location

State module: `core/src/state/`

This is a placeholder subdocument. The state machine design will be elaborated in a future phase.
