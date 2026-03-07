#!/usr/bin/env bash
set -euo pipefail

scripts/publish/elixir/stage-rust-core.sh

# Generate Cargo.lock so it's included in the Hex package (required by mix.exs files list)
pushd packages/elixir/native/html_to_markdown_elixir >/dev/null
cargo generate-lockfile
popd >/dev/null

pushd packages/elixir >/dev/null
publish_log=$(mktemp)
set +e
mix hex.publish --yes 2>&1 | tee "${publish_log}"
status=${PIPESTATUS[0]}
set -e

if [[ "${status}" -ne 0 ]]; then
	if grep -q "already published" "${publish_log}" || grep -q "already exists" "${publish_log}"; then
		echo "::notice::Hex package already published; skipping."
	else
		exit "${status}"
	fi
fi
popd >/dev/null
