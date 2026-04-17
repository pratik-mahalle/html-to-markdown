#!/usr/bin/env bash
set -euo pipefail

# Download prebuilt C FFI library from GitHub releases
VERSION="${1:-3.2.4}"
REPO="kreuzberg-dev/html-to-markdown"

# Detect platform
UNAME_S=$(uname -s)
UNAME_M=$(uname -m)

if [[ "$UNAME_S" == "Darwin" ]]; then
  PLATFORM="darwin-arm64"
  [[ "$UNAME_M" == "x86_64" ]] && PLATFORM="darwin-x64"
elif [[ "$UNAME_S" == "Linux" ]]; then
  PLATFORM="linux-x64"
  [[ "$UNAME_M" == "aarch64" ]] && PLATFORM="linux-arm64"
else
  echo "Unsupported platform: $UNAME_S $UNAME_M" >&2
  exit 1
fi

ARCHIVE="html-to-markdown-ffi-${VERSION}-${PLATFORM}.tar.gz"
URL="https://github.com/${REPO}/releases/download/v${VERSION}/${ARCHIVE}"
DEST="ffi"

TMPDIR=$(mktemp -d)
trap 'rm -rf "$TMPDIR"' EXIT

echo "Downloading ${ARCHIVE}..."
curl -fsSL "$URL" | tar xz -C "$TMPDIR"

# Flatten: move contents of the extracted subdirectory to $DEST
rm -rf "$DEST"
EXTRACTED=$(find "$TMPDIR" -mindepth 1 -maxdepth 1 -type d | head -1)
mv "$EXTRACTED" "$DEST"

# Fix macOS dylib install_name (published dylibs have CI runner paths baked in)
if [[ "$UNAME_S" == "Darwin" ]] && [[ -f "$DEST/lib/libhtml_to_markdown_ffi.dylib" ]]; then
  install_name_tool -id @rpath/libhtml_to_markdown_ffi.dylib "$DEST/lib/libhtml_to_markdown_ffi.dylib" 2>/dev/null || true
fi

echo "FFI library installed to ${DEST}/"
ls -R "$DEST"
