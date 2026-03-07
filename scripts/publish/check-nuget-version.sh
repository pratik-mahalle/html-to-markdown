#!/usr/bin/env bash
set -euo pipefail

version="${VERSION:?VERSION is required}"
pkg="KreuzbergDev.HtmlToMarkdown"
max_attempts=3
attempt=1

while [ $attempt -le $max_attempts ]; do
	echo "::debug::Checking NuGet for ${pkg}@${version} (attempt ${attempt}/${max_attempts})" >&2

	exists=$(
		python3 - "$pkg" "$version" <<'PY'
import json, sys, urllib.request
pkg = sys.argv[1]
version = sys.argv[2]
try:
    with urllib.request.urlopen(
        f"https://api.nuget.org/v3-flatcontainer/{pkg.lower()}/index.json",
        timeout=30,
    ) as resp:
        data = json.load(resp)
    exists = version in data.get("versions", [])
    print("true" if exists else "false")
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
		echo "::warning::NuGet check failed, retrying in ${sleep_time}s..." >&2
		sleep "$sleep_time"
	fi

	attempt=$((attempt + 1))
done

if [ "$exists" = "error" ]; then
	echo "::error::Failed to check NuGet after $max_attempts attempts" >&2
	exit 1
fi

echo "exists=${exists}" >>"${GITHUB_OUTPUT}"
if [ "$exists" = "true" ]; then
	echo "::notice::NuGet package ${pkg}@${version} already exists" >&2
else
	echo "::notice::NuGet package ${pkg}@${version} not found, will build and publish" >&2
fi
