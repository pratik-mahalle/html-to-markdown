#!/usr/bin/env bash
set -euo pipefail

version="${VERSION:?VERSION is required}"
url="https://pypi.org/pypi/html-to-markdown/${version}/json"
max_attempts=3
attempt=1
http_code=""

while [ $attempt -le $max_attempts ]; do
	echo "::debug::Checking PyPI for html-to-markdown==${version} (attempt ${attempt}/${max_attempts})" >&2

	http_code=$(curl \
		--silent \
		--show-error \
		--retry 3 \
		--retry-delay 5 \
		--connect-timeout 30 \
		--max-time 60 \
		-o /dev/null \
		-w "%{http_code}" \
		"$url" 2>/dev/null || echo "000")

	if [ "$http_code" = "200" ] || [ "$http_code" = "404" ]; then
		break
	fi

	if [ $attempt -lt $max_attempts ]; then
		sleep_time=$((attempt * 5))
		echo "::warning::PyPI check failed (HTTP $http_code), retrying in ${sleep_time}s..." >&2
		sleep "$sleep_time"
	fi

	attempt=$((attempt + 1))
done

if [ "$http_code" = "200" ]; then
	echo "exists=true" >>"${GITHUB_OUTPUT}"
	echo "::notice::Python package html-to-markdown==${version} already exists on PyPI" >&2
elif [ "$http_code" = "404" ]; then
	echo "exists=false" >>"${GITHUB_OUTPUT}"
	echo "::notice::Python package html-to-markdown==${version} not found on PyPI, will build and publish" >&2
else
	echo "::error::Failed to check PyPI after $max_attempts attempts (last HTTP code: $http_code)" >&2
	exit 1
fi
