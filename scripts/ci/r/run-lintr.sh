#!/usr/bin/env bash
set -euo pipefail

script_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
repo_root="$(cd "${script_dir}/../../.." && pwd)"

cd "${repo_root}/packages/r" && Rscript -e 'lints <- lintr::lint_package(); if (length(lints) > 0) { print(lints); quit(status = 1) }'
