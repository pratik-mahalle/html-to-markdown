#!/usr/bin/env bash
set -euo pipefail

# Stage the Rust core crate source into vendor/
scripts/publish/elixir/vendor-dependencies.sh

# Remove Rust build artifacts — these are platform-specific and rebuilt during
# package installation. Including them would exceed Hex's 16 MB size limit.
rm -rf packages/elixir/native/html_to_markdown_elixir/target

# Generate Cargo.lock so it's included in the Hex package (required by mix.exs files list)
pushd packages/elixir/native/html_to_markdown_elixir >/dev/null
cargo generate-lockfile
popd >/dev/null

pushd packages/elixir >/dev/null
mix hex.build
popd >/dev/null
