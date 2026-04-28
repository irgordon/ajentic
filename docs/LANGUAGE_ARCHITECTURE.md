# Language Architecture

This file reserves the location for the AJENTIC language architecture documentation.

## Language ownership model

| Language   | Role        | Authority |
|------------|-------------|-----------|
| Rust       | authority   | Authoritative harness behavior, lifecycle, governance, ledger, replay, promotion |
| Python     | adaptation  | Non-authoritative adapters and advisory evaluators |
| TypeScript | visibility  | Non-authoritative UI and review surface |
| Bash       | glue        | Thin POSIX shell scripts only |
| Go         | optional    | Not part of v0.0.0 |

## Rust authority

Rust owns all authoritative decisions once implementation begins. No other language may override or bypass Rust governance decisions.

## Non-authoritative boundaries

Python adapters, TypeScript UI components, and Bash scripts must not contain:
- Promotion logic
- Policy authority
- Lifecycle transition decisions
- Ledger integrity rules
- Replay logic

## v0.0.0 status

This component is non-authoritative in v0.0.0. Implementation will be added in later phases.
