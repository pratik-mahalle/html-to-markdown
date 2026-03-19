#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$SCRIPT_DIR/../../.."
R_PKG="$REPO_ROOT/packages/r"
RUST_DIR="$R_PKG/src/rust"

echo "=== Vendoring R package dependencies ==="

# Step 1: Stage the Rust core crate (copies html-to-markdown to vendor/)
echo "Step 1: Staging Rust core crate..."
"$SCRIPT_DIR/stage-rust-core.sh"

# Step 2: Vendor all transitive dependencies using cargo vendor
echo ""
echo "Step 2: Vendoring all transitive dependencies..."
cd "$RUST_DIR"

echo "Running cargo vendor..."
cargo vendor vendor >/dev/null

# Step 3: Create vendor-config.toml (source replacement for vendored builds)
echo "Step 3: Creating vendor-config.toml..."
cat >vendor-config.toml <<'TOML'
[source.crates-io]
replace-with = "vendored-sources"

[source.vendored-sources]
directory = "vendor"
TOML

# Step 4: Add .cargo-checksum.json for html-to-markdown-rs (path dep needs it)
echo "Step 4: Creating checksum for html-to-markdown-rs..."
echo '{"files":{}}' >vendor/html-to-markdown-rs/.cargo-checksum.json

# Step 5: Clean up unnecessary files to reduce tarball size
echo "Step 5: Cleaning up vendored dependencies..."

# Remove test/bench/doc directories
while IFS= read -r dir; do
  rm -rf "$dir"
done < <(find vendor -type d \( -name "tests" -o -name "benches" -o -name "examples" -o -name "docs" -o -name ".github" -o -name "ci" \) 2>/dev/null)

# Remove documentation and metadata files
find vendor -type f \( \
  -name "*.md" -o \
  -name "LICENSE*" -o \
  -name "CHANGELOG*" -o \
  -name ".git*" -o \
  -name ".cargo-ok" -o \
  -name "*.html" -o \
  -name "*.yml" -o \
  -name "*.yaml" \
  \) -delete 2>/dev/null || true

# Remove static libraries (pre-built binaries not needed for source distribution)
find vendor -type f -name "*.a" -delete 2>/dev/null || true

# Step 6: Generate inst/AUTHORS from vendored crate metadata
echo "Step 6: Generating inst/AUTHORS..."
mkdir -p "$R_PKG/inst"
python3 - "$RUST_DIR/vendor" "$R_PKG/inst/AUTHORS" <<'PY'
import sys
import tomllib
from pathlib import Path

vendor_dir = Path(sys.argv[1])
out_path = Path(sys.argv[2])

lines = [
    "# Authors of vendored Rust crates",
    "",
    "This file lists the authors of the Rust crates vendored in this package.",
    "",
    "The htmltomarkdown R package includes Rust code from the following crates:",
    "",
]

entries = []
for cargo_toml in sorted(vendor_dir.glob("*/Cargo.toml")):
    try:
        data = tomllib.loads(cargo_toml.read_text(encoding="utf-8"))
        pkg = data.get("package", {})
        name = pkg.get("name", cargo_toml.parent.name)
        version = pkg.get("version", "?")
        authors = pkg.get("authors", [])
        license_val = pkg.get("license", "unknown")
        author_str = ", ".join(authors) if authors else "(no authors listed)"
        entries.append(f"{name} {version} ({license_val}): {author_str}")
    except Exception:
        continue

lines.extend(entries)
lines.append("")
out_path.write_text("\n".join(lines), encoding="utf-8")
print(f"  Generated {len(entries)} crate entries in inst/AUTHORS")
PY

# Step 7: Create vendor.tar.xz
echo "Step 7: Creating vendor.tar.xz..."
rm -f vendor.tar.xz
tar -cJ -f vendor.tar.xz vendor

# Step 8: Remove extracted vendor directory
echo "Step 8: Cleaning up extracted vendor directory..."
rm -rf vendor

# Summary
crate_count=$(tar -tf vendor.tar.xz | grep -c '^vendor/[^/]*/Cargo.toml$' || true)
tarball_size=$(du -h vendor.tar.xz | cut -f1)
echo ""
echo "=== Vendoring complete ==="
echo "  Vendored ${crate_count} crates"
echo "  Tarball size: ${tarball_size}"
echo "  Output: ${RUST_DIR}/vendor.tar.xz"
