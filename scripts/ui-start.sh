#!/usr/bin/env bash
set -euo pipefail

# ui-start.sh - Start the browser UI development server.
# Does not implement UI authority. Delegates to npm.

echo "Starting browser UI (placeholder)..."
cd ui
npm run build
