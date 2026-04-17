#!/usr/bin/env bash
set -euo pipefail

# With rustler_precompiled, the Hex package only contains Elixir code + checksum file.
# Precompiled NIF binaries are downloaded from GitHub releases at install time.
# No Rust source vendoring needed.

pushd packages/elixir >/dev/null
mix hex.build
popd >/dev/null
