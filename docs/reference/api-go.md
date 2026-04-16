---
title: "Go API Reference"
---

## Go API Reference <span class="version-badge">v3.2.0</span>

### Functions

#### TableTotalColumns()

Calculate total columns in a table.

Scans all rows and cells to determine the maximum column count,
accounting for colspan values.

**Returns:**
Maximum column count (minimum 1, maximum MAX_TABLE_COLS)

**Signature:**

```go
func TableTotalColumns(nodeHandle NodeHandle, parser Parser, domCtx DomContext) int
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `NodeHandle` | `NodeHandle` | Yes | Handle to the table element |
| `Parser` | `Parser` | Yes | HTML parser instance |
| `DomCtx` | `DomContext` | Yes | DOM context for tag name resolution |

**Returns:** `int`


---

#### HandleTable()

Convert an entire table element to Markdown.

Main entry point for table conversion. Analyzes table structure to determine
if it should be rendered as a Markdown table or converted to list format.
Handles layout tables, blank tables, and tables with semantic meaning.
Integrates with visitor pattern for custom table handling.

**Signature:**

```go
func HandleTable(nodeHandle NodeHandle, parser Parser, output string, options ConversionOptions, ctx Context, domCtx DomContext, depth int)
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `NodeHandle` | `NodeHandle` | Yes | Handle to the table element |
| `Parser` | `Parser` | Yes | HTML parser instance |
| `Output` | `string` | Yes | Mutable string to append table content |
| `Options` | `ConversionOptions` | Yes | Conversion options |
| `Ctx` | `Context` | Yes | Conversion context (visitor, etc) |
| `DomCtx` | `DomContext` | Yes | DOM context |
| `Depth` | `int` | Yes | Nesting depth |

**Returns:** ``


---

#### HandleCaption()

Handles caption elements within tables.

Extracts text content from the caption and formats it as italicized text
with escaped hyphens to prevent Markdown table separator interpretation.

**Signature:**

```go
func HandleCaption(nodeHandle NodeHandle, parser Parser, output string, options ConversionOptions, ctx Context, depth int, domCtx DomContext)
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `NodeHandle` | `NodeHandle` | Yes | Handle to the caption element |
| `Parser` | `Parser` | Yes | HTML parser instance |
| `Output` | `string` | Yes | Output string to append caption text to |
| `Options` | `ConversionOptions` | Yes | Conversion options |
| `Ctx` | `Context` | Yes | Conversion context |
| `Depth` | `int` | Yes | Current recursion depth |
| `DomCtx` | `DomContext` | Yes | DOM context for tag name resolution |

**Returns:** ``


---

#### GetColspan()

Get colspan attribute value from an element.

Reads the colspan attribute from a table cell, with bounds checking
to prevent memory exhaustion attacks.

**Returns:**
The colspan value (minimum 1, maximum MAX_TABLE_COLS)

**Signature:**

```go
func GetColspan(nodeHandle NodeHandle, parser Parser) int
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `NodeHandle` | `NodeHandle` | Yes | Handle to the cell element |
| `Parser` | `Parser` | Yes | HTML parser instance |

**Returns:** `int`


---

#### GetColspanRowspan()

Get both colspan and rowspan in a single lookup.

More efficient than calling get_colspan and a separate rowspan lookup.

**Returns:**
A tuple of (colspan, rowspan), both minimum 1 and maximum MAX_TABLE_COLS

**Signature:**

```go
func GetColspanRowspan(nodeHandle NodeHandle, parser Parser) UsizeUsize
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `NodeHandle` | `NodeHandle` | Yes | Handle to the cell element |
| `Parser` | `Parser` | Yes | HTML parser instance |

**Returns:** `UsizeUsize`


---

#### CollectTableCells()

Collect table cells (td/th) from a row element.

Extracts only the direct cell children of a row, filtering by tag name.

**Signature:**

```go
func CollectTableCells(nodeHandle NodeHandle, parser Parser, domCtx DomContext, cells []NodeHandle)
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `NodeHandle` | `NodeHandle` | Yes | Handle to the row element |
| `Parser` | `Parser` | Yes | HTML parser instance |
| `DomCtx` | `DomContext` | Yes | DOM context for tag name resolution |
| `Cells` | `[]NodeHandle` | Yes | Mutable vector to populate with cell handles |

**Returns:** ``


---

#### ConvertTableCell()

Convert a table cell (td or th) to Markdown format.

Processes cell content and renders it with pipe delimiters for Markdown tables.
Handles colspan by adding extra pipes, and escapes pipes in cell content.

**Signature:**

```go
func ConvertTableCell(nodeHandle NodeHandle, parser Parser, output string, options ConversionOptions, ctx Context, tagName string, domCtx DomContext)
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `NodeHandle` | `NodeHandle` | Yes | Handle to the cell element |
| `Parser` | `Parser` | Yes | HTML parser instance |
| `Output` | `string` | Yes | Mutable string to append cell content |
| `Options` | `ConversionOptions` | Yes | Conversion options (escape settings, br_in_tables) |
| `Ctx` | `Context` | Yes | Conversion context (visitor, etc) |
| `TagName` | `string` | Yes | Tag name (for consistency, not used) |
| `DomCtx` | `DomContext` | Yes | DOM context for content extraction |

**Returns:** ``


---

#### AppendLayoutRow()

Append a layout table row as a list item.

For tables used for visual layout, converts rows to list items
instead of table format for better readability.

**Signature:**

```go
func AppendLayoutRow(rowHandle NodeHandle, parser Parser, output string, options ConversionOptions, ctx Context, domCtx DomContext)
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `RowHandle` | `NodeHandle` | Yes | Handle to the row element |
| `Parser` | `Parser` | Yes | HTML parser instance |
| `Output` | `string` | Yes | Mutable string to append content |
| `Options` | `ConversionOptions` | Yes | Conversion options |
| `Ctx` | `Context` | Yes | Conversion context |
| `DomCtx` | `DomContext` | Yes | DOM context |

**Returns:** ``


---

#### ConvertTableRow()

Convert a table row (tr) to Markdown format.

Processes all cells in a row, handling colspan and rowspan for proper
column alignment. Renders header separator row after the first row.
Integrates with visitor pattern for custom row handling.

**Signature:**

```go
func ConvertTableRow(nodeHandle NodeHandle, parser Parser, output string, options ConversionOptions, ctx Context, rowIndex int, hasSpan bool, rowspanTracker []*int, totalCols int, headerCols int, domCtx DomContext, depth int, isHeader bool)
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `NodeHandle` | `NodeHandle` | Yes | Handle to the row element |
| `Parser` | `Parser` | Yes | HTML parser instance |
| `Output` | `string` | Yes | Mutable string to append row content |
| `Options` | `ConversionOptions` | Yes | Conversion options |
| `Ctx` | `Context` | Yes | Conversion context (visitor, etc) |
| `RowIndex` | `int` | Yes | Index of this row in the table |
| `HasSpan` | `bool` | Yes | Whether table has colspan/rowspan |
| `RowspanTracker` | `[]*int` | Yes | Mutable array tracking rowspan remainder for each column |
| `TotalCols` | `int` | Yes | Total columns in the table |
| `HeaderCols` | `int` | Yes | Columns to render in separator row |
| `DomCtx` | `DomContext` | Yes | DOM context |
| `Depth` | `int` | Yes | Nesting depth |
| `IsHeader` | `bool` | Yes | Whether this is a header row |

**Returns:** ``


---

#### ScanTable()

Scan a table element for structural metadata.

Analyzes the table to determine characteristics that influence rendering:

- Whether to render as a Markdown table or layout table
- If spanning cells are present
- If the table has semantic meaning (headers, captions)

**Signature:**

```go
func ScanTable(nodeHandle NodeHandle, parser Parser, domCtx DomContext) TableScan
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `NodeHandle` | `NodeHandle` | Yes | Handle to the table element |
| `Parser` | `Parser` | Yes | HTML parser instance |
| `DomCtx` | `DomContext` | Yes | DOM context for tag name resolution |

**Returns:** `TableScan`


---

#### DispatchTableHandler()

Dispatches table element handling to the main convert_table function.

## Usage in converter.rs

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

```go
func DispatchTableHandler(tagName string, nodeHandle NodeHandle, parser Parser, output string, options ConversionOptions, ctx Context, depth int, domCtx DomContext) bool
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `TagName` | `string` | Yes | The tag name |
| `NodeHandle` | `NodeHandle` | Yes | The node handle |
| `Parser` | `Parser` | Yes | The parser |
| `Output` | `string` | Yes | The output destination |
| `Options` | `ConversionOptions` | Yes | The options to use |
| `Ctx` | `Context` | Yes | The context |
| `Depth` | `int` | Yes | The depth |
| `DomCtx` | `DomContext` | Yes | The dom context |

**Returns:** `bool`


---

### DispatchBlockHandler()

Dispatches block element handling to the appropriate handler.

This function is designed to be called from the main walk_node function
in converter.rs once the module is refactored. It returns `true` if the
element was handled, `false` otherwise.

## Usage in converter.rs

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

```go
func DispatchBlockHandler(tagName string, nodeHandle NodeHandle, parser Parser, output string, options ConversionOptions, ctx Context, depth int, domCtx DomContext) bool
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `TagName` | `string` | Yes | The tag name |
| `NodeHandle` | `NodeHandle` | Yes | The node handle |
| `Parser` | `Parser` | Yes | The parser |
| `Output` | `string` | Yes | The output destination |
| `Options` | `ConversionOptions` | Yes | The options to use |
| `Ctx` | `Context` | Yes | The context |
| `Depth` | `int` | Yes | The depth |
| `DomCtx` | `DomContext` | Yes | The dom context |

**Returns:** `bool`


---

### Handle()

Dispatcher for form elements.

Routes all form-related elements to their respective handlers.

**Signature:**

```go
func Handle(tagName string, nodeHandle NodeHandle, parser Parser, output string, options ConversionOptions, ctx Context, depth int, domCtx DomContext)
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `TagName` | `string` | Yes | The tag name |
| `NodeHandle` | `NodeHandle` | Yes | The node handle |
| `Parser` | `Parser` | Yes | The parser |
| `Output` | `string` | Yes | The output destination |
| `Options` | `ConversionOptions` | Yes | The options to use |
| `Ctx` | `Context` | Yes | The context |
| `Depth` | `int` | Yes | The depth |
| `DomCtx` | `DomContext` | Yes | The dom context |

**Returns:** ``


---

#### DispatchFormHandler()

Dispatches form element handling to the appropriate handler.

This function routes form-related HTML elements to their specialized handlers
based on tag name. It is designed to be called from the main `walk_node`
function in `converter.rs`.

## Routing Table

The following tag routes are supported:

- **Containers**: form, fieldset, legend, label
- **Inputs**: input, textarea, select, option, optgroup, button
- **Measurements**: progress, meter, output, datalist

**Returns:**

Returns `true` if the tag was successfully handled by a form handler,
`false` if the tag is not a form element and requires other handling.

**Signature:**

```go
func DispatchFormHandler(tagName string, nodeHandle NodeHandle, parser Parser, output string, options ConversionOptions, ctx Context, depth int, domCtx DomContext) bool
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `TagName` | `string` | Yes | The tag name |
| `NodeHandle` | `NodeHandle` | Yes | The node handle |
| `Parser` | `Parser` | Yes | The parser |
| `Output` | `string` | Yes | The output destination |
| `Options` | `ConversionOptions` | Yes | The options to use |
| `Ctx` | `Context` | Yes | The context |
| `Depth` | `int` | Yes | The depth |
| `DomCtx` | `DomContext` | Yes | The dom context |

**Returns:** `bool`


---

### HandleBlockquote()

Handle a `<blockquote>` element and convert to Markdown.

This handler processes blockquote elements including:

- Converting inline blockquotes by processing children as inline
- Handling nested blockquotes via blockquote_depth tracking
- Processing citation URLs from cite attribute
- Invoking visitor callbacks when the visitor feature is enabled
- Adding proper spacing and blockquote prefix formatting

**Signature:**

```go
func HandleBlockquote(nodeHandle NodeHandle, tag HtmlTag, parser Parser, output string, options ConversionOptions, ctx Context, depth int, domCtx DomContext)
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `NodeHandle` | `NodeHandle` | Yes | The node handle |
| `Tag` | `HtmlTag` | Yes | The h t m l tag |
| `Parser` | `Parser` | Yes | The parser |
| `Output` | `string` | Yes | The output destination |
| `Options` | `ConversionOptions` | Yes | The options to use |
| `Ctx` | `Context` | Yes | The context |
| `Depth` | `int` | Yes | The depth |
| `DomCtx` | `DomContext` | Yes | The dom context |

**Returns:** ``


---

#### HandleCode()

Handle an inline `<code>` element and convert to Markdown.

This handler processes inline code elements including:

- Extracting code content and applying backtick delimiters
- Handling backticks in content by using multiple delimiters
- Invoking visitor callbacks when the visitor feature is enabled
- Generating appropriate markdown output with proper escaping

**Signature:**

```go
func HandleCode(nodeHandle NodeHandle, tag HtmlTag, parser Parser, output string, options ConversionOptions, ctx Context, depth int, domCtx DomContext)
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `NodeHandle` | `NodeHandle` | Yes | The node handle |
| `Tag` | `HtmlTag` | Yes | The h t m l tag |
| `Parser` | `Parser` | Yes | The parser |
| `Output` | `string` | Yes | The output destination |
| `Options` | `ConversionOptions` | Yes | The options to use |
| `Ctx` | `Context` | Yes | The context |
| `Depth` | `int` | Yes | The depth |
| `DomCtx` | `DomContext` | Yes | The dom context |

**Returns:** ``


---

#### HandlePre()

Handle a `<pre>` element and convert to Markdown.

This handler processes code block elements including:

- Extracting language information from class attributes
- Processing whitespace and dedenting code content
- Supporting multiple code block styles (indented, backticks, tildes)
- Invoking visitor callbacks when the visitor feature is enabled
- Generating appropriate markdown output

