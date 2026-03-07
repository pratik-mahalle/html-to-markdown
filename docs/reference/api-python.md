---
title: Python API Reference
description: API reference for the html-to-markdown Python package
---

# Python API Reference

**Package:** [`html-to-markdown`](https://pypi.org/project/html-to-markdown/) | **Version:** 2.28.1 | **Python:** 3.10+

---

## Installation

```bash
pip install html-to-markdown
```

---

## Functions

### `convert`

Convert HTML to Markdown.

```python
def convert(
    html: str,
    options: ConversionOptions | None = None,
    visitor: object | None = None,
) -> str
```

**Arguments:**

| Parameter | Type | Description |
|-----------|------|-------------|
| `html` | `str` | The HTML string to convert |
| `options` | `ConversionOptions \| None` | Optional conversion configuration |
| `visitor` | `object \| None` | Optional visitor for custom conversion logic (requires `visitor` feature in the wheel) |

**Returns:** `str` -- the converted Markdown string.

**Raises:** `ValueError` if the HTML is invalid or conversion fails.

**Example:**

```python
from html_to_markdown import convert, ConversionOptions

html = "<h1>Hello</h1><p>World</p>"
markdown = convert(html)

# With options
options = ConversionOptions(heading_style="atx")
markdown = convert(html, options)

# With visitor
class SkipImages:
    def visit_image(self, ctx, src, alt, title):
        return {"type": "skip"}

markdown = convert(html, visitor=SkipImages())
```

---

### `convert_with_metadata`

Convert HTML to Markdown with metadata extraction. Requires the `metadata` feature.

```python
def convert_with_metadata(
    html: str,
    options: ConversionOptions | None = None,
    metadata_config: MetadataConfig | None = None,
) -> tuple[str, dict]
```

**Arguments:**

| Parameter | Type | Description |
|-----------|------|-------------|
| `html` | `str` | The HTML string to convert |
| `options` | `ConversionOptions \| None` | Optional conversion configuration |
| `metadata_config` | `MetadataConfig \| None` | Metadata extraction configuration. Defaults extract all. |

**Returns:** `tuple[str, dict]` -- a tuple of `(markdown_string, metadata_dict)`.

The metadata dict contains keys: `document`, `headers`, `links`, `images`, `structured_data`. See the [Types Reference](types.md) for details.

**Example:**

```python
from html_to_markdown import convert_with_metadata, MetadataConfig

html = """
<html lang="en">
  <head><title>My Article</title></head>
  <body>
    <h1 id="intro">Introduction</h1>
    <p>Visit <a href="https://example.com">our site</a></p>
  </body>
</html>
"""

markdown, metadata = convert_with_metadata(html)
print(metadata["document"]["title"])   # "My Article"
print(metadata["document"]["language"])  # "en"
print(len(metadata["headers"]))         # 1
print(len(metadata["links"]))           # 1

# Selective extraction
config = MetadataConfig(
    extract_headers=True,
    extract_links=False,
    extract_images=False,
)
markdown, metadata = convert_with_metadata(html, metadata_config=config)
```

---

### `convert_with_visitor`

!!! warning "Deprecated"
    Use `convert(html, visitor=my_visitor)` instead. All `convert` functions now accept an optional `visitor` parameter.

Convert HTML to Markdown with a custom visitor. Requires the `visitor` feature.

```python
def convert_with_visitor(
    html: str,
    options: ConversionOptions | None = None,
    visitor: object | None = None,
) -> str
```

---

### `convert_with_async_visitor`

Convert HTML to Markdown with an async-compatible visitor. Requires the `async-visitor` feature.

```python
def convert_with_async_visitor(
    html: str,
    options: ConversionOptions | None = None,
    visitor: object | None = None,
) -> str
```

Supports visitor methods defined as `async def`. The async event loop is managed internally.

**Example:**

```python
from html_to_markdown import convert_with_async_visitor

class AsyncVisitor:
    async def visit_text(self, ctx, text):
        result = await some_async_operation(text)
        return {"type": "continue"}

markdown = convert_with_async_visitor(html, visitor=AsyncVisitor())
```

---

### `create_options_handle`

Create a reusable, pre-compiled options handle for repeated conversions.

```python
def create_options_handle(
    options: ConversionOptions | None = None,
) -> ConversionOptionsHandle
```

### `convert_with_options_handle`

Convert using a pre-compiled options handle (faster for batch conversions).

```python
def convert_with_options_handle(
    html: str,
    handle: ConversionOptionsHandle,
) -> str
```

**Example:**

```python
from html_to_markdown import create_options_handle, convert_with_options_handle, ConversionOptions

handle = create_options_handle(ConversionOptions(heading_style="atx"))

for html in html_documents:
    markdown = convert_with_options_handle(html, handle)
```

---

## Classes

### `ConversionOptions`

Configuration class for HTML to Markdown conversion. All parameters are keyword-only with defaults.

```python
class ConversionOptions:
    def __init__(
        self,
        heading_style: str = "underlined",       # "underlined", "atx", "atx_closed"
        list_indent_type: str = "spaces",         # "spaces", "tabs"
        list_indent_width: int = 4,
        bullets: str = "*+-",
        strong_em_symbol: str = "*",
        escape_asterisks: bool = False,
        escape_underscores: bool = False,
        escape_misc: bool = False,
        escape_ascii: bool = False,
        code_language: str = "",
        autolinks: bool = True,
        default_title: bool = False,
        br_in_tables: bool = False,
        hocr_spatial_tables: bool = True,
        highlight_style: str = "double-equal",    # "double-equal", "html", "bold", "none"
        extract_metadata: bool = True,
        whitespace_mode: str = "normalized",      # "normalized", "strict"
        strip_newlines: bool = False,
        wrap: bool = False,
        wrap_width: int = 80,
        convert_as_inline: bool = False,
        sub_symbol: str = "",
        sup_symbol: str = "",
        newline_style: str = "spaces",            # "spaces", "backslash"
        code_block_style: str = "indented",       # "indented", "backticks", "tildes"
        preserve_tags: list[str] | None = None,
        strip_tags: list[str] | None = None,
        skip_images: bool = False,
        output_format: str = "markdown",          # "markdown", "djot", "plain"
        preprocessing: PreprocessingOptions | None = None,
        encoding: str = "utf-8",
        debug: bool = False,
    )
```

See the [Configuration Reference](configuration.md) for detailed descriptions of each field.

---

### `MetadataConfig`

Configuration for metadata extraction.

```python
class MetadataConfig:
    def __init__(
        self,
        extract_document: bool = True,
        extract_headers: bool = True,
        extract_links: bool = True,
        extract_images: bool = True,
        extract_structured_data: bool = True,
        max_structured_data_size: int = 1_000_000,
    )
```

---

### `PreprocessingOptions`

HTML preprocessing configuration.

```python
class PreprocessingOptions:
    def __init__(
        self,
        enabled: bool = False,
        preset: str = "default",        # "default", "aggressive"
        remove_navigation: bool = False,
        remove_forms: bool = False,
    )
```

---

## Visitor Protocol

Visitor objects are plain Python classes. Define methods matching the callbacks you need. Each method should return a dict with a `"type"` key.

### Return Types

| Type | Description |
|------|-------------|
| `{"type": "continue"}` | Continue with default conversion |
| `{"type": "skip"}` | Skip this element entirely |
| `{"type": "preserve_html"}` | Keep original HTML verbatim |
| `{"type": "custom", "output": "..."}` | Replace with custom markdown |
| `{"type": "error", "message": "..."}` | Stop conversion with error |

### Available Callbacks

```python
class MyVisitor:
    def visit_text(self, ctx, text): ...
    def visit_link(self, ctx, href, text, title): ...
    def visit_image(self, ctx, src, alt, title): ...
    def visit_heading(self, ctx, level, text, id): ...
    def visit_code_block(self, ctx, language, code): ...
    def visit_code_inline(self, ctx, code): ...
    def visit_list_item(self, ctx, ordered, marker, text): ...
    def visit_table_row(self, ctx, cells, is_header): ...
    def visit_blockquote(self, ctx, content, depth): ...
    def visit_strong(self, ctx, text): ...
    def visit_emphasis(self, ctx, text): ...
    def visit_strikethrough(self, ctx, text): ...
    def visit_element_start(self, ctx): ...
    def visit_element_end(self, ctx, output): ...
    # ... and more
```

---

## See Also

- [Configuration Reference](configuration.md) -- full options documentation
- [Types Reference](types.md) -- cross-language type definitions
- [Visitor Pattern Guide](../guides/visitor.md) -- usage patterns and examples
- [Metadata Extraction Guide](../guides/metadata.md) -- metadata extraction workflows
