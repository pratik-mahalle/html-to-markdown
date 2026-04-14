# PHP Test App for html-to-markdown

Comprehensive test suite validating the published `kreuzberg-dev/html-to-markdown` PHP package from Packagist.

This test app verifies:

- **Package installation** from Packagist registry (not local path dependency)
- **Core conversion functions** - basic HTML to Markdown conversion
- **Metadata extraction** - document properties, headers, links, images
- **Inline image extraction** - embedding images during conversion
- **Visitor pattern** - extensible conversion interface
- **Type safety** - PHPStan level 9 static analysis
- **Error handling** - graceful error handling and edge cases
- **Fixture-based tests** - Parametrized tests with comprehensive fixtures

## Requirements

- PHP 8.2+
- Composer
- html_to_markdown native extension (installed via PIE/binary download)

## Setup

```bash
# Install dependencies (includes phpunit and phpstan)
composer install

# The test app will download and test the published package from Packagist
# (not a local path dependency)
```

## Run Tests

```bash
# Run all tests
composer test

# Run with verbose output
vendor/bin/phpunit --verbose

# Run specific test class
vendor/bin/phpunit SmokeTest.php
vendor/bin/phpunit ComprehensiveTest.php
```

## Static Analysis

```bash
# Type check with PHPStan level 9
composer lint

# Or directly
vendor/bin/phpstan analyse SmokeTest.php ComprehensiveTest.php
```

## Test Coverage

### SmokeTest.php

- Package loads (extension_loaded check)
- Basic conversion with function interface
- Basic conversion with extension interface
- Heading, paragraph, list, link, code, blockquote conversion
- Empty input handling

### ComprehensiveTest.php

- **Fixture-based tests**: Parametrized tests from JSON fixtures
  - basic-html.json (10+ basic element tests)
  - complex-html.json (5+ complex structure tests)
  - edge-cases.json (5+ edge case tests)
  - metadata-extraction.json (5+ metadata tests)
  - real-world.json (5+ real-world HTML samples)

- **Metadata extraction tests**:
  - Document metadata (title, description, language, author)
  - Header extraction with levels and ids
  - Link extraction (external, internal, email)
  - Image extraction with alt text
  - Open Graph and Twitter Card tags
  - Selective metadata extraction configuration

- **Conversion features**:
  - ConversionOptions class usage
  - Options as array
  - Inline image conversion
  - Complex HTML with multiple elements
  - Multiple heading levels

- **Error handling**:
  - UTF-8 handling
  - Null options handling
  - Invalid input resilience

## Features Tested

### Core Conversion

- ✅ Headings (h1-h6)
- ✅ Paragraphs
- ✅ Bold/Strong
- ✅ Italic/Emphasis
- ✅ Code (inline and blocks)
- ✅ Links (external, internal, email)
- ✅ Images
- ✅ Lists (ordered and unordered)
- ✅ Nested lists
- ✅ Blockquotes
- ✅ Definitions lists
- ✅ Mixed formatting

### Metadata Extraction

- ✅ Document metadata (title, description, keywords, author, language)
- ✅ Header structure with levels and ids
- ✅ Link types and attributes
- ✅ Image sources and alt text
- ✅ Open Graph tags
- ✅ Twitter Card tags
- ✅ Structured data (JSON-LD, Microdata)

### Configuration

- ✅ ConversionOptions class
- ✅ Array-based options
- ✅ Selective metadata extraction
- ✅ Heading style configuration
- ✅ List indentation configuration

### Type Safety

- ✅ PHPStan level 9 validation
- ✅ Type stubs for native functions
- ✅ Return type verification
- ✅ Parameter type checking

## Gaps in Test Coverage

### Current Coverage

- Basic HTML conversion: 10+ test cases from fixtures
- Metadata extraction: 5+ test cases
- Error handling: Basic UTF-8, null options
- Type safety: PHPStan level 9

### Known Gaps

- **HTML5 semantic elements**: Further testing of `<article>`, `<aside>`, `<section>`, `<nav>`
- **Complex tables**: Table to Markdown conversion
- **Horizontal rules**: `<hr>` conversion
- **Strikethrough**: `<del>` and `<strike>` elements
- **Inline HTML preservation**: `<span>` and `<div>` handling
- **Script/Style filtering**: Ensuring `<script>` and `<style>` are properly removed
- **Performance tests**: Benchmarking large document conversion
- **Memory tests**: Memory usage profiling
- **Visitor pattern direct testing**: Custom visitor implementation examples
- **Stream processing**: Handling very large HTML documents
- **Concurrency**: Thread-safety validation (if applicable)

## Contributing

When adding new tests:

1. Use parametrized fixtures for HTML conversion tests
2. Add test cases to appropriate JSON fixture file
3. Maintain PHPStan level 9 compliance
4. Document expected behavior in test methods
5. Update this README with new features tested
