# Adapter Protocol

This file reserves the location for the AJENTIC adapter protocol documentation.

## Purpose

The adapter protocol defines how non-authoritative Python adapters communicate with the Rust harness. Adapters provide inputs; the harness makes decisions.

## Roles

- **Harness (Rust):** Sends requests, receives responses, normalizes results, applies governance.
- **Adapter (Python):** Receives requests, calls providers, returns raw results.

## Constraints

- Adapters must not promote candidates.
- Adapters must not write ledger records.
- Adapters must not enforce governance.
- Adapters must return results in the schema defined by `schemas/adapter_response.schema.json`.

## v0.0.0 status

This file reserves the location for the adapter protocol. Implementation will be added in later phases.
