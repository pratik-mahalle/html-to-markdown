# html-to-markdown-ffi

<div align="center" style="display: flex; flex-wrap: wrap; gap: 8px; justify-content: center; margin: 20px 0;">
  <!-- Language Bindings -->
  <a href="https://crates.io/crates/html-to-markdown-rs">
    <img src="https://img.shields.io/crates/v/html-to-markdown-rs?label=Rust&color=007ec6" alt="Rust">
  </a>
  <a href="https://pypi.org/project/html-to-markdown/">
    <img src="https://img.shields.io/pypi/v/html-to-markdown?label=Python&color=007ec6" alt="Python">
  </a>
  <a href="https://www.npmjs.com/package/@kreuzberg/html-to-markdown-node">
    <img src="https://img.shields.io/npm/v/@kreuzberg/html-to-markdown-node?label=Node.js&color=007ec6" alt="Node.js">
  </a>
  <a href="https://www.npmjs.com/package/@kreuzberg/html-to-markdown-wasm">
    <img src="https://img.shields.io/npm/v/@kreuzberg/html-to-markdown-wasm?label=WASM&color=007ec6" alt="WASM">
  </a>
  <a href="https://central.sonatype.com/artifact/dev.kreuzberg/html-to-markdown">
    <img src="https://img.shields.io/maven-central/v/dev.kreuzberg/html-to-markdown?label=Java&color=007ec6" alt="Java">
  </a>
  <a href="https://pkg.go.dev/github.com/kreuzberg-dev/html-to-markdown/packages/go/v2/htmltomarkdown">
    <img src="https://img.shields.io/badge/Go-v2.25.1-007ec6" alt="Go">
  </a>
  <a href="https://www.nuget.org/packages/KreuzbergDev.HtmlToMarkdown/">
    <img src="https://img.shields.io/nuget/v/KreuzbergDev.HtmlToMarkdown?label=C%23&color=007ec6" alt="C#">
  </a>
  <a href="https://packagist.org/packages/kreuzberg-dev/html-to-markdown">
    <img src="https://img.shields.io/packagist/v/kreuzberg-dev/html-to-markdown?label=PHP&color=007ec6" alt="PHP">
  </a>
  <a href="https://rubygems.org/gems/html-to-markdown">
    <img src="https://img.shields.io/gem/v/html-to-markdown?label=Ruby&color=007ec6" alt="Ruby">
  </a>
  <a href="https://hex.pm/packages/html_to_markdown">
    <img src="https://img.shields.io/hexpm/v/html_to_markdown?label=Elixir&color=007ec6" alt="Elixir">
  </a>
  <a href="https://kreuzberg-dev.r-universe.dev/htmltomarkdown">
    <img src="https://img.shields.io/badge/R-htmltomarkdown-007ec6" alt="R">
  </a>
  <a href="https://github.com/kreuzberg-dev/html-to-markdown/releases">
    <img src="https://img.shields.io/badge/C-FFI-007ec6" alt="C">
  </a>

  <!-- Project Info -->
  <a href="https://github.com/kreuzberg-dev/html-to-markdown/blob/main/LICENSE">
    <img src="https://img.shields.io/badge/License-MIT-blue.svg" alt="License">
  </a>
</div>

<img width="1128" height="191" alt="html-to-markdown" src="https://github.com/user-attachments/assets/419fc06c-8313-4324-b159-4b4d3cfce5c0" />

<div align="center" style="margin-top: 20px;">
  <a href="https://discord.gg/pXxagNK2zN">
      <img height="22" src="https://img.shields.io/badge/Discord-Join%20our%20community-7289da?logo=discord&logoColor=white" alt="Discord">
  </a>
</div>

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

## Architecture

### FFI Bridge Layers

```
Language-Specific Bindings
    ↓
html-to-markdown FFI C Library (crates/html-to-markdown-ffi) ← This crate
    ↓
Rust Core Library (crates/html-to-markdown)
```

