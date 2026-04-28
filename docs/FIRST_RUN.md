# First Run

This file reserves the location for the AJENTIC first-run guide.

## v0.0.0 first run

```sh
./scripts/bootstrap.sh
./scripts/check.sh
cargo check --workspace
```

## What these commands do

- `bootstrap.sh` — Prints a bootstrap message. No installation occurs in v0.0.0.
- `check.sh` — Runs `cargo check --workspace` and optionally `cargo fmt --all -- --check`.
- `cargo check --workspace` — Verifies the Rust workspace compiles.

## What is not available in v0.0.0

- No model calls
- No governance engine
- No evaluators
- No ledger
- No replay
- No UI dashboard

## v0.0.0 status

This file reserves the location for the first-run guide. A full first-run experience will be added in later phases.
