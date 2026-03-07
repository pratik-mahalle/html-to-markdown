#!/usr/bin/env bash
set -euo pipefail

version="${VERSION:?VERSION is required}"
url="https://repo.packagist.org/p2/kreuzberg/html-to-markdown.json"
max_attempts=3
attempt=1
exists="false"
resolved=false

while [ $attempt -le $max_attempts ]; do
	echo "::debug::Checking Packagist for kreuzberg/html-to-markdown:${version} (attempt ${attempt}/${max_attempts})" >&2

	tmpfile=$(mktemp)
	http_code=$(curl \
		--silent \
		--show-error \
		--retry 3 \
		--retry-delay 5 \
		--connect-timeout 30 \
		--max-time 60 \
		-o "$tmpfile" \
		-w "%{http_code}" \
		"$url" 2>/dev/null || echo "000")

	if [ "$http_code" = "404" ]; then
		exists="false"
		resolved=true
		rm -f "$tmpfile"
		break
	fi

	if [ "$http_code" = "200" ]; then
		if python3 -c "
import json, sys
with open('$tmpfile') as f:
    data = json.load(f)
pkgs = data.get('packages', {}).get('kreuzberg/html-to-markdown', [])
sys.exit(0 if any(p.get('version') == '$version' for p in pkgs) else 1)
" 2>/dev/null; then
			exists="true"
		else
			exists="false"
		fi
		resolved=true
		rm -f "$tmpfile"
		break
	fi

	rm -f "$tmpfile"

	if [ $attempt -lt $max_attempts ]; then
		sleep_time=$((attempt * 5))
		echo "::warning::Packagist check failed (HTTP $http_code), retrying in ${sleep_time}s..." >&2
		sleep "$sleep_time"
	fi

	attempt=$((attempt + 1))
done

if [ "$resolved" = false ]; then
	echo "::error::Failed to check Packagist after $max_attempts attempts" >&2
	exit 1
fi

if [ "$exists" = "true" ]; then
	echo "exists=true" >>"${GITHUB_OUTPUT}"
	echo "::notice::PHP package kreuzberg/html-to-markdown:${version} already exists on Packagist" >&2
else
	echo "exists=false" >>"${GITHUB_OUTPUT}"
	echo "::notice::PHP package kreuzberg/html-to-markdown:${version} not found on Packagist" >&2
fi
