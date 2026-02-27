#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

echo "=== Staging Elixir native dependencies ==="

# Stage the Rust core crate source into vendor/html-to-markdown-rs.
# Transitive dependencies are NOT vendored — Cargo fetches them from
# crates.io when the NIF is compiled during package installation.
# This keeps the Hex tarball well under the 16 MB compressed size limit.
echo "Staging Rust core crate..."
"$SCRIPT_DIR/stage-rust-core.sh"

REPO_ROOT="$SCRIPT_DIR/../../.."
VENDOR_DIR="$REPO_ROOT/packages/elixir/native/html_to_markdown_elixir/vendor"

echo "Package size:"
du -sh "$VENDOR_DIR" 2>/dev/null || true
