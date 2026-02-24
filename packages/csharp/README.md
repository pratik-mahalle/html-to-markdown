# html-to-markdown

<div align="center" style="display: flex; flex-wrap: wrap; gap: 8px; justify-content: center; margin: 20px 0;">
  <!-- Language Bindings -->
  <a href="https://crates.io/crates/html-to-markdown-rs">
    <img src="https://img.shields.io/crates/v/html-to-markdown-rs?label=Rust&color=007ec6" alt="Rust">
  </a>
  <a href="https://pypi.org/project/html-to-markdown/">
    <img src="https://img.shields.io/pypi/v/html-to-markdown?label=Python&color=007ec6" alt="Python">
  </a>
  <a href="https://www.npmjs.com/package/@kreuzberg/html-to-markdown-node">
    <img src="https://img.shields.io/npm/v/@kreuzberg/html-to-markdown-node?label=Node.js&color=007ec6" alt="Node.js">
  </a>
  <a href="https://www.npmjs.com/package/@kreuzberg/html-to-markdown-wasm">
    <img src="https://img.shields.io/npm/v/@kreuzberg/html-to-markdown-wasm?label=WASM&color=007ec6" alt="WASM">
  </a>
  <a href="https://central.sonatype.com/artifact/dev.kreuzberg/html-to-markdown">
    <img src="https://img.shields.io/maven-central/v/dev.kreuzberg/html-to-markdown?label=Java&color=007ec6" alt="Java">
  </a>
  <a href="https://pkg.go.dev/github.com/kreuzberg-dev/html-to-markdown/packages/go/v2/htmltomarkdown">
    <img src="https://img.shields.io/badge/Go-v2.25.1-007ec6" alt="Go">
  </a>
  <a href="https://www.nuget.org/packages/KreuzbergDev.HtmlToMarkdown/">
    <img src="https://img.shields.io/nuget/v/KreuzbergDev.HtmlToMarkdown?label=C%23&color=007ec6" alt="C#">
  </a>
  <a href="https://packagist.org/packages/kreuzberg-dev/html-to-markdown">
    <img src="https://img.shields.io/packagist/v/kreuzberg-dev/html-to-markdown?label=PHP&color=007ec6" alt="PHP">
  </a>
  <a href="https://rubygems.org/gems/html-to-markdown">
    <img src="https://img.shields.io/gem/v/html-to-markdown?label=Ruby&color=007ec6" alt="Ruby">
  </a>
  <a href="https://hex.pm/packages/html_to_markdown">
    <img src="https://img.shields.io/hexpm/v/html_to_markdown?label=Elixir&color=007ec6" alt="Elixir">
  </a>
  <a href="https://cran.r-project.org/package=htmltomarkdown">
    <img src="https://img.shields.io/cran/v/htmltomarkdown?label=R&color=007ec6" alt="R">
  </a>

  <!-- Project Info -->
  <a href="https://github.com/kreuzberg-dev/html-to-markdown/blob/main/LICENSE">
    <img src="https://img.shields.io/badge/License-MIT-blue.svg" alt="License">
  </a>
</div>

<img width="1128" height="191" alt="html-to-markdown" src="https://github.com/user-attachments/assets/419fc06c-8313-4324-b159-4b4d3cfce5c0" />

<div align="center" style="margin-top: 20px;">
  <a href="https://discord.gg/pXxagNK2zN">
      <img height="22" src="https://img.shields.io/badge/Discord-Join%20our%20community-7289da?logo=discord&logoColor=white" alt="Discord">
  </a>
</div>


High-performance HTML to Markdown converter with C#/.NET bindings using P/Invoke to the Rust core.
Provides type-safe record-based APIs for metadata extraction, visitor patterns, and thread-safe concurrent conversion.


## Installation

```bash
dotnet add package KreuzbergDev.HtmlToMarkdown
```



Requires .NET 8.0+ SDK.

```bash
dotnet add package KreuzbergDev.HtmlToMarkdown
```






## Performance Snapshot

