---
title: "Java API Reference"
---

# Java API Reference <span class="version-badge">v3.2.0</span>

## Functions

### tableTotalColumns()

Calculate total columns in a table.

Scans all rows and cells to determine the maximum column count,
accounting for colspan values.

**Returns:**
Maximum column count (minimum 1, maximum MAX_TABLE_COLS)

**Signature:**

```java
public static long tableTotalColumns(NodeHandle nodeHandle, Parser parser, DomContext domCtx)
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `nodeHandle` | `NodeHandle` | Yes | Handle to the table element |
| `parser` | `Parser` | Yes | HTML parser instance |
| `domCtx` | `DomContext` | Yes | DOM context for tag name resolution |

**Returns:** `long`


---

### handleTable()

Convert an entire table element to Markdown.

Main entry point for table conversion. Analyzes table structure to determine
if it should be rendered as a Markdown table or converted to list format.
Handles layout tables, blank tables, and tables with semantic meaning.
Integrates with visitor pattern for custom table handling.

**Signature:**

```java
public static void handleTable(NodeHandle nodeHandle, Parser parser, String output, ConversionOptions options, Context ctx, DomContext domCtx, long depth)
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `nodeHandle` | `NodeHandle` | Yes | Handle to the table element |
| `parser` | `Parser` | Yes | HTML parser instance |
| `output` | `String` | Yes | Mutable string to append table content |
| `options` | `ConversionOptions` | Yes | Conversion options |
| `ctx` | `Context` | Yes | Conversion context (visitor, etc) |
| `domCtx` | `DomContext` | Yes | DOM context |
| `depth` | `long` | Yes | Nesting depth |

**Returns:** `void`


---

### handleCaption()

Handles caption elements within tables.

Extracts text content from the caption and formats it as italicized text
with escaped hyphens to prevent Markdown table separator interpretation.

**Signature:**

```java
public static void handleCaption(NodeHandle nodeHandle, Parser parser, String output, ConversionOptions options, Context ctx, long depth, DomContext domCtx)
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `nodeHandle` | `NodeHandle` | Yes | Handle to the caption element |
| `parser` | `Parser` | Yes | HTML parser instance |
| `output` | `String` | Yes | Output string to append caption text to |
| `options` | `ConversionOptions` | Yes | Conversion options |
| `ctx` | `Context` | Yes | Conversion context |
| `depth` | `long` | Yes | Current recursion depth |
| `domCtx` | `DomContext` | Yes | DOM context for tag name resolution |

**Returns:** `void`


---

### getColspan()

Get colspan attribute value from an element.

Reads the colspan attribute from a table cell, with bounds checking
to prevent memory exhaustion attacks.

**Returns:**
The colspan value (minimum 1, maximum MAX_TABLE_COLS)

**Signature:**

```java
public static long getColspan(NodeHandle nodeHandle, Parser parser)
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `nodeHandle` | `NodeHandle` | Yes | Handle to the cell element |
| `parser` | `Parser` | Yes | HTML parser instance |

**Returns:** `long`


---

### getColspanRowspan()

Get both colspan and rowspan in a single lookup.

More efficient than calling get_colspan and a separate rowspan lookup.

**Returns:**
A tuple of (colspan, rowspan), both minimum 1 and maximum MAX_TABLE_COLS

**Signature:**

```java
public static UsizeUsize getColspanRowspan(NodeHandle nodeHandle, Parser parser)
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `nodeHandle` | `NodeHandle` | Yes | Handle to the cell element |
| `parser` | `Parser` | Yes | HTML parser instance |

**Returns:** `UsizeUsize`


---

### collectTableCells()

Collect table cells (td/th) from a row element.

Extracts only the direct cell children of a row, filtering by tag name.

**Signature:**

```java
public static void collectTableCells(NodeHandle nodeHandle, Parser parser, DomContext domCtx, List<NodeHandle> cells)
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `nodeHandle` | `NodeHandle` | Yes | Handle to the row element |
| `parser` | `Parser` | Yes | HTML parser instance |
| `domCtx` | `DomContext` | Yes | DOM context for tag name resolution |
| `cells` | `List<NodeHandle>` | Yes | Mutable vector to populate with cell handles |

**Returns:** `void`


---

### convertTableCell()

Convert a table cell (td or th) to Markdown format.

Processes cell content and renders it with pipe delimiters for Markdown tables.
Handles colspan by adding extra pipes, and escapes pipes in cell content.

**Signature:**

```java
public static void convertTableCell(NodeHandle nodeHandle, Parser parser, String output, ConversionOptions options, Context ctx, String tagName, DomContext domCtx)
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `nodeHandle` | `NodeHandle` | Yes | Handle to the cell element |
| `parser` | `Parser` | Yes | HTML parser instance |
| `output` | `String` | Yes | Mutable string to append cell content |
| `options` | `ConversionOptions` | Yes | Conversion options (escape settings, br_in_tables) |
| `ctx` | `Context` | Yes | Conversion context (visitor, etc) |
| `tagName` | `String` | Yes | Tag name (for consistency, not used) |
| `domCtx` | `DomContext` | Yes | DOM context for content extraction |

**Returns:** `void`


---

### appendLayoutRow()

Append a layout table row as a list item.

For tables used for visual layout, converts rows to list items
instead of table format for better readability.

**Signature:**

```java
public static void appendLayoutRow(NodeHandle rowHandle, Parser parser, String output, ConversionOptions options, Context ctx, DomContext domCtx)
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `rowHandle` | `NodeHandle` | Yes | Handle to the row element |
| `parser` | `Parser` | Yes | HTML parser instance |
| `output` | `String` | Yes | Mutable string to append content |
| `options` | `ConversionOptions` | Yes | Conversion options |
| `ctx` | `Context` | Yes | Conversion context |
| `domCtx` | `DomContext` | Yes | DOM context |

**Returns:** `void`


---

### convertTableRow()

Convert a table row (tr) to Markdown format.

Processes all cells in a row, handling colspan and rowspan for proper
column alignment. Renders header separator row after the first row.
Integrates with visitor pattern for custom row handling.

**Signature:**

```java
public static void convertTableRow(NodeHandle nodeHandle, Parser parser, String output, ConversionOptions options, Context ctx, long rowIndex, boolean hasSpan, List<Optional<Long>> rowspanTracker, long totalCols, long headerCols, DomContext domCtx, long depth, boolean isHeader)
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `nodeHandle` | `NodeHandle` | Yes | Handle to the row element |
| `parser` | `Parser` | Yes | HTML parser instance |
| `output` | `String` | Yes | Mutable string to append row content |
| `options` | `ConversionOptions` | Yes | Conversion options |
| `ctx` | `Context` | Yes | Conversion context (visitor, etc) |
| `rowIndex` | `long` | Yes | Index of this row in the table |
| `hasSpan` | `boolean` | Yes | Whether table has colspan/rowspan |
| `rowspanTracker` | `List<Optional<Long>>` | Yes | Mutable array tracking rowspan remainder for each column |
| `totalCols` | `long` | Yes | Total columns in the table |
| `headerCols` | `long` | Yes | Columns to render in separator row |
| `domCtx` | `DomContext` | Yes | DOM context |
| `depth` | `long` | Yes | Nesting depth |
| `isHeader` | `boolean` | Yes | Whether this is a header row |

**Returns:** `void`


---

### scanTable()

Scan a table element for structural metadata.

Analyzes the table to determine characteristics that influence rendering:
- Whether to render as a Markdown table or layout table
- If spanning cells are present
- If the table has semantic meaning (headers, captions)

**Signature:**

```java
public static TableScan scanTable(NodeHandle nodeHandle, Parser parser, DomContext domCtx)
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `nodeHandle` | `NodeHandle` | Yes | Handle to the table element |
| `parser` | `Parser` | Yes | HTML parser instance |
| `domCtx` | `DomContext` | Yes | DOM context for tag name resolution |

**Returns:** `TableScan`


---

### dispatchTableHandler()

Dispatches table element handling to the main convert_table function.

# Usage in converter.rs
```text
if "table" == tag_name {
    crate::converter::block::table::handle_table(
        node_handle,
        parser,
        output,
        options,
        ctx,
        dom_ctx,
        depth,
    );
    return;
}
```

**Signature:**

```java
public static boolean dispatchTableHandler(String tagName, NodeHandle nodeHandle, Parser parser, String output, ConversionOptions options, Context ctx, long depth, DomContext domCtx)
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `tagName` | `String` | Yes | The tag name |
| `nodeHandle` | `NodeHandle` | Yes | The node handle |
| `parser` | `Parser` | Yes | The parser |
| `output` | `String` | Yes | The output destination |
| `options` | `ConversionOptions` | Yes | The options to use |
| `ctx` | `Context` | Yes | The context |
| `depth` | `long` | Yes | The depth |
| `domCtx` | `DomContext` | Yes | The dom context |

**Returns:** `boolean`


---

### dispatchBlockHandler()

Dispatches block element handling to the appropriate handler.

This function is designed to be called from the main walk_node function
in converter.rs once the module is refactored. It returns `true` if the
element was handled, `false` otherwise.

# Usage in converter.rs
```text
if crate::converter::block::dispatch_block_handler(
    &tag_name,
    node_handle,
    parser,
    output,
    options,
    ctx,
    depth,
    dom_ctx,
) {
    return; // Element was handled
}
```

**Signature:**

```java
public static boolean dispatchBlockHandler(String tagName, NodeHandle nodeHandle, Parser parser, String output, ConversionOptions options, Context ctx, long depth, DomContext domCtx)
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `tagName` | `String` | Yes | The tag name |
| `nodeHandle` | `NodeHandle` | Yes | The node handle |
| `parser` | `Parser` | Yes | The parser |
| `output` | `String` | Yes | The output destination |
| `options` | `ConversionOptions` | Yes | The options to use |
| `ctx` | `Context` | Yes | The context |
| `depth` | `long` | Yes | The depth |
| `domCtx` | `DomContext` | Yes | The dom context |

**Returns:** `boolean`


---

### handle()

Dispatcher for form elements.

Routes all form-related elements to their respective handlers.

**Signature:**

```java
public static void handle(String tagName, NodeHandle nodeHandle, Parser parser, String output, ConversionOptions options, Context ctx, long depth, DomContext domCtx)
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `tagName` | `String` | Yes | The tag name |
| `nodeHandle` | `NodeHandle` | Yes | The node handle |
| `parser` | `Parser` | Yes | The parser |
| `output` | `String` | Yes | The output destination |
| `options` | `ConversionOptions` | Yes | The options to use |
| `ctx` | `Context` | Yes | The context |
| `depth` | `long` | Yes | The depth |
| `domCtx` | `DomContext` | Yes | The dom context |

**Returns:** `void`


---

### dispatchFormHandler()

Dispatches form element handling to the appropriate handler.

This function routes form-related HTML elements to their specialized handlers
based on tag name. It is designed to be called from the main `walk_node`
function in `converter.rs`.

# Routing Table

The following tag routes are supported:
- **Containers**: form, fieldset, legend, label
- **Inputs**: input, textarea, select, option, optgroup, button
- **Measurements**: progress, meter, output, datalist

**Returns:**

Returns `true` if the tag was successfully handled by a form handler,
`false` if the tag is not a form element and requires other handling.

**Signature:**

```java
public static boolean dispatchFormHandler(String tagName, NodeHandle nodeHandle, Parser parser, String output, ConversionOptions options, Context ctx, long depth, DomContext domCtx)
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `tagName` | `String` | Yes | The tag name |
| `nodeHandle` | `NodeHandle` | Yes | The node handle |
| `parser` | `Parser` | Yes | The parser |
| `output` | `String` | Yes | The output destination |
| `options` | `ConversionOptions` | Yes | The options to use |
| `ctx` | `Context` | Yes | The context |
| `depth` | `long` | Yes | The depth |
| `domCtx` | `DomContext` | Yes | The dom context |

