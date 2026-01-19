#!/usr/bin/env bash
set -euo pipefail

cd packages/typescript

# Build only the TypeScript part (native bindings are already published)
# The native bindings should already be available from npm at this point
# since we depend on publish-node completing first
pnpm exec tsc --project tsconfig.json

echo "TypeScript wrapper package built successfully"
