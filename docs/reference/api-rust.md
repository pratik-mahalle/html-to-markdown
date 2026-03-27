---
title: Rust API Reference
description: API reference for the html-to-markdown-rs Rust crate
---

# Rust API Reference

**Crate:** [`html-to-markdown-rs`](https://crates.io/crates/html-to-markdown-rs) | **Docs.rs:** [`html_to_markdown_rs`](https://docs.rs/html-to-markdown-rs) | **Version:** 2.28.1 | **MSRV:** 1.85

---

## Installation

```toml
[dependencies]
html-to-markdown-rs = "2.28.1"
```

To enable optional features:

```toml
[dependencies]
html-to-markdown-rs = { version = "2.28.1", features = ["metadata", "visitor"] }
```

---

## Functions

### `convert`

Convert HTML to Markdown.

```rust
pub fn convert(html: &str, options: Option<ConversionOptions>) -> Result<String>
```

**Arguments:**

| Parameter | Type | Description |
|-----------|------|-------------|
| `html` | `&str` | The HTML string to convert |
| `options` | `Option<ConversionOptions>` | Optional conversion options. Defaults to `ConversionOptions::default()` if `None`. |

**Returns:** `Result<String>` -- the converted Markdown string, or a `ConversionError`.

**Example:**

```rust
use html_to_markdown_rs::{convert, ConversionOptions};

let html = "<h1>Hello World</h1><p>This is a paragraph.</p>";
let markdown = convert(html, None).unwrap();
assert!(markdown.contains("# Hello World"));

// With options
let options = ConversionOptions {
    heading_style: html_to_markdown_rs::HeadingStyle::Atx,
    wrap: true,
    wrap_width: 80,
    ..Default::default()
};
let markdown = convert(html, Some(options)).unwrap();
```

---

### `convert_with_metadata`

Convert HTML to Markdown with comprehensive metadata extraction. Requires the `metadata` feature.

```rust
pub fn convert_with_metadata(
    html: &str,
    options: Option<ConversionOptions>,
    metadata_cfg: MetadataConfig,
    visitor: Option<VisitorHandle>,  // only with "visitor" feature
) -> Result<(String, HtmlMetadata)>
```

**Arguments:**

| Parameter | Type | Description |
|-----------|------|-------------|
| `html` | `&str` | The HTML string to convert |
| `options` | `Option<ConversionOptions>` | Optional conversion configuration |
| `metadata_cfg` | `MetadataConfig` | Configuration for metadata extraction granularity |
| `visitor` | `Option<VisitorHandle>` | Optional visitor for customizing conversion (requires `visitor` feature) |

**Returns:** `Result<(String, HtmlMetadata)>` -- a tuple of the Markdown string and extracted metadata.

**Example:**

```rust
use html_to_markdown_rs::{convert_with_metadata, MetadataConfig};

let html = r#"
  <html lang="en">
    <head><title>My Article</title></head>
    <body>
      <h1 id="intro">Introduction</h1>
      <p>Welcome to <a href="https://example.com">our site</a></p>
    </body>
  </html>
"#;

let (markdown, metadata) = convert_with_metadata(
    html, None, MetadataConfig::default(), None
).unwrap();

assert_eq!(metadata.document.title, Some("My Article".to_string()));
assert_eq!(metadata.document.language, Some("en".to_string()));
assert_eq!(metadata.headers[0].text, "Introduction");
assert_eq!(metadata.links.len(), 1);
```

---

### `convert_with_visitor`

Convert HTML to Markdown with a custom visitor callback. Requires the `visitor` feature.

```rust
pub fn convert_with_visitor(
    html: &str,
    options: Option<ConversionOptions>,
    visitor: Option<VisitorHandle>,
) -> Result<String>
```

**Arguments:**

| Parameter | Type | Description |
|-----------|------|-------------|
| `html` | `&str` | The HTML input to convert |
| `options` | `Option<ConversionOptions>` | Optional conversion options |
| `visitor` | `Option<VisitorHandle>` | Visitor handle wrapping an `HtmlVisitor` implementation |

**Returns:** `Result<String>` -- the converted Markdown string.

**Example:**

```rust
use html_to_markdown_rs::convert_with_visitor;
use html_to_markdown_rs::visitor::{HtmlVisitor, NodeContext, VisitResult};
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
struct SkipImages;

impl HtmlVisitor for SkipImages {
    fn visit_image(
        &mut self,
        _ctx: &NodeContext,
        _src: &str,
        _alt: &str,
        _title: Option<&str>,
    ) -> VisitResult {
        VisitResult::Skip
    }
}

let html = "<h1>Title</h1><img src=\"photo.jpg\" alt=\"Photo\">";
let visitor = Rc::new(RefCell::new(SkipImages));
let markdown = convert_with_visitor(html, None, Some(visitor)).unwrap();
```

---

### `convert_with_async_visitor`

Async variant of `convert_with_visitor`. Requires the `async-visitor` feature.

```rust
pub async fn convert_with_async_visitor(
    html: &str,
    options: Option<ConversionOptions>,
    visitor: Option<AsyncVisitorHandle>,
) -> Result<String>
```

---

### `convert_with_inline_images`

Convert HTML while extracting inline image assets. Requires the `inline-images` feature.

```rust
pub fn convert_with_inline_images(
    html: &str,
    options: Option<ConversionOptions>,
    image_cfg: InlineImageConfig,
    visitor: Option<VisitorHandle>,
) -> Result<HtmlExtraction>
```

---

### `convert_with_tables`

Convert HTML to Markdown with structured table extraction. Requires the `visitor` feature.

```rust
pub fn convert_with_tables(
    html: &str,
    options: Option<ConversionOptions>,
    metadata_cfg: Option<MetadataConfig>,  // only with "metadata" feature
) -> Result<ConversionWithTables>
```

**Arguments:**

| Parameter | Type | Description |
|-----------|------|-------------|
| `html` | `&str` | The HTML string to convert |
| `options` | `Option<ConversionOptions>` | Optional conversion configuration |
| `metadata_cfg` | `Option<MetadataConfig>` | Optional metadata extraction configuration (requires `metadata` feature) |

**Returns:** `Result<ConversionWithTables>` -- a struct containing the converted content, extracted tables, and optional metadata.

#### `ConversionWithTables`

| Field | Type | Description |
|-------|------|-------------|
| `content` | `String` | Converted markdown/djot/plain text content |
| `metadata` | `Option<HtmlMetadata>` | Extended metadata, if metadata extraction was requested (requires `metadata` feature) |
| `tables` | `Vec<TableData>` | All tables found in the HTML, in document order |

#### `TableData`

| Field | Type | Description |
|-------|------|-------------|
| `cells` | `Vec<Vec<String>>` | Table cells organized as rows x columns, with contents converted to the target output format |
| `markdown` | `String` | Complete rendered table in the target output format |
| `is_header_row` | `Vec<bool>` | Per-row flag indicating whether the row was inside `<thead>` |

**Example:**

```rust
use html_to_markdown_rs::convert_with_tables;

let html = r#"<table>
    <tr><th>Name</th><th>Age</th></tr>
    <tr><td>Alice</td><td>30</td></tr>
</table>"#;

let result = convert_with_tables(html, None, None).unwrap();

assert_eq!(result.tables.len(), 1);
assert_eq!(result.tables[0].cells[0], vec!["Name", "Age"]);
assert_eq!(result.tables[0].cells[1], vec!["Alice", "30"]);
assert!(result.tables[0].is_header_row[0]);  // first row is a header
assert!(!result.tables[0].is_header_row[1]); // second row is data
```

---

## Structs

### `ConversionOptions`

Main configuration struct for HTML to Markdown conversion. See the [Configuration Reference](configuration.md) for full details.

```rust
pub struct ConversionOptions {
    pub heading_style: HeadingStyle,          // default: Atx
    pub list_indent_type: ListIndentType,     // default: Spaces
    pub list_indent_width: usize,             // default: 2
    pub bullets: String,                      // default: "-"
    pub strong_em_symbol: char,               // default: '*'
    pub escape_asterisks: bool,               // default: false
    pub escape_underscores: bool,             // default: false
    pub escape_misc: bool,                    // default: false
    pub escape_ascii: bool,                   // default: false
    pub code_language: String,                // default: ""
    pub autolinks: bool,                      // default: true
    pub default_title: bool,                  // default: false
    pub br_in_tables: bool,                   // default: false
    pub hocr_spatial_tables: bool,            // default: true
    pub highlight_style: HighlightStyle,      // default: DoubleEqual
    pub extract_metadata: bool,               // default: true
    pub whitespace_mode: WhitespaceMode,      // default: Normalized
    pub strip_newlines: bool,                 // default: false
    pub wrap: bool,                           // default: false
    pub wrap_width: usize,                    // default: 80
    pub convert_as_inline: bool,              // default: false
    pub sub_symbol: String,                   // default: ""
    pub sup_symbol: String,                   // default: ""
    pub newline_style: NewlineStyle,          // default: Spaces
    pub code_block_style: CodeBlockStyle,     // default: Indented
    pub keep_inline_images_in: Vec<String>,   // default: []
    pub preprocessing: PreprocessingOptions,  // default: PreprocessingOptions::default()
    pub encoding: String,                     // default: "utf-8"
    pub debug: bool,                          // default: false
    pub strip_tags: Vec<String>,              // default: []
    pub preserve_tags: Vec<String>,           // default: []
    pub skip_images: bool,                    // default: false
    pub output_format: OutputFormat,          // default: Markdown
}
```

### `MetadataConfig`

Configuration for metadata extraction granularity. See the [Types Reference](types.md) for full details.

```rust
pub struct MetadataConfig {
    pub extract_document: bool,           // default: true
    pub extract_headers: bool,            // default: true
    pub extract_links: bool,              // default: true
    pub extract_images: bool,             // default: true
    pub extract_structured_data: bool,    // default: true
    pub max_structured_data_size: usize,  // default: 1_000_000 (1 MB)
}
```

### `HtmlMetadata`

Comprehensive metadata extraction result. See the [Types Reference](types.md) for field details.

```rust
pub struct HtmlMetadata {
    pub document: DocumentMetadata,
    pub headers: Vec<HeaderMetadata>,
    pub links: Vec<LinkMetadata>,
    pub images: Vec<ImageMetadata>,
    pub structured_data: Vec<StructuredData>,
}
```

---

## Enums

### `HeadingStyle`

| Variant | Description |
|---------|-------------|
| `Underlined` | Setext style (`===` for h1, `---` for h2) |
| `Atx` | ATX style (`#` for h1, `##` for h2, etc.) -- **default** |
| `AtxClosed` | ATX closed style (`# title #`) |

### `CodeBlockStyle`

| Variant | Description |
|---------|-------------|
| `Indented` | 4-space indented code blocks -- **default** |
| `Backticks` | Fenced with backticks (` ``` `) |
| `Tildes` | Fenced with tildes (`~~~`) |

### `OutputFormat`

| Variant | Description |
|---------|-------------|
| `Markdown` | Standard CommonMark Markdown -- **default** |
| `Djot` | Djot lightweight markup |
| `Plain` | Plain text (no markup, visible text only) |

### `ConversionError`

```rust
pub enum ConversionError {
    InvalidInput(String),
    ParseError(String),
    ConfigError(String),
    Panic(String),
    Other(String),
}
```

---

## Visitor Trait

Requires the `visitor` feature. Implement `HtmlVisitor` to customize conversion behavior.

```rust
pub trait HtmlVisitor: std::fmt::Debug {
    fn visit_element_start(&mut self, ctx: &NodeContext) -> VisitResult;
    fn visit_element_end(&mut self, ctx: &NodeContext, output: &str) -> VisitResult;
    fn visit_text(&mut self, ctx: &NodeContext, text: &str) -> VisitResult;
    fn visit_link(&mut self, ctx: &NodeContext, href: &str, text: &str, title: Option<&str>) -> VisitResult;
    fn visit_image(&mut self, ctx: &NodeContext, src: &str, alt: &str, title: Option<&str>) -> VisitResult;
    fn visit_heading(&mut self, ctx: &NodeContext, level: u32, text: &str, id: Option<&str>) -> VisitResult;
    fn visit_code_block(&mut self, ctx: &NodeContext, lang: Option<&str>, code: &str) -> VisitResult;
    fn visit_code_inline(&mut self, ctx: &NodeContext, code: &str) -> VisitResult;
    fn visit_list_item(&mut self, ctx: &NodeContext, ordered: bool, marker: &str, text: &str) -> VisitResult;
    fn visit_table_row(&mut self, ctx: &NodeContext, cells: &[String], is_header: bool) -> VisitResult;
    fn visit_blockquote(&mut self, ctx: &NodeContext, content: &str, depth: usize) -> VisitResult;
    fn visit_strong(&mut self, ctx: &NodeContext, text: &str) -> VisitResult;
    fn visit_emphasis(&mut self, ctx: &NodeContext, text: &str) -> VisitResult;
    fn visit_strikethrough(&mut self, ctx: &NodeContext, text: &str) -> VisitResult;
    // ... and more (all methods have default Continue implementations)
}
```

### `VisitResult`

```rust
pub enum VisitResult {
    Continue,           // Proceed with default conversion
    Custom(String),     // Replace with custom markdown output
    Skip,               // Skip this element entirely
    PreserveHtml,       // Keep original HTML verbatim
    Error(String),      // Stop conversion with error
}
```

### `NodeContext`

```rust
pub struct NodeContext {
    pub node_type: NodeType,
    pub tag_name: String,
    pub attributes: BTreeMap<String, String>,
    pub depth: usize,
    pub index_in_parent: usize,
    pub parent_tag: Option<String>,
    pub is_inline: bool,
}
```

---

## Feature Flags

| Feature | Description | Default |
|---------|-------------|---------|
| `metadata` | Metadata extraction (`convert_with_metadata`, `MetadataConfig`, `HtmlMetadata`) | **yes** |
| `visitor` | Synchronous visitor pattern (`HtmlVisitor` trait, `convert_with_visitor`) | no |
| `async-visitor` | Async visitor pattern (implies `visitor`) | no |
| `inline-images` | Inline image extraction (`convert_with_inline_images`) | no |
| `serde` | Serde serialization/deserialization for options and types | no |

---

## Error Handling

All conversion functions return `Result<T, ConversionError>`. Use pattern matching or the `?` operator:

```rust
use html_to_markdown_rs::{convert, ConversionError};

match convert("<p>test</p>", None) {
    Ok(markdown) => println!("{}", markdown),
    Err(ConversionError::InvalidInput(msg)) => eprintln!("Bad input: {}", msg),
    Err(e) => eprintln!("Conversion failed: {}", e),
}
```

---

## See Also

- [Configuration Reference](configuration.md) -- full `ConversionOptions` field documentation
- [Types Reference](types.md) -- cross-language type definitions
- [Visitor Pattern Guide](../guides/visitor.md) -- usage patterns and examples
- [Metadata Extraction Guide](../guides/metadata.md) -- metadata extraction workflows
