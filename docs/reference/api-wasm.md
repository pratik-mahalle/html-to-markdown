---
title: WASM API Reference
description: API reference for the @kreuzberg/html-to-markdown-wasm npm package
---

# WASM API Reference

**Package:** [`@kreuzberg/html-to-markdown-wasm`](https://www.npmjs.com/package/@kreuzberg/html-to-markdown-wasm) | **Version:** 2.26.0

The WASM binding runs in browsers, Node.js, Deno, and edge runtimes (Cloudflare Workers, Vercel Edge). No native compilation required.

---

## Installation

```bash
npm install @kreuzberg/html-to-markdown-wasm
```

---

## Functions

### `convert`

Convert HTML to Markdown.

```typescript
function convert(html: string, options?: ConversionOptions): string
```

**Parameters:**

| Parameter | Type | Description |
|-----------|------|-------------|
| `html` | `string` | The HTML string to convert |
| `options` | `ConversionOptions?` | Optional conversion configuration |

**Returns:** `string` -- the converted Markdown.

**Throws:** `Error` if conversion fails.

**Example:**

```typescript
import { convert } from "@kreuzberg/html-to-markdown-wasm";

const markdown = convert("<h1>Hello</h1><p>World</p>");

// With options
const markdown = convert(html, {
  headingStyle: "atx",
  codeBlockStyle: "backticks",
});
```

---

### `convertWithMetadata`

Convert HTML with metadata extraction.

```typescript
function convertWithMetadata(
  html: string,
  options?: ConversionOptions,
  metadataConfig?: MetadataConfig
): MetadataExtraction
```

**Returns:** `MetadataExtraction` with `markdown` and `metadata` properties.

**Example:**

```typescript
import { convertWithMetadata } from "@kreuzberg/html-to-markdown-wasm";

const { markdown, metadata } = convertWithMetadata(html);
console.log(metadata.document.title);
console.log(metadata.headers.length);
```

---

### `convertBytes`

Convert UTF-8 HTML bytes to Markdown (useful for binary data from fetch responses).

```typescript
function convertBytes(html: Uint8Array, options?: ConversionOptions): string
```

---

### `convertWithOptionsHandle`

Convert using a pre-compiled options handle.

```typescript
function convertWithOptionsHandle(
  html: string,
  handle: ConversionOptionsHandle
): string
```

### `createConversionOptionsHandle`

Create a reusable options handle.

```typescript
function createConversionOptionsHandle(
  options?: ConversionOptions
): ConversionOptionsHandle
```

---

## Browser Usage

### ES Module Import

```html
<script type="module">
  import init, { convert } from "@kreuzberg/html-to-markdown-wasm";

  // Initialize the WASM module (required once)
  await init();

  const html = "<h1>Hello</h1><p>World</p>";
  const markdown = convert(html);
  document.getElementById("output").textContent = markdown;
</script>
```

### With a Bundler (Vite, Webpack, etc.)

```typescript
import init, { convert } from "@kreuzberg/html-to-markdown-wasm";

async function setup() {
  await init();
  return convert;
}

const convertHtml = await setup();
const markdown = convertHtml("<p>Hello World</p>");
```

### Dynamic Import (Lazy Loading)

```typescript
async function convertHtml(html: string): Promise<string> {
  const { default: init, convert } = await import(
    "@kreuzberg/html-to-markdown-wasm"
  );
  await init();
  return convert(html);
}
```

---

## Node.js Usage

```typescript
import init, { convert } from "@kreuzberg/html-to-markdown-wasm";

await init();
const markdown = convert("<h1>Title</h1>");
console.log(markdown);
```

!!! tip "Node.js alternative"
    For Node.js, consider using [`@kreuzberg/html-to-markdown-node`](api-typescript.md) instead. The native NAPI-RS binding is significantly faster than WASM.

---

## Deno Usage

```typescript
import init, { convert } from "npm:@kreuzberg/html-to-markdown-wasm";

await init();
const markdown = convert("<h1>Title</h1>");
console.log(markdown);
```

---

## Cloudflare Workers / Edge Runtimes

```typescript
import init, { convert } from "@kreuzberg/html-to-markdown-wasm";

let initialized = false;

export default {
  async fetch(request: Request): Promise<Response> {
    if (!initialized) {
      await init();
      initialized = true;
    }

    const html = await request.text();
    const markdown = convert(html);
    return new Response(markdown, {
      headers: { "Content-Type": "text/plain" },
    });
  },
};
```

---

## Interfaces

### `ConversionOptions`

Same interface as the [TypeScript API](api-typescript.md). All fields are optional with camelCase naming.

```typescript
interface ConversionOptions {
  headingStyle?: "underlined" | "atx" | "atxClosed";
  codeBlockStyle?: "indented" | "backticks" | "tildes";
  wrap?: boolean;
  wrapWidth?: number;
  preserveTags?: string[];
  stripTags?: string[];
  skipImages?: boolean;
  outputFormat?: "markdown" | "djot";
  // ... see Configuration Reference for full list
}
```

### `MetadataConfig`

```typescript
interface MetadataConfig {
  extractDocument?: boolean;
  extractHeaders?: boolean;
  extractLinks?: boolean;
  extractImages?: boolean;
  extractStructuredData?: boolean;
  maxStructuredDataSize?: number;
}
```

---

## Bundle Size

The WASM binary is approximately **2.6 MB** uncompressed. With gzip compression (standard for HTTP), this reduces to approximately **800 KB**.

Tips for minimizing bundle impact:

- Use dynamic imports to lazy-load the WASM module
- The WASM binary is cached by the browser after first load
- Consider the native Node.js binding for server-side use

---

## Browser Compatibility

| Browser | Minimum Version |
|---------|-----------------|
| Chrome | 57+ |
| Firefox | 52+ |
| Safari | 11+ |
| Edge | 16+ |

Requires WebAssembly support. No IE11 support.

---

## Live Demo

Try the converter in your browser: [Live Demo](https://kreuzberg-dev.github.io/html-to-markdown/)

---

## See Also

- [TypeScript API Reference](api-typescript.md) -- native Node.js binding (faster for server-side)
- [Configuration Reference](configuration.md) -- full options documentation
- [Types Reference](types.md) -- cross-language type definitions
