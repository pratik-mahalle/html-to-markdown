#!/usr/bin/env bash
set -euo pipefail

version="${VERSION:?VERSION is required}"
echo "CRAN package ${version} already published; skipping." >>"${GITHUB_STEP_SUMMARY}"
