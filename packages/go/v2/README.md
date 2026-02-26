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
  <a href="https://github.com/kreuzberg-dev/html-to-markdown/releases">
    <img src="https://img.shields.io/badge/C-FFI-007ec6" alt="C">
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

High-performance HTML to Markdown converter with Go bindings to the Rust core library.
Supports automatic downloading of prebuilt FFI libraries for Linux, macOS, and Windows with customizable caching.

## Installation

```bash
go get github.com/kreuzberg-dev/html-to-markdown/packages/go/v2/htmltomarkdown
```

Requires Go 1.25+. After installing the package, run `go generate` to automatically download the platform-specific FFI library:

```bash
go generate
```

This downloads the native library from GitHub releases and generates the necessary CGO flags. The library is cached in `~/.html-to-markdown/` for subsequent builds.

Alternatively, you can manually set `CGO_CFLAGS` and `CGO_LDFLAGS` environment variables if you prefer to manage the FFI library yourself.

## Quick Start

Basic conversion:

```go
package main

import (
    "fmt"
    "log"

    "github.com/kreuzberg-dev/html-to-markdown/packages/go/v2/htmltomarkdown"
)

func main() {
    html := "<h1>Hello World</h1><p>This is a paragraph.</p>"

    markdown, err := htmltomarkdown.Convert(html)
    if err != nil {
        log.Fatal(err)
    }

    fmt.Println(markdown)
}
```

With `MustConvert` (panics on error, useful when input is known-safe):

```go
package main

import (
    "fmt"

    "github.com/kreuzberg-dev/html-to-markdown/packages/go/v2/htmltomarkdown"
)

func main() {
    markdown := htmltomarkdown.MustConvert("<p>Safe HTML</p>")
    fmt.Println(markdown)

    version := htmltomarkdown.Version()
    fmt.Printf("html-to-markdown version: %s\n", version)
}
```

## API Reference

### Core Functions

**`Convert(html string) (string, error)`**

Basic HTML-to-Markdown conversion. Returns the converted Markdown string or an error if conversion fails. Handles memory management automatically.

**`MustConvert(html string) string`**

Like `Convert` but panics on error. Useful when conversion errors are unexpected.

**`ConvertWithMetadata(html string) (MetadataExtraction, error)`**

Convert HTML to Markdown and extract comprehensive metadata (document info, headers, links, images, structured data) in a single pass.

**`MustConvertWithMetadata(html string) MetadataExtraction`**

Like `ConvertWithMetadata` but panics on error.

**`ConvertWithVisitor(html string, visitor Visitor, options *ConversionOptions) (string, error)`**

Customize conversion with visitor callbacks for element interception. Supports 40+ callbacks for text, inline elements, links, images, headings, lists, blocks, and tables.

**`Version() string`**

Returns the version string of the underlying html-to-markdown library.

### Options

**`ConversionOptions`** -- Key configuration fields:
- `heading_style`: Heading format (`"underlined"` | `"atx"` | `"atx_closed"`) -- default: `"underlined"`
- `list_indent_width`: Spaces per indent level -- default: `2`
- `bullets`: Bullet characters cycle -- default: `"*+-"`
- `wrap`: Enable text wrapping -- default: `false`
- `wrap_width`: Wrap at column -- default: `80`
- `code_language`: Default fenced code block language -- default: none
- `extract_metadata`: Embed metadata as YAML frontmatter -- default: `false`
- `output_format`: Output markup format (`"markdown"` | `"djot"`) -- default: `"markdown"`

**`MetadataConfig`** -- Selective metadata extraction:
- `extract_headers`: h1-h6 elements -- default: `true`
- `extract_links`: Hyperlinks -- default: `true`
- `extract_images`: Image elements -- default: `true`
- `extract_structured_data`: JSON-LD, Microdata, RDFa -- default: `true`
- `max_structured_data_size`: Size limit in bytes -- default: `100KB`

## Metadata Extraction

The metadata extraction feature enables comprehensive document analysis during conversion. Extract document properties, headers, links, images, and structured data in a single pass.

```go
package main

import (
    "fmt"
    "log"

    "github.com/kreuzberg-dev/html-to-markdown/packages/go/v2/htmltomarkdown"
)

func main() {
    html := `<html>
      <head>
        <title>My Article</title>
        <meta name="description" content="A great article">
      </head>
      <body>
        <h1>Main Title</h1>
        <p>Content with <a href="https://example.com">a link</a></p>
        <img src="image.jpg" alt="An image">
      </body>
    </html>`

    result, err := htmltomarkdown.ConvertWithMetadata(html)
    if err != nil {
        log.Fatal(err)
    }

    fmt.Println(result.Markdown)
    fmt.Printf("Title: %s\n", *result.Metadata.Document.Title)
    fmt.Printf("Headers: %d\n", len(result.Metadata.Headers))
    fmt.Printf("Links: %d\n", len(result.Metadata.Links))
    fmt.Printf("Images: %d\n", len(result.Metadata.Images))
}
```

## Visitor Pattern

The visitor pattern enables custom HTML-to-Markdown conversion logic by providing callbacks for specific HTML elements during traversal. Use visitors to transform content, filter elements, validate structure, or collect analytics.

```go
package main

import (
    "fmt"
    "log"
    "strings"

    "github.com/kreuzberg-dev/html-to-markdown/packages/go/v2/htmltomarkdown"
)

func main() {
    visitor := &htmltomarkdown.BaseVisitor{}
    visitor.OnVisitLink = func(ctx htmltomarkdown.NodeContext, href, text, title string) htmltomarkdown.VisitResult {
        // Rewrite CDN URLs
        if strings.HasPrefix(href, "https://old-cdn.com") {
            href = strings.Replace(href, "https://old-cdn.com", "https://new-cdn.com", 1)
            return htmltomarkdown.VisitResult{Type: htmltomarkdown.VisitCustom, Output: fmt.Sprintf("[%s](%s)", text, href)}
        }
        return htmltomarkdown.VisitResult{Type: htmltomarkdown.VisitContinue}
    }

    html := `<a href="https://old-cdn.com/file.pdf">Download</a>`
    markdown, err := htmltomarkdown.ConvertWithVisitor(html, visitor, nil)
    if err != nil {
        log.Fatal(err)
    }
    fmt.Println(markdown)
}
```

## Links

- **Documentation:** [docs.html-to-markdown.kreuzberg.dev](https://docs.html-to-markdown.kreuzberg.dev)
- **GitHub:** [github.com/kreuzberg-dev/html-to-markdown](https://github.com/kreuzberg-dev/html-to-markdown)
- **Go Packages:** [pkg.go.dev/github.com/kreuzberg-dev/html-to-markdown/packages/go/v2](https://pkg.go.dev/github.com/kreuzberg-dev/html-to-markdown/packages/go/v2)
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

MIT License -- see [LICENSE](https://github.com/kreuzberg-dev/html-to-markdown/blob/main/LICENSE).
