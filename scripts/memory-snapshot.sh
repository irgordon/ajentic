#!/usr/bin/env bash
set -euo pipefail

# memory-snapshot.sh - Request a memory snapshot through the Rust CLI.
# Does not directly mutate memory. Delegates to Rust.

echo "Memory snapshot invocation (placeholder)..."
cargo run --manifest-path core/Cargo.toml -- memory snapshot "$@"
