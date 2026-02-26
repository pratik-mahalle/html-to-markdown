# Basic Conversion

This guide walks through the fundamentals of converting HTML to Markdown with html-to-markdown. You will learn how to perform simple conversions, handle common HTML patterns, and apply basic options.

---

## Quick Start

The simplest conversion takes an HTML string and returns Markdown:

=== "Python"

    --8<-- "docs/snippets/python/getting-started/basic_usage.md"

=== "TypeScript"

    --8<-- "docs/snippets/typescript/getting-started/basic_usage.md"

=== "Rust"

    ```rust
    use html_to_markdown_rs::convert;

    let html = "<h1>Hello</h1><p>This is <strong>fast</strong>!</p>";
    let markdown = convert(html, None).unwrap();
    // # Hello
    //
    // This is **fast**!
    ```

=== "Ruby"

    --8<-- "docs/snippets/ruby/getting-started/basic_usage.md"

=== "PHP"

    --8<-- "docs/snippets/php/getting-started/basic_usage.md"

=== "Go"

    ```go
    package main

    import (
        "fmt"
        "github.com/kreuzberg-dev/html-to-markdown/packages/go/v2/htmltomarkdown"
    )

    func main() {
        markdown, _ := htmltomarkdown.Convert("<h1>Hello</h1><p>World</p>")
        fmt.Println(markdown)
    }
    ```

=== "C"

    --8<-- "docs/snippets/c/getting-started/basic_usage.md"

=== "Elixir"

    ```elixir
    {:ok, markdown} = HtmlToMarkdown.convert("<h1>Hello</h1><p>World</p>")
    IO.puts(markdown)
    ```

---

## Converting HTML Fragments

You do not need to provide a complete HTML document. html-to-markdown handles fragments gracefully:

```python
from html_to_markdown import convert

# Full document
convert("<html><body><h1>Title</h1></body></html>")

# Fragment -- works equally well
convert("<h1>Title</h1><p>Paragraph</p>")

# Single element
convert("<strong>Bold text</strong>")

# Plain text (no HTML tags) -- uses fast path
convert("Just plain text")
```

---

## Common HTML Patterns

### Headings

```html
<h1>Main Title</h1>
<h2>Subtitle</h2>
<h3>Section</h3>
```

Converts to (with default ATX style):

```markdown
# Main Title

## Subtitle

### Section
```

### Paragraphs and Formatting

```html
<p>This is a paragraph with <strong>bold</strong>, <em>italic</em>,
and <code>inline code</code>.</p>
<p>A second paragraph with a <a href="https://example.com">link</a>.</p>
```

Converts to:

```markdown
This is a paragraph with **bold**, *italic*, and `inline code`.

A second paragraph with a [link](https://example.com).
```

### Lists

```html
<ul>
  <li>First item</li>
  <li>Second item
    <ul>
      <li>Nested item</li>
    </ul>
  </li>
</ul>
<ol>
  <li>Step one</li>
  <li>Step two</li>
</ol>
```

Converts to:

```markdown
- First item
- Second item
  - Nested item

1. Step one
2. Step two
```

### Code Blocks

```html
<pre><code class="language-python">def hello():
    print("Hello, world!")
</code></pre>
```

Converts to:

````markdown
```python
def hello():
    print("Hello, world!")
```
````

### Tables

```html
<table>
  <thead>
    <tr><th>Name</th><th>Language</th></tr>
  </thead>
  <tbody>
    <tr><td>PyO3</td><td>Python</td></tr>
    <tr><td>NAPI-RS</td><td>TypeScript</td></tr>
  </tbody>
</table>
```

Converts to:

```markdown
| Name | Language |
| --- | --- |
| PyO3 | Python |
| NAPI-RS | TypeScript |
```

### Images

```html
<img src="photo.jpg" alt="A sunset" title="Beautiful sunset">
```

Converts to:

```markdown
![A sunset](photo.jpg "Beautiful sunset")
```

### Blockquotes

```html
<blockquote>
  <p>To be or not to be, that is the question.</p>
  <p>-- Shakespeare</p>
</blockquote>
```

Converts to:

```markdown
> To be or not to be, that is the question.
>
> -- Shakespeare
```

---

## Using Options

Pass a configuration object to control output formatting:

=== "Python"

    --8<-- "docs/snippets/python/getting-started/with_options.md"

=== "TypeScript"

    --8<-- "docs/snippets/typescript/getting-started/with_options.md"

=== "Rust"

    ```rust
    use html_to_markdown_rs::{convert, ConversionOptions, HeadingStyle};

    let options = ConversionOptions {
        heading_style: HeadingStyle::Atx,
        list_indent_width: 2,
        ..Default::default()
    };
    let markdown = convert(html, Some(options))?;
    ```

=== "Ruby"

    --8<-- "docs/snippets/ruby/getting-started/with_options.md"

=== "PHP"

    --8<-- "docs/snippets/php/getting-started/with_options.md"

For a complete list of all configuration options, see the [Configuration Options](configuration.md) guide.

---

## Error Handling

Conversion can fail if the input is invalid (binary data, PDF files, etc.). Always handle errors appropriately:

=== "Python"

    ```python
    from html_to_markdown import convert, ConversionError

    try:
        markdown = convert(html_input)
    except ConversionError as e:
        print(f"Conversion failed: {e}")
    ```

=== "TypeScript"

    ```typescript
    import { convert } from '@kreuzberg/html-to-markdown';

    try {
      const markdown = convert(htmlInput);
    } catch (error) {
      console.error('Conversion failed:', error);
    }
    ```

=== "Rust"

    ```rust
    use html_to_markdown_rs::{convert, ConversionError};

    match convert(html, None) {
        Ok(markdown) => println!("{}", markdown),
        Err(ConversionError::InvalidInput(msg)) => {
            eprintln!("Invalid input: {}", msg);
        }
        Err(e) => eprintln!("Error: {}", e),
    }
    ```

=== "Go"

    ```go
    markdown, err := htmltomarkdown.Convert(html)
    if err != nil {
        log.Fatalf("Conversion failed: %v", err)
    }
    ```

!!! warning "Binary input rejection"
    html-to-markdown validates input and rejects binary data such as PDF files, images, and other non-text content. If you receive an `InvalidInput` error, verify that your input is actually HTML text.

---

## Tips and Best Practices

1. **Let the library handle malformed HTML.** html5ever implements browser-grade error recovery. You do not need to pre-clean HTML before passing it to html-to-markdown.

2. **Use the default options first.** The defaults produce clean, CommonMark-compatible Markdown. Only customize when you have a specific need.

3. **Reuse option objects.** If you are converting multiple documents with the same settings, create the options once and pass them to each call.

4. **Fragment conversion is safe.** You can pass `<div>` fragments, single elements, or even plain text. No `<html>` wrapper is needed.

5. **Character encoding.** The library expects UTF-8 input. If your source uses a different encoding, convert it to UTF-8 first. UTF-16 input is automatically detected and recovered.

---

## Next Steps

- [Configuration Options](configuration.md) -- customize every aspect of the conversion
- [Metadata Extraction](metadata.md) -- extract titles, links, headers, and structured data
- [Visitor Pattern](visitor.md) -- programmatic customization of element conversion
- [CLI Usage](cli.md) -- convert files from the command line
