#!/usr/bin/env bash
set -euo pipefail

# memory-clear-ephemeral.sh - Request ephemeral memory clear through the Rust CLI.
# Does not directly mutate persistent memory. Delegates to Rust.

echo "Ephemeral memory clear invocation (placeholder)..."
cargo run --manifest-path core/Cargo.toml -- memory clear-ephemeral "$@"
