#!/bin/bash
set -e

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
cd "$SCRIPT_DIR"

echo "Installing FFI library from remote..."
go generate github.com/kreuzberg-dev/html-to-markdown/packages/go/v2/htmltomarkdown

echo "Running tests..."
go test -v -count=1 ./...