**Signature:**

```go
func HandlePre(nodeHandle NodeHandle, tag HtmlTag, parser Parser, output string, options ConversionOptions, ctx Context, depth int, domCtx DomContext)
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `NodeHandle` | `NodeHandle` | Yes | The node handle |
| `Tag` | `HtmlTag` | Yes | The h t m l tag |
| `Parser` | `Parser` | Yes | The parser |
| `Output` | `string` | Yes | The output destination |
| `Options` | `ConversionOptions` | Yes | The options to use |
| `Ctx` | `Context` | Yes | The context |
| `Depth` | `int` | Yes | The depth |
| `DomCtx` | `DomContext` | Yes | The dom context |

**Returns:** ``


---

#### HandleGraphic()

Handle a `<graphic>` element and convert to Markdown.

This handler processes graphic elements including:

- Extracting source from url, href, xlink:href, or src attributes
- Using alt attribute, with fallback to filename
- Collecting metadata when the metadata feature is enabled
- Invoking visitor callbacks when the visitor feature is enabled
- Generating appropriate markdown output

**Signature:**

```go
func HandleGraphic(nodeHandle NodeHandle, tag HtmlTag, parser Parser, output string, options ConversionOptions, ctx Context, depth int, domCtx DomContext)
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `NodeHandle` | `NodeHandle` | Yes | The node handle |
| `Tag` | `HtmlTag` | Yes | The h t m l tag |
| `Parser` | `Parser` | Yes | The parser |
| `Output` | `string` | Yes | The output destination |
| `Options` | `ConversionOptions` | Yes | The options to use |
| `Ctx` | `Context` | Yes | The context |
| `Depth` | `int` | Yes | The depth |
| `DomCtx` | `DomContext` | Yes | The dom context |

**Returns:** ``


---

#### HandleImg()

Handle an `<img>` element and convert to Markdown.

This handler processes image elements including:

- Extracting src, alt, and title attributes
- Collecting metadata when the metadata feature is enabled
- Handling inline data URIs when the inline-images feature is enabled
- Invoking visitor callbacks when the visitor feature is enabled
- Generating appropriate markdown output

**Signature:**

```go
func HandleImg(nodeHandle NodeHandle, tag HtmlTag, parser Parser, output string, options ConversionOptions, ctx Context, depth int, domCtx DomContext)
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `NodeHandle` | `NodeHandle` | Yes | The node handle |
| `Tag` | `HtmlTag` | Yes | The h t m l tag |
| `Parser` | `Parser` | Yes | The parser |
| `Output` | `string` | Yes | The output destination |
| `Options` | `ConversionOptions` | Yes | The options to use |
| `Ctx` | `Context` | Yes | The context |
| `Depth` | `int` | Yes | The depth |
| `DomCtx` | `DomContext` | Yes | The dom context |

**Returns:** ``


---

#### HandleLink()

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

```go
func HandleLink(nodeHandle NodeHandle, tag HtmlTag, parser Parser, output string, options ConversionOptions, ctx Context, depth int, domCtx DomContext)
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `NodeHandle` | `NodeHandle` | Yes | The node handle |
| `Tag` | `HtmlTag` | Yes | The h t m l tag |
| `Parser` | `Parser` | Yes | The parser |
| `Output` | `string` | Yes | The output destination |
| `Options` | `ConversionOptions` | Yes | The options to use |
| `Ctx` | `Context` | Yes | The context |
| `Depth` | `int` | Yes | The depth |
| `DomCtx` | `DomContext` | Yes | The dom context |

**Returns:** ``


---

#### DispatchInlineHandler()

Dispatches inline element handling to the appropriate handler.

This function routes inline HTML elements to their specialized handlers
based on tag name. It is designed to be called from the main `walk_node`
function in `converter.rs`.

## Routing Table

The following tag routes are supported:

| Tag(s) | Handler | Description |
|--------|---------|-------------|
| `strong`, `b` | emphasis | Bold/strong text formatting |
| `em`, `i` | emphasis | Italic/emphasis text formatting |
| `a` | link | Hyperlinks and anchors |
| `code`, `kbd`, `samp` | code | Inline code and keyboard input |
| `mark`, `del`, `s`, `ins`, `u`, `small`, `sub`, `sup`, `var`, `dfn`, `abbr`, `span` | semantic | Semantic formatting |
| `ruby`, `rb`, `rt`, `rp`, `rtc` | ruby | Ruby annotations (East Asian typography) |

## Return Value

Returns `true` if the tag was recognized and handled, `false` otherwise.
This allows the caller to distinguish between:

- Handled inline elements (return `true`)
- Unhandled elements (return `false`) that should be processed as text or passed through

## Usage in converter.rs

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

## Parameters

- `tag_name` - The normalized HTML tag name (lowercase)
- `node_handle` - The DOM node handle from the parser
- `parser` - Reference to the tl HTML parser
- `output` - Output buffer to write converted content to
- `options` - Conversion configuration options
- `ctx` - Processing context with state tracking
- `depth` - Current DOM tree depth for recursion tracking
- `dom_ctx` - DOM context for accessing tree structure

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

```go
func DispatchInlineHandler(tagName string, nodeHandle NodeHandle, parser Parser, output string, options ConversionOptions, ctx Context, depth int, domCtx DomContext) bool
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `TagName` | `string` | Yes | The normalized HTML tag name (lowercase) |
| `NodeHandle` | `NodeHandle` | Yes | The DOM node handle from the parser |
| `Parser` | `Parser` | Yes | Reference to the tl HTML parser |
| `Output` | `string` | Yes | Output buffer to write converted content to |
| `Options` | `ConversionOptions` | Yes | Conversion configuration options |
| `Ctx` | `Context` | Yes | Processing context with state tracking |
| `Depth` | `int` | Yes | Current DOM tree depth for recursion tracking |
| `DomCtx` | `DomContext` | Yes | DOM context for accessing tree structure |

**Returns:** `bool`


---

### CalculateListContinuationIndent()

Calculate indentation level for list item continuations.

Returns the number of 4-space indent groups needed for list continuations.

List continuations (block elements inside list items) need special indentation:

- Base indentation: (depth - 1) groups (for the nesting level)
- Content indentation: depth groups (for the list item content)
- Combined formula: (2 * depth - 1) groups of 4 spaces each

**Signature:**

```go
func CalculateListContinuationIndent(depth int) int
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `Depth` | `int` | Yes | The depth |

**Returns:** `int`


---

#### IsLooseList()

Check if a list (ul or ol) is "loose".

A loose list is one where any list item contains block-level elements
like paragraphs (<p>). In loose lists, all items should have blank line
separation (ending with \n\n) regardless of their own content.

**Signature:**

```go
func IsLooseList(nodeHandle NodeHandle, parser Parser, domCtx DomContext) bool
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `NodeHandle` | `NodeHandle` | Yes | The node handle |
| `Parser` | `Parser` | Yes | The parser |
| `DomCtx` | `DomContext` | Yes | The dom context |

**Returns:** `bool`


---

#### AddListContinuationIndent()

Add list continuation indentation to output.

Used when block elements (like <p> or <div>) appear inside list items.
Adds appropriate line separation and indentation to continue the list item.

**Signature:**

```go
func AddListContinuationIndent(output string, listDepth int, blankLine bool, options ConversionOptions)
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `Output` | `string` | Yes | The output string to append to |
| `ListDepth` | `int` | Yes | Current list nesting depth |
| `BlankLine` | `bool` | Yes | If true, adds blank line separation (\n\n); if false, single newline (\n) |
| `Options` | `ConversionOptions` | Yes | The options to use |

**Returns:** ``


---

#### ContinuationIndentString()

Calculate the indentation string for list continuations based on depth and options.

**Signature:**

```go
func ContinuationIndentString(listDepth int, options ConversionOptions) *string
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `ListDepth` | `int` | Yes | The list depth |
| `Options` | `ConversionOptions` | Yes | The options to use |

**Returns:** `*string`


---

#### AddListLeadingSeparator()

Add appropriate leading separator before a list.

Lists need different separators depending on context:

- In table cells: <br> tag if there's already content
- Outside lists: blank line (\n\n) if needed
- Inside list items: blank line before nested list

**Signature:**

```go
func AddListLeadingSeparator(output string, ctx Context)
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `Output` | `string` | Yes | The output destination |
| `Ctx` | `Context` | Yes | The context |

**Returns:** ``


---

#### AddNestedListTrailingSeparator()

Add appropriate trailing separator after a nested list.

Nested lists inside list items need trailing newlines to separate
from following content. In loose lists, use blank line (\n\n). In tight lists, single newline (\n).

**Signature:**

```go
func AddNestedListTrailingSeparator(output string, ctx Context)
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `Output` | `string` | Yes | The output destination |
| `Ctx` | `Context` | Yes | The context |

**Returns:** ``


---

#### CalculateListNestingDepth()

Calculate the nesting depth for a list.

If we're in a list but NOT in a list item, this is incorrectly nested HTML
and we need to increment the depth. If in a list item, the depth was already
incremented by the <li> element.

**Signature:**

```go
func CalculateListNestingDepth(ctx Context) int
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `Ctx` | `Context` | Yes | The context |

**Returns:** `int`


---

#### IsListItem()

Check if a node is a list item element.

**Signature:**

```go
func IsListItem(nodeHandle NodeHandle, parser Parser, domCtx DomContext) bool
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `NodeHandle` | `NodeHandle` | Yes | The node handle |
| `Parser` | `Parser` | Yes | The parser |
| `DomCtx` | `DomContext` | Yes | The dom context |

**Returns:** `bool`


---

#### ProcessListChildren()

Process a list's children, tracking which items had block elements.

This is used to determine proper spacing between list items.
Returns true if the last processed item had block children.

**Signature:**

```go
func ProcessListChildren(nodeHandle NodeHandle, parser Parser, output string, options ConversionOptions, ctx Context, depth int, isOrdered bool, isLoose bool, nestedDepth int, startCounter int, domCtx DomContext)
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `NodeHandle` | `NodeHandle` | Yes | The node handle |
| `Parser` | `Parser` | Yes | The parser |
| `Output` | `string` | Yes | The output destination |
| `Options` | `ConversionOptions` | Yes | The options to use |
| `Ctx` | `Context` | Yes | The context |
| `Depth` | `int` | Yes | The depth |
| `IsOrdered` | `bool` | Yes | The is ordered |
| `IsLoose` | `bool` | Yes | The is loose |
| `NestedDepth` | `int` | Yes | The nested depth |
| `StartCounter` | `int` | Yes | The start counter |
| `DomCtx` | `DomContext` | Yes | The dom context |

**Returns:** ``


---

#### DispatchListHandler()

Dispatches list element handling to the appropriate handler.

Returns `true` if the element was handled, `false` otherwise.

## Supported Elements

- `ol`: Ordered list - routed to `ordered.handle`
- `ul`: Unordered list - routed to `unordered.handle`
- `li`: List item - routed to `item.handle_li`
- `dl`: Definition list - routed to `definition.handle_dl`
- `dt`: Definition term - routed to `definition.handle_dt`
- `dd`: Definition description - routed to `definition.handle_dd`

**Signature:**

```go
func DispatchListHandler(tagName string, nodeHandle NodeHandle, tag HtmlTag, parser Parser, output string, options ConversionOptions, ctx Context, depth int, domCtx DomContext) bool
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `TagName` | `string` | Yes | The tag name |
| `NodeHandle` | `NodeHandle` | Yes | The node handle |
| `Tag` | `HtmlTag` | Yes | The h t m l tag |
| `Parser` | `Parser` | Yes | The parser |
| `Output` | `string` | Yes | The output destination |
| `Options` | `ConversionOptions` | Yes | The options to use |
| `Ctx` | `Context` | Yes | The context |
| `Depth` | `int` | Yes | The depth |
| `DomCtx` | `DomContext` | Yes | The dom context |

**Returns:** `bool`


---

### ConvertHtml()

Converts HTML to Markdown using the provided conversion options.

This is the main entry point for HTML to Markdown conversion.

**Signature:**

```go
func ConvertHtml(html string, options ConversionOptions) (string, error)
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `Html` | `string` | Yes | The html |
| `Options` | `ConversionOptions` | Yes | The options to use |

**Returns:** `string`

**Errors:** Returns `error`.


---

#### ConvertHtmlWithVisitor()

Converts HTML to Markdown with a custom visitor for callbacks during traversal.

This variant allows passing a visitor that will receive callbacks for each node
during the tree walk, enabling custom processing or analysis.

**Signature:**

```go
func ConvertHtmlWithVisitor(html string, options ConversionOptions, visitor VisitorHandle) (string, error)
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `Html` | `string` | Yes | The html |
| `Options` | `ConversionOptions` | Yes | The options to use |
| `Visitor` | `*VisitorHandle` | No | The visitor handle |

**Returns:** `string`

**Errors:** Returns `error`.


---

#### DispatchMediaHandler()

Dispatches media element handling to the appropriate handler.

This function routes media-related HTML elements to their specialized handlers
based on tag name. It is designed to be called from the main `walk_node`
function in `converter.rs`.

## Routing Table

The following tag routes are supported:

| Tag(s) | Handler | Description |
|--------|---------|-------------|
| `iframe` | embedded | Embedded content frames |
| `video` | embedded | Video elements |
| `audio` | embedded | Audio elements |
| `picture` | embedded | Responsive image containers |
| `svg` | svg | SVG image elements |
| `math` | svg | MathML elements |

## Return Value

Returns `true` if the tag was recognized and handled, `false` otherwise.

**Signature:**

```go
func DispatchMediaHandler(tagName string, nodeHandle NodeHandle, parser Parser, output string, options ConversionOptions, ctx Context, depth int, domCtx DomContext) bool
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `TagName` | `string` | Yes | The tag name |
| `NodeHandle` | `NodeHandle` | Yes | The node handle |
| `Parser` | `Parser` | Yes | The parser |
| `Output` | `string` | Yes | The output destination |
| `Options` | `ConversionOptions` | Yes | The options to use |
| `Ctx` | `Context` | Yes | The context |
| `Depth` | `int` | Yes | The depth |
| `DomCtx` | `DomContext` | Yes | The dom context |

