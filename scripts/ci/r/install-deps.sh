#!/usr/bin/env bash
set -euo pipefail

Rscript -e 'for (pkg in c("devtools", "testthat", "rextendr", "lintr", "styler", "covr", "remotes")) { if (!requireNamespace(pkg, quietly = TRUE)) install.packages(pkg, repos = "https://cran.r-project.org") }'
