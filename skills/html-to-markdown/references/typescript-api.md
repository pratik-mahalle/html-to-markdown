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

All fields are optional. Defaults match Rust defaults.

```typescript
interface JsConversionOptions {
    headingStyle?: 'atx' | 'underlined' | 'atxClosed';
    listIndentType?: 'spaces' | 'tabs';
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
    highlightStyle?: 'doubleEqual' | 'html' | 'bold' | 'none';
    extractMetadata?: boolean;
    whitespaceMode?: 'normalized' | 'strict';
    stripNewlines?: boolean;
    wrap?: boolean;
    wrapWidth?: number;
    convertAsInline?: boolean;
    subSymbol?: string;
    supSymbol?: string;
    newlineStyle?: 'spaces' | 'backslash';
    codeBlockStyle?: 'indented' | 'backticks' | 'tildes';
    keepInlineImagesIn?: string[];
    preprocessing?: JsPreprocessingOptions;
    encoding?: string;
    debug?: boolean;
    stripTags?: string[];
    preserveTags?: string[];
    skipImages?: boolean;
    outputFormat?: 'markdown' | 'djot' | 'plain';
    includeDocumentStructure?: boolean;
    extractImages?: boolean;
    maxImageSize?: bigint;            // BigInt! Use BigInt(5_242_880) for 5 MiB
    captureSvg?: boolean;
    inferDimensions?: boolean;
}
```

**Note on `maxImageSize`:** This field is `bigint` in NAPI-RS (maps to Rust `u64`). Pass `BigInt(5_242_880)` not the number `5_242_880`.

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

```typescript
interface JsMetadataConfig {
    extractDocument?: boolean;
    extractHeaders?: boolean;
    extractLinks?: boolean;
    extractImages?: boolean;
    extractStructuredData?: boolean;
    maxStructuredDataSize?: number;
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

### InlineImage (in ConversionResult.images)

When `extractImages: true` is set in options, `result.images` is populated:

```typescript
interface JsInlineImage {
    data: Buffer;
    format: string;
    filename: string | null;
    description: string | null;
    dimensions: number[] | null;    // [width, height]
    source: string;                 // "img_data_uri" | "svg_element"
    attributes: Record<string, string>;
}
```

## ConversionResult (from convert())

The result of `JSON.parse(convert(html))`:

```typescript
interface ConversionResult {
    content: string | null;     // Markdown text
    document: null;             // not yet wired
    metadata: object | null;    // HtmlMetadata if metadata feature enabled
    tables: Array<{
        grid: {
            rows: number;
            cols: number;
            cells: Array<{
                content: string;
                row: number;
                col: number;
                row_span: number;
                col_span: number;
                is_header: boolean;
            }>;
        };
        markdown: string;
    }>;
    warnings: Array<{
        message: string;
        kind: string;
    }>;
}
```

## Visitor Pattern

The visitor is passed via options to `convert()`:

```typescript
import { convert } from '@kreuzberg/html-to-markdown-node';
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

// Pass visitor via options to the single convert() function
const json = convert(html, { ...options, visitor });
const result = JSON.parse(json);
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

// Inline images — enabled via extractImages option, result in result.images
const result4 = JSON.parse(convert(html, { extractImages: true }));
for (const image of result4.images) {
    console.log(image.format, image.filename);
}

// File conversion
import { convertFile } from '@kreuzberg/html-to-markdown';
const json = await convertFile('./page.html', { headingStyle: 'atx' });
const fileResult = JSON.parse(json);
console.log(fileResult.content);

// Buffer conversion (avoids string overhead)
import { convertBuffer } from '@kreuzberg/html-to-markdown-node';
const html = Buffer.from('<h1>Hello</h1>', 'utf8');
const json2 = convertBuffer(html, { headingStyle: 'atx' });
const bufResult = JSON.parse(json2);
```
