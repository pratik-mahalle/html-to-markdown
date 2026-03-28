# Ruby Test App for html-to-markdown

Tests the published html-to-markdown gem from RubyGems (v2.24.1).

This test app validates that the html-to-markdown gem is properly installed from RubyGems (not a local path dependency) and that all major features work correctly across all supported language bindings.

## Features Tested

### Basic HTML Conversion

- Paragraphs, headings (h1-h6)
- Text styling (bold, italic, strikethrough)
- Lists (ordered, unordered, nested)
- Links and blockquotes
- Code blocks and inline code
- Comprehensive fixture-based tests

### Metadata Extraction (`convert_with_metadata`)

- Document-level metadata (title, description, keywords, author)
- Header hierarchy extraction
- Link extraction and validation
- Image metadata collection
- Structured data extraction (JSON-LD, Microdata, RDFa)

### Inline Images (`convert_with_inline_images`)

- Base64-encoded image extraction
- Image metadata (src, alt text, dimensions)
- Multiple inline images handling
- Data URL parsing and validation

### Visitor Pattern (`convert_with_visitor`)

- Custom element interception
- Visitor callback system validation
- Element traversal and modification
- HTML tree processing customization

### Conversion Options

- Heading styles (atx, atx_closed, underlined)
- List indentation customization
- Code block formatting (fenced, indented)
- Text wrapping and column width
- Bullet style customization
- Output format selection (markdown, djot)

### Error Handling

- Empty HTML input
- Malformed HTML recovery
- Large document processing (100KB+)
- Special character escaping (XSS prevention)
- Unicode and emoji support

### RBS Type Definitions

- Module method availability
- Class/type existence validation
- Type system integration

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

## Test Coverage

- **Smoke Tests**: 8 test cases
  - Gem loading
  - Published gem validation (not local path)
  - Basic conversion
  - Options handling
  - Empty input
  - Feature availability (metadata, inline images, visitor)

- **Comprehensive Tests**: 40+ test cases
  - Basic HTML conversion (10+ fixture-based tests)
  - Metadata extraction (5 test cases)
  - Inline images (3 test cases)
  - Visitor pattern (2 test cases)
  - Conversion options (5 test cases)
  - Error handling (5 test cases)
  - RBS type definitions (5 test cases)

## Validation Checklist

Before release, ensure:

- [ ] `bundle install` completes without errors
- [ ] All tests pass: `bundle exec rspec`
- [ ] Smoke tests confirm gem is from RubyGems (not local)
- [ ] Metadata extraction works and returns Hash with expected keys
- [ ] Inline images feature returns proper structure
- [ ] Visitor pattern callbacks are invoked
- [ ] All conversion options apply correctly
- [ ] Error cases handled gracefully
- [ ] Unicode/emoji content preserved
- [ ] RBS types available and correct

## Gemfile.lock

The Gemfile.lock is updated after `bundle install` and should be committed to ensure reproducible builds.

Current lock specifies:

- html-to-markdown (2.24.1)
- rspec (~> 3.12)
- All transitive dependencies

## Known Limitations / Gaps

Currently testing:

- Basic usage patterns ✓
- All core API methods ✓
- Common error cases ✓
- Metadata extraction ✓
- Inline images ✓
- Visitor pattern ✓

Future enhancements:

- Integration with actual RubyGems release (once published)
- Performance benchmarks
- Edge cases for visitor callbacks
- Structured data parsing validation
- Network/URL-based image testing (currently data URLs only)