**Returns:** `bool`


---

### ExtractPlainText()

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

```go
func ExtractPlainText(dom VDom, parser Parser, options ConversionOptions) string
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `Dom` | `VDom` | Yes | The v dom |
| `Parser` | `Parser` | Yes | The parser |
| `Options` | `ConversionOptions` | Yes | The options to use |

**Returns:** `string`


---

#### HandleDfn()

Handles the `<dfn>` element.

A dfn element marks a term that is being defined. The content represents
the term, and its definition would typically appear in surrounding context.
It is rendered as emphasized (italic) text.

## Behavior

- Content is collected from children
- Non-empty content is wrapped with the configured emphasis symbol (default: `*`)
- Inline suffix handling is applied (e.g., footnote references)

**Signature:**

```go
func HandleDfn(tagName string, nodeHandle NodeHandle, parser Parser, output string, options ConversionOptions, ctx Context, depth int, domCtx DomContext)
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `TagName` | `string` | Yes | The  tag name |
| `NodeHandle` | `NodeHandle` | Yes | The node handle |
| `Parser` | `Parser` | Yes | The parser |
| `Output` | `string` | Yes | The output destination |
| `Options` | `ConversionOptions` | Yes | The options to use |
| `Ctx` | `Context` | Yes | The context |
| `Depth` | `int` | Yes | The depth |
| `DomCtx` | `DomContext` | Yes | The dom context |

**Returns:** ``


---

### HandleAbbr()

Handles the `<abbr>` element.

An abbr element marks an abbreviation or acronym. The `title` attribute
provides the expansion of the abbreviation, which is appended in parentheses
if present.

## Behavior

- Content is collected from children
- Non-empty content is output as-is
- If `title` attribute exists, it is appended in parentheses: `abbr (title)`

Produces: `HTML (HyperText Markup Language)`

**Signature:**

```go
func HandleAbbr(tagName string, nodeHandle NodeHandle, parser Parser, output string, options ConversionOptions, ctx Context, depth int, domCtx DomContext)
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `TagName` | `string` | Yes | The  tag name |
| `NodeHandle` | `NodeHandle` | Yes | The node handle |
| `Parser` | `Parser` | Yes | The parser |
| `Output` | `string` | Yes | The output destination |
| `Options` | `ConversionOptions` | Yes | The options to use |
| `Ctx` | `Context` | Yes | The context |
| `Depth` | `int` | Yes | The depth |
| `DomCtx` | `DomContext` | Yes | The dom context |

**Returns:** ``


---

### HandleTimeData()

Handles the `<time>` and `<data>` elements.

Time and data elements contain machine-readable content in their attributes
and human-readable content in their text. For Markdown purposes, we output
only the human-readable text content, as Markdown doesn't have a way to
preserve machine-readable metadata.

## Behavior

- Content is extracted from children and output as-is
- Attributes (datetime, value) are not rendered in Markdown output

**Signature:**

```go
func HandleTimeData(tagName string, nodeHandle NodeHandle, parser Parser, output string, options ConversionOptions, ctx Context, depth int, domCtx DomContext)
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `TagName` | `string` | Yes | The  tag name |
| `NodeHandle` | `NodeHandle` | Yes | The node handle |
| `Parser` | `Parser` | Yes | The parser |
| `Output` | `string` | Yes | The output destination |
| `Options` | `ConversionOptions` | Yes | The options to use |
| `Ctx` | `Context` | Yes | The context |
| `Depth` | `int` | Yes | The depth |
| `DomCtx` | `DomContext` | Yes | The dom context |

**Returns:** ``


---

### HandleCite()

Handles the `<cite>` element.

A cite element marks the title of a cited work (book, article, website, etc.).
It is rendered as emphasized (italic) text in block mode, or as plain text in inline mode.

## Behavior

- **Block mode**: Content is wrapped with emphasis markers (default: `*`)
- **Inline mode**: Content is output as-is without formatting

**Signature:**

```go
func HandleCite(tagName string, nodeHandle NodeHandle, parser Parser, output string, options ConversionOptions, ctx Context, depth int, domCtx DomContext)
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `TagName` | `string` | Yes | The  tag name |
| `NodeHandle` | `NodeHandle` | Yes | The node handle |
| `Parser` | `Parser` | Yes | The parser |
| `Output` | `string` | Yes | The output destination |
| `Options` | `ConversionOptions` | Yes | The options to use |
| `Ctx` | `Context` | Yes | The context |
| `Depth` | `int` | Yes | The depth |
| `DomCtx` | `DomContext` | Yes | The dom context |

**Returns:** ``


---

### HandleQ()

Handles the `<q>` element.

A q element marks an inline quotation. In Markdown, it is rendered as
quoted text enclosed in double quotes. Backslashes and quotes within
the content are escaped.

## Behavior

- **Block mode**: Content is wrapped in escaped double quotes: `"content"`
- **Inline mode**: Content is output as-is without quotes

## Escaping

Internal backslashes and double quotes are escaped:

- `\` → `\\`
- `"` → `\"`

**Signature:**

```go
func HandleQ(tagName string, nodeHandle NodeHandle, parser Parser, output string, options ConversionOptions, ctx Context, depth int, domCtx DomContext)
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `TagName` | `string` | Yes | The  tag name |
| `NodeHandle` | `NodeHandle` | Yes | The node handle |
| `Parser` | `Parser` | Yes | The parser |
| `Output` | `string` | Yes | The output destination |
| `Options` | `ConversionOptions` | Yes | The options to use |
| `Ctx` | `Context` | Yes | The context |
| `Depth` | `int` | Yes | The depth |
| `DomCtx` | `DomContext` | Yes | The dom context |

**Returns:** ``


---

### HandleHgroup()

Handles the `<hgroup>` element.

An hgroup element groups related headings together (e.g., a title and subtitle).
In Markdown, we simply process all children sequentially, allowing nested
headings to maintain their individual formatting.

## Behavior

- Children are processed sequentially in the current context
- No special formatting is applied at the hgroup level

**Signature:**

```go
func HandleHgroup(tagName string, nodeHandle NodeHandle, parser Parser, output string, options ConversionOptions, ctx Context, depth int, domCtx DomContext)
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `TagName` | `string` | Yes | The  tag name |
| `NodeHandle` | `NodeHandle` | Yes | The node handle |
| `Parser` | `Parser` | Yes | The parser |
| `Output` | `string` | Yes | The output destination |
| `Options` | `ConversionOptions` | Yes | The options to use |
| `Ctx` | `Context` | Yes | The context |
| `Depth` | `int` | Yes | The depth |
| `DomCtx` | `DomContext` | Yes | The dom context |

**Returns:** ``


---

### HandleDl()

Handles the `<dl>` element.

A definition list contains terms and their definitions. Terms and definitions
are output as plain blocks without Pandoc-style colon syntax, since standard
Markdown and GFM do not support definition lists.

## Behavior

- **Inline mode**: Children are processed inline without block spacing
- **Block mode**: Content is collected and wrapped with proper spacing

**Signature:**

```go
func HandleDl(tagName string, nodeHandle NodeHandle, parser Parser, output string, options ConversionOptions, ctx Context, depth int, domCtx DomContext)
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `TagName` | `string` | Yes | The  tag name |
| `NodeHandle` | `NodeHandle` | Yes | The node handle |
| `Parser` | `Parser` | Yes | The parser |
| `Output` | `string` | Yes | The output destination |
| `Options` | `ConversionOptions` | Yes | The options to use |
| `Ctx` | `Context` | Yes | The context |
| `Depth` | `int` | Yes | The depth |
| `DomCtx` | `DomContext` | Yes | The dom context |

**Returns:** ``


---

### HandleDt()

Handles the `<dt>` element.

A dt element contains a term being defined. Terms are output on their own line,
with definitions following on subsequent lines.

## Behavior

- **Inline mode**: Content is output as-is
- **Block mode**: Content is followed by a newline

**Signature:**

```go
func HandleDt(tagName string, nodeHandle NodeHandle, parser Parser, output string, options ConversionOptions, ctx Context, depth int, domCtx DomContext)
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `TagName` | `string` | Yes | The  tag name |
| `NodeHandle` | `NodeHandle` | Yes | The node handle |
| `Parser` | `Parser` | Yes | The parser |
| `Output` | `string` | Yes | The output destination |
| `Options` | `ConversionOptions` | Yes | The options to use |
| `Ctx` | `Context` | Yes | The context |
| `Depth` | `int` | Yes | The depth |
| `DomCtx` | `DomContext` | Yes | The dom context |

**Returns:** ``


---

### HandleDd()

Handles the `<dd>` element.

A dd element contains the definition for a term. It is output as a plain
block since standard Markdown and GFM do not support definition list syntax.

## Behavior

- **Inline mode**: Content is output as-is
- **Block mode**: Content is output as a block

**Signature:**

```go
func HandleDd(tagName string, nodeHandle NodeHandle, parser Parser, output string, options ConversionOptions, ctx Context, depth int, domCtx DomContext)
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `TagName` | `string` | Yes | The  tag name |
| `NodeHandle` | `NodeHandle` | Yes | The node handle |
| `Parser` | `Parser` | Yes | The parser |
| `Output` | `string` | Yes | The output destination |
| `Options` | `ConversionOptions` | Yes | The options to use |
| `Ctx` | `Context` | Yes | The context |
| `Depth` | `int` | Yes | The depth |
| `DomCtx` | `DomContext` | Yes | The dom context |

**Returns:** ``


---

### HandleMenu()

Handles the `<menu>` element.

A menu element is a semantic list, typically used for command menus or
navigation. It is rendered as an unordered list with dashes.

## Behavior

- **Inline mode**: Children are processed inline without list formatting
- **Block mode**: Content is rendered as an unordered list
- Uses `-` as the list bullet (overrides configured bullets)
- Proper blank-line spacing is maintained

**Signature:**

```go
func HandleMenu(tagName string, nodeHandle NodeHandle, parser Parser, output string, options ConversionOptions, ctx Context, depth int, domCtx DomContext)
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `TagName` | `string` | Yes | The  tag name |
| `NodeHandle` | `NodeHandle` | Yes | The node handle |
| `Parser` | `Parser` | Yes | The parser |
| `Output` | `string` | Yes | The output destination |
| `Options` | `ConversionOptions` | Yes | The options to use |
| `Ctx` | `Context` | Yes | The context |
| `Depth` | `int` | Yes | The depth |
| `DomCtx` | `DomContext` | Yes | The dom context |

**Returns:** ``


---

### HandleFigure()

Handles the `<figure>` element.

A figure element contains content (typically images) and optionally a figcaption.
The handler collects all content and cleans up extra line breaks.

## Behavior

- **Inline mode**: Children are processed inline without block spacing
- **Block mode**: Content is collected, line breaks normalized, and wrapped with blank lines
- **Image normalization**: Removes extra spaces before `![` to improve Markdown formatting

## Implementation Details

The handler performs the following on the collected content:

1. Normalizes newline + image sequences: `\n![` → `![`
2. Normalizes space + image sequences: `![` → `![`
3. Trims the final content and wraps it with blank lines

**Signature:**

```go
func HandleFigure(tagName string, nodeHandle NodeHandle, parser Parser, output string, options ConversionOptions, ctx Context, depth int, domCtx DomContext)
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `TagName` | `string` | Yes | The  tag name |
| `NodeHandle` | `NodeHandle` | Yes | The node handle |
| `Parser` | `Parser` | Yes | The parser |
| `Output` | `string` | Yes | The output destination |
| `Options` | `ConversionOptions` | Yes | The options to use |
| `Ctx` | `Context` | Yes | The context |
| `Depth` | `int` | Yes | The depth |
| `DomCtx` | `DomContext` | Yes | The dom context |

**Returns:** ``


---

### HandleFigcaption()

Handles the `<figcaption>` element.

A figcaption element contains text that describes or supplements the figure.
It is rendered as emphasized (italic) text to distinguish it from regular content.

## Behavior

- Content is collected and trimmed
- Non-empty content is wrapped in `*text*` (emphasis) markers
- Proper spacing is maintained around the caption

## Implementation Details

The handler:

1. Collects and processes all children
2. Checks for existing output and adds spacing as needed
3. Wraps content in emphasis markers: `*caption*`
4. Ensures proper blank-line spacing after the caption

**Signature:**

```go
func HandleFigcaption(tagName string, nodeHandle NodeHandle, parser Parser, output string, options ConversionOptions, ctx Context, depth int, domCtx DomContext)
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `TagName` | `string` | Yes | The  tag name |
| `NodeHandle` | `NodeHandle` | Yes | The node handle |
| `Parser` | `Parser` | Yes | The parser |
| `Output` | `string` | Yes | The output destination |
| `Options` | `ConversionOptions` | Yes | The options to use |
| `Ctx` | `Context` | Yes | The context |
| `Depth` | `int` | Yes | The depth |
| `DomCtx` | `DomContext` | Yes | The dom context |

**Returns:** ``


---

### HandleDetails()

Handles the `<details>` element.

A details element represents a disclosure widget that can be toggled
to show/hide additional content. In Markdown, it's rendered as a block
with all content visible.

## Behavior

- **Inline mode**: Children are processed inline without block spacing
- **Block mode**: Content is collected and wrapped with proper blank-line spacing
- **Empty content**: Skipped entirely

**Signature:**

