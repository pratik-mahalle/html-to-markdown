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
    <img src="https://img.shields.io/badge/Go-v2.28.2-007ec6" alt="Go">
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
  <a href="https://kreuzberg-dev.r-universe.dev/htmltomarkdown">
    <img src="https://img.shields.io/badge/R-htmltomarkdown-007ec6" alt="R">
  </a>
  <a href="https://github.com/kreuzberg-dev/html-to-markdown/releases">
    <img src="https://img.shields.io/badge/C-FFI-007ec6" alt="C">
  </a>
  <a href="https://docs.html-to-markdown.kreuzberg.dev">
    <img src="https://img.shields.io/badge/Docs-kreuzberg.dev-007ec6" alt="Documentation">
  </a>
  <a href="https://github.com/kreuzberg-dev/html-to-markdown/blob/main/LICENSE">
    <img src="https://img.shields.io/badge/License-MIT-blue.svg" alt="License">
  </a>
</div>

<img width="3384" height="573" alt="Banner" src="https://github.com/user-attachments/assets/478a83da-237b-446b-b3a8-e564c13e00a8" />

<div align="center" style="margin-top: 20px;">
  <a href="https://discord.gg/pXxagNK2zN">
      <img height="22" src="https://img.shields.io/badge/Discord-Join%20our%20community-7289da?logo=discord&logoColor=white" alt="Discord">
  </a>
</div>

High-performance HTML to Markdown conversion powered by Rust. Ships as native bindings for **Rust, Python, TypeScript/Node.js, Ruby, PHP, Go, Java, C#, Elixir, R, C (FFI), and WebAssembly** with identical rendering across all runtimes.

**[Documentation](https://docs.html-to-markdown.kreuzberg.dev)** | **[Live Demo](https://docs.html-to-markdown.kreuzberg.dev/demo/)** | **[API Reference](https://docs.html-to-markdown.kreuzberg.dev/reference/api-rust/)**

## Highlights

- **150-280 MB/s** throughput (10-80x faster than pure Python alternatives)
- **12 language bindings** with consistent output across all runtimes
- **Metadata extraction** — title, headers, links, images, structured data (JSON-LD, Microdata, RDFa)
- **Visitor pattern** — custom callbacks for content filtering, URL rewriting, domain-specific dialects
- **Table extraction** — extract structured table data (cells, headers, rendered markdown) during conversion
- **Secure by default** — built-in HTML sanitization via ammonia

## Quick Start

```bash
# Rust
cargo add html-to-markdown-rs

# Python
pip install html-to-markdown

# TypeScript / Node.js
npm install @kreuzberg/html-to-markdown-node

# Ruby
gem install html-to-markdown

# CLI
cargo install html-to-markdown-cli
# or
brew install kreuzberg-dev/tap/html-to-markdown
```

See the **[Installation Guide](https://docs.html-to-markdown.kreuzberg.dev/getting-started/installation/)** for all languages including PHP, Go, Java, C#, Elixir, R, and WASM.

## Part of the Kreuzberg Ecosystem

html-to-markdown is developed by [kreuzberg.dev](https://kreuzberg.dev) and powers the HTML conversion pipeline in [Kreuzberg](https://docs.kreuzberg.dev), a document intelligence library for extracting text from PDFs, images, and office documents.

## Contributing

Contributions welcome! See [CONTRIBUTING.md](CONTRIBUTING.md) for setup instructions and guidelines.

## License

MIT License — see [LICENSE](LICENSE) for details.
