# TypeScript / Node.js API Reference

Package: `@kreuzberg/html-to-markdown`
The TypeScript package re-exports everything from `@kreuzberg/html-to-markdown-node` (the native NAPI-RS binding) and adds file/stream helpers.

## Installation

```bash
npm install @kreuzberg/html-to-markdown
# or
pnpm add @kreuzberg/html-to-markdown
```

## Primary Function

```typescript
import { convert } from '@kreuzberg/html-to-markdown';

// convert() returns a JSON string — always JSON.parse() the result
const result = JSON.parse(convert(html, options?));
console.log(result.content);    // Markdown string or null
console.log(result.tables);     // array of table objects
console.log(result.warnings);   // array of warning objects
console.log(result.metadata);   // metadata object or null
```

**Important:** `convert()` returns a JSON-encoded string, not a parsed object. This is intentional — NAPI-RS serialization of deeply-nested objects is expensive. Always call `JSON.parse()` on the result.

## Function Signatures

### Core (from `@kreuzberg/html-to-markdown-node`)

```typescript
// Primary conversion — returns JSON string, always JSON.parse() the result
function convert(html: string, options?: JsConversionOptions): string;

// Convert Buffer/Uint8Array (avoids intermediate JS string)
function convertBuffer(html: Buffer, options?: JsConversionOptions): string;

// Pre-parsed options handle (reuse for many conversions)
function createConversionOptionsHandle(options?: JsConversionOptions): External<RustConversionOptions>;
function convertWithOptionsHandle(html: string, handle: External<RustConversionOptions>): string;
function convertBufferWithOptionsHandle(html: Buffer, handle: External<RustConversionOptions>): string;
```

### File and Stream Helpers (from `@kreuzberg/html-to-markdown`)

```typescript
import {
    convertFile,
    convertStream,
    wrapVisitorCallback,
    wrapVisitorCallbacks,
    hasMetadataSupport,
} from '@kreuzberg/html-to-markdown';
import type { Readable } from 'node:stream';

// File helpers (async, return JSON string — JSON.parse() the result)
async function convertFile(filePath: string, options?: JsConversionOptions | null): Promise<string>;

// Stream helpers (async, return JSON string — JSON.parse() the result)
async function convertStream(stream: Readable | AsyncIterable<string | Buffer>, options?: JsConversionOptions | null): Promise<string>;
```

## Interfaces

### JsConversionOptions

All fields are optional. Defaults match Rust defaults. Enum values are PascalCase strings (e.g. `'Atx'`, `'Spaces'`).

```typescript
interface JsConversionOptions {
    headingStyle?: 'Atx' | 'Underlined' | 'AtxClosed';
    listIndentType?: 'Spaces' | 'Tabs';
    listIndentWidth?: number;
    bullets?: string;
    strongEmSymbol?: string;          // '*' or '_'
    escapeAsterisks?: boolean;
    escapeUnderscores?: boolean;
    escapeMisc?: boolean;
    escapeAscii?: boolean;
    codeLanguage?: string;
    autolinks?: boolean;
    defaultTitle?: boolean;
    brInTables?: boolean;
    highlightStyle?: 'DoubleEqual' | 'Html' | 'Bold' | 'None';
    extractMetadata?: boolean;
    whitespaceMode?: 'Normalized' | 'Strict';
    stripNewlines?: boolean;
    wrap?: boolean;
    wrapWidth?: number;
    convertAsInline?: boolean;
    subSymbol?: string;
    supSymbol?: string;
    newlineStyle?: 'Spaces' | 'Backslash';
    codeBlockStyle?: 'Indented' | 'Backticks' | 'Tildes';
    keepInlineImagesIn?: string[];
    preprocessing?: JsPreprocessingOptions;
    encoding?: string;
    debug?: boolean;
    stripTags?: string[];
    preserveTags?: string[];
    skipImages?: boolean;
    outputFormat?: 'Markdown' | 'Djot' | 'Plain';
}
```

**Note on enum values:** NAPI-RS `const enum` values are PascalCase strings (e.g. `'Atx'` not `'atx'`, `'Spaces'` not `'spaces'`). Using lowercase will be rejected at runtime.

### JsPreprocessingOptions

```typescript
interface JsPreprocessingOptions {
    enabled?: boolean;
    preset?: 'minimal' | 'standard' | 'aggressive';
    removeNavigation?: boolean;
    removeForms?: boolean;
}
```

### JsMetadataConfig

