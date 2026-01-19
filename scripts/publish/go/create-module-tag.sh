#!/usr/bin/env bash
# Creates the Go submodule tag required for Go proxy to recognize the module.
# For modules in subdirectories, Go requires tags in the format: {subdir}/{version}
# e.g., packages/go/v2.23.0 for module github.com/kreuzberg-dev/html-to-markdown/packages/go/v2@v2.23.0
#
# Usage: create-module-tag.sh <version> [--dry-run]
# Example: create-module-tag.sh v2.23.0

set -euo pipefail

VERSION="${1:-}"
DRY_RUN="${2:-}"

if [[ -z "$VERSION" ]]; then
	echo "Error: VERSION argument required" >&2
	echo "Usage: $0 <version> [--dry-run]" >&2
	exit 1
fi

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

# Check if the main release tag exists
if ! git rev-parse --verify "refs/tags/${VERSION}" >/dev/null 2>&1; then
	echo "Error: Release tag ${VERSION} does not exist" >&2
	exit 1
fi

# Get the commit that the release tag points to
TARGET_COMMIT=$(git rev-parse --verify "refs/tags/${VERSION}^{commit}")
echo "  Target commit: ${TARGET_COMMIT}"

# Check if Go tag already exists
if git rev-parse --verify "refs/tags/${GO_TAG}" >/dev/null 2>&1; then
	EXISTING_COMMIT=$(git rev-parse --verify "refs/tags/${GO_TAG}^{commit}")
	if [[ "$EXISTING_COMMIT" == "$TARGET_COMMIT" ]]; then
		echo "Go module tag ${GO_TAG} already exists and points to correct commit"
		exit 0
	else
		echo "Warning: Go module tag ${GO_TAG} exists but points to different commit"
		echo "  Existing: ${EXISTING_COMMIT}"
		echo "  Expected: ${TARGET_COMMIT}"
		exit 1
	fi
fi

if [[ "$DRY_RUN" == "--dry-run" ]]; then
	echo "[DRY RUN] Would create tag: ${GO_TAG} -> ${TARGET_COMMIT}"
	exit 0
fi

# Create the Go submodule tag
git tag "${GO_TAG}" "${TARGET_COMMIT}"
echo "Created tag: ${GO_TAG}"

# Push the tag
git push origin "${GO_TAG}"
echo "Pushed tag: ${GO_TAG}"

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
