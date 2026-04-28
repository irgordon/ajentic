# Contributing to AJENTIC

## v0.0.0 bootstrap phase

**Do not add runtime behavior during the bootstrap phase.**

This phase creates structure, documentation, schemas, and scaffolding only.

## Where to add future work

| Component | Location |
|-----------|----------|
| Rust core logic | `core/src/` |
| Rust CLI commands | `cli/src/commands/` |
| Python adapters | `adapters/python/ajentic_adapter/providers/` |
| Python advisory evaluators | `adapters/python/ajentic_adapter/evaluators/` |
| TypeScript UI views | `ui/src/` |
| Schemas | `schemas/` |
| Examples | `examples/` |
| Documentation | `docs/` |

## Language boundaries

- **Rust** owns all authoritative behavior: lifecycle, governance, ledger, replay, and promotion.
- **Python** is limited to non-authoritative adapters and advisory evaluators.
- **TypeScript** is limited to visibility and review surfaces.
- **Bash** is limited to thin glue scripts.

## Governance rules

See [GOVERNANCE.md](GOVERNANCE.md) for the full governance model.

## Running checks

```sh
./scripts/bootstrap.sh
./scripts/check.sh
cargo check --workspace
```
