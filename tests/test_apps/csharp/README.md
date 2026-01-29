# C# Test App for html-to-markdown

Tests the published `KreuzbergDev.HtmlToMarkdown` NuGet package via P/Invoke bindings.

## Overview

This test application validates that the C# bindings for html-to-markdown work correctly when installed from NuGet. The test suite includes:

- **SmokeTest.cs** - Basic P/Invoke functionality tests
  - Package loading and type availability
  - Basic HTML to Markdown conversion
  - All major HTML element types (headings, lists, links, etc.)
  - Error handling for edge cases
  - Null and malformed input handling
  - 20+ individual test cases

- **ComprehensiveTest.cs** - Fixture-driven comprehensive tests
  - Conversion accuracy against known test cases
  - Output type safety and consistency
  - Batch processing capabilities
  - Large input handling (1000+ paragraphs)
  - Deeply nested HTML structures
  - Unicode and special character support
  - HTML entity handling
  - Mixed formatting scenarios
  - Script and comment removal
  - Whitespace normalization
  - Text encoding validation
  - 20+ test cases

## Package Information

- **Package ID**: `KreuzbergDev.HtmlToMarkdown`
- **Current Version**: 2.24.1
- **Source**: Published on NuGet.org (not local path reference)
- **Type**: C# P/Invoke bindings to Rust core via FFI

## Prerequisites

- .NET 10.0 or higher (uses `net10.0` target framework)
- NuGet access to nuget.org
- xUnit test framework (auto-installed via dotnet restore)

## Setup

```bash
# Restore NuGet packages (installs KreuzbergDev.HtmlToMarkdown 2.24.1 from nuget.org)
dotnet restore

# Optionally, verify package installation
dotnet list package
```

## Run Tests

```bash
# Run all tests
dotnet test

# Run only smoke tests (basic functionality)
dotnet test --filter FullyQualifiedName~SmokeTest

# Run only comprehensive tests (fixture-driven)
dotnet test --filter FullyQualifiedName~ComprehensiveTest

# Run with verbose output
dotnet test --verbosity detailed

# Run with detailed logging
dotnet test --logger "console;verbosity=detailed"

# Run specific test by name
dotnet test --filter "Name~TestBasicParagraphConversion"
```

## Test Coverage

### SmokeTest.cs (20+ tests)
- Package loading and type availability
- Basic paragraph, heading, and multi-level heading conversion
- Bold and italic text formatting
- Unordered and ordered lists
- Hyperlinks with proper Markdown format
- Inline code and code blocks
- Blockquotes and horizontal rules
- Line breaks
- Null input and malformed HTML error handling

### ComprehensiveTest.cs (20+ tests)
- Fixture-driven conversion accuracy (basic-html.json)
- Output type safety (returns non-null string)
- Conversion consistency (idempotence)
- Batch processing of multiple documents
- Edge case handling (empty strings, whitespace)
- Large HTML input (1000+ paragraphs)
- Deeply nested HTML structures
- Unicode character support (Chinese, accented, emoji)
- HTML entity decoding (&nbsp;, &lt;, &amp;, &quot;)
- Mixed formatting combinations
- Script and comment removal from output
- Whitespace normalization
- UTF-8 text encoding validation

## Test Fixtures

Test fixtures are located in the shared `tests/test_apps/fixtures/` directory:

- `basic-html.json` - 10 basic HTML element conversion tests
- `complex-html.json` - Complex structure tests (placeholder - 0 tests)
- `edge-cases.json` - Edge case handling (placeholder - 0 tests)
- `metadata-extraction.json` - Metadata extraction (placeholder - 0 tests)
- `real-world.json` - Real-world HTML samples (placeholder - 0 tests)

## Type Safety

The C# test app uses:
- **Strict typing**: All variables properly typed
- **xUnit assertions**: Type-safe assertion framework
- **Nullable reference types**: `#nullable enable` for safety
- **Record types**: Immutable test case definitions
- **Generics**: Type-safe collection handling

## Verifying NuGet Installation

To verify that the test app is using the published NuGet package (not local paths):

```bash
# Check project dependencies
dotnet list package

# Output should show:
#   KreuzbergDev.HtmlToMarkdown (direct dependency, version 2.24.1)
#   Microsoft.NET.Test.Sdk (transitive)
#   xunit (transitive)
#   Newtonsoft.Json (transitive)

# Inspect package location
dotnet nuget list source

# View installed package details
nuget list KreuzbergDev.HtmlToMarkdown -AllVersions
```

## Troubleshooting

### Package Not Found
If you get a "Package not found" error:
```bash
# Ensure NuGet sources are configured
dotnet nuget list source

# Restore with verbose logging
dotnet restore --verbosity detailed
```

### Test Fixture Path Issues
If fixture files are not found:
```bash
# Verify fixture files exist
ls -la ../fixtures/

# Check working directory
pwd
```

### P/Invoke Binding Issues
If P/Invoke binding fails:
- Verify the native library is installed on the system
- Check that the architecture (x64, arm64) matches
- Ensure the FFI version matches the NuGet package version

## CI/CD Integration

This test app can be used in CI pipelines:

```yaml
# Example GitHub Actions workflow
- name: Run C# Tests
  run: |
    cd tests/test_apps/csharp
    dotnet restore
    dotnet test --verbosity detailed --logger "trx;LogFileName=results.trx"
```

## Notes

- This is a **published package test**, not a development test
- It validates the P/Invoke interface to the Rust FFI library
- All test fixtures are shared across language bindings for consistency
- The test app does NOT use local path references to `packages/csharp`
- Performance tests are not included (see `task bench` for Rust benchmarks)
