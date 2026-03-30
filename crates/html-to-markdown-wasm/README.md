# @kreuzberg/html-to-markdown-wasm

> **npm package:** `@kreuzberg/html-to-markdown-wasm` (this README).
> Use [`@kreuzberg/html-to-markdown-node`](https://www.npmjs.com/package/@kreuzberg/html-to-markdown-node) when you only target Node.js or Bun and want native performance.

Universal HTML to Markdown converter using WebAssembly.

Powered by the same Rust engine as the Node.js, Python, Ruby, and PHP bindings, so Markdown output stays identical regardless of runtime.

Runs anywhere: Node.js, Deno, Bun, browsers, and edge runtimes.

[![Crates.io](https://img.shields.io/crates/v/html-to-markdown-rs.svg?logo=rust&label=crates.io)](https://crates.io/crates/html-to-markdown-rs)
[![npm (node)](https://img.shields.io/npm/v/%40kreuzberg%2Fhtml-to-markdown-node.svg?logo=npm)](https://www.npmjs.com/package/@kreuzberg/html-to-markdown-node)
[![npm (wasm)](https://img.shields.io/npm/v/%40kreuzberg%2Fhtml-to-markdown-wasm.svg?logo=npm)](https://www.npmjs.com/package/@kreuzberg/html-to-markdown-wasm)
[![PyPI](https://img.shields.io/pypi/v/html-to-markdown.svg?logo=pypi)](https://pypi.org/project/html-to-markdown/)
[![Packagist](https://img.shields.io/packagist/v/kreuzberg-dev/html-to-markdown.svg)](https://packagist.org/packages/kreuzberg-dev/html-to-markdown)
[![RubyGems](https://badge.fury.io/rb/html-to-markdown.svg)](https://rubygems.org/gems/html-to-markdown)
[![NuGet](https://img.shields.io/nuget/v/KreuzbergDev.HtmlToMarkdown.svg)](https://www.nuget.org/packages/KreuzbergDev.HtmlToMarkdown/)
[![Maven Central](https://img.shields.io/maven-central/v/dev.kreuzberg/html-to-markdown.svg)](https://central.sonatype.com/artifact/dev.kreuzberg/html-to-markdown)
[![Go Reference](https://pkg.go.dev/badge/github.com/kreuzberg-dev/html-to-markdown/packages/go/v3/htmltomarkdown.svg)](https://pkg.go.dev/github.com/kreuzberg-dev/html-to-markdown/packages/go/v3/htmltomarkdown)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://github.com/kreuzberg-dev/html-to-markdown/blob/main/LICENSE)

## Performance

Universal WebAssembly bindings with **excellent performance** across all JavaScript runtimes.

### Benchmark Results (Apple M4)

| Document Type              | ops/sec    | Notes              |
| -------------------------- | ---------- | ------------------ |
| **Small (5 paragraphs)**   | **70,300** | Simple documents   |
| **Medium (25 paragraphs)** | **15,282** | Nested formatting  |
| **Large (100 paragraphs)** | **3,836**  | Complex structures |
| **Tables (20 tables)**     | **3,748**  | Table processing   |
| **Lists (500 items)**      | **1,391**  | Nested lists       |
| **Wikipedia (129KB)**      | **1,022**  | Real-world content |
| **Wikipedia (653KB)**      | **147**    | Large documents    |

**Average: ~15,536 ops/sec** across varied workloads.

### Comparison

- **vs Native NAPI**: ~1.17x slower (WASM has minimal overhead)
- **vs Python**: ~6.3x faster (no FFI overhead)
- **Best for**: Universal deployment (browsers, Deno, edge runtimes, cross-platform apps)

### Benchmark Fixtures (Apple M4)

Numbers captured via the shared fixture harness in `tools/benchmark-harness`:

| Document               | Size   | ops/sec (WASM) |
| ---------------------- | ------ | -------------- |
| Lists (Timeline)       | 129 KB | 882            |
| Tables (Countries)     | 360 KB | 242            |
| Medium (Python)        | 657 KB | 121            |
| Large (Rust)           | 567 KB | 124            |
| Small (Intro)          | 463 KB | 163            |

> Expect slightly higher numbers in long-lived browser/Deno workers once the WASM module is warm.

## Installation

### npm / Yarn / pnpm

```bash
npm install @kreuzberg/html-to-markdown-wasm
# or
yarn add @kreuzberg/html-to-markdown-wasm
# or
pnpm add @kreuzberg/html-to-markdown-wasm
```

### Deno

```typescript
// Via npm specifier
import { convert } from "npm:@kreuzberg/html-to-markdown-wasm";
```

## Usage

### Basic Conversion

```javascript
import { convert } from '@kreuzberg/html-to-markdown-wasm';

const html = '<h1>Hello World</h1><p>This is <strong>fast</strong>!</p>';
const result = convert(html);
console.log(result.content);
// # Hello World
//
// This is **fast**!
```

> **Heads up for edge runtimes:** Cloudflare Workers, Vite dev servers, and other environments that instantiate `.wasm` files asynchronously must call `await initWasm()` (or `await wasmReady`) once during startup before invoking `convert`. Traditional bundlers (Webpack, Rollup) and Deno/Node imports continue to work without manual initialization.

### WasmConversionResult Fields

Every call to `convert()` returns a `WasmConversionResult` object with six fields:

```typescript
import { convert } from '@kreuzberg/html-to-markdown-wasm';

const result = convert(html);

result.content;   // string | null  -- converted Markdown (or djot/plain text)
result.document;  // string | null  -- structured document tree as JSON
result.metadata;  // string | null  -- extracted HTML metadata as JSON
result.tables;    // Array          -- all tables found in document order
result.images;    // Array          -- extracted inline images (data URIs, SVGs)
result.warnings;  // Array          -- non-fatal processing warnings
```

### Byte-Based Input (Buffers / Uint8Array)

When you already have raw bytes (e.g., `fs.readFileSync`, Fetch API responses), skip re-encoding with `TextDecoder` by calling the byte-friendly helper:

```ts
import { convertBytes } from '@kreuzberg/html-to-markdown-wasm';
import { readFileSync } from 'node:fs';

const htmlBytes = readFileSync('input.html'); // Buffer -> Uint8Array
const result = convertBytes(htmlBytes);
console.log(result.content);
```

### With Options

```typescript
import { convert } from '@kreuzberg/html-to-markdown-wasm';

const result = convert(html, {
  headingStyle: 'atx',
  codeBlockStyle: 'backticks',
  listIndentWidth: 2,
  bullets: '-',
  wrap: true,
  wrapWidth: 80
});
console.log(result.content);
```

### Preserve Complex HTML

```typescript
import { convert } from '@kreuzberg/html-to-markdown-wasm';

const html = `
<h1>Report</h1>
<table>
  <tr><th>Name</th><th>Value</th></tr>
  <tr><td>Foo</td><td>Bar</td></tr>
</table>
`;

const result = convert(html, {
  preserveTags: ['table'] // Keep tables as HTML
});
console.log(result.content);
```

### Deno

```typescript
import { convert } from "npm:@kreuzberg/html-to-markdown-wasm";

const html = await Deno.readTextFile("input.html");
const result = convert(html, { headingStyle: "atx" });
await Deno.writeTextFile("output.md", result.content ?? "");
```

> **Performance Tip:** For Node.js/Bun, use [@kreuzberg/html-to-markdown-node](https://www.npmjs.com/package/@kreuzberg/html-to-markdown-node) for 1.17x better performance with native bindings.

### Browser (ESM)

```html
<!DOCTYPE html>
<html>
<head>
  <title>HTML to Markdown</title>
</head>
<body>
  <script type="module">
    import init, { convert } from 'https://unpkg.com/@kreuzberg/html-to-markdown-wasm/dist-web/html_to_markdown_wasm.js';

    // Initialize WASM module
    await init();

    const html = '<h1>Hello World</h1><p>This runs in the <strong>browser</strong>!</p>';
    const result = convert(html, { headingStyle: 'atx' });

    console.log(result.content);
    document.body.innerHTML = `<pre>${result.content}</pre>`;
  </script>
</body>
</html>
```

### Vite / Webpack / Bundlers

```typescript
import { convert } from '@kreuzberg/html-to-markdown-wasm';

const result = convert('<h1>Hello</h1>', {
  headingStyle: 'atx',
  codeBlockStyle: 'backticks'
});
console.log(result.content);
```

### Cloudflare Workers

```typescript
import { convert, initWasm, wasmReady } from '@kreuzberg/html-to-markdown-wasm';

// Cloudflare Workers / other edge runtimes instantiate WASM asynchronously.
// Kick off initialization once at module scope.
const ready = wasmReady ?? initWasm();

export default {
  async fetch(request: Request): Promise<Response> {
    await ready;
    const html = await request.text();
    const result = convert(html, { headingStyle: 'atx' });

    return new Response(result.content ?? "", {
      headers: { 'Content-Type': 'text/markdown' }
    });
  }
};
```


## TypeScript

Full TypeScript support with type definitions:

```typescript
import { convert, type WasmConversionOptions } from '@kreuzberg/html-to-markdown-wasm';

const options: WasmConversionOptions = {
  headingStyle: 'atx',
  codeBlockStyle: 'backticks',
  listIndentWidth: 2,
  wrap: true,
  wrapWidth: 80
};

const result = convert('<h1>Hello</h1>', options);
console.log(result.content);
```

## Metadata and Tables

Extract document metadata and structured tables from the conversion result:

```typescript
import { convert } from '@kreuzberg/html-to-markdown-wasm';

const html = `
  <html lang="en">
    <head><title>My Article</title></head>
    <body>
      <h1>Main Title</h1>
      <p>Content with <a href="https://example.com">a link</a></p>
      <img src="https://example.com/image.jpg" alt="Example image">
      <table>
        <tr><th>Name</th><th>Value</th></tr>
        <tr><td>Foo</td><td>42</td></tr>
      </table>
    </body>
  </html>
`;

const result = convert(html, {
  extractMetadata: true,
});

console.log(result.content);            // Markdown output
console.log(result.metadata);           // JSON string with title, links, headers, etc.
console.log(result.tables.length);      // Number of tables found
console.log(result.images.length);      // Number of inline images extracted
console.log(result.warnings);           // Any processing warnings
```

## Build Targets

Three build targets are provided for different environments:

| Target      | Path                              | Use Case                       |
| ----------- | --------------------------------- | ------------------------------ |
| **Bundler** | `@kreuzberg/html-to-markdown-wasm`           | Webpack, Vite, Rollup, esbuild |
| **Node.js** | `@kreuzberg/html-to-markdown-wasm/dist-node` | Node.js, Bun (CommonJS/ESM)    |
| **Web**     | `@kreuzberg/html-to-markdown-wasm/dist-web`  | Direct browser ESM imports     |

## Runtime Compatibility

| Runtime                   | Support                      | Package        |
| ------------------------- | ---------------------------- | -------------- |
| **Node.js** 18+           | Full support                 | `dist-node`    |
| **Deno**                  | Full support                 | npm: specifier |
| **Bun**                   | Full support (prefer native) | Default export |
| **Browsers**              | Full support                 | `dist-web`     |
| **Cloudflare Workers**    | Full support                 | Default export |
| **Deno Deploy**           | Full support                 | npm: specifier |

## When to Use

Choose `@kreuzberg/html-to-markdown-wasm` when:

- Running in browsers or edge runtimes
- Using Deno
- Deploying to Cloudflare Workers, Deno Deploy
- Building universal libraries
- Need consistent behavior across all platforms

Use [@kreuzberg/html-to-markdown-node](https://www.npmjs.com/package/@kreuzberg/html-to-markdown-node) for:

- Maximum performance in Node.js/Bun (~3x faster)
- Server-side only applications

## Visitor Pattern Support

**The WebAssembly binding does not support the visitor pattern.** The visitor pattern requires callbacks and stateful execution across the WebAssembly/JavaScript boundary, which has fundamental limitations:

### Why WASM Does Not Support Visitors

1. **Memory safety across FFI boundary**: The WASM/JS boundary cannot safely pass mutable function callbacks that maintain state across multiple invocations
2. **Single-threaded execution model**: WASM runs on a single thread with no equivalent to Node.js's `ThreadsafeFunction` FFI primitive
3. **No callback marshaling**: JavaScript callbacks cannot be directly invoked from within WASM without significant overhead and memory leaks
4. **Serialization overhead**: Converting context objects between WASM and JS for each visitor callback would eliminate performance benefits

### Alternatives for WASM Users

Choose one of these approaches:

#### 1. Use Node.js Binding (Recommended)

For best performance with visitor support, use the native Node.js binding:

```typescript
import { convert, type Visitor } from '@kreuzberg/html-to-markdown-node';

const visitor: Visitor = {
  visitLink(ctx, href, text, title) {
    // Your visitor logic here
    return { type: 'continue' };
  },
};

const result = convert(html, undefined, visitor);
console.log(result.content);
```

**Performance:** ~3x faster than WASM, full visitor pattern support.
**Use when:** Running on Node.js or Bun server-side.

#### 2. Use Server-Side Bindings

For other platforms, use Python, Ruby, or PHP bindings with visitor support:

**Python:**

```python
from html_to_markdown import convert

class MyVisitor:
    def visit_link(self, ctx, href, text, title):
        # Your visitor logic here
        return {"type": "continue"}

result = convert(html, None, MyVisitor())
```

**Ruby:**

```ruby
require 'html_to_markdown'

class MyVisitor
  def visit_link(ctx, href, text, title)
    { type: :continue }
  end
end

result = HtmlToMarkdown.convert(html, nil, MyVisitor.new)
```

**PHP:**

```php
use HtmlToMarkdown\Converter;

class MyVisitor {
    public function visitLink(array $ctx, string $href, string $text, ?string $title): array {
        return ['type' => 'continue'];
    }
}

$result = Converter::convert($html, null, new MyVisitor());
```

#### 3. Preprocess HTML Before Conversion

For simple transformations, manipulate the HTML before passing to WASM:

```typescript
import { convert } from '@kreuzberg/html-to-markdown-wasm';

// Rewrite URLs before conversion
const processedHtml = html.replace(
  /https:\/\/old-cdn\.com/g,
  'https://new-cdn.com'
);

const result = convert(processedHtml);
console.log(result.content);
```

**Use when:** Only simple text replacements are needed.

#### 4. Post-Process Markdown

Transform the output Markdown after conversion:

```typescript
import { convert } from '@kreuzberg/html-to-markdown-wasm';

const markdown = convert(html).content ?? "";

// Post-process the markdown
const transformed = markdown
  .replace(/\[(.+?)\]\(https:\/\/old-cdn\.com/g, '[$1](https://new-cdn.com')
  .replace(/!\[(.+?)\]\(https:\/\/old-cdn\.com/g, '![$1](https://new-cdn.com');
```

**Use when:** Transformations can be applied to final Markdown output.

### Visitor Pattern Support Matrix

| Binding | Visitor Support | Best For |
|---------|-----------------|----------|
| **Rust** | Yes | Core library, performance-critical code |
| **Python** | Yes (sync and async) | Server-side, bulk processing |
| **TypeScript/Node.js** | Yes (sync and async) | Server-side Node.js/Bun, best performance |
| **Ruby** | Yes | Server-side Ruby on Rails, Sinatra |
| **PHP** | Yes | Server-side PHP, content management |
| **Go** | No | Basic conversion only |
| **Java** | No | Basic conversion only |
| **C#** | No | Basic conversion only |
| **Elixir** | No | Basic conversion only |
| **WebAssembly** | No | Browser, Edge, Deno (see alternatives above) |

For comprehensive visitor pattern documentation with examples, see the [full documentation](https://docs.html-to-markdown.kreuzberg.dev).

## Configuration Options

See the [TypeScript definitions](./dist-node/html_to_markdown_wasm.d.ts) for all available options:

- Heading styles (atx, underlined, atxClosed)
- Code block styles (indented, backticks, tildes)
- List formatting (indent width, bullet characters)
- Text escaping and formatting
- Tag preservation (`preserveTags`) and stripping (`stripTags`)
- Metadata extraction (`extractMetadata`)
- Preprocessing for web scraping
- And more...

## Examples

### Preserving HTML Tags

Keep specific HTML tags in their original form:

```typescript
import { convert } from '@kreuzberg/html-to-markdown-wasm';

const html = `
<p>Before table</p>
<table class="data">
    <tr><th>Name</th><th>Value</th></tr>
    <tr><td>Item 1</td><td>100</td></tr>
</table>
<p>After table</p>
`;

const result = convert(html, {
  preserveTags: ['table']
});

// result.content includes the table as HTML
```

Combine with `stripTags`:

```typescript
const result = convert(html, {
  preserveTags: ['table', 'form'],  // Keep as HTML
  stripTags: ['script', 'style']    // Remove entirely
});
console.log(result.content);
```

### Deno Web Server

```typescript
import { convert } from "npm:@kreuzberg/html-to-markdown-wasm";

Deno.serve(async (req) => {
  const url = new URL(req.url);

  if (url.pathname === "/convert" && req.method === "POST") {
    const html = await req.text();
    const result = convert(html, { headingStyle: "atx" });

    return new Response(result.content ?? "", {
      headers: { "Content-Type": "text/markdown" }
    });
  }

  return new Response("Not found", { status: 404 });
});
```

### Browser File Conversion

```html
<input type="file" id="htmlFile" accept=".html">
<button onclick="convertFile()">Convert to Markdown</button>
<pre id="output"></pre>

<script type="module">
  import init, { convert } from 'https://unpkg.com/@kreuzberg/html-to-markdown-wasm/dist-web/html_to_markdown_wasm.js';

  await init();

  window.convertFile = async () => {
    const file = document.getElementById('htmlFile').files[0];
    const html = await file.text();
    const result = convert(html, { headingStyle: 'atx' });
    document.getElementById('output').textContent = result.content ?? "";
  };
</script>
```

### Web Scraping (Deno)

```typescript
import { convert } from "npm:@kreuzberg/html-to-markdown-wasm";

const response = await fetch("https://example.com");
const html = await response.text();

const result = convert(html, {
  preprocess: true,
  preset: "aggressive",
  keepNavigation: false,
  headingStyle: "atx",
  codeBlockStyle: "backticks"
});

console.log(result.content);
```

## Other Runtimes

The same Rust engine ships as native bindings for other ecosystems:

- Node.js / Bun: [`@kreuzberg/html-to-markdown-node`](https://www.npmjs.com/package/@kreuzberg/html-to-markdown-node)
- Python: [`html-to-markdown`](https://pypi.org/project/html-to-markdown/)
- Ruby: [`html-to-markdown`](https://rubygems.org/gems/html-to-markdown)
- PHP: [`kreuzberg-dev/html-to-markdown`](https://packagist.org/packages/kreuzberg-dev/html-to-markdown)
- Rust crate and CLI: [`html-to-markdown-rs`](https://crates.io/crates/html-to-markdown-rs)

## Links

- [GitHub Repository](https://github.com/kreuzberg-dev/html-to-markdown)
- [Full Documentation](https://github.com/kreuzberg-dev/html-to-markdown/blob/main/README.md)
- [Native Node Package](https://www.npmjs.com/package/@kreuzberg/html-to-markdown-node)
- [Python Package](https://pypi.org/project/html-to-markdown/)
- [PHP Extension and Helpers](https://packagist.org/packages/kreuzberg-dev/html-to-markdown)
- [Rust Crate](https://crates.io/crates/html-to-markdown-rs)

## License

MIT
