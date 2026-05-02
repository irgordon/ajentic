---
truth_dimension: structural
authority_level: authoritative
mutation_path: architecture_pr
---

# Browser UI

This document describes the structural role of the browser UI in AJENTIC.

This document is subordinate to `docs/architecture/ARCHITECTURE.md`.

This document does not define governance rules, implementation status, roadmap progress, or release history.

## Role

The TypeScript browser UI is a review and operator-intent surface.

It displays state projections, context projections, memory projections, policy and validation results, run timelines, replay reports, and audit projections.

It submits typed operator intents only.

## Authority boundary

The UI must not directly mutate authoritative state, write ledger events, decide policy, or call model providers directly.

Validated UI data is renderable data, not authoritative state.

## Location

UI source: `ui/`

See `docs/architecture/ARCHITECTURE.md` and `docs/governance/GOVERNANCE.md` for the full authority boundary description.
