---
title: Configuration Reference
description: Full ConversionOptions reference for html-to-markdown
---

# Configuration Reference

Complete reference for all conversion options available across all language bindings.

---

## Option Naming Conventions

Options use different naming conventions depending on the language:

| Language | Style | Example |
|----------|-------|---------|
| **Rust** | `snake_case` | `heading_style` |
| **Python** | `snake_case` | `heading_style` |
| **TypeScript** | `camelCase` | `headingStyle` |
| **Ruby** | `snake_case` (symbol keys) | `:heading_style` |
| **PHP** | `camelCase` (class) / `snake_case` (array) | `headingStyle` / `heading_style` |
| **Go** | N/A (default options only) | -- |
| **Java** | `camelCase` (builder) | `.headingStyle("atx")` |
| **C#** | `PascalCase` | `HeadingStyle` |
| **Elixir** | `snake_case` (atom keys) | `:heading_style` |
| **R** | `snake_case` (list keys) | `heading_style` |
| **C** | N/A (default options via FFI) | -- |
| **WASM** | `camelCase` | `headingStyle` |

---

## Heading & Structure Options

### `heading_style`

Controls how heading elements (h1-h6) are rendered in the Markdown output.

| Value | Output | Description |
|-------|--------|-------------|
| `"atx"` | `# Heading` | ATX style with hash prefixes -- **default in Rust** |
| `"underlined"` / `"setext"` | `Heading\n===` | Setext style with underlines (h1/h2 only; h3+ fall back to ATX) -- **default in Python/Ruby** |
| `"atx_closed"` / `"atxClosed"` | `# Heading #` | ATX with closing hashes |

### `code_block_style`

Controls how code blocks (`<pre><code>`) are rendered.

| Value | Output | Description |
|-------|--------|-------------|
| `"indented"` | 4-space indent | Indented code blocks -- **default** |
| `"backticks"` | ` ``` ` | Fenced with backticks (supports language hints) |
| `"tildes"` | `~~~` | Fenced with tildes (supports language hints) |

### `code_language`

Default language for fenced code blocks when no language is specified in the HTML.

**Type:** `string` | **Default:** `""` (empty)

### `highlight_style`

Controls how `<mark>` elements are rendered.

| Value | Output | Description |
|-------|--------|-------------|
| `"double_equal"` / `"doubleEqual"` | `==text==` | Pandoc-compatible highlight syntax -- **default** |
| `"html"` | `<mark>text</mark>` | Preserve as HTML |
| `"bold"` | `**text**` | Render as bold |
| `"none"` | `text` | Strip formatting, plain text |

### `output_format` <span class="version-badge new">v2.23.0</span>

Target markup format for conversion.

| Value | Description |
|-------|-------------|
| `"markdown"` | Standard CommonMark Markdown -- **default** |
| `"djot"` | Djot lightweight markup language |
| `"plain"` | Plain text (no markup, visible text only) |

---

## List Options

### `bullets`

Bullet characters for unordered lists. A string where each character is used for successive nesting levels.

**Type:** `string` | **Default:** `"-"` (Rust), `"*+-"` (Python/Ruby)

### `list_indent_type`

Controls whether list items are indented with spaces or tabs.

| Value | Description |
|-------|-------------|
| `"spaces"` | Use spaces for indentation -- **default** |
| `"tabs"` | Use tabs for indentation |

### `list_indent_width`

Number of spaces per indentation level for nested lists.

**Type:** `integer` | **Default:** `2` (Rust), `4` (Python/Ruby)

---

## Text & Escaping Options

### `strong_em_symbol`

Character used for strong (`**`) and emphasis (`*`) rendering.

**Type:** `char` | **Default:** `'*'`

### `escape_asterisks`

Escape `*` characters in text content to prevent accidental Markdown formatting.

**Type:** `boolean` | **Default:** `false`

### `escape_underscores`

Escape `_` characters in text content to prevent accidental Markdown formatting.

**Type:** `boolean` | **Default:** `false`

### `escape_misc`

Escape miscellaneous Markdown characters: `\ & < [ ] > ~ # = + | -`.

**Type:** `boolean` | **Default:** `false`

### `escape_ascii`

Escape all ASCII punctuation characters for full CommonMark spec compliance.

**Type:** `boolean` | **Default:** `false`

---

## Whitespace & Wrapping Options

### `whitespace_mode`

Controls how whitespace sequences are handled.

| Value | Description |
|-------|-------------|
| `"normalized"` | Collapse multiple whitespace to single spaces (matches browser behavior) -- **default** |
| `"strict"` | Preserve all whitespace exactly as in the HTML |

### `strip_newlines`

Strip newline characters from HTML before processing.

**Type:** `boolean` | **Default:** `false`

### `wrap`

Enable automatic text wrapping at `wrap_width`.

**Type:** `boolean` | **Default:** `false`

### `wrap_width`

Text wrapping width in characters when `wrap` is enabled.

**Type:** `integer` | **Default:** `80`

### `newline_style`