### Binding Support

| Language | FFI Mechanism | Integration |
|----------|--------------|-------------|
| Java | Panama FFI (JDK 22+) | `packages/java` |
| Go | cgo | `packages/go` |
| C# | P/Invoke | `packages/csharp` |
| C/C++ | Direct linking | Header + library |

### Key Components

- **Core conversion** -- HTML→Markdown via null-terminated strings or byte buffers
- **Metadata extraction** -- Title, description, language, OpenGraph data as JSON
- **Visitor pattern** -- Per-element callbacks for custom conversion logic
- **Error handling** -- Thread-local error storage with typed error codes
- **Profiling** -- Flamegraph generation for performance analysis

## API Reference

### Core Conversion (3 functions)

```c
// Basic conversion -- returns malloc'd string or NULL on error
char *html_to_markdown_convert(const char *html);

// Conversion with output length
char *html_to_markdown_convert_with_len(const char *html, size_t *len_out);

// Conversion from raw UTF-8 bytes with output length
char *html_to_markdown_convert_bytes_with_len(const uint8_t *html, size_t len, size_t *len_out);
```

### Metadata Extraction (3 functions)

```c
// Convert with metadata -- metadata_json_out receives a JSON string
char *html_to_markdown_convert_with_metadata(const char *html, char **metadata_json_out);

// Convert with metadata and output lengths
char *html_to_markdown_convert_with_metadata_with_len(
    const char *html, char **metadata_json_out,
    size_t *markdown_len_out, size_t *metadata_len_out);

// Convert bytes with metadata and output lengths
char *html_to_markdown_convert_with_metadata_bytes_with_len(
    const uint8_t *html, size_t len, char **metadata_json_out,
    size_t *markdown_len_out, size_t *metadata_len_out);
```

### Visitor Pattern (4 functions)

```c
// Create a visitor from a callback table (returns opaque handle or NULL)
HtmlToMarkdownVisitor html_to_markdown_visitor_create(
    const HtmlToMarkdownVisitorCallbacks *callbacks);

// Free a visitor handle (NULL-safe)
void html_to_markdown_visitor_free(HtmlToMarkdownVisitor visitor);

// Convert with visitor callbacks (null-terminated string)
char *html_to_markdown_convert_with_visitor(
    const char *html, HtmlToMarkdownVisitor visitor, size_t *len_out);

// Convert bytes with visitor callbacks
char *html_to_markdown_convert_bytes_with_visitor(
    const uint8_t *html, size_t len, HtmlToMarkdownVisitor visitor, size_t *len_out);
```

### Visit Result Constructors (5 functions)

```c
HtmlToMarkdownVisitResult html_to_markdown_visit_result_continue(void);
HtmlToMarkdownVisitResult html_to_markdown_visit_result_custom(char *output);     // output must be malloc'd
HtmlToMarkdownVisitResult html_to_markdown_visit_result_skip(void);
HtmlToMarkdownVisitResult html_to_markdown_visit_result_preserve_html(void);
HtmlToMarkdownVisitResult html_to_markdown_visit_result_error(char *message);     // message must be malloc'd
```

### Error Handling (3 functions)

```c
// Get the last error message (thread-local, do NOT free)
const char *html_to_markdown_last_error(void);

// Get the last error code (0 = Ok)
uint32_t html_to_markdown_last_error_code(void);

// Get the name of an error code ("Ok", "InvalidUtf8", etc.)
const char *html_to_markdown_error_code_name(uint32_t code);
```

### Utility (2 functions)

```c
// Get the library version string (static, do NOT free)
const char *html_to_markdown_version(void);

// Free a string returned by any convert function (NULL-safe)
void html_to_markdown_free_string(char *s);
```

### Profiling (2 functions)

```c
// Start flamegraph profiling to the given file path
bool html_to_markdown_profile_start(const char *output, int32_t frequency);

// Stop profiling and flush the flamegraph
bool html_to_markdown_profile_stop(void);
```

