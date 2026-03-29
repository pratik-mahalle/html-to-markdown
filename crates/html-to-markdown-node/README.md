# @kreuzberg/html-to-markdown-node

> **npm package:** `@kreuzberg/html-to-markdown-node` (this README).
> Use [`@kreuzberg/html-to-markdown-wasm`](https://www.npmjs.com/package/@kreuzberg/html-to-markdown-wasm) for the portable WASM build.

Native Node.js and Bun bindings for html-to-markdown using NAPI-RS v3.

Built on the shared Rust engine that powers the Python wheels, Ruby gem, PHP extension, WebAssembly package, and CLI -- ensuring identical Markdown output across every language target.

High-performance HTML to Markdown conversion using native Rust code compiled to platform-specific binaries.

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

---

## Performance

Native NAPI-RS bindings deliver **the fastest HTML to Markdown conversion** available in JavaScript.

### Benchmark Results (Apple M4)

| Document Type              | ops/sec    | Notes              |
| -------------------------- | ---------- | ------------------ |
| **Small (5 paragraphs)**   | **86,233** | Simple documents   |
| **Medium (25 paragraphs)** | **18,979** | Nested formatting  |
| **Large (100 paragraphs)** | **4,907**  | Complex structures |
| **Tables (20 tables)**     | **5,003**  | Table processing   |
| **Lists (500 items)**      | **1,819**  | Nested lists       |
| **Wikipedia (129KB)**      | **1,125**  | Real-world content |
| **Wikipedia (653KB)**      | **156**    | Large documents    |

**Average: ~18,162 ops/sec** across varied workloads.

### Comparison

- **vs WASM**: ~1.17x faster (native has zero startup time, direct memory access)
- **vs Python**: ~7.4x faster (avoids FFI overhead)
- **Best for**: Node.js and Bun server-side applications requiring maximum throughput

### Benchmark Fixtures (Apple M4)

The shared benchmark harness lives in `tools/benchmark-harness`. Node keeps pace with the Rust CLI across the board:

| Document               | Size   | ops/sec (Node) |
| ---------------------- | ------ | -------------- |
| Lists (Timeline)       | 129 KB | 3,137          |
| Tables (Countries)     | 360 KB | 932            |
| Medium (Python)        | 657 KB | 460            |
| Large (Rust)           | 567 KB | 554            |
| Small (Intro)          | 463 KB | 627            |
| hOCR German PDF        | 44 KB  | 8,724          |
| hOCR Invoice           | 4 KB   | 96,138         |
| hOCR Embedded Tables   | 37 KB  | 9,591          |

> Run `task bench:harness -- --frameworks node` to regenerate these numbers.

## Installation

### Node.js

```bash
npm install @kreuzberg/html-to-markdown-node
# or
yarn add @kreuzberg/html-to-markdown-node
# or
pnpm add @kreuzberg/html-to-markdown-node
```

### Bun

```bash
bun add @kreuzberg/html-to-markdown-node
```

## Usage

### Basic Conversion

```javascript
import { convert } from '@kreuzberg/html-to-markdown-node';

const html = '<h1>Hello World</h1><p>This is <strong>fast</strong>!</p>';
const result = convert(html);

console.log(result.content);
// # Hello World
//
// This is **fast**!
```

### ConversionResult Fields

Every call to `convert()` returns a `ConversionResult` object with six fields:

```typescript
import { convert } from '@kreuzberg/html-to-markdown-node';

const result = convert(html);

result.content;   // string | null  -- converted Markdown (or djot/plain text)
result.document;  // string | null  -- structured document tree as JSON
result.metadata;  // string | null  -- extracted HTML metadata as JSON
result.tables;    // Array          -- all tables found in document order
result.images;    // Array          -- extracted inline images (data URIs, SVGs)
result.warnings;  // Array          -- non-fatal processing warnings
```

### With Options

```typescript
import { convert } from '@kreuzberg/html-to-markdown-node';

const result = convert(html, {
  headingStyle: 'Atx',
  codeBlockStyle: 'Backticks',
  listIndentWidth: 2,
  bullets: '-',
  wrap: true,
  wrapWidth: 80
});

console.log(result.content);
```

### Preserve Complex HTML

```typescript
import { convert } from '@kreuzberg/html-to-markdown-node';

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
// # Report
//
// <table>
//   <tr><th>Name</th><th>Value</th></tr>
//   <tr><td>Foo</td><td>Bar</td></tr>
// </table>
```

## TypeScript

Full TypeScript definitions included:

```typescript
import { convert, type JsConversionOptions } from '@kreuzberg/html-to-markdown-node';

const options: JsConversionOptions = {
  headingStyle: 'Atx',
  codeBlockStyle: 'Backticks',
  listIndentWidth: 2,
  bullets: '-',
  wrap: true,
  wrapWidth: 80
};

const result = convert('<h1>Hello</h1>', options);
console.log(result.content);
```

## Metadata and Tables

Extract metadata and structured tables from the conversion result:

```typescript
import { convert } from '@kreuzberg/html-to-markdown-node';

const html = `
<html lang="en">
  <head><title>My Article</title></head>
  <body>
    <h1>Main Title</h1>
    <p>Content with <a href="https://example.com">a link</a></p>
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
console.log(result.warnings);           // Any processing warnings

for (const table of result.tables) {
  console.log(table.markdown);          // Table as Markdown
  console.log(table.grid.rows);         // Number of rows
  console.log(table.grid.cols);         // Number of columns
  console.log(table.grid.cells);        // Cell-level data
}
```

## Inline Images

Inline images (data URIs, SVGs) are automatically extracted and available on the result:

```typescript
import { convert } from '@kreuzberg/html-to-markdown-node';
import { writeFileSync } from 'node:fs';

const html = '<img src="data:image/png;base64,iVBORw0..." alt="Logo">';

const result = convert(html);

console.log(result.content);
console.log(`Extracted ${result.images.length} images`);

for (const img of result.images) {
  console.log(`${img.filename}: ${img.format}, ${img.data.length} bytes`);
  writeFileSync(img.filename, img.data);
}
```

## Supported Platforms

Pre-built native binaries are provided for:

| Platform    | Architectures                                       |
| ----------- | --------------------------------------------------- |
| **macOS**   | x64 (Intel), ARM64 (Apple Silicon)                  |
| **Linux**   | x64 (glibc/musl), ARM64 (glibc/musl), ARMv7 (glibc) |
| **Windows** | x64, ARM64                                          |

### Runtime Compatibility

- **Node.js** 18+ (LTS)
- **Bun** 1.0+ (full NAPI-RS support)
- **Deno** -- use [@kreuzberg/html-to-markdown-wasm](https://www.npmjs.com/package/@kreuzberg/html-to-markdown-wasm) instead

## When to Use

Choose `@kreuzberg/html-to-markdown-node` when:

- Running in Node.js or Bun
- Maximum performance is required
- Server-side conversion at scale

Use [`@kreuzberg/html-to-markdown-wasm`](https://www.npmjs.com/package/@kreuzberg/html-to-markdown-wasm) for:

- Browser/client-side conversion
- Deno runtime
- Edge runtimes (Cloudflare Workers, Deno Deploy)
- Universal packages

Other runtimes:

- Python: [`html-to-markdown`](https://pypi.org/project/html-to-markdown/)
- Ruby: [`html-to-markdown`](https://rubygems.org/gems/html-to-markdown)
- PHP: [`kreuzberg-dev/html-to-markdown`](https://packagist.org/packages/kreuzberg-dev/html-to-markdown)
- WebAssembly: [`@kreuzberg/html-to-markdown-wasm`](https://www.npmjs.com/package/@kreuzberg/html-to-markdown-wasm)

## Configuration Options

See [ConversionOptions](https://github.com/kreuzberg-dev/html-to-markdown/tree/main/crates/html-to-markdown-node#types) for all available options including:

- Heading styles (ATX, underlined, ATX closed)
- Code block styles (indented, backticks, tildes)
- List formatting (indent width, bullet characters)
- Text escaping and formatting
- Tag preservation (`preserveTags`) and stripping (`stripTags`)
- Preprocessing for web scraping
- Metadata extraction (`extractMetadata`)
- hOCR table extraction
- And more...

## Examples

### Preserving HTML Tags

Keep specific HTML tags in their original form instead of converting to Markdown:

```typescript
import { convert } from '@kreuzberg/html-to-markdown-node';

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

console.log(result.content);
// "Before table\n\n<table class=\"data\">...</table>\n\nAfter table\n"
```

Combine with `stripTags` for fine-grained control:

```typescript
const result = convert(html, {
  preserveTags: ['table', 'form'],  // Keep these as HTML
  stripTags: ['script', 'style']    // Remove these entirely
});
```

### Web Scraping

```javascript
import { convert } from '@kreuzberg/html-to-markdown-node';

const scrapedHtml = await fetch('https://example.com').then(r => r.text());

const result = convert(scrapedHtml, {
  preprocessing: {
    enabled: true,
    preset: 'Aggressive',
    removeNavigation: true,
    removeForms: true
  },
  headingStyle: 'Atx',
  codeBlockStyle: 'Backticks'
});

console.log(result.content);
```

### hOCR Document Processing

```javascript
import { convert } from '@kreuzberg/html-to-markdown-node';
import { readFileSync } from 'node:fs';

// OCR output from Tesseract in hOCR format
const hocrHtml = readFileSync('scan.hocr', 'utf8');

// Automatically detects hOCR and reconstructs tables
const result = convert(hocrHtml, {
  hocrSpatialTables: true  // Enable spatial table reconstruction
});

console.log(result.content);
console.log(`Found ${result.tables.length} tables`);
```

## Links

- [GitHub Repository](https://github.com/kreuzberg-dev/html-to-markdown)
- [Full Documentation](https://github.com/kreuzberg-dev/html-to-markdown/blob/main/README.md)
- [WASM Package](https://www.npmjs.com/package/@kreuzberg/html-to-markdown-wasm)
- [Python Package](https://pypi.org/project/html-to-markdown/)
- [Rust Crate](https://crates.io/crates/html-to-markdown-rs)

## License

MIT