**Returns:** `boolean`


---

### handleBlockquote()

Handle a `<blockquote>` element and convert to Markdown.

This handler processes blockquote elements including:
- Converting inline blockquotes by processing children as inline
- Handling nested blockquotes via blockquote_depth tracking
- Processing citation URLs from cite attribute
- Invoking visitor callbacks when the visitor feature is enabled
- Adding proper spacing and blockquote prefix formatting

**Signature:**

```java
public static void handleBlockquote(NodeHandle nodeHandle, HtmlTag tag, Parser parser, String output, ConversionOptions options, Context ctx, long depth, DomContext domCtx)
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `nodeHandle` | `NodeHandle` | Yes | The node handle |
| `tag` | `HtmlTag` | Yes | The h t m l tag |
| `parser` | `Parser` | Yes | The parser |
| `output` | `String` | Yes | The output destination |
| `options` | `ConversionOptions` | Yes | The options to use |
| `ctx` | `Context` | Yes | The context |
| `depth` | `long` | Yes | The depth |
| `domCtx` | `DomContext` | Yes | The dom context |

**Returns:** `void`


---

### handleCode()

Handle an inline `<code>` element and convert to Markdown.

This handler processes inline code elements including:
- Extracting code content and applying backtick delimiters
- Handling backticks in content by using multiple delimiters
- Invoking visitor callbacks when the visitor feature is enabled
- Generating appropriate markdown output with proper escaping

**Signature:**

```java
public static void handleCode(NodeHandle nodeHandle, HtmlTag tag, Parser parser, String output, ConversionOptions options, Context ctx, long depth, DomContext domCtx)
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `nodeHandle` | `NodeHandle` | Yes | The node handle |
| `tag` | `HtmlTag` | Yes | The h t m l tag |
| `parser` | `Parser` | Yes | The parser |
| `output` | `String` | Yes | The output destination |
| `options` | `ConversionOptions` | Yes | The options to use |
| `ctx` | `Context` | Yes | The context |
| `depth` | `long` | Yes | The depth |
| `domCtx` | `DomContext` | Yes | The dom context |

**Returns:** `void`


---

### handlePre()

Handle a `<pre>` element and convert to Markdown.

This handler processes code block elements including:
- Extracting language information from class attributes
- Processing whitespace and dedenting code content
- Supporting multiple code block styles (indented, backticks, tildes)
- Invoking visitor callbacks when the visitor feature is enabled
- Generating appropriate markdown output

**Signature:**

```java
public static void handlePre(NodeHandle nodeHandle, HtmlTag tag, Parser parser, String output, ConversionOptions options, Context ctx, long depth, DomContext domCtx)
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `nodeHandle` | `NodeHandle` | Yes | The node handle |
| `tag` | `HtmlTag` | Yes | The h t m l tag |
| `parser` | `Parser` | Yes | The parser |
| `output` | `String` | Yes | The output destination |
| `options` | `ConversionOptions` | Yes | The options to use |
| `ctx` | `Context` | Yes | The context |
| `depth` | `long` | Yes | The depth |
| `domCtx` | `DomContext` | Yes | The dom context |

**Returns:** `void`


---

### handleGraphic()

Handle a `<graphic>` element and convert to Markdown.

This handler processes graphic elements including:
- Extracting source from url, href, xlink:href, or src attributes
- Using alt attribute, with fallback to filename
- Collecting metadata when the metadata feature is enabled
- Invoking visitor callbacks when the visitor feature is enabled
- Generating appropriate markdown output

**Signature:**

```java
public static void handleGraphic(NodeHandle nodeHandle, HtmlTag tag, Parser parser, String output, ConversionOptions options, Context ctx, long depth, DomContext domCtx)
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `nodeHandle` | `NodeHandle` | Yes | The node handle |
| `tag` | `HtmlTag` | Yes | The h t m l tag |
| `parser` | `Parser` | Yes | The parser |
| `output` | `String` | Yes | The output destination |
| `options` | `ConversionOptions` | Yes | The options to use |
| `ctx` | `Context` | Yes | The context |
| `depth` | `long` | Yes | The depth |
| `domCtx` | `DomContext` | Yes | The dom context |

**Returns:** `void`


---

### handleImg()

Handle an `<img>` element and convert to Markdown.

This handler processes image elements including:
- Extracting src, alt, and title attributes
- Collecting metadata when the metadata feature is enabled
- Handling inline data URIs when the inline-images feature is enabled
- Invoking visitor callbacks when the visitor feature is enabled
- Generating appropriate markdown output

**Signature:**

```java
public static void handleImg(NodeHandle nodeHandle, HtmlTag tag, Parser parser, String output, ConversionOptions options, Context ctx, long depth, DomContext domCtx)
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `nodeHandle` | `NodeHandle` | Yes | The node handle |
| `tag` | `HtmlTag` | Yes | The h t m l tag |
| `parser` | `Parser` | Yes | The parser |
| `output` | `String` | Yes | The output destination |
| `options` | `ConversionOptions` | Yes | The options to use |
| `ctx` | `Context` | Yes | The context |
| `depth` | `long` | Yes | The depth |
| `domCtx` | `DomContext` | Yes | The dom context |

**Returns:** `void`


---

### handleLink()

Handle an `<a>` (link) element and convert to Markdown.

This handler processes link elements including:
- Extracting href and title attributes
- Detecting autolinks (where text equals href)
- Handling links that contain heading elements
- Processing complex link content (mixed block/inline)
- Invoking visitor callbacks when the visitor feature is enabled
- Collecting link metadata when the metadata feature is enabled
- Generating appropriate markdown link output

**Signature:**

```java
public static void handleLink(NodeHandle nodeHandle, HtmlTag tag, Parser parser, String output, ConversionOptions options, Context ctx, long depth, DomContext domCtx)
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `nodeHandle` | `NodeHandle` | Yes | The node handle |
| `tag` | `HtmlTag` | Yes | The h t m l tag |
| `parser` | `Parser` | Yes | The parser |
| `output` | `String` | Yes | The output destination |
| `options` | `ConversionOptions` | Yes | The options to use |
| `ctx` | `Context` | Yes | The context |
| `depth` | `long` | Yes | The depth |
| `domCtx` | `DomContext` | Yes | The dom context |

**Returns:** `void`


---

### dispatchInlineHandler()

Dispatches inline element handling to the appropriate handler.

This function routes inline HTML elements to their specialized handlers
based on tag name. It is designed to be called from the main `walk_node`
function in `converter.rs`.

# Routing Table

The following tag routes are supported:

| Tag(s) | Handler | Description |
|--------|---------|-------------|
| `strong`, `b` | emphasis | Bold/strong text formatting |
| `em`, `i` | emphasis | Italic/emphasis text formatting |
| `a` | link | Hyperlinks and anchors |
| `code`, `kbd`, `samp` | code | Inline code and keyboard input |
| `mark`, `del`, `s`, `ins`, `u`, `small`, `sub`, `sup`, `var`, `dfn`, `abbr`, `span` | semantic | Semantic formatting |
| `ruby`, `rb`, `rt`, `rp`, `rtc` | ruby | Ruby annotations (East Asian typography) |

# Return Value

Returns `true` if the tag was recognized and handled, `false` otherwise.
This allows the caller to distinguish between:
- Handled inline elements (return `true`)
- Unhandled elements (return `false`) that should be processed as text or passed through

# Usage in converter.rs

```text
if crate::converter::inline::dispatch_inline_handler(
    &tag_name,
    &node_handle,
    parser,
    output,
    options,
    ctx,
    depth,
    dom_ctx,
) {
    return; // Element was handled, move to next sibling
}
// Element was not handled, process as default inline element
```

# Parameters

* `tag_name` - The normalized HTML tag name (lowercase)
* `node_handle` - The DOM node handle from the parser
* `parser` - Reference to the tl HTML parser
* `output` - Output buffer to write converted content to
* `options` - Conversion configuration options
* `ctx` - Processing context with state tracking
* `depth` - Current DOM tree depth for recursion tracking
* `dom_ctx` - DOM context for accessing tree structure

For `<strong>Bold text</strong>`, the dispatcher:
1. Recognizes "strong" tag
2. Routes to emphasis handler
3. Returns `true`
4. Emphasis handler outputs `**Bold text**` to output buffer

For `<span>Normal text</span>`, the dispatcher:
1. Fails to recognize "span" tag
2. Returns `false`
3. Caller processes as default inline content

**Signature:**

```java
public static boolean dispatchInlineHandler(String tagName, NodeHandle nodeHandle, Parser parser, String output, ConversionOptions options, Context ctx, long depth, DomContext domCtx)
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `tagName` | `String` | Yes | The normalized HTML tag name (lowercase) |
| `nodeHandle` | `NodeHandle` | Yes | The DOM node handle from the parser |
| `parser` | `Parser` | Yes | Reference to the tl HTML parser |
| `output` | `String` | Yes | Output buffer to write converted content to |
| `options` | `ConversionOptions` | Yes | Conversion configuration options |
| `ctx` | `Context` | Yes | Processing context with state tracking |
| `depth` | `long` | Yes | Current DOM tree depth for recursion tracking |
| `domCtx` | `DomContext` | Yes | DOM context for accessing tree structure |

**Returns:** `boolean`


---

### calculateListContinuationIndent()

Calculate indentation level for list item continuations.

Returns the number of 4-space indent groups needed for list continuations.

List continuations (block elements inside list items) need special indentation:
- Base indentation: (depth - 1) groups (for the nesting level)
- Content indentation: depth groups (for the list item content)
- Combined formula: (2 * depth - 1) groups of 4 spaces each

**Signature:**

```java
public static long calculateListContinuationIndent(long depth)
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `depth` | `long` | Yes | The depth |

**Returns:** `long`


---

### isLooseList()

Check if a list (ul or ol) is "loose".

A loose list is one where any list item contains block-level elements
like paragraphs (<p>). In loose lists, all items should have blank line
separation (ending with \n\n) regardless of their own content.

**Signature:**

```java
public static boolean isLooseList(NodeHandle nodeHandle, Parser parser, DomContext domCtx)
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `nodeHandle` | `NodeHandle` | Yes | The node handle |
| `parser` | `Parser` | Yes | The parser |
| `domCtx` | `DomContext` | Yes | The dom context |

**Returns:** `boolean`


---

### addListContinuationIndent()

Add list continuation indentation to output.

Used when block elements (like <p> or <div>) appear inside list items.
Adds appropriate line separation and indentation to continue the list item.

**Signature:**

```java
public static void addListContinuationIndent(String output, long listDepth, boolean blankLine, ConversionOptions options)
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `output` | `String` | Yes | The output string to append to |
| `listDepth` | `long` | Yes | Current list nesting depth |
| `blankLine` | `boolean` | Yes | If true, adds blank line separation (\n\n); if false, single newline (\n) |
| `options` | `ConversionOptions` | Yes | The options to use |

**Returns:** `void`


---

### continuationIndentString()

Calculate the indentation string for list continuations based on depth and options.

**Signature:**

```java
public static Optional<String> continuationIndentString(long listDepth, ConversionOptions options)
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `listDepth` | `long` | Yes | The list depth |
| `options` | `ConversionOptions` | Yes | The options to use |

**Returns:** `Optional<String>`


---

### addListLeadingSeparator()

Add appropriate leading separator before a list.

Lists need different separators depending on context:
- In table cells: <br> tag if there's already content
- Outside lists: blank line (\n\n) if needed
- Inside list items: blank line before nested list

**Signature:**

```java
public static void addListLeadingSeparator(String output, Context ctx)
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `output` | `String` | Yes | The output destination |
| `ctx` | `Context` | Yes | The context |

**Returns:** `void`


---

### addNestedListTrailingSeparator()

Add appropriate trailing separator after a nested list.

