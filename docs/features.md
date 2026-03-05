---
title: Features
description: Detailed overview of html-to-markdown capabilities
---

# Features

html-to-markdown is a comprehensive HTML-to-Markdown conversion library built on a high-performance Rust core with native bindings for 12 language ecosystems.

---

## Performance

The Rust core delivers **150--280 MB/s** throughput for HTML-to-Markdown conversion, making it **10--80x faster** than pure Python alternatives. Performance is consistent across all language bindings because the heavy lifting always happens in native compiled Rust code.

Key performance characteristics:

- **Zero-copy parsing** where possible, minimizing memory allocations
- **Single-pass conversion** -- HTML is parsed and converted to Markdown in one pass
- **Compiled native extensions** -- Python (PyO3), Node.js (NAPI-RS), Ruby (Magnus), PHP (ext-php-rs), Elixir (Rustler), R (extendr)
- **Benchmark-verified** with Criterion.rs, tracked per release to prevent regressions

!!! tip "Performance in kreuzberg"
    [kreuzberg](https://docs.kreuzberg.dev) uses html-to-markdown internally for HTML conversion. The speed of html-to-markdown directly benefits kreuzberg's document processing pipeline, particularly when batch-processing large volumes of HTML documents.

---

## 12 Language Bindings

Every binding wraps the same Rust core, producing **identical Markdown output** regardless of which language you use. No logic is duplicated in binding code -- all conversion logic lives in Rust.

| Binding | Mechanism | Package | Since |
|---|---|---|---|
| **Rust** | Native | [`html-to-markdown-rs`](https://crates.io/crates/html-to-markdown-rs) | <span class="version-badge">v2.0.0</span> |
| **Python** | PyO3 | [`html-to-markdown`](https://pypi.org/project/html-to-markdown/) | <span class="version-badge">v2.0.0</span> |
| **TypeScript / Node.js** | NAPI-RS | [`@kreuzberg/html-to-markdown-node`](https://www.npmjs.com/package/@kreuzberg/html-to-markdown-node) | <span class="version-badge">v2.3.0</span> |
| **WASM (Browser)** | wasm-bindgen | [`@kreuzberg/html-to-markdown-wasm`](https://www.npmjs.com/package/@kreuzberg/html-to-markdown-wasm) | <span class="version-badge">v2.3.0</span> |
| **Ruby** | Magnus | [`html-to-markdown`](https://rubygems.org/gems/html-to-markdown) | <span class="version-badge">v2.5.1</span> |
| **PHP** | ext-php-rs | [`kreuzberg-dev/html-to-markdown`](https://packagist.org/packages/kreuzberg-dev/html-to-markdown) | <span class="version-badge">v2.5.6</span> |
| **Go** | CGO / FFI | [`htmltomarkdown`](https://pkg.go.dev/github.com/kreuzberg-dev/html-to-markdown/packages/go/v2/htmltomarkdown) | <span class="version-badge">v2.8.0</span> |
| **Java** | Panama FFM API | [`dev.kreuzberg:html-to-markdown`](https://central.sonatype.com/artifact/dev.kreuzberg/html-to-markdown) | <span class="version-badge new">v2.24.2</span> |
| **C#** | P/Invoke | [`KreuzbergDev.HtmlToMarkdown`](https://www.nuget.org/packages/KreuzbergDev.HtmlToMarkdown/) | <span class="version-badge">v2.8.0</span> |
| **Elixir** | Rustler NIF | [`html_to_markdown`](https://hex.pm/packages/html_to_markdown) | <span class="version-badge">v2.8.2</span> |
| **R** | extendr | [`htmltomarkdown`](https://kreuzberg-dev.r-universe.dev/htmltomarkdown) | <span class="version-badge new">v2.25.2</span> |
| **C** | FFI | [Releases](https://github.com/kreuzberg-dev/html-to-markdown/releases) | <span class="version-badge new">v2.26.0</span> |

---

## Smart Conversion

html-to-markdown handles complex HTML structures that trip up simpler converters:

- **Nested tables** -- Correctly converts tables within tables, preserving structure
- **Code blocks** -- Detects `<pre><code>` blocks, respects language hints, handles fenced and indented styles
- **Task lists** -- Converts checkbox inputs to GitHub-flavored Markdown task lists
- **hOCR output** -- Handles OCR-generated HTML (hOCR format) for document digitization workflows
- **Definition lists** -- Converts `<dl>/<dt>/<dd>` structures
- **Inline formatting** -- Bold, italic, strikethrough, subscript, superscript, highlights
- **Block quotes** -- Nested `<blockquote>` elements with proper indentation
- **Horizontal rules** -- `<hr>` elements converted to Markdown thematic breaks

---

## Metadata Extraction <span class="version-badge">v2.13.0</span>

Extract comprehensive document metadata alongside the Markdown conversion in a single pass. No need for a separate parsing step.

Extracted metadata includes:

- **Document info** -- Title, description, author, canonical URL, language, text direction
- **Headers** -- All heading elements with level, text, ID, and DOM depth
- **Links** -- Every hyperlink with href, text, title, type classification (internal, external, anchor, email, phone), and rel attributes
- **Images** -- Image sources with alt text, title, dimensions, and type classification (external, data URI, inline SVG, relative)
- **Structured data** -- JSON-LD, Microdata, and RDFa embedded in the HTML
- **Open Graph** -- All `og:*` meta properties
- **Twitter Card** -- All `twitter:*` meta properties

Available in: Rust, Python, TypeScript/Node.js, Ruby, PHP, Go, Java, C#, Elixir, R, and C.

---

## Visitor Pattern <span class="version-badge new">v2.23.0</span>

The visitor pattern lets you intercept and customize the conversion of specific HTML elements. Register callbacks for headings, links, images, lists, code blocks, and more.

Use cases:

- **Domain-specific Markdown dialects** -- Generate Obsidian, Notion, or custom Markdown flavors
- **Content filtering** -- Skip or transform specific elements during conversion
- **URL rewriting** -- Rewrite relative URLs to absolute, proxy through CDN, or remove tracking parameters
- **Accessibility validation** -- Flag images without alt text or links without descriptive text
- **Analytics** -- Count elements, collect statistics, and build document profiles during conversion

Each visitor callback returns a result that controls processing:

- **Continue** -- Proceed with default conversion
- **Skip** -- Omit the element from output
- **Custom** -- Replace the element with custom Markdown text

Visitor support varies by binding:

| Binding | Sync Visitor | Async Visitor |
|---|---|---|
| Rust | Yes | Yes (Tokio) |
| Python | Yes | Yes (asyncio) |
| TypeScript / Node.js | Yes | Yes (Promise) |
| Ruby | Yes | No |
| PHP | Yes | No |

---

## Table Extraction <span class="version-badge new">v2.27.3</span>

Extract structured table data alongside the Markdown conversion in a single pass. Each `<table>` element found in the HTML is returned with its cell contents (already converted to the target format), header row flags, and the fully rendered table output.

Use cases:

- **Data extraction** -- Pull structured tabular data from HTML pages for processing or storage
- **Spreadsheet import** -- Convert HTML tables to structured data suitable for CSV or spreadsheet formats
- **Content analysis** -- Analyze table structures, count rows/columns, and inspect header patterns during conversion

Table extraction is available in all language bindings: Rust, Python, TypeScript/Node.js, Ruby, PHP, Go, Java, C#, Elixir, R, C (FFI), and WASM.

---

## Configuration

Control every aspect of the Markdown output through `ConversionOptions`:

### Heading Styles

- **ATX** (default) -- `# Heading` with hash prefixes
- **ATX Closed** -- `# Heading #` with closing hashes
- **Underlined** (Setext) -- Underlined with `===` or `---`

### Code Block Styles

- **Fenced** (default) -- Triple backtick code fences
- **Indented** -- Four-space indentation

### List Formatting

- **Indent type** -- Spaces or tabs
- **Indent width** -- Configurable number of spaces per indent level
- **Marker style** -- Control bullet and ordered list markers

### Whitespace and Wrapping

- **Text wrapping** -- Enable CommonMark soft line breaks at a configurable column width
- **Newline style** -- Control line endings (Unix, Windows)

### Image Handling

- **Skip images** -- Remove all image tags from output
- **Inline image preservation** -- Keep images inline within specific parent tags

### Preprocessing

- **Remove navigation** -- Strip `<nav>`, `<header>`, `<footer>` elements before conversion
- **Remove forms** -- Strip `<form>` elements and their contents
- **Preset modes** -- Built-in preprocessing profiles for common use cases

---

## Tag Preservation <span class="version-badge">v2.5.0</span>

When Markdown is not expressive enough, preserve specific HTML tags unconverted in the output. Useful for:

- Complex table layouts that lose structure in Markdown tables
- SVG graphics and embedded media
- Custom HTML elements with semantic meaning
- Mixed HTML/Markdown output workflows

Configure via `preserve_tags` in options -- specify an array of tag names to keep as raw HTML in the Markdown output.

---

## HTML Sanitization

html-to-markdown includes built-in HTML sanitization powered by [ammonia](https://github.com/rust-ammonia/ammonia), protecting against:

- Cross-site scripting (XSS) via malicious `<script>` or event handler attributes
- Dangerous protocols in URLs (`javascript:`, `data:` with executable content)
- Style injection attacks
- Malformed HTML designed to exploit parsers

Sanitization runs before conversion, so the Markdown output is clean by default. No additional sanitization step is required.

---

## CLI Tool

A standalone command-line tool for converting HTML files to Markdown without writing any code.

```bash
# Install via cargo
cargo install html-to-markdown-cli

# Or via Homebrew
brew install kreuzberg-dev/tap/html-to-markdown

# Convert a file
html-to-markdown input.html

# Pipe from stdin
curl -s https://example.com | html-to-markdown

# With options
html-to-markdown --heading-style atx --wrap --wrap-width 80 input.html
```

---

## WASM Support <span class="version-badge">v2.3.0</span>

Run html-to-markdown in the browser, Deno, or Cloudflare Workers via WebAssembly. The WASM build provides the same conversion quality as the native bindings with no server round-trips.

- **Browser** -- ES module import, works in Chrome 89+, Firefox 79+, Safari 14.1+
- **Node.js** -- CommonJS and ESM support
- **Deno** -- Native import support
- **Cloudflare Workers** -- Edge compute with WASM execution

All conversion happens client-side. No data is sent to any server.

[Try the Live Demo](https://kreuzberg-dev.github.io/html-to-markdown/){ .md-button }
