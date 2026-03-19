#!/usr/bin/env bash
set -euo pipefail

platform="${1:?platform is required (linux-x64, linux-arm64, darwin-arm64, windows-x64)}"
version="${2:?version is required}"
out_dir="${3:?out_dir is required}"

# Determine platform-specific file names and extensions
case "${platform}" in
linux-x64 | linux-arm64)
  shared_lib="libhtml_to_markdown_ffi.so"
  static_lib="libhtml_to_markdown_ffi.a"
  archive_ext="tar.gz"
  libs_private="-lpthread -ldl -lm"
  ;;
darwin-arm64)
  shared_lib="libhtml_to_markdown_ffi.dylib"
  static_lib="libhtml_to_markdown_ffi.a"
  archive_ext="tar.gz"
  libs_private="-framework CoreFoundation -framework Security -lpthread"
  ;;
windows-x64)
  shared_lib="html_to_markdown_ffi.dll"
  static_lib="html_to_markdown_ffi.lib"
  archive_ext="tar.gz"
  libs_private="-lws2_32 -luserenv -lbcrypt"
  ;;
*)
  echo "Unsupported platform: ${platform}" >&2
  exit 1
  ;;
esac

# Build the FFI crate (produces both cdylib and staticlib)
echo "Building html-to-markdown-ffi for ${platform}..."
cargo build -p html-to-markdown-ffi --release

# Locate built libraries
release_dir="target/release"

find_lib() {
  local name="$1"
  local found
  found="$(find "${release_dir}" -maxdepth 1 -type f -name "${name}" -print -quit)"
  echo "${found}"
}

shared_path="$(find_lib "${shared_lib}")"
static_path="$(find_lib "${static_lib}")"

if [[ -z "${shared_path}" || ! -f "${shared_path}" ]]; then
  echo "Failed to locate shared library ${shared_lib} in ${release_dir}" >&2
  ls -la "${release_dir}"/*.{so,dylib,dll} 2>/dev/null || true
  exit 1
fi

if [[ -z "${static_path}" || ! -f "${static_path}" ]]; then
  echo "Warning: Static library ${static_lib} not found; skipping" >&2
fi

# Source paths
header_path="crates/html-to-markdown-ffi/html_to_markdown.h"
cmake_config="crates/html-to-markdown-ffi/cmake/html-to-markdown-ffi-config.cmake"
cmake_version_template="crates/html-to-markdown-ffi/cmake/html-to-markdown-ffi-config-version.cmake"
pc_template="crates/html-to-markdown-ffi/html-to-markdown-ffi.pc.in"
license_path="LICENSE"

# Stage the distribution tree
dist_name="html-to-markdown-ffi-${version}-${platform}"
stage_dir="$(mktemp -d)"
trap 'rm -rf "${stage_dir}"' EXIT

root="${stage_dir}/${dist_name}"
mkdir -p "${root}/include" "${root}/lib/pkgconfig" "${root}/cmake"

# Copy header
if [[ -f "${header_path}" ]]; then
  cp -f "${header_path}" "${root}/include/html_to_markdown.h"
else
  echo "Error: Header file not found at ${header_path}" >&2
  exit 1
fi

# Copy shared library
cp -f "${shared_path}" "${root}/lib/${shared_lib}"

# Copy static library (if it exists)
if [[ -n "${static_path}" && -f "${static_path}" ]]; then
  cp -f "${static_path}" "${root}/lib/${static_lib}"
fi

# Generate pkg-config .pc file from template
if [[ -f "${pc_template}" ]]; then
  sed -e "s|@PREFIX@|/usr/local|g" \
    -e "s|@VERSION@|${version}|g" \
    -e "s|@LIBS_PRIVATE@|${libs_private}|g" \
    "${pc_template}" >"${root}/lib/pkgconfig/html-to-markdown.pc"
else
  echo "Warning: pkg-config template not found at ${pc_template}" >&2
fi

# Copy CMake config
if [[ -f "${cmake_config}" ]]; then
  cp -f "${cmake_config}" "${root}/cmake/html-to-markdown-ffi-config.cmake"
else
  echo "Warning: CMake config not found at ${cmake_config}" >&2
fi

# Generate CMake version config with correct version from argument
IFS='.' read -r major minor patch <<<"${version}"
if [[ -f "${cmake_version_template}" ]]; then
  sed -e "s|set(PACKAGE_VERSION_MAJOR [0-9]*)|set(PACKAGE_VERSION_MAJOR ${major})|" \
    -e "s|set(PACKAGE_VERSION_MINOR [0-9]*)|set(PACKAGE_VERSION_MINOR ${minor})|" \
    -e "s|set(PACKAGE_VERSION_PATCH [0-9]*)|set(PACKAGE_VERSION_PATCH ${patch})|" \
    "${cmake_version_template}" >"${root}/cmake/html-to-markdown-ffi-config-version.cmake"
else
  echo "Warning: CMake version config not found at ${cmake_version_template}" >&2
fi

# Copy LICENSE
if [[ -f "${license_path}" ]]; then
  cp -f "${license_path}" "${root}/LICENSE"
else
  echo "Warning: LICENSE file not found" >&2
fi

# Create the archive
mkdir -p "${out_dir}"
archive_name="${dist_name}.${archive_ext}"

if [[ "${archive_ext}" == "tar.gz" ]]; then
  tar -czf "${out_dir}/${archive_name}" -C "${stage_dir}" "${dist_name}"
else
  echo "Unsupported archive extension: ${archive_ext}" >&2
  exit 1
fi

echo "Packaged ${out_dir}/${archive_name}"
