---
title: PHP API Reference
description: API reference for the kreuzberg-dev/html-to-markdown PHP package
---

# PHP API Reference <span class="version-badge">v2.5.6</span>

**Package:** [`kreuzberg-dev/html-to-markdown`](https://packagist.org/packages/kreuzberg-dev/html-to-markdown) | **Version:** 2.28.1 | **PHP:** 8.2+

---

## Installation

```bash
composer require kreuzberg-dev/html-to-markdown
```

The package requires the `html_to_markdown` PHP extension (installed automatically via PIE or from the pre-built binaries).

---

## Static Methods

All methods are available on the `HtmlToMarkdown\HtmlToMarkdown` class.

### `HtmlToMarkdown::convert`

Convert HTML to Markdown.

```php
public static function convert(
    string $html,
    ConversionOptions|array|null $options = null,
): string
```

**Parameters:**

| Parameter | Type | Description |
|-----------|------|-------------|
| `$html` | `string` | The HTML string to convert |
| `$options` | `ConversionOptions\|array\|null` | Optional conversion configuration (object or associative array) |

**Returns:** `string` -- the converted Markdown.

**Throws:** `\RuntimeException` on conversion failure.

**Example:**

```php
use HtmlToMarkdown\HtmlToMarkdown;
use HtmlToMarkdown\Config\ConversionOptions;

$html = '<h1>Hello</h1><p>World</p>';
$markdown = HtmlToMarkdown::convert($html);

// With options as array
$markdown = HtmlToMarkdown::convert($html, [
    'heading_style' => 'atx',
    'code_block_style' => 'backticks',
]);

// With ConversionOptions object
$options = new ConversionOptions(
    headingStyle: 'atx',
    codeBlockStyle: 'backticks',
    wrap: true,
    wrapWidth: 80,
);
$markdown = HtmlToMarkdown::convert($html, $options);
```

---

### `HtmlToMarkdown::convertWithMetadata`

Convert HTML to Markdown with metadata extraction.

```php
public static function convertWithMetadata(
    string $html,
    ConversionOptions|array|null $options = null,
    ?array $metadataConfig = null,
): array
```

**Parameters:**

| Parameter | Type | Description |
|-----------|------|-------------|
| `$html` | `string` | The HTML string to convert |
| `$options` | `ConversionOptions\|array\|null` | Optional conversion configuration |
| `$metadataConfig` | `?array` | Metadata extraction configuration |

**Returns:** `array{markdown: string, metadata: ExtendedMetadata}` -- associative array with `markdown` and `metadata` keys.

**Example:**

```php
use HtmlToMarkdown\HtmlToMarkdown;

$html = '<html lang="en"><head><title>Article</title></head>
         <body><h1>Title</h1><a href="https://example.com">Link</a></body></html>';

$result = HtmlToMarkdown::convertWithMetadata($html);
echo $result['markdown'];
echo $result['metadata']->document->title;  // "Article"
echo count($result['metadata']->headers);   // 1
echo count($result['metadata']->links);     // 1

// Selective extraction
$result = HtmlToMarkdown::convertWithMetadata($html, null, [
    'extract_headers' => true,
    'extract_links' => false,
    'extract_images' => false,
]);
```

---

### `HtmlToMarkdown::convertWithVisitor`

Convert HTML with a custom visitor for advanced control.

```php
public static function convertWithVisitor(
    string $html,
    ConversionOptions|array|null $options = null,
    ?HtmlVisitor $visitor = null,
): string
```

**Parameters:**

| Parameter | Type | Description |
|-----------|------|-------------|
| `$html` | `string` | The HTML string to convert |
| `$options` | `ConversionOptions\|array\|null` | Optional conversion configuration |
| `$visitor` | `?HtmlVisitor` | Visitor implementing callback methods |

**Returns:** `string` -- the converted Markdown.

**Example:**

```php
use HtmlToMarkdown\HtmlToMarkdown;
use HtmlToMarkdown\Visitor\HtmlVisitor;

class SkipImages implements HtmlVisitor
{
    public function visitImage($ctx, $src, $alt, $title): array
    {
        return ['type' => 'skip'];
    }
}

$markdown = HtmlToMarkdown::convertWithVisitor($html, null, new SkipImages());
```

---

### `HtmlToMarkdown::convertWithInlineImages`

Convert HTML while extracting inline image assets.

```php
public static function convertWithInlineImages(
    string $html,
    ConversionOptions|array|null $options = null,
    InlineImageConfig|array|null $config = null,
): InlineImageExtraction
```

---

## Classes

### `ConversionOptions`

```php
namespace HtmlToMarkdown\Config;

class ConversionOptions
{
    public function __construct(
        public readonly string $headingStyle = 'underlined',
        public readonly string $listIndentType = 'spaces',
        public readonly int $listIndentWidth = 4,
        public readonly string $bullets = '*+-',
        public readonly string $strongEmSymbol = '*',
        public readonly bool $escapeAsterisks = false,
        public readonly bool $escapeUnderscores = false,
        public readonly bool $escapeMisc = false,
        public readonly bool $escapeAscii = false,
        public readonly string $codeLanguage = '',
        public readonly bool $autolinks = true,
        public readonly bool $defaultTitle = false,
        public readonly bool $brInTables = false,
        public readonly bool $hocrSpatialTables = true,
        public readonly string $highlightStyle = 'double-equal',
        public readonly bool $extractMetadata = true,
        public readonly string $whitespaceMode = 'normalized',
        public readonly bool $stripNewlines = false,
        public readonly bool $wrap = false,
        public readonly int $wrapWidth = 80,
        public readonly bool $convertAsInline = false,
        public readonly string $subSymbol = '',
        public readonly string $supSymbol = '',
        public readonly string $newlineStyle = 'spaces',
        public readonly string $codeBlockStyle = 'indented',
        public readonly array $preserveTags = [],
        public readonly array $stripTags = [],
        public readonly bool $skipImages = false,
        public readonly string $outputFormat = 'markdown',
        public readonly ?PreprocessingOptions $preprocessing = null,
        public readonly string $encoding = 'utf-8',
        public readonly bool $debug = false,
    )
}
```

See the [Configuration Reference](configuration.md) for detailed descriptions.

---

### `HtmlVisitor` Interface

```php
namespace HtmlToMarkdown\Visitor;

interface HtmlVisitor
{
    // All methods are optional -- implement only the ones you need
}
```

Visitor callback methods return associative arrays:

```php
return ['type' => 'continue'];
return ['type' => 'skip'];
return ['type' => 'preserve_html'];
return ['type' => 'custom', 'output' => 'Custom markdown'];
return ['type' => 'error', 'message' => 'Error description'];
```

---

## Metadata Config Array

```php
$metadataConfig = [
    'extract_document' => true,
    'extract_headers' => true,
    'extract_links' => true,
    'extract_images' => true,
    'extract_structured_data' => true,
    'max_structured_data_size' => 1_000_000,
];
```

---

## See Also

- [Configuration Reference](configuration.md) -- full options documentation
- [Types Reference](types.md) -- cross-language type definitions
