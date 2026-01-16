#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

# Vendor html-to-markdown crate
"$SCRIPT_DIR/vendor-html-to-markdown.sh"

pushd packages/ruby >/dev/null
bundle exec rake clean
ruby ../../scripts/prepare_ruby_gem.rb
bundle exec rake build
popd >/dev/null
