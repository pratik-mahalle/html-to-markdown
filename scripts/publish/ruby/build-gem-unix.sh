#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

# Clean up any vendored files from previous runs before building CLI
rm -rf packages/ruby/.cargo packages/ruby/rust-vendor packages/ruby/ext/html-to-markdown-rb/native/Cargo.lock
git restore packages/ruby/ext/html-to-markdown-rb/native/Cargo.toml 2>/dev/null || true

# Build CLI binary BEFORE vendoring to avoid package collision
echo "Building CLI binary before vendoring..."
cargo build --release --package html-to-markdown-cli

# Vendor all dependencies using cargo vendor
"$SCRIPT_DIR/vendor-dependencies.sh"

pushd packages/ruby >/dev/null
bundle install
bundle exec rake clean
ruby ../../scripts/prepare_ruby_gem.rb
bundle exec rake build
popd >/dev/null
