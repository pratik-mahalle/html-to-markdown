#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

echo "=== Vendoring Elixir dependencies ==="

# First stage the Rust core crate (copies html-to-markdown to vendor/)
echo "Step 1: Staging Rust core crate..."
"$SCRIPT_DIR/stage-rust-core.sh"

# Now vendor all transitive dependencies using cargo vendor
echo ""
echo "Step 2: Vendoring all transitive dependencies..."
REPO_ROOT="$SCRIPT_DIR/../../.."
NATIVE_DIR="$REPO_ROOT/packages/elixir/native/html_to_markdown_elixir"

cd "$NATIVE_DIR"

# Save existing config.toml
if [ -f .cargo/config.toml ]; then
	cp .cargo/config.toml .cargo/config.toml.backup
fi

# Run cargo vendor to vendor all dependencies
echo "Running cargo vendor..."
cargo vendor vendor | sed 's|directory = ".*|directory = "vendor"|' >.cargo/config.toml.vendor

# Merge with backup config.toml (which has macOS-specific rustflags)
if [ -f .cargo/config.toml.backup ]; then
	cat .cargo/config.toml.backup .cargo/config.toml.vendor >.cargo/config.toml
	rm .cargo/config.toml.backup .cargo/config.toml.vendor
else
	mv .cargo/config.toml.vendor .cargo/config.toml
fi

echo "✓ Vendored all dependencies to vendor/"

# Count vendored crates
crate_count=$(find vendor -maxdepth 1 -type d 2>/dev/null | wc -l)
echo "✓ Vendored $((crate_count - 1)) crates"
