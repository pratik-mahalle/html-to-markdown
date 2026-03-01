# Configuration Options

html-to-markdown provides comprehensive configuration through the `ConversionOptions` struct. This guide explains every option with examples.

---

## Heading Style

Controls how HTML headings (`<h1>` through `<h6>`) are formatted.

| Value | Output | Notes |
|-------|--------|-------|
| `atx` (default) | `# Heading` | CommonMark standard |
| `atx_closed` | `# Heading #` | Closing hashes |
| `underlined` / `setext` | `Heading` + `=======` | Only works for h1 and h2; h3+ fall back to ATX |

=== "Python"

    ```python
    from html_to_markdown import ConversionOptions, convert

    options = ConversionOptions(heading_style="atx")
    markdown = convert("<h1>Title</h1>", options)
    # Output: # Title
    ```

=== "TypeScript"

    ```typescript
    import { convert } from '@kreuzberg/html-to-markdown';

    const markdown = convert('<h1>Title</h1>', { headingStyle: 'atx' });
    ```

=== "Rust"

    ```rust
    use html_to_markdown_rs::{convert, ConversionOptions, HeadingStyle};

    let options = ConversionOptions {
        heading_style: HeadingStyle::Atx,
        ..Default::default()
    };
    let markdown = convert("<h1>Title</h1>", Some(options))?;
    ```

=== "Ruby"

    ```ruby
    markdown = HtmlToMarkdown.convert("<h1>Title</h1>", heading_style: :atx)
    ```

=== "PHP"

    ```php
    $options = new ConversionOptions(headingStyle: 'Atx');
    $markdown = $converter->convert('<h1>Title</h1>', $options);
    ```

---

## Code Block Style

Controls how `<pre><code>` blocks are formatted.

| Value | Output | Notes |
|-------|--------|-------|
| `backticks` | ` ```code``` ` | Fenced with backticks (default) |
| `tildes` | `~~~code~~~` | Fenced with tildes |
| `indented` | 4-space indented | No language annotation possible |

The `code_language` option sets a default language for fenced code blocks when the HTML does not specify one:

```python
options = ConversionOptions(
    code_block_style="backticks",
    code_language="python",  # Default language when none specified
)
```

---

## List Options

### Bullet List Marker

The `bullets` option controls the character(s) used for unordered list items. Characters cycle through nesting levels:

| Value | Level 1 | Level 2 | Level 3 |
|-------|---------|---------|---------|
| `"-"` (default) | `-` | `-` | `-` |
| `"*"` | `*` | `*` | `*` |
| `"*+-"` | `*` | `+` | `-` |

### List Indent Width

The `list_indent_width` option controls the number of spaces per indentation level (default: 2):

```python
options = ConversionOptions(list_indent_width=4)
```

### List Indent Type

Choose between spaces (default) and tabs:

```python
options = ConversionOptions(list_indent_type="spaces")  # or "tabs"
```

---

## Text Formatting

### Bold/Italic Symbol

The `strong_em_symbol` option sets the character used for bold and italic text:

| Value | Bold | Italic |
|-------|------|--------|
| `"*"` (default) | `**bold**` | `*italic*` |
| `"_"` | `__bold__` | `_italic_` |

### Newline Style

Controls how `<br>` tags are rendered:

| Value | Output | Notes |
|-------|--------|-------|
| `backslash` (default) | `\` at end of line | CommonMark standard |
| `spaces` | Two trailing spaces | Legacy Markdown |

### Subscript and Superscript

Custom symbols for `<sub>` and `<sup>` elements:

```python
options = ConversionOptions(
    sub_symbol="~",   # <sub>text</sub> -> ~text~
    sup_symbol="^",   # <sup>text</sup> -> ^text^
)
```

### Highlight Style

Controls how `<mark>` elements are rendered:

| Value | Output |
|-------|--------|
| `double_equal` (default) | `==highlighted==` |
| `html` | `<mark>highlighted</mark>` |
| `bold` | `**highlighted**` |
| `none` | `highlighted` (plain text) |

---

## Escaping Options

Control which characters are escaped to prevent accidental Markdown formatting:

| Option | Characters | Default |
|--------|-----------|---------|
| `escape_asterisks` | `*` | `false` |
| `escape_underscores` | `_` | `false` |
| `escape_misc` | `\ & < ` `` ` `` ` [ > ~ # = + \| -` | `false` |
| `escape_ascii` | All ASCII punctuation | `false` |

```python
options = ConversionOptions(
    escape_asterisks=True,
    escape_underscores=True,
)
markdown = convert("Price: $10*2 = $20", options)
# Output: Price: $10\*2 = $20
```

!!! note "When to enable escaping"
    Enable escaping when your input contains literal characters that look like Markdown syntax (e.g., user-generated content with asterisks). For typical HTML conversion, the defaults work well.

---

## Tag Preservation

Keep specific HTML elements as raw HTML in the output:

```python
options = ConversionOptions(
    preserve_tags=["table", "svg", "details"],
)
```

See [Tag Preservation](../concepts/tag-preservation.md) for details.

---

## Image Handling

### Skip Images <span class="version-badge">v2.21.0</span>

Omit all `<img>` elements from output:

