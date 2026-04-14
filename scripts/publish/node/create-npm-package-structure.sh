#!/usr/bin/env bash
set -euo pipefail

pnpm --filter ./crates/html-to-markdown-node exec napi create-npm-dirs
