#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

# Vendor html-to-markdown crate
ruby "$SCRIPT_DIR/vendor-html-to-markdown.rb"

pushd packages/ruby >/dev/null
bundle install
bundle exec rake clean
ruby ../../scripts/prepare_ruby_gem.rb
bundle exec rake build
popd >/dev/null
