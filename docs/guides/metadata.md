# Metadata Extraction Guide <span class="version-badge">v2.13.0</span>

This guide shows how to extract structured metadata from HTML documents during conversion using the `convert_with_metadata()` API.

---

## Overview

The metadata API returns both the converted Markdown and a structured metadata object in a single call. Metadata extraction happens during the same traversal pass as conversion, so there is minimal overhead.

For background on what metadata is extracted and how it works, see the [Metadata Extraction concept page](../concepts/metadata-extraction.md).

---

## Basic Usage

=== "Python"

    --8<-- "docs/snippets/python/metadata/basic_extraction.md"

=== "TypeScript"

    --8<-- "docs/snippets/typescript/metadata/basic_extraction.md"

=== "Ruby"

    --8<-- "docs/snippets/ruby/metadata/basic_extraction.md"

=== "PHP"

    --8<-- "docs/snippets/php/metadata/basic_extraction.md"

=== "C"

    --8<-- "docs/snippets/c/metadata/basic_extraction.md"

=== "Elixir"

    ```elixir
    html = """
    <html>
      <head><title>Example</title></head>
      <body>
        <h1 id="welcome">Welcome</h1>
        <a href="https://example.com">Example link</a>
      </body>
    </html>
    """

    {:ok, markdown, metadata} = HtmlToMarkdown.convert_with_metadata(html)

    metadata["document"]["title"]        # "Example"
    metadata["headers"] |> hd() |> Map.get("text") # "Welcome"
    ```

=== "R"

    ```r
    library(htmltomarkdown)

    html <- '
    <html>
      <head><title>Example</title></head>
      <body>
        <h1 id="welcome">Welcome</h1>
        <a href="https://example.com">Example link</a>
      </body>
    </html>'

    result <- convert_with_metadata(html)

    cat(result$markdown)
    result$metadata$document$title       # "Example"
    result$metadata$headers[[1]]$text    # "Welcome"
    ```

=== "Rust"

    ```rust
    use html_to_markdown_rs::{convert_with_metadata, MetadataConfig};

    let html = r#"
      <html lang="en">
        <head><title>Example</title></head>
        <body>
          <h1 id="welcome">Welcome</h1>
          <a href="https://example.com">Example link</a>
        </body>
      </html>
    "#;

    let config = MetadataConfig::default();
    let (markdown, metadata) = convert_with_metadata(html, None, config, None)?;

    println!("Title: {:?}", metadata.document.title);
    println!("Headers: {:?}", metadata.headers);
    println!("Links: {:?}", metadata.links);
    ```

---

## Configuring Metadata Extraction

Control which categories of metadata are extracted using `MetadataConfig`:

=== "Python"

    ```python
    from html_to_markdown import convert_with_metadata, MetadataConfig

    # Extract only headers and links, skip everything else
    config = MetadataConfig(
        extract_document=False,
        extract_headers=True,
        extract_links=True,
        extract_images=False,
        extract_structured_data=False,
    )

    markdown, metadata = convert_with_metadata(html, metadata_config=config)
    ```

=== "TypeScript"

    ```typescript
    import { convertWithMetadata } from '@kreuzberg/html-to-markdown';

    const result = convertWithMetadata(html, {
      metadataConfig: {
        extractDocument: false,
        extractHeaders: true,
        extractLinks: true,
        extractImages: false,
        extractStructuredData: false,
      },
    });
    ```

=== "Rust"

    ```rust
    let config = MetadataConfig {
        extract_document: false,
        extract_headers: true,
        extract_links: true,
        extract_images: false,
        extract_structured_data: false,
        max_structured_data_size: 0,
    };

    let (markdown, metadata) = convert_with_metadata(html, None, config, None)?;
    assert!(metadata.images.is_empty()); // Not extracted
    ```

!!! tip "Performance benefit"
    Disabling extraction categories you do not need skips the corresponding collection logic entirely. This is particularly beneficial when processing large documents with many links or images that you do not need to catalog.

---

## Working with Document Metadata

Document-level metadata comes from `<head>` tags and the `<html>` element attributes:

```python
markdown, metadata = convert_with_metadata(html)

doc = metadata.document  # or metadata["document"] depending on binding

# Basic fields
print(doc.title)        # From <title> tag
print(doc.description)  # From <meta name="description">
print(doc.author)       # From <meta name="author">
print(doc.language)     # From <html lang="...">
print(doc.charset)      # From <meta charset="...">

# Open Graph
print(doc.open_graph)   # Dict of og:* properties
# {"title": "...", "description": "...", "image": "...", "url": "..."}

# Twitter Card
print(doc.twitter_card) # Dict of twitter:* properties
# {"card": "summary_large_image", "site": "@handle"}
```

---

## Working with Headers

