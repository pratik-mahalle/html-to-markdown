#!/usr/bin/env bash
set -euo pipefail

# Vendor all dependencies (stages core crate + vendors transitive deps)
scripts/publish/elixir/vendor-dependencies.sh

pushd packages/elixir >/dev/null
mix hex.build
popd >/dev/null
