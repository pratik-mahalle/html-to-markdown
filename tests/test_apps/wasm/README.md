# WebAssembly Test App for html-to-markdown

Validates the published `@kreuzberg/html-to-markdown-wasm` package from npm in real-world conditions across all target environments (browser, Node.js, Deno, Cloudflare Workers).

## Purpose

This test app:
- Tests the **published npm package** (not local path dependencies)
- Verifies WASM module loads and functions correctly
- Validates HTML to Markdown conversion across all platform targets
- Ensures async operations work as expected
- Tests comprehensive error handling
- Validates bundle size constraints
- Covers all major HTML elements and edge cases

## Prerequisites

- Node.js 18+ (check `.nvmrc`)
- pnpm (recommended) or npm

## Setup

```bash
# Install dependencies (installs published WASM package from npm)
pnpm install
```

The package.json specifies version `2.24.1` of the WASM package, ensuring it tests the published npm package rather than a local build.

## Run Tests

```bash
# All tests
pnpm test

# Smoke tests only (fast, basic functionality)
pnpm test:smoke

# Comprehensive tests (fixture-driven, edge cases, bundle size)
pnpm test:comprehensive
```

## Test Coverage

### Smoke Tests (smoke.spec.ts)

**Module Loading:**
- Package import verification
- Function exports validation
- Version information checks

**Basic Conversion:**
- Simple HTML to Markdown conversion
- Heading support
- List handling (ordered/unordered)
- Link conversion
- Empty input handling

**Async Operations:**
- Async conversion support (if available)
- Sync conversion verification

**Error Handling:**
- Malformed HTML resilience
- Very long input handling (10KB+)
- Special HTML character escaping
- Null/undefined input safety
- XSS prevention (script tag escaping)

### Comprehensive Tests (comprehensive.spec.ts)

**Fixture-Based Tests:**
- Load and run fixture files from `../fixtures/`
- Basic HTML conversions from JSON fixtures

**HTML Element Coverage:**
- All heading levels (h1-h6)
- Paragraphs and text formatting
- Unordered and ordered lists
- Nested lists
- Links with URLs
- Bold/strong text
- Italic/emphasis text
- Inline code
- Code blocks
- Blockquotes
- Horizontal rules
- Images (if supported)
- Tables (if supported)

**Edge Cases & Special Scenarios:**
- Nested HTML structures
- Mixed content types
- Special HTML characters (&, <, >, etc.)
- HTML entity decoding
- Excessive whitespace normalization
- Empty elements
- Deeply nested structures (5+ levels)
- Very long content (5000+ characters)
- Unicode and special characters
- Multi-language content (English, French, German, Chinese)

**Bundle Size Validation:**
- Verifies WASM binary size is within acceptable range (1KB - 2MB)
- Logs actual bundle size in KB
- Warns if size constraints are violated

**Module Functionality:**
- Result consistency (same input = same output)
- Multiple sequential conversions
- Options parameter support

## Environment Support

This test app validates the WASM module works across all target platforms:

- **Browser** (ESM modules, modern browsers)
- **Node.js** (CommonJS/ESM, v18+)
- **Deno** (ES modules)
- **Cloudflare Workers** (edge computing)

All tests use standard Node.js testing patterns compatible with all platforms.

## Test Fixtures

Fixtures are stored in `../fixtures/`:
- `basic-html.json` - Core HTML element conversions
- `complex-html.json` - Complex nested structures
- `edge-cases.json` - Edge case handling
- `metadata-extraction.json` - Metadata extraction tests
- `real-world.json` - Real-world HTML samples

## Notes

- Tests validate the **published npm package**, ensuring release quality
- No local development builds or path dependencies are used
- vitest is configured for ESM modules (Node.js native support)
- All tests are async-safe and await WASM module initialization
- Tests are comprehensive enough to catch API breaking changes
- Bundle size validation helps track performance regressions

## Troubleshooting

If tests fail:

1. **Package not found**: Ensure version 2.24.1 has been published to npm
2. **WASM initialization error**: Check Node.js version (18+ required)
3. **File not found errors**: Ensure fixture files exist in `../fixtures/`
4. **Test timeouts**: WASM initialization may be slow on first run

## CI Integration

This test app is designed to run in CI/CD pipelines post-release to validate published packages before marking the release as stable.
