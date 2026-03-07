#!/usr/bin/env bash
set -euo pipefail

version="${VERSION:?VERSION is required}"
group="dev.kreuzberg"
artifact="html-to-markdown"
group_path="${group//.//}"
repo_url="https://repo1.maven.org/maven2/${group_path}/${artifact}/${version}/${artifact}-${version}.pom"
max_attempts=12
attempt=1
found=false

while [ $attempt -le $max_attempts ]; do
	echo "::debug::Checking Maven Central for ${group}:${artifact}:${version} (attempt ${attempt}/${max_attempts})" >&2

	status_code=$(curl \
		--silent \
		--show-error \
		--retry 2 \
		--retry-delay 3 \
		--connect-timeout 30 \
		--max-time 30 \
		-o /dev/null \
		-w "%{http_code}" \
		"${repo_url}" 2>/dev/null || echo "000")

	if [[ "${status_code}" == "200" ]]; then
		found=true
		echo "::notice::Found ${group}:${artifact}:${version} in Maven Central after ${attempt} attempt(s)" >&2
		break
	fi

	if [[ "${status_code}" == "404" ]]; then
		break
	fi

	if [ $attempt -lt $max_attempts ]; then
		sleep_time=$((attempt * 5))
		echo "::warning::Maven Central check returned HTTP ${status_code}, retrying in ${sleep_time}s... (attempt ${attempt}/${max_attempts})" >&2
		sleep "$sleep_time"
	fi

	attempt=$((attempt + 1))
done

if [ "$found" = true ]; then
	echo "exists=true" >>"${GITHUB_OUTPUT}"
	echo "::notice::Java package ${group}:${artifact}:${version} already exists on Maven Central" >&2
else
	echo "exists=false" >>"${GITHUB_OUTPUT}"
	echo "::notice::Java package ${group}:${artifact}:${version} not found on Maven Central, will build and publish" >&2
fi
