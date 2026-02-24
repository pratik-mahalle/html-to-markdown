#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$SCRIPT_DIR/../../.."

# Vendor all dependencies (stages core crate + vendors transitive deps + creates vendor.tar.xz)
"$SCRIPT_DIR/vendor-dependencies.sh"

# Build the R source package
echo ""
echo "=== Building R CRAN source package ==="
cd "$REPO_ROOT/packages/r"
R CMD build .

echo ""
echo "=== Build complete ==="
ls -lh htmltomarkdown_*.tar.gz
