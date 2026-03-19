#!/usr/bin/env bash
# Creates the Go submodule tag required for Go proxy to recognize the module.
# For modules in subdirectories, Go requires tags in the format: {subdir}/{version}
# e.g., packages/go/v2.23.0 for module github.com/kreuzberg-dev/html-to-markdown/packages/go/v2@v2.23.0
#
# Usage: create-module-tag.sh <version> [--dry-run]
# Example: create-module-tag.sh v2.23.0

set -euo pipefail

VERSION="${1:?Version argument required (e.g. v2.25.1)}"

# Ensure version starts with 'v'
if [[ ! "$VERSION" =~ ^v ]]; then
  VERSION="v${VERSION}"
fi

# The Go submodule tag format for modules in subdirectories
# Module path: github.com/kreuzberg-dev/html-to-markdown/packages/go/v2
# Tag format: packages/go/{version} (the /v2 is part of the module path, not the tag prefix)
GO_TAG="packages/go/${VERSION}"

echo "Creating Go module tag: ${GO_TAG}"
echo "  For module: github.com/kreuzberg-dev/html-to-markdown/packages/go/v2@${VERSION}"

# Check if Go tag already exists locally
if git rev-parse "$GO_TAG" >/dev/null 2>&1; then
  echo "::notice::Go module tag $GO_TAG already exists locally; skipping."
  exit 0
fi

# Check if tag exists on remote
if git ls-remote --tags origin | grep -q "refs/tags/${GO_TAG}$"; then
  echo "::notice::Go module tag $GO_TAG already exists on remote; skipping."
  exit 0
fi

if [[ "${2:-}" == "--dry-run" ]]; then
  echo "[DRY RUN] Would create tag: ${GO_TAG} -> ${VERSION}"
  exit 0
fi

git tag "$GO_TAG" "$VERSION"
git push origin "$GO_TAG"

echo "Go module tag created and pushed: ${GO_TAG}"

# Trigger Go proxy to fetch the module (optional but speeds up availability)
echo "Triggering Go proxy fetch..."
GOPROXY_URL="https://proxy.golang.org/github.com/kreuzberg-dev/html-to-markdown/packages/go/v2/@v/${VERSION}.info"
if curl -sf "${GOPROXY_URL}" >/dev/null 2>&1; then
  echo "Go proxy successfully fetched module version"
else
  echo "Note: Go proxy may take a few minutes to index the new version"
fi

echo ""
echo "Go module published successfully!"
echo "  Module: github.com/kreuzberg-dev/html-to-markdown/packages/go/v2@${VERSION}"
echo "  Install: go get github.com/kreuzberg-dev/html-to-markdown/packages/go/v2@${VERSION}"
