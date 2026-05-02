---
truth_dimension: structural
authority_level: authoritative
mutation_path: architecture_pr
---

# Ledger and replay model

This document describes the structural role of the ledger and replay model in AJENTIC.

This document is subordinate to `ARCHITECTURE.md`.

This document does not define governance rules, implementation status, roadmap progress, or release history.

## Role

The ledger records accepted authoritative events.

Replay reconstructs runs from ledger entries and recorded inputs.

Replay must be deterministic and must fail closed on missing or malformed inputs.

## Location

Ledger module: `core/src/ledger/`

Replay module: `core/src/replay/`

Audit module: `core/src/audit/`

Ledger schemas: `schemas/events/`

Trace schemas: `schemas/traces/`

This is a placeholder subdocument. The ledger and replay design will be elaborated in a future phase.
