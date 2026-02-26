---
title: C FFI API Reference
description: API reference for the html_to_markdown C FFI header
---

# C FFI API Reference <span class="version-badge new">v2.26.0</span>

**Header:** `html_to_markdown.h` | **Library:** `libhtml_to_markdown_ffi` | **Version:** 2.26.0

The C FFI provides the foundation that Go, Java, C#, and other language bindings use. You can also call it directly from C or C++ programs.

---

## Installation

### Header

Include the generated header from the FFI crate:

```c
#include "html_to_markdown.h"
```

### Linking

Link against the shared library:

```bash
# Linux
gcc -o myapp myapp.c -lhtml_to_markdown_ffi -L/path/to/lib

# macOS
clang -o myapp myapp.c -lhtml_to_markdown_ffi -L/path/to/lib

# Windows (MSVC)
cl myapp.c /link html_to_markdown_ffi.lib
```

### Building from Source

```bash
cargo build --release -p html-to-markdown-ffi
# Output: target/release/libhtml_to_markdown_ffi.{so,dylib,dll}
# Header: crates/html-to-markdown-ffi/html_to_markdown.h
```

---

## Version Constants

```c
#define HTML_TO_MARKDOWN_VERSION_MAJOR 2
#define HTML_TO_MARKDOWN_VERSION_MINOR 26
#define HTML_TO_MARKDOWN_VERSION_PATCH 0
#define HTML_TO_MARKDOWN_VERSION "2.26.0"
```

---

## Core Functions

### `html_to_markdown_convert`

Convert HTML to Markdown using default options.

```c
HTM_EXPORT char *html_to_markdown_convert(const char *html);
```

**Parameters:**

| Parameter | Type | Description |
|-----------|------|-------------|
| `html` | `const char*` | Null-terminated UTF-8 HTML string |

**Returns:** `char*` -- malloc'd null-terminated Markdown string, or `NULL` on error.

**Memory:** The returned string must be freed with `html_to_markdown_free_string`.

**Example:**

```c
const char *html = "<h1>Hello</h1><p>World</p>";
char *markdown = html_to_markdown_convert(html);
if (markdown != NULL) {
    printf("%s\n", markdown);
    html_to_markdown_free_string(markdown);
}
```

---

### `html_to_markdown_convert_with_len`

Convert HTML, also returning the output length.

```c
HTM_EXPORT char *html_to_markdown_convert_with_len(
    const char *html,
    uintptr_t *len_out
);
```

**Parameters:**

| Parameter | Type | Description |
|-----------|------|-------------|
| `html` | `const char*` | Null-terminated UTF-8 HTML string |
| `len_out` | `uintptr_t*` | Pointer where the output byte length is written |

---

### `html_to_markdown_convert_bytes_with_len`

Convert UTF-8 HTML bytes (not null-terminated) to Markdown.

```c
HTM_EXPORT char *html_to_markdown_convert_bytes_with_len(
    const uint8_t *html,
    uintptr_t len,
    uintptr_t *len_out
);
```

**Parameters:**

| Parameter | Type | Description |
|-----------|------|-------------|
| `html` | `const uint8_t*` | Pointer to UTF-8 HTML bytes |
| `len` | `uintptr_t` | Number of bytes |
| `len_out` | `uintptr_t*` | Pointer where the output byte length is written |

---

### `html_to_markdown_convert_with_metadata`

Convert HTML with metadata extraction.

```c
HTM_EXPORT char *html_to_markdown_convert_with_metadata(
    const char *html,
    char **metadata_json_out
);
```

**Parameters:**

| Parameter | Type | Description |
|-----------|------|-------------|
| `html` | `const char*` | Null-terminated UTF-8 HTML string |
| `metadata_json_out` | `char**` | Pointer where the metadata JSON string is written |

**Returns:** `char*` -- Markdown string, or `NULL` on error. Both the returned Markdown and the metadata JSON must be freed with `html_to_markdown_free_string`.

**Example:**

```c
const char *html = "<html><head><title>Test</title></head>"
                    "<body><h1>Hello</h1></body></html>";
char *metadata_json = NULL;
char *markdown = html_to_markdown_convert_with_metadata(html, &metadata_json);

if (markdown != NULL && metadata_json != NULL) {
    printf("Markdown:\n%s\n", markdown);
    printf("Metadata:\n%s\n", metadata_json);
    html_to_markdown_free_string(markdown);
    html_to_markdown_free_string(metadata_json);
}
```

---

### `html_to_markdown_convert_with_metadata_with_len`

Convert with metadata, also returning output lengths.

```c
HTM_EXPORT char *html_to_markdown_convert_with_metadata_with_len(
    const char *html,
    char **metadata_json_out,
    uintptr_t *markdown_len_out,
    uintptr_t *metadata_len_out
);
```

---

## Memory Management

### `html_to_markdown_free_string`

Free a string returned by any conversion function.

```c
HTM_EXPORT void html_to_markdown_free_string(char *s);
```

Passing `NULL` is safe (no-op). After calling this function, the pointer is invalid.

---

## Error Handling

### `html_to_markdown_last_error`

Get the last error message from a failed conversion.

```c
HTM_EXPORT const char *html_to_markdown_last_error(void);
```

**Returns:** Pointer to a thread-local buffer. Copy it immediately if needed. May return `NULL` if no error has occurred.

### `html_to_markdown_last_error_code`

Get the error code from the last failed operation.

```c
HTM_EXPORT uint32_t html_to_markdown_last_error_code(void);
```

### Error Codes

