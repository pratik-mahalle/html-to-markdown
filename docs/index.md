---
title: html-to-markdown
description: High-performance HTML to Markdown conversion powered by Rust
---

# html-to-markdown

**High-performance HTML to Markdown conversion powered by Rust**

Convert HTML to clean, readable Markdown at 150--280 MB/s. A single Rust core with native bindings for 12 language ecosystems, delivering identical output across every runtime.

---

## Key Features

| | Feature | Description |
|---|---|---|
| :material-lightning-bolt: | **Blazing Fast** | 150--280 MB/s throughput, 10--80x faster than pure Python alternatives |
| :material-translate: | **Polyglot** | 12 native bindings -- Rust, Python, TypeScript, Ruby, PHP, Go, Java, C#, Elixir, R, C, WASM |
| :material-file-document-check: | **Smart Conversion** | Nested tables, code blocks, task lists, hOCR, and complex HTML structures |
| :material-tag-text: | **Metadata Extraction** | Title, description, headers, links, images, Open Graph, JSON-LD, Microdata |
| :material-eye: | **Visitor Pattern** | Custom callbacks for content filtering, URL rewriting, and domain-specific dialects |
| :material-shield-check: | **Secure by Default** | Built-in HTML sanitization powered by ammonia prevents malicious content |

---

## Quick Install

=== "Python"

    ```bash
    pip install html-to-markdown
    ```

=== "TypeScript"

    ```bash
    npm install @kreuzberg/html-to-markdown-node
    ```

=== "Rust"

    ```bash
    cargo add html-to-markdown-rs
    ```

=== "Ruby"

    ```bash
    gem install html-to-markdown
    ```

=== "PHP"

    ```bash
    composer require kreuzberg-dev/html-to-markdown
    ```

=== "CLI"

    ```bash
    cargo install html-to-markdown-cli
    ```

    Or via Homebrew:

    ```bash
    brew install kreuzberg-dev/tap/html-to-markdown
    ```

---

## Quick Example

=== "Python"

    ```python
    from html_to_markdown import convert

    html = "<h1>Hello</h1><p>This is <strong>fast</strong>!</p>"
    markdown = convert(html)
    ```

=== "TypeScript"

    ```typescript
    import { convert } from '@kreuzberg/html-to-markdown';

    const markdown: string = convert('<h1>Hello World</h1>');
    console.log(markdown); // # Hello World
    ```

=== "Rust"

    ```rust
    use html_to_markdown_rs::convert;

    let html = "<h1>Hello</h1><p>This is <strong>fast</strong>!</p>";
    let markdown = convert(html, None)?;
    ```

---

## Live Demo

Try html-to-markdown directly in your browser -- no installation required. The demo runs entirely client-side using the WebAssembly build.

[Try the Live Demo](https://kreuzberg-dev.github.io/html-to-markdown/){ .md-button .md-button--primary }

---

## Part of the Kreuzberg Ecosystem

html-to-markdown powers the HTML conversion pipeline in [kreuzberg](https://docs.kreuzberg.dev), a document intelligence library for extracting text and structured data from any document format. If you need to process PDFs, DOCX, images, or other document types, check out kreuzberg -- it uses html-to-markdown internally for all HTML-to-Markdown conversion.

---

## Explore the Docs

- **[Installation](getting-started/installation.md)** -- Package manager commands for all 12 language bindings
- **[Quick Start](getting-started/quickstart.md)** -- Get converting in under a minute
- **[Features](features.md)** -- Detailed overview of capabilities
- **[Configuration](guides/configuration.md)** -- Control heading styles, code fences, list formatting, and more
- **[Visitor Pattern](guides/visitor.md)** -- Custom callbacks for advanced conversion control
- **[Metadata Extraction](guides/metadata.md)** -- Extract structured document metadata alongside conversion
- **[API Reference](reference/api-python.md)** -- Language-specific API documentation
- **[Contributing](contributing.md)** -- Development setup and contribution guidelines