Nested lists inside list items need trailing newlines to separate
from following content. In loose lists, use blank line (\n\n). In tight lists, single newline (\n).

**Signature:**

```java
public static void addNestedListTrailingSeparator(String output, Context ctx)
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `output` | `String` | Yes | The output destination |
| `ctx` | `Context` | Yes | The context |

**Returns:** `void`


---

### calculateListNestingDepth()

Calculate the nesting depth for a list.

If we're in a list but NOT in a list item, this is incorrectly nested HTML
and we need to increment the depth. If in a list item, the depth was already
incremented by the <li> element.

**Signature:**

```java
public static long calculateListNestingDepth(Context ctx)
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `ctx` | `Context` | Yes | The context |

**Returns:** `long`


---

### isListItem()

Check if a node is a list item element.

**Signature:**

```java
public static boolean isListItem(NodeHandle nodeHandle, Parser parser, DomContext domCtx)
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `nodeHandle` | `NodeHandle` | Yes | The node handle |
| `parser` | `Parser` | Yes | The parser |
| `domCtx` | `DomContext` | Yes | The dom context |

**Returns:** `boolean`


---

### processListChildren()

Process a list's children, tracking which items had block elements.

This is used to determine proper spacing between list items.
Returns true if the last processed item had block children.

**Signature:**

```java
public static void processListChildren(NodeHandle nodeHandle, Parser parser, String output, ConversionOptions options, Context ctx, long depth, boolean isOrdered, boolean isLoose, long nestedDepth, long startCounter, DomContext domCtx)
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `nodeHandle` | `NodeHandle` | Yes | The node handle |
| `parser` | `Parser` | Yes | The parser |
| `output` | `String` | Yes | The output destination |
| `options` | `ConversionOptions` | Yes | The options to use |
| `ctx` | `Context` | Yes | The context |
| `depth` | `long` | Yes | The depth |
| `isOrdered` | `boolean` | Yes | The is ordered |
| `isLoose` | `boolean` | Yes | The is loose |
| `nestedDepth` | `long` | Yes | The nested depth |
| `startCounter` | `long` | Yes | The start counter |
| `domCtx` | `DomContext` | Yes | The dom context |

**Returns:** `void`


---

### dispatchListHandler()

Dispatches list element handling to the appropriate handler.

Returns `true` if the element was handled, `false` otherwise.

# Supported Elements

- `ol`: Ordered list - routed to `ordered.handle`
- `ul`: Unordered list - routed to `unordered.handle`
- `li`: List item - routed to `item.handle_li`
- `dl`: Definition list - routed to `definition.handle_dl`
- `dt`: Definition term - routed to `definition.handle_dt`
- `dd`: Definition description - routed to `definition.handle_dd`

**Signature:**

```java
public static boolean dispatchListHandler(String tagName, NodeHandle nodeHandle, HtmlTag tag, Parser parser, String output, ConversionOptions options, Context ctx, long depth, DomContext domCtx)
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `tagName` | `String` | Yes | The tag name |
| `nodeHandle` | `NodeHandle` | Yes | The node handle |
| `tag` | `HtmlTag` | Yes | The h t m l tag |
| `parser` | `Parser` | Yes | The parser |
| `output` | `String` | Yes | The output destination |
| `options` | `ConversionOptions` | Yes | The options to use |
| `ctx` | `Context` | Yes | The context |
| `depth` | `long` | Yes | The depth |
| `domCtx` | `DomContext` | Yes | The dom context |

**Returns:** `boolean`


---

### convertHtml()

Converts HTML to Markdown using the provided conversion options.

This is the main entry point for HTML to Markdown conversion.

**Signature:**

```java
public static String convertHtml(String html, ConversionOptions options) throws Error
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `html` | `String` | Yes | The html |
| `options` | `ConversionOptions` | Yes | The options to use |

**Returns:** `String`

**Errors:** Throws `ErrorException`.


---

### convertHtmlWithVisitor()

Converts HTML to Markdown with a custom visitor for callbacks during traversal.

This variant allows passing a visitor that will receive callbacks for each node
during the tree walk, enabling custom processing or analysis.

**Signature:**

```java
public static String convertHtmlWithVisitor(String html, ConversionOptions options, VisitorHandle visitor) throws Error
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `html` | `String` | Yes | The html |
| `options` | `ConversionOptions` | Yes | The options to use |
| `visitor` | `Optional<VisitorHandle>` | No | The visitor handle |

**Returns:** `String`

**Errors:** Throws `ErrorException`.


---

### dispatchMediaHandler()

Dispatches media element handling to the appropriate handler.

This function routes media-related HTML elements to their specialized handlers
based on tag name. It is designed to be called from the main `walk_node`
function in `converter.rs`.

# Routing Table

The following tag routes are supported:

| Tag(s) | Handler | Description |
|--------|---------|-------------|
| `iframe` | embedded | Embedded content frames |
| `video` | embedded | Video elements |
| `audio` | embedded | Audio elements |
| `picture` | embedded | Responsive image containers |
| `svg` | svg | SVG image elements |
| `math` | svg | MathML elements |

# Return Value

Returns `true` if the tag was recognized and handled, `false` otherwise.

**Signature:**

```java
public static boolean dispatchMediaHandler(String tagName, NodeHandle nodeHandle, Parser parser, String output, ConversionOptions options, Context ctx, long depth, DomContext domCtx)
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `tagName` | `String` | Yes | The tag name |
| `nodeHandle` | `NodeHandle` | Yes | The node handle |
| `parser` | `Parser` | Yes | The parser |
| `output` | `String` | Yes | The output destination |
| `options` | `ConversionOptions` | Yes | The options to use |
| `ctx` | `Context` | Yes | The context |
| `depth` | `long` | Yes | The depth |
| `domCtx` | `DomContext` | Yes | The dom context |

**Returns:** `boolean`


---

### extractPlainText()

Extract plain text from a parsed DOM tree.

Walks the tree collecting visible text with structural whitespace:
- Block elements get blank-line separation
- `<br>` becomes a newline, `<hr>` a blank line
- `<pre>` preserves internal whitespace
- `<img>` outputs alt text (unless `skip_images` is set)
- `<script>`, `<style>`, `<head>`, `<template>`, `<noscript>` are skipped
- Tables: cells separated by tab, rows by newline
- Inline elements are recursed without markers

**Signature:**

```java
public static String extractPlainText(VDom dom, Parser parser, ConversionOptions options)
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `dom` | `VDom` | Yes | The v dom |
| `parser` | `Parser` | Yes | The parser |
| `options` | `ConversionOptions` | Yes | The options to use |

**Returns:** `String`


---

### handleDfn()

Handles the `<dfn>` element.

A dfn element marks a term that is being defined. The content represents
the term, and its definition would typically appear in surrounding context.
It is rendered as emphasized (italic) text.

# Behavior

- Content is collected from children
- Non-empty content is wrapped with the configured emphasis symbol (default: `*`)
- Inline suffix handling is applied (e.g., footnote references)

**Signature:**

```java
public static void handleDfn(String tagName, NodeHandle nodeHandle, Parser parser, String output, ConversionOptions options, Context ctx, long depth, DomContext domCtx)
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `tagName` | `String` | Yes | The  tag name |
| `nodeHandle` | `NodeHandle` | Yes | The node handle |
| `parser` | `Parser` | Yes | The parser |
| `output` | `String` | Yes | The output destination |
| `options` | `ConversionOptions` | Yes | The options to use |
| `ctx` | `Context` | Yes | The context |
| `depth` | `long` | Yes | The depth |
| `domCtx` | `DomContext` | Yes | The dom context |

**Returns:** `void`


---

### handleAbbr()

Handles the `<abbr>` element.

An abbr element marks an abbreviation or acronym. The `title` attribute
provides the expansion of the abbreviation, which is appended in parentheses
if present.

# Behavior

- Content is collected from children
- Non-empty content is output as-is
- If `title` attribute exists, it is appended in parentheses: `abbr (title)`

Produces: `HTML (HyperText Markup Language)`

**Signature:**

```java
public static void handleAbbr(String tagName, NodeHandle nodeHandle, Parser parser, String output, ConversionOptions options, Context ctx, long depth, DomContext domCtx)
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `tagName` | `String` | Yes | The  tag name |
| `nodeHandle` | `NodeHandle` | Yes | The node handle |
| `parser` | `Parser` | Yes | The parser |
| `output` | `String` | Yes | The output destination |
| `options` | `ConversionOptions` | Yes | The options to use |
| `ctx` | `Context` | Yes | The context |
| `depth` | `long` | Yes | The depth |
| `domCtx` | `DomContext` | Yes | The dom context |

**Returns:** `void`


---

### handleTimeData()

Handles the `<time>` and `<data>` elements.

Time and data elements contain machine-readable content in their attributes
and human-readable content in their text. For Markdown purposes, we output
only the human-readable text content, as Markdown doesn't have a way to
preserve machine-readable metadata.

# Behavior

- Content is extracted from children and output as-is
- Attributes (datetime, value) are not rendered in Markdown output

**Signature:**

```java
public static void handleTimeData(String tagName, NodeHandle nodeHandle, Parser parser, String output, ConversionOptions options, Context ctx, long depth, DomContext domCtx)
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `tagName` | `String` | Yes | The  tag name |
| `nodeHandle` | `NodeHandle` | Yes | The node handle |
| `parser` | `Parser` | Yes | The parser |
| `output` | `String` | Yes | The output destination |
| `options` | `ConversionOptions` | Yes | The options to use |
| `ctx` | `Context` | Yes | The context |
| `depth` | `long` | Yes | The depth |
| `domCtx` | `DomContext` | Yes | The dom context |

**Returns:** `void`


---

### handleCite()

Handles the `<cite>` element.

A cite element marks the title of a cited work (book, article, website, etc.).
It is rendered as emphasized (italic) text in block mode, or as plain text in inline mode.

# Behavior

- **Block mode**: Content is wrapped with emphasis markers (default: `*`)
- **Inline mode**: Content is output as-is without formatting

**Signature:**

```java
public static void handleCite(String tagName, NodeHandle nodeHandle, Parser parser, String output, ConversionOptions options, Context ctx, long depth, DomContext domCtx)
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `tagName` | `String` | Yes | The  tag name |
| `nodeHandle` | `NodeHandle` | Yes | The node handle |
| `parser` | `Parser` | Yes | The parser |
| `output` | `String` | Yes | The output destination |
| `options` | `ConversionOptions` | Yes | The options to use |
| `ctx` | `Context` | Yes | The context |
| `depth` | `long` | Yes | The depth |
| `domCtx` | `DomContext` | Yes | The dom context |

**Returns:** `void`


---

### handleQ()

Handles the `<q>` element.

A q element marks an inline quotation. In Markdown, it is rendered as
quoted text enclosed in double quotes. Backslashes and quotes within
the content are escaped.

# Behavior

- **Block mode**: Content is wrapped in escaped double quotes: `"content"`
- **Inline mode**: Content is output as-is without quotes

# Escaping

Internal backslashes and double quotes are escaped:
- `\` → `\\`
- `"` → `\"`

**Signature:**

