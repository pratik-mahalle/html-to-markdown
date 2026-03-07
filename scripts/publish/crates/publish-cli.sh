#!/usr/bin/env bash
set -euo pipefail

publish_log=$(mktemp)
set +e
cargo publish -p html-to-markdown-cli --token "${CARGO_TOKEN:?CARGO_TOKEN is required}" 2>&1 | tee "${publish_log}"
status=${PIPESTATUS[0]}
set -e

if [[ "${status}" -ne 0 ]]; then
	if grep -q "already uploaded" "${publish_log}" || grep -q "is already published" "${publish_log}"; then
		echo "::notice::html-to-markdown-cli already published; skipping."
	else
		exit "${status}"
	fi
fi
