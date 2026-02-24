#!/usr/bin/env bash
set -euo pipefail

tag="${TAG:?TAG is required}"

shopt -s nullglob
for pkg in dist/r/*.tar.gz; do
	gh release upload "${tag}" "${pkg}" --clobber
done