## Visitor Pattern Example

The visitor pattern lets you intercept individual HTML elements during conversion and provide custom markdown output, skip elements, preserve raw HTML, or abort with an error.

```c
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <html_to_markdown.h>

// Skip all images during conversion
static HtmlToMarkdownVisitResult skip_images(
    const char *src, const char *alt, const char *title, void *user_data) {
    (void)src; (void)alt; (void)title; (void)user_data;
    return html_to_markdown_visit_result_skip();
}

// Custom heading output: prefix with emoji
static HtmlToMarkdownVisitResult custom_heading(
    uint8_t level, const char *content, const char *id, void *user_data) {
    (void)id; (void)user_data;
    char *output = (char *)malloc(strlen(content) + 32);
    snprintf(output, strlen(content) + 32, ">> %s", content);
    return html_to_markdown_visit_result_custom(output);
}

int main(void) {
    HtmlToMarkdownVisitorCallbacks callbacks = {0};
    callbacks.visit_image = /* cast */ skip_images;
    callbacks.visit_heading = /* cast */ custom_heading;

    HtmlToMarkdownVisitor visitor = html_to_markdown_visitor_create(&callbacks);
    if (!visitor) {
        fprintf(stderr, "Failed to create visitor: %s\n", html_to_markdown_last_error());
        return 1;
    }

    const char *html = "<h1>Title</h1><img src='photo.jpg'/><p>Text</p>";
    size_t len = 0;
    char *md = html_to_markdown_convert_with_visitor(html, visitor, &len);
    if (md) {
        printf("%s\n", md);
        html_to_markdown_free_string(md);
    }

    html_to_markdown_visitor_free(visitor);
    return 0;
}
```

## Metadata Extraction Example

Metadata extraction returns a JSON string containing document metadata (title, description, language, OpenGraph tags, etc.) alongside the converted markdown.

```c
#include <stdio.h>
#include <html_to_markdown.h>

int main(void) {
    const char *html =
        "<html><head>"
        "<title>My Page</title>"
        "<meta name=\"description\" content=\"A sample page\">"
        "<meta property=\"og:title\" content=\"OG Title\">"
        "</head><body><h1>Hello</h1><p>World</p></body></html>";

    char *metadata_json = NULL;
    size_t md_len = 0, meta_len = 0;

    char *md = html_to_markdown_convert_with_metadata_with_len(
        html, &metadata_json, &md_len, &meta_len);

    if (md && metadata_json) {
        printf("Markdown (%zu bytes):\n%s\n", md_len, md);
        printf("Metadata (%zu bytes):\n%s\n", meta_len, metadata_json);
        html_to_markdown_free_string(md);
        html_to_markdown_free_string(metadata_json);
    } else {
        fprintf(stderr, "Error: %s\n", html_to_markdown_last_error());
    }

    return 0;
}
```

## Type Definitions

### Enums

```c
// Visit result actions
typedef enum HtmlToMarkdownVisitResultType {
    Continue     = 0,  // Use default conversion
    Custom       = 1,  // Replace with custom_output
    Skip         = 2,  // Omit element entirely
    PreserveHtml = 3,  // Keep raw HTML verbatim
    Error        = 4,  // Abort with error_message
} HtmlToMarkdownVisitResultType;

// Error classification
enum HtmlToMarkdownErrorCode {
    Ok         = 0,
    InvalidUtf8 = 1,
    Parse      = 2,
    Visitor    = 3,
    Memory     = 4,
    Internal   = 5,
};
typedef uint32_t HtmlToMarkdownErrorCode;
```

### Structs

