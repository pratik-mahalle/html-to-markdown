# Ruby Test App for html-to-markdown

Tests the published html-to-markdown gem from RubyGems (v3).

This test app validates that the html-to-markdown gem is properly installed from RubyGems (not a local path dependency) and that the v3 `convert()` API works correctly.

## Features Tested

### Basic HTML Conversion

- Paragraphs, headings (h1-h6)
- Text styling (bold, italic, strikethrough)
- Lists (ordered, unordered, nested)
- Links and blockquotes
- Code blocks and inline code
- Comprehensive fixture-based tests

### Conversion Options

- Heading styles (atx, atx_closed, underlined)
- List indentation customization
- Code block formatting (fenced, indented)
- Text wrapping and column width

### Error Handling

- Empty HTML input
- Malformed HTML recovery
- Large document processing (100KB+)
- Special character escaping (XSS prevention)
- Unicode and emoji support

## Setup

Requires Ruby 3.2+ (see .ruby-version)

```bash
bundle install
```

## Run Tests

```bash
# Smoke tests (quick validation)
bundle exec rspec smoke_test.rb

# Comprehensive tests (full feature coverage)
bundle exec rspec comprehensive_test.rb

# All tests
bundle exec rspec

# Run with verbose output
bundle exec rspec -v

# Run specific test
bundle exec rspec smoke_test.rb -e "can load the gem"
```

## v3 API

The v3 API has a single function: `HtmlToMarkdown.convert(html, options)`. All previous `convert_with_*` methods have been removed.