```c
enum HtmlToMarkdownErrorCode {
    Ok          = 0,  // No error
    InvalidUtf8 = 1,  // Input was not valid UTF-8
    Parse       = 2,  // HTML parsing or sanitization failed
    Visitor     = 3,  // A visitor callback returned an error
    Memory      = 4,  // Memory allocation failure
    Internal    = 5,  // Internal error (I/O, panic, or other)
};
```

### `html_to_markdown_error_code_name`

Get the name of an error code as a string.

```c
HTM_EXPORT const char *html_to_markdown_error_code_name(uint32_t code);
```

---

## Version

### `html_to_markdown_version`

Get the library version string.

```c
HTM_EXPORT const char *html_to_markdown_version(void);
```

**Returns:** Static string that does not need to be freed.

---

## Visitor API

The C FFI supports the visitor pattern through a callback table.

### Creating a Visitor

```c
HTM_EXPORT HtmlToMarkdownVisitor html_to_markdown_visitor_create(
    const HtmlToMarkdownVisitorCallbacks *callbacks
);
```

**Returns:** Opaque visitor handle, or `NULL` on failure.

### Freeing a Visitor

```c
HTM_EXPORT void html_to_markdown_visitor_free(HtmlToMarkdownVisitor visitor);
```

### Converting with a Visitor

```c
HTM_EXPORT char *html_to_markdown_convert_with_visitor(
    const char *html,
    HtmlToMarkdownVisitor visitor,
    uintptr_t *len_out
);
```

### Callback Table

```c
typedef struct HtmlToMarkdownVisitorCallbacks {
    void *user_data;
    // Element callbacks (set to NULL for defaults)
    Option_HtmlToMarkdownVisitTextCallback visit_text;
    Option_HtmlToMarkdownVisitElementStartCallback visit_element_start;
    Option_HtmlToMarkdownVisitElementEndCallback visit_element_end;
    Option_HtmlToMarkdownVisitLinkCallback visit_link;
    Option_HtmlToMarkdownVisitImageCallback visit_image;
    Option_HtmlToMarkdownVisitHeadingCallback visit_heading;
    Option_HtmlToMarkdownVisitCodeBlockCallback visit_code_block;
    Option_HtmlToMarkdownVisitCodeInlineCallback visit_code_inline;
    Option_HtmlToMarkdownVisitListItemCallback visit_list_item;
    Option_HtmlToMarkdownVisitTableRowCallback visit_table_row;
    Option_HtmlToMarkdownVisitBlockquoteCallback visit_blockquote;
    Option_HtmlToMarkdownVisitStrongCallback visit_strong;
    Option_HtmlToMarkdownVisitEmphasisCallback visit_emphasis;
    // ... and more (set unused callbacks to NULL)
} HtmlToMarkdownVisitorCallbacks;
```

### Visit Result

```c
typedef struct HtmlToMarkdownVisitResult {
    enum HtmlToMarkdownVisitResultType result_type;
    char *custom_output;   // only if result_type == Custom (malloc'd, freed by FFI)
    char *error_message;   // only if result_type == Error (malloc'd, freed by FFI)
} HtmlToMarkdownVisitResult;

enum HtmlToMarkdownVisitResultType {
    Continue     = 0,
    Custom       = 1,
    Skip         = 2,
    PreserveHtml = 3,
    Error        = 4,
};
```

### Visit Result Helpers

```c
HTM_EXPORT HtmlToMarkdownVisitResult html_to_markdown_visit_result_continue(void);
HTM_EXPORT HtmlToMarkdownVisitResult html_to_markdown_visit_result_custom(char *output);
HTM_EXPORT HtmlToMarkdownVisitResult html_to_markdown_visit_result_skip(void);
HTM_EXPORT HtmlToMarkdownVisitResult html_to_markdown_visit_result_preserve_html(void);
HTM_EXPORT HtmlToMarkdownVisitResult html_to_markdown_visit_result_error(char *message);
```

### Full Visitor Example

```c
#include "html_to_markdown.h"
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

HtmlToMarkdownVisitResult my_visit_image(void *user_data, /* ... */) {
    // Skip all images
    return html_to_markdown_visit_result_skip();
}

int main() {
    HtmlToMarkdownVisitorCallbacks callbacks = {0};
    callbacks.user_data = NULL;
    // callbacks.visit_image = my_visit_image;  // set your callback

    HtmlToMarkdownVisitor visitor = html_to_markdown_visitor_create(&callbacks);
    if (visitor == NULL) {
        fprintf(stderr, "Failed: %s\n", html_to_markdown_last_error());
        return 1;
    }

    const char *html = "<h1>Title</h1><img src='photo.jpg'>";
    size_t out_len = 0;
    char *md = html_to_markdown_convert_with_visitor(html, visitor, &out_len);

    if (md != NULL) {
        printf("%s\n", md);
        html_to_markdown_free_string(md);
    }

    html_to_markdown_visitor_free(visitor);
    return 0;
}
```

---

## Thread Safety

- Conversion functions are thread-safe (each call uses its own state)
- Error state is thread-local (`html_to_markdown_last_error` is per-thread)
- Visitor handles are NOT thread-safe; create one per thread
- The library itself is safe to load from multiple threads

---

## Symbol Visibility

The header uses the `HTM_EXPORT` macro for symbol visibility:

- Defaults to `__attribute__((visibility("default")))` on GCC/Clang
- Uses `__declspec(dllexport/dllimport)` on Windows
- Define `HTM_STATIC` before including the header for static linking

---

## See Also

- [Go API Reference](api-go.md) -- Go bindings that wrap this C FFI
- [Java API Reference](api-java.md) -- Java bindings via Panama FFI
- [C# API Reference](api-csharp.md) -- .NET bindings via P/Invoke
- [Types Reference](types.md) -- cross-language type definitions
