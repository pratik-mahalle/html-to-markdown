#!/usr/bin/env bash
set -euo pipefail

: "${VERSION:?VERSION is required}"

exists=$(
	python3 - <<'PY'
import json, os, urllib.request, urllib.error

version = os.environ["VERSION"]
try:
    with urllib.request.urlopen(f"https://crandb.r-pkg.org/htmltomarkdown/all") as resp:
        data = json.load(resp)
    versions = list(data.get("versions", {}).keys())
    exists = version in versions
except urllib.error.HTTPError as e:
    if e.code == 404:
        exists = False
    else:
        raise
print("true" if exists else "false")
PY
)

echo "exists=${exists}" >>"${GITHUB_OUTPUT}"
