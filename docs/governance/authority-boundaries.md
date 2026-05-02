---
truth_dimension: normative
authority_level: authoritative
mutation_path: governance_pr
---

# Authority boundaries

This document defines which repository surfaces may request, enforce, record, display, or explain authority.

This document is subordinate to `docs/governance/GOVERNANCE.md`.

This document does not replace `docs/architecture/ARCHITECTURE.md`, define component structure, describe implementation status, or specify runtime workflow details.

## 1. Boundary roles

| Role | Meaning |
| --- | --- |
| Define | Establish a rule, contract, or structural description within the artifact's scope. |
| Enforce | Check or reject behavior according to an existing rule or contract. |
| Request | Submit a typed request for Rust-owned validation. |
| Record | Persist an accepted authoritative event. |
| Display | Render a projection or report. |
| Explain | Help humans understand a concept without creating authority. |

A surface must not exceed its allowed role.

## 2. Authority role table

| Surface | Allowed role | Boundary |
| --- | --- | --- |
| `docs/governance/GOVERNANCE.md` | Define | Defines top-level normative truth. |
| `docs/governance/` | Define | Defines subordinate normative rules consistent with `docs/governance/GOVERNANCE.md`. |
| `docs/architecture/ARCHITECTURE.md` | Define | Defines top-level structural truth. |
| `docs/architecture/` | Define | Defines subordinate structural descriptions consistent with `docs/architecture/ARCHITECTURE.md`. |
| `schemas/` | Define | Defines contract truth. |
| `core/` | Enforce, record | Enforces runtime authority and records accepted events. |
| `tests/` | Enforce | Verifies executable behavior. |
| `ui/` | Request, display | Displays projections and submits typed intents. |
| `scripts/` | Request | Wraps Rust CLI commands. |
| `memory/` | Data | Stores governed data, not policy. |
| `checklists/` | Explain | Provides bounded procedural execution steps. |
| `.github/workflows/` | Enforce | Runs checks defined elsewhere. |
| `README.md` | Explain | Provides human orientation. |
| `AGENTS.md` | Explain | Provides navigation to authority sources. |
| `docs/examples/` | Explain | Provides non-authoritative examples. |

## 3. Runtime authority

Runtime authority belongs to the Rust core.

The Rust core may validate typed inputs, enforce policy and state constraints, accept or reject intents, access governed memory through typed paths, execute controlled workflows, record accepted events, and produce replay and audit projections.

Other surfaces may request or display runtime outcomes.

Other surfaces must not create runtime authority.

## 4. Prohibited boundary crossings

The following boundary crossings are prohibited:

- UI directly writing state, memory, ledger, policy, or replay data
- Bash directly editing authoritative runtime data
- workflows creating rules absent from governed sources
- memory storing policy
- examples defining required behavior
- README defining governance
- AGENTS.md becoming a full instruction manual
- TypeScript or Zod redefining schema contracts
- audit projections creating authority
- replay repairing invalid history
- model output being treated as accepted work without validation
