---
title: Types Reference
description: Cross-language type definitions for html-to-markdown
---

# Types Reference

Type definitions shared across all language bindings. This page documents the canonical types and their representations in each supported language.

---

## ConversionOptions

The primary configuration type for HTML to Markdown conversion. See the [Configuration Reference](configuration.md) for full field documentation.

=== "Rust"

    ```rust
    pub struct ConversionOptions {
        pub heading_style: HeadingStyle,
        pub code_block_style: CodeBlockStyle,
        pub wrap: bool,
        pub wrap_width: usize,
        pub preserve_tags: Vec<String>,
        // ... see Configuration Reference for all fields
    }
    ```

=== "Python"

    ```python
    class ConversionOptions:
        heading_style: str       # "underlined", "atx", "atx_closed"
        code_block_style: str    # "indented", "backticks", "tildes"
        wrap: bool
        wrap_width: int
        preserve_tags: list[str]
        # ...
    ```

=== "TypeScript"

    ```typescript
    interface ConversionOptions {
        headingStyle?: string;
        codeBlockStyle?: string;
        wrap?: boolean;
        wrapWidth?: number;
        preserveTags?: string[];
        // ...
    }
    ```

=== "PHP"

    ```php
    class ConversionOptions {
        public readonly string $headingStyle;
        public readonly string $codeBlockStyle;
        public readonly bool $wrap;
        public readonly int $wrapWidth;
        public readonly array $preserveTags;
        // ...
    }
    ```

---

## MetadataConfig

Configuration controlling which metadata types are extracted during conversion.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `extract_document` | `bool` | `true` | Extract document-level metadata (title, description, author, OG, Twitter Card) |
| `extract_headers` | `bool` | `true` | Extract h1-h6 heading elements with hierarchy |
| `extract_links` | `bool` | `true` | Extract anchor elements with link type classification |
| `extract_images` | `bool` | `true` | Extract image elements with source and dimensions |
| `extract_structured_data` | `bool` | `true` | Extract JSON-LD, Microdata, and RDFa blocks |
| `max_structured_data_size` | `int` | `1_000_000` | Maximum bytes of structured data to collect (safety limit) |

=== "Rust"

    ```rust
    pub struct MetadataConfig {
        pub extract_document: bool,
        pub extract_headers: bool,
        pub extract_links: bool,
        pub extract_images: bool,
        pub extract_structured_data: bool,
        pub max_structured_data_size: usize,
    }
    ```

=== "Python"

    ```python
    class MetadataConfig:
        extract_document: bool = True
        extract_headers: bool = True
        extract_links: bool = True
        extract_images: bool = True
        extract_structured_data: bool = True
        max_structured_data_size: int = 1_000_000
    ```

=== "TypeScript"

    ```typescript
    interface MetadataConfig {
        extractDocument?: boolean;
        extractHeaders?: boolean;
        extractLinks?: boolean;
        extractImages?: boolean;
        extractStructuredData?: boolean;
        maxStructuredDataSize?: number;
    }
    ```

=== "Elixir"

    ```elixir
    %HtmlToMarkdown.MetadataConfig{
      extract_document: true,
      extract_headers: true,
      extract_links: true,
      extract_images: true,
      extract_structured_data: true,
      max_structured_data_size: 1_000_000
    }
    ```

---

## ExtendedMetadata

The result type containing all extracted metadata from an HTML document.

| Field | Type | Description |
|-------|------|-------------|
| `document` | `DocumentMetadata` | Document-level metadata |
| `headers` | `HeaderMetadata[]` | Heading elements with hierarchy |
| `links` | `LinkMetadata[]` | Hyperlinks with type classification |
| `images` | `ImageMetadata[]` | Image elements with source and dimensions |
| `structured_data` | `StructuredData[]` | JSON-LD, Microdata, and RDFa blocks |

---

## DocumentMetadata

Document-level metadata extracted from `<head>` and top-level elements.

