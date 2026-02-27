#!/usr/bin/env bash
set -euo pipefail

# Vendor all dependencies (stages core crate + vendors transitive deps)
scripts/publish/elixir/vendor-dependencies.sh

# Remove Rust build artifacts — these are platform-specific and rebuilt during
# package installation. Including them would exceed Hex's 16 MB size limit.
rm -rf packages/elixir/native/html_to_markdown_elixir/target

pushd packages/elixir >/dev/null
mix hex.build
popd >/dev/null