```go
func HandleDetails(tagName string, nodeHandle NodeHandle, parser Parser, output string, options ConversionOptions, ctx Context, depth int, domCtx DomContext)
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `TagName` | `string` | Yes | The  tag name |
| `NodeHandle` | `NodeHandle` | Yes | The node handle |
| `Parser` | `Parser` | Yes | The parser |
| `Output` | `string` | Yes | The output destination |
| `Options` | `ConversionOptions` | Yes | The options to use |
| `Ctx` | `Context` | Yes | The context |
| `Depth` | `int` | Yes | The depth |
| `DomCtx` | `DomContext` | Yes | The dom context |

**Returns:** ``


---

### HandleSummary()

Handles the `<summary>` element.

A summary element contains a caption for a details element.
It is rendered as strong (bold) text to distinguish it from regular content.

## Behavior

- **Inline mode**: Content is rendered inline without emphasis
- **Block mode**: Content is wrapped in strong markers (e.g., `**text**`)
- Uses the configured strong/emphasis symbol from ConversionOptions

## Implementation Details

The handler:

1. Creates a context with `in_strong: true` for nested formatting
2. Collects content from all children
3. Wraps non-empty content in strong markers (repeated twice per Markdown spec)

**Signature:**

```go
func HandleSummary(tagName string, nodeHandle NodeHandle, parser Parser, output string, options ConversionOptions, ctx Context, depth int, domCtx DomContext)
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `TagName` | `string` | Yes | The  tag name |
| `NodeHandle` | `NodeHandle` | Yes | The node handle |
| `Parser` | `Parser` | Yes | The parser |
| `Output` | `string` | Yes | The output destination |
| `Options` | `ConversionOptions` | Yes | The options to use |
| `Ctx` | `Context` | Yes | The context |
| `Depth` | `int` | Yes | The depth |
| `DomCtx` | `DomContext` | Yes | The dom context |

**Returns:** ``


---

### HandleDialog()

Handles the `<dialog>` element.

A dialog element represents a modal dialog box. In Markdown, it's rendered
as a block container with content visible.

## Behavior

- **Inline mode**: Children are processed inline without block spacing
- **Block mode**: Content is processed and wrapped with proper blank lines
- Trailing whitespace is removed from collected content

## Implementation Details

The handler:

1. Marks the position in output before processing children
2. Processes all children in the normal context
3. Removes trailing spaces and tabs from the output
4. Ensures proper blank-line spacing after the dialog

**Signature:**

```go
func HandleDialog(tagName string, nodeHandle NodeHandle, parser Parser, output string, options ConversionOptions, ctx Context, depth int, domCtx DomContext)
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `TagName` | `string` | Yes | The  tag name |
| `NodeHandle` | `NodeHandle` | Yes | The node handle |
| `Parser` | `Parser` | Yes | The parser |
| `Output` | `string` | Yes | The output destination |
| `Options` | `ConversionOptions` | Yes | The options to use |
| `Ctx` | `Context` | Yes | The context |
| `Depth` | `int` | Yes | The depth |
| `DomCtx` | `DomContext` | Yes | The dom context |

**Returns:** ``


---

### DispatchSemanticHandler()

Dispatches semantic element handling to the appropriate handler.

This function routes semantic HTML5 elements to their specialized handlers
based on tag name. It is designed to be called from the main `walk_node`
function in `converter.rs`.

## Routing Table

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

```go
func DispatchSemanticHandler(tagName string, nodeHandle NodeHandle, parser Parser, output string, options ConversionOptions, ctx Context, depth int, domCtx DomContext) bool
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `TagName` | `string` | Yes | The tag name |
| `NodeHandle` | `NodeHandle` | Yes | The node handle |
| `Parser` | `Parser` | Yes | The parser |
| `Output` | `string` | Yes | The output destination |
| `Options` | `ConversionOptions` | Yes | The options to use |
| `Ctx` | `Context` | Yes | The context |
| `Depth` | `int` | Yes | The depth |
| `DomCtx` | `DomContext` | Yes | The dom context |

**Returns:** `bool`


---

### EscapeLinkLabel()

Escape special characters in link labels.

Markdown link labels can contain brackets, which need careful escaping to avoid
being interpreted as nested links. This function escapes unescaped closing brackets
that would break the link syntax.

**Signature:**

```go
func EscapeLinkLabel(text string) string
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `Text` | `string` | Yes | The text |

**Returns:** `string`


---

#### EscapeMalformedAngleBrackets()

Escape malformed angle brackets in markdown output.

Markdown uses `<...>` for automatic links. Angle brackets that don't form valid
link syntax should be escaped as `&lt;` to prevent parser confusion.

A valid tag must have:

- `<!` followed by `-` or alphabetic character (for comments/declarations)
- `</` followed by alphabetic character (for closing tags)
- `<?` (for processing instructions)
- `<` followed by alphabetic character (for opening tags)

**Signature:**

```go
func EscapeMalformedAngleBrackets(input string) Str
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `Input` | `string` | Yes | The input data |

**Returns:** `Str`


---

#### TrimLineEndWhitespace()

Remove trailing spaces/tabs from every line while preserving newlines.

**Signature:**

```go
func TrimLineEndWhitespace(output string)
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `Output` | `string` | Yes | The output destination |

**Returns:** ``


---

#### TruncateAtCharBoundary()

Truncate a string at a valid UTF-8 boundary.

**Signature:**

```go
func TruncateAtCharBoundary(value string, maxLen int)
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `Value` | `string` | Yes | The value |
| `MaxLen` | `int` | Yes | The max len |

**Returns:** ``


---

#### NormalizeHeadingText()

Normalize heading text by replacing newlines and extra whitespace.

Heading text should be on a single line in Markdown. This function collapses
any newlines and multiple spaces into single spaces.

**Signature:**

```go
func NormalizeHeadingText(text string) Str
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `Text` | `string` | Yes | The text |

**Returns:** `Str`


---

#### DedentCodeBlock()

Remove common leading whitespace from all lines in a code block.

This is useful when HTML authors indent `<pre>` content for readability,
so we can strip the shared indentation without touching meaningful spacing.

**Signature:**

```go
func DedentCodeBlock(content string) string
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `Content` | `string` | Yes | The content to process |

**Returns:** `string`


---

#### FloorCharBoundary()

Returns the largest valid char boundary index at or before `index`.

If `index` is already a char boundary it is returned unchanged.
Otherwise it walks backwards to find one. Returns 0 if no boundary
is found before `index`.

**Signature:**

```go
func FloorCharBoundary(s string, index int) int
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `S` | `string` | Yes | The s |
| `Index` | `int` | Yes | The index |

**Returns:** `int`


---

#### HandleVisitorElementStart()

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

```go
func HandleVisitorElementStart(visitorHandle VisitorHandle, tagName string, nodeHandle NodeHandle, tag HtmlTag, parser Parser, output string, ctx Context, depth int, domCtx DomContext) VisitAction
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `VisitorHandle` | `VisitorHandle` | Yes | Reference to the visitor for callbacks |
| `TagName` | `string` | Yes | The normalized tag name being processed |
| `NodeHandle` | `NodeHandle` | Yes | Handle to the DOM node |
| `Tag` | `HtmlTag` | Yes | Reference to the tag object |
| `Parser` | `Parser` | Yes | Reference to the tl parser |
| `Output` | `string` | Yes | Mutable reference to output string |
| `Ctx` | `Context` | Yes | The context |
| `Depth` | `int` | Yes | Current tree depth |
| `DomCtx` | `DomContext` | Yes | Reference to DOM context for tree navigation |

**Returns:** `VisitAction`


---

#### HandleVisitorElementEnd()

Handles visitor callback for element end (after processing).

This function is called when exiting an element after its content has been processed.
The visitor can:

- Accept the output normally (Continue)
- Replace the output with custom content (Custom)
- Remove the output entirely (Skip)
- Signal an error (Error)

**Signature:**

```go
func HandleVisitorElementEnd(visitorHandle VisitorHandle, tagName string, nodeHandle NodeHandle, tag HtmlTag, parser Parser, output string, elementOutputStart int, ctx Context, depth int, domCtx DomContext)
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `VisitorHandle` | `VisitorHandle` | Yes | Reference to the visitor for callbacks |
| `TagName` | `string` | Yes | The normalized tag name that was processed |
| `NodeHandle` | `NodeHandle` | Yes | Handle to the DOM node |
| `Tag` | `HtmlTag` | Yes | Reference to the tag object |
| `Parser` | `Parser` | Yes | Reference to the tl parser |
| `Output` | `string` | Yes | Mutable reference to output string |
| `ElementOutputStart` | `int` | Yes | Byte position where this element's output started |
| `Ctx` | `Context` | Yes | Reference to the conversion context |
| `Depth` | `int` | Yes | Current tree depth |
| `DomCtx` | `DomContext` | Yes | Reference to DOM context for tree navigation |

**Returns:** ``


---

#### Escape()

Escape Markdown special characters in text.

**Returns:**

Escaped text

**Signature:**

```go
func Escape(text string, escapeMisc bool, escapeAsterisks bool, escapeUnderscores bool, escapeAscii bool) Str
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `Text` | `string` | Yes | Text to escape |
| `EscapeMisc` | `bool` | Yes | Escape miscellaneous characters (`\` `&` `<` `` ` `` `[` `>` `~` `#` `=` `+` `\|` `-`) |
| `EscapeAsterisks` | `bool` | Yes | Escape asterisks (`*`) |
| `EscapeUnderscores` | `bool` | Yes | Escape underscores (`_`) |
| `EscapeAscii` | `bool` | Yes | Escape all ASCII punctuation (for `CommonMark` spec compliance) |

**Returns:** `Str`


---

#### Chomp()

Extract boundary whitespace from text (chomp).

Returns (prefix, suffix, `trimmed_text`) tuple.
Prefix/suffix are " " if original text had leading/trailing whitespace.
However, suffix is "" if the trailing whitespace is only newlines (not spaces/tabs).
This prevents trailing newlines from becoming trailing spaces in the output.
The trimmed text has all leading/trailing whitespace removed.

**Signature:**

```go
func Chomp(text string) StrStrStr
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `Text` | `string` | Yes | The text |

**Returns:** `StrStrStr`


---

#### NormalizeWhitespace()

Normalize whitespace by collapsing consecutive spaces and tabs.

Multiple spaces and tabs are replaced with a single space.
Newlines are preserved.
Unicode spaces are normalized to ASCII spaces.

**Returns:**

Normalized text with collapsed spaces/tabs but preserved newlines

**Signature:**

```go
func NormalizeWhitespace(text string) string
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `Text` | `string` | Yes | The text to normalize |

**Returns:** `string`


---

#### NormalizeWhitespaceCow()

Normalize whitespace in text, returning borrowed or owned result as needed.

This function optimizes memory by returning a borrowed reference when no normalization
is needed, and only allocating a new string when whitespace changes are necessary.

Multiple consecutive spaces, tabs, and Unicode space characters are replaced with
a single ASCII space. Newlines are preserved as-is.

**Returns:**

`Cow.Borrowed` if text is already normalized, or `Cow.Owned` with normalized text

**Signature:**

```go
func NormalizeWhitespaceCow(text string) Str
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `Text` | `string` | Yes | The text to normalize |

**Returns:** `Str`


---

#### DecodeHtmlEntities()

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

```go
func DecodeHtmlEntities(text string) string
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `Text` | `string` | Yes | Text containing HTML entities |

**Returns:** `string`


---

#### DecodeHtmlEntitiesCow()

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

```go
func DecodeHtmlEntitiesCow(text string) Str
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `Text` | `string` | Yes | Text potentially containing HTML entities |

**Returns:** `Str`


---

#### Underline()

Underline text with a character.

**Signature:**

```go
func Underline(text string, padChar string) string
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `Text` | `string` | Yes | The text |
| `PadChar` | `string` | Yes | The pad char |

**Returns:** `string`


---

#### Indent()

Indent text with a string prefix.

**Signature:**

```go
func Indent(text string, level int, indentStr string) string
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `Text` | `string` | Yes | The text |
| `Level` | `int` | Yes | The level |
| `IndentStr` | `string` | Yes | The indent str |

**Returns:** `string`


---

#### BuildDocumentStructure()

Build a `DocumentStructure` from an already-parsed `tl.VDom`.

Walks the DOM once, mapping HTML elements to semantic `NodeContent` variants,
tracking parent/child relationships, extracting inline `TextAnnotation`s, and
constructing heading-based `Group` nodes.

**Signature:**

```go
func BuildDocumentStructure(dom VDom) DocumentStructure
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `Dom` | `VDom` | Yes | The v dom |

**Returns:** `DocumentStructure`


---

#### BuildNodeContext()

Build a `NodeContext` from current parsing state.

Creates a complete `NodeContext` suitable for passing to visitor callbacks.
This function collects metadata about the current node from various sources:

- Tag name and attributes from the HTML element
- Depth and parent information from the DOM tree
- Index among siblings for positional awareness
- Inline/block classification

## Parameters

- `node_type`: Coarse-grained classification (Link, Image, Heading, etc.)
- `tag_name`: Raw HTML tag name (e.g., "div", "h1", "custom-element")
- `attributes`: All HTML attributes as key-value pairs
- `depth`: Nesting depth in the DOM tree (0 = root)
- `index_in_parent`: Zero-based index among siblings
- `parent_tag`: Parent element's tag name (None if root)
- `is_inline`: Whether this element is treated as inline vs block

**Returns:**

A fully populated `NodeContext` ready for visitor dispatch.

## Performance

This function performs minimal allocations:

- Clones `tag_name` (typically 2-10 bytes)
- Clones `parent_tag` if present (typically 2-10 bytes)
- Clones the attributes `BTreeMap` (heap allocation if non-empty)

For text nodes and simple elements without attributes, allocations are minimal.

**Signature:**

```go
func BuildNodeContext(nodeType NodeType, tagName string, attributes map[string]string, depth int, indexInParent int, parentTag string, isInline bool) NodeContext
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `NodeType` | `NodeType` | Yes | Coarse-grained classification (Link, Image, Heading, etc.) |
| `TagName` | `string` | Yes | Raw HTML tag name (e.g., "div", "h1", "custom-element") |
| `Attributes` | `map[string]string` | Yes | All HTML attributes as key-value pairs |
| `Depth` | `int` | Yes | Nesting depth in the DOM tree (0 = root) |
| `IndexInParent` | `int` | Yes | Zero-based index among siblings |
| `ParentTag` | `*string` | No | Parent element's tag name (None if root) |
| `IsInline` | `bool` | Yes | Whether this element is treated as inline vs block |

**Returns:** `NodeContext`


---

### Convert()

Convert HTML to Markdown, returning a `ConversionResult` with content, metadata, images,
and warnings.

**Errors:**

Returns an error if HTML parsing fails or if the input contains invalid UTF-8.

**Signature:**

```go
func Convert(html string, options ConversionOptions) (ConversionResult, error)
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `Html` | `string` | Yes | The HTML string to convert |
| `Options` | `*ConversionOptions` | No | Optional conversion options (defaults to `default options`) |

