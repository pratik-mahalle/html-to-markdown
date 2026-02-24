#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$SCRIPT_DIR/../../.."
R_PKG="$REPO_ROOT/packages/r"

# Build CRAN source package
"$SCRIPT_DIR/build-cran-package.sh"

# Run R CMD check --as-cran
echo ""
echo "=== Running R CMD check --as-cran ==="
cd "$R_PKG"
tarball=$(find . -maxdepth 1 -name 'htmltomarkdown_*.tar.gz' -print -quit)
R CMD check --as-cran "$tarball"

echo ""
echo "=== CRAN Submission ==="
echo "The source package is ready for manual CRAN submission:"
echo "  Package: $R_PKG/$tarball"
echo ""
echo "Submit at: https://cran.r-project.org/submit.html"
echo ""
echo "Before submitting, ensure:"
echo "  1. R CMD check --as-cran passes with no ERRORs or WARNINGs"
echo "  2. cran-comments.md is up to date"
echo "  3. NEWS.md documents changes since last CRAN release"