Controls how soft line breaks (from `<br>`) are rendered in Markdown.

| Value | Output | Description |
|-------|--------|-------------|
| `"spaces"` | `text  \n` | Two trailing spaces -- **default** |
| `"backslash"` | `text\\\n` | Backslash at end of line |

---

## Content Filtering Options

### `preserve_tags` <span class="version-badge">v2.5.0</span>

HTML tags to preserve as-is in the output (keep original HTML, no Markdown conversion).

**Type:** `string[]` | **Default:** `[]`

**Example:** `["table", "div", "iframe"]`

### `strip_tags`

HTML tags to strip (extract inner text content only, no Markdown conversion for the tag itself).

**Type:** `string[]` | **Default:** `[]`

### `skip_images` <span class="version-badge">v2.21.0</span>

Skip all `<img>` elements during conversion, omitting them entirely from output.

**Type:** `boolean` | **Default:** `false`

### `keep_inline_images_in`

HTML element contexts where images should remain as Markdown image links rather than being converted to alt text.

**Type:** `string[]` | **Default:** `[]`

---

## Link & URL Options

### `autolinks`

Use autolink syntax for bare URLs: `<http://example.com>`.

**Type:** `boolean` | **Default:** `true`

### `default_title`

Add a default title element to the HTML if none exists before conversion.

**Type:** `boolean` | **Default:** `false`

---

## Table Options

### `br_in_tables` <span class="version-badge">v2.22.4</span>

Use HTML `<br>` elements in tables instead of spaces for line breaks.

**Type:** `boolean` | **Default:** `false`

### `hocr_spatial_tables`

> **Deprecated since 2.30.0**: hOCR support will be removed in v3.

Enable spatial table reconstruction in hOCR documents via positioning analysis.

**Type:** `boolean` | **Default:** `true`

---

## Symbol Options

### `sub_symbol`

Custom symbol for subscript content (`<sub>`). When empty, subscript is rendered with HTML tags.

**Type:** `string` | **Default:** `""` (empty -- uses HTML)

### `sup_symbol`

Custom symbol for superscript content (`<sup>`). When empty, superscript is rendered with HTML tags.

**Type:** `string` | **Default:** `""` (empty -- uses HTML)

---

## Preprocessing Options

### `preprocessing`

HTML preprocessing configuration for cleaning up input before conversion.

```
PreprocessingOptions {
    enabled: bool,              // default: false
    preset: string,             // "default" | "aggressive"
    remove_navigation: bool,    // default: false
    remove_forms: bool,         // default: false
}
```

---

## Advanced Options

### `convert_as_inline`

Treat block-level elements as inline during conversion.

**Type:** `boolean` | **Default:** `false`

### `extract_metadata`

Enable metadata extraction during conversion.

**Type:** `boolean` | **Default:** `true`

### `encoding`

Source document encoding (informational).

**Type:** `string` | **Default:** `"utf-8"`

### `debug`

Enable debug mode with diagnostic warnings on conversion issues.

**Type:** `boolean` | **Default:** `false`

---

## Language-Specific Examples

=== "Rust"

    ```rust
    use html_to_markdown_rs::{ConversionOptions, HeadingStyle, CodeBlockStyle};

    let options = ConversionOptions {
        heading_style: HeadingStyle::Atx,
        code_block_style: CodeBlockStyle::Backticks,
        wrap: true,
        wrap_width: 80,
        preserve_tags: vec!["table".to_string()],
        ..Default::default()
    };
    ```

=== "Python"

    ```python
    from html_to_markdown import ConversionOptions

    options = ConversionOptions(
        heading_style="atx",
        code_block_style="backticks",
        wrap=True,
        wrap_width=80,
        preserve_tags=["table"],
    )
    ```

=== "TypeScript"

    ```typescript
    const options = {
      headingStyle: "atx",
      codeBlockStyle: "backticks",
      wrap: true,
      wrapWidth: 80,
      preserveTags: ["table"],
    };
    ```

=== "Ruby"

    ```ruby
    options = {
      heading_style: "atx",
      code_block_style: "backticks",
      wrap: true,
      wrap_width: 80,
      preserve_tags: ["table"],
    }
    ```

=== "PHP"

    ```php
    $options = new ConversionOptions(
        headingStyle: 'atx',
        codeBlockStyle: 'backticks',
        wrap: true,
        wrapWidth: 80,
        preserveTags: ['table'],
    );
    ```

=== "Elixir"

    ```elixir
    options = %{
      heading_style: "atx",
      code_block_style: "backticks",
      wrap: true,
      wrap_width: 80,
      preserve_tags: ["table"]
    }
    ```

=== "R"

    ```r
    options <- list(
      heading_style = "atx",
      code_block_style = "backticks",
      wrap = TRUE,
      wrap_width = 80,
      preserve_tags = c("table")
    )
    ```

---

## See Also

- [Types Reference](types.md) -- `MetadataConfig`, `HtmlMetadata`, and other types
- [Basic Conversion Guide](../guides/basic-conversion.md) -- getting started
