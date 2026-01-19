# frozen_string_literal: true

require "fileutils"
require "pathname"

root = Pathname.new(__dir__).parent

binary_name = Gem.win_platform? ? "html-to-markdown.exe" : "html-to-markdown"
source = root.join("target", "release", binary_name)

# CLI binary should already be built before vendoring
# This avoids package collision with the vendored html-to-markdown-rs crate
unless source.file?
  abort "CLI binary not found at #{source}. Please build it first with: cargo build --release --package html-to-markdown-cli"
end

puts "Using CLI binary at #{source}"

bin_dir = root.join("packages", "ruby", "lib", "bin")
FileUtils.mkdir_p(bin_dir)

plain_binary = bin_dir.join("html-to-markdown")
windows_binary = bin_dir.join("html-to-markdown.exe")

[plain_binary, windows_binary].each do |path|
  next unless path.exist?

  FileUtils.rm_f(path)
end

dest = bin_dir.join(binary_name)
FileUtils.cp(source, dest)
FileUtils.chmod(0o755, dest) unless Gem.win_platform?

puts "Copied CLI binary to #{dest}"
