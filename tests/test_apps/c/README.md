# html-to-markdown C FFI Test App

Comprehensive test suite for the html-to-markdown C FFI API (html-to-markdown-ffi).

## Quick Start

```bash
# Build the FFI library first
cargo build --release -p html-to-markdown-ffi

# Build and run the test suite
make test
```

## Prerequisites

- C11-compatible compiler (gcc or clang)
- html-to-markdown FFI library compiled: `cargo build --release -p html-to-markdown-ffi`

## Test Coverage

The test suite (`main.c`) validates 7 sections of the C FFI API:

1. **Library Info** - Version string, error state after success
2. **Error Code Functions** - All 6 error codes (ok, invalid_utf8, parse, visitor, memory, internal), unknown codes
3. **Basic Conversion** - Headings, paragraphs, bold, italic, links, empty input, nested HTML, Unicode
4. **Error Handling** - NULL input, error state propagation, error state clearing, `free_string(NULL)`
5. **Visitor API** - Result constructors, visitor create/free, convert with visitor, bytes variant
6. **Profiling API** - Start/stop lifecycle, NULL path, platform availability
7. **Memory Safety** - 100 repeated conversions, alternating success/failure cycles

## File Structure

```text
tests/test_apps/c/
├── main.c              # Comprehensive test suite
├── Makefile            # Build instructions
└── README.md           # This file
```

## Build Options

```bash
# Debug build
make BUILD_MODE=debug

# Use specific compiler
make CC=clang

# Custom repo root
make HTM_ROOT=/path/to/html-to-markdown
```

## Expected Output

```text
================================================================================
HTML-TO-MARKDOWN C FFI COMPREHENSIVE TEST SUITE
================================================================================
Library version: 2.26.2

[SECTION 1] Library Info
--------------------------------------------------------------------------------
  PASS  html_to_markdown_version() returns "2.26.2"
  ...

================================================================================
TEST SUMMARY
================================================================================
Total Tests: 50+
  Passed:  50+
  Failed:  0
  Skipped: 0

ALL TESTS PASSED
```

Exit codes: `0` = all passed, `1` = failures detected.

## Troubleshooting

### Library not found at runtime

```bash
# macOS
export DYLD_LIBRARY_PATH=/path/to/html-to-markdown/target/release:$DYLD_LIBRARY_PATH

# Linux
export LD_LIBRARY_PATH=/path/to/html-to-markdown/target/release:$LD_LIBRARY_PATH
```

### Header note

This test app declares FFI functions directly in `main.c` rather than including
the generated `html_to_markdown.h` header. The generated header contains cbindgen
visitor callback types that use incomplete struct fields, which some C compilers
reject. The direct declarations are kept in sync with the actual API.
