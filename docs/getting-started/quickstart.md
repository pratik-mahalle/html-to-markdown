---
title: Quick Start
description: Get started converting HTML to Markdown in under a minute
---

# Quick Start

This guide walks you through your first HTML-to-Markdown conversion, then shows how to customize the output with options.

---

## Basic Conversion

Convert an HTML string to Markdown with a single function call.

=== "Python"

    --8<-- "docs/snippets/python/getting-started/basic_usage.md"

=== "TypeScript"

    --8<-- "docs/snippets/typescript/getting-started/basic_usage.md"

=== "Rust"

    ```rust
    use html_to_markdown_rs::convert;

    fn main() -> Result<(), Box<dyn std::error::Error>> {
        let html = "<h1>Hello</h1><p>This is <strong>fast</strong>!</p>";
        let markdown = convert(html, None)?;
        println!("{}", markdown);
        // Output:
        // # Hello
        //
        // This is **fast**!
        Ok(())
    }
    ```

=== "Ruby"

    --8<-- "docs/snippets/ruby/getting-started/basic_usage.md"

=== "PHP"

    --8<-- "docs/snippets/php/getting-started/basic_usage.md"

=== "Go"

    --8<-- "docs/snippets/go/getting-started/basic_usage.md"

=== "Java"

    --8<-- "docs/snippets/java/getting-started/basic_usage.md"

=== "C#"

    --8<-- "docs/snippets/csharp/getting-started/basic_usage.md"

=== "Elixir"

    --8<-- "docs/snippets/elixir/getting-started/basic_usage.md"

=== "R"

    --8<-- "docs/snippets/r/getting-started/basic_usage.md"

=== "C"

    --8<-- "docs/snippets/c/getting-started/basic_usage.md"

---

## Conversion with Options

Customize the Markdown output by passing configuration options. Every binding exposes the same set of options through language-idiomatic APIs.

=== "Python"

    --8<-- "docs/snippets/python/getting-started/with_options.md"

=== "TypeScript"

    --8<-- "docs/snippets/typescript/getting-started/with_options.md"

=== "Rust"

    ```rust
    use html_to_markdown_rs::{convert, ConversionOptions};

    let options = ConversionOptions {
        heading_style: Some("atx".into()),
        list_indent_width: Some(2),
        ..Default::default()
    };

    let html = "<h1>Title</h1><p>Content</p>";
    let markdown = convert(html, Some(options))?;
    ```

=== "Ruby"

    --8<-- "docs/snippets/ruby/getting-started/with_options.md"

=== "PHP"

    --8<-- "docs/snippets/php/getting-started/with_options.md"

=== "Go"

    --8<-- "docs/snippets/go/getting-started/with_options.md"

=== "Java"

    --8<-- "docs/snippets/java/getting-started/with_options.md"

=== "C#"

    --8<-- "docs/snippets/csharp/getting-started/with_options.md"

=== "Elixir"

    --8<-- "docs/snippets/elixir/getting-started/with_options.md"

=== "R"

    --8<-- "docs/snippets/r/getting-started/with_options.md"

=== "C"

    --8<-- "docs/snippets/c/getting-started/with_options.md"

---

## What Gets Converted

html-to-markdown handles the full range of HTML elements you would encounter in web content:

| HTML | Markdown |
|---|---|
| `<h1>` through `<h6>` | `#` through `######` headings |
| `<p>` | Paragraphs separated by blank lines |
| `<strong>`, `<b>` | `**bold**` |
| `<em>`, `<i>` | `*italic*` |
| `<a href="...">` | `[text](url)` links |
| `<img src="...">` | `![alt](src)` images |
| `<ul>`, `<ol>` | Bulleted and numbered lists |
| `<pre><code>` | Fenced code blocks |
| `<blockquote>` | `>` block quotes |
| `<table>` | Markdown tables with alignment |
| `<hr>` | `---` thematic breaks |
| `<del>`, `<s>` | `~~strikethrough~~` |
| `<input type="checkbox">` | `- [ ]` / `- [x]` task lists |

---

## Next Steps

- **[Configuration Options](../guides/configuration.md)** -- Full reference for all conversion options
- **[Metadata Extraction](../guides/metadata.md)** -- Extract document metadata alongside conversion
- **[Visitor Pattern](../guides/visitor.md)** -- Customize conversion with callbacks
- **[CLI Usage](../guides/cli.md)** -- Convert files from the command line
- **[Features](../features.md)** -- Complete feature overview

!!! tip "Using kreuzberg?"
    If you are processing diverse document types (PDFs, DOCX, images) and need Markdown output, consider [kreuzberg](https://docs.kreuzberg.dev) which uses html-to-markdown internally and adds OCR, format detection, and multi-format extraction.
