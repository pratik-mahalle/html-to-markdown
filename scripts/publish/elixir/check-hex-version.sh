#!/usr/bin/env bash
set -euo pipefail

: "${VERSION:?VERSION is required}"

max_attempts=3
attempt=1
exists=""

while [ $attempt -le $max_attempts ]; do
	echo "::debug::Checking Hex.pm for html_to_markdown@${VERSION} (attempt ${attempt}/${max_attempts})" >&2

	exists=$(
		python3 - <<'PY'
import json, os, urllib.request, urllib.error
version = os.environ["VERSION"]
try:
    with urllib.request.urlopen("https://hex.pm/api/packages/html_to_markdown", timeout=30) as resp:
        data = json.load(resp)
    found = any(rel.get("version") == version for rel in data.get("releases", []))
    print("true" if found else "false")
except urllib.error.HTTPError as e:
    if e.code == 404:
        print("false")
    else:
        print("error")
except Exception:
    print("error")
PY
	)

	if [ "$exists" = "true" ] || [ "$exists" = "false" ]; then
		break
	fi

	if [ $attempt -lt $max_attempts ]; then
		sleep_time=$((attempt * 5))
		echo "::warning::Hex.pm check failed, retrying in ${sleep_time}s..." >&2
		sleep "$sleep_time"
	fi

	attempt=$((attempt + 1))
done

if [ "$exists" = "error" ] || [ -z "$exists" ]; then
	echo "::error::Failed to check Hex.pm after $max_attempts attempts" >&2
	exit 1
fi

echo "exists=${exists}" >>"${GITHUB_OUTPUT}"
if [ "$exists" = "true" ]; then
	echo "::notice::Elixir package html_to_markdown@${VERSION} already exists on Hex.pm" >&2
else
	echo "::notice::Elixir package html_to_markdown@${VERSION} not found on Hex.pm, will build and publish" >&2
fi
