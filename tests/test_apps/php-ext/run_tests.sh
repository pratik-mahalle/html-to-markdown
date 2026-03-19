#!/bin/bash
set -e

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/../../.." && pwd)"

# Set library paths for the Rust extension
export DYLD_LIBRARY_PATH="$REPO_ROOT/target/release:${DYLD_LIBRARY_PATH:-}"
export LD_LIBRARY_PATH="$REPO_ROOT/target/release:${LD_LIBRARY_PATH:-}"

# Determine extension loading arguments
PHP_EXT_ARGS=""
if ! php -m 2>/dev/null | grep -q html_to_markdown; then
  # Try to locate the compiled extension
  # macOS produces .dylib, Linux produces .so
  CANDIDATES=(
    "$REPO_ROOT/target/release/libhtml_to_markdown_php.dylib"
    "$REPO_ROOT/target/release/deps/libhtml_to_markdown_php.dylib"
    "$REPO_ROOT/target/release/libhtml_to_markdown_php.so"
    "$REPO_ROOT/target/release/deps/libhtml_to_markdown_php.so"
  )

  EXT_PATH=""
  for candidate in "${CANDIDATES[@]}"; do
    if [ -f "$candidate" ]; then
      EXT_PATH="$candidate"
      break
    fi
  done

  if [ -n "$EXT_PATH" ]; then
    PHP_EXT_ARGS="-d extension=$EXT_PATH"
    echo "Loading extension from: $EXT_PATH"
  else
    echo "Warning: html_to_markdown extension not found. Build it first:"
    echo "  cargo build --release -p html-to-markdown-php"
    echo ""
    echo "Searched locations:"
    for candidate in "${CANDIDATES[@]}"; do
      echo "  $candidate"
    done
    exit 1
  fi
fi

echo "Running html-to-markdown PHP extension test suite..."
echo ""

# shellcheck disable=SC2086
php $PHP_EXT_ARGS "$SCRIPT_DIR/main.php"
