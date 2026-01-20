#!/usr/bin/env bash
set -euo pipefail

PACKAGE="@kreuzberg/html-to-markdown-node"
MAX_ATTEMPTS=30
SLEEP_SECONDS=10

echo "Waiting for ${PACKAGE} to be available on npm registry..."

for i in $(seq 1 ${MAX_ATTEMPTS}); do
	echo "Attempt ${i}/${MAX_ATTEMPTS}: Checking if ${PACKAGE} is available..."

	if npm view "${PACKAGE}" version &>/dev/null; then
		echo "✓ ${PACKAGE} is available on npm!"
		exit 0
	fi

	if [ "${i}" -lt "${MAX_ATTEMPTS}" ]; then
		echo "Package not yet available. Waiting ${SLEEP_SECONDS} seconds before retry..."
		sleep ${SLEEP_SECONDS}
	fi
done

echo "ERROR: ${PACKAGE} did not become available after ${MAX_ATTEMPTS} attempts"
exit 1
