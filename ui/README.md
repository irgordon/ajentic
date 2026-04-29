# AJENTIC UI (Phase 12)

This UI is a **non-authoritative review surface** for AJENTIC state.

- **UI is visibility only.**
- **Rust remains authority.**
- **Adapter output is untrusted.**
- **Evaluation result ingestion is not governance approval.**
- **Required evaluator satisfaction is not promotion eligibility.**
- **Only Rust may promote to Tier-1.**
- **Tier-1 means downstream review, not production approval.**

## Scope

Phase 12 adds static UI display panels for:

- run overview
- candidate detail
- evaluation results
- governance result and policy checks
- promotion decision
- ledger summary
- audit summary
- replay summary

## Non-goals

This UI does **not**:

- compute promotion eligibility
- mutate candidate lifecycle state
- call adapters or providers
- execute evaluators
- write ledger records
- emit audit records
- perform replay
- call Rust CLI commands
- call Python
- call remote services

## Local check

This directory is dependency-free. Run:

```sh
npm --prefix ui run check
```

## View

Open `ui/src/index.html` in a browser. The UI renders static sample state only.
