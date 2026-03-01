---
title: TypeScript API Reference
description: API reference for the @kreuzberg/html-to-markdown-node npm package
---

# TypeScript API Reference <span class="version-badge">v2.3.0</span>

**Package:** [`@kreuzberg/html-to-markdown-node`](https://www.npmjs.com/package/@kreuzberg/html-to-markdown-node) | **Version:** 2.26.0 | **Node.js:** 18+

---

## Installation

```bash
npm install @kreuzberg/html-to-markdown-node
```

```bash
pnpm add @kreuzberg/html-to-markdown-node
```

---

## Functions

### `convert`

Convert HTML to Markdown. This is a synchronous function backed by NAPI-RS native bindings.

```typescript
function convert(html: string, options?: ConversionOptions): string
```

**Parameters:**

| Parameter | Type | Description |
|-----------|------|-------------|
| `html` | `string` | The HTML string to convert |
| `options` | `ConversionOptions?` | Optional conversion configuration |

**Returns:** `string` -- the converted Markdown.

**Throws:** `Error` if the conversion fails.

**Example:**

```typescript
import { convert } from "@kreuzberg/html-to-markdown-node";

const html = "<h1>Hello</h1><p>World</p>";
const markdown = convert(html);

// With options
const markdown = convert(html, {
  headingStyle: "atx",
  codeBlockStyle: "backticks",
  wrap: true,
  wrapWidth: 80,
});
```

---

### `convertWithMetadata`

Convert HTML to Markdown with metadata extraction.

```typescript
function convertWithMetadata(
  html: string,
  options?: ConversionOptions,
  metadataConfig?: MetadataConfig
): MetadataExtraction
```

**Parameters:**

| Parameter | Type | Description |
|-----------|------|-------------|
| `html` | `string` | The HTML string to convert |
| `options` | `ConversionOptions?` | Optional conversion configuration |
| `metadataConfig` | `MetadataConfig?` | Metadata extraction configuration |

**Returns:** `MetadataExtraction` -- object containing `markdown` and `metadata`.

**Example:**

```typescript
import { convertWithMetadata } from "@kreuzberg/html-to-markdown-node";

const html = `
  <html lang="en">
    <head><title>My Article</title></head>
    <body>
      <h1 id="intro">Introduction</h1>
      <p>Visit <a href="https://example.com">our site</a></p>
    </body>
  </html>
`;

const { markdown, metadata } = convertWithMetadata(html);
console.log(metadata.document.title);    // "My Article"
console.log(metadata.document.language); // "en"
console.log(metadata.headers.length);    // 1
console.log(metadata.links.length);      // 1

// Selective extraction
const { markdown, metadata } = convertWithMetadata(html, undefined, {
  extractHeaders: true,
  extractLinks: false,
  extractImages: false,
});
```

---

### `convertWithVisitor`

Convert HTML to Markdown with an async visitor. Requires the `async-visitor` build.

```typescript
function convertWithVisitor(
  html: string,
  visitor: Visitor,
  options?: ConversionOptions
): Promise<string>
```

**Parameters:**

| Parameter | Type | Description |
|-----------|------|-------------|
| `html` | `string` | The HTML string to convert |
| `visitor` | `Visitor` | Visitor object with callback methods |
| `options` | `ConversionOptions?` | Optional conversion configuration |

**Returns:** `Promise<string>` -- the converted Markdown.

**Example:**

```typescript
import { convertWithVisitor } from "@kreuzberg/html-to-markdown-node";

const visitor = {
  visitImage(ctx, src, alt, title) {
    return { type: "skip" };
  },
  visitLink(ctx, href, text, title) {
    return { type: "custom", output: `${text} (${href})` };
  },
};

const markdown = await convertWithVisitor(html, visitor);
```

---

### `convertWithOptionsHandle`

Convert using a pre-compiled options handle for batch conversions.

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

**Example:**

```typescript
import {
  createConversionOptionsHandle,
  convertWithOptionsHandle,
} from "@kreuzberg/html-to-markdown-node";

const handle = createConversionOptionsHandle({ headingStyle: "atx" });

for (const html of htmlDocuments) {
  const markdown = convertWithOptionsHandle(html, handle);
}
```

---

## Interfaces

### `ConversionOptions`

```typescript
interface ConversionOptions {
  headingStyle?: "underlined" | "atx" | "atxClosed";
  listIndentType?: "spaces" | "tabs";
  listIndentWidth?: number;
  bullets?: string;
  strongEmSymbol?: string;
  escapeAsterisks?: boolean;
  escapeUnderscores?: boolean;
  escapeMisc?: boolean;
  escapeAscii?: boolean;
  codeLanguage?: string;
  autolinks?: boolean;
  defaultTitle?: boolean;
  brInTables?: boolean;
  hocrSpatialTables?: boolean;
  highlightStyle?: "doubleEqual" | "html" | "bold" | "none";
  extractMetadata?: boolean;
  whitespaceMode?: "normalized" | "strict";
  stripNewlines?: boolean;
  wrap?: boolean;
  wrapWidth?: number;
  convertAsInline?: boolean;
  subSymbol?: string;
  supSymbol?: string;
  newlineStyle?: "spaces" | "backslash";
  codeBlockStyle?: "indented" | "backticks" | "tildes";
  keepInlineImagesIn?: string[];
  preprocessing?: PreprocessingOptions;
  encoding?: string;
  debug?: boolean;
  stripTags?: string[];
  preserveTags?: string[];
  skipImages?: boolean;
  outputFormat?: "markdown" | "djot" | "plain";
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

### `MetadataExtraction`

```typescript
interface MetadataExtraction {
  markdown: string;
  metadata: ExtendedMetadata;
}

interface ExtendedMetadata {
  document: DocumentMetadata;
  headers: HeaderMetadata[];
  links: LinkMetadata[];
  images: ImageMetadata[];
  structuredData: StructuredData[];
}
```

See the [Types Reference](types.md) for full type definitions.

---

### Visitor Interface

```typescript
interface Visitor {
  visitText?(ctx: NodeContext, text: string): VisitResult | Promise<VisitResult>;
  visitLink?(ctx: NodeContext, href: string, text: string, title?: string): VisitResult | Promise<VisitResult>;
  visitImage?(ctx: NodeContext, src: string, alt: string, title?: string): VisitResult | Promise<VisitResult>;
  visitHeading?(ctx: NodeContext, level: number, text: string, id?: string): VisitResult | Promise<VisitResult>;
  visitCodeBlock?(ctx: NodeContext, language?: string, code: string): VisitResult | Promise<VisitResult>;
  visitCodeInline?(ctx: NodeContext, code: string): VisitResult | Promise<VisitResult>;
  visitListItem?(ctx: NodeContext, ordered: boolean, marker: string, text: string): VisitResult | Promise<VisitResult>;
  visitTableRow?(ctx: NodeContext, cells: string[], isHeader: boolean): VisitResult | Promise<VisitResult>;
  visitBlockquote?(ctx: NodeContext, content: string, depth: number): VisitResult | Promise<VisitResult>;
  visitStrong?(ctx: NodeContext, text: string): VisitResult | Promise<VisitResult>;
  visitEmphasis?(ctx: NodeContext, text: string): VisitResult | Promise<VisitResult>;
  visitElementStart?(ctx: NodeContext): VisitResult | Promise<VisitResult>;
  visitElementEnd?(ctx: NodeContext, output: string): VisitResult | Promise<VisitResult>;
  // ... and more
}

type VisitResult =
  | { type: "continue" }
  | { type: "skip" }
  | { type: "preserveHtml" }
  | { type: "custom"; output: string }
  | { type: "error"; message: string };
```

---

## See Also

- [WASM API Reference](api-wasm.md) -- browser/edge runtime alternative
- [Configuration Reference](configuration.md) -- full options documentation
- [Types Reference](types.md) -- cross-language type definitions
