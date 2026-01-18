#!/usr/bin/env ruby
# frozen_string_literal: true

require 'fileutils'
require 'pathname'

# ============================================================================
# Vendoring Script: html-to-markdown Ruby Bindings
# ============================================================================
#
# This script vendors the Rust html-to-markdown crate into the Ruby bindings
# package for standalone builds without workspace inheritance.
#
# It performs the following operations:
# 1. Extracts version information from the root Cargo.toml
# 2. Extracts dependency versions from workspace dependencies
# 3. Copies the crate to packages/ruby/vendor/
# 4. Generates a vendor workspace Cargo.toml
# 5. Updates the vendored crate's Cargo.toml with explicit versions
# 6. Updates Ruby native Cargo.toml to use vendored path
#
# Cross-platform compatible: macOS, Linux, Windows
# No shell or AWK dependencies

# ============================================================================
# Configuration & Paths
# ============================================================================

SCRIPT_DIR = File.expand_path(__dir__)
REPO_ROOT = File.expand_path(File.join(SCRIPT_DIR, '..', '..', '..'))
WORKSPACE_CARGO = File.join(REPO_ROOT, 'Cargo.toml')
VENDOR_DIR = File.join(REPO_ROOT, 'packages', 'ruby', 'vendor')
VENDORED_CRATE_DIR = File.join(VENDOR_DIR, 'html-to-markdown')
RUBY_NATIVE_CARGO = File.join(REPO_ROOT, 'packages', 'ruby', 'ext',
                              'html-to-markdown-rb', 'native', 'Cargo.toml')

# Dependency names to extract versions for
DEPENDENCIES = [
  'tl',
  'html5ever',
  'markup5ever_rcdom',
  'regex',
  'once_cell',
  'thiserror',
  'base64',
  'encoding_rs',
  'serde',
  'serde_json',
  'async-trait'
].freeze

# ============================================================================
# TOML Parsing Functions
# ============================================================================

# Parse a simple TOML file and extract key-value pairs
# @param file_path [String] Path to TOML file
# @return [Hash] Parsed key-value pairs
def parse_toml(file_path)
  result = {}
  current_section = nil

  File.readlines(file_path).each do |line|
    line.strip!

    # Skip empty lines and comments
    next if line.empty? || line.start_with?('#')

    # Track current section
    if line.match?(/^\[[\w.]+\]/)
      current_section = line.gsub(/[\[\]]/, '')
      next
    end

    # Store key-value pair with section context
    if line.include?('=')
      key, value = line.split('=', 2)
      key = key.strip
      value = value.strip
      full_key = current_section ? "#{current_section}.#{key}" : key
      result[full_key] = value
    end
  end

  result
end

# Extract workspace version from Cargo.toml
# @param cargo_toml_path [String] Path to root Cargo.toml
# @return [String] Version string
def extract_workspace_version(cargo_toml_path)
  content = File.read(cargo_toml_path)

  # Find [workspace.package] section and extract version
  in_workspace_section = false
  content.each_line do |line|
    in_workspace_section = true if line.match?(/^\[workspace\.package\]/)

    if in_workspace_section && line.match?(/^version\s*=/)
      # Extract quoted version string
      match = line.match(/version\s*=\s*"([^"]+)"/)
      return match[1] if match

      in_workspace_section = false
    end
  end

  raise 'Could not extract workspace version from Cargo.toml'
end

