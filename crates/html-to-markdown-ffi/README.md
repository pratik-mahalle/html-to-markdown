# html-to-markdown-ffi

C Foreign Function Interface (FFI) bindings for the html-to-markdown conversion library.

## Overview

This crate provides a C-compatible API for converting HTML to Markdown. It is the foundation for language bindings in Java (Panama FFI), Go (cgo), C# (P/Invoke), and other languages with C interoperability.

## Installation

### Build from Source

```bash
cargo build --release -p html-to-markdown-ffi
```

This produces:
- **Dynamic library**: `target/release/libhtml_to_markdown_ffi.{so,dylib,dll}`
- **Static library**: `target/release/libhtml_to_markdown_ffi.{a,lib}`
- **Header file**: `crates/html-to-markdown-ffi/html_to_markdown.h` (auto-generated via cbindgen)

### Installing from Release Artifacts

Pre-built packages are available from the [releases page](https://github.com/kreuzberg-dev/html-to-markdown/releases).

Each `html-to-markdown-ffi-{version}-{platform}.tar.gz` archive contains:

```
include/html_to_markdown.h
lib/libhtml_to_markdown_ffi.{so|dylib|dll}
lib/libhtml_to_markdown_ffi.{a|lib}
lib/pkgconfig/html-to-markdown.pc
cmake/html-to-markdown-ffi-config.cmake
cmake/html-to-markdown-ffi-config-version.cmake
LICENSE
```

Platforms: `linux-x64`, `linux-arm64`, `darwin-arm64`, `windows-x64`

Installation:

```bash
tar -xzf html-to-markdown-ffi-2.25.2-linux-x64.tar.gz

sudo cp include/html_to_markdown.h /usr/local/include/
sudo cp lib/libhtml_to_markdown_ffi.* /usr/local/lib/
sudo mkdir -p /usr/local/lib/pkgconfig
sudo cp lib/pkgconfig/html-to-markdown.pc /usr/local/lib/pkgconfig/
sudo mkdir -p /usr/local/lib/cmake/html-to-markdown-ffi
sudo cp cmake/*.cmake /usr/local/lib/cmake/html-to-markdown-ffi/
sudo ldconfig  # Linux only
```

### Homebrew

```bash
brew install kreuzberg-dev/tap/libhtml-to-markdown
```

### pkg-config

```bash
pkg-config --cflags html-to-markdown   # -I/usr/local/include
pkg-config --libs html-to-markdown     # -L/usr/local/lib -lhtml_to_markdown_ffi
```

### CMake

```cmake
find_package(html-to-markdown-ffi REQUIRED)
target_link_libraries(my_app PRIVATE html-to-markdown-ffi::html-to-markdown-ffi)
```

## Quick Start

```c
#include <stdio.h>
#include <html_to_markdown.h>

int main(void) {
    char *md = html_to_markdown_convert("<h1>Hello</h1><p>World</p>");
    if (md) {
        printf("%s\n", md);
        html_to_markdown_free_string(md);
    } else {
        printf("Error: %s\n", html_to_markdown_last_error());
    }
    return 0;
}
```

Compile:

```bash
cc -o example example.c $(pkg-config --cflags --libs html-to-markdown)
```

## Memory Management

- Strings returned by `html_to_markdown_convert()` must be freed with `html_to_markdown_free_string()`.
- Passing `NULL` to `html_to_markdown_free_string()` is safe (no-op).
- Error strings from `html_to_markdown_last_error()` are thread-local and must NOT be freed.

## Error Handling

```c
char *result = html_to_markdown_convert(html);
if (!result) {
    const char *msg = html_to_markdown_last_error();
    uint32_t code = html_to_markdown_last_error_code();
    const char *name = html_to_markdown_error_code_name(code);
    fprintf(stderr, "Error [%s]: %s\n", name, msg);
}
```

Error codes: `Ok` (0), `InvalidUtf8` (1), `Parse` (2), `Visitor` (3), `Memory` (4), `Internal` (5).

## Version Constants

The generated header includes compile-time version constants:

```c
#define HTML_TO_MARKDOWN_VERSION_MAJOR 2
#define HTML_TO_MARKDOWN_VERSION_MINOR 25
#define HTML_TO_MARKDOWN_VERSION_PATCH 2
#define HTML_TO_MARKDOWN_VERSION "2.25.2"
```

Runtime version: `html_to_markdown_version()`.

## Thread Safety

All functions are thread-safe. Error state is stored in thread-local storage, so concurrent calls from different threads do not interfere.

## ABI Stability

The C API follows semantic versioning. Within a major version, no breaking changes to function signatures, struct layouts, or enum values.

## Platform Support

| Platform | Architecture | Library |
|----------|-------------|---------|
| Linux | x86_64, aarch64 | `.so` + `.a` |
| macOS | arm64 | `.dylib` + `.a` |
| Windows | x86_64 | `.dll` + `.lib` |

## License

MIT
