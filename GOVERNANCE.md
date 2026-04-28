# AJENTIC Governance Rules

## Core principle

Generated output is untrusted by default.

Only Rust may make authoritative lifecycle, governance, ledger, replay, and promotion decisions.
Python, TypeScript, Bash, and future Go wrappers are non-authoritative.

## Result handling

UNKNOWN, missing, malformed, incomplete, or indeterminate results must **not** be treated as PASS.

Any result that cannot be positively confirmed as passing is treated as failing.

## Prohibited bypass flags

The following flags are explicitly prohibited and must never be implemented:

- `--force-promote`
- `--skip-policy`
- `--trust-adapter-output`
- `--ignore-unknown`
- `--promote-anyway`

## Tier-1 definition

A Tier-1 output means **reviewable**. It does not mean automatically production-approved.

Tier-1 status indicates that a candidate solution has passed governance, safety, quality, and performance thresholds. It remains subject to human or downstream review before production use.

## Non-authoritative components

- Python adapters must not promote candidates.
- Python adapters must not write ledger records.
- Python adapters must not enforce governance.
- TypeScript UI must not compute authoritative promotion eligibility.
- Bash scripts must not encode governance, policy, promotion, replay, or lifecycle decisions.

## v0.0.0 status

No governance logic is implemented in this version. This file reserves governance rules for future implementation.
