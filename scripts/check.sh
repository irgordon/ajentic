#!/usr/bin/env bash
set -euo pipefail

# check.sh - Run local validation checks.
# Calls Rust CLI and other deterministic checks.
# This script does not implement policy or mutate authoritative state.

echo "Running schema validation..."
find schemas -type f -name "*.json" -print0 | xargs -0 -n1 python3 -m json.tool > /dev/null
echo "  schemas OK"

echo "Running shell script parse check..."
find scripts -type f -name "*.sh" -print0 | xargs -0 -n1 bash -n
echo "  scripts OK"

echo "Running Rust check..."
cargo check --manifest-path core/Cargo.toml
echo "  Rust OK"

echo "All checks passed."