```java
public static void handleQ(String tagName, NodeHandle nodeHandle, Parser parser, String output, ConversionOptions options, Context ctx, long depth, DomContext domCtx)
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `tagName` | `String` | Yes | The  tag name |
| `nodeHandle` | `NodeHandle` | Yes | The node handle |
| `parser` | `Parser` | Yes | The parser |
| `output` | `String` | Yes | The output destination |
| `options` | `ConversionOptions` | Yes | The options to use |
| `ctx` | `Context` | Yes | The context |
| `depth` | `long` | Yes | The depth |
| `domCtx` | `DomContext` | Yes | The dom context |

**Returns:** `void`


---

### handleHgroup()

Handles the `<hgroup>` element.

An hgroup element groups related headings together (e.g., a title and subtitle).
In Markdown, we simply process all children sequentially, allowing nested
headings to maintain their individual formatting.

# Behavior

- Children are processed sequentially in the current context
- No special formatting is applied at the hgroup level

**Signature:**

```java
public static void handleHgroup(String tagName, NodeHandle nodeHandle, Parser parser, String output, ConversionOptions options, Context ctx, long depth, DomContext domCtx)
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `tagName` | `String` | Yes | The  tag name |
| `nodeHandle` | `NodeHandle` | Yes | The node handle |
| `parser` | `Parser` | Yes | The parser |
| `output` | `String` | Yes | The output destination |
| `options` | `ConversionOptions` | Yes | The options to use |
| `ctx` | `Context` | Yes | The context |
| `depth` | `long` | Yes | The depth |
| `domCtx` | `DomContext` | Yes | The dom context |

**Returns:** `void`


---

### handleDl()

Handles the `<dl>` element.

A definition list contains terms and their definitions. Terms and definitions
are output as plain blocks without Pandoc-style colon syntax, since standard
Markdown and GFM do not support definition lists.

# Behavior

- **Inline mode**: Children are processed inline without block spacing
- **Block mode**: Content is collected and wrapped with proper spacing

**Signature:**

```java
public static void handleDl(String tagName, NodeHandle nodeHandle, Parser parser, String output, ConversionOptions options, Context ctx, long depth, DomContext domCtx)
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `tagName` | `String` | Yes | The  tag name |
| `nodeHandle` | `NodeHandle` | Yes | The node handle |
| `parser` | `Parser` | Yes | The parser |
| `output` | `String` | Yes | The output destination |
| `options` | `ConversionOptions` | Yes | The options to use |
| `ctx` | `Context` | Yes | The context |
| `depth` | `long` | Yes | The depth |
| `domCtx` | `DomContext` | Yes | The dom context |

**Returns:** `void`


---

### handleDt()

Handles the `<dt>` element.

A dt element contains a term being defined. Terms are output on their own line,
with definitions following on subsequent lines.

# Behavior

- **Inline mode**: Content is output as-is
- **Block mode**: Content is followed by a newline

**Signature:**

```java
public static void handleDt(String tagName, NodeHandle nodeHandle, Parser parser, String output, ConversionOptions options, Context ctx, long depth, DomContext domCtx)
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `tagName` | `String` | Yes | The  tag name |
| `nodeHandle` | `NodeHandle` | Yes | The node handle |
| `parser` | `Parser` | Yes | The parser |
| `output` | `String` | Yes | The output destination |
| `options` | `ConversionOptions` | Yes | The options to use |
| `ctx` | `Context` | Yes | The context |
| `depth` | `long` | Yes | The depth |
| `domCtx` | `DomContext` | Yes | The dom context |

**Returns:** `void`


---

### handleDd()

Handles the `<dd>` element.

A dd element contains the definition for a term. It is output as a plain
block since standard Markdown and GFM do not support definition list syntax.

# Behavior

- **Inline mode**: Content is output as-is
- **Block mode**: Content is output as a block

**Signature:**

```java
public static void handleDd(String tagName, NodeHandle nodeHandle, Parser parser, String output, ConversionOptions options, Context ctx, long depth, DomContext domCtx)
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `tagName` | `String` | Yes | The  tag name |
| `nodeHandle` | `NodeHandle` | Yes | The node handle |
| `parser` | `Parser` | Yes | The parser |
| `output` | `String` | Yes | The output destination |
| `options` | `ConversionOptions` | Yes | The options to use |
| `ctx` | `Context` | Yes | The context |
| `depth` | `long` | Yes | The depth |
| `domCtx` | `DomContext` | Yes | The dom context |

**Returns:** `void`


---

### handleMenu()

Handles the `<menu>` element.

A menu element is a semantic list, typically used for command menus or
navigation. It is rendered as an unordered list with dashes.

# Behavior

- **Inline mode**: Children are processed inline without list formatting
- **Block mode**: Content is rendered as an unordered list
- Uses `-` as the list bullet (overrides configured bullets)
- Proper blank-line spacing is maintained

**Signature:**

```java
public static void handleMenu(String tagName, NodeHandle nodeHandle, Parser parser, String output, ConversionOptions options, Context ctx, long depth, DomContext domCtx)
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `tagName` | `String` | Yes | The  tag name |
| `nodeHandle` | `NodeHandle` | Yes | The node handle |
| `parser` | `Parser` | Yes | The parser |
| `output` | `String` | Yes | The output destination |
| `options` | `ConversionOptions` | Yes | The options to use |
| `ctx` | `Context` | Yes | The context |
| `depth` | `long` | Yes | The depth |
| `domCtx` | `DomContext` | Yes | The dom context |

**Returns:** `void`


---

### handleFigure()

Handles the `<figure>` element.

A figure element contains content (typically images) and optionally a figcaption.
The handler collects all content and cleans up extra line breaks.

# Behavior

- **Inline mode**: Children are processed inline without block spacing
- **Block mode**: Content is collected, line breaks normalized, and wrapped with blank lines
- **Image normalization**: Removes extra spaces before `![` to improve Markdown formatting

# Implementation Details

The handler performs the following on the collected content:
1. Normalizes newline + image sequences: `\n![` → `![`
2. Normalizes space + image sequences: ` ![` → `![`
3. Trims the final content and wraps it with blank lines

**Signature:**

```java
public static void handleFigure(String tagName, NodeHandle nodeHandle, Parser parser, String output, ConversionOptions options, Context ctx, long depth, DomContext domCtx)
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `tagName` | `String` | Yes | The  tag name |
| `nodeHandle` | `NodeHandle` | Yes | The node handle |
| `parser` | `Parser` | Yes | The parser |
| `output` | `String` | Yes | The output destination |
| `options` | `ConversionOptions` | Yes | The options to use |
| `ctx` | `Context` | Yes | The context |
| `depth` | `long` | Yes | The depth |
| `domCtx` | `DomContext` | Yes | The dom context |

**Returns:** `void`


---

### handleFigcaption()

Handles the `<figcaption>` element.

A figcaption element contains text that describes or supplements the figure.
It is rendered as emphasized (italic) text to distinguish it from regular content.

# Behavior

- Content is collected and trimmed
- Non-empty content is wrapped in `*text*` (emphasis) markers
- Proper spacing is maintained around the caption

# Implementation Details

The handler:
1. Collects and processes all children
2. Checks for existing output and adds spacing as needed
3. Wraps content in emphasis markers: `*caption*`
4. Ensures proper blank-line spacing after the caption

**Signature:**

```java
public static void handleFigcaption(String tagName, NodeHandle nodeHandle, Parser parser, String output, ConversionOptions options, Context ctx, long depth, DomContext domCtx)
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `tagName` | `String` | Yes | The  tag name |
| `nodeHandle` | `NodeHandle` | Yes | The node handle |
| `parser` | `Parser` | Yes | The parser |
| `output` | `String` | Yes | The output destination |
| `options` | `ConversionOptions` | Yes | The options to use |
| `ctx` | `Context` | Yes | The context |
| `depth` | `long` | Yes | The depth |
| `domCtx` | `DomContext` | Yes | The dom context |

**Returns:** `void`


---

### handleDetails()

Handles the `<details>` element.

A details element represents a disclosure widget that can be toggled
to show/hide additional content. In Markdown, it's rendered as a block
with all content visible.

# Behavior

- **Inline mode**: Children are processed inline without block spacing
- **Block mode**: Content is collected and wrapped with proper blank-line spacing
- **Empty content**: Skipped entirely

**Signature:**

```java
public static void handleDetails(String tagName, NodeHandle nodeHandle, Parser parser, String output, ConversionOptions options, Context ctx, long depth, DomContext domCtx)
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `tagName` | `String` | Yes | The  tag name |
| `nodeHandle` | `NodeHandle` | Yes | The node handle |
| `parser` | `Parser` | Yes | The parser |
| `output` | `String` | Yes | The output destination |
| `options` | `ConversionOptions` | Yes | The options to use |
| `ctx` | `Context` | Yes | The context |
| `depth` | `long` | Yes | The depth |
| `domCtx` | `DomContext` | Yes | The dom context |

**Returns:** `void`


---

### handleSummary()

Handles the `<summary>` element.

A summary element contains a caption for a details element.
It is rendered as strong (bold) text to distinguish it from regular content.

# Behavior

- **Inline mode**: Content is rendered inline without emphasis
- **Block mode**: Content is wrapped in strong markers (e.g., `**text**`)
- Uses the configured strong/emphasis symbol from ConversionOptions

# Implementation Details

The handler:
1. Creates a context with `in_strong: true` for nested formatting
2. Collects content from all children
3. Wraps non-empty content in strong markers (repeated twice per Markdown spec)

**Signature:**

```java
public static void handleSummary(String tagName, NodeHandle nodeHandle, Parser parser, String output, ConversionOptions options, Context ctx, long depth, DomContext domCtx)
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `tagName` | `String` | Yes | The  tag name |
| `nodeHandle` | `NodeHandle` | Yes | The node handle |
| `parser` | `Parser` | Yes | The parser |
| `output` | `String` | Yes | The output destination |
| `options` | `ConversionOptions` | Yes | The options to use |
| `ctx` | `Context` | Yes | The context |
| `depth` | `long` | Yes | The depth |
| `domCtx` | `DomContext` | Yes | The dom context |

**Returns:** `void`


---

### handleDialog()

Handles the `<dialog>` element.

A dialog element represents a modal dialog box. In Markdown, it's rendered
as a block container with content visible.

# Behavior

- **Inline mode**: Children are processed inline without block spacing
- **Block mode**: Content is processed and wrapped with proper blank lines
- Trailing whitespace is removed from collected content

# Implementation Details

The handler:
1. Marks the position in output before processing children
2. Processes all children in the normal context
3. Removes trailing spaces and tabs from the output
4. Ensures proper blank-line spacing after the dialog

**Signature:**

```java
public static void handleDialog(String tagName, NodeHandle nodeHandle, Parser parser, String output, ConversionOptions options, Context ctx, long depth, DomContext domCtx)
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `tagName` | `String` | Yes | The  tag name |
| `nodeHandle` | `NodeHandle` | Yes | The node handle |
| `parser` | `Parser` | Yes | The parser |
| `output` | `String` | Yes | The output destination |
| `options` | `ConversionOptions` | Yes | The options to use |
| `ctx` | `Context` | Yes | The context |
| `depth` | `long` | Yes | The depth |
| `domCtx` | `DomContext` | Yes | The dom context |

**Returns:** `void`


---

### dispatchSemanticHandler()

Dispatches semantic element handling to the appropriate handler.

This function routes semantic HTML5 elements to their specialized handlers
based on tag name. It is designed to be called from the main `walk_node`
function in `converter.rs`.

# Routing Table

The following tag routes are supported:
- **Sectioning**: article, section, nav, aside, header, footer, main
- **Figure**: figure, figcaption
- **Summary**: details, summary, dialog
- **Definition List**: hgroup, dl, dt, dd, menu
- **Attributes**: cite, q, abbr, dfn, time, data

**Returns:**

Returns `true` if the tag was successfully handled by a semantic handler,
`false` if the tag is not a semantic element and requires other handling.

**Signature:**

```java
public static boolean dispatchSemanticHandler(String tagName, NodeHandle nodeHandle, Parser parser, String output, ConversionOptions options, Context ctx, long depth, DomContext domCtx)
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `tagName` | `String` | Yes | The tag name |
| `nodeHandle` | `NodeHandle` | Yes | The node handle |
| `parser` | `Parser` | Yes | The parser |
| `output` | `String` | Yes | The output destination |
| `options` | `ConversionOptions` | Yes | The options to use |
| `ctx` | `Context` | Yes | The context |
| `depth` | `long` | Yes | The depth |
| `domCtx` | `DomContext` | Yes | The dom context |