# Extract a specific dependency version from workspace dependencies
# @param cargo_toml_path [String] Path to Cargo.toml
# @param dep_name [String] Dependency name to search for
# @return [String, nil] Version string or nil if not found
def extract_dependency_version(cargo_toml_path, dep_name)
  content = File.read(cargo_toml_path)
  in_workspace_deps = false

  content.each_line do |line|
    in_workspace_deps = true if line.match?(/^\[workspace\.dependencies\]/)
    in_workspace_deps = false if line.match?(/^\[/) && !line.match?(/^\[workspace\.dependencies\]/)

    next unless in_workspace_deps

    # Match simple string version: tl = "0.7"
    if line.match?(/^#{Regexp.escape(dep_name)}\s*=\s*"/)
      match = line.match(/^#{Regexp.escape(dep_name)}\s*=\s*"([^"]+)"/)
      return match[1] if match
    end

    # Match version in object: tl = { package = "astral-tl", version = "0.7" }
    if line.match?(/^#{Regexp.escape(dep_name)}\s*=\s*\{/)
      match = line.match(/version\s*=\s*"([^"]+)"/)
      return match[1] if match
    end
  end

  nil
end

# ============================================================================
# File System Operations
# ============================================================================

# Clean and create the vendor directory
# @param vendor_path [String] Path to vendor directory
def prepare_vendor_directory(vendor_path)
  FileUtils.rm_rf(vendor_path) if Dir.exist?(vendor_path)
  FileUtils.mkdir_p(vendor_path)
end

# Copy crate to vendor directory
# @param src [String] Source crate path
# @param dst [String] Destination vendor path
def copy_crate(src, dst)
  FileUtils.cp_r(src, dst)
end

# Clean up build artifacts from vendored crate
# @param crate_path [String] Path to vendored crate
def cleanup_artifacts(crate_path)
  # Remove target directory
  target_dir = File.join(crate_path, 'target')
  FileUtils.rm_rf(target_dir) if Dir.exist?(target_dir)

  # Remove temporary files
  %w[*.swp *.bak *.tmp].each do |pattern|
    Dir.glob(File.join(crate_path, '**', pattern)).each do |file|
      FileUtils.rm_f(file)
    end
  end
end

# ============================================================================
# Cargo.toml Manipulation
# ============================================================================

# Update vendored crate's Cargo.toml with explicit versions
# @param cargo_path [String] Path to vendored Cargo.toml
# @param version [String] Version to set
def update_vendored_cargo_toml(cargo_path, version)
  content = File.read(cargo_path)

  # Replace workspace-inherited fields with explicit values
  replacements = {
    /^version\.workspace\s*=\s*true/ => "version = \"#{version}\"",
    /^edition\.workspace\s*=\s*true/ => 'edition = "2024"',
    /^rust-version\.workspace\s*=\s*true/ => 'rust-version = "1.85"',
    /^authors\.workspace\s*=\s*true/ => 'authors = ["Na\'aman Hirschfeld <nhirschfeld@gmail.com>"]',
    /^license\.workspace\s*=\s*true/ => 'license = "MIT"',
    /^repository\.workspace\s*=\s*true/ => 'repository = "https://github.com/kreuzberg-dev/html-to-markdown"',
    /^homepage\.workspace\s*=\s*true/ => 'homepage = "https://github.com/kreuzberg-dev/html-to-markdown"',
    /^documentation\.workspace\s*=\s*true/ => 'documentation = "https://docs.rs/html-to-markdown-rs"',
    /^readme\.workspace\s*=\s*true/ => 'readme = "README.md"',
    /^\[lints\]\s*\n\s*workspace\s*=\s*true/ => "[lints.rust]\nunsafe_code = \"forbid\"\nmissing_docs = \"warn\"\nunused_must_use = \"deny\"\n\n[lints.clippy]\nall = { level = \"deny\", priority = -1 }\ncargo = { level = \"deny\", priority = -1 }\npedantic = { level = \"warn\", priority = -1 }\nnursery = { level = \"warn\", priority = -1 }\nmultiple_crate_versions = \"allow\""
  }

  replacements.each do |pattern, replacement|
    content.gsub!(pattern, replacement)
  end

  File.write(cargo_path, content)
end

# Update Ruby native Cargo.toml to use vendored path and explicit versions
# @param cargo_path [String] Path to Ruby native Cargo.toml
# @param version [String] Version to set
def update_ruby_native_cargo_toml(cargo_path, version)
  content = File.read(cargo_path)

  # Replace workspace reference with path to vendored crate
  content.gsub!(
    /html-to-markdown-rs\s*=\s*\{\s*workspace\s*=\s*true,/,
    'html-to-markdown-rs = { path = "../../../vendor/html-to-markdown",'
  )

  # Replace workspace-inherited package fields with explicit values
  replacements = {
    /^version\.workspace\s*=\s*true/ => "version = \"#{version}\"",
    /^edition\.workspace\s*=\s*true/ => 'edition = "2024"',
    /^rust-version\.workspace\s*=\s*true/ => 'rust-version = "1.85"',
    /^authors\.workspace\s*=\s*true/ => 'authors = ["Na\'aman Hirschfeld <nhirschfeld@gmail.com>"]',
    /^license\.workspace\s*=\s*true/ => 'license = "MIT"',
    /^repository\.workspace\s*=\s*true/ => 'repository = "https://github.com/kreuzberg-dev/html-to-markdown"',
    /^homepage\.workspace\s*=\s*true/ => 'homepage = "https://github.com/kreuzberg-dev/html-to-markdown"',
    /^documentation\.workspace\s*=\s*true/ => 'documentation = "https://docs.rs/html-to-markdown-rs"',
    /^readme\.workspace\s*=\s*true/ => 'readme = "README.md"'
  }

  replacements.each do |pattern, replacement|
    content.gsub!(pattern, replacement)
  end

  File.write(cargo_path, content)
end

# Generate vendor workspace Cargo.toml
# @param vendor_path [String] Path to vendor directory
# @param version [String] Workspace version
# @param deps [Hash] Dependency versions keyed by name
def generate_vendor_workspace_cargo(vendor_path, version, deps)
  # Format dependency lines
  tl_line = "tl = { package = \"astral-tl\", version = \"#{deps['tl']}\" }"
  html5ever_line = "html5ever = \"#{deps['html5ever']}\""
  markup5ever_line = "markup5ever_rcdom = \"#{deps['markup5ever_rcdom']}\""
  regex_line = "regex = \"#{deps['regex']}\""
  once_cell_line = "once_cell = \"#{deps['once_cell']}\""
  thiserror_line = "thiserror = \"#{deps['thiserror']}\""
  base64_line = "base64 = \"#{deps['base64']}\""
  encoding_rs_line = "encoding_rs = \"#{deps['encoding_rs']}\""
  serde_line = "serde = { version = \"#{deps['serde']}\", features = [\"derive\"] }"
  serde_json_line = "serde_json = \"#{deps['serde_json']}\""
  async_trait_line = "async-trait = \"#{deps['async-trait']}\""

  workspace_toml = <<~TOML
    [workspace]
    members = ["html-to-markdown"]
    resolver = "2"

    [workspace.package]
    version = "#{version}"
    edition = "2024"
    rust-version = "1.85"
    authors = ["Na'aman Hirschfeld <nhirschfeld@gmail.com>"]
    license = "MIT"
    repository = "https://github.com/kreuzberg-dev/html-to-markdown"

    [workspace.dependencies]
    #{tl_line}
    #{html5ever_line}
    #{markup5ever_line}
    #{regex_line}
    #{once_cell_line}
    #{thiserror_line}
    #{base64_line}
    #{encoding_rs_line}
    #{serde_line}
    #{serde_json_line}
    #{async_trait_line}
  TOML

  File.write(File.join(vendor_path, 'Cargo.toml'), workspace_toml)
end

# ============================================================================
# Main Vendoring Logic
# ============================================================================

def main
  puts '=== Vendoring html-to-markdown crate ==='
  puts

  # Step 1: Extract versions
  puts 'Extracting versions...'
  workspace_version = extract_workspace_version(WORKSPACE_CARGO)
  puts "Extracted workspace version: #{workspace_version}"

  dependency_versions = {}
  DEPENDENCIES.each do |dep|
    dep_version = extract_dependency_version(WORKSPACE_CARGO, dep)
    dependency_versions[dep] = dep_version if dep_version
  end

  puts 'Extracted dependency versions:'
  dependency_versions.each do |name, version|
    puts "  #{name}: #{version}"
  end
  puts

  # Step 2: Prepare vendor directory
  puts 'Preparing vendor directory...'
  prepare_vendor_directory(VENDOR_DIR)
  puts "✓ Created vendor directory: #{VENDOR_DIR}"
  puts

  # Step 3: Copy crate
  puts 'Copying crate to vendor directory...'
  source_crate = File.join(REPO_ROOT, 'crates', 'html-to-markdown')
  copy_crate(source_crate, VENDORED_CRATE_DIR)
  puts "✓ Copied #{source_crate}"
  puts "  → #{VENDORED_CRATE_DIR}"
  puts

  # Step 4: Clean up artifacts
  puts 'Cleaning up build artifacts...'
  cleanup_artifacts(VENDORED_CRATE_DIR)
  puts '✓ Removed target directory and temporary files'
  puts

  # Step 5: Update vendored crate Cargo.toml
  puts 'Updating vendored crate Cargo.toml...'
  vendored_cargo_path = File.join(VENDORED_CRATE_DIR, 'Cargo.toml')
  update_vendored_cargo_toml(vendored_cargo_path, workspace_version)
  puts '✓ Replaced workspace references with explicit values'
  puts

  # Step 6: Update Ruby native Cargo.toml
  puts 'Updating Ruby native Cargo.toml...'
  update_ruby_native_cargo_toml(RUBY_NATIVE_CARGO, workspace_version)
  puts '✓ Updated path to vendored crate and replaced workspace inheritance'
  puts

  # Step 7: Generate vendor workspace Cargo.toml
  puts 'Generating vendor workspace Cargo.toml...'
  generate_vendor_workspace_cargo(VENDOR_DIR, workspace_version, dependency_versions)
  puts "✓ Created #{File.join(VENDOR_DIR, 'Cargo.toml')}"
  puts

  puts "✓ Vendoring complete (version: #{workspace_version})"
end

# ============================================================================
# Script Entry Point
# ============================================================================

main
