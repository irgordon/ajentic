# Ledger Rules

This file reserves the location for the AJENTIC ledger rules documentation.

## Purpose

The ledger records all lifecycle events for auditability and replay. Every governance decision, lifecycle transition, and promotion decision is recorded.

## Integrity rules

- Ledger records are append-only.
- Ledger integrity is the exclusive responsibility of Rust core.
- No adapter or UI component may write to the ledger directly.
- Ledger records must reference the schema defined in `schemas/audit_record.schema.json`.

## v0.0.0 status

This file reserves the location for ledger rules. Implementation will be added in later phases.