**Returns:** `boolean`


---

### escapeLinkLabel()

Escape special characters in link labels.

Markdown link labels can contain brackets, which need careful escaping to avoid
being interpreted as nested links. This function escapes unescaped closing brackets
that would break the link syntax.

**Signature:**

```java
public static String escapeLinkLabel(String text)
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `text` | `String` | Yes | The text |

**Returns:** `String`


---

### escapeMalformedAngleBrackets()

Escape malformed angle brackets in markdown output.

Markdown uses `<...>` for automatic links. Angle brackets that don't form valid
link syntax should be escaped as `&lt;` to prevent parser confusion.

A valid tag must have:
- `<!` followed by `-` or alphabetic character (for comments/declarations)
- `</` followed by alphabetic character (for closing tags)
- `<?` (for processing instructions)
- `<` followed by alphabetic character (for opening tags)

**Signature:**

```java
public static Str escapeMalformedAngleBrackets(String input)
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `input` | `String` | Yes | The input data |

**Returns:** `Str`


---

### trimLineEndWhitespace()

Remove trailing spaces/tabs from every line while preserving newlines.

**Signature:**

```java
public static void trimLineEndWhitespace(String output)
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `output` | `String` | Yes | The output destination |

**Returns:** `void`


---

### truncateAtCharBoundary()

Truncate a string at a valid UTF-8 boundary.

**Signature:**

```java
public static void truncateAtCharBoundary(String value, long maxLen)
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `value` | `String` | Yes | The value |
| `maxLen` | `long` | Yes | The max len |

**Returns:** `void`


---

### normalizeHeadingText()

Normalize heading text by replacing newlines and extra whitespace.

Heading text should be on a single line in Markdown. This function collapses
any newlines and multiple spaces into single spaces.

**Signature:**

```java
public static Str normalizeHeadingText(String text)
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `text` | `String` | Yes | The text |

**Returns:** `Str`


---

### dedentCodeBlock()

Remove common leading whitespace from all lines in a code block.

This is useful when HTML authors indent `<pre>` content for readability,
so we can strip the shared indentation without touching meaningful spacing.

**Signature:**

```java
public static String dedentCodeBlock(String content)
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `content` | `String` | Yes | The content to process |

**Returns:** `String`


---

### floorCharBoundary()

Returns the largest valid char boundary index at or before `index`.

If `index` is already a char boundary it is returned unchanged.
Otherwise it walks backwards to find one.  Returns 0 if no boundary
is found before `index`.

**Signature:**

```java
public static long floorCharBoundary(String s, long index)
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `s` | `String` | Yes | The s |
| `index` | `long` | Yes | The index |

**Returns:** `long`


---

### handleVisitorElementStart()

Handles visitor callback for element start (before processing).

This function is called when entering an element during tree traversal,
before the element's content is processed. The visitor can:
- Continue with normal processing (Continue)
- Skip the element entirely (Skip)
- Provide custom output to replace the element (Custom)
- Signal an error (Error)

**Returns:**

`VisitAction` enum indicating what should happen next:
- `VisitAction.Continue` - Process element normally
- `VisitAction.Skip` - Skip element, don't process or call visit_element_end
- `VisitAction.Custom(output)` - Use custom output, skip normal processing
- `VisitAction.Error` - Stop processing with error

**Signature:**

```java
public static VisitAction handleVisitorElementStart(VisitorHandle visitorHandle, String tagName, NodeHandle nodeHandle, HtmlTag tag, Parser parser, String output, Context ctx, long depth, DomContext domCtx)
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `visitorHandle` | `VisitorHandle` | Yes | Reference to the visitor for callbacks |
| `tagName` | `String` | Yes | The normalized tag name being processed |
| `nodeHandle` | `NodeHandle` | Yes | Handle to the DOM node |
| `tag` | `HtmlTag` | Yes | Reference to the tag object |
| `parser` | `Parser` | Yes | Reference to the tl parser |
| `output` | `String` | Yes | Mutable reference to output string |
| `ctx` | `Context` | Yes | The context |
| `depth` | `long` | Yes | Current tree depth |
| `domCtx` | `DomContext` | Yes | Reference to DOM context for tree navigation |

**Returns:** `VisitAction`


---

### handleVisitorElementEnd()

Handles visitor callback for element end (after processing).

This function is called when exiting an element after its content has been processed.
The visitor can:
- Accept the output normally (Continue)
- Replace the output with custom content (Custom)
- Remove the output entirely (Skip)
- Signal an error (Error)

**Signature:**

```java
public static void handleVisitorElementEnd(VisitorHandle visitorHandle, String tagName, NodeHandle nodeHandle, HtmlTag tag, Parser parser, String output, long elementOutputStart, Context ctx, long depth, DomContext domCtx)
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `visitorHandle` | `VisitorHandle` | Yes | Reference to the visitor for callbacks |
| `tagName` | `String` | Yes | The normalized tag name that was processed |
| `nodeHandle` | `NodeHandle` | Yes | Handle to the DOM node |
| `tag` | `HtmlTag` | Yes | Reference to the tag object |
| `parser` | `Parser` | Yes | Reference to the tl parser |
| `output` | `String` | Yes | Mutable reference to output string |
| `elementOutputStart` | `long` | Yes | Byte position where this element's output started |
| `ctx` | `Context` | Yes | Reference to the conversion context |
| `depth` | `long` | Yes | Current tree depth |
| `domCtx` | `DomContext` | Yes | Reference to DOM context for tree navigation |

**Returns:** `void`


---

### escape()

Escape Markdown special characters in text.

**Returns:**

Escaped text

**Signature:**

```java
public static Str escape(String text, boolean escapeMisc, boolean escapeAsterisks, boolean escapeUnderscores, boolean escapeAscii)
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `text` | `String` | Yes | Text to escape |
| `escapeMisc` | `boolean` | Yes | Escape miscellaneous characters (`\` `&` `<` `` ` `` `[` `>` `~` `#` `=` `+` `\|` `-`) |
| `escapeAsterisks` | `boolean` | Yes | Escape asterisks (`*`) |
| `escapeUnderscores` | `boolean` | Yes | Escape underscores (`_`) |
| `escapeAscii` | `boolean` | Yes | Escape all ASCII punctuation (for `CommonMark` spec compliance) |

**Returns:** `Str`


---

### chomp()

Extract boundary whitespace from text (chomp).

Returns (prefix, suffix, `trimmed_text`) tuple.
Prefix/suffix are " " if original text had leading/trailing whitespace.
However, suffix is "" if the trailing whitespace is only newlines (not spaces/tabs).
This prevents trailing newlines from becoming trailing spaces in the output.
The trimmed text has all leading/trailing whitespace removed.

**Signature:**

```java
public static StrStrStr chomp(String text)
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `text` | `String` | Yes | The text |

**Returns:** `StrStrStr`


---

### normalizeWhitespace()

Normalize whitespace by collapsing consecutive spaces and tabs.

Multiple spaces and tabs are replaced with a single space.
Newlines are preserved.
Unicode spaces are normalized to ASCII spaces.

**Returns:**

Normalized text with collapsed spaces/tabs but preserved newlines

**Signature:**

```java
public static String normalizeWhitespace(String text)
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `text` | `String` | Yes | The text to normalize |

**Returns:** `String`


---

### normalizeWhitespaceCow()

Normalize whitespace in text, returning borrowed or owned result as needed.

This function optimizes memory by returning a borrowed reference when no normalization
is needed, and only allocating a new string when whitespace changes are necessary.

Multiple consecutive spaces, tabs, and Unicode space characters are replaced with
a single ASCII space. Newlines are preserved as-is.

**Returns:**

`Cow.Borrowed` if text is already normalized, or `Cow.Owned` with normalized text

**Signature:**

```java
public static Str normalizeWhitespaceCow(String text)
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `text` | `String` | Yes | The text to normalize |

**Returns:** `Str`


---

### decodeHtmlEntities()

Decode common HTML entities.

Decodes the most common HTML entities to their character equivalents:
- `&quot;` → `"`
- `&apos;` → `'`
- `&lt;` → `<`
- `&gt;` → `>`
- `&amp;` → `&` (must be last to avoid double-decoding)

**Returns:**

Text with entities decoded

**Signature:**

```java
public static String decodeHtmlEntities(String text)
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `text` | `String` | Yes | Text containing HTML entities |

**Returns:** `String`


---

### decodeHtmlEntitiesCow()

Decode HTML entities in text, returning borrowed or owned result as needed.

This function optimizes memory by returning a borrowed reference when no HTML
entities are present, and only allocating a new string when entity decoding
is necessary.

Decodes common HTML entities like:
- `&quot;` → `"`
- `&apos;` → `'`
- `&lt;` → `<`
- `&gt;` → `>`
- `&amp;` → `&` (decoded last to avoid double-decoding)

**Returns:**

`Cow.Borrowed` if no entities found, or `Cow.Owned` with entities decoded

**Signature:**

```java
public static Str decodeHtmlEntitiesCow(String text)
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `text` | `String` | Yes | Text potentially containing HTML entities |

**Returns:** `Str`


---

### underline()

Underline text with a character.

**Signature:**

```java
public static String underline(String text, String padChar)
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `text` | `String` | Yes | The text |
| `padChar` | `String` | Yes | The pad char |

**Returns:** `String`


---

### indent()

Indent text with a string prefix.

**Signature:**

```java
public static String indent(String text, long level, String indentStr)
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `text` | `String` | Yes | The text |
| `level` | `long` | Yes | The level |
| `indentStr` | `String` | Yes | The indent str |

**Returns:** `String`


---

### buildDocumentStructure()

Build a `DocumentStructure` from an already-parsed `tl.VDom`.

Walks the DOM once, mapping HTML elements to semantic `NodeContent` variants,
tracking parent/child relationships, extracting inline `TextAnnotation`s, and
constructing heading-based `Group` nodes.

**Signature:**

```java
public static DocumentStructure buildDocumentStructure(VDom dom)
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `dom` | `VDom` | Yes | The v dom |

**Returns:** `DocumentStructure`


---

### buildNodeContext()

Build a `NodeContext` from current parsing state.

Creates a complete `NodeContext` suitable for passing to visitor callbacks.
This function collects metadata about the current node from various sources:
- Tag name and attributes from the HTML element
- Depth and parent information from the DOM tree
- Index among siblings for positional awareness
- Inline/block classification

# Parameters

- `node_type`: Coarse-grained classification (Link, Image, Heading, etc.)
- `tag_name`: Raw HTML tag name (e.g., "div", "h1", "custom-element")
- `attributes`: All HTML attributes as key-value pairs
- `depth`: Nesting depth in the DOM tree (0 = root)
- `index_in_parent`: Zero-based index among siblings
- `parent_tag`: Parent element's tag name (None if root)
- `is_inline`: Whether this element is treated as inline vs block

**Returns:**

A fully populated `NodeContext` ready for visitor dispatch.

# Performance

This function performs minimal allocations:
- Clones `tag_name` (typically 2-10 bytes)
- Clones `parent_tag` if present (typically 2-10 bytes)
- Clones the attributes `BTreeMap` (heap allocation if non-empty)

For text nodes and simple elements without attributes, allocations are minimal.

**Signature:**

