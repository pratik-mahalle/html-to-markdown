#!/usr/bin/env bash
set -euo pipefail

version="${VERSION:?VERSION is required}"
max_attempts=3

check_crate() {
	local crate="$1"
	local attempt=1
	local result="false"

	while [ $attempt -le $max_attempts ]; do
		echo "::debug::Checking crates.io for ${crate}@${version} (attempt ${attempt}/${max_attempts})" >&2

		result=$(
			python3 - "$crate" "$version" <<'PY'
import json, sys, urllib.request
crate = sys.argv[1]
version = sys.argv[2]
try:
    req = urllib.request.Request(
        f"https://crates.io/api/v1/crates/{crate}",
        headers={"User-Agent": "html-to-markdown-ci/1.0"},
    )
    with urllib.request.urlopen(req, timeout=30) as resp:
        data = json.load(resp)
    versions = [item.get("num") for item in data.get("versions", [])]
    print("true" if version in versions else "false")
except urllib.error.HTTPError as e:
    if e.code == 404:
        print("false")
    else:
        print("error")
except Exception:
    print("error")
PY
		)

		if [ "$result" = "true" ] || [ "$result" = "false" ]; then
			break
		fi

		if [ $attempt -lt $max_attempts ]; then
			sleep_time=$((attempt * 5))
			echo "::warning::crates.io check for ${crate} failed, retrying in ${sleep_time}s..." >&2
			sleep "$sleep_time"
		fi

		attempt=$((attempt + 1))
	done

	if [ "$result" = "error" ]; then
		echo "::error::Failed to check crates.io for ${crate} after $max_attempts attempts" >&2
		echo "false"
	else
		echo "$result"
	fi
}

rs_exists=$(check_crate "html-to-markdown-rs")
cli_exists=$(check_crate "html-to-markdown-cli")

if [[ "${rs_exists}" == "true" && "${cli_exists}" == "true" ]]; then
	all_exist=true
else
	all_exist=false
fi

{
	echo "rs_exists=${rs_exists}"
	echo "cli_exists=${cli_exists}"
	echo "all_exist=${all_exist}"
} >>"${GITHUB_OUTPUT}"
