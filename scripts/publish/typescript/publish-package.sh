#!/usr/bin/env bash
set -euo pipefail

# NPM_TOKEN authentication for scoped packages
if [[ -z "${NPM_TOKEN:-}" ]]; then
	echo "ERROR: NPM_TOKEN is not set. Required for publishing scoped @kreuzberg/* packages."
	exit 1
fi

# Configure npm authentication
cat >~/.npmrc <<'EOF'
//registry.npmjs.org/:_authToken=${NPM_TOKEN}
@kreuzberg:registry=https://registry.npmjs.org/
EOF

cd packages/typescript

# Use pnpm publish to properly resolve workspace:* dependencies to actual versions
publish_log=$(mktemp)
set +e
pnpm publish --access public --no-git-checks 2>&1 | tee "${publish_log}"
status=${PIPESTATUS[0]}
set -e
if [[ "${status}" -ne 0 ]]; then
	if grep -q "previously published versions" "${publish_log}" || grep -q "You cannot publish over" "${publish_log}"; then
		echo "TypeScript wrapper package already published; skipping."
	else
		exit "${status}"
	fi
fi
