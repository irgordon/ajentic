---
truth_dimension: orientation
authority_level: advisory
mutation_path: readme_update
---

# Operator handbook

This document provides orientation for operators running AJENTIC locally.

This document is non-authoritative. It does not define governance rules, architecture authority, or runtime behavior.

## Overview

AJENTIC is a deterministic control layer for model-driven work.

Operators use the browser UI to review harness runs, submit typed intents, and inspect context, memory, and audit projections.

Operators use the Bash scripts in `scripts/` as convenience wrappers around the Rust CLI.

## Scripts

| Script | Purpose |
| --- | --- |
| `scripts/check.sh` | Run local validation checks |
| `scripts/dev-run.sh` | Start the Rust core in development mode |
| `scripts/replay.sh` | Request a replay run |
| `scripts/memory-snapshot.sh` | Request a memory snapshot |
| `scripts/memory-clear-ephemeral.sh` | Clear ephemeral memory |
| `scripts/ui-start.sh` | Start the browser UI |
| `scripts/bootstrap_repo.py` | Recreate the Phase 0 skeleton idempotently |

## Status

This handbook is a placeholder for Phase 0. Operator procedures will be elaborated in future phases.
