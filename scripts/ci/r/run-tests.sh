#!/usr/bin/env bash
set -euo pipefail

script_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
repo_root="$(cd "${script_dir}/../../.." && pwd)"

"${repo_root}/scripts/publish/r/stage-rust-core.sh"

cd "${repo_root}/packages/r"

# Ensure devtools is available (may be missing after cache invalidation)
Rscript -e 'if (!requireNamespace("devtools", quietly = TRUE)) install.packages("devtools", repos = "https://cran.r-project.org")'

Rscript -e 'devtools::test()'
