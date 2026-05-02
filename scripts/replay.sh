#!/usr/bin/env bash
set -euo pipefail

# replay.sh - Request a replay run through the Rust CLI.
# Does not implement replay logic. Delegates to Rust.

echo "Replay invocation (placeholder)..."
cargo run --manifest-path core/Cargo.toml -- replay "$@"
