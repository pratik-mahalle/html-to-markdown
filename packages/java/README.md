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
    <img src="https://img.shields.io/badge/Go-v2.24.6-007ec6" alt="Go">
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


High-performance HTML to Markdown converter with Java Panama FFI bindings to the Rust core.
Uses Foreign Function &amp; Memory API for zero-dependency, thread-safe conversion with full metadata extraction support.


## Installation

```bash
&lt;dependency&gt;
    &lt;groupId&gt;dev.kreuzberg&lt;/groupId&gt;
    &lt;artifactId&gt;html-to-markdown&lt;/artifactId&gt;
    &lt;version&gt;2.24.1&lt;/version&gt;
    &lt;classifier&gt;linux&lt;/classifier&gt; &lt;!-- or macos, windows --&gt;
&lt;/dependency&gt;

```



Requires Java 25+ with Panama FFI support.

**Maven:**
```xml
<dependency>
    <groupId>dev.kreuzberg</groupId>
    <artifactId>html-to-markdown</artifactId>
    <version>2.24.6</version>
</dependency>
```

**Gradle (Kotlin DSL):**
```kotlin
implementation("dev.kreuzberg:html-to-markdown:2.24.6")
```






## Performance Snapshot

Apple M4 • Real Wikipedia documents • `convert()` (Java)

| Document | Size | Ops/sec | Throughput |
| -------- | ---- | ------- | ---------- |
| Lists (Timeline) | 129KB | 2,308 | 291.5 MB/s |
| Tables (Countries) | 360KB | 773 | 272.0 MB/s |
| Mixed (Python) | 656KB | 403 | 258.5 MB/s |


See [Performance Guide](../../examples/performance/) for detailed benchmarks.


## Quick Start

Basic conversion:

```java
import dev.kreuzberg.htmltomarkdown.HtmlToMarkdown;

public class Example {
    public static void main(String[] args) {
        String html = &#34;&lt;h1&gt;Hello World&lt;/h1&gt;&lt;p&gt;This is a &lt;strong&gt;test&lt;/strong&gt;.&lt;/p&gt;&#34;;
        String markdown = HtmlToMarkdown.convert(html);
        System.out.println(markdown);
        // Output:
        // # Hello World
        //
        // This is a **test**.
    }
}
```



With conversion options:

```java
import dev.kreuzberg.htmltomarkdown.HtmlToMarkdown;
import dev.kreuzberg.htmltomarkdown.metadata.MetadataExtraction;

public class MetadataExample {
    public static void main(String[] args) {
        String html = &#34;&#34;&#34;
            &lt;html&gt;
            &lt;head&gt;
                &lt;title&gt;My Article&lt;/title&gt;
                &lt;meta name=&#34;description&#34; content=&#34;An interesting read&#34;&gt;
                &lt;meta name=&#34;author&#34; content=&#34;Jane Doe&#34;&gt;
                &lt;meta property=&#34;og:image&#34; content=&#34;image.jpg&#34;&gt;
            &lt;/head&gt;
            &lt;body&gt;
                &lt;h1&gt;Welcome&lt;/h1&gt;
                &lt;a href=&#34;https://example.com&#34;&gt;Link&lt;/a&gt;
                &lt;img src=&#34;image.jpg&#34; alt=&#34;Featured image&#34;&gt;
            &lt;/body&gt;
            &lt;/html&gt;
            &#34;&#34;&#34;;

        try {
            MetadataExtraction result = HtmlToMarkdown.convertWithMetadata(html);

            // Access document metadata
            var doc = result.metadata().document();
            if (doc.title() != null) {
                System.out.println(&#34;Title: &#34; + doc.title());
            }
            if (doc.author() != null) {
                System.out.println(&#34;Author: &#34; + doc.author());
            }

            // Access Open Graph metadata
            doc.openGraph().forEach((key, value) -&gt;
                System.out.println(&#34;OG &#34; + key + &#34;: &#34; + value)
            );

            // Count extracted elements
            System.out.println(&#34;Headers: &#34; + result.metadata().headers().size());
            System.out.println(&#34;Links: &#34; + result.metadata().links().size());
            System.out.println(&#34;Images: &#34; + result.metadata().images().size());

            // Print markdown output
            System.out.println(&#34;\nMarkdown:\n&#34; + result.markdown());
        } catch (HtmlToMarkdown.ConversionException e) {
            System.err.println(&#34;Conversion failed: &#34; + e.getMessage());
        }
    }
}
```






## API Reference

### Core Functions


**`convert(String html) : String`**
**`convert(String html, ConversionOptions options) : String`**

Basic HTML-to-Markdown conversion. Fast and simple.

**`convertWithMetadata(String html) : ConversionResult<MetadataResult>`**
**`convertWithMetadata(String html, ConversionOptions options, MetadataConfig config) : ConversionResult<MetadataResult>`**

Extract Markdown plus metadata in a single pass. See [Metadata Extraction Guide](../../examples/metadata-extraction/).

**`convertWithInlineImages(String html, InlineImageConfig config) : ConversionResult<InlineImagesResult>`**

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


```java
import dev.kreuzberg.HtmlToMarkdown;
import dev.kreuzberg.ConversionOptions;

String html = "<p>This is <strong>bold</strong> and <em>italic</em> text.</p>";

// Default Markdown output
String markdown = HtmlToMarkdown.convert(html);
// Result: "This is **bold** and *italic* text."

// Djot output
String djot = HtmlToMarkdown.convert(html,
    new ConversionOptions().setOutputFormat("djot"));
// Result: "This is *bold* and _italic_ text."
```


Djot's extended syntax allows you to express more semantic meaning in lightweight text, making it useful for documents that require strikethrough, insertion tracking, or mathematical notation.






## Examples

- [Visitor Pattern Guide](../../examples/visitor-pattern/)
- [Metadata Extraction Guide](../../examples/metadata-extraction/)
- [Performance Guide](../../examples/performance/)

## Links

- **GitHub:** [github.com/kreuzberg-dev/html-to-markdown](https://github.com/kreuzberg-dev/html-to-markdown)

- **Maven Central:** [central.sonatype.com/artifact/dev.kreuzberg/html-to-markdown](https://central.sonatype.com/artifact/dev.kreuzberg/html-to-markdown)

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
