#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$SCRIPT_DIR/../../.."

echo "=== Vendoring html-to-markdown crate ==="

# Extract version from root workspace
VERSION=$(awk -F '"' '/^\[workspace.package\]/,/^version =/ {if ($0 ~ /^version =/) {print $2; exit}}' "$REPO_ROOT/Cargo.toml")

# Extract dependency versions from workspace dependencies
extract_version() {
	local dep_name="$1"
	awk -F '"' "
    /^${dep_name} = \\{ / {
      if (\$0 ~ /version =/) {
        match(\$0, /version = \"([^\"]+)\"/, arr);
        print arr[1];
        exit;
      }
    }
    /^${dep_name} = \"/ {print \$2; exit}
  " "$REPO_ROOT/Cargo.toml"
}

TL_VERSION=$(extract_version "tl")
HTML5EVER_VERSION=$(extract_version "html5ever")
MARKUP5EVER_VERSION=$(extract_version "markup5ever_rcdom")
REGEX_VERSION=$(extract_version "regex")
ONCE_CELL_VERSION=$(extract_version "once_cell")
THISERROR_VERSION=$(extract_version "thiserror")
BASE64_VERSION=$(extract_version "base64")
ENCODING_RS_VERSION=$(extract_version "encoding_rs")
SERDE_VERSION=$(extract_version "serde")
SERDE_JSON_VERSION=$(extract_version "serde_json")

echo "Extracted version: $VERSION"
echo "Dependency versions:"
echo "  tl: $TL_VERSION"
echo "  html5ever: $HTML5EVER_VERSION"
echo "  markup5ever_rcdom: $MARKUP5EVER_VERSION"
echo "  regex: $REGEX_VERSION"
echo "  once_cell: $ONCE_CELL_VERSION"
echo "  thiserror: $THISERROR_VERSION"
echo "  base64: $BASE64_VERSION"
echo "  encoding_rs: $ENCODING_RS_VERSION"
echo "  serde: $SERDE_VERSION"
echo "  serde_json: $SERDE_JSON_VERSION"

# Clean and create vendor directory
rm -rf "$REPO_ROOT/packages/ruby/vendor"
mkdir -p "$REPO_ROOT/packages/ruby/vendor"

# Copy crate
cp -R "$REPO_ROOT/crates/html-to-markdown" "$REPO_ROOT/packages/ruby/vendor/html-to-markdown"

# Clean up build artifacts
rm -rf "$REPO_ROOT/packages/ruby/vendor/html-to-markdown/target"
find "$REPO_ROOT/packages/ruby/vendor/html-to-markdown" -name '*.swp' -delete 2>/dev/null || true
find "$REPO_ROOT/packages/ruby/vendor/html-to-markdown" -name '*.bak' -delete 2>/dev/null || true
find "$REPO_ROOT/packages/ruby/vendor/html-to-markdown" -name '*.tmp' -delete 2>/dev/null || true

# Update vendored crate to use explicit versions (not workspace)
sed -i.bak "s/^version.workspace = true/version = \"${VERSION}\"/" "$REPO_ROOT/packages/ruby/vendor/html-to-markdown/Cargo.toml"
sed -i.bak 's/^edition.workspace = true/edition = "2024"/' "$REPO_ROOT/packages/ruby/vendor/html-to-markdown/Cargo.toml"
sed -i.bak 's/^rust-version.workspace = true/rust-version = "1.85"/' "$REPO_ROOT/packages/ruby/vendor/html-to-markdown/Cargo.toml"
sed -i.bak 's/^authors.workspace = true/authors = ["Na'\''aman Hirschfeld <nhirschfeld@gmail.com>"]/' "$REPO_ROOT/packages/ruby/vendor/html-to-markdown/Cargo.toml"
sed -i.bak 's/^license.workspace = true/license = "MIT"/' "$REPO_ROOT/packages/ruby/vendor/html-to-markdown/Cargo.toml"
sed -i.bak 's/^repository.workspace = true/repository = "https:\/\/github.com\/kreuzberg-dev\/html-to-markdown"/' "$REPO_ROOT/packages/ruby/vendor/html-to-markdown/Cargo.toml"
sed -i.bak 's/^homepage.workspace = true/homepage = "https:\/\/github.com\/kreuzberg-dev\/html-to-markdown"/' "$REPO_ROOT/packages/ruby/vendor/html-to-markdown/Cargo.toml"
sed -i.bak 's/^documentation.workspace = true/documentation = "https:\/\/docs.rs\/html-to-markdown-rs"/' "$REPO_ROOT/packages/ruby/vendor/html-to-markdown/Cargo.toml"
sed -i.bak 's/^readme.workspace = true/readme = "README.md"/' "$REPO_ROOT/packages/ruby/vendor/html-to-markdown/Cargo.toml"
rm -f "$REPO_ROOT/packages/ruby/vendor/html-to-markdown/Cargo.toml.bak"

# Update Ruby native Cargo.toml to use vendored path
RUBY_NATIVE_CARGO="$REPO_ROOT/packages/ruby/ext/html-to-markdown-rb/native/Cargo.toml"
sed -i.bak 's/html-to-markdown-rs = { workspace = true,/html-to-markdown-rs = { path = "..\/..\/..\/vendor\/html-to-markdown",/' "$RUBY_NATIVE_CARGO"
rm -f "$RUBY_NATIVE_CARGO.bak"

# Generate vendor workspace Cargo.toml
cat >"$REPO_ROOT/packages/ruby/vendor/Cargo.toml" <<EOF
[workspace]
members = ["html-to-markdown"]
resolver = "2"

[workspace.package]
version = "${VERSION}"
edition = "2024"
rust-version = "1.85"
authors = ["Na'aman Hirschfeld <nhirschfeld@gmail.com>"]
license = "MIT"
repository = "https://github.com/kreuzberg-dev/html-to-markdown"

[workspace.dependencies]
tl = { package = "astral-tl", version = "${TL_VERSION}" }
html5ever = "${HTML5EVER_VERSION}"
markup5ever_rcdom = "${MARKUP5EVER_VERSION}"
regex = "${REGEX_VERSION}"
once_cell = "${ONCE_CELL_VERSION}"
thiserror = "${THISERROR_VERSION}"
base64 = "${BASE64_VERSION}"
encoding_rs = "${ENCODING_RS_VERSION}"
serde = { version = "${SERDE_VERSION}", features = ["derive"] }
serde_json = "${SERDE_JSON_VERSION}"
EOF

echo "✓ Vendoring complete (version: $VERSION)"