```c
// Visitor callback result
typedef struct HtmlToMarkdownVisitResult {
    HtmlToMarkdownVisitResultType result_type;
    char *custom_output;   // Set when result_type == Custom (malloc'd, freed by FFI)
    char *error_message;   // Set when result_type == Error (malloc'd, freed by FFI)
} HtmlToMarkdownVisitResult;

// Callback table -- set fields to NULL for default behavior
typedef struct HtmlToMarkdownVisitorCallbacks {
    void *user_data;
    // 37 callback fields for individual HTML elements:
    // visit_text, visit_link, visit_image, visit_heading,
    // visit_emphasis, visit_strong, visit_code_block,
    // visit_code_inline, visit_blockquote, visit_list_start,
    // visit_list_end, visit_list_item, visit_horizontal_rule,
    // visit_line_break, visit_table_start, visit_table_end,
    // visit_table_row, visit_strikethrough, visit_underline,
    // visit_subscript, visit_superscript, visit_mark,
    // visit_details, visit_summary, visit_figure_start,
    // visit_figure_end, visit_figcaption, visit_audio, visit_video,
    // visit_iframe, visit_form, visit_input, visit_button,
    // visit_definition_list_start, visit_definition_list_end,
    // visit_definition_term, visit_definition_description,
    // visit_custom_element, visit_element_start, visit_element_end
} HtmlToMarkdownVisitorCallbacks;
```

### Opaque Types

```c
typedef void *HtmlToMarkdownVisitor;  // Opaque visitor handle
```

## Memory Management

- Strings returned by `html_to_markdown_convert()` and variants must be freed with `html_to_markdown_free_string()`.
- Passing `NULL` to `html_to_markdown_free_string()` is safe (no-op).
- Error strings from `html_to_markdown_last_error()` are thread-local and must NOT be freed.
- Metadata JSON strings from `html_to_markdown_convert_with_metadata()` must be freed with `html_to_markdown_free_string()`.
- Visitor handles from `html_to_markdown_visitor_create()` must be freed with `html_to_markdown_visitor_free()`.
- Custom output and error message strings passed to visit result constructors must be `malloc()`'d -- the FFI layer takes ownership and frees them.

## Error Handling

```c
char *result = html_to_markdown_convert(html);
if (!result) {
    const char *msg = html_to_markdown_last_error();
    uint32_t code = html_to_markdown_last_error_code();
    const char *name = html_to_markdown_error_code_name(code);
    fprintf(stderr, "Error [%s] (code %u): %s\n", name, code, msg);
}
```

| Code | Name | Description |
|------|------|-------------|
| 0 | `Ok` | No error |
| 1 | `InvalidUtf8` | Input was not valid UTF-8 |
| 2 | `Parse` | HTML parsing or sanitization failed |
| 3 | `Visitor` | A visitor callback returned an error |
| 4 | `Memory` | Memory allocation failure |
| 5 | `Internal` | Internal error (I/O, panic, or other) |

## Thread Safety

All conversion functions are thread-safe. Error state is stored in thread-local storage, so concurrent calls from different threads do not interfere with each other.

Visitor handles are NOT thread-safe -- each thread must create its own visitor via `html_to_markdown_visitor_create()`.

## Version Constants

The generated header includes compile-time version constants:

```c
#define HTML_TO_MARKDOWN_VERSION_MAJOR 2
#define HTML_TO_MARKDOWN_VERSION_MINOR 25
#define HTML_TO_MARKDOWN_VERSION_PATCH 2
#define HTML_TO_MARKDOWN_VERSION "2.25.2"
```

Runtime version: `html_to_markdown_version()` returns a static string.

## Building from C

### Static Linking

```bash
cc -o myapp myapp.c -I/usr/local/include -L/usr/local/lib -lhtml_to_markdown_ffi -lm -lpthread -ldl
```

### Dynamic Linking

```bash
cc -o myapp myapp.c $(pkg-config --cflags --libs html-to-markdown)
```

### CMake Integration

```cmake
cmake_minimum_required(VERSION 3.14)
project(myapp C)

find_package(html-to-markdown-ffi REQUIRED)
add_executable(myapp main.c)
target_link_libraries(myapp PRIVATE html-to-markdown-ffi::html-to-markdown-ffi)
```

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
