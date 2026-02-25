#!/usr/bin/env bash
set -euo pipefail

tag="${TAG:?TAG is required}"

shopt -s nullglob
archives=(dist/c-ffi/*/*.{tar.gz,zip})

if [ ${#archives[@]} -eq 0 ]; then
	echo "ERROR: No artifact files found in dist/c-ffi/"
	exit 1
fi

for archive in "${archives[@]}"; do
	if [ -f "$archive" ]; then
		gh release upload "${tag}" "${archive}" --clobber
	fi
done