```python
options = ConversionOptions(skip_images=True)
markdown = convert('<p>Text <img src="photo.jpg" alt="Photo"> more text</p>', options)
# Output: Text more text
```

### Keep Inline Images In

Specify which parent elements should retain images as Markdown rather than converting to alt text:

```python
options = ConversionOptions(
    keep_inline_images_in=["a", "strong"],
)
```

---

## Links

### Autolinks

When link text equals the URL, use compact autolink syntax:

```python
options = ConversionOptions(autolinks=True)
markdown = convert('<a href="https://example.com">https://example.com</a>', options)
# Output: <https://example.com>
```

### Default Title

Use the `href` as a link title when no `title` attribute exists:

```python
options = ConversionOptions(default_title=True)
```

---

## Tables

### BR in Tables <span class="version-badge">v2.22.4</span>

Preserve line breaks inside table cells using `<br>` tags:

```python
options = ConversionOptions(br_in_tables=True)
```

---

## Whitespace and Wrapping

### Whitespace Mode

| Value | Behavior |
|-------|----------|
| `normalized` (default) | Collapses multiple spaces/newlines |
| `strict` | Preserves whitespace exactly as-is |

### Strip Newlines

Remove all newlines from HTML before processing (useful for minified HTML):

```python
options = ConversionOptions(strip_newlines=True)
```

### Text Wrapping

Enable automatic text wrapping at a specified column width:

```python
options = ConversionOptions(
    wrap=True,
    wrap_width=80,  # Default is 80
)
```

!!! info "What gets wrapped"
    Wrapping applies to paragraph text, blockquote content, and list item text. Code blocks, headings, and table cells are never wrapped.

---

## Preprocessing

HTML preprocessing cleans up web content before conversion. It removes navigation, ads, forms, and other boilerplate.

```python
from html_to_markdown import ConversionOptions, PreprocessingOptions

options = ConversionOptions(
    preprocessing=PreprocessingOptions(
        enabled=True,
        preset="standard",  # "minimal", "standard", or "aggressive"
    )
)
```

| Preset | Removes |
|--------|---------|
| `minimal` | `<script>`, `<style>`, comments |
| `standard` | + navigation, sidebars, footers, ads |
| `aggressive` | + forms, social widgets, cookie banners |

Override specific behaviors:

```python
preprocessing = PreprocessingOptions(
    enabled=True,
    preset="standard",
    keep_navigation=True,   # Override: keep nav elements
    keep_forms=True,        # Override: keep form elements
)
```

---

## Strip Tags

Remove specific HTML tags while keeping their text content:

```python
options = ConversionOptions(
    strip_tags=["script", "style", "nav"],
)
```

This differs from `preserve_tags` (which keeps the HTML) and from the default behavior (which converts the element to Markdown).

---

## Output Format <span class="version-badge new">v2.23.0</span>

Convert to Markdown (default), Djot, or plain text:

```python
options = ConversionOptions(output_format="markdown")  # or "djot", "plain"
```

Plain text mode strips all markup and returns only visible text content — useful for search indexing, text extraction, or feeding content to LLMs.

---

## Block Element Handling

### Convert As Inline

Treat block-level elements as inline during conversion (no paragraph breaks):

```python
options = ConversionOptions(convert_as_inline=True)
```

This is useful when converting HTML fragments that will be inserted into an existing Markdown paragraph.

---

## Full Configuration Example

=== "Python"

    ```python
    from html_to_markdown import ConversionOptions, convert

    options = ConversionOptions(
        heading_style="atx",
        code_block_style="backticks",
        code_language="",
        bullets="-",
        list_indent_width=2,
        strong_em_symbol="*",
        newline_style="backslash",
        highlight_style="double_equal",
        autolinks=True,
        wrap=True,
        wrap_width=80,
        preserve_tags=["svg"],
        skip_images=False,
        escape_asterisks=False,
        escape_underscores=False,
    )

    markdown = convert(html, options)
    ```

=== "TypeScript"

    ```typescript
    import { convert, ConversionOptions } from '@kreuzberg/html-to-markdown';

    const options: ConversionOptions = {
      headingStyle: 'atx',
      codeBlockStyle: 'backticks',
      bullets: '-',
      listIndentWidth: 2,
      strongEmSymbol: '*',
      autolinks: true,
      wrap: true,
      wrapWidth: 80,
      preserveTags: ['svg'],
      skipImages: false,
    };

    const markdown = convert(html, options);
    ```

=== "Rust"

    ```rust
    use html_to_markdown_rs::{convert, ConversionOptions, HeadingStyle, CodeBlockStyle};

    let options = ConversionOptions {
        heading_style: HeadingStyle::Atx,
        code_block_style: CodeBlockStyle::Backticks,
        bullets: "-".to_string(),
        list_indent_width: 2,
        autolinks: true,
        wrap: true,
        wrap_width: 80,
        preserve_tags: vec!["svg".to_string()],
        ..Default::default()
    };

    let markdown = convert(html, Some(options))?;
    ```

---

## Further Reading

- [Basic Conversion](basic-conversion.md) -- getting started with simple conversions
- [Tag Preservation](../concepts/tag-preservation.md) -- deep dive into preserve_tags
- [Performance](../concepts/performance.md) -- optimization tips for configuration choices
