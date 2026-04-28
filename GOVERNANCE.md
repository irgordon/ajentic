# AJENTIC Governance Rules

## Core principle

Generated output is untrusted by default.

Rust is authoritative for lifecycle, governance, ledger, replay, and promotion authority.
Python, TypeScript, and Bash are non-authoritative.

## Result handling

UNKNOWN is not PASS.

Any UNKNOWN, missing, malformed, incomplete, or indeterminate result must not be treated as PASS.

## Phase 1 boundary note

Schemas and Rust contract types define boundary shape only; by themselves they do not enforce behavior.
Validation behavior and enforcement are intentionally deferred to later phases.

## Prohibited bypass flags

The following flags are prohibited:

- `--force-promote`
- `--skip-policy`
- `--trust-adapter-output`
- `--ignore-unknown`
- `--promote-anyway`
