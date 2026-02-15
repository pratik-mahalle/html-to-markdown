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

# Clean up any existing vendor directory and restore native Cargo.toml to original state
rm -rf "${RUBY_PKG:?}/${VENDOR_DIR:?}" "${RUBY_PKG:?}/.cargo" "$NATIVE_EXT/Cargo.lock"
git restore "$NATIVE_EXT/Cargo.toml" 2>/dev/null || true

# Step 1: Update local registry cache to get latest crate versions
# This ensures the vendored crates will match what cargo generate-lockfile resolves to
echo "Updating local crate registry..."
cargo metadata --format-version=1 --manifest-path="$NATIVE_EXT/Cargo.toml" >/dev/null 2>&1 || true

# Step 2: Run cargo vendor to vendor all external dependencies
# cargo vendor outputs the config.toml content to stdout, progress to stderr
mkdir -p "$RUBY_PKG/.cargo"
echo "Running cargo vendor..."
cargo vendor "$RUBY_PKG/$VENDOR_DIR" | sed "s|directory = \".*|directory = \"$VENDOR_DIR\"|" >"$RUBY_PKG/.cargo/config.toml"

# Step 3: Copy html-to-markdown-rs core crate to vendor directory
echo "Copying html-to-markdown-rs core crate..."

if command -v rsync >/dev/null 2>&1; then
	rsync -a --exclude target --exclude .git "$CORE_CRATE/" "$RUBY_PKG/$VENDOR_DIR/html-to-markdown-rs/"
else
	cp -R "$CORE_CRATE" "$RUBY_PKG/$VENDOR_DIR/html-to-markdown-rs"
	rm -rf "$RUBY_PKG/$VENDOR_DIR/html-to-markdown-rs/target" "$RUBY_PKG/$VENDOR_DIR/html-to-markdown-rs/.git" || true
fi

# Step 4: Expand workspace references in core crate Cargo.toml
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
    r"^ahash\.workspace\s*=\s*true\s*$": 'ahash = "0.8"',
    r"^html5ever\.workspace\s*=\s*true\s*$": 'html5ever = "0.36"',
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

# Step 5: Create .cargo-checksum.json for html-to-markdown-rs
echo "Creating checksum for html-to-markdown-rs..."
echo '{"files":{}}' >"$RUBY_PKG/$VENDOR_DIR/html-to-markdown-rs/.cargo-checksum.json"

# Step 6: Expand workspace references in native Cargo.toml
echo "Expanding workspace references in native Cargo.toml..."
python3 - "Cargo.toml" "$VENDOR_DIR" <<'PY'
import re
import sys
from pathlib import Path

path = Path(sys.argv[1])
vendor_dir = sys.argv[2]
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
}

for pattern, replacement in replacements.items():
    text = re.sub(pattern, replacement, text, flags=re.MULTILINE)

# Replace workspace dependency with path to vendored crate
text = re.sub(
    r'^html-to-markdown-rs\s*=\s*\{\s*workspace\s*=\s*true,\s*features\s*=\s*\[([^\]]+)\]\s*\}',
    rf'html-to-markdown-rs = {{ path = "../../../{vendor_dir}/html-to-markdown-rs", features = [\1] }}',
    text,
    flags=re.MULTILINE
)

# Handle lints section separately (multi-line)
text = re.sub(
    r"^\[lints\.rust\]\s*\n(?:^.*workspace.*\n)+",
    '[lints.rust]\nunsafe_code = "forbid"\nmissing_docs = "warn"\nunused_must_use = "deny"\n\n',
    text,
    flags=re.MULTILINE
)
text = re.sub(
    r"^\[lints\.clippy\]\s*\n(?:^.*workspace.*\n)+",
    '[lints.clippy]\nall = { level = "deny", priority = -1 }\ncargo = { level = "deny", priority = -1 }\npedantic = { level = "warn", priority = -1 }\nnursery = { level = "warn", priority = -1 }\nmultiple_crate_versions = "allow"\n',
    text,
    flags=re.MULTILINE
)

path.write_text(text, encoding="utf-8")
PY

# Step 7: Generate Cargo.lock without source replacements
# For vendored git dependencies, cargo requires the lock file to be generated
# BEFORE the source replacement config is present
# We also need to hide the workspace root to avoid package collision errors
echo "Temporarily moving source replacement config and workspace root..."
mv "$RUBY_PKG/.cargo/config.toml" "$RUBY_PKG/.cargo/config.toml.tmp"
mv "$REPO_ROOT/Cargo.toml" "$REPO_ROOT/Cargo.toml.tmp"

echo "Generating Cargo.lock..."
cargo generate-lockfile --manifest-path="$NATIVE_EXT/Cargo.toml"

# Step 8: Fetch locked versions and re-vendor to ensure version consistency
# This ensures the vendored crates exactly match the Cargo.lock
echo "Fetching locked dependency versions..."
cargo fetch --locked --manifest-path="$NATIVE_EXT/Cargo.toml"

# Re-vendor with locked versions - cargo vendor will overwrite existing crates
# but skip path dependencies like html-to-markdown-rs
echo "Re-vendoring with locked versions..."
cargo vendor --locked "$RUBY_PKG/$VENDOR_DIR" --manifest-path="$NATIVE_EXT/Cargo.toml" >/dev/null

echo "Restoring source replacement config and workspace root..."
mv "$RUBY_PKG/.cargo/config.toml.tmp" "$RUBY_PKG/.cargo/config.toml"
mv "$REPO_ROOT/Cargo.toml.tmp" "$REPO_ROOT/Cargo.toml"

# Step 9: Update .cargo-checksum.json files to remove entries for excluded files
# The gemspec excludes .dll, .so, .dylib, .lib, .a files but the checksum files reference them
echo "Updating checksum files to remove excluded file entries..."
python3 - "$RUBY_PKG/$VENDOR_DIR" <<'PY'
import json
import re
import sys
from pathlib import Path

vendor_dir = Path(sys.argv[1])
excluded_pattern = re.compile(r'\.(dll|so|dylib|lib|a)$', re.IGNORECASE)

for checksum_file in vendor_dir.rglob('.cargo-checksum.json'):
    try:
        data = json.loads(checksum_file.read_text(encoding='utf-8'))
        if 'files' in data and isinstance(data['files'], dict):
            # Filter out excluded file entries
            original_count = len(data['files'])
            data['files'] = {
                k: v for k, v in data['files'].items()
                if not excluded_pattern.search(k)
            }
            if len(data['files']) < original_count:
                checksum_file.write_text(json.dumps(data), encoding='utf-8')
    except (json.JSONDecodeError, IOError):
        pass  # Skip files that can't be parsed
PY

echo "✓ Vendored all dependencies to packages/ruby/$VENDOR_DIR/"
echo "✓ Created .cargo/config.toml with source replacements"
echo "✓ Generated Cargo.lock (matches vendored versions)"

# Count vendored crates
crate_count=$(find "$RUBY_PKG/$VENDOR_DIR" -maxdepth 1 -type d 2>/dev/null | wc -l)
echo "✓ Vendored $((crate_count - 1)) crates"
