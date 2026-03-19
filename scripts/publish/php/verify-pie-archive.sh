#!/usr/bin/env bash
set -euo pipefail

version="${VERSION:?VERSION is required}"
tmp_dir="$(mktemp -d)"
tar -xzf "build/artifacts/php_html_to_markdown-${version}-src.tgz" -C "${tmp_dir}"
echo "Extracted archive root:"
ls -la "${tmp_dir}"
php build/pie.phar repository:add path "${tmp_dir}"
CARGO_BIN="$(command -v cargo)"
if [[ "$(uname -s)" == "Darwin" ]]; then
  export RUSTFLAGS="${RUSTFLAGS:-} -C link-arg=-Wl,-undefined,dynamic_lookup"
fi
php build/pie.phar build 'kreuzberg-dev/html-to-markdown:*@dev' --working-dir "${tmp_dir}" --with-cargo-bin="${CARGO_BIN}"
