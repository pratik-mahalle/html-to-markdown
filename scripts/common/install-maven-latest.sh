#!/usr/bin/env bash
set -euo pipefail

: "${RUNNER_TEMP:?RUNNER_TEMP is required}"
: "${GITHUB_PATH:?GITHUB_PATH is required}"

# Pin Maven 3.9.x — Maven 4.x breaks central-publishing-maven-plugin deploy lifecycle
maven_version="3.9.11"

tmp_dir="${RUNNER_TEMP}"
if [[ "${RUNNER_OS:-}" == "Windows" ]]; then
	if command -v cygpath >/dev/null 2>&1; then
		tmp_dir="$(cygpath -u "${RUNNER_TEMP}")"
	fi
fi

maven_dir="${tmp_dir}/apache-maven-${maven_version}"
archive_path="${tmp_dir}/maven.tar.gz"

if [[ ! -d "${maven_dir}" ]]; then
	if ! curl -fsSL --retry 3 --retry-all-errors --retry-delay 5 \
		"https://repo.maven.apache.org/maven2/org/apache/maven/apache-maven/${maven_version}/apache-maven-${maven_version}-bin.tar.gz" \
		-o "${archive_path}"; then
		curl -fsSL --retry 3 --retry-all-errors --retry-delay 5 \
			"https://archive.apache.org/dist/maven/maven-3/${maven_version}/binaries/apache-maven-${maven_version}-bin.tar.gz" \
			-o "${archive_path}"
	fi
	tar -xzf "${archive_path}" -C "${tmp_dir}"
fi

echo "${maven_dir}/bin" >>"${GITHUB_PATH}"
