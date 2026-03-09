#!/usr/bin/env bash
set -euo pipefail

shopt -s nullglob
files=(packages/ruby/pkg/*.gem)
if [ ${#files[@]} -eq 0 ]; then
	echo "No gems were produced" >&2
	exit 1
fi

# Verify each gem contains the required Cargo.lock for vendored git dependencies
for gem in "${files[@]}"; do
	echo "Verifying gem: $(basename "$gem")"

	# Extract file list from the gem's data.tar.gz
	if ! gem spec "$gem" --ruby 2>/dev/null | grep -q 'ext/html-to-markdown-rb/native/Cargo.lock'; then
		# Fallback: check the actual gem contents
		gem_contents=$(gem contents "$gem" --spec 2>/dev/null || true)
		if [ -z "$gem_contents" ]; then
			# Use tar to inspect the gem directly
			gem_files=$(tar -xf "$gem" -O data.tar.gz 2>/dev/null | tar -tz 2>/dev/null || true)
			if echo "$gem_files" | grep -q 'ext/html-to-markdown-rb/native/Cargo.lock'; then
				echo "  ✓ Cargo.lock found in gem (tar inspection)"
			else
				echo "ERROR: ext/html-to-markdown-rb/native/Cargo.lock is MISSING from $(basename "$gem")" >&2
				echo "This will cause 'gem install' to fail when building native extensions." >&2
				echo "The vendoring step must generate Cargo.lock before gem build." >&2
				exit 1
			fi
		fi
	else
		echo "  ✓ Cargo.lock found in gem spec"
	fi

	# Also verify .cargo/config.toml and rust-vendor/ are present
	gem_files=$(tar -xf "$gem" -O data.tar.gz 2>/dev/null | tar -tz 2>/dev/null || true)
	if ! echo "$gem_files" | grep -q '.cargo/config.toml'; then
		echo "WARNING: .cargo/config.toml missing from gem - vendored sources may not resolve" >&2
	fi
	if ! echo "$gem_files" | grep -q 'rust-vendor/'; then
		echo "WARNING: rust-vendor/ directory missing from gem - dependencies not vendored" >&2
	fi
done

echo "All gem artifacts verified successfully"
