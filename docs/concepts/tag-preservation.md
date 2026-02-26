# Tag Preservation

Tag preservation allows you to keep specific HTML elements as raw HTML in the Markdown output instead of converting them to Markdown syntax. This is useful when Markdown cannot represent the full richness of certain HTML structures.

---

## What Tag Preservation Does

When you add a tag name to the `preserve_tags` configuration list, any matching HTML element and its contents are passed through verbatim into the Markdown output. The element is not converted -- the raw HTML appears as-is in the final Markdown string.

**Without preservation:**

```html
<table>
  <tr><td style="color: red">Important</td><td>Normal</td></tr>
</table>
```

Becomes a standard GFM table (losing the inline style):

```markdown
| Important | Normal |
| --- | --- |
```

**With `preserve_tags: ["table"]`:**

The original HTML table is kept intact in the output:

```html
<table>
  <tr><td style="color: red">Important</td><td>Normal</td></tr>
</table>
```

---

## When to Use Tag Preservation

### Complex Tables

Markdown tables are limited to simple grids without merged cells, colspans, rowspans, or styling. Preserve `<table>` when you need:

- Merged cells (`colspan`, `rowspan`)
- Cell background colors or alignment styles
- Nested tables
- Caption elements (`<caption>`)

### SVG Content

SVG elements cannot be represented in Markdown. Preserve `<svg>` to keep vector graphics inline:

```python
options = ConversionOptions(preserve_tags=["svg"])
```

### Custom Elements / Web Components

Web components use custom tag names that have no Markdown equivalent:

```html
<my-widget data-config='{"theme": "dark"}'>Content</my-widget>
```

Preserving `my-widget` keeps the custom element intact for client-side JavaScript to process.

### Interactive Elements

Elements like `<details>`/`<summary>`, `<dialog>`, or form elements may need to be preserved when the Markdown renderer supports inline HTML:

```python
options = ConversionOptions(preserve_tags=["details", "summary"])
```

### Mixed HTML/Markdown Content

When generating Markdown that will be rendered in environments supporting inline HTML (GitHub, GitLab, most static site generators), preserving specific elements gives you the best of both worlds:

- Standard content converts cleanly to Markdown
- Complex structures stay as HTML where Markdown falls short

---

## Configuration

Tag preservation is configured through the `preserve_tags` option, which accepts a list of HTML tag names (case-insensitive).

=== "Python"

    ```python
    from html_to_markdown import ConversionOptions, convert

    options = ConversionOptions(
        preserve_tags=["table", "svg", "details", "summary"],
    )
    markdown = convert(html, options)
    ```

=== "TypeScript"

    ```typescript
    import { convert } from '@kreuzberg/html-to-markdown';

    const markdown = convert(html, {
      preserveTags: ['table', 'svg', 'details', 'summary'],
    });
    ```

=== "Rust"

    ```rust
    use html_to_markdown_rs::{convert, ConversionOptions};

    let options = ConversionOptions {
        preserve_tags: vec![
            "table".to_string(),
            "svg".to_string(),
            "details".to_string(),
            "summary".to_string(),
        ],
        ..Default::default()
    };
    let markdown = convert(html, Some(options))?;
    ```

=== "Ruby"

    ```ruby
    require 'html_to_markdown'

    markdown = HtmlToMarkdown.convert(html, preserve_tags: %w[table svg details summary])
    ```

=== "PHP"

    ```php
    use HtmlToMarkdown\Config\ConversionOptions;
    use HtmlToMarkdown\Service\Converter;

    $converter = Converter::create();
    $options = new ConversionOptions(
        preserveTags: ['table', 'svg', 'details', 'summary'],
    );
    $markdown = $converter->convert($html, $options);
    ```

---

## How It Works

During [DOM traversal](conversion-pipeline.md#stage-3-dom-traversal), when the engine encounters an element whose tag name is in the `preserve_tags` list:

1. The element's opening tag, attributes, children, and closing tag are serialized back to HTML
2. The HTML string is inserted directly into the Markdown output
3. No Markdown conversion is applied to the element or any of its descendants
4. Surrounding content continues to be converted normally

!!! warning "Children are also preserved"
    When a tag is preserved, all of its child elements are included as raw HTML too. If you preserve `<div>`, everything inside that `<div>` stays as HTML, even elements that would normally convert cleanly to Markdown.

---

## Tag Preservation vs. Other Approaches

| Approach | Behavior | Best For |
|----------|----------|----------|
| `preserve_tags` | Keep entire element as HTML | Complex structures that need full HTML |
| `strip_tags` | Remove tags, keep text content | Removing unwanted wrappers |
| `skip_images` | Omit `<img>` elements entirely | Text-only extraction |
| Visitor `PreserveHtml` | Per-element decision via callback | Conditional preservation logic |

!!! tip "Visitor for conditional preservation"
    If you need to preserve some instances of a tag but convert others (e.g., preserve tables with `colspan` but convert simple tables), use the [visitor pattern](visitor-pattern.md) with a `PreserveHtml` return value based on element attributes.

---

## Further Reading

- [Configuration Options Guide](../guides/configuration.md) -- all configuration options including `preserve_tags`
- [Visitor Pattern](visitor-pattern.md) -- programmatic control over element handling
- [Conversion Pipeline](conversion-pipeline.md) -- how tag preservation fits into the pipeline
