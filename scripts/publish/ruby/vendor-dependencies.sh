#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$SCRIPT_DIR/../../.."
RUBY_PKG="$REPO_ROOT/packages/ruby"
NATIVE_EXT="$RUBY_PKG/ext/html-to-markdown-rb/native"

echo "=== Vendoring Ruby gem dependencies with cargo vendor ==="

cd "$NATIVE_EXT"

# Run cargo vendor to vendor all dependencies
# cargo vendor outputs the config.toml content to stdout, progress to stderr
mkdir -p "$RUBY_PKG/.cargo"
echo "Running cargo vendor..."
cargo vendor "$RUBY_PKG/vendor" | sed 's|directory = ".*|directory = "vendor"|' >"$RUBY_PKG/.cargo/config.toml"

echo "✓ Vendored all dependencies to packages/ruby/vendor/"
echo "✓ Created .cargo/config.toml with source replacements"

# Count vendored crates
crate_count=$(find "$RUBY_PKG/vendor" -maxdepth 1 -type d | wc -l)
echo "✓ Vendored $((crate_count - 1)) crates"
