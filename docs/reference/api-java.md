---
title: "Java API Reference"
---

## Java API Reference <span class="version-badge">v3.2.6</span>

### Functions

#### convert()

Convert HTML to Markdown, returning a `ConversionResult` with content, metadata, images,
and warnings.

**Errors:**

Returns an error if HTML parsing fails or if the input contains invalid UTF-8.

**Signature:**

```java
public static ConversionResult convert(String html, ConversionOptions options, String visitor) throws Error
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `html` | `String` | Yes | The HTML string to convert |
| `options` | `Optional<ConversionOptions>` | No | Optional conversion options (defaults to `default options`) |
| `visitor` | `Optional<String>` | No | The visitor |

**Returns:** `ConversionResult`

**Errors:** Throws `ErrorException`.


---

### Types

#### ConversionOptions

Main conversion options for HTML to Markdown conversion.

Use `ConversionOptions.builder()` to construct, or `the default constructor` for defaults.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `headingStyle` | `HeadingStyle` | `HeadingStyle.ATX` | Heading style to use in Markdown output (ATX `#` or Setext underline). |
| `listIndentType` | `ListIndentType` | `ListIndentType.SPACES` | How to indent nested list items (spaces or tab). |
| `listIndentWidth` | `long` | `2` | Number of spaces (or tabs) to use for each level of list indentation. |
| `bullets` | `String` | `"-*+"` | Bullet character(s) to use for unordered list items (e.g. `"-"`, `"*"`). |
| `strongEmSymbol` | `String` | `"*"` | Character used for bold/italic emphasis markers (`*` or `_`). |
| `escapeAsterisks` | `boolean` | `false` | Escape `*` characters in plain text to avoid unintended bold/italic. |
| `escapeUnderscores` | `boolean` | `false` | Escape `_` characters in plain text to avoid unintended bold/italic. |
| `escapeMisc` | `boolean` | `false` | Escape miscellaneous Markdown metacharacters (`[]()#` etc.) in plain text. |
| `escapeAscii` | `boolean` | `false` | Escape ASCII characters that have special meaning in certain Markdown dialects. |
| `codeLanguage` | `String` | `""` | Default language annotation for fenced code blocks that have no language hint. |
| `autolinks` | `boolean` | `true` | Automatically convert bare URLs into Markdown autolinks. |
| `defaultTitle` | `boolean` | `false` | Emit a default title when no `<title>` tag is present. |
| `brInTables` | `boolean` | `false` | Render `<br>` elements inside table cells as literal line breaks. |
| `highlightStyle` | `HighlightStyle` | `HighlightStyle.DOUBLE_EQUAL` | Style used for `<mark>` / highlighted text (e.g. `==text==`). |
| `extractMetadata` | `boolean` | `true` | Extract `<meta>` and `<head>` information into the result metadata. |
| `whitespaceMode` | `WhitespaceMode` | `WhitespaceMode.NORMALIZED` | Controls how whitespace is normalised during conversion. |
| `stripNewlines` | `boolean` | `false` | Strip all newlines from the output, producing a single-line result. |
| `wrap` | `boolean` | `false` | Wrap long lines at `wrap_width` characters. |
| `wrapWidth` | `long` | `80` | Maximum line width when `wrap` is enabled (default `80`). |
| `convertAsInline` | `boolean` | `false` | Treat the entire document as inline content (no block-level wrappers). |
| `subSymbol` | `String` | `""` | Markdown notation for subscript text (e.g. `"~"`). |
| `supSymbol` | `String` | `""` | Markdown notation for superscript text (e.g. `"^"`). |
| `newlineStyle` | `NewlineStyle` | `NewlineStyle.SPACES` | How to encode hard line breaks (`<br>`) in Markdown. |
| `codeBlockStyle` | `CodeBlockStyle` | `CodeBlockStyle.BACKTICKS` | Style used for fenced code blocks (backticks or tilde). |
| `keepInlineImagesIn` | `List<String>` | `Collections.emptyList()` | HTML tag names whose `<img>` children are kept inline instead of block. |
| `preprocessing` | `PreprocessingOptions` | — | Pre-processing options applied to the HTML before conversion. |
| `encoding` | `String` | `"utf-8"` | Expected character encoding of the input HTML (default `"utf-8"`). |
| `debug` | `boolean` | `false` | Emit debug information during conversion. |
| `stripTags` | `List<String>` | `Collections.emptyList()` | HTML tag names whose content is stripped from the output entirely. |
| `preserveTags` | `List<String>` | `Collections.emptyList()` | HTML tag names that are preserved verbatim in the output. |
| `skipImages` | `boolean` | `false` | Skip conversion of `<img>` elements (omit images from output). |
| `linkStyle` | `LinkStyle` | `LinkStyle.INLINE` | Link rendering style (inline or reference). |
| `outputFormat` | `OutputFormat` | `OutputFormat.MARKDOWN` | Target output format (Markdown, plain text, etc.). |
| `includeDocumentStructure` | `boolean` | `false` | Include structured document tree in result. |
| `extractImages` | `boolean` | `false` | Extract inline images from data URIs and SVGs. |
| `maxImageSize` | `long` | `5242880` | Maximum decoded image size in bytes (default 5MB). |
| `captureSvg` | `boolean` | `false` | Capture SVG elements as images. |
| `inferDimensions` | `boolean` | `true` | Infer image dimensions from data. |
| `maxDepth` | `Optional<long>` | `null` | Maximum DOM traversal depth. `None` means unlimited. When set, subtrees beyond this depth are silently truncated. |