Fields use `snake_case` (matching the actual `.d.ts`):

```typescript
interface JsMetadataConfig {
    extract_document?: boolean;
    extract_headers?: boolean;
    extract_links?: boolean;
    extract_images?: boolean;
    extract_structured_data?: boolean;
    max_structured_data_size?: number;
}
```

### JsInlineImageConfig

```typescript
interface JsInlineImageConfig {
    maxDecodedSizeBytes?: bigint;   // BigInt
    filenamePrefix?: string | null;
    captureSvg?: boolean;
    inferDimensions?: boolean;
}
```

### JsInlineImage (in JsHtmlExtraction.inlineImages)

Inline images are extracted via `convertWithInlineImages()`, not via `convert()`. The result is in `extraction.inlineImages`:

```typescript
interface JsInlineImage {
    data: Buffer;
    format: string;
    filename?: string;
    description?: string;
    dimensions?: number[];    // [width, height]
    source: string;           // "img_data_uri" | "svg_element"
    attributes: Record<string, string>;
}
```

## ConversionResult (from convert())

The result of `JSON.parse(convert(html))`:

```typescript
interface ConversionResult {
    content: string | null;     // Markdown text
    document: object | null;    // structured document tree (null unless includeDocumentStructure enabled)
    metadata: object | null;    // HtmlMetadata if metadata feature enabled
    tables: Array<{
        cells: Array<Array<string>>;    // rows x columns of cell text
        markdown: string;               // rendered table in target format
        isHeaderRow: Array<boolean>;    // per-row flag: true if row was inside <thead>
    }>;
    warnings: Array<{
        message: string;
        kind: string;
    }>;
}
```

**Note on `tables`:** The Node.js binding uses a flat `cells: Array<Array<string>>` structure (no `grid` wrapper), plus `isHeaderRow` for header detection. This differs from the Rust `TableGrid` struct.

## Visitor Pattern

The visitor is passed as a separate parameter to `convertWithInlineImages()` or `convertWithMetadata()` — it is **not** a field on `JsConversionOptions`. The primary `convert()` function does not accept a visitor.

```typescript
import {
    convertWithInlineImages,
    convertWithMetadata,
} from '@kreuzberg/html-to-markdown-node';
import { wrapVisitorCallbacks } from '@kreuzberg/html-to-markdown';

const visitor = wrapVisitorCallbacks({
    visitElementStart: async (ctx) => {
        // ctx.tagName, ctx.attributes available
        return { type: 'continue' };
    },
    visitText: async (ctx, text) => {
        return { type: 'continue' };
    },
});

// Visitor is the 4th parameter to convertWithInlineImages / convertWithMetadata
const extraction = convertWithInlineImages(html, options, imageConfig, visitor);
const metaExtraction = convertWithMetadata(html, options, metadataConfig, visitor);
```

Visitor return types: `{ type: 'continue' }` | `{ type: 'skip' }` | `{ type: 'preserve_html' }` | `{ type: 'custom', output: string }` | `{ type: 'error', message: string }`.

## Examples

```typescript
// Simple conversion
import { convert } from '@kreuzberg/html-to-markdown';
const result = JSON.parse(convert('<h1>Hello</h1>'));
console.log(result.content); // "# Hello\n"

// Metadata extraction — enabled via extractMetadata option, result in result.metadata
const result2 = JSON.parse(convert(html, { extractMetadata: true }));
console.log(result2.metadata.document.title);
console.log(result2.metadata.headers.length);

// Tables — always in result.tables
const result3 = JSON.parse(convert(html));
for (const table of result3.tables) {
    console.log(table.markdown);
}

// Inline images — use convertWithInlineImages() (separate function, not via convert())
import { convertWithInlineImages } from '@kreuzberg/html-to-markdown-node';
const extraction = convertWithInlineImages(html, options, { captureSvg: true });
for (const image of extraction.inlineImages) {
    console.log(image.format, image.filename);
}

// File conversion
import { convertFile } from '@kreuzberg/html-to-markdown';
const json = await convertFile('./page.html', { headingStyle: 'Atx' });
const fileResult = JSON.parse(json);
console.log(fileResult.content);

// Buffer conversion (avoids string overhead)
import { convertBuffer } from '@kreuzberg/html-to-markdown-node';
const html = Buffer.from('<h1>Hello</h1>', 'utf8');
const json2 = convertBuffer(html, { headingStyle: 'Atx' });
const bufResult = JSON.parse(json2);
```
