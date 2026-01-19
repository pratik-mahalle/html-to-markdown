# frozen_string_literal: true

require_relative 'lib/html_to_markdown/version'

repo_root = File.expand_path('../..', __dir__)
crate_prefix = 'packages/ruby/'
git_cmd = %(git -C "#{repo_root}" ls-files -z #{crate_prefix})
git_files =
  `#{git_cmd}`.split("\x0")
              .select { |path| path.start_with?(crate_prefix) }
              .map { |path| path.delete_prefix(crate_prefix) }
              .reject { |path| path == 'ext/html-to-markdown-rb/native/Cargo.toml' }
fallback_files = Dir.chdir(__dir__) do
  Dir.glob(
    %w[
      README.md
      ext/**/*
      exe/*
      lib/**/*.rb
      lib/bin/*
      src/**/*.rs
      spec/**/*.rb
      sig/**/*.rbs
      vendor/**/*
    ]
  )
end

# Always include rust-vendor and .cargo directories if they exist (created by vendoring script)
# Exclude native artifacts (.lib, .a, .dll, .so, .dylib) as they shouldn't be in the gem
vendor_files = Dir.chdir(__dir__) do
  (Dir.glob('rust-vendor/**/*') + Dir.glob('.cargo/**/*'))
    .select { |f| File.file?(f) }
    .grep_v(/\.(lib|a|dll|so|dylib)$/i)
end

# Include Cargo.lock and modified Cargo.toml if they exist (created by vendoring script)
cargo_lock = 'ext/html-to-markdown-rb/native/Cargo.lock'
cargo_toml = 'ext/html-to-markdown-rb/native/Cargo.toml'
vendor_files << cargo_lock if File.file?(File.join(__dir__, cargo_lock))
vendor_files << cargo_toml if File.file?(File.join(__dir__, cargo_toml))

files = git_files.empty? ? fallback_files : git_files
files = (files + vendor_files).uniq

Gem::Specification.new do |spec|
  spec.name          = 'html-to-markdown'
  spec.version       = HtmlToMarkdown::VERSION
  spec.authors       = ["Na'aman Hirschfeld"]
  spec.email         = ['nhirschfeld@gmail.com']

  spec.summary       = 'Blazing-fast HTML to Markdown conversion for Ruby, powered by Rust.'
  spec.description   = <<~DESC.strip
    html-to-markdown is a native Ruby extension built on the shared Rust engine that powers the html-to-markdown project.
    It delivers identical HTML-to-Markdown output across languages, exposes inline image extraction, and ships with a CLI for automation workflows.
  DESC
  spec.homepage      = 'https://github.com/kreuzberg-dev/html-to-markdown'
  spec.license       = 'MIT'

  spec.required_ruby_version = Gem::Requirement.new('>= 3.2')

  spec.bindir = 'exe'
  spec.executables = ['html-to-markdown']
  spec.require_paths = ['lib']

  spec.files = files
  spec.extra_rdoc_files = ['README.md']

  spec.extensions = ['ext/html-to-markdown-rb/extconf.rb']

  spec.add_dependency 'rb_sys', '>= 0.9', '< 1.0'
  spec.metadata['rubygems_mfa_required'] = 'true'
  spec.metadata['homepage_uri'] = 'https://github.com/kreuzberg-dev/html-to-markdown'
  spec.metadata['source_code_uri'] = 'https://github.com/kreuzberg-dev/html-to-markdown'
  spec.metadata['bug_tracker_uri'] = 'https://github.com/kreuzberg-dev/html-to-markdown/issues'
  spec.metadata['changelog_uri'] = 'https://github.com/kreuzberg-dev/html-to-markdown/releases'
  spec.metadata['documentation_uri'] = 'https://github.com/kreuzberg-dev/html-to-markdown/blob/main/packages/ruby/README.md'
end