##### Methods

###### defaultOptions()

**Signature:**

```java
public static ConversionOptions defaultOptions()
```

###### builder()

Create a new builder with default values.

**Signature:**

```java
public static ConversionOptionsBuilder builder()
```

###### applyUpdate()

Apply a partial update to these conversion options.

**Signature:**

```java
public void applyUpdate(ConversionOptionsUpdate update)
```

###### fromUpdate()

Create from a partial update, applying to defaults.

**Signature:**

```java
public static ConversionOptions fromUpdate(ConversionOptionsUpdate update)
```

###### from()

**Signature:**

```java
public static ConversionOptions from(ConversionOptionsUpdate update)
```


---

#### ConversionResult

The primary result of HTML conversion and extraction.

Contains the converted text output, optional structured document tree,
metadata, extracted tables, images, and processing warnings.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `content` | `Optional<String>` | `null` | Converted text output (markdown, djot, or plain text). `None` when `output_format` is set to `OutputFormat.None`, indicating extraction-only mode. |
| `document` | `Optional<DocumentStructure>` | `null` | Structured document tree with semantic elements. Populated when `include_document_structure` is `True` in options. |
| `metadata` | `HtmlMetadata` | — | Extracted HTML metadata (title, OG, links, images, structured data). |
| `tables` | `List<TableData>` | `Collections.emptyList()` | Extracted tables with structured cell data and markdown representation. |
| `images` | `List<String>` | `Collections.emptyList()` | Extracted inline images (data URIs and SVGs). Populated when `extract_images` is `True` in options. |
| `warnings` | `List<ProcessingWarning>` | `Collections.emptyList()` | Non-fatal processing warnings. |


---

#### ConversionOptionsBuilder

Builder for `ConversionOptions`.

All fields start with default values. Call `.build()` to produce the final options.

##### Methods

###### stripTags()

Set the list of HTML tag names whose content is stripped from output.

**Signature:**

```java
public ConversionOptionsBuilder stripTags(List<String> tags)
```

###### preserveTags()

Set the list of HTML tag names that are preserved verbatim in output.

**Signature:**

```java
public ConversionOptionsBuilder preserveTags(List<String> tags)
```

###### keepInlineImagesIn()

Set the list of HTML tag names whose `<img>` children are kept inline.

**Signature:**

```java
public ConversionOptionsBuilder keepInlineImagesIn(List<String> tags)
```

###### preprocessing()

Set the pre-processing options applied to the HTML before conversion.

**Signature:**

```java
public ConversionOptionsBuilder preprocessing(PreprocessingOptions preprocessing)
```

###### build()

Build the final `ConversionOptions`.

**Signature:**

```java
public ConversionOptions build()
```


---

#### DocumentMetadata

Document-level metadata extracted from `<head>` and top-level elements.

