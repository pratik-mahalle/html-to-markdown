#!/usr/bin/env bash
set -euo pipefail

tag="${TAG:?TAG is required}"

shopt -s nullglob
packages=(dist/php-package/*.tar.gz dist/php-package/*.tar.gz.sha256)

if [[ ${#packages[@]} -eq 0 ]]; then
  echo "::warning::No PHP package artifacts found in dist/php-package/" >&2
  exit 0
fi

for file in "${packages[@]}"; do
  echo "Uploading: ${file}"
  gh release upload "${tag}" "${file}" --clobber
done
