#!/usr/bin/env bash
set -euo pipefail

version="${VERSION:?VERSION is required}"
version="${version#v}"
package_name="html-to-markdown"
url="https://rubygems.org/api/v1/versions/${package_name}.json"
max_attempts=3
attempt=1
http_code=""

normalize_rubygems_version() {
	local v="$1"
	if [[ "$v" == *-* ]]; then
		local base="${v%%-*}"
		local prerelease="${v#*-}"
		echo "${base}.pre.${prerelease//-/.}"
	else
		echo "$v"
	fi
}

rubygems_version="$(normalize_rubygems_version "$version")"
version_candidates=("$version")
if [[ "$rubygems_version" != "$version" ]]; then
	version_candidates+=("$rubygems_version")
fi

while [ $attempt -le $max_attempts ]; do
	echo "::debug::Checking RubyGems for ${package_name} versions: ${version_candidates[*]} (attempt ${attempt}/${max_attempts})" >&2

	http_code=$(curl \
		--silent \
		--show-error \
		--retry 3 \
		--retry-delay 5 \
		--connect-timeout 30 \
		--max-time 60 \
		-o /tmp/rubygems-check.json \
		-w "%{http_code}" \
		"$url" 2>/dev/null || echo "000")

	if [ "$http_code" = "200" ] || [ "$http_code" = "404" ]; then
		break
	fi

	if [ $attempt -lt $max_attempts ]; then
		sleep_time=$((attempt * 5))
		echo "::warning::RubyGems check failed (HTTP $http_code), retrying in ${sleep_time}s..." >&2
		sleep "$sleep_time"
	fi

	attempt=$((attempt + 1))
done

if [ "$http_code" = "200" ]; then
	found=false
	for candidate in "${version_candidates[@]}"; do
		if python3 -c "
import json, sys
with open('/tmp/rubygems-check.json') as f:
    data = json.load(f)
sys.exit(0 if any(e.get('number') == '${candidate}' for e in data) else 1)
" 2>/dev/null; then
			found=true
			break
		fi
	done

	if [ "$found" = "true" ]; then
		echo "exists=true" >>"${GITHUB_OUTPUT}"
		echo "::notice::Ruby gem ${package_name} version found on RubyGems" >&2
	else
		echo "exists=false" >>"${GITHUB_OUTPUT}"
		echo "::notice::Ruby gem ${package_name} not found on RubyGems for versions: ${version_candidates[*]}, will build and publish" >&2
	fi
elif [ "$http_code" = "404" ]; then
	echo "exists=false" >>"${GITHUB_OUTPUT}"
	echo "::notice::Ruby gem ${package_name} not found on RubyGems (first publish), will build and publish" >&2
else
	echo "::error::Failed to check RubyGems after $max_attempts attempts (last HTTP code: $http_code)" >&2
	exit 1
fi