Contains all metadata typically used by search engines, social media platforms,
and browsers for document indexing and presentation.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `title` | `Optional<String>` | `null` | Document title from `<title>` tag |
| `description` | `Optional<String>` | `null` | Document description from `<meta name="description">` tag |
| `keywords` | `List<String>` | `Collections.emptyList()` | Document keywords from `<meta name="keywords">` tag, split on commas |
| `author` | `Optional<String>` | `null` | Document author from `<meta name="author">` tag |
| `canonicalUrl` | `Optional<String>` | `null` | Canonical URL from `<link rel="canonical">` tag |
| `baseHref` | `Optional<String>` | `null` | Base URL from `<base href="">` tag for resolving relative URLs |
| `language` | `Optional<String>` | `null` | Document language from `lang` attribute |
| `textDirection` | `Optional<TextDirection>` | `null` | Document text direction from `dir` attribute |
| `openGraph` | `Map<String, String>` | `Collections.emptyMap()` | Open Graph metadata (og:* properties) for social media Keys like "title", "description", "image", "url", etc. |
| `twitterCard` | `Map<String, String>` | `Collections.emptyMap()` | Twitter Card metadata (twitter:* properties) Keys like "card", "site", "creator", "title", "description", "image", etc. |
| `metaTags` | `Map<String, String>` | `Collections.emptyMap()` | Additional meta tags not covered by specific fields Keys are meta name/property attributes, values are content |


---

#### DocumentNode

A single node in the document tree.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `id` | `String` | — | Deterministic node identifier. |
| `content` | `NodeContent` | — | The semantic content of this node. |
| `parent` | `Optional<int>` | `null` | Index of the parent node (None for root nodes). |
| `children` | `List<Integer>` | — | Indices of child nodes in reading order. |
| `annotations` | `List<TextAnnotation>` | — | Inline formatting annotations (bold, italic, links, etc.) with byte offsets into the text. |
| `attributes` | `Optional<Map<String, String>>` | `null` | Format-specific attributes (e.g. class, id, data-* attributes). |


---

#### DocumentStructure

A structured document tree representing the semantic content of an HTML document.

Uses a flat node array with index-based parent/child references for efficient traversal.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `nodes` | `List<DocumentNode>` | — | All nodes in document reading order. |
| `sourceFormat` | `Optional<String>` | `null` | The source format (always "html" for this library). |


---

#### GridCell

A single cell in a table grid.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `content` | `String` | — | The text content of the cell. |
| `row` | `int` | — | 0-indexed row position. |
| `col` | `int` | — | 0-indexed column position. |
| `rowSpan` | `int` | — | Number of rows this cell spans (default 1). |
| `colSpan` | `int` | — | Number of columns this cell spans (default 1). |
| `isHeader` | `boolean` | — | Whether this is a header cell (`<th>`). |


---

#### HeaderMetadata

Header element metadata with hierarchy tracking.

Captures heading elements (h1-h6) with their text content, identifiers,
and position in the document structure.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `level` | `byte` | — | Header level: 1 (h1) through 6 (h6) |
| `text` | `String` | — | Normalized text content of the header |
| `id` | `Optional<String>` | `null` | HTML id attribute if present |
| `depth` | `long` | — | Document tree depth at the header element |
| `htmlOffset` | `long` | — | Byte offset in original HTML document |

##### Methods

###### isValid()

Validate that the header level is within valid range (1-6).

**Returns:**

`true` if level is 1-6, `false` otherwise.

**Signature:**

```java
public boolean isValid()
```


---

#### HtmlMetadata

Comprehensive metadata extraction result from HTML document.

Contains all extracted metadata types in a single structure,
suitable for serialization and transmission across language boundaries.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `document` | `DocumentMetadata` | — | Document-level metadata (title, description, canonical, etc.) |
| `headers` | `List<HeaderMetadata>` | `Collections.emptyList()` | Extracted header elements with hierarchy |
| `links` | `List<LinkMetadata>` | `Collections.emptyList()` | Extracted hyperlinks with type classification |
| `images` | `List<ImageMetadata>` | `Collections.emptyList()` | Extracted images with source and dimensions |
| `structuredData` | `List<StructuredData>` | `Collections.emptyList()` | Extracted structured data blocks |


---

#### ImageMetadata

Image metadata with source and dimensions.

Captures `<img>` elements and inline `<svg>` elements with metadata
for image analysis and optimization.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `src` | `String` | — | Image source (URL, data URI, or SVG content identifier) |
| `alt` | `Optional<String>` | `null` | Alternative text from alt attribute (for accessibility) |
| `title` | `Optional<String>` | `null` | Title attribute (often shown as tooltip) |
| `dimensions` | `Optional<String>` | `null` | Image dimensions as (width, height) if available |
| `imageType` | `ImageType` | — | Image type classification |
| `attributes` | `Map<String, String>` | — | Additional HTML attributes |