**Returns:** `ConversionResult`

**Errors:** Returns `error`.


---

#### ConvertWithVisitor()

Internal: convert with visitor support. Used by FFI crate.
Will be removed when convert() accepts visitor parameter directly.

**Signature:**

```go
func ConvertWithVisitor(html string, options ConversionOptions, visitor VisitorHandle) (string, error)
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `Html` | `string` | Yes | The html |
| `Options` | `*ConversionOptions` | No | The options to use |
| `Visitor` | `*VisitorHandle` | No | The visitor handle |

**Returns:** `string`

**Errors:** Returns `error`.


---

#### ConversionOptionsFromJson()

Parse JSON string into `ConversionOptions`.

Deserializes a JSON string into a full set of conversion options.
The JSON can be either a complete or partial options object.

**Returns:**

Fully populated `ConversionOptions` with defaults applied to any unspecified values

**Errors:**

Returns `ConversionError.ConfigError` if JSON parsing fails or contains invalid option values

**Signature:**

```go
func ConversionOptionsFromJson(json string) (ConversionOptions, error)
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `Json` | `string` | Yes | JSON string representing conversion options |

**Returns:** `ConversionOptions`

**Errors:** Returns `error`.


---

#### ConversionOptionsUpdateFromJson()

Parse JSON string into partial `ConversionOptions` update.

Deserializes a JSON string into a partial set of conversion options.
Only specified options are included; unspecified options are None.

**Returns:**

`ConversionOptionsUpdate` with only specified fields populated

**Errors:**

Returns `ConversionError.ConfigError` if JSON parsing fails or contains invalid option values

**Signature:**

```go
func ConversionOptionsUpdateFromJson(json string) (ConversionOptionsUpdate, error)
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `Json` | `string` | Yes | JSON string representing partial conversion options |

**Returns:** `ConversionOptionsUpdate`

**Errors:** Returns `error`.


---

#### InlineImageConfigFromJson()

Parse JSON string into `InlineImageConfig` (requires `inline-images` feature).

Deserializes a JSON string into inline image extraction configuration.
The JSON can be either a complete or partial configuration object.

**Returns:**

Fully populated `InlineImageConfig` with defaults applied to any unspecified values

**Errors:**

Returns `ConversionError.ConfigError` if JSON parsing fails or contains invalid configuration values

**Signature:**

```go
func InlineImageConfigFromJson(json string) (InlineImageConfig, error)
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `Json` | `string` | Yes | JSON string representing inline image configuration |

**Returns:** `InlineImageConfig`

**Errors:** Returns `error`.


---

#### MetadataConfigFromJson()

Parse JSON string into `MetadataConfig` (requires `metadata` feature).

Deserializes a JSON string into metadata extraction configuration.
The JSON can be either a complete or partial configuration object.

**Returns:**

Fully populated `MetadataConfig` with defaults applied to any unspecified values

**Errors:**

Returns `ConversionError.ConfigError` if JSON parsing fails or contains invalid configuration values

**Signature:**

```go
func MetadataConfigFromJson(json string) (MetadataConfig, error)
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `Json` | `string` | Yes | JSON string representing metadata extraction configuration |

**Returns:** `MetadataConfig`

**Errors:** Returns `error`.


---

### Types

#### ConversionOptions

Main conversion options for HTML to Markdown conversion.

Use `ConversionOptions.builder()` to construct, or `the default constructor` for defaults.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `HeadingStyle` | `HeadingStyle` | `HeadingStyle.Atx` | Heading style to use in Markdown output (ATX `#` or Setext underline). |
| `ListIndentType` | `ListIndentType` | `ListIndentType.Spaces` | How to indent nested list items (spaces or tab). |
| `ListIndentWidth` | `int` | `2` | Number of spaces (or tabs) to use for each level of list indentation. |
| `Bullets` | `string` | `"-*+"` | Bullet character(s) to use for unordered list items (e.g. `"-"`, `"*"`). |
| `StrongEmSymbol` | `string` | `"*"` | Character used for bold/italic emphasis markers (`*` or `_`). |
| `EscapeAsterisks` | `bool` | `false` | Escape `*` characters in plain text to avoid unintended bold/italic. |
| `EscapeUnderscores` | `bool` | `false` | Escape `_` characters in plain text to avoid unintended bold/italic. |
| `EscapeMisc` | `bool` | `false` | Escape miscellaneous Markdown metacharacters (`[]()#` etc.) in plain text. |
| `EscapeAscii` | `bool` | `false` | Escape ASCII characters that have special meaning in certain Markdown dialects. |
| `CodeLanguage` | `string` | `""` | Default language annotation for fenced code blocks that have no language hint. |
| `Autolinks` | `bool` | `true` | Automatically convert bare URLs into Markdown autolinks. |
| `DefaultTitle` | `bool` | `false` | Emit a default title when no `<title>` tag is present. |
| `BrInTables` | `bool` | `false` | Render `<br>` elements inside table cells as literal line breaks. |
| `HighlightStyle` | `HighlightStyle` | `HighlightStyle.DoubleEqual` | Style used for `<mark>` / highlighted text (e.g. `==text==`). |
| `ExtractMetadata` | `bool` | `true` | Extract `<meta>` and `<head>` information into the result metadata. |
| `WhitespaceMode` | `WhitespaceMode` | `WhitespaceMode.Normalized` | Controls how whitespace is normalised during conversion. |
| `StripNewlines` | `bool` | `false` | Strip all newlines from the output, producing a single-line result. |
| `Wrap` | `bool` | `false` | Wrap long lines at `wrap_width` characters. |
| `WrapWidth` | `int` | `80` | Maximum line width when `wrap` is enabled (default `80`). |
| `ConvertAsInline` | `bool` | `false` | Treat the entire document as inline content (no block-level wrappers). |
| `SubSymbol` | `string` | `""` | Markdown notation for subscript text (e.g. `"~"`). |
| `SupSymbol` | `string` | `""` | Markdown notation for superscript text (e.g. `"^"`). |
| `NewlineStyle` | `NewlineStyle` | `NewlineStyle.Spaces` | How to encode hard line breaks (`<br>`) in Markdown. |
| `CodeBlockStyle` | `CodeBlockStyle` | `CodeBlockStyle.Backticks` | Style used for fenced code blocks (backticks or tilde). |
| `KeepInlineImagesIn` | `[]string` | `nil` | HTML tag names whose `<img>` children are kept inline instead of block. |
| `Preprocessing` | `PreprocessingOptions` | — | Pre-processing options applied to the HTML before conversion. |
| `Encoding` | `string` | `"utf-8"` | Expected character encoding of the input HTML (default `"utf-8"`). |
| `Debug` | `bool` | `false` | Emit debug information during conversion. |
| `StripTags` | `[]string` | `nil` | HTML tag names whose content is stripped from the output entirely. |
| `PreserveTags` | `[]string` | `nil` | HTML tag names that are preserved verbatim in the output. |
| `SkipImages` | `bool` | `false` | Skip conversion of `<img>` elements (omit images from output). |
| `LinkStyle` | `LinkStyle` | `LinkStyle.Inline` | Link rendering style (inline or reference). |
| `OutputFormat` | `OutputFormat` | `OutputFormat.Markdown` | Target output format (Markdown, plain text, etc.). |
| `IncludeDocumentStructure` | `bool` | `false` | Include structured document tree in result. |
| `ExtractImages` | `bool` | `false` | Extract inline images from data URIs and SVGs. |
| `MaxImageSize` | `uint64` | `5242880` | Maximum decoded image size in bytes (default 5MB). |
| `CaptureSvg` | `bool` | `false` | Capture SVG elements as images. |
| `InferDimensions` | `bool` | `true` | Infer image dimensions from data. |

##### Methods

###### Default()

**Signature:**

```go
func (o *ConversionOptions) Default() ConversionOptions
```

###### Builder()

Create a new builder with default values.

**Signature:**

```go
func (o *ConversionOptions) Builder() ConversionOptionsBuilder
```

###### ApplyUpdate()

Apply a partial update to these conversion options.

**Signature:**

```go
func (o *ConversionOptions) ApplyUpdate(update ConversionOptionsUpdate)
```

###### FromUpdate()

Create from a partial update, applying to defaults.

**Signature:**

```go
func (o *ConversionOptions) FromUpdate(update ConversionOptionsUpdate) ConversionOptions
```

###### From()

**Signature:**

```go
func (o *ConversionOptions) From(update ConversionOptionsUpdate) ConversionOptions
```


---

#### ConversionResult

The primary result of HTML conversion and extraction.

Contains the converted text output, optional structured document tree,
metadata, extracted tables, images, and processing warnings.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `Content` | `*string` | `nil` | Converted text output (markdown, djot, or plain text). `None` when `output_format` is set to `OutputFormat.None`, indicating extraction-only mode. |
| `Document` | `*DocumentStructure` | `nil` | Structured document tree with semantic elements. Populated when `include_document_structure` is `True` in options. |
| `Metadata` | `HtmlMetadata` | — | Extracted HTML metadata (title, OG, links, images, structured data). |
| `Tables` | `[]TableData` | `nil` | Extracted tables with structured cell data and markdown representation. |
| `Images` | `[]InlineImage` | `nil` | Extracted inline images (data URIs and SVGs). Populated when `extract_images` is `True` in options. |
| `Warnings` | `[]ProcessingWarning` | `nil` | Non-fatal processing warnings. |


---

#### Context

Conversion context that tracks state during HTML to Markdown conversion.

This context is passed through the recursive tree walker and maintains information
about the current position in the document tree, nesting levels, and enabled features.


---

#### ConversionOptionsBuilder

Builder for `ConversionOptions`.

All fields start with default values. Call `.build()` to produce the final options.

##### Methods

###### StripTags()

Set the list of HTML tag names whose content is stripped from output.

**Signature:**

```go
func (o *ConversionOptionsBuilder) StripTags(tags []string) ConversionOptionsBuilder
```

###### PreserveTags()

Set the list of HTML tag names that are preserved verbatim in output.

**Signature:**

```go
func (o *ConversionOptionsBuilder) PreserveTags(tags []string) ConversionOptionsBuilder
```

###### KeepInlineImagesIn()

Set the list of HTML tag names whose `<img>` children are kept inline.

**Signature:**

```go
func (o *ConversionOptionsBuilder) KeepInlineImagesIn(tags []string) ConversionOptionsBuilder
```

###### Preprocessing()

Set the pre-processing options applied to the HTML before conversion.

**Signature:**

```go
func (o *ConversionOptionsBuilder) Preprocessing(preprocessing PreprocessingOptions) ConversionOptionsBuilder
```

###### Build()

Build the final `ConversionOptions`.

**Signature:**

```go
func (o *ConversionOptionsBuilder) Build() ConversionOptions
```


---

#### DjotRenderer

Renderer for Djot lightweight markup output.

##### Methods

###### Emphasis()

**Signature:**

```go
func (o *DjotRenderer) Emphasis(content string) string
```

###### Strong()

**Signature:**

```go
func (o *DjotRenderer) Strong(content string, symbol string) string
```

###### Strikethrough()

**Signature:**

```go
func (o *DjotRenderer) Strikethrough(content string) string
```

###### Highlight()

**Signature:**

```go
func (o *DjotRenderer) Highlight(content string) string
```

###### Inserted()

**Signature:**

```go
func (o *DjotRenderer) Inserted(content string) string
```

###### Subscript()

**Signature:**

```go
func (o *DjotRenderer) Subscript(content string, customSymbol string) string
```

###### Superscript()

**Signature:**

```go
func (o *DjotRenderer) Superscript(content string, customSymbol string) string
```

###### SpanWithAttributes()

**Signature:**

```go
func (o *DjotRenderer) SpanWithAttributes(content string, classes []string, id string) string
```

###### DivWithAttributes()

**Signature:**

```go
func (o *DjotRenderer) DivWithAttributes(content string, classes []string) string
```

###### IsDjot()

**Signature:**

```go
func (o *DjotRenderer) IsDjot() bool
```


---

#### DocumentMetadata

Document-level metadata extracted from `<head>` and top-level elements.

Contains all metadata typically used by search engines, social media platforms,
and browsers for document indexing and presentation.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `Title` | `*string` | `nil` | Document title from `<title>` tag |
| `Description` | `*string` | `nil` | Document description from `<meta name="description">` tag |
| `Keywords` | `[]string` | `nil` | Document keywords from `<meta name="keywords">` tag, split on commas |
| `Author` | `*string` | `nil` | Document author from `<meta name="author">` tag |
| `CanonicalUrl` | `*string` | `nil` | Canonical URL from `<link rel="canonical">` tag |
| `BaseHref` | `*string` | `nil` | Base URL from `<base href="">` tag for resolving relative URLs |
| `Language` | `*string` | `nil` | Document language from `lang` attribute |
| `TextDirection` | `*TextDirection` | `nil` | Document text direction from `dir` attribute |
| `OpenGraph` | `map[string]string` | `nil` | Open Graph metadata (og:* properties) for social media Keys like "title", "description", "image", "url", etc. |
| `TwitterCard` | `map[string]string` | `nil` | Twitter Card metadata (twitter:* properties) Keys like "card", "site", "creator", "title", "description", "image", etc. |
| `MetaTags` | `map[string]string` | `nil` | Additional meta tags not covered by specific fields Keys are meta name/property attributes, values are content |


---

#### DocumentNode

