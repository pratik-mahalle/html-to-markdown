#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$SCRIPT_DIR/../../.."
RUBY_PKG="$REPO_ROOT/packages/ruby"
NATIVE_EXT="$RUBY_PKG/ext/html-to-markdown-rb/native"
VENDOR_DIR="rust-vendor"
CORE_CRATE="$REPO_ROOT/crates/html-to-markdown"

echo "=== Vendoring Ruby gem dependencies with cargo vendor ==="

cd "$NATIVE_EXT"

# Clean up any existing vendor directory and restore Cargo.toml
rm -rf "${RUBY_PKG:?}/${VENDOR_DIR:?}" "${RUBY_PKG:?}/.cargo"
git restore "$NATIVE_EXT/Cargo.toml" 2>/dev/null || true

# Step 1: Run cargo vendor to vendor all external dependencies
# cargo vendor outputs the config.toml content to stdout, progress to stderr
mkdir -p "$RUBY_PKG/.cargo"
echo "Running cargo vendor..."
cargo vendor "$RUBY_PKG/$VENDOR_DIR" | sed "s|directory = \".*|directory = \"$VENDOR_DIR\"|" >"$RUBY_PKG/.cargo/config.toml"

# Step 2: Copy html-to-markdown-rs core crate to vendor directory
echo "Copying html-to-markdown-rs core crate..."

if command -v rsync >/dev/null 2>&1; then
	rsync -a --exclude target --exclude .git "$CORE_CRATE/" "$RUBY_PKG/$VENDOR_DIR/html-to-markdown-rs/"
else
	cp -R "$CORE_CRATE" "$RUBY_PKG/$VENDOR_DIR/html-to-markdown-rs"
	rm -rf "$RUBY_PKG/$VENDOR_DIR/html-to-markdown-rs/target" "$RUBY_PKG/$VENDOR_DIR/html-to-markdown-rs/.git" || true
fi

# Step 3: Expand workspace references in core crate Cargo.toml
echo "Expanding workspace references in html-to-markdown-rs..."
python3 - "$RUBY_PKG/$VENDOR_DIR/html-to-markdown-rs/Cargo.toml" <<'PY'
import re
import sys
from pathlib import Path

path = Path(sys.argv[1])
text = path.read_text(encoding="utf-8")

replacements = {
    r"^version\.workspace\s*=\s*true\s*$": 'version = "2.23.0"',
    r"^edition\.workspace\s*=\s*true\s*$": 'edition = "2024"',
    r"^authors\.workspace\s*=\s*true\s*$": 'authors = ["Na\'aman Hirschfeld <naaman@kreuzberg.dev>"]',
    r"^license\.workspace\s*=\s*true\s*$": 'license = "MIT"',
    r"^repository\.workspace\s*=\s*true\s*$": 'repository = "https://github.com/kreuzberg-dev/html-to-markdown"',
    r"^homepage\.workspace\s*=\s*true\s*$": 'homepage = "https://github.com/kreuzberg-dev/html-to-markdown"',
    r"^documentation\.workspace\s*=\s*true\s*$": 'documentation = "https://docs.rs/html-to-markdown-rs"',
    r"^readme\.workspace\s*=\s*true\s*$": 'readme = "README.md"',
    r"^rust-version\.workspace\s*=\s*true\s*$": 'rust-version = "1.85"',
    r"^tl\.workspace\s*=\s*true\s*$": 'tl = { package = "astral-tl", version = "0.7.11" }',
    r"^regex\.workspace\s*=\s*true\s*$": 'regex = "1.12"',
    r"^once_cell\.workspace\s*=\s*true\s*$": 'once_cell = "1.21"',
    r"^thiserror\.workspace\s*=\s*true\s*$": 'thiserror = "2.0"',
    r"^base64\.workspace\s*=\s*true\s*$": 'base64 = "0.22"',
    r"^html5ever\.workspace\s*=\s*true\s*$": 'html5ever = "0.36"',
    r"^markup5ever_rcdom\.workspace\s*=\s*true\s*$": 'markup5ever_rcdom = "0.36"',
    r"^async-trait\s*=\s*\{\s*workspace\s*=\s*true,\s*optional\s*=\s*true\s*\}\s*$": 'async-trait = { version = "0.1", optional = true }',
}

for pattern, replacement in replacements.items():
    text = re.sub(pattern, replacement, text, flags=re.MULTILINE)

# Handle lints section (multi-line) - remove workspace = true
text = re.sub(
    r"\[lints\]\s*\nworkspace\s*=\s*true\s*",
    '',
    text,
    flags=re.MULTILINE
)

path.write_text(text, encoding="utf-8")
PY

# Step 4: Create .cargo-checksum.json for html-to-markdown-rs
echo "Creating checksum for html-to-markdown-rs..."
echo '{"files":{}}' >"$RUBY_PKG/$VENDOR_DIR/html-to-markdown-rs/.cargo-checksum.json"

# Step 5: Add path override to .cargo/config.toml for html-to-markdown-rs
echo "Adding path override for html-to-markdown-rs..."
cat >>"$RUBY_PKG/.cargo/config.toml" <<EOF

[patch.crates-io]
html-to-markdown-rs = { path = "$VENDOR_DIR/html-to-markdown-rs" }
EOF

echo "✓ Vendored all dependencies to packages/ruby/$VENDOR_DIR/"
echo "✓ Created .cargo/config.toml with source replacements"

# Count vendored crates
crate_count=$(find "$RUBY_PKG/$VENDOR_DIR" -maxdepth 1 -type d 2>/dev/null | wc -l)
echo "✓ Vendored $((crate_count - 1)) crates"
