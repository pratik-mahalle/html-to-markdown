# Java Test App for html-to-markdown

Integration tests for the published `html-to-markdown` Java package from Maven Central.

This test app validates that the package is correctly published and accessible via Maven Central, with comprehensive test coverage for core conversion, error handling, and type safety. The v3 API uses a single `convert()` function.

## Prerequisites

- Java 25+ (with preview features enabled)
- Maven 3.8+
- Rust toolchain (1.85+) - for FFI library building
- cargo 1.85+ - Rust package manager

## Setup

### Option 1: Using Published Maven Central Package (Recommended for Production)

Once the `html-to-markdown` package is published to Maven Central:

```bash
mvn clean install
```

This will download the package from Maven Central.

### Option 2: Development Build from Local Workspace

To test against the current workspace build:

```bash
# Step 1: Build and install the Java package locally
cd ../../packages/java
mvn clean install -DskipTests -Dskip.rust.ffi=false

# Step 2: Build the test app
cd ../../tests/test_apps/java
mvn clean install
```

The first command will:

1. Build the Rust FFI library via cargo
2. Create native bindings for JNI/FFI
3. Package the library for local Maven repository
4. Install it in your local Maven repository (~/.m2/repository)

The second command will then use that local package to build the test app.

## Run Tests

### All Tests

```bash
mvn test
```

### Individual Test Suites

```bash
# Smoke tests - basic functionality validation
mvn test -Dtest=SmokeTest

# Comprehensive tests - fixture-based validation and feature coverage
mvn test -Dtest=ComprehensiveTest

# Error handling tests - edge cases and error conditions
mvn test -Dtest=ErrorHandlingTest
```

### Run Specific Test Methods

```bash
mvn test -Dtest=SmokeTest#testBasicConversion
```

## Test Coverage

### SmokeTest (14 tests)

- Package loading and class availability
- Basic HTML to Markdown conversion
- Heading, list, link, code, and blockquote conversion
- Library version retrieval
- Null input handling
- Combined element conversion

### ComprehensiveTest (13 tests)

- Fixture-based test validation (basic-html.json)
- Complex HTML formatting with mixed elements
- Metadata extraction with document metadata
- Custom visitor implementation
- Batch conversion operations
- ConversionOptions type safety
- FFI functionality verification
- Table conversion
- Code block conversion
- HTML attribute preservation

### ErrorHandlingTest (13 tests)

- Null input validation
- Empty and empty element handling
- Malformed HTML resilience
- HTML entity and special character handling
- Unicode character support
- Large input processing
- Deep HTML nesting
- Mixed content handling
- HTML comment handling
- Script and style tag handling
- Data URI security
- Visitor error cases

### TypeSafetyTest (13 tests)

- ConversionOptions type validation
- Builder pattern type safety
- OutputFormat enum type safety
- Conversion return type validation
- MetadataExtraction type validation
- Visitor interface type safety
- VisitResult type hierarchy
- ConversionException exception type validation
- Generic method handling
- Heading style parameter validation
- NodeContext type safety
- Version string type safety
- Metadata component type validation

### MetadataExtractionTest (18 tests)

- Basic document metadata extraction
- Markdown content extraction with metadata
- Multiple header extraction
- Link extraction
- Image extraction
- Complex document extraction with mixed content
- Nested HTML structure extraction
- Special content types (blockquotes, code, tables)
- Empty and minimal document extraction
- Meta tags extraction (title, description, keywords)
- Open Graph metadata extraction
- Twitter Card metadata extraction
- Large document extraction
- Null input validation

### VisitorFunctionalityTest (14 tests)

- Basic visitor implementation
- Element skipping functionality
- Conditional element handling
- Multiple callback implementations
- Empty visitor with default behavior
- Complex HTML structure processing
- Heading level discrimination
- Image filtering
- Code block handling
- Null input validation
- Mixed content processing
- Multiple conversions with same visitor instance

## Total Test Count: 85 tests

## Package Information

- **GroupId**: `dev.kreuzberg`
- **ArtifactId**: `html-to-markdown`
- **Version**: `2.24.1`
- **Repository**: Maven Central

## Dependencies

- **junit-jupiter**: 5.10.2 (Testing framework)
- **jackson-databind**: 2.17.0 (JSON fixture parsing)
- **html-to-markdown**: 2.24.1 (Native Java FFI bindings)

## Features Tested

### Core Conversion

- Basic HTML elements (paragraphs, headings, lists, links, code, blockquotes)
- Text formatting (bold, italic, strikethrough)
- Nested structures and complex layouts

### Error Handling

- Null input validation with proper exceptions
- Malformed HTML resilience
- Unicode and special character support
- Large input processing
- Deep nesting handling

### Type Safety

- Strong Java typing with generics
- Builder pattern implementation
- Enum type validation
- Exception hierarchy

### Metadata Extraction

- Document title, description, author extraction
- Header hierarchy extraction
- Link and image metadata
- Open Graph and Twitter Card support

### Advanced Features

- Custom visitor pattern for element interception
- Element skipping and filtering
- Conditional processing based on attributes
- Multiple conversion formats

## Performance Notes

- All tests use the published Maven Central package (not local path dependencies)
- Tests validate JNI/FFI functionality through the Panama FFI API
- Tests are independent and can run in parallel
- No build steps for native code (uses pre-built binaries in Maven package)

## Troubleshooting

### "Failed to find html-to-markdown"

Ensure Maven Central is accessible and that your Maven repositories are configured correctly.

### "Cannot enable preview features"

Make sure Java 25+ is installed and the maven-compiler-plugin is configured with `--enable-preview`.

### "UnsatisfiedLinkError"

This test app does not build the FFI library - it uses the published package which includes pre-built binaries. Ensure the html-to-markdown Maven package is properly downloaded.