A single node in the document tree.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `Id` | `string` | — | Deterministic node identifier. |
| `Content` | `NodeContent` | — | The semantic content of this node. |
| `Parent` | `*uint32` | `nil` | Index of the parent node (None for root nodes). |
| `Children` | `[]uint32` | — | Indices of child nodes in reading order. |
| `Annotations` | `[]TextAnnotation` | — | Inline formatting annotations (bold, italic, links, etc.) with byte offsets into the text. |
| `Attributes` | `*map[string]string` | `nil` | Format-specific attributes (e.g. class, id, data-* attributes). |


---

#### DocumentStructure

A structured document tree representing the semantic content of an HTML document.

Uses a flat node array with index-based parent/child references for efficient traversal.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `Nodes` | `[]DocumentNode` | — | All nodes in document reading order. |
| `SourceFormat` | `*string` | `nil` | The source format (always "html" for this library). |


---

#### DomContext

DOM context that provides efficient access to parent/child relationships and text content.

This context is built once during conversion and provides O(1) access to node relationships
via precomputed maps. It also includes an LRU cache for text content extraction.


---

#### FormatRenderer

Trait for format-specific rendering of inline elements.

Implementations provide the syntax for emphasis, strong, strikethrough, etc.
in their respective output formats.

##### Methods

###### Emphasis()

Render emphasis (em, i elements)

**Signature:**

```go
func (o *FormatRenderer) Emphasis(content string) string
```

###### Strong()

Render strong emphasis (strong, b elements)

**Signature:**

```go
func (o *FormatRenderer) Strong(content string, symbol string) string
```

###### Strikethrough()

Render strikethrough (del, s elements)

**Signature:**

```go
func (o *FormatRenderer) Strikethrough(content string) string
```

###### Highlight()

Render highlight (mark element)

**Signature:**

```go
func (o *FormatRenderer) Highlight(content string) string
```

###### Inserted()

Render inserted text (ins element)

**Signature:**

```go
func (o *FormatRenderer) Inserted(content string) string
```

###### Subscript()

Render subscript (sub element)

**Signature:**

```go
func (o *FormatRenderer) Subscript(content string, customSymbol string) string
```

###### Superscript()

Render superscript (sup element)

**Signature:**

```go
func (o *FormatRenderer) Superscript(content string, customSymbol string) string
```

###### SpanWithAttributes()

Render span with attributes (for Djot: [text]{.class})

**Signature:**

```go
func (o *FormatRenderer) SpanWithAttributes(content string, classes []string, id string) string
```

###### DivWithAttributes()

Render div with attributes (for Djot: .: class)

**Signature:**

```go
func (o *FormatRenderer) DivWithAttributes(content string, classes []string) string
```

###### IsDjot()

Check if this is Djot format

**Signature:**

```go
func (o *FormatRenderer) IsDjot() bool
```


---

#### GridCell

A single cell in a table grid.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `Content` | `string` | — | The text content of the cell. |
| `Row` | `uint32` | — | 0-indexed row position. |
| `Col` | `uint32` | — | 0-indexed column position. |
| `RowSpan` | `uint32` | — | Number of rows this cell spans (default 1). |
| `ColSpan` | `uint32` | — | Number of columns this cell spans (default 1). |
| `IsHeader` | `bool` | — | Whether this is a header cell (`<th>`). |


---

#### HeaderMetadata

Header element metadata with hierarchy tracking.

Captures heading elements (h1-h6) with their text content, identifiers,
and position in the document structure.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `Level` | `uint8` | — | Header level: 1 (h1) through 6 (h6) |
| `Text` | `string` | — | Normalized text content of the header |
| `Id` | `*string` | `nil` | HTML id attribute if present |
| `Depth` | `int` | — | Document tree depth at the header element |
| `HtmlOffset` | `int` | — | Byte offset in original HTML document |

##### Methods

###### IsValid()

Validate that the header level is within valid range (1-6).

**Returns:**

`true` if level is 1-6, `false` otherwise.

**Signature:**

```go
func (o *HeaderMetadata) IsValid() bool
```


---

#### HtmlMetadata

Comprehensive metadata extraction result from HTML document.

Contains all extracted metadata types in a single structure,
suitable for serialization and transmission across language boundaries.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `Document` | `DocumentMetadata` | — | Document-level metadata (title, description, canonical, etc.) |
| `Headers` | `[]HeaderMetadata` | `nil` | Extracted header elements with hierarchy |
| `Links` | `[]LinkMetadata` | `nil` | Extracted hyperlinks with type classification |
| `Images` | `[]ImageMetadata` | `nil` | Extracted images with source and dimensions |
| `StructuredData` | `[]StructuredData` | `nil` | Extracted structured data blocks |


---

#### HtmlVisitor

Visitor trait for HTML→Markdown conversion.

Implement this trait to customize the conversion behavior for any HTML element type.
All methods have default implementations that return `VisitResult.Continue`, allowing
selective override of only the elements you care about.

## Method Naming Convention

- `visit_*_start`: Called before entering an element (pre-order traversal)
- `visit_*_end`: Called after exiting an element (post-order traversal)
- `visit_*`: Called for specific element types (e.g., `visit_link`, `visit_image`)

## Execution Order

For a typical element like `<div><p>text</p></div>`:

1. `visit_element_start` for `<div>`
2. `visit_element_start` for `<p>`
3. `visit_text` for "text"
4. `visit_element_end` for `<p>`
5. `visit_element_end` for `</div>`

## Performance Notes

- `visit_text` is the most frequently called method (~100+ times per document)
- Return `VisitResult.Continue` quickly for elements you don't need to customize
- Avoid heavy computation in visitor methods; consider caching if needed

### Methods

#### VisitElementStart()

Called before entering any element.

This is the first callback invoked for every HTML element, allowing
visitors to implement generic element handling before tag-specific logic.

**Signature:**

```go
func (o *HtmlVisitor) VisitElementStart(ctx NodeContext) VisitResult
```

##### VisitElementEnd()

Called after exiting any element.

Receives the default markdown output that would be generated.
Visitors can inspect or replace this output.

**Signature:**

```go
func (o *HtmlVisitor) VisitElementEnd(ctx NodeContext, output string) VisitResult
```

###### VisitText()

Visit text nodes (most frequent callback - ~100+ per document).

**Signature:**

```go
func (o *HtmlVisitor) VisitText(ctx NodeContext, text string) VisitResult
```

###### VisitLink()

Visit anchor links `<a href="...">`.

**Signature:**

```go
func (o *HtmlVisitor) VisitLink(ctx NodeContext, href string, text string, title string) VisitResult
```

###### VisitImage()

Visit images `<img src="...">`.

**Signature:**

```go
func (o *HtmlVisitor) VisitImage(ctx NodeContext, src string, alt string, title string) VisitResult
```

###### VisitHeading()

Visit heading elements `<h1>` through `<h6>`.

**Signature:**

```go
func (o *HtmlVisitor) VisitHeading(ctx NodeContext, level uint32, text string, id string) VisitResult
```

###### VisitCodeBlock()

Visit code blocks `<pre><code>`.

**Signature:**

```go
func (o *HtmlVisitor) VisitCodeBlock(ctx NodeContext, lang string, code string) VisitResult
```

###### VisitCodeInline()

Visit inline code `<code>`.

**Signature:**

```go
func (o *HtmlVisitor) VisitCodeInline(ctx NodeContext, code string) VisitResult
```

###### VisitListItem()

Visit list items `<li>`.

**Signature:**

```go
func (o *HtmlVisitor) VisitListItem(ctx NodeContext, ordered bool, marker string, text string) VisitResult
```

###### VisitListStart()

Called before processing a list `<ul>` or `<ol>`.

**Signature:**

```go
func (o *HtmlVisitor) VisitListStart(ctx NodeContext, ordered bool) VisitResult
```

###### VisitListEnd()

Called after processing a list `</ul>` or `</ol>`.

**Signature:**

```go
func (o *HtmlVisitor) VisitListEnd(ctx NodeContext, ordered bool, output string) VisitResult
```

###### VisitTableStart()

Called before processing a table `<table>`.

**Signature:**

```go
func (o *HtmlVisitor) VisitTableStart(ctx NodeContext) VisitResult
```

###### VisitTableRow()

Visit table rows `<tr>`.

**Signature:**

```go
func (o *HtmlVisitor) VisitTableRow(ctx NodeContext, cells []string, isHeader bool) VisitResult
```

###### VisitTableEnd()

Called after processing a table `</table>`.

**Signature:**

```go
func (o *HtmlVisitor) VisitTableEnd(ctx NodeContext, output string) VisitResult
```

###### VisitBlockquote()

Visit blockquote elements `<blockquote>`.

**Signature:**

```go
func (o *HtmlVisitor) VisitBlockquote(ctx NodeContext, content string, depth int) VisitResult
```

###### VisitStrong()

Visit strong/bold elements `<strong>`, `<b>`.

**Signature:**

```go
func (o *HtmlVisitor) VisitStrong(ctx NodeContext, text string) VisitResult
```

###### VisitEmphasis()

Visit emphasis/italic elements `<em>`, `<i>`.

**Signature:**

```go
func (o *HtmlVisitor) VisitEmphasis(ctx NodeContext, text string) VisitResult
```

###### VisitStrikethrough()

Visit strikethrough elements `<s>`, `<del>`, `<strike>`.

**Signature:**

```go
func (o *HtmlVisitor) VisitStrikethrough(ctx NodeContext, text string) VisitResult
```

###### VisitUnderline()

Visit underline elements `<u>`, `<ins>`.

**Signature:**

```go
func (o *HtmlVisitor) VisitUnderline(ctx NodeContext, text string) VisitResult
```

###### VisitSubscript()

Visit subscript elements `<sub>`.

**Signature:**

```go
func (o *HtmlVisitor) VisitSubscript(ctx NodeContext, text string) VisitResult
```

###### VisitSuperscript()

Visit superscript elements `<sup>`.

**Signature:**

```go
func (o *HtmlVisitor) VisitSuperscript(ctx NodeContext, text string) VisitResult
```

###### VisitMark()

Visit mark/highlight elements `<mark>`.

**Signature:**

```go
func (o *HtmlVisitor) VisitMark(ctx NodeContext, text string) VisitResult
```

###### VisitLineBreak()

Visit line break elements `<br>`.

**Signature:**

```go
func (o *HtmlVisitor) VisitLineBreak(ctx NodeContext) VisitResult
```

###### VisitHorizontalRule()

Visit horizontal rule elements `<hr>`.

**Signature:**

```go
func (o *HtmlVisitor) VisitHorizontalRule(ctx NodeContext) VisitResult
```

###### VisitCustomElement()

Visit custom elements (web components) or unknown tags.

**Signature:**

```go
func (o *HtmlVisitor) VisitCustomElement(ctx NodeContext, tagName string, html string) VisitResult
```

###### VisitDefinitionListStart()

Visit definition list `<dl>`.

**Signature:**

```go
func (o *HtmlVisitor) VisitDefinitionListStart(ctx NodeContext) VisitResult
```

###### VisitDefinitionTerm()

Visit definition term `<dt>`.

**Signature:**

```go
func (o *HtmlVisitor) VisitDefinitionTerm(ctx NodeContext, text string) VisitResult
```

###### VisitDefinitionDescription()

Visit definition description `<dd>`.

**Signature:**

```go
func (o *HtmlVisitor) VisitDefinitionDescription(ctx NodeContext, text string) VisitResult
```

###### VisitDefinitionListEnd()

Called after processing a definition list `</dl>`.

**Signature:**

```go
func (o *HtmlVisitor) VisitDefinitionListEnd(ctx NodeContext, output string) VisitResult
```

###### VisitForm()

Visit form elements `<form>`.

**Signature:**

```go
func (o *HtmlVisitor) VisitForm(ctx NodeContext, action string, method string) VisitResult
```

###### VisitInput()

Visit input elements `<input>`.

**Signature:**

```go
func (o *HtmlVisitor) VisitInput(ctx NodeContext, inputType string, name string, value string) VisitResult
```

###### VisitButton()

Visit button elements `<button>`.

**Signature:**

```go
func (o *HtmlVisitor) VisitButton(ctx NodeContext, text string) VisitResult
```

###### VisitAudio()

Visit audio elements `<audio>`.

**Signature:**

```go
func (o *HtmlVisitor) VisitAudio(ctx NodeContext, src string) VisitResult
```

###### VisitVideo()

Visit video elements `<video>`.

**Signature:**

```go
func (o *HtmlVisitor) VisitVideo(ctx NodeContext, src string) VisitResult
```

###### VisitIframe()

Visit iframe elements `<iframe>`.

**Signature:**

```go
func (o *HtmlVisitor) VisitIframe(ctx NodeContext, src string) VisitResult
```

###### VisitDetails()

Visit details elements `<details>`.

**Signature:**

```go
func (o *HtmlVisitor) VisitDetails(ctx NodeContext, open bool) VisitResult
```

###### VisitSummary()

Visit summary elements `<summary>`.

**Signature:**

```go
func (o *HtmlVisitor) VisitSummary(ctx NodeContext, text string) VisitResult
```

###### VisitFigureStart()

Visit figure elements `<figure>`.

**Signature:**

```go
func (o *HtmlVisitor) VisitFigureStart(ctx NodeContext) VisitResult
```

###### VisitFigcaption()

Visit figcaption elements `<figcaption>`.

**Signature:**

```go
func (o *HtmlVisitor) VisitFigcaption(ctx NodeContext, text string) VisitResult
```

###### VisitFigureEnd()

Called after processing a figure `</figure>`.

**Signature:**

```go
func (o *HtmlVisitor) VisitFigureEnd(ctx NodeContext, output string) VisitResult
```


---

#### ImageMetadata

Image metadata with source and dimensions.

Captures `<img>` elements and inline `<svg>` elements with metadata
for image analysis and optimization.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `Src` | `string` | — | Image source (URL, data URI, or SVG content identifier) |
| `Alt` | `*string` | `nil` | Alternative text from alt attribute (for accessibility) |
| `Title` | `*string` | `nil` | Title attribute (often shown as tooltip) |
| `Dimensions` | `*U32U32` | `nil` | Image dimensions as (width, height) if available |
| `ImageType` | `ImageType` | — | Image type classification |
| `Attributes` | `map[string]string` | — | Additional HTML attributes |