Extracted headers preserve hierarchy and IDs for table-of-contents generation:

```python
markdown, metadata = convert_with_metadata(html)

for header in metadata.headers:
    indent = "  " * (header.level - 1)
    anchor = f"#{header.id}" if header.id else ""
    print(f"{indent}H{header.level}: {header.text} {anchor}")

# Output:
# H1: Introduction #intro
#   H2: Background #background
#   H2: Methodology #methodology
#     H3: Data Collection #data-collection
```

### Building a Table of Contents

```python
def build_toc(headers):
    lines = []
    for h in headers:
        indent = "  " * (h.level - 1)
        if h.id:
            lines.append(f"{indent}- [{h.text}](#{h.id})")
        else:
            lines.append(f"{indent}- {h.text}")
    return "\n".join(lines)

toc = build_toc(metadata.headers)
```

---

## Working with Links

Links are classified by type for easy filtering:

```python
markdown, metadata = convert_with_metadata(html)

# Filter by type
external = [l for l in metadata.links if l.link_type == "external"]
internal = [l for l in metadata.links if l.link_type == "internal"]
anchors  = [l for l in metadata.links if l.link_type == "anchor"]
emails   = [l for l in metadata.links if l.link_type == "email"]

# Access link details
for link in external:
    print(f"  {link.text} -> {link.href}")
    print(f"  rel: {link.rel}")
```

### Link Types

| Type | Pattern | Example |
|------|---------|---------|
| `external` | Full URL with domain | `https://example.com/page` |
| `internal` | Relative path | `/about`, `../contact` |
| `anchor` | Fragment-only | `#section`, `#top` |
| `email` | `mailto:` scheme | `mailto:user@example.com` |
| `phone` | `tel:` scheme | `tel:+1234567890` |

---

## Working with Images

```python
markdown, metadata = convert_with_metadata(html)

for img in metadata.images:
    print(f"  Source: {img.src}")
    print(f"  Alt: {img.alt}")
    print(f"  Type: {img.image_type}")  # "external", "data_uri", "inline"
```

### Image Types

| Type | Description |
|------|-------------|
| `external` | Standard URL (`https://cdn.example.com/img.jpg`) |
| `data_uri` | Base64-encoded data URI (`data:image/png;base64,...`) |
| `inline` | Inline SVG or other embedded content |

---

## Working with Structured Data

Structured data extraction captures machine-readable data embedded in HTML:

```python
markdown, metadata = convert_with_metadata(html)

for sd in metadata.structured_data:
    print(f"  Type: {sd.data_type}")    # "json_ld", "microdata", "rdfa"
    print(f"  Schema: {sd.schema_type}")  # e.g., "Product", "Article"
    print(f"  Content: {sd.content}")   # Raw content string
```

### JSON-LD Example

For HTML containing:

```html
<script type="application/ld+json">
{
  "@context": "https://schema.org",
  "@type": "Article",
  "headline": "My Article",
  "author": {"@type": "Person", "name": "Jane Doe"}
}
</script>
```

The structured data result contains:

```python
sd = metadata.structured_data[0]
sd.data_type    # "json_ld"
sd.schema_type  # "Article"
sd.content      # The raw JSON string
```

!!! warning "Size limits"
    Structured data extraction is limited to `max_structured_data_size` bytes (default 100,000) to prevent memory exhaustion from very large JSON-LD blocks. Increase this limit if your documents contain large structured data payloads.

---

## Combining Metadata with Conversion Options

You can use both `ConversionOptions` and `MetadataConfig` together:

=== "Python"

    ```python
    from html_to_markdown import (
        ConversionOptions,
        MetadataConfig,
        convert_with_metadata,
    )

    options = ConversionOptions(
        heading_style="atx",
        wrap=True,
        wrap_width=80,
    )

    config = MetadataConfig(
        extract_headers=True,
        extract_links=True,
    )

    markdown, metadata = convert_with_metadata(
        html,
        options,
        metadata_config=config,
    )
    ```

=== "Rust"

    ```rust
    use html_to_markdown_rs::{
        convert_with_metadata, ConversionOptions, HeadingStyle, MetadataConfig,
    };

    let options = ConversionOptions {
        heading_style: HeadingStyle::Atx,
        wrap: true,
        wrap_width: 80,
        ..Default::default()
    };

    let config = MetadataConfig {
        extract_headers: true,
        extract_links: true,
        ..Default::default()
    };

    let (markdown, metadata) = convert_with_metadata(html, Some(options), config, None)?;
    ```

---

## Further Reading

- [Metadata Extraction Concepts](../concepts/metadata-extraction.md) -- architecture and design details
- [Configuration Options](configuration.md) -- conversion options reference
- [Visitor Pattern Guide](visitor.md) -- combine visitors with metadata extraction
