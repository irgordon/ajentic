# First Run

This file provides a minimal first-run guide for the current static CLI surface.

## v0.3.0 first run

```sh
./scripts/bootstrap.sh
./scripts/check.sh
cargo check --workspace
cargo run -p ajentic-cli -- validate examples/minimal_run
cargo run -p ajentic-cli -- inspect examples/minimal_run
```

## What these commands do

- `bootstrap.sh` — Prints a bootstrap message.
- `check.sh` — Runs `cargo check --workspace` and optionally `cargo fmt --all -- --check`.
- `cargo check --workspace` — Verifies the Rust workspace compiles.
- `cargo run -p ajentic-cli -- validate examples/minimal_run` — Runs static run directory checks.
- `cargo run -p ajentic-cli -- inspect examples/minimal_run` — Prints static run file summary and byte lengths.

## Phase 3 scope note

- This phase adds static run directory checks only.
- YAML parsing and schema validation are reserved for later phases.
- Static validation does not prove governance validity or candidate correctness.