```java
public static NodeContext buildNodeContext(NodeType nodeType, String tagName, Map<String, String> attributes, long depth, long indexInParent, String parentTag, boolean isInline)
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `nodeType` | `NodeType` | Yes | Coarse-grained classification (Link, Image, Heading, etc.) |
| `tagName` | `String` | Yes | Raw HTML tag name (e.g., "div", "h1", "custom-element") |
| `attributes` | `Map<String, String>` | Yes | All HTML attributes as key-value pairs |
| `depth` | `long` | Yes | Nesting depth in the DOM tree (0 = root) |
| `indexInParent` | `long` | Yes | Zero-based index among siblings |
| `parentTag` | `Optional<String>` | No | Parent element's tag name (None if root) |
| `isInline` | `boolean` | Yes | Whether this element is treated as inline vs block |

**Returns:** `NodeContext`


---

### convert()

Convert HTML to Markdown, returning a `ConversionResult` with content, metadata, images,
and warnings.

**Errors:**

Returns an error if HTML parsing fails or if the input contains invalid UTF-8.

**Signature:**

```java
public static ConversionResult convert(String html, ConversionOptions options) throws Error
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `html` | `String` | Yes | The HTML string to convert |
| `options` | `Optional<ConversionOptions>` | No | Optional conversion options (defaults to `default options`) |

**Returns:** `ConversionResult`

**Errors:** Throws `ErrorException`.


---

### convertWithVisitor()

Internal: convert with visitor support. Used by FFI crate.
Will be removed when convert() accepts visitor parameter directly.

**Signature:**

```java
public static String convertWithVisitor(String html, ConversionOptions options, VisitorHandle visitor) throws Error
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `html` | `String` | Yes | The html |
| `options` | `Optional<ConversionOptions>` | No | The options to use |
| `visitor` | `Optional<VisitorHandle>` | No | The visitor handle |

**Returns:** `String`

**Errors:** Throws `ErrorException`.


---

### conversionOptionsFromJson()

Parse JSON string into `ConversionOptions`.

Deserializes a JSON string into a full set of conversion options.
The JSON can be either a complete or partial options object.

**Returns:**

Fully populated `ConversionOptions` with defaults applied to any unspecified values

**Errors:**

Returns `ConversionError.ConfigError` if JSON parsing fails or contains invalid option values

**Signature:**

```java
public static ConversionOptions conversionOptionsFromJson(String json) throws Error
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `json` | `String` | Yes | JSON string representing conversion options |

**Returns:** `ConversionOptions`

**Errors:** Throws `ErrorException`.


---

### conversionOptionsUpdateFromJson()

Parse JSON string into partial `ConversionOptions` update.

Deserializes a JSON string into a partial set of conversion options.
Only specified options are included; unspecified options are None.

**Returns:**

`ConversionOptionsUpdate` with only specified fields populated

**Errors:**

Returns `ConversionError.ConfigError` if JSON parsing fails or contains invalid option values

**Signature:**

```java
public static ConversionOptionsUpdate conversionOptionsUpdateFromJson(String json) throws Error
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `json` | `String` | Yes | JSON string representing partial conversion options |

**Returns:** `ConversionOptionsUpdate`

**Errors:** Throws `ErrorException`.


---

### inlineImageConfigFromJson()

Parse JSON string into `InlineImageConfig` (requires `inline-images` feature).

Deserializes a JSON string into inline image extraction configuration.
The JSON can be either a complete or partial configuration object.

**Returns:**

Fully populated `InlineImageConfig` with defaults applied to any unspecified values

**Errors:**

Returns `ConversionError.ConfigError` if JSON parsing fails or contains invalid configuration values

**Signature:**

```java
public static InlineImageConfig inlineImageConfigFromJson(String json) throws Error
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `json` | `String` | Yes | JSON string representing inline image configuration |

**Returns:** `InlineImageConfig`

**Errors:** Throws `ErrorException`.


---

### metadataConfigFromJson()

Parse JSON string into `MetadataConfig` (requires `metadata` feature).

Deserializes a JSON string into metadata extraction configuration.
The JSON can be either a complete or partial configuration object.

**Returns:**

Fully populated `MetadataConfig` with defaults applied to any unspecified values

**Errors:**

Returns `ConversionError.ConfigError` if JSON parsing fails or contains invalid configuration values

**Signature:**

