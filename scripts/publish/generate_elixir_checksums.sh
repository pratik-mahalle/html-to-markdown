#!/usr/bin/env bash
set -euo pipefail

VERSION="${1:?Usage: $0 <version>}"
REPO="kreuzberg-dev/html-to-markdown"
CHECKSUM_FILE="packages/elixir/checksum-Elixir.HtmlToMarkdown.Native.exs"

TARGETS=(
  "aarch64-apple-darwin"
  "aarch64-unknown-linux-gnu"
  "x86_64-unknown-linux-gnu"
)

NIF_VERSIONS=("2.16" "2.17")

TMPDIR=$(mktemp -d)
trap 'rm -rf "$TMPDIR"' EXIT

echo "Generating checksums for v${VERSION}..."
CHECKSUMS=()

for TARGET in "${TARGETS[@]}"; do
  for NIF_VERSION in "${NIF_VERSIONS[@]}"; do
    if [[ "$TARGET" == *"windows"* ]]; then EXT="dll"; else EXT="so"; fi
    FILENAME="libhtml_to_markdown_nif-v${VERSION}-nif-${NIF_VERSION}-${TARGET}.${EXT}.tar.gz"
    URL="https://github.com/${REPO}/releases/download/v${VERSION}/${FILENAME}"
    echo "Downloading: $FILENAME"
    if curl -fsSL -o "${TMPDIR}/${FILENAME}" "$URL"; then
      if command -v sha256sum &>/dev/null; then
        CHECKSUM=$(sha256sum "${TMPDIR}/${FILENAME}" | cut -d' ' -f1)
      else
        CHECKSUM=$(shasum -a 256 "${TMPDIR}/${FILENAME}" | cut -d' ' -f1)
      fi
      CHECKSUMS+=("  \"${FILENAME}\" => \"sha256:${CHECKSUM}\",")
    else
      echo "  ERROR: Failed to download $FILENAME"
      exit 1
    fi
  done
done

mapfile -t SORTED < <(printf '%s\n' "${CHECKSUMS[@]}" | sort)
{
  echo "%{"
  for C in "${SORTED[@]}"; do echo "$C"; done
  echo "}"
} >"$CHECKSUM_FILE"

echo "Generated checksums for ${#SORTED[@]} files."
cat "$CHECKSUM_FILE"
