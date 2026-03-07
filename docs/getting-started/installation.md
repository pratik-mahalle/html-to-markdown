---
title: Installation
description: Install html-to-markdown for Rust, Python, TypeScript, Ruby, PHP, Go, Java, C#, Elixir, R, C, WASM, or CLI
---

# Installation

html-to-markdown provides native bindings for 12 language ecosystems, plus a standalone CLI tool. Choose your platform below.

---

## Language Bindings

=== "Python"

    <span class="version-badge">v2.0.0</span>

    ```bash
    pip install html-to-markdown
    ```

    Or with [uv](https://docs.astral.sh/uv/):

    ```bash
    uv add html-to-markdown
    ```

    **Requirements:** Python 3.10+

    Pre-built wheels are available for Linux (x86_64, aarch64), macOS (x86_64, arm64), and Windows (x86_64). The package includes compiled Rust extensions via PyO3 -- no Rust toolchain needed.

    !!! info "Used by kreuzberg"
        The Python binding is the same one used internally by [kreuzberg](https://docs.kreuzberg.dev) for its HTML conversion pipeline. If you are already using kreuzberg for document processing, html-to-markdown is included as a dependency.

=== "TypeScript / Node.js"

    <span class="version-badge">v2.3.0</span>

    For **Node.js** and **Bun** (native NAPI-RS bindings, best performance):

    ```bash
    npm install @kreuzberg/html-to-markdown-node
    ```

    For **browsers**, **Deno**, and **Cloudflare Workers** (WebAssembly):

    ```bash
    npm install @kreuzberg/html-to-markdown-wasm
    ```

    Unified TypeScript package (auto-selects native or WASM based on environment):

    ```bash
    npm install @kreuzberg/html-to-markdown
    ```

    **Requirements:** Node.js 18+ or Bun 1.0+

    Pre-built native binaries are available for Linux (x86_64, aarch64), macOS (x86_64, arm64), and Windows (x86_64).

=== "Rust"

    <span class="version-badge">v2.0.0</span>

    ```bash
    cargo add html-to-markdown-rs
    ```

    Or add to your `Cargo.toml`:

    ```toml
    [dependencies]
    html-to-markdown-rs = "2.26"
    ```

    **Requirements:** Rust 1.80+ (2024 edition)

    The core Rust crate has no system dependencies and compiles on all major platforms.

=== "Ruby"

    <span class="version-badge">v2.5.1</span>

    ```bash
    gem install html-to-markdown
    ```

    Or add to your `Gemfile`:

    ```ruby
    gem 'html-to-markdown'
    ```

    **Requirements:** Ruby 3.2+

    Pre-built native extensions are included. The gem uses Magnus for Rust bindings.

=== "PHP"

    <span class="version-badge">v2.5.6</span>

    ```bash
    composer require kreuzberg-dev/html-to-markdown
    ```

    **Requirements:** PHP 8.4+

    The package includes a native Rust extension via ext-php-rs. PHPStan level 9 compatible.

=== "Go"

    <span class="version-badge">v2.8.0</span>

    ```bash
    go get github.com/kreuzberg-dev/html-to-markdown/packages/go/v2/htmltomarkdown
    ```

    **Requirements:** Go 1.21+

    The Go binding uses FFI to call the Rust core. The shared library is automatically downloaded and cached on first use. Set `HTML_TO_MARKDOWN_FFI_PATH` to provide a custom library path, or `HTML_TO_MARKDOWN_FFI_CACHE_DIR` to control the cache directory.

=== "Java"

    <span class="version-badge new">v2.24.2</span>

    Add to your `pom.xml`:

    ```xml
    <dependency>
        <groupId>dev.kreuzberg</groupId>
        <artifactId>html-to-markdown</artifactId>
        <version>2.28.1</version>
    </dependency>
    ```

    Or with Gradle:

    ```kotlin
    implementation("dev.kreuzberg:html-to-markdown:2.28.1")
    ```

    **Requirements:** Java 24+ (Panama Foreign Function & Memory API)

    The Java binding uses the Panama FFM API for zero-overhead FFI calls to the Rust core.

=== "C#"

    <span class="version-badge">v2.8.0</span>

    ```bash
    dotnet add package KreuzbergDev.HtmlToMarkdown
    ```

    Or via the NuGet Package Manager:

    ```powershell
    Install-Package KreuzbergDev.HtmlToMarkdown
    ```

    **Requirements:** .NET 8.0+

    The C# binding uses P/Invoke for FFI to the Rust core.

=== "Elixir"

    <span class="version-badge">v2.8.2</span>

    Add to your `mix.exs` dependencies:

    ```elixir
    defp deps do
      [
        {:html_to_markdown, "~> 2.26"}
      ]
    end
    ```

    Then fetch dependencies:

    ```bash
    mix deps.get
    ```

    **Requirements:** Elixir 1.19+, OTP 25+

    The Elixir binding uses Rustler for safe NIF bindings to the Rust core.

=== "R"

    <span class="version-badge new">v2.25.2</span>

    Install from r-universe:

    ```r
    install.packages("htmltomarkdown",
      repos = c("https://kreuzberg-dev.r-universe.dev",
                "https://cloud.r-project.org"))
    ```

    **Requirements:** R 4.3+

    The R binding uses extendr for Rust FFI.

=== "C (FFI)"

    <span class="version-badge new">v2.28.1</span>

    Download the pre-built shared library from the [GitHub Releases](https://github.com/kreuzberg-dev/html-to-markdown/releases) page, or build from source:

    ```bash
    cargo build --release -p html-to-markdown-ffi
    ```

    The C header file `html_to_markdown.h` is generated by cbindgen and included in the release artifacts. Link against `libhtml_to_markdown_ffi.so` (Linux), `libhtml_to_markdown_ffi.dylib` (macOS), or `html_to_markdown_ffi.dll` (Windows).

    **Requirements:** Any C compiler with C11 support

=== "WASM (Browser)"

    <span class="version-badge">v2.3.0</span>

    Install the npm package:

    ```bash
    npm install @kreuzberg/html-to-markdown-wasm
    ```

    Or use directly from a CDN:

    ```html
    <script type="module">
      import init, { convert } from 'https://unpkg.com/@kreuzberg/html-to-markdown-wasm/html_to_markdown_wasm.js';
      await init();
      const markdown = convert('<h1>Hello</h1>');
    </script>
    ```

    **Requirements:** Any modern browser with WebAssembly support (Chrome 89+, Firefox 79+, Safari 14.1+)

---

## CLI Tool

The command-line interface lets you convert HTML files without writing code.

=== "Cargo"

    ```bash
    cargo install html-to-markdown-cli
    ```

=== "Homebrew"

    ```bash
    brew install kreuzberg-dev/tap/html-to-markdown
    ```

=== "Pre-built Binary"

    Download platform-specific binaries from the [GitHub Releases](https://github.com/kreuzberg-dev/html-to-markdown/releases) page.

After installation, verify it works:

```bash
html-to-markdown --version
```

---

## Next Steps

Once you have html-to-markdown installed, head to the [Quick Start](quickstart.md) guide to convert your first HTML document.
