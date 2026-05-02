#!/usr/bin/env bash
set -euo pipefail

# dev-run.sh - Start the AJENTIC core in development mode.
# Delegates to the Rust CLI. Does not implement policy.

echo "Starting ajentic core (placeholder)..."
cargo run --manifest-path core/Cargo.toml
