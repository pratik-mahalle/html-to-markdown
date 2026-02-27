#!/usr/bin/env bash
set -euo pipefail

scripts/publish/elixir/stage-rust-core.sh

# Generate Cargo.lock so it's included in the Hex package (required by mix.exs files list)
pushd packages/elixir/native/html_to_markdown_elixir >/dev/null
cargo generate-lockfile
popd >/dev/null

pushd packages/elixir >/dev/null
mix hex.publish --yes
popd >/dev/null
