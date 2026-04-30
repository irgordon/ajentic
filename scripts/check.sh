#!/bin/sh
set -eu

echo "AJENTIC check: running cargo fmt --all -- --check"
cargo fmt --all -- --check

echo "AJENTIC check: running cargo clippy --all-targets -- -D warnings"
cargo clippy --all-targets -- -D warnings

echo "AJENTIC check: running cargo test --all with warnings denied"
RUSTFLAGS="-D warnings" cargo test --all

echo "AJENTIC check: running cargo check --workspace with warnings denied"
RUSTFLAGS="-D warnings" cargo check --workspace

echo "AJENTIC check: passed"
