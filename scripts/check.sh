#!/bin/sh
set -eu

echo "AJENTIC check: running cargo check --workspace"
cargo check --workspace

if command -v rustfmt > /dev/null 2>&1; then
  echo "AJENTIC check: running cargo fmt --all -- --check"
  cargo fmt --all -- --check
else
  echo "AJENTIC check: rustfmt not found, skipping format check"
fi

echo "AJENTIC check: passed"
