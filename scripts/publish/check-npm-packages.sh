#!/usr/bin/env bash
set -euo pipefail

version="${VERSION:?VERSION is required}"
max_attempts=3

check_pkg() {
	local pkg="$1"
	local attempt=1
	local result="false"

	while [ $attempt -le $max_attempts ]; do
		echo "::debug::Checking npm for ${pkg}@${version} (attempt ${attempt}/${max_attempts})" >&2

		result=$(
			node - "$pkg" "$version" <<'NODE'
const https = require("https");
const pkg = process.argv[1];
const version = process.argv[2];

function fetch(url) {
  return new Promise((resolve) => {
    https
      .get(url, (res) => {
        let data = "";
        res.on("data", (chunk) => (data += chunk));
        res.on("end", () => {
          try {
            const body = JSON.parse(data);
            const exists = body.versions && body.versions[version] ? "true" : "false";
            resolve(exists);
          } catch (err) {
            resolve("error");
          }
        });
      })
      .on("error", () => resolve("error"));
  });
}

fetch(`https://registry.npmjs.org/${encodeURIComponent(pkg)}`)
  .then((exists) => console.log(exists))
  .catch(() => console.log("error"));
NODE
		)

		if [ "$result" = "true" ] || [ "$result" = "false" ]; then
			break
		fi

		if [ $attempt -lt $max_attempts ]; then
			sleep_time=$((attempt * 5))
			echo "::warning::npm check for ${pkg} failed, retrying in ${sleep_time}s..." >&2
			sleep "$sleep_time"
		fi

		attempt=$((attempt + 1))
	done

	if [ "$result" = "error" ]; then
		echo "::error::Failed to check npm for ${pkg} after $max_attempts attempts" >&2
		echo "false"
	else
		echo "$result"
	fi
}

node_exists=$(check_pkg "@kreuzberg/html-to-markdown-node")
wasm_exists=$(check_pkg "@kreuzberg/html-to-markdown-wasm")
ts_exists=$(check_pkg "@kreuzberg/html-to-markdown")

{
	echo "node_exists=${node_exists}"
	echo "wasm_exists=${wasm_exists}"
	echo "ts_exists=${ts_exists}"
} >>"${GITHUB_OUTPUT}"