---

#### LinkMetadata

Hyperlink metadata with categorization and attributes.

Represents `<a>` elements with parsed href values, text content, and link type classification.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `href` | `String` | — | The href URL value |
| `text` | `String` | — | Link text content (normalized, concatenated if mixed with elements) |
| `title` | `Optional<String>` | `null` | Optional title attribute (often shown as tooltip) |
| `linkType` | `LinkType` | — | Link type classification |
| `rel` | `List<String>` | — | Rel attribute values (e.g., "nofollow", "stylesheet", "canonical") |
| `attributes` | `Map<String, String>` | — | Additional HTML attributes |

##### Methods

###### classifyLink()

Classify a link based on href value.

**Returns:**

Appropriate `LinkType` based on protocol and content.

**Signature:**

```java
public static LinkType classifyLink(String href)
```


---

#### MetadataConfig

Configuration for metadata extraction granularity.

Controls which metadata types are extracted and size limits for safety.
Enables selective extraction of different metadata categories from HTML documents,
allowing fine-grained control over which types of information to collect during
the HTML-to-Markdown conversion process.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `extractDocument` | `boolean` | `true` | Extract document-level metadata (title, description, author, etc.). When enabled, collects metadata from `<head>` section including: - `<title>` element content - `<meta name="description">` and other standard meta tags - Open Graph (og:*) properties for social media optimization - Twitter Card (twitter:*) properties - Language and text direction attributes - Canonical URL and base href references |
| `extractHeaders` | `boolean` | `true` | Extract h1-h6 header elements and their hierarchy. When enabled, collects all heading elements with: - Header level (1-6) - Text content (normalized) - HTML id attribute if present - Document tree depth for hierarchy tracking - Byte offset in original HTML for positioning |
| `extractLinks` | `boolean` | `true` | Extract anchor (a) elements as links with type classification. When enabled, collects all hyperlinks with: - href attribute value - Link text content - Title attribute (tooltip text) - Automatic link type classification (anchor, internal, external, email, phone, other) - Rel attribute values - Additional custom attributes |
| `extractImages` | `boolean` | `true` | Extract image elements and data URIs. When enabled, collects all image elements with: - Source URL or data URI - Alt text for accessibility - Title attribute - Dimensions (width, height) if available - Automatic image type classification (data URI, external, relative, inline SVG) - Additional custom attributes |
| `extractStructuredData` | `boolean` | `true` | Extract structured data (JSON-LD, Microdata, RDFa). When enabled, collects machine-readable structured data including: - JSON-LD script blocks with schema detection - Microdata attributes (itemscope, itemtype, itemprop) - RDFa markup - Extracted schema type if detectable |
| `maxStructuredDataSize` | `long` | — | Maximum total size of structured data to collect (bytes). Prevents memory exhaustion attacks on malformed or adversarial documents containing excessively large structured data blocks. When the accumulated size of structured data exceeds this limit, further collection stops. Default: `1_000_000` bytes (1 MB) |

##### Methods

###### defaultOptions()

Create default metadata configuration.

Defaults to extracting all metadata types with 1MB limit on structured data.

**Signature:**

```java
public static MetadataConfig defaultOptions()
```

###### anyEnabled()

Check if any metadata extraction is enabled.

Returns `true` if at least one extraction category is enabled, `false` if all are disabled.
This is useful for early exit optimization when the application doesn't need metadata.

**Returns:**

`true` if any of the extraction flags are enabled, `false` if all are disabled.

**Signature:**

```java
public boolean anyEnabled()
```

###### applyUpdate()

Apply a partial update to this metadata configuration.

Any specified fields in the update (Some values) will override the current values.
Unspecified fields (None) are left unchanged. This allows selective modification
of configuration without affecting unrelated settings.

**Signature:**

```java
public void applyUpdate(MetadataConfigUpdate update)
```

###### fromUpdate()

Create new metadata configuration from a partial update.

Creates a new `MetadataConfig` struct with defaults, then applies the update.
Fields not specified in the update (None) keep their default values.
This is a convenience method for constructing a configuration from a partial specification
without needing to explicitly call `.default()` first.

**Returns:**

New `MetadataConfig` with specified updates applied to defaults

**Signature:**

