# TypeScript Metadata Extraction - Quick Start Guide

## Installation

The metadata extraction feature is included in the `html-to-markdown` package:

```bash
npm install html-to-markdown
# or
pnpm add html-to-markdown
```

## Basic Usage

```typescript
import { convert } from '@kreuzberg/html-to-markdown';

const html = `
  <html lang="en">
    <head>
      <title>Hello World</title>
    </head>
    <body>
      <h1>Welcome</h1>
      <p>Check out <a href="https://example.com">this link</a></p>
    </body>
  </html>
`;

const result = JSON.parse(convert(html, { extractMetadata: true }));

// Access extracted metadata
console.log(result.metadata.document.title);     // "Hello World"
console.log(result.metadata.document.language);  // "en"
console.log(result.metadata.headers[0].text);    // "Welcome"
console.log(result.metadata.links[0].href);      // "https://example.com"
```

## API Overview

### Functions

#### `convert(html, options?)`

Convert HTML string to Markdown with metadata extraction. Returns a JSON string -- always `JSON.parse()` the result.

```typescript
const result = JSON.parse(convert(html, { extractMetadata: true }));
const { content, metadata } = result;
```

#### `convertBuffer(buffer, options?)`

Convert from Buffer/Uint8Array (avoids intermediate JS string).

```typescript
const buffer = Buffer.from(html);
const result = JSON.parse(convertBuffer(buffer, { extractMetadata: true }));
```

#### `convertFile(filePath, options?)`

Convert HTML file asynchronously.

```typescript
const json = await convertFile('page.html', { extractMetadata: true });
const result = JSON.parse(json);
```

#### `convertStream(stream, options?)`

Convert from Node.js stream.

```typescript
import fs from 'node:fs';
const stream = fs.createReadStream('page.html', 'utf8');
const json = await convertStream(stream, { extractMetadata: true });
const result = JSON.parse(json);
```

## Configuration

Metadata extraction is controlled via the `extractMetadata` field in `JsConversionOptions`:

```typescript
import { convert } from '@kreuzberg/html-to-markdown';

const result = JSON.parse(convert(html, { extractMetadata: true }));
console.log(result.metadata);
```

## Metadata Structure

### Document Metadata

```typescript
const doc = result.metadata.document;
console.log(doc.title);           // Page title
console.log(doc.description);     // Meta description
console.log(doc.keywords);        // Array of keywords
console.log(doc.author);          // Author name
console.log(doc.canonicalUrl);    // Canonical URL
console.log(doc.language);        // Language code (e.g., "en")
console.log(doc.textDirection);   // "ltr" | "rtl" | "auto"
console.log(doc.openGraph);       // og:* properties
console.log(doc.twitterCard);     // twitter:* properties
```

### Headers

```typescript
result.metadata.headers.forEach(header => {
  console.log(`H${header.level}: ${header.text}`);
  console.log(`ID: ${header.id}`);
  console.log(`Depth: ${header.depth}`);
});
```

### Links

```typescript
result.metadata.links.forEach(link => {
  console.log(`[${link.text}](${link.href})`);
  console.log(`Type: ${link.linkType}`); // "internal", "external", "email", etc.
  console.log(`Rel: ${link.rel.join(' ')}`);
});
```

### Images

```typescript
result.metadata.images.forEach(img => {
  console.log(`![${img.alt}](${img.src})`);
  console.log(`Type: ${img.imageType}`); // "external", "data_uri", "inline_svg", etc.
  if (img.dimensions) {
    console.log(`Size: ${img.dimensions[0]}x${img.dimensions[1]}`);
  }
});
```

### Structured Data

```typescript
result.metadata.structuredData.forEach(data => {
  console.log(`Type: ${data.dataType}`); // "json_ld", "microdata", "rdfa"
  const schema = JSON.parse(data.rawJson);
  console.log(`Schema: ${data.schemaType}`);
});
```

## Common Use Cases

### Extract SEO Metadata

```typescript
function getSeoMetadata(html: string) {
  const result = JSON.parse(convert(html, { extractMetadata: true }));
  const doc = result.metadata.document;

  return {
    title: doc.title,
    description: doc.description,
    keywords: doc.keywords,
    canonical: doc.canonicalUrl,
    ogImage: doc.openGraph.image,
    twitterCard: doc.twitterCard.card,
  };
}
```

### Build Table of Contents

```typescript
function buildTOC(html: string) {
  const result = JSON.parse(convert(html, { extractMetadata: true }));

  return result.metadata.headers.map(h => ({
    level: h.level,
    text: h.text,
    anchor: h.id || h.text.toLowerCase().replace(/\s+/g, '-'),
  }));
}
```

### Extract All Links

```typescript
function getLinks(html: string) {
  const result = JSON.parse(convert(html, { extractMetadata: true }));

  return {
    internal: result.metadata.links.filter(l => l.linkType === 'internal'),
    external: result.metadata.links.filter(l => l.linkType === 'external'),
    emails: result.metadata.links.filter(l => l.linkType === 'email'),
  };
}
```

### Extract Images

```typescript
function getImages(html: string) {
  const result = JSON.parse(convert(html, { extractMetadata: true }));

  return result.metadata.images.map(img => ({
    url: img.src,
    alt: img.alt || 'No description',
    title: img.title,
  }));
}
```

### Check Accessibility

```typescript
function checkA11y(html: string) {
  const result = JSON.parse(convert(html, { extractMetadata: true }));

  return {
    imagesWithoutAlt: result.metadata.images
      .filter(img => !img.alt)
      .map(img => img.src),
    linksWithoutText: result.metadata.links
      .filter(link => !link.text.trim())
      .map(link => link.href),
  };
}
```

## Performance Tips

1. **Use Buffer variant for large files**:

   ```typescript
   import { convertBuffer } from '@kreuzberg/html-to-markdown-node';
   const buffer = await fs.promises.readFile('large.html');
   const result = JSON.parse(convertBuffer(buffer, { extractMetadata: true }));
   ```

2. **Use Stream for processing**:

   ```typescript
   import { convertStream } from '@kreuzberg/html-to-markdown';
   const stream = fs.createReadStream('large.html', 'utf8');
   const result = JSON.parse(await convertStream(stream, { extractMetadata: true }));
   ```

3. **Disable metadata extraction if not needed**:

   ```typescript
   const result = JSON.parse(convert(html, { extractMetadata: false }));
   ```

## Type Safety

All metadata types are fully typed for TypeScript:

```typescript
import { convert } from '@kreuzberg/html-to-markdown';

// JSON.parse() the result for full type safety
const result = JSON.parse(convert(html, { extractMetadata: true }));
const title: string | undefined = result.metadata?.document?.title;
const headers = result.metadata?.headers ?? [];
```

## Error Handling

```typescript
try {
  const result = JSON.parse(convert(html, { extractMetadata: true }));
  // Use result.content, result.metadata
} catch (error) {
  console.error('Conversion failed:', error);
}
```

## More Information

- **Complete API Documentation**: See `METADATA.md`
- **Implementation Details**: See `IMPLEMENTATION_SUMMARY.md`
- **Test Examples**: Check `tests/metadata.spec.ts`

## Support

For issues or questions:

- Check the main README.md
- See METADATA.md for detailed API documentation
- Review test cases in `tests/metadata.spec.ts`