```java
public static MetadataConfig metadataConfigFromJson(String json) throws Error
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `json` | `String` | Yes | JSON string representing metadata extraction configuration |

**Returns:** `MetadataConfig`

**Errors:** Throws `ErrorException`.


---

## Types

### ConversionOptions

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

#### Methods

##### defaultOptions()

**Signature:**

```java
public static ConversionOptions defaultOptions()
```

##### builder()

Create a new builder with default values.

**Signature:**

```java
public static ConversionOptionsBuilder builder()
```

##### applyUpdate()

Apply a partial update to these conversion options.

**Signature:**

```java
public void applyUpdate(ConversionOptionsUpdate update)
```

##### fromUpdate()

Create from a partial update, applying to defaults.

**Signature:**

```java
public static ConversionOptions fromUpdate(ConversionOptionsUpdate update)
```

##### from()

**Signature:**

```java
public static ConversionOptions from(ConversionOptionsUpdate update)
```


---

### ConversionResult

The primary result of HTML conversion and extraction.

Contains the converted text output, optional structured document tree,
metadata, extracted tables, images, and processing warnings.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `content` | `Optional<String>` | `null` | Converted text output (markdown, djot, or plain text). `None` when `output_format` is set to `OutputFormat.None`, indicating extraction-only mode. |
| `document` | `Optional<DocumentStructure>` | `null` | Structured document tree with semantic elements. Populated when `include_document_structure` is `True` in options. |
| `metadata` | `HtmlMetadata` | — | Extracted HTML metadata (title, OG, links, images, structured data). |
| `tables` | `List<TableData>` | `Collections.emptyList()` | Extracted tables with structured cell data and markdown representation. |
| `images` | `List<InlineImage>` | `Collections.emptyList()` | Extracted inline images (data URIs and SVGs). Populated when `extract_images` is `True` in options. |
| `warnings` | `List<ProcessingWarning>` | `Collections.emptyList()` | Non-fatal processing warnings. |


---

### Context

Conversion context that tracks state during HTML to Markdown conversion.

This context is passed through the recursive tree walker and maintains information
about the current position in the document tree, nesting levels, and enabled features.


---

### ConversionOptionsBuilder

Builder for `ConversionOptions`.

All fields start with default values. Call `.build()` to produce the final options.

#### Methods

##### stripTags()

Set the list of HTML tag names whose content is stripped from output.

**Signature:**

```java
public ConversionOptionsBuilder stripTags(List<String> tags)
```

##### preserveTags()

Set the list of HTML tag names that are preserved verbatim in output.

**Signature:**

```java
public ConversionOptionsBuilder preserveTags(List<String> tags)
```

##### keepInlineImagesIn()

Set the list of HTML tag names whose `<img>` children are kept inline.

**Signature:**

```java
public ConversionOptionsBuilder keepInlineImagesIn(List<String> tags)
```

##### preprocessing()

Set the pre-processing options applied to the HTML before conversion.

**Signature:**

```java
public ConversionOptionsBuilder preprocessing(PreprocessingOptions preprocessing)
```

##### build()

Build the final `ConversionOptions`.

**Signature:**

```java
public ConversionOptions build()
```


---

### DjotRenderer

Renderer for Djot lightweight markup output.

#### Methods

##### emphasis()

**Signature:**

```java
public String emphasis(String content)
```

##### strong()

**Signature:**

```java
public String strong(String content, String symbol)
```

##### strikethrough()

**Signature:**

```java
public String strikethrough(String content)
```

##### highlight()

**Signature:**

```java
public String highlight(String content)
```

##### inserted()

**Signature:**

```java
public String inserted(String content)
```

##### subscript()

**Signature:**

```java
public String subscript(String content, String customSymbol)
```

##### superscript()

**Signature:**

```java
public String superscript(String content, String customSymbol)
```

##### spanWithAttributes()

**Signature:**

```java
public String spanWithAttributes(String content, List<String> classes, String id)
```

##### divWithAttributes()

**Signature:**

```java
public String divWithAttributes(String content, List<String> classes)
```

##### isDjot()

**Signature:**

```java
public boolean isDjot()
```


---

### DocumentMetadata

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

### DocumentNode

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

### DocumentStructure

A structured document tree representing the semantic content of an HTML document.

Uses a flat node array with index-based parent/child references for efficient traversal.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `nodes` | `List<DocumentNode>` | — | All nodes in document reading order. |
| `sourceFormat` | `Optional<String>` | `null` | The source format (always "html" for this library). |


---

### DomContext

DOM context that provides efficient access to parent/child relationships and text content.

This context is built once during conversion and provides O(1) access to node relationships
via precomputed maps. It also includes an LRU cache for text content extraction.


---

### FormatRenderer

Trait for format-specific rendering of inline elements.

Implementations provide the syntax for emphasis, strong, strikethrough, etc.
in their respective output formats.

#### Methods

##### emphasis()

Render emphasis (em, i elements)

**Signature:**

```java
public String emphasis(String content)
```

##### strong()

Render strong emphasis (strong, b elements)

**Signature:**

```java
public String strong(String content, String symbol)
```

##### strikethrough()

Render strikethrough (del, s elements)

**Signature:**

```java
public String strikethrough(String content)
```

##### highlight()

Render highlight (mark element)

**Signature:**

```java
public String highlight(String content)
```

##### inserted()

Render inserted text (ins element)

**Signature:**

```java
public String inserted(String content)
```

##### subscript()

Render subscript (sub element)

**Signature:**

```java
public String subscript(String content, String customSymbol)
```

##### superscript()

Render superscript (sup element)

**Signature:**

```java
public String superscript(String content, String customSymbol)
```

##### spanWithAttributes()

Render span with attributes (for Djot: [text]{.class})

**Signature:**

```java
public String spanWithAttributes(String content, List<String> classes, String id)
```

##### divWithAttributes()

Render div with attributes (for Djot: .: class)

**Signature:**

```java
public String divWithAttributes(String content, List<String> classes)
```

##### isDjot()

Check if this is Djot format

**Signature:**

```java
public boolean isDjot()
```


---

### GridCell

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

### HeaderMetadata

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

#### Methods

##### isValid()

Validate that the header level is within valid range (1-6).

**Returns:**

`true` if level is 1-6, `false` otherwise.

**Signature:**

```java
public boolean isValid()
```


---

### HtmlMetadata

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

### HtmlVisitor

Visitor trait for HTML→Markdown conversion.

Implement this trait to customize the conversion behavior for any HTML element type.
All methods have default implementations that return `VisitResult.Continue`, allowing
selective override of only the elements you care about.

# Method Naming Convention

- `visit_*_start`: Called before entering an element (pre-order traversal)
- `visit_*_end`: Called after exiting an element (post-order traversal)
- `visit_*`: Called for specific element types (e.g., `visit_link`, `visit_image`)

# Execution Order

For a typical element like `<div><p>text</p></div>`:
1. `visit_element_start` for `<div>`
2. `visit_element_start` for `<p>`
3. `visit_text` for "text"
4. `visit_element_end` for `<p>`
5. `visit_element_end` for `</div>`

# Performance Notes

- `visit_text` is the most frequently called method (~100+ times per document)
- Return `VisitResult.Continue` quickly for elements you don't need to customize
- Avoid heavy computation in visitor methods; consider caching if needed

#### Methods

##### visitElementStart()

Called before entering any element.

This is the first callback invoked for every HTML element, allowing
visitors to implement generic element handling before tag-specific logic.

**Signature:**

```java
public VisitResult visitElementStart(NodeContext ctx)
```

##### visitElementEnd()

Called after exiting any element.

Receives the default markdown output that would be generated.
Visitors can inspect or replace this output.

**Signature:**

```java
public VisitResult visitElementEnd(NodeContext ctx, String output)
```

##### visitText()

Visit text nodes (most frequent callback - ~100+ per document).

**Signature:**

```java
public VisitResult visitText(NodeContext ctx, String text)
```

##### visitLink()

Visit anchor links `<a href="...">`.

**Signature:**

```java
public VisitResult visitLink(NodeContext ctx, String href, String text, String title)
```

##### visitImage()

Visit images `<img src="...">`.

**Signature:**

```java
public VisitResult visitImage(NodeContext ctx, String src, String alt, String title)
```

##### visitHeading()

Visit heading elements `<h1>` through `<h6>`.

**Signature:**

```java
public VisitResult visitHeading(NodeContext ctx, int level, String text, String id)
```

##### visitCodeBlock()

Visit code blocks `<pre><code>`.

**Signature:**

```java
public VisitResult visitCodeBlock(NodeContext ctx, String lang, String code)
```

##### visitCodeInline()

Visit inline code `<code>`.

**Signature:**

```java
public VisitResult visitCodeInline(NodeContext ctx, String code)
```

##### visitListItem()

Visit list items `<li>`.

**Signature:**

```java
public VisitResult visitListItem(NodeContext ctx, boolean ordered, String marker, String text)
```

##### visitListStart()

Called before processing a list `<ul>` or `<ol>`.

**Signature:**

```java
public VisitResult visitListStart(NodeContext ctx, boolean ordered)
```

##### visitListEnd()

Called after processing a list `</ul>` or `</ol>`.

**Signature:**

```java
public VisitResult visitListEnd(NodeContext ctx, boolean ordered, String output)
```

##### visitTableStart()

Called before processing a table `<table>`.

**Signature:**

```java
public VisitResult visitTableStart(NodeContext ctx)
```

##### visitTableRow()

Visit table rows `<tr>`.

**Signature:**

```java
public VisitResult visitTableRow(NodeContext ctx, List<String> cells, boolean isHeader)
```

##### visitTableEnd()

Called after processing a table `</table>`.

**Signature:**

```java
public VisitResult visitTableEnd(NodeContext ctx, String output)
```

##### visitBlockquote()

Visit blockquote elements `<blockquote>`.

**Signature:**

```java
public VisitResult visitBlockquote(NodeContext ctx, String content, long depth)
```

##### visitStrong()

Visit strong/bold elements `<strong>`, `<b>`.

**Signature:**

```java
public VisitResult visitStrong(NodeContext ctx, String text)
```

##### visitEmphasis()

Visit emphasis/italic elements `<em>`, `<i>`.

**Signature:**

```java
public VisitResult visitEmphasis(NodeContext ctx, String text)
```

##### visitStrikethrough()

Visit strikethrough elements `<s>`, `<del>`, `<strike>`.

**Signature:**

```java
public VisitResult visitStrikethrough(NodeContext ctx, String text)
```

##### visitUnderline()

Visit underline elements `<u>`, `<ins>`.

**Signature:**

```java
public VisitResult visitUnderline(NodeContext ctx, String text)
```

##### visitSubscript()

Visit subscript elements `<sub>`.

**Signature:**

```java
public VisitResult visitSubscript(NodeContext ctx, String text)
```

##### visitSuperscript()

Visit superscript elements `<sup>`.

**Signature:**

```java
public VisitResult visitSuperscript(NodeContext ctx, String text)
```

##### visitMark()

Visit mark/highlight elements `<mark>`.

**Signature:**

```java
public VisitResult visitMark(NodeContext ctx, String text)
```

##### visitLineBreak()

Visit line break elements `<br>`.

**Signature:**

```java
public VisitResult visitLineBreak(NodeContext ctx)
```

##### visitHorizontalRule()

Visit horizontal rule elements `<hr>`.

**Signature:**

```java
public VisitResult visitHorizontalRule(NodeContext ctx)
```

##### visitCustomElement()

Visit custom elements (web components) or unknown tags.

**Signature:**

```java
public VisitResult visitCustomElement(NodeContext ctx, String tagName, String html)
```

##### visitDefinitionListStart()

Visit definition list `<dl>`.

**Signature:**

```java
public VisitResult visitDefinitionListStart(NodeContext ctx)
```

##### visitDefinitionTerm()

Visit definition term `<dt>`.

**Signature:**

```java
public VisitResult visitDefinitionTerm(NodeContext ctx, String text)
```

##### visitDefinitionDescription()

Visit definition description `<dd>`.

**Signature:**

```java
public VisitResult visitDefinitionDescription(NodeContext ctx, String text)
```

##### visitDefinitionListEnd()

Called after processing a definition list `</dl>`.

**Signature:**

```java
public VisitResult visitDefinitionListEnd(NodeContext ctx, String output)
```

##### visitForm()

Visit form elements `<form>`.

**Signature:**

```java
public VisitResult visitForm(NodeContext ctx, String action, String method)
```

##### visitInput()

Visit input elements `<input>`.

**Signature:**

```java
public VisitResult visitInput(NodeContext ctx, String inputType, String name, String value)
```

##### visitButton()

Visit button elements `<button>`.

**Signature:**

```java
public VisitResult visitButton(NodeContext ctx, String text)
```

##### visitAudio()

Visit audio elements `<audio>`.

**Signature:**

```java
public VisitResult visitAudio(NodeContext ctx, String src)
```

##### visitVideo()

Visit video elements `<video>`.

**Signature:**

```java
public VisitResult visitVideo(NodeContext ctx, String src)
```

##### visitIframe()

Visit iframe elements `<iframe>`.

**Signature:**

```java
public VisitResult visitIframe(NodeContext ctx, String src)
```

##### visitDetails()

Visit details elements `<details>`.

**Signature:**

```java
public VisitResult visitDetails(NodeContext ctx, boolean open)
```

##### visitSummary()

Visit summary elements `<summary>`.

**Signature:**

```java
public VisitResult visitSummary(NodeContext ctx, String text)
```

##### visitFigureStart()

Visit figure elements `<figure>`.

**Signature:**

```java
public VisitResult visitFigureStart(NodeContext ctx)
```

##### visitFigcaption()

Visit figcaption elements `<figcaption>`.

**Signature:**

```java
public VisitResult visitFigcaption(NodeContext ctx, String text)
```

##### visitFigureEnd()

Called after processing a figure `</figure>`.

**Signature:**

```java
public VisitResult visitFigureEnd(NodeContext ctx, String output)
```


---

### ImageMetadata

Image metadata with source and dimensions.

Captures `<img>` elements and inline `<svg>` elements with metadata
for image analysis and optimization.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `src` | `String` | — | Image source (URL, data URI, or SVG content identifier) |
| `alt` | `Optional<String>` | `null` | Alternative text from alt attribute (for accessibility) |
| `title` | `Optional<String>` | `null` | Title attribute (often shown as tooltip) |
| `dimensions` | `Optional<U32U32>` | `null` | Image dimensions as (width, height) if available |
| `imageType` | `ImageType` | — | Image type classification |
| `attributes` | `Map<String, String>` | — | Additional HTML attributes |


---

### ImageMetadataPayload

Payload type for image metadata extraction.


---

### InlineCollectorHandle

Handle type for inline image collector when feature is enabled.


---

### InlineImageConfig

Inline image configuration that specifies contexts where images remain as markdown links.

This is a wrapper type that provides semantic clarity for the vector of element
names where inline images should be preserved.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `keepInlineImagesIn` | `List<String>` | `Collections.emptyList()` | HTML elements where images should remain as markdown links (not converted to alt text) |

#### Methods

##### fromElements()

Create a new inline image configuration from a list of element names.

**Signature:**

```java
public static InlineImageConfig fromElements(List<String> elements)
```

##### addElement()

Add an element name to the list of elements where images are kept inline.

**Signature:**

```java
public void addElement(String element)
```

##### shouldKeepImages()

Check if a given element should keep images inline.

**Returns:**

`true` if the element is in the configured list, `false` otherwise

**Signature:**

```java
public boolean shouldKeepImages(String element)
```

##### defaultOptions()

**Signature:**

```java
public static InlineImageConfig defaultOptions()
```


---

### LinkMetadata

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

#### Methods

##### classifyLink()

Classify a link based on href value.

**Returns:**

Appropriate `LinkType` based on protocol and content.

**Signature:**

```java
public static LinkType classifyLink(String href)
```


---

### MarkdownRenderer

Renderer for standard Markdown output.

#### Methods

##### emphasis()

**Signature:**

```java
public String emphasis(String content)
```

##### strong()

**Signature:**

```java
public String strong(String content, String symbol)
```

##### strikethrough()

**Signature:**

```java
public String strikethrough(String content)
```

##### highlight()

**Signature:**

```java
public String highlight(String content)
```

##### inserted()

**Signature:**

```java
public String inserted(String content)
```

##### subscript()

**Signature:**

```java
public String subscript(String content, String customSymbol)
```

##### superscript()

**Signature:**

```java
public String superscript(String content, String customSymbol)
```

##### spanWithAttributes()

**Signature:**

```java
public String spanWithAttributes(String content, List<String> classes, String id)
```

##### divWithAttributes()

**Signature:**

```java
public String divWithAttributes(String content, List<String> classes)
```

##### isDjot()

**Signature:**

```java
public boolean isDjot()
```


---

### MetadataCollector

Internal metadata collector for single-pass extraction.

Follows a pattern for efficient metadata extraction during tree traversal.
Maintains state for:
- Document metadata from head elements
- Header hierarchy tracking
- Link accumulation
- Structured data collection
- Language and directionality attributes

# Architecture

The collector is designed to be:
- **Performant**: Pre-allocated collections, minimal cloning
- **Single-pass**: Collects during main tree walk without separate passes
- **Optional**: Zero overhead when disabled via feature flags
- **Type-safe**: Strict separation of collection and result types


---

### MetadataConfig

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

#### Methods

##### defaultOptions()

Create default metadata configuration.

Defaults to extracting all metadata types with 1MB limit on structured data.

**Signature:**

```java
public static MetadataConfig defaultOptions()
```

##### anyEnabled()

Check if any metadata extraction is enabled.

Returns `true` if at least one extraction category is enabled, `false` if all are disabled.
This is useful for early exit optimization when the application doesn't need metadata.

**Returns:**

`true` if any of the extraction flags are enabled, `false` if all are disabled.

**Signature:**

```java
public boolean anyEnabled()
```

##### applyUpdate()

Apply a partial update to this metadata configuration.

Any specified fields in the update (Some values) will override the current values.
Unspecified fields (None) are left unchanged. This allows selective modification
of configuration without affecting unrelated settings.

**Signature:**

```java
public void applyUpdate(MetadataConfigUpdate update)
```

##### fromUpdate()

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

##### from()

**Signature:**

```java
public static MetadataConfig from(MetadataConfigUpdate update)
```


---

### NodeContext

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

### PreprocessingOptions

HTML preprocessing options for document cleanup before conversion.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `enabled` | `boolean` | `true` | Enable HTML preprocessing globally |
| `preset` | `PreprocessingPreset` | `PreprocessingPreset.STANDARD` | Preprocessing preset level (Minimal, Standard, Aggressive) |
| `removeNavigation` | `boolean` | `true` | Remove navigation elements (nav, breadcrumbs, menus, sidebars) |
| `removeForms` | `boolean` | `true` | Remove form elements (forms, inputs, buttons, etc.) |

#### Methods

##### defaultOptions()

**Signature:**

```java
public static PreprocessingOptions defaultOptions()
```

##### applyUpdate()

Apply a partial update to these preprocessing options.

Any specified fields in the update will override the current values.
Unspecified fields (None) are left unchanged.

**Signature:**

```java
public void applyUpdate(PreprocessingOptionsUpdate update)
```

##### fromUpdate()

Create new preprocessing options from a partial update.

Creates a new `PreprocessingOptions` struct with defaults, then applies the update.
Fields not specified in the update keep their default values.

**Returns:**

New `PreprocessingOptions` with specified updates applied to defaults

**Signature:**

```java
public static PreprocessingOptions fromUpdate(PreprocessingOptionsUpdate update)
```

##### from()

**Signature:**

```java
public static PreprocessingOptions from(PreprocessingOptionsUpdate update)
```


---

### ProcessingWarning

A non-fatal warning generated during HTML processing.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `message` | `String` | — | Human-readable warning message. |
| `kind` | `WarningKind` | — | The category of warning. |


---

### ReferenceCollector

Collects link/image references during conversion and produces a reference
definitions section at the end of the document.

#### Methods

##### getOrInsert()

Register a URL (and optional title) and return its 1-based reference number.

If the same URL+title pair was already registered, the existing number is returned.

**Signature:**

```java
public long getOrInsert(String url, String title)
```

##### finish()

Produce the reference definitions section.

Returns an empty string when no references were collected.

**Signature:**

```java
public String finish()
```


---

### ReferenceCollectorHandle

Shared handle for passing the collector through the conversion context.


---

### StructureCollector

Incremental builder for `DocumentStructure` during a single DOM walk.

#### Methods

##### pushHeading()

Record a heading element.

Creates a `NodeContent.Group` (which owns all subsequent sibling content until a
heading of equal or higher rank closes it) followed by a `NodeContent.Heading` child.

Returns the index of the **heading** node (the group node is one before it).

**Signature:**

```java
public int pushHeading(byte level, String text, String id)
```

##### pushParagraph()

Record a paragraph element.

Returns the node index.

**Signature:**

```java
public int pushParagraph(String text)
```

##### pushListStart()

Open a list container.

Returns the node index; call `push_list_end` to close it.

**Signature:**

```java
public int pushListStart(boolean ordered)
```

##### pushListEnd()

Close the innermost open list container.

**Signature:**

```java
public void pushListEnd()
```

##### pushListItem()

Record a list item under the current open list.

If there is no open list, the item is parented under the current section/container.
Returns the node index.

**Signature:**

```java
public int pushListItem(String text)
```

##### pushTable()

Record a table.

Returns the node index.

**Signature:**

```java
public int pushTable(TableGrid grid)
```

##### pushImage()

Record an image element.

Returns the node index.

**Signature:**

```java
public int pushImage(String src, String alt)
```

##### pushCode()

Record a code block.

Returns the node index.

**Signature:**

```java
public int pushCode(String text, String language)
```

##### pushQuoteStart()

Open a blockquote container.

Returns the node index; call `push_quote_end` to close it.

**Signature:**

```java
public int pushQuoteStart()
```

##### pushQuoteEnd()

Close the innermost open blockquote container.

**Signature:**

```java
public void pushQuoteEnd()
```

##### pushRawBlock()

Record a raw block (e.g. preserved `<script>` or `<style>` content).

Returns the node index.

**Signature:**

```java
public int pushRawBlock(String format, String content)
```

##### finish()

Consume the collector and return the completed `DocumentStructure`.

**Signature:**

```java
public DocumentStructure finish()
```

##### defaultOptions()

**Signature:**

```java
public static StructureCollector defaultOptions()
```


---

### StructureCollectorHandle

Shared mutable handle used in `crate.converter.Context`.


---

### StructuredData

Structured data block (JSON-LD, Microdata, or RDFa).

Represents machine-readable structured data found in the document.
JSON-LD blocks are collected as raw JSON strings for flexibility.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `dataType` | `StructuredDataType` | — | Type of structured data (JSON-LD, Microdata, RDFa) |
| `rawJson` | `String` | — | Raw JSON string (for JSON-LD) or serialized representation |
| `schemaType` | `Optional<String>` | `null` | Schema type if detectable (e.g., "Article", "Event", "Product") |


---

### TableData

A top-level extracted table with both structured data and markdown representation.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `grid` | `TableGrid` | — | The structured table grid. |
| `markdown` | `String` | — | The markdown rendering of this table. |


---

### TableGrid

A structured table grid with cell-level data including spans.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `rows` | `int` | — | Number of rows. |
| `cols` | `int` | — | Number of columns. |
| `cells` | `List<GridCell>` | `Collections.emptyList()` | All cells in the table (may be fewer than rows*cols due to spans). |


---

### TableScan

Scan results for a table element.

Contains metadata about table structure to determine optimal rendering:
- Row counts for consistency checking
- Presence of headers, captions, and nested tables
- Presence of colspan/rowspan (spanning cells)
- Link and text content counts

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `rowCounts` | `List<Long>` | `Collections.emptyList()` | Number of cells in each row |
| `hasSpan` | `boolean` | — | Whether any cells have colspan or rowspan attributes |
| `hasHeader` | `boolean` | — | Whether the table has header cells (th elements or role="head") |
| `hasCaption` | `boolean` | — | Whether the table has a caption element |
| `nestedTableCount` | `long` | — | Number of nested tables found inside this table |
| `linkCount` | `long` | — | Count of anchor elements in the table |
| `hasText` | `boolean` | — | Whether the table contains text content (not empty) |


---

### TextAnnotation

An inline text annotation with byte-range offsets.

Annotations describe formatting (bold, italic, etc.) and links within a node's text content.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `start` | `int` | — | Start byte offset (inclusive) into the parent node's text. |
| `end` | `int` | — | End byte offset (exclusive) into the parent node's text. |
| `kind` | `AnnotationKind` | — | The type of annotation. |


---

### VisitorHandle

Type alias for a visitor handle (Rc-wrapped `RefCell` for interior mutability).

This allows visitors to be passed around and shared while still being mutable.


---

## Enums

### VisitAction

Result of visitor element start callback indicating what should happen next.

| Value | Description |
|-------|-------------|
| `CONTINUE` | Continue with normal element processing |
| `SKIP` | Skip the element entirely (don't process children or call visit_element_end) |
| `CUSTOM` | Custom output was provided, skip normal processing |
| `ERROR` | Error occurred during visitor callback |


---

### TextDirection

Text directionality of document content.

Corresponds to the HTML `dir` attribute and `bdi` element directionality.

| Value | Description |
|-------|-------------|
| `LEFT_TO_RIGHT` | Left-to-right text flow (default for Latin scripts) |
| `RIGHT_TO_LEFT` | Right-to-left text flow (Hebrew, Arabic, Urdu, etc.) |
| `AUTO` | Automatic directionality detection |


---

### LinkType

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

### ImageType

Image source classification for proper handling and processing.

Determines whether an image is embedded (data URI), inline SVG, external, or relative.

| Value | Description |
|-------|-------------|
| `DATA_URI` | Data URI embedded image (base64 or other encoding) |
| `INLINE_SVG` | Inline SVG element |
| `EXTERNAL` | External image URL (http/https) |
| `RELATIVE` | Relative image path |


---

### StructuredDataType

Structured data format type.

Identifies the schema/format used for structured data markup.

| Value | Description |
|-------|-------------|
| `JSON_LD` | JSON-LD (JSON for Linking Data) script blocks |
| `MICRODATA` | HTML5 Microdata attributes (itemscope, itemtype, itemprop) |
| `RDFA` | RDF in Attributes (RDFa) markup |


---

### PreprocessingPreset

HTML preprocessing aggressiveness level.

Controls the extent of cleanup performed before conversion. Higher levels remove more elements.

| Value | Description |
|-------|-------------|
| `MINIMAL` | Minimal cleanup. Remove only essential noise (scripts, styles). |
| `STANDARD` | Standard cleanup. Default. Removes navigation, forms, and other auxiliary content. |
| `AGGRESSIVE` | Aggressive cleanup. Remove extensive non-content elements and structure. |


---

### HeadingStyle

Heading style options for Markdown output.

Controls how headings (h1-h6) are rendered in the output Markdown.

| Value | Description |
|-------|-------------|
| `UNDERLINED` | Underlined style (=== for h1, --- for h2). |
| `ATX` | ATX style (# for h1, ## for h2, etc.). Default. |
| `ATX_CLOSED` | ATX closed style (# title #, with closing hashes). |


---

### ListIndentType

List indentation character type.

Controls whether list items are indented with spaces or tabs.

| Value | Description |
|-------|-------------|
| `SPACES` | Use spaces for indentation. Default. Width controlled by `list_indent_width`. |
| `TABS` | Use tabs for indentation. |


---

### WhitespaceMode

Whitespace handling strategy during conversion.

Determines how sequences of whitespace characters (spaces, tabs, newlines) are processed.

| Value | Description |
|-------|-------------|
| `NORMALIZED` | Collapse multiple whitespace characters to single spaces. Default. Matches browser behavior. |
| `STRICT` | Preserve all whitespace exactly as it appears in the HTML. |


---

### NewlineStyle

Line break syntax in Markdown output.

Controls how soft line breaks (from `<br>` or line breaks in source) are rendered.

| Value | Description |
|-------|-------------|
| `SPACES` | Two trailing spaces at end of line. Default. Standard Markdown syntax. |
| `BACKSLASH` | Backslash at end of line. Alternative Markdown syntax. |


---

### CodeBlockStyle

Code block fence style in Markdown output.

Determines how code blocks (`<pre><code>`) are rendered in Markdown.

| Value | Description |
|-------|-------------|
| `INDENTED` | Indented code blocks (4 spaces). `CommonMark` standard. |
| `BACKTICKS` | Fenced code blocks with backticks (```). Default (GFM). Supports language hints. |
| `TILDES` | Fenced code blocks with tildes (~~~). Supports language hints. |