Apple M4 • Real Wikipedia documents • `Convert()` (C# / .NET)

| Document | Size | Ops/sec | Throughput |
| -------- | ---- | ------- | ---------- |
| Lists (Timeline) | 129KB | 3,111 | 392.9 MB/s |
| Tables (Countries) | 360KB | 853 | 300.1 MB/s |
| Mixed (Python) | 656KB | 456 | 292.3 MB/s |


See [Performance Guide](../../examples/performance/) for detailed benchmarks.


## Quick Start

Basic conversion:

```csharp
using HtmlToMarkdown;

var html = "<h1>Hello World</h1><p>This is a paragraph.</p>";

try
{
    var markdown = HtmlToMarkdownConverter.Convert(html);
    Console.WriteLine(markdown);
}
catch (HtmlToMarkdownException ex)
{
    Console.Error.WriteLine($"Conversion failed: {ex.Message}");
}
```



With conversion options:

```csharp
using HtmlToMarkdown;

try
{
    string html = """
        <html>
        <head>
            <title>My Article</title>
            <meta name="description" content="An interesting read">
            <meta name="author" content="Jane Doe">
            <meta property="og:image" content="image.jpg">
        </head>
        <body>
            <h1>Welcome</h1>
            <a href="https://example.com">Link</a>
            <img src="image.jpg" alt="Featured image">
        </body>
        </html>
        """;

    var result = HtmlToMarkdownConverter.ConvertWithMetadata(html);

    // Access document metadata
    var doc = result.Metadata.Document;
    if (doc.Title != null)
    {
        Console.WriteLine($"Title: {doc.Title}");
    }
    if (doc.Author != null)
    {
        Console.WriteLine($"Author: {doc.Author}");
    }

    // Access Open Graph metadata
    if (doc.OpenGraph != null)
    {
        foreach (var (key, value) in doc.OpenGraph)
        {
            Console.WriteLine($"OG {key}: {value}");
        }
    }

    // Count extracted elements
    Console.WriteLine($"Headers: {result.Metadata.Headers.Count}");
    Console.WriteLine($"Links: {result.Metadata.Links.Count}");
    Console.WriteLine($"Images: {result.Metadata.Images.Count}");

    // Print markdown output
    Console.WriteLine($"\nMarkdown:\n{result.Markdown}");
}
catch (HtmlToMarkdownException ex)
{
    Console.Error.WriteLine($"Conversion failed: {ex.Message}");
}
catch (JsonException ex)
{
    Console.Error.WriteLine($"Metadata parsing failed: {ex.Message}");
}
```






## API Reference

### Core Functions


**`Convert(string html, ConversionOptions? options = null) : string`**

Basic HTML-to-Markdown conversion. Fast and simple.

**`ConvertWithMetadata(string html, ConversionOptions? options = null, MetadataConfig? config = null) : (string markdown, MetadataResult metadata)`**

Extract Markdown plus metadata in a single pass. See [Metadata Extraction Guide](../../examples/metadata-extraction/).

**`ConvertWithInlineImages(string html, InlineImageConfig? config = null) : (string markdown, InlineImageData[] images, string[] warnings)`**

Extract base64-encoded inline images with metadata.



### Options

**`ConversionOptions`** – Key configuration fields:
- `heading_style`: Heading format (`"underlined"` | `"atx"` | `"atx_closed"`) — default: `"underlined"`
- `list_indent_width`: Spaces per indent level — default: `2`
- `bullets`: Bullet characters cycle — default: `"*+-"`
- `wrap`: Enable text wrapping — default: `false`
- `wrap_width`: Wrap at column — default: `80`
- `code_language`: Default fenced code block language — default: none
- `extract_metadata`: Embed metadata as YAML frontmatter — default: `false`
- `output_format`: Output markup format (`"markdown"` | `"djot"`) — default: `"markdown"`

**`MetadataConfig`** – Selective metadata extraction:
- `extract_headers`: h1-h6 elements — default: `true`
- `extract_links`: Hyperlinks — default: `true`
- `extract_images`: Image elements — default: `true`
- `extract_structured_data`: JSON-LD, Microdata, RDFa — default: `true`
- `max_structured_data_size`: Size limit in bytes — default: `100KB`


## Djot Output Format

The library supports converting HTML to [Djot](https://djot.net/), a lightweight markup language similar to Markdown but with a different syntax for some elements. Set `output_format` to `"djot"` to use this format.

### Syntax Differences

| Element | Markdown | Djot |
|---------|----------|------|
| Strong | `**text**` | `*text*` |
| Emphasis | `*text*` | `_text_` |
| Strikethrough | `~~text~~` | `{-text-}` |
| Inserted/Added | N/A | `{+text+}` |
| Highlighted | N/A | `{=text=}` |
| Subscript | N/A | `~text~` |
| Superscript | N/A | `^text^` |

### Example Usage


```csharp
using HtmlToMarkdown;

var html = "<p>This is <strong>bold</strong> and <em>italic</em> text.</p>";

// Default Markdown output
var markdown = Converter.Convert(html);
// Result: "This is **bold** and *italic* text."

// Djot output
var djot = Converter.Convert(html, new ConversionOptions { OutputFormat = "djot" });
// Result: "This is *bold* and _italic_ text."
```


Djot's extended syntax allows you to express more semantic meaning in lightweight text, making it useful for documents that require strikethrough, insertion tracking, or mathematical notation.






## Examples

- [Visitor Pattern Guide](../../examples/visitor-pattern/)
- [Metadata Extraction Guide](../../examples/metadata-extraction/)
- [Performance Guide](../../examples/performance/)

## Links

- **GitHub:** [github.com/kreuzberg-dev/html-to-markdown](https://github.com/kreuzberg-dev/html-to-markdown)

- **NuGet:** [nuget.org/packages/KreuzbergDev.HtmlToMarkdown](https://www.nuget.org/packages/KreuzbergDev.HtmlToMarkdown/)

- **Kreuzberg Ecosystem:** [kreuzberg.dev](https://kreuzberg.dev)
- **Discord:** [discord.gg/pXxagNK2zN](https://discord.gg/pXxagNK2zN)

## Contributing

We welcome contributions! Please see our [Contributing Guide](https://github.com/kreuzberg-dev/html-to-markdown/blob/main/CONTRIBUTING.md) for details on:

- Setting up the development environment
- Running tests locally
- Submitting pull requests
- Reporting issues

All contributions must follow our code quality standards (enforced via pre-commit hooks):

- Proper test coverage (Rust 95%+, language bindings 80%+)
- Formatting and linting checks
- Documentation for public APIs

## License

MIT License – see [LICENSE](https://github.com/kreuzberg-dev/html-to-markdown/blob/main/LICENSE).

## Support

If you find this library useful, consider [sponsoring the project](https://github.com/sponsors/kreuzberg-dev).

Have questions or run into issues? We're here to help:

- **GitHub Issues:** [github.com/kreuzberg-dev/html-to-markdown/issues](https://github.com/kreuzberg-dev/html-to-markdown/issues)
- **Discussions:** [github.com/kreuzberg-dev/html-to-markdown/discussions](https://github.com/kreuzberg-dev/html-to-markdown/discussions)
- **Discord Community:** [discord.gg/pXxagNK2zN](https://discord.gg/pXxagNK2zN)
