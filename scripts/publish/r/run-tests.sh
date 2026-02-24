#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$SCRIPT_DIR/../../.."

# Stage the Rust core crate
"$SCRIPT_DIR/stage-rust-core.sh"

# Install the R package
echo "Installing R package..."
R CMD INSTALL "$REPO_ROOT/packages/r"

# Run tests
echo "Running tests..."
cd "$REPO_ROOT/packages/r"
Rscript -e 'devtools::test()'