---

### HighlightStyle

Highlight rendering style for `<mark>` elements.

Controls how highlighted text is rendered in Markdown output.

| Value | Description |
|-------|-------------|
| `DOUBLE_EQUAL` | Double equals syntax (==text==). Default. Pandoc-compatible. |
| `HTML` | Preserve as HTML (==text==). Original HTML tag. |
| `BOLD` | Render as bold (**text**). Uses strong emphasis. |
| `NONE` | Strip formatting, render as plain text. No markup. |


---

### LinkStyle

Link rendering style in Markdown output.

Controls whether links and images use inline `[text](url)` syntax or
reference-style `[text][1]` syntax with definitions collected at the end.

| Value | Description |
|-------|-------------|
| `INLINE` | Inline links: `[text](url)`. Default. |
| `REFERENCE` | Reference-style links: `[text][1]` with `[1]: url` at end of document. |


---

### OutputFormat

Output format for conversion.

Specifies the target markup language format for the conversion output.

| Value | Description |
|-------|-------------|
| `MARKDOWN` | Standard Markdown (CommonMark compatible). Default. |
| `DJOT` | Djot lightweight markup language. |
| `PLAIN` | Plain text output (no markup, visible text only). |


---

### NodeContent

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
| `METADATA_BLOCK` | A block of key-value metadata pairs (from `<head>` meta tags). — Fields: `entries`: `List<StringString>` |
| `GROUP` | A section grouping container (auto-generated from heading hierarchy). — Fields: `label`: `String`, `headingLevel`: `byte`, `headingText`: `String` |


---

### AnnotationKind

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

### WarningKind

Categories of processing warnings.

| Value | Description |
|-------|-------------|
| `IMAGE_EXTRACTION_FAILED` | An image could not be extracted (e.g. invalid data URI, unsupported format). |
| `ENCODING_FALLBACK` | The input encoding was not recognized; fell back to UTF-8. |
| `TRUNCATED_INPUT` | The input was truncated due to size limits. |
| `MALFORMED_HTML` | The HTML was malformed but processing continued with best effort. |
| `SANITIZATION_APPLIED` | Sanitization was applied to remove potentially unsafe content. |


---

### NodeType

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

### VisitResult

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

### VisitorDispatch

Result of dispatching a visitor callback.

This enum represents the outcome of a visitor callback dispatch,
providing a more ergonomic interface for control flow than the
raw `VisitResult` type.

| Value | Description |
|-------|-------------|
| `CONTINUE` | Continue with default conversion behavior |
| `CUSTOM` | Replace default output with custom markdown — Fields: `0`: `String` |
| `SKIP` | Skip this element entirely (don't output anything) |
| `PRESERVE_HTML` | Preserve original HTML (don't convert to markdown) |


---

## Errors

### ConversionError

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

