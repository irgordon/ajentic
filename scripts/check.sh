#!/usr/bin/env bash
set -euo pipefail

# check.sh - Run deterministic non-mutating local validation checks.
# This script does not implement policy or mutate authoritative runtime state.

echo "Running bootstrap idempotence check..."
python3 scripts/bootstrap_repo.py > /dev/null
echo "  bootstrap OK"

echo "Running Python compile checks..."
python3 -m compileall scripts/bootstrap_repo.py scripts/validate_structure.py scripts/validate_docs.py > /dev/null
echo "  Python OK"

echo "Running repository structure validation..."
python3 scripts/validate_structure.py
echo "  structure OK"

echo "Running documentation boundary validation..."
python3 scripts/validate_docs.py
echo "  docs OK"

echo "Running schema validation..."
find schemas -type f -name "*.json" -print0 | xargs -0 -n1 python3 -m json.tool > /dev/null
echo "  schemas OK"

echo "Running shell script parse check..."
find scripts -type f -name "*.sh" -print0 | xargs -0 -n1 bash -n
echo "  scripts OK"

echo "Running Rust boundary lint self-tests..."
node scripts/test_rust_boundary_lint.mjs
echo "  Rust boundary lint self-tests OK"

echo "Running Rust boundary lint..."
node scripts/rust_boundary_lint.mjs
echo "  Rust boundary lint OK"

echo "Running UI boundary AST lint self-tests..."
node scripts/test_lint_ui_boundaries.mjs
echo "  UI boundary lint self-tests OK"

echo "Running UI boundary AST lint..."
node scripts/lint_ui_boundaries.mjs
echo "  UI boundary lint OK"

echo "Running UI validation..."
(
  cd ui
  npm run typecheck
  npm run lint
  npm run build
)
echo "  UI validation OK"

echo "Running Rust formatting check..."
cargo fmt --manifest-path core/Cargo.toml -- --check
echo "  Rust formatting OK"

echo "Running Rust checks..."
cargo check --manifest-path core/Cargo.toml --all-targets
cargo test --manifest-path core/Cargo.toml --all-targets
cargo clippy --manifest-path core/Cargo.toml --all-targets -- -D warnings
echo "  Rust OK"

echo "All checks passed."
