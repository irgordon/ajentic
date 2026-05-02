---
truth_dimension: structural
authority_level: authoritative
mutation_path: architecture_pr
---

# Memory model

This document describes the structural role of the memory model in AJENTIC.

This document is subordinate to `ARCHITECTURE.md`.

This document does not define governance rules, implementation status, roadmap progress, or release history.

## Role

Memory is governed data.

Persistent memory is typed, versioned, and provenance-bearing.

Ephemeral memory is run-scoped and must not be committed.

Memory content may be used only after validation through Rust-owned paths.

## Location

Memory module: `core/src/memory/`

Governed memory data: `memory/`

Memory schemas: `schemas/memory/`

This is a placeholder subdocument. The memory model design will be elaborated in a future phase.
