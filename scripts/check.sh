#!/usr/bin/env bash
set -euo pipefail

# check.sh - Run deterministic non-mutating local validation checks.
# This script does not implement policy or mutate authoritative runtime state.
#
# CARGO_TARGET_DIR defaults outside the repository to prevent Cargo-generated
# target metadata from creating tracked working-tree drift during validation.

export CARGO_TARGET_DIR="${CARGO_TARGET_DIR:-/tmp/ajentic-check-target}"

require_clean_worktree() {
  local label="$1"

  if ! git diff --quiet -- .; then
    echo "::error::Working tree changed during ${label}."
    git diff --stat -- .
    exit 1
  fi

  if ! git diff --cached --quiet -- .; then
    echo "::error::Index changed during ${label}."
    git diff --cached --stat -- .
    exit 1
  fi
}

echo "Checking initial repository cleanliness..."
require_clean_worktree "initial validation setup"
echo "  repository clean"

echo "Running bootstrap idempotence check..."
PYTHONDONTWRITEBYTECODE=1 python3 scripts/bootstrap_repo.py > /dev/null
require_clean_worktree "bootstrap idempotence check"
echo "  bootstrap OK"

echo "Running Python compile checks..."
PYTHONDONTWRITEBYTECODE=1 python3 -m py_compile \
  scripts/bootstrap_repo.py \
  scripts/validate_structure.py \
  scripts/validate_docs.py
require_clean_worktree "Python compile checks"
echo "  Python OK"

echo "Running repository structure validation..."
PYTHONDONTWRITEBYTECODE=1 python3 scripts/validate_structure.py
echo "  structure OK"

echo "Running documentation boundary validation..."
PYTHONDONTWRITEBYTECODE=1 python3 scripts/validate_docs.py
echo "  docs OK"

echo "Running schema validation..."
while IFS= read -r -d "" schema_file; do
  python3 -m json.tool "$schema_file" > /dev/null
done < <(find schemas -type f -name "*.json" -print0)
echo "  schemas OK"

echo "Running shell script parse check..."
while IFS= read -r -d "" shell_file; do
  bash -n "$shell_file"
done < <(find scripts -type f -name "*.sh" -print0)
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
  rm -rf dist
  npm run test:api
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

echo "Checking final repository cleanliness..."
require_clean_worktree "final validation"
echo "  repository clean"

echo "All checks passed."