```java
public static MetadataConfig fromUpdate(MetadataConfigUpdate update)
```

###### from()

**Signature:**

```java
public static MetadataConfig from(MetadataConfigUpdate update)
```


---

#### NodeContext

Context information passed to all visitor methods.

Provides comprehensive metadata about the current node being visited,
including its type, attributes, position in the DOM tree, and parent context.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `nodeType` | `NodeType` | — | Coarse-grained node type classification |
| `tagName` | `String` | — | Raw HTML tag name (e.g., "div", "h1", "custom-element") |
| `attributes` | `Map<String, String>` | — | All HTML attributes as key-value pairs |
| `depth` | `long` | — | Depth in the DOM tree (0 = root) |
| `indexInParent` | `long` | — | Index among siblings (0-based) |
| `parentTag` | `Optional<String>` | `null` | Parent element's tag name (None if root) |
| `isInline` | `boolean` | — | Whether this element is treated as inline vs block |


---

#### PreprocessingOptions

HTML preprocessing options for document cleanup before conversion.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `enabled` | `boolean` | `true` | Enable HTML preprocessing globally |
| `preset` | `PreprocessingPreset` | `PreprocessingPreset.STANDARD` | Preprocessing preset level (Minimal, Standard, Aggressive) |
| `removeNavigation` | `boolean` | `true` | Remove navigation elements (nav, breadcrumbs, menus, sidebars) |
| `removeForms` | `boolean` | `true` | Remove form elements (forms, inputs, buttons, etc.) |

##### Methods

###### defaultOptions()

**Signature:**

```java
public static PreprocessingOptions defaultOptions()
```

###### applyUpdate()

Apply a partial update to these preprocessing options.

Any specified fields in the update will override the current values.
Unspecified fields (None) are left unchanged.

**Signature:**

```java
public void applyUpdate(PreprocessingOptionsUpdate update)
```

###### fromUpdate()

Create new preprocessing options from a partial update.

Creates a new `PreprocessingOptions` struct with defaults, then applies the update.
Fields not specified in the update keep their default values.

**Returns:**

New `PreprocessingOptions` with specified updates applied to defaults

**Signature:**

```java
public static PreprocessingOptions fromUpdate(PreprocessingOptionsUpdate update)
```

###### from()

**Signature:**

```java
public static PreprocessingOptions from(PreprocessingOptionsUpdate update)
```


---

#### ProcessingWarning

A non-fatal warning generated during HTML processing.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `message` | `String` | — | Human-readable warning message. |
| `kind` | `WarningKind` | — | The category of warning. |


---

#### StructuredData

Structured data block (JSON-LD, Microdata, or RDFa).

Represents machine-readable structured data found in the document.
JSON-LD blocks are collected as raw JSON strings for flexibility.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `dataType` | `StructuredDataType` | — | Type of structured data (JSON-LD, Microdata, RDFa) |
| `rawJson` | `String` | — | Raw JSON string (for JSON-LD) or serialized representation |
| `schemaType` | `Optional<String>` | `null` | Schema type if detectable (e.g., "Article", "Event", "Product") |


---

#### TableData

A top-level extracted table with both structured data and markdown representation.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `grid` | `TableGrid` | — | The structured table grid. |
| `markdown` | `String` | — | The markdown rendering of this table. |


---

#### TableGrid

A structured table grid with cell-level data including spans.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `rows` | `int` | — | Number of rows. |
| `cols` | `int` | — | Number of columns. |
| `cells` | `List<GridCell>` | `Collections.emptyList()` | All cells in the table (may be fewer than rows*cols due to spans). |


---

#### TextAnnotation

An inline text annotation with byte-range offsets.

Annotations describe formatting (bold, italic, etc.) and links within a node's text content.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `start` | `int` | — | Start byte offset (inclusive) into the parent node's text. |
| `end` | `int` | — | End byte offset (exclusive) into the parent node's text. |
| `kind` | `AnnotationKind` | — | The type of annotation. |


---

### Enums

#### TextDirection

Text directionality of document content.

Corresponds to the HTML `dir` attribute and `bdi` element directionality.

| Value | Description |
|-------|-------------|
| `LEFT_TO_RIGHT` | Left-to-right text flow (default for Latin scripts) |
| `RIGHT_TO_LEFT` | Right-to-left text flow (Hebrew, Arabic, Urdu, etc.) |
| `AUTO` | Automatic directionality detection |


