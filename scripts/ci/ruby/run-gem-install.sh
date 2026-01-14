#!/usr/bin/env bash
set -euo pipefail

pushd packages/ruby >/dev/null
bundle exec rake clean
ruby ../../scripts/prepare_ruby_gem.rb
bundle exec rake build
popd >/dev/null

GEM_FILE=$(find packages/ruby/pkg -name "*.gem" -print -quit)
TMP_DIR=$(mktemp -d)
trap 'rm -rf "$TMP_DIR"' EXIT

cp "$GEM_FILE" "$TMP_DIR/"
cd "$TMP_DIR"
gem install "$(basename "$GEM_FILE")" --no-document
