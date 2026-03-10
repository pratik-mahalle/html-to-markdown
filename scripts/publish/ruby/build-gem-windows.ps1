$ErrorActionPreference = "Stop"

$workspace = ridk exec bash -lc "cygpath -au '$env:GITHUB_WORKSPACE'"
$gemdir = "$workspace/packages/ruby"

# Build CLI binary BEFORE vendoring to avoid package collision
Write-Host "Building CLI binary before vendoring..."
ridk exec bash -lc "cd $workspace && export RUSTUP_TOOLCHAIN=stable-gnu && cargo build --release --package html-to-markdown-cli"

# Vendor all dependencies using cargo vendor
ridk exec bash -lc "cd $workspace && scripts/publish/ruby/vendor-dependencies.sh"

ridk exec bash -lc "cd $workspace && export RUSTUP_TOOLCHAIN=stable-gnu && ruby scripts/prepare_ruby_gem.rb"
# Note: Skipping 'rake clean' because it deletes the CLI binary we just built
ridk exec bash -lc "cd $gemdir && export RUSTUP_TOOLCHAIN=stable-gnu CC=x86_64-w64-mingw32-gcc CXX=x86_64-w64-mingw32-g++ && bundle exec rake build"
# Also build a native platform gem with precompiled extension
ridk exec bash -lc "cd $gemdir && export RUSTUP_TOOLCHAIN=stable-gnu CC=x86_64-w64-mingw32-gcc CXX=x86_64-w64-mingw32-g++ && bundle exec rake native gem" 2>$null
if ($LASTEXITCODE -ne 0) { Write-Host "WARNING: native gem build failed, source gem still available" }