---

#### ImageMetadataPayload

Payload type for image metadata extraction.


---

#### InlineCollectorHandle

Handle type for inline image collector when feature is enabled.


---

#### InlineImageConfig

Inline image configuration that specifies contexts where images remain as markdown links.

This is a wrapper type that provides semantic clarity for the vector of element
names where inline images should be preserved.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `KeepInlineImagesIn` | `[]string` | `nil` | HTML elements where images should remain as markdown links (not converted to alt text) |

##### Methods

###### FromElements()

Create a new inline image configuration from a list of element names.

**Signature:**

```go
func (o *InlineImageConfig) FromElements(elements []string) InlineImageConfig
```

###### AddElement()

Add an element name to the list of elements where images are kept inline.

**Signature:**

```go
func (o *InlineImageConfig) AddElement(element string)
```

###### ShouldKeepImages()

Check if a given element should keep images inline.

**Returns:**

`true` if the element is in the configured list, `false` otherwise

**Signature:**

```go
func (o *InlineImageConfig) ShouldKeepImages(element string) bool
```

###### Default()

**Signature:**

```go
func (o *InlineImageConfig) Default() InlineImageConfig
```


---

#### LinkMetadata

Hyperlink metadata with categorization and attributes.

Represents `<a>` elements with parsed href values, text content, and link type classification.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `Href` | `string` | — | The href URL value |
| `Text` | `string` | — | Link text content (normalized, concatenated if mixed with elements) |
| `Title` | `*string` | `nil` | Optional title attribute (often shown as tooltip) |
| `LinkType` | `LinkType` | — | Link type classification |
| `Rel` | `[]string` | — | Rel attribute values (e.g., "nofollow", "stylesheet", "canonical") |
| `Attributes` | `map[string]string` | — | Additional HTML attributes |

##### Methods

###### ClassifyLink()

Classify a link based on href value.

**Returns:**

Appropriate `LinkType` based on protocol and content.

**Signature:**

```go
func (o *LinkMetadata) ClassifyLink(href string) LinkType
```


---

#### MarkdownRenderer

Renderer for standard Markdown output.

##### Methods

###### Emphasis()

**Signature:**

```go
func (o *MarkdownRenderer) Emphasis(content string) string
```

###### Strong()

**Signature:**

```go
func (o *MarkdownRenderer) Strong(content string, symbol string) string
```

###### Strikethrough()

**Signature:**

```go
func (o *MarkdownRenderer) Strikethrough(content string) string
```

###### Highlight()

**Signature:**

```go
func (o *MarkdownRenderer) Highlight(content string) string
```

###### Inserted()

**Signature:**

```go
func (o *MarkdownRenderer) Inserted(content string) string
```

###### Subscript()

**Signature:**

```go
func (o *MarkdownRenderer) Subscript(content string, customSymbol string) string
```

###### Superscript()

**Signature:**

```go
func (o *MarkdownRenderer) Superscript(content string, customSymbol string) string
```

###### SpanWithAttributes()

**Signature:**

```go
func (o *MarkdownRenderer) SpanWithAttributes(content string, classes []string, id string) string
```

###### DivWithAttributes()

**Signature:**

```go
func (o *MarkdownRenderer) DivWithAttributes(content string, classes []string) string
```

###### IsDjot()

**Signature:**

```go
func (o *MarkdownRenderer) IsDjot() bool
```


---

#### MetadataCollector

Internal metadata collector for single-pass extraction.

Follows a pattern for efficient metadata extraction during tree traversal.
Maintains state for:

- Document metadata from head elements
- Header hierarchy tracking
- Link accumulation
- Structured data collection
- Language and directionality attributes

## Architecture

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
| `ExtractDocument` | `bool` | `true` | Extract document-level metadata (title, description, author, etc.). When enabled, collects metadata from `<head>` section including: - `<title>` element content - `<meta name="description">` and other standard meta tags - Open Graph (og:*) properties for social media optimization - Twitter Card (twitter:*) properties - Language and text direction attributes - Canonical URL and base href references |
| `ExtractHeaders` | `bool` | `true` | Extract h1-h6 header elements and their hierarchy. When enabled, collects all heading elements with: - Header level (1-6) - Text content (normalized) - HTML id attribute if present - Document tree depth for hierarchy tracking - Byte offset in original HTML for positioning |
| `ExtractLinks` | `bool` | `true` | Extract anchor (a) elements as links with type classification. When enabled, collects all hyperlinks with: - href attribute value - Link text content - Title attribute (tooltip text) - Automatic link type classification (anchor, internal, external, email, phone, other) - Rel attribute values - Additional custom attributes |
| `ExtractImages` | `bool` | `true` | Extract image elements and data URIs. When enabled, collects all image elements with: - Source URL or data URI - Alt text for accessibility - Title attribute - Dimensions (width, height) if available - Automatic image type classification (data URI, external, relative, inline SVG) - Additional custom attributes |
| `ExtractStructuredData` | `bool` | `true` | Extract structured data (JSON-LD, Microdata, RDFa). When enabled, collects machine-readable structured data including: - JSON-LD script blocks with schema detection - Microdata attributes (itemscope, itemtype, itemprop) - RDFa markup - Extracted schema type if detectable |
| `MaxStructuredDataSize` | `int` | — | Maximum total size of structured data to collect (bytes). Prevents memory exhaustion attacks on malformed or adversarial documents containing excessively large structured data blocks. When the accumulated size of structured data exceeds this limit, further collection stops. Default: `1_000_000` bytes (1 MB) |

#### Methods

##### Default()

Create default metadata configuration.

Defaults to extracting all metadata types with 1MB limit on structured data.

**Signature:**

```go
func (o *MetadataConfig) Default() MetadataConfig
```

###### AnyEnabled()

Check if any metadata extraction is enabled.

Returns `true` if at least one extraction category is enabled, `false` if all are disabled.
This is useful for early exit optimization when the application doesn't need metadata.

**Returns:**

`true` if any of the extraction flags are enabled, `false` if all are disabled.

**Signature:**

```go
func (o *MetadataConfig) AnyEnabled() bool
```

###### ApplyUpdate()

Apply a partial update to this metadata configuration.

Any specified fields in the update (Some values) will override the current values.
Unspecified fields (None) are left unchanged. This allows selective modification
of configuration without affecting unrelated settings.

**Signature:**

```go
func (o *MetadataConfig) ApplyUpdate(update MetadataConfigUpdate)
```

###### FromUpdate()

Create new metadata configuration from a partial update.

Creates a new `MetadataConfig` struct with defaults, then applies the update.
Fields not specified in the update (None) keep their default values.
This is a convenience method for constructing a configuration from a partial specification
without needing to explicitly call `.default()` first.

**Returns:**

New `MetadataConfig` with specified updates applied to defaults

**Signature:**

```go
func (o *MetadataConfig) FromUpdate(update MetadataConfigUpdate) MetadataConfig
```

###### From()

**Signature:**

```go
func (o *MetadataConfig) From(update MetadataConfigUpdate) MetadataConfig
```


---

#### NodeContext

Context information passed to all visitor methods.

Provides comprehensive metadata about the current node being visited,
including its type, attributes, position in the DOM tree, and parent context.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `NodeType` | `NodeType` | — | Coarse-grained node type classification |
| `TagName` | `string` | — | Raw HTML tag name (e.g., "div", "h1", "custom-element") |
| `Attributes` | `map[string]string` | — | All HTML attributes as key-value pairs |
| `Depth` | `int` | — | Depth in the DOM tree (0 = root) |
| `IndexInParent` | `int` | — | Index among siblings (0-based) |
| `ParentTag` | `*string` | `nil` | Parent element's tag name (None if root) |
| `IsInline` | `bool` | — | Whether this element is treated as inline vs block |


---

#### PreprocessingOptions

HTML preprocessing options for document cleanup before conversion.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `Enabled` | `bool` | `true` | Enable HTML preprocessing globally |
| `Preset` | `PreprocessingPreset` | `PreprocessingPreset.Standard` | Preprocessing preset level (Minimal, Standard, Aggressive) |
| `RemoveNavigation` | `bool` | `true` | Remove navigation elements (nav, breadcrumbs, menus, sidebars) |
| `RemoveForms` | `bool` | `true` | Remove form elements (forms, inputs, buttons, etc.) |

##### Methods

###### Default()

**Signature:**

```go
func (o *PreprocessingOptions) Default() PreprocessingOptions
```

###### ApplyUpdate()

Apply a partial update to these preprocessing options.

Any specified fields in the update will override the current values.
Unspecified fields (None) are left unchanged.

**Signature:**

```go
func (o *PreprocessingOptions) ApplyUpdate(update PreprocessingOptionsUpdate)
```

###### FromUpdate()

Create new preprocessing options from a partial update.

Creates a new `PreprocessingOptions` struct with defaults, then applies the update.
Fields not specified in the update keep their default values.

**Returns:**

New `PreprocessingOptions` with specified updates applied to defaults

**Signature:**

```go
func (o *PreprocessingOptions) FromUpdate(update PreprocessingOptionsUpdate) PreprocessingOptions
```

###### From()

**Signature:**

```go
func (o *PreprocessingOptions) From(update PreprocessingOptionsUpdate) PreprocessingOptions
```


---

#### ProcessingWarning

A non-fatal warning generated during HTML processing.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `Message` | `string` | — | Human-readable warning message. |
| `Kind` | `WarningKind` | — | The category of warning. |


---

#### ReferenceCollector

Collects link/image references during conversion and produces a reference
definitions section at the end of the document.

##### Methods

###### GetOrInsert()

Register a URL (and optional title) and return its 1-based reference number.

If the same URL+title pair was already registered, the existing number is returned.

**Signature:**

```go
func (o *ReferenceCollector) GetOrInsert(url string, title string) int
```

###### Finish()

Produce the reference definitions section.

Returns an empty string when no references were collected.

**Signature:**

```go
func (o *ReferenceCollector) Finish() string
```


---

#### ReferenceCollectorHandle

Shared handle for passing the collector through the conversion context.


---

#### StructureCollector

Incremental builder for `DocumentStructure` during a single DOM walk.

##### Methods

###### PushHeading()

Record a heading element.

Creates a `NodeContent.Group` (which owns all subsequent sibling content until a
heading of equal or higher rank closes it) followed by a `NodeContent.Heading` child.

Returns the index of the **heading** node (the group node is one before it).

**Signature:**

```go
func (o *StructureCollector) PushHeading(level uint8, text string, id string) uint32
```

###### PushParagraph()

Record a paragraph element.

Returns the node index.

**Signature:**

```go
func (o *StructureCollector) PushParagraph(text string) uint32
```

###### PushListStart()

Open a list container.

Returns the node index; call `push_list_end` to close it.

**Signature:**

```go
func (o *StructureCollector) PushListStart(ordered bool) uint32
```

###### PushListEnd()

Close the innermost open list container.

**Signature:**

```go
func (o *StructureCollector) PushListEnd()
```

###### PushListItem()

Record a list item under the current open list.

If there is no open list, the item is parented under the current section/container.
Returns the node index.

**Signature:**

```go
func (o *StructureCollector) PushListItem(text string) uint32
```

###### PushTable()

Record a table.

Returns the node index.

**Signature:**

```go
func (o *StructureCollector) PushTable(grid TableGrid) uint32
```

###### PushImage()

Record an image element.

Returns the node index.

**Signature:**

```go
func (o *StructureCollector) PushImage(src string, alt string) uint32
```

###### PushCode()

Record a code block.

Returns the node index.

**Signature:**

```go
func (o *StructureCollector) PushCode(text string, language string) uint32
```

###### PushQuoteStart()

Open a blockquote container.

Returns the node index; call `push_quote_end` to close it.

**Signature:**

```go
func (o *StructureCollector) PushQuoteStart() uint32
```

###### PushQuoteEnd()

Close the innermost open blockquote container.

**Signature:**

```go
func (o *StructureCollector) PushQuoteEnd()
```

###### PushRawBlock()

Record a raw block (e.g. preserved `<script>` or `<style>` content).

Returns the node index.

**Signature:**

```go
func (o *StructureCollector) PushRawBlock(format string, content string) uint32
```

###### Finish()

Consume the collector and return the completed `DocumentStructure`.

**Signature:**

```go
func (o *StructureCollector) Finish() DocumentStructure
```

###### Default()

**Signature:**

```go
func (o *StructureCollector) Default() StructureCollector
```


---

#### StructureCollectorHandle

Shared mutable handle used in `crate.converter.Context`.


---

#### StructuredData

Structured data block (JSON-LD, Microdata, or RDFa).

Represents machine-readable structured data found in the document.
JSON-LD blocks are collected as raw JSON strings for flexibility.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `DataType` | `StructuredDataType` | — | Type of structured data (JSON-LD, Microdata, RDFa) |
| `RawJson` | `string` | — | Raw JSON string (for JSON-LD) or serialized representation |
| `SchemaType` | `*string` | `nil` | Schema type if detectable (e.g., "Article", "Event", "Product") |


---

#### TableData

A top-level extracted table with both structured data and markdown representation.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `Grid` | `TableGrid` | — | The structured table grid. |
| `Markdown` | `string` | — | The markdown rendering of this table. |


---

#### TableGrid

A structured table grid with cell-level data including spans.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `Rows` | `uint32` | — | Number of rows. |
| `Cols` | `uint32` | — | Number of columns. |
| `Cells` | `[]GridCell` | `nil` | All cells in the table (may be fewer than rows*cols due to spans). |


---

#### TableScan

Scan results for a table element.

Contains metadata about table structure to determine optimal rendering:

- Row counts for consistency checking
- Presence of headers, captions, and nested tables
- Presence of colspan/rowspan (spanning cells)
- Link and text content counts

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `RowCounts` | `[]int` | `nil` | Number of cells in each row |
| `HasSpan` | `bool` | — | Whether any cells have colspan or rowspan attributes |
| `HasHeader` | `bool` | — | Whether the table has header cells (th elements or role="head") |
| `HasCaption` | `bool` | — | Whether the table has a caption element |
| `NestedTableCount` | `int` | — | Number of nested tables found inside this table |
| `LinkCount` | `int` | — | Count of anchor elements in the table |
| `HasText` | `bool` | — | Whether the table contains text content (not empty) |


---

#### TextAnnotation

An inline text annotation with byte-range offsets.

Annotations describe formatting (bold, italic, etc.) and links within a node's text content.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `Start` | `uint32` | — | Start byte offset (inclusive) into the parent node's text. |
| `End` | `uint32` | — | End byte offset (exclusive) into the parent node's text. |
| `Kind` | `AnnotationKind` | — | The type of annotation. |


---

#### VisitorHandle

Type alias for a visitor handle (Rc-wrapped `RefCell` for interior mutability).

This allows visitors to be passed around and shared while still being mutable.


---

### Enums

#### VisitAction

Result of visitor element start callback indicating what should happen next.

| Value | Description |
|-------|-------------|
| `Continue` | Continue with normal element processing |
| `Skip` | Skip the element entirely (don't process children or call visit_element_end) |
| `Custom` | Custom output was provided, skip normal processing |
| `Error` | Error occurred during visitor callback |


---

#### TextDirection

Text directionality of document content.

Corresponds to the HTML `dir` attribute and `bdi` element directionality.

| Value | Description |
|-------|-------------|
| `LeftToRight` | Left-to-right text flow (default for Latin scripts) |
| `RightToLeft` | Right-to-left text flow (Hebrew, Arabic, Urdu, etc.) |
| `Auto` | Automatic directionality detection |


---

#### LinkType

Link classification based on href value and document context.

Used to categorize links during extraction for filtering and analysis.

| Value | Description |
|-------|-------------|
| `Anchor` | Anchor link within same document (href starts with #) |
| `Internal` | Internal link within same domain |
| `External` | External link to different domain |
| `Email` | Email link (mailto:) |
| `Phone` | Phone link (tel:) |
| `Other` | Other protocol or unclassifiable |


---

#### ImageType

Image source classification for proper handling and processing.

Determines whether an image is embedded (data URI), inline SVG, external, or relative.

| Value | Description |
|-------|-------------|
| `DataUri` | Data URI embedded image (base64 or other encoding) |
| `InlineSvg` | Inline SVG element |
| `External` | External image URL (http/https) |
| `Relative` | Relative image path |


---

#### StructuredDataType

Structured data format type.

Identifies the schema/format used for structured data markup.

| Value | Description |
|-------|-------------|
| `JsonLd` | JSON-LD (JSON for Linking Data) script blocks |
| `Microdata` | HTML5 Microdata attributes (itemscope, itemtype, itemprop) |
| `RDFa` | RDF in Attributes (RDFa) markup |


---

#### PreprocessingPreset

HTML preprocessing aggressiveness level.

Controls the extent of cleanup performed before conversion. Higher levels remove more elements.

| Value | Description |
|-------|-------------|
| `Minimal` | Minimal cleanup. Remove only essential noise (scripts, styles). |
| `Standard` | Standard cleanup. Default. Removes navigation, forms, and other auxiliary content. |
| `Aggressive` | Aggressive cleanup. Remove extensive non-content elements and structure. |


---

#### HeadingStyle

Heading style options for Markdown output.

Controls how headings (h1-h6) are rendered in the output Markdown.

| Value | Description |
|-------|-------------|
| `Underlined` | Underlined style (=== for h1, --- for h2). |
| `Atx` | ATX style (# for h1, ## for h2, etc.). Default. |
| `AtxClosed` | ATX closed style (# title #, with closing hashes). |


---

#### ListIndentType

List indentation character type.

Controls whether list items are indented with spaces or tabs.

| Value | Description |
|-------|-------------|
| `Spaces` | Use spaces for indentation. Default. Width controlled by `list_indent_width`. |
| `Tabs` | Use tabs for indentation. |


---

#### WhitespaceMode

Whitespace handling strategy during conversion.

Determines how sequences of whitespace characters (spaces, tabs, newlines) are processed.

| Value | Description |
|-------|-------------|
| `Normalized` | Collapse multiple whitespace characters to single spaces. Default. Matches browser behavior. |
| `Strict` | Preserve all whitespace exactly as it appears in the HTML. |


---

#### NewlineStyle

Line break syntax in Markdown output.

Controls how soft line breaks (from `<br>` or line breaks in source) are rendered.

| Value | Description |
|-------|-------------|
| `Spaces` | Two trailing spaces at end of line. Default. Standard Markdown syntax. |
| `Backslash` | Backslash at end of line. Alternative Markdown syntax. |


---

#### CodeBlockStyle

Code block fence style in Markdown output.

Determines how code blocks (`<pre><code>`) are rendered in Markdown.

| Value | Description |
|-------|-------------|
| `Indented` | Indented code blocks (4 spaces). `CommonMark` standard. |
| `Backticks` | Fenced code blocks with backticks (```). Default (GFM). Supports language hints. |
| `Tildes` | Fenced code blocks with tildes (~~~). Supports language hints. |


---

#### HighlightStyle

Highlight rendering style for `<mark>` elements.

Controls how highlighted text is rendered in Markdown output.

| Value | Description |
|-------|-------------|
| `DoubleEqual` | Double equals syntax (==text==). Default. Pandoc-compatible. |
| `Html` | Preserve as HTML (==text==). Original HTML tag. |
| `Bold` | Render as bold (**text**). Uses strong emphasis. |
| `None` | Strip formatting, render as plain text. No markup. |


---

#### LinkStyle

Link rendering style in Markdown output.

Controls whether links and images use inline `[text](url)` syntax or
reference-style `[text][1]` syntax with definitions collected at the end.

| Value | Description |
|-------|-------------|
| `Inline` | Inline links: `[text](url)`. Default. |
| `Reference` | Reference-style links: `[text][1]` with `[1]: url` at end of document. |


---

#### OutputFormat

Output format for conversion.

Specifies the target markup language format for the conversion output.

| Value | Description |
|-------|-------------|
| `Markdown` | Standard Markdown (CommonMark compatible). Default. |
| `Djot` | Djot lightweight markup language. |
| `Plain` | Plain text output (no markup, visible text only). |


---

#### NodeContent

The semantic content type of a document node.

Uses internally tagged representation (`"node_type": "heading"`) for JSON serialization.

| Value | Description |
|-------|-------------|
| `Heading` | A heading element (h1-h6). — Fields: `Level`: `uint8`, `Text`: `string` |
| `Paragraph` | A paragraph of text. — Fields: `Text`: `string` |
| `List` | A list container (ordered or unordered). Children are `ListItem` nodes. — Fields: `Ordered`: `bool` |
| `ListItem` | A single list item. — Fields: `Text`: `string` |
| `Table` | A table with structured cell data. — Fields: `Grid`: `TableGrid` |
| `Image` | An image element. — Fields: `Description`: `string`, `Src`: `string`, `ImageIndex`: `uint32` |
| `Code` | A code block or inline code. — Fields: `Text`: `string`, `Language`: `string` |
| `Quote` | A block quote container. |
| `DefinitionList` | A definition list container. |
| `DefinitionItem` | A definition list entry with term and description. — Fields: `Term`: `string`, `Definition`: `string` |
| `RawBlock` | A raw block preserved as-is (e.g. `<script>`, `<style>` content). — Fields: `Format`: `string`, `Content`: `string` |
| `MetadataBlock` | A block of key-value metadata pairs (from `<head>` meta tags). — Fields: `Entries`: `[]StringString` |
| `Group` | A section grouping container (auto-generated from heading hierarchy). — Fields: `Label`: `string`, `HeadingLevel`: `uint8`, `HeadingText`: `string` |


---

#### AnnotationKind

The type of an inline text annotation.

Uses internally tagged representation (`"annotation_type": "bold"`) for JSON serialization.

| Value | Description |
|-------|-------------|
| `Bold` | Bold / strong emphasis. |
| `Italic` | Italic / emphasis. |
| `Underline` | Underline. |
| `Strikethrough` | Strikethrough / deleted text. |
| `Code` | Inline code. |
| `Subscript` | Subscript text. |
| `Superscript` | Superscript text. |
| `Highlight` | Highlighted / marked text. |
| `Link` | A hyperlink. — Fields: `Url`: `string`, `Title`: `string` |


---

#### WarningKind

Categories of processing warnings.

| Value | Description |
|-------|-------------|
| `ImageExtractionFailed` | An image could not be extracted (e.g. invalid data URI, unsupported format). |
| `EncodingFallback` | The input encoding was not recognized; fell back to UTF-8. |
| `TruncatedInput` | The input was truncated due to size limits. |
| `MalformedHtml` | The HTML was malformed but processing continued with best effort. |
| `SanitizationApplied` | Sanitization was applied to remove potentially unsafe content. |


---

#### NodeType

Node type enumeration covering all HTML element types.

This enum categorizes all HTML elements that the converter recognizes,
providing a coarse-grained classification for visitor dispatch.

| Value | Description |
|-------|-------------|
| `Text` | Text node (most frequent - 100+ per document) |
| `Element` | Generic element node |
| `Heading` | Heading elements (h1-h6) |
| `Paragraph` | Paragraph element |
| `Div` | Generic div container |
| `Blockquote` | Blockquote element |
| `Pre` | Preformatted text block |
| `Hr` | Horizontal rule |
| `List` | Ordered or unordered list (ul, ol) |
| `ListItem` | List item (li) |
| `DefinitionList` | Definition list (dl) |
| `DefinitionTerm` | Definition term (dt) |
| `DefinitionDescription` | Definition description (dd) |
| `Table` | Table element |
| `TableRow` | Table row (tr) |
| `TableCell` | Table cell (td, th) |
| `TableHeader` | Table header cell (th) |
| `TableBody` | Table body (tbody) |
| `TableHead` | Table head (thead) |
| `TableFoot` | Table foot (tfoot) |
| `Link` | Anchor link (a) |
| `Image` | Image (img) |
| `Strong` | Strong/bold (strong, b) |
| `Em` | Emphasis/italic (em, i) |
| `Code` | Inline code (code) |
| `Strikethrough` | Strikethrough (s, del, strike) |
| `Underline` | Underline (u, ins) |
| `Subscript` | Subscript (sub) |
| `Superscript` | Superscript (sup) |
| `Mark` | Mark/highlight (mark) |
| `Small` | Small text (small) |
| `Br` | Line break (br) |
| `Span` | Span element |
| `Article` | Article element |
| `Section` | Section element |
| `Nav` | Navigation element |
| `Aside` | Aside element |
| `Header` | Header element |
| `Footer` | Footer element |
| `Main` | Main element |
| `Figure` | Figure element |
| `Figcaption` | Figure caption |
| `Time` | Time element |
| `Details` | Details element |
| `Summary` | Summary element |
| `Form` | Form element |
| `Input` | Input element |
| `Select` | Select element |
| `Option` | Option element |
| `Button` | Button element |
| `Textarea` | Textarea element |
| `Label` | Label element |
| `Fieldset` | Fieldset element |
| `Legend` | Legend element |
| `Audio` | Audio element |
| `Video` | Video element |
| `Picture` | Picture element |
| `Source` | Source element |
| `Iframe` | Iframe element |
| `Svg` | SVG element |
| `Canvas` | Canvas element |
| `Ruby` | Ruby annotation |
| `Rt` | Ruby text |
| `Rp` | Ruby parenthesis |
| `Abbr` | Abbreviation |
| `Kbd` | Keyboard input |
| `Samp` | Sample output |
| `Var` | Variable |
| `Cite` | Citation |
| `Q` | Quote |
| `Del` | Deleted text |
| `Ins` | Inserted text |
| `Data` | Data element |
| `Meter` | Meter element |
| `Progress` | Progress element |
| `Output` | Output element |
| `Template` | Template element |
| `Slot` | Slot element |
| `Html` | HTML root element |
| `Head` | Head element |
| `Body` | Body element |
| `Title` | Title element |
| `Meta` | Meta element |
| `LinkTag` | Link element (not anchor) |
| `Style` | Style element |
| `Script` | Script element |
| `Base` | Base element |
| `Custom` | Custom element (web components) or unknown tag |


---

#### VisitResult

Result of a visitor callback.

Allows visitors to control the conversion flow by either proceeding
with default behavior, providing custom output, skipping elements,
preserving HTML, or signaling errors.

| Value | Description |
|-------|-------------|
| `Continue` | Continue with default conversion behavior |
| `Custom` | Replace default output with custom markdown The visitor takes full responsibility for the markdown output of this node and its children. — Fields: `0`: `string` |
| `Skip` | Skip this element entirely (don't output anything) The element and all its children are ignored in the output. |
| `PreserveHtml` | Preserve original HTML (don't convert to markdown) The element's raw HTML is included verbatim in the output. |
| `Error` | Stop conversion with an error The conversion process halts and returns this error message. — Fields: `0`: `string` |


---

#### VisitorDispatch

Result of dispatching a visitor callback.

This enum represents the outcome of a visitor callback dispatch,
providing a more ergonomic interface for control flow than the
raw `VisitResult` type.

| Value | Description |
|-------|-------------|
| `Continue` | Continue with default conversion behavior |
| `Custom` | Replace default output with custom markdown — Fields: `0`: `string` |
| `Skip` | Skip this element entirely (don't output anything) |
| `PreserveHtml` | Preserve original HTML (don't convert to markdown) |


---

### Errors

#### ConversionError

Errors that can occur during HTML to Markdown conversion.

| Variant | Description |
|---------|-------------|
| `ParseError` | HTML parsing error |
| `SanitizationError` | HTML sanitization error |
| `ConfigError` | Invalid configuration |
| `IoError` | I/O error |
| `Panic` | Internal error caught during conversion |
| `InvalidInput` | Invalid input data |
| `Other` | Generic conversion error |


---
