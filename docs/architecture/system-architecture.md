---
truth_dimension: structural
authority_level: authoritative
mutation_path: architecture_pr
---

# System architecture

This document describes the high-level system structure of AJENTIC.

This document is subordinate to `docs/architecture/ARCHITECTURE.md`.

This document does not define governance rules, implementation status, roadmap progress, or release history.

## Purpose

AJENTIC is a deterministic control layer for model-driven work.

This document describes the component boundaries and authority flow at a structural level.

## Components

| Component | Location | Role |
| --- | --- | --- |
| Rust core | `core/` | Authoritative runtime. Owns policy, validation, state, memory, ledger, replay, audit, API. |
| TypeScript browser UI | `ui/` | Review and operator-intent surface. Display only. |
| Bash scripts | `scripts/` | Operator convenience wrappers around Rust CLI. |
| JSON Schema | `schemas/` | Shared contract definitions. |
| GitHub workflows | `.github/workflows/` | Enforcement wiring. |
| Memory | `memory/` | Governed data storage. |

## Authority flow

Authority flows from typed intents through Rust-owned API and CLI surfaces.

No surface outside Rust may create runtime authority.

See `docs/architecture/ARCHITECTURE.md` for the full structural description.
