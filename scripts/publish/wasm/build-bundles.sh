#!/usr/bin/env bash
set -euo pipefail

pnpm --filter @kreuzberg/html-to-markdown-wasm run build:all