| Field | Type | Description |
|-------|------|-------------|
| `title` | `string?` | From `<title>` tag |
| `description` | `string?` | From `<meta name="description">` |
| `keywords` | `string[]` | From `<meta name="keywords">`, comma-split |
| `author` | `string?` | From `<meta name="author">` |
| `canonical_url` | `string?` | From `<link rel="canonical">` |
| `base_href` | `string?` | From `<base href="">` |
| `language` | `string?` | From `lang` attribute |
| `text_direction` | `TextDirection?` | From `dir` attribute |
| `open_graph` | `Map<string, string>` | Open Graph properties (`og:*`) |
| `twitter_card` | `Map<string, string>` | Twitter Card properties (`twitter:*`) |
| `meta_tags` | `Map<string, string>` | Other `<meta>` tags |

---

## HeaderMetadata

Heading element (h1-h6) metadata.

| Field | Type | Description |
|-------|------|-------------|
| `level` | `int` | Header level: 1 through 6 |
| `text` | `string` | Normalized text content |
| `id` | `string?` | HTML `id` attribute (for anchor links) |
| `depth` | `int` | Document tree depth |
| `html_offset` | `int` | Byte offset in original HTML |

---

## LinkMetadata

Hyperlink (`<a>`) element metadata.

| Field | Type | Description |
|-------|------|-------------|
| `href` | `string` | The href URL value |
| `text` | `string` | Link text content |
| `title` | `string?` | Title attribute |
| `link_type` | `LinkType` | Classification of the link |
| `rel` | `string[]` | Rel attribute values (e.g., `"nofollow"`) |
| `attributes` | `Map<string, string>` | Additional HTML attributes |

### LinkType Enum

| Value | Description |
|-------|-------------|
| `anchor` | Same-document link (`#section`) |
| `internal` | Same-domain link (`/page`, `../other`) |
| `external` | Cross-domain link (`https://example.com`) |
| `email` | Email link (`mailto:user@example.com`) |
| `phone` | Phone link (`tel:+1234567890`) |
| `other` | Unclassifiable protocol or format |

---

## ImageMetadata

Image element metadata.

| Field | Type | Description |
|-------|------|-------------|
| `src` | `string` | Image source URL or data URI |
| `alt` | `string?` | Alt text for accessibility |
| `title` | `string?` | Title attribute |
| `dimensions` | `(int, int)?` | Width and height if available |
| `image_type` | `ImageType` | Classification of the image source |
| `attributes` | `Map<string, string>` | Additional HTML attributes |

### ImageType Enum

| Value | Description |
|-------|-------------|
| `data_uri` | Data URI embedded image (base64) |
| `inline_svg` | Inline SVG element |
| `external` | External URL (`http://`, `https://`) |
| `relative` | Relative path |

---

## StructuredData

Machine-readable structured data block.

| Field | Type | Description |
|-------|------|-------------|
| `data_type` | `StructuredDataType` | Format of the structured data |
| `raw_json` | `string` | Raw JSON string (or serialized representation) |
| `schema_type` | `string?` | Detected schema type (e.g., `"Article"`, `"Event"`) |

### StructuredDataType Enum

| Value | Description |
|-------|-------------|
| `json_ld` | JSON-LD script blocks |
| `microdata` | HTML5 Microdata attributes |
| `rdfa` | RDFa markup |

---

## TextDirection Enum

| Value | Description |
|-------|-------------|
| `ltr` | Left-to-right (Latin, Cyrillic, etc.) |
| `rtl` | Right-to-left (Arabic, Hebrew, etc.) |
| `auto` | Automatic detection |

---

## Visitor Types

### NodeContext

Context information passed to all visitor callbacks.

| Field | Type | Description |
|-------|------|-------------|
| `node_type` | `NodeType` | Coarse-grained node type classification |
| `tag_name` | `string` | Raw HTML tag name (e.g., `"div"`, `"h1"`) |
| `attributes` | `Map<string, string>` | All HTML attributes as key-value pairs |
| `depth` | `int` | Depth in the DOM tree (0 = root) |
| `index_in_parent` | `int` | Index among siblings (0-based) |
| `parent_tag` | `string?` | Parent element's tag name |
| `is_inline` | `bool` | Whether this element is inline vs block |