---

#### LinkType

Link classification based on href value and document context.

Used to categorize links during extraction for filtering and analysis.

| Value | Description |
|-------|-------------|
| `ANCHOR` | Anchor link within same document (href starts with #) |
| `INTERNAL` | Internal link within same domain |
| `EXTERNAL` | External link to different domain |
| `EMAIL` | Email link (mailto:) |
| `PHONE` | Phone link (tel:) |
| `OTHER` | Other protocol or unclassifiable |


---

#### ImageType

Image source classification for proper handling and processing.

Determines whether an image is embedded (data URI), inline SVG, external, or relative.

| Value | Description |
|-------|-------------|
| `DATA_URI` | Data URI embedded image (base64 or other encoding) |
| `INLINE_SVG` | Inline SVG element |
| `EXTERNAL` | External image URL (http/https) |
| `RELATIVE` | Relative image path |


---

#### StructuredDataType

Structured data format type.

Identifies the schema/format used for structured data markup.

| Value | Description |
|-------|-------------|
| `JSON_LD` | JSON-LD (JSON for Linking Data) script blocks |
| `MICRODATA` | HTML5 Microdata attributes (itemscope, itemtype, itemprop) |
| `RDFA` | RDF in Attributes (RDFa) markup |


---

#### PreprocessingPreset

HTML preprocessing aggressiveness level.

Controls the extent of cleanup performed before conversion. Higher levels remove more elements.

| Value | Description |
|-------|-------------|
| `MINIMAL` | Minimal cleanup. Remove only essential noise (scripts, styles). |
| `STANDARD` | Standard cleanup. Default. Removes navigation, forms, and other auxiliary content. |
| `AGGRESSIVE` | Aggressive cleanup. Remove extensive non-content elements and structure. |


---

#### HeadingStyle

Heading style options for Markdown output.

Controls how headings (h1-h6) are rendered in the output Markdown.

| Value | Description |
|-------|-------------|
| `UNDERLINED` | Underlined style (=== for h1, --- for h2). |
| `ATX` | ATX style (# for h1, ## for h2, etc.). Default. |
| `ATX_CLOSED` | ATX closed style (# title #, with closing hashes). |


---

#### ListIndentType

List indentation character type.

Controls whether list items are indented with spaces or tabs.

| Value | Description |
|-------|-------------|
| `SPACES` | Use spaces for indentation. Default. Width controlled by `list_indent_width`. |
| `TABS` | Use tabs for indentation. |


---

#### WhitespaceMode

Whitespace handling strategy during conversion.

Determines how sequences of whitespace characters (spaces, tabs, newlines) are processed.

| Value | Description |
|-------|-------------|
| `NORMALIZED` | Collapse multiple whitespace characters to single spaces. Default. Matches browser behavior. |
| `STRICT` | Preserve all whitespace exactly as it appears in the HTML. |


---

#### NewlineStyle

Line break syntax in Markdown output.

Controls how soft line breaks (from `<br>` or line breaks in source) are rendered.

| Value | Description |
|-------|-------------|
| `SPACES` | Two trailing spaces at end of line. Default. Standard Markdown syntax. |
| `BACKSLASH` | Backslash at end of line. Alternative Markdown syntax. |


---

#### CodeBlockStyle

Code block fence style in Markdown output.

Determines how code blocks (`<pre><code>`) are rendered in Markdown.

| Value | Description |
|-------|-------------|
| `INDENTED` | Indented code blocks (4 spaces). `CommonMark` standard. |
| `BACKTICKS` | Fenced code blocks with backticks (```). Default (GFM). Supports language hints. |
| `TILDES` | Fenced code blocks with tildes (~~~). Supports language hints. |


---

#### HighlightStyle

Highlight rendering style for `<mark>` elements.

Controls how highlighted text is rendered in Markdown output.

| Value | Description |
|-------|-------------|
| `DOUBLE_EQUAL` | Double equals syntax (==text==). Default. Pandoc-compatible. |
| `HTML` | Preserve as HTML (==text==). Original HTML tag. |
| `BOLD` | Render as bold (**text**). Uses strong emphasis. |
| `NONE` | Strip formatting, render as plain text. No markup. |


---

#### LinkStyle

Link rendering style in Markdown output.

Controls whether links and images use inline `[text](url)` syntax or
reference-style `[text][1]` syntax with definitions collected at the end.

| Value | Description |
|-------|-------------|
| `INLINE` | Inline links: `[text](url)`. Default. |
| `REFERENCE` | Reference-style links: `[text][1]` with `[1]: url` at end of document. |


---

#### OutputFormat

Output format for conversion.

Specifies the target markup language format for the conversion output.

| Value | Description |
|-------|-------------|
| `MARKDOWN` | Standard Markdown (CommonMark compatible). Default. |
| `DJOT` | Djot lightweight markup language. |
| `PLAIN` | Plain text output (no markup, visible text only). |


---

#### NodeContent

The semantic content type of a document node.

Uses internally tagged representation (`"node_type": "heading"`) for JSON serialization.

| Value | Description |
|-------|-------------|
| `HEADING` | A heading element (h1-h6). — Fields: `level`: `byte`, `text`: `String` |
| `PARAGRAPH` | A paragraph of text. — Fields: `text`: `String` |
| `LIST` | A list container (ordered or unordered). Children are `ListItem` nodes. — Fields: `ordered`: `boolean` |
| `LIST_ITEM` | A single list item. — Fields: `text`: `String` |
| `TABLE` | A table with structured cell data. — Fields: `grid`: `TableGrid` |
| `IMAGE` | An image element. — Fields: `description`: `String`, `src`: `String`, `imageIndex`: `int` |
| `CODE` | A code block or inline code. — Fields: `text`: `String`, `language`: `String` |
| `QUOTE` | A block quote container. |
| `DEFINITION_LIST` | A definition list container. |
| `DEFINITION_ITEM` | A definition list entry with term and description. — Fields: `term`: `String`, `definition`: `String` |
| `RAW_BLOCK` | A raw block preserved as-is (e.g. `<script>`, `<style>` content). — Fields: `format`: `String`, `content`: `String` |
| `METADATA_BLOCK` | A block of key-value metadata pairs (from `<head>` meta tags). — Fields: `entries`: `List<String>` |
| `GROUP` | A section grouping container (auto-generated from heading hierarchy). — Fields: `label`: `String`, `headingLevel`: `byte`, `headingText`: `String` |


---

#### AnnotationKind

The type of an inline text annotation.

Uses internally tagged representation (`"annotation_type": "bold"`) for JSON serialization.

| Value | Description |
|-------|-------------|
| `BOLD` | Bold / strong emphasis. |
| `ITALIC` | Italic / emphasis. |
| `UNDERLINE` | Underline. |
| `STRIKETHROUGH` | Strikethrough / deleted text. |
| `CODE` | Inline code. |
| `SUBSCRIPT` | Subscript text. |
| `SUPERSCRIPT` | Superscript text. |
| `HIGHLIGHT` | Highlighted / marked text. |
| `LINK` | A hyperlink. — Fields: `url`: `String`, `title`: `String` |


---

#### WarningKind

Categories of processing warnings.

| Value | Description |
|-------|-------------|
| `IMAGE_EXTRACTION_FAILED` | An image could not be extracted (e.g. invalid data URI, unsupported format). |
| `ENCODING_FALLBACK` | The input encoding was not recognized; fell back to UTF-8. |
| `TRUNCATED_INPUT` | The input was truncated due to size limits. |
| `MALFORMED_HTML` | The HTML was malformed but processing continued with best effort. |
| `SANITIZATION_APPLIED` | Sanitization was applied to remove potentially unsafe content. |
| `DEPTH_LIMIT_EXCEEDED` | DOM traversal was truncated because max_depth was exceeded. |


---

#### NodeType

Node type enumeration covering all HTML element types.

This enum categorizes all HTML elements that the converter recognizes,
providing a coarse-grained classification for visitor dispatch.

| Value | Description |
|-------|-------------|
| `TEXT` | Text node (most frequent - 100+ per document) |
| `ELEMENT` | Generic element node |
| `HEADING` | Heading elements (h1-h6) |
| `PARAGRAPH` | Paragraph element |
| `DIV` | Generic div container |
| `BLOCKQUOTE` | Blockquote element |
| `PRE` | Preformatted text block |
| `HR` | Horizontal rule |
| `LIST` | Ordered or unordered list (ul, ol) |
| `LIST_ITEM` | List item (li) |
| `DEFINITION_LIST` | Definition list (dl) |
| `DEFINITION_TERM` | Definition term (dt) |
| `DEFINITION_DESCRIPTION` | Definition description (dd) |
| `TABLE` | Table element |
| `TABLE_ROW` | Table row (tr) |
| `TABLE_CELL` | Table cell (td, th) |
| `TABLE_HEADER` | Table header cell (th) |
| `TABLE_BODY` | Table body (tbody) |
| `TABLE_HEAD` | Table head (thead) |
| `TABLE_FOOT` | Table foot (tfoot) |
| `LINK` | Anchor link (a) |
| `IMAGE` | Image (img) |
| `STRONG` | Strong/bold (strong, b) |
| `EM` | Emphasis/italic (em, i) |
| `CODE` | Inline code (code) |
| `STRIKETHROUGH` | Strikethrough (s, del, strike) |
| `UNDERLINE` | Underline (u, ins) |
| `SUBSCRIPT` | Subscript (sub) |
| `SUPERSCRIPT` | Superscript (sup) |
| `MARK` | Mark/highlight (mark) |
| `SMALL` | Small text (small) |
| `BR` | Line break (br) |
| `SPAN` | Span element |
| `ARTICLE` | Article element |
| `SECTION` | Section element |
| `NAV` | Navigation element |
| `ASIDE` | Aside element |
| `HEADER` | Header element |
| `FOOTER` | Footer element |
| `MAIN` | Main element |
| `FIGURE` | Figure element |
| `FIGCAPTION` | Figure caption |
| `TIME` | Time element |
| `DETAILS` | Details element |
| `SUMMARY` | Summary element |
| `FORM` | Form element |
| `INPUT` | Input element |
| `SELECT` | Select element |
| `OPTION` | Option element |
| `BUTTON` | Button element |
| `TEXTAREA` | Textarea element |
| `LABEL` | Label element |
| `FIELDSET` | Fieldset element |
| `LEGEND` | Legend element |
| `AUDIO` | Audio element |
| `VIDEO` | Video element |
| `PICTURE` | Picture element |
| `SOURCE` | Source element |
| `IFRAME` | Iframe element |
| `SVG` | SVG element |
| `CANVAS` | Canvas element |
| `RUBY` | Ruby annotation |
| `RT` | Ruby text |
| `RP` | Ruby parenthesis |
| `ABBR` | Abbreviation |
| `KBD` | Keyboard input |
| `SAMP` | Sample output |
| `VAR` | Variable |
| `CITE` | Citation |
| `Q` | Quote |
| `DEL` | Deleted text |
| `INS` | Inserted text |
| `DATA` | Data element |
| `METER` | Meter element |
| `PROGRESS` | Progress element |
| `OUTPUT` | Output element |
| `TEMPLATE` | Template element |
| `SLOT` | Slot element |
| `HTML` | HTML root element |
| `HEAD` | Head element |
| `BODY` | Body element |
| `TITLE` | Title element |
| `META` | Meta element |
| `LINK_TAG` | Link element (not anchor) |
| `STYLE` | Style element |
| `SCRIPT` | Script element |
| `BASE` | Base element |
| `CUSTOM` | Custom element (web components) or unknown tag |


---

#### VisitResult

Result of a visitor callback.

Allows visitors to control the conversion flow by either proceeding
with default behavior, providing custom output, skipping elements,
preserving HTML, or signaling errors.

| Value | Description |
|-------|-------------|
| `CONTINUE` | Continue with default conversion behavior |
| `CUSTOM` | Replace default output with custom markdown The visitor takes full responsibility for the markdown output of this node and its children. — Fields: `0`: `String` |
| `SKIP` | Skip this element entirely (don't output anything) The element and all its children are ignored in the output. |
| `PRESERVE_HTML` | Preserve original HTML (don't convert to markdown) The element's raw HTML is included verbatim in the output. |
| `ERROR` | Stop conversion with an error The conversion process halts and returns this error message. — Fields: `0`: `String` |


---

### Errors

#### ConversionError

Errors that can occur during HTML to Markdown conversion.

| Variant | Description |
|---------|-------------|
| `PARSE_ERROR` | HTML parsing error |
| `SANITIZATION_ERROR` | HTML sanitization error |
| `CONFIG_ERROR` | Invalid configuration |
| `IO_ERROR` | I/O error |
| `PANIC` | Internal error caught during conversion |
| `INVALID_INPUT` | Invalid input data |
| `OTHER` | Generic conversion error |


---