### VisitResult

Control flow signal returned by visitor callbacks.

| Variant | Description | Additional Data |
|---------|-------------|-----------------|
| **Continue** | Proceed with default conversion | -- |
| **Custom** | Replace with custom Markdown output | `output: string` |
| **Skip** | Skip this element and children entirely | -- |
| **PreserveHtml** | Keep original HTML verbatim | -- |
| **Error** | Stop conversion with error | `message: string` |

**Language representations:**

=== "Rust"

    ```rust
    pub enum VisitResult {
        Continue,
        Custom(String),
        Skip,
        PreserveHtml,
        Error(String),
    }
    ```

=== "Python / Ruby / PHP"

    ```python
    {"type": "continue"}
    {"type": "custom", "output": "markdown text"}
    {"type": "skip"}
    {"type": "preserve_html"}
    {"type": "error", "message": "error description"}
    ```

=== "TypeScript"

    ```typescript
    type VisitResult =
      | { type: "continue" }
      | { type: "custom"; output: string }
      | { type: "skip" }
      | { type: "preserveHtml" }
      | { type: "error"; message: string };
    ```

=== "Java / C#"

    ```java
    VisitResult.continueResult()
    VisitResult.custom("markdown text")
    VisitResult.skip()
    VisitResult.preserveHtml()
    VisitResult.error("error description")
    ```

=== "C"

    ```c
    html_to_markdown_visit_result_continue()
    html_to_markdown_visit_result_custom(malloc_string)
    html_to_markdown_visit_result_skip()
    html_to_markdown_visit_result_preserve_html()
    html_to_markdown_visit_result_error(malloc_string)
    ```

### NodeType Enum

Coarse-grained classification of HTML element types. Key values include:

| Category | Values |
|----------|--------|
| **Content** | `Text`, `Heading`, `Paragraph`, `Blockquote`, `Pre`, `Hr` |
| **Lists** | `List`, `ListItem`, `DefinitionList`, `DefinitionTerm`, `DefinitionDescription` |
| **Tables** | `Table`, `TableRow`, `TableCell`, `TableHeader`, `TableBody`, `TableHead`, `TableFoot` |
| **Inline** | `Link`, `Image`, `Strong`, `Em`, `Code`, `Strikethrough`, `Underline`, `Subscript`, `Superscript`, `Mark`, `Small`, `Br`, `Span` |
| **Semantic** | `Article`, `Section`, `Nav`, `Aside`, `Header`, `Footer`, `Main`, `Figure`, `Figcaption`, `Time`, `Details`, `Summary` |
| **Form** | `Form`, `Input`, `Select`, `Option`, `Button`, `Textarea`, `Label`, `Fieldset`, `Legend` |
| **Media** | `Audio`, `Video`, `Picture`, `Source`, `Iframe`, `Svg`, `Canvas` |
| **Document** | `Html`, `Head`, `Body`, `Title`, `Meta`, `LinkTag`, `Style`, `Script`, `Base` |
| **Other** | `Element`, `Div`, `Custom` |

---

## Conversion Error Types

### ConversionError (Rust)

```rust
pub enum ConversionError {
    InvalidInput(String),   // Binary or malformed input
    ParseError(String),     // HTML parsing failure
    ConfigError(String),    // Invalid configuration
    Panic(String),          // Internal panic (wrapped)
    Other(String),          // Other errors
}
```

### Error Codes (C FFI)

| Code | Name | Description |
|------|------|-------------|
| `0` | `Ok` | No error |
| `1` | `InvalidUtf8` | Input was not valid UTF-8 |
| `2` | `Parse` | HTML parsing failed |
| `3` | `Visitor` | Visitor callback returned error |
| `4` | `Memory` | Memory allocation failure |
| `5` | `Internal` | Internal error |

---

## See Also

- [Configuration Reference](configuration.md) -- full `ConversionOptions` field documentation
- [Metadata Extraction Guide](../guides/metadata.md) -- usage patterns for metadata
- [Visitor Pattern Guide](../guides/visitor.md) -- visitor callback patterns
