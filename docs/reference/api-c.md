---
title: "C API Reference"
---

# C API Reference <span class="version-badge">v3.2.0</span>

## Functions

### htm_table_total_columns()

Calculate total columns in a table.

Scans all rows and cells to determine the maximum column count,
accounting for colspan values.

**Returns:**
Maximum column count (minimum 1, maximum MAX_TABLE_COLS)

**Signature:**

```c
uintptr_t htm_table_total_columns(HtmNodeHandle node_handle, HtmParser parser, HtmDomContext dom_ctx);
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `node_handle` | `HtmNodeHandle` | Yes | Handle to the table element |
| `parser` | `HtmParser` | Yes | HTML parser instance |
| `dom_ctx` | `HtmDomContext` | Yes | DOM context for tag name resolution |

**Returns:** `uintptr_t`


---

### htm_handle_table()

Convert an entire table element to Markdown.

Main entry point for table conversion. Analyzes table structure to determine
if it should be rendered as a Markdown table or converted to list format.
Handles layout tables, blank tables, and tables with semantic meaning.
Integrates with visitor pattern for custom table handling.

**Signature:**

```c
void htm_handle_table(HtmNodeHandle node_handle, HtmParser parser, const char* output, HtmConversionOptions options, HtmContext ctx, HtmDomContext dom_ctx, uintptr_t depth);
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `node_handle` | `HtmNodeHandle` | Yes | Handle to the table element |
| `parser` | `HtmParser` | Yes | HTML parser instance |
| `output` | `const char*` | Yes | Mutable string to append table content |
| `options` | `HtmConversionOptions` | Yes | Conversion options |
| `ctx` | `HtmContext` | Yes | Conversion context (visitor, etc) |
| `dom_ctx` | `HtmDomContext` | Yes | DOM context |
| `depth` | `uintptr_t` | Yes | Nesting depth |

**Returns:** `void`


---

### htm_handle_caption()

Handles caption elements within tables.

Extracts text content from the caption and formats it as italicized text
with escaped hyphens to prevent Markdown table separator interpretation.

**Signature:**

```c
void htm_handle_caption(HtmNodeHandle node_handle, HtmParser parser, const char* output, HtmConversionOptions options, HtmContext ctx, uintptr_t depth, HtmDomContext dom_ctx);
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `node_handle` | `HtmNodeHandle` | Yes | Handle to the caption element |
| `parser` | `HtmParser` | Yes | HTML parser instance |
| `output` | `const char*` | Yes | Output string to append caption text to |
| `options` | `HtmConversionOptions` | Yes | Conversion options |
| `ctx` | `HtmContext` | Yes | Conversion context |
| `depth` | `uintptr_t` | Yes | Current recursion depth |
| `dom_ctx` | `HtmDomContext` | Yes | DOM context for tag name resolution |

**Returns:** `void`


---

### htm_get_colspan()

Get colspan attribute value from an element.

Reads the colspan attribute from a table cell, with bounds checking
to prevent memory exhaustion attacks.

**Returns:**
The colspan value (minimum 1, maximum MAX_TABLE_COLS)

**Signature:**

```c
uintptr_t htm_get_colspan(HtmNodeHandle node_handle, HtmParser parser);
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `node_handle` | `HtmNodeHandle` | Yes | Handle to the cell element |
| `parser` | `HtmParser` | Yes | HTML parser instance |

**Returns:** `uintptr_t`


---

### htm_get_colspan_rowspan()

Get both colspan and rowspan in a single lookup.

More efficient than calling get_colspan and a separate rowspan lookup.

**Returns:**
A tuple of (colspan, rowspan), both minimum 1 and maximum MAX_TABLE_COLS

**Signature:**

```c
HtmUsizeUsize* htm_get_colspan_rowspan(HtmNodeHandle node_handle, HtmParser parser);
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `node_handle` | `HtmNodeHandle` | Yes | Handle to the cell element |
| `parser` | `HtmParser` | Yes | HTML parser instance |

**Returns:** `HtmUsizeUsize`


---

### htm_collect_table_cells()

Collect table cells (td/th) from a row element.

Extracts only the direct cell children of a row, filtering by tag name.

**Signature:**

```c
void htm_collect_table_cells(HtmNodeHandle node_handle, HtmParser parser, HtmDomContext dom_ctx, HtmNodeHandle* cells);
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `node_handle` | `HtmNodeHandle` | Yes | Handle to the row element |
| `parser` | `HtmParser` | Yes | HTML parser instance |
| `dom_ctx` | `HtmDomContext` | Yes | DOM context for tag name resolution |
| `cells` | `HtmNodeHandle*` | Yes | Mutable vector to populate with cell handles |

**Returns:** `void`


---

### htm_convert_table_cell()

Convert a table cell (td or th) to Markdown format.

Processes cell content and renders it with pipe delimiters for Markdown tables.
Handles colspan by adding extra pipes, and escapes pipes in cell content.

**Signature:**

```c
void htm_convert_table_cell(HtmNodeHandle node_handle, HtmParser parser, const char* output, HtmConversionOptions options, HtmContext ctx, const char* tag_name, HtmDomContext dom_ctx);
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `node_handle` | `HtmNodeHandle` | Yes | Handle to the cell element |
| `parser` | `HtmParser` | Yes | HTML parser instance |
| `output` | `const char*` | Yes | Mutable string to append cell content |
| `options` | `HtmConversionOptions` | Yes | Conversion options (escape settings, br_in_tables) |
| `ctx` | `HtmContext` | Yes | Conversion context (visitor, etc) |
| `tag_name` | `const char*` | Yes | Tag name (for consistency, not used) |
| `dom_ctx` | `HtmDomContext` | Yes | DOM context for content extraction |

**Returns:** `void`


---

### htm_append_layout_row()

Append a layout table row as a list item.

For tables used for visual layout, converts rows to list items
instead of table format for better readability.

**Signature:**

```c
void htm_append_layout_row(HtmNodeHandle row_handle, HtmParser parser, const char* output, HtmConversionOptions options, HtmContext ctx, HtmDomContext dom_ctx);
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `row_handle` | `HtmNodeHandle` | Yes | Handle to the row element |
| `parser` | `HtmParser` | Yes | HTML parser instance |
| `output` | `const char*` | Yes | Mutable string to append content |
| `options` | `HtmConversionOptions` | Yes | Conversion options |
| `ctx` | `HtmContext` | Yes | Conversion context |
| `dom_ctx` | `HtmDomContext` | Yes | DOM context |

**Returns:** `void`


---

### htm_convert_table_row()

Convert a table row (tr) to Markdown format.

Processes all cells in a row, handling colspan and rowspan for proper
column alignment. Renders header separator row after the first row.
Integrates with visitor pattern for custom row handling.

**Signature:**

```c
void htm_convert_table_row(HtmNodeHandle node_handle, HtmParser parser, const char* output, HtmConversionOptions options, HtmContext ctx, uintptr_t row_index, bool has_span, uintptr_t** rowspan_tracker, uintptr_t total_cols, uintptr_t header_cols, HtmDomContext dom_ctx, uintptr_t depth, bool is_header);
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `node_handle` | `HtmNodeHandle` | Yes | Handle to the row element |
| `parser` | `HtmParser` | Yes | HTML parser instance |
| `output` | `const char*` | Yes | Mutable string to append row content |
| `options` | `HtmConversionOptions` | Yes | Conversion options |
| `ctx` | `HtmContext` | Yes | Conversion context (visitor, etc) |
| `row_index` | `uintptr_t` | Yes | Index of this row in the table |
| `has_span` | `bool` | Yes | Whether table has colspan/rowspan |
| `rowspan_tracker` | `uintptr_t**` | Yes | Mutable array tracking rowspan remainder for each column |
| `total_cols` | `uintptr_t` | Yes | Total columns in the table |
| `header_cols` | `uintptr_t` | Yes | Columns to render in separator row |
| `dom_ctx` | `HtmDomContext` | Yes | DOM context |
| `depth` | `uintptr_t` | Yes | Nesting depth |
| `is_header` | `bool` | Yes | Whether this is a header row |

**Returns:** `void`


---

### htm_scan_table()

Scan a table element for structural metadata.

Analyzes the table to determine characteristics that influence rendering:
- Whether to render as a Markdown table or layout table
- If spanning cells are present
- If the table has semantic meaning (headers, captions)

**Signature:**

```c
HtmTableScan* htm_scan_table(HtmNodeHandle node_handle, HtmParser parser, HtmDomContext dom_ctx);
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `node_handle` | `HtmNodeHandle` | Yes | Handle to the table element |
| `parser` | `HtmParser` | Yes | HTML parser instance |
| `dom_ctx` | `HtmDomContext` | Yes | DOM context for tag name resolution |

**Returns:** `HtmTableScan`


---

### htm_dispatch_table_handler()

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

```c
bool htm_dispatch_table_handler(const char* tag_name, HtmNodeHandle node_handle, HtmParser parser, const char* output, HtmConversionOptions options, HtmContext ctx, uintptr_t depth, HtmDomContext dom_ctx);
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `tag_name` | `const char*` | Yes | The tag name |
| `node_handle` | `HtmNodeHandle` | Yes | The node handle |
| `parser` | `HtmParser` | Yes | The parser |
| `output` | `const char*` | Yes | The output destination |
| `options` | `HtmConversionOptions` | Yes | The options to use |
| `ctx` | `HtmContext` | Yes | The context |
| `depth` | `uintptr_t` | Yes | The depth |
| `dom_ctx` | `HtmDomContext` | Yes | The dom context |

**Returns:** `bool`


---

### htm_dispatch_block_handler()

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

```c
bool htm_dispatch_block_handler(const char* tag_name, HtmNodeHandle node_handle, HtmParser parser, const char* output, HtmConversionOptions options, HtmContext ctx, uintptr_t depth, HtmDomContext dom_ctx);
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `tag_name` | `const char*` | Yes | The tag name |
| `node_handle` | `HtmNodeHandle` | Yes | The node handle |
| `parser` | `HtmParser` | Yes | The parser |
| `output` | `const char*` | Yes | The output destination |
| `options` | `HtmConversionOptions` | Yes | The options to use |
| `ctx` | `HtmContext` | Yes | The context |
| `depth` | `uintptr_t` | Yes | The depth |
| `dom_ctx` | `HtmDomContext` | Yes | The dom context |

**Returns:** `bool`


---

### htm_handle()

Dispatcher for form elements.

Routes all form-related elements to their respective handlers.

**Signature:**

```c
void htm_handle(const char* tag_name, HtmNodeHandle node_handle, HtmParser parser, const char* output, HtmConversionOptions options, HtmContext ctx, uintptr_t depth, HtmDomContext dom_ctx);
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `tag_name` | `const char*` | Yes | The tag name |
| `node_handle` | `HtmNodeHandle` | Yes | The node handle |
| `parser` | `HtmParser` | Yes | The parser |
| `output` | `const char*` | Yes | The output destination |
| `options` | `HtmConversionOptions` | Yes | The options to use |
| `ctx` | `HtmContext` | Yes | The context |
| `depth` | `uintptr_t` | Yes | The depth |
| `dom_ctx` | `HtmDomContext` | Yes | The dom context |

**Returns:** `void`


---

### htm_dispatch_form_handler()

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

```c
bool htm_dispatch_form_handler(const char* tag_name, HtmNodeHandle node_handle, HtmParser parser, const char* output, HtmConversionOptions options, HtmContext ctx, uintptr_t depth, HtmDomContext dom_ctx);
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `tag_name` | `const char*` | Yes | The tag name |
| `node_handle` | `HtmNodeHandle` | Yes | The node handle |
| `parser` | `HtmParser` | Yes | The parser |
| `output` | `const char*` | Yes | The output destination |
| `options` | `HtmConversionOptions` | Yes | The options to use |
| `ctx` | `HtmContext` | Yes | The context |
| `depth` | `uintptr_t` | Yes | The depth |
| `dom_ctx` | `HtmDomContext` | Yes | The dom context |

**Returns:** `bool`


---

### htm_handle_blockquote()

Handle a `<blockquote>` element and convert to Markdown.

This handler processes blockquote elements including:
- Converting inline blockquotes by processing children as inline
- Handling nested blockquotes via blockquote_depth tracking
- Processing citation URLs from cite attribute
- Invoking visitor callbacks when the visitor feature is enabled
- Adding proper spacing and blockquote prefix formatting

**Signature:**

```c
void htm_handle_blockquote(HtmNodeHandle node_handle, HtmHtmlTag tag, HtmParser parser, const char* output, HtmConversionOptions options, HtmContext ctx, uintptr_t depth, HtmDomContext dom_ctx);
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `node_handle` | `HtmNodeHandle` | Yes | The node handle |
| `tag` | `HtmHtmlTag` | Yes | The h t m l tag |
| `parser` | `HtmParser` | Yes | The parser |
| `output` | `const char*` | Yes | The output destination |
| `options` | `HtmConversionOptions` | Yes | The options to use |
| `ctx` | `HtmContext` | Yes | The context |
| `depth` | `uintptr_t` | Yes | The depth |
| `dom_ctx` | `HtmDomContext` | Yes | The dom context |

**Returns:** `void`


---

### htm_handle_code()

Handle an inline `<code>` element and convert to Markdown.

This handler processes inline code elements including:
- Extracting code content and applying backtick delimiters
- Handling backticks in content by using multiple delimiters
- Invoking visitor callbacks when the visitor feature is enabled
- Generating appropriate markdown output with proper escaping

**Signature:**

```c
void htm_handle_code(HtmNodeHandle node_handle, HtmHtmlTag tag, HtmParser parser, const char* output, HtmConversionOptions options, HtmContext ctx, uintptr_t depth, HtmDomContext dom_ctx);
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `node_handle` | `HtmNodeHandle` | Yes | The node handle |
| `tag` | `HtmHtmlTag` | Yes | The h t m l tag |
| `parser` | `HtmParser` | Yes | The parser |
| `output` | `const char*` | Yes | The output destination |
| `options` | `HtmConversionOptions` | Yes | The options to use |
| `ctx` | `HtmContext` | Yes | The context |
| `depth` | `uintptr_t` | Yes | The depth |
| `dom_ctx` | `HtmDomContext` | Yes | The dom context |

**Returns:** `void`


---

### htm_handle_pre()

Handle a `<pre>` element and convert to Markdown.

This handler processes code block elements including:
- Extracting language information from class attributes
- Processing whitespace and dedenting code content
- Supporting multiple code block styles (indented, backticks, tildes)
- Invoking visitor callbacks when the visitor feature is enabled
- Generating appropriate markdown output

**Signature:**

```c
void htm_handle_pre(HtmNodeHandle node_handle, HtmHtmlTag tag, HtmParser parser, const char* output, HtmConversionOptions options, HtmContext ctx, uintptr_t depth, HtmDomContext dom_ctx);
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `node_handle` | `HtmNodeHandle` | Yes | The node handle |
| `tag` | `HtmHtmlTag` | Yes | The h t m l tag |
| `parser` | `HtmParser` | Yes | The parser |
| `output` | `const char*` | Yes | The output destination |
| `options` | `HtmConversionOptions` | Yes | The options to use |
| `ctx` | `HtmContext` | Yes | The context |
| `depth` | `uintptr_t` | Yes | The depth |
| `dom_ctx` | `HtmDomContext` | Yes | The dom context |

**Returns:** `void`


---

### htm_handle_graphic()

Handle a `<graphic>` element and convert to Markdown.

This handler processes graphic elements including:
- Extracting source from url, href, xlink:href, or src attributes
- Using alt attribute, with fallback to filename
- Collecting metadata when the metadata feature is enabled
- Invoking visitor callbacks when the visitor feature is enabled
- Generating appropriate markdown output

**Signature:**

```c
void htm_handle_graphic(HtmNodeHandle node_handle, HtmHtmlTag tag, HtmParser parser, const char* output, HtmConversionOptions options, HtmContext ctx, uintptr_t depth, HtmDomContext dom_ctx);
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `node_handle` | `HtmNodeHandle` | Yes | The node handle |
| `tag` | `HtmHtmlTag` | Yes | The h t m l tag |
| `parser` | `HtmParser` | Yes | The parser |
| `output` | `const char*` | Yes | The output destination |
| `options` | `HtmConversionOptions` | Yes | The options to use |
| `ctx` | `HtmContext` | Yes | The context |
| `depth` | `uintptr_t` | Yes | The depth |
| `dom_ctx` | `HtmDomContext` | Yes | The dom context |

**Returns:** `void`


---

### htm_handle_img()

Handle an `<img>` element and convert to Markdown.

This handler processes image elements including:
- Extracting src, alt, and title attributes
- Collecting metadata when the metadata feature is enabled
- Handling inline data URIs when the inline-images feature is enabled
- Invoking visitor callbacks when the visitor feature is enabled
- Generating appropriate markdown output

**Signature:**

```c
void htm_handle_img(HtmNodeHandle node_handle, HtmHtmlTag tag, HtmParser parser, const char* output, HtmConversionOptions options, HtmContext ctx, uintptr_t depth, HtmDomContext dom_ctx);
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `node_handle` | `HtmNodeHandle` | Yes | The node handle |
| `tag` | `HtmHtmlTag` | Yes | The h t m l tag |
| `parser` | `HtmParser` | Yes | The parser |
| `output` | `const char*` | Yes | The output destination |
| `options` | `HtmConversionOptions` | Yes | The options to use |
| `ctx` | `HtmContext` | Yes | The context |
| `depth` | `uintptr_t` | Yes | The depth |
| `dom_ctx` | `HtmDomContext` | Yes | The dom context |

**Returns:** `void`


---

### htm_handle_link()

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

```c
void htm_handle_link(HtmNodeHandle node_handle, HtmHtmlTag tag, HtmParser parser, const char* output, HtmConversionOptions options, HtmContext ctx, uintptr_t depth, HtmDomContext dom_ctx);
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `node_handle` | `HtmNodeHandle` | Yes | The node handle |
| `tag` | `HtmHtmlTag` | Yes | The h t m l tag |
| `parser` | `HtmParser` | Yes | The parser |
| `output` | `const char*` | Yes | The output destination |
| `options` | `HtmConversionOptions` | Yes | The options to use |
| `ctx` | `HtmContext` | Yes | The context |
| `depth` | `uintptr_t` | Yes | The depth |
| `dom_ctx` | `HtmDomContext` | Yes | The dom context |

**Returns:** `void`


---

### htm_dispatch_inline_handler()

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

```c
bool htm_dispatch_inline_handler(const char* tag_name, HtmNodeHandle node_handle, HtmParser parser, const char* output, HtmConversionOptions options, HtmContext ctx, uintptr_t depth, HtmDomContext dom_ctx);
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `tag_name` | `const char*` | Yes | The normalized HTML tag name (lowercase) |
| `node_handle` | `HtmNodeHandle` | Yes | The DOM node handle from the parser |
| `parser` | `HtmParser` | Yes | Reference to the tl HTML parser |
| `output` | `const char*` | Yes | Output buffer to write converted content to |
| `options` | `HtmConversionOptions` | Yes | Conversion configuration options |
| `ctx` | `HtmContext` | Yes | Processing context with state tracking |
| `depth` | `uintptr_t` | Yes | Current DOM tree depth for recursion tracking |
| `dom_ctx` | `HtmDomContext` | Yes | DOM context for accessing tree structure |

**Returns:** `bool`


---

### htm_calculate_list_continuation_indent()

Calculate indentation level for list item continuations.

Returns the number of 4-space indent groups needed for list continuations.

List continuations (block elements inside list items) need special indentation:
- Base indentation: (depth - 1) groups (for the nesting level)
- Content indentation: depth groups (for the list item content)
- Combined formula: (2 * depth - 1) groups of 4 spaces each

**Signature:**

```c
uintptr_t htm_calculate_list_continuation_indent(uintptr_t depth);
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `depth` | `uintptr_t` | Yes | The depth |

**Returns:** `uintptr_t`


---

### htm_is_loose_list()

Check if a list (ul or ol) is "loose".

A loose list is one where any list item contains block-level elements
like paragraphs (<p>). In loose lists, all items should have blank line
separation (ending with \n\n) regardless of their own content.

**Signature:**

```c
bool htm_is_loose_list(HtmNodeHandle node_handle, HtmParser parser, HtmDomContext dom_ctx);
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `node_handle` | `HtmNodeHandle` | Yes | The node handle |
| `parser` | `HtmParser` | Yes | The parser |
| `dom_ctx` | `HtmDomContext` | Yes | The dom context |

**Returns:** `bool`


---

### htm_add_list_continuation_indent()

Add list continuation indentation to output.

Used when block elements (like <p> or <div>) appear inside list items.
Adds appropriate line separation and indentation to continue the list item.

**Signature:**

```c
void htm_add_list_continuation_indent(const char* output, uintptr_t list_depth, bool blank_line, HtmConversionOptions options);
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `output` | `const char*` | Yes | The output string to append to |
| `list_depth` | `uintptr_t` | Yes | Current list nesting depth |
| `blank_line` | `bool` | Yes | If true, adds blank line separation (\n\n); if false, single newline (\n) |
| `options` | `HtmConversionOptions` | Yes | The options to use |

**Returns:** `void`


---

### htm_continuation_indent_string()

Calculate the indentation string for list continuations based on depth and options.

**Signature:**

```c
const char** htm_continuation_indent_string(uintptr_t list_depth, HtmConversionOptions options);
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `list_depth` | `uintptr_t` | Yes | The list depth |
| `options` | `HtmConversionOptions` | Yes | The options to use |

**Returns:** `const char**`


---

### htm_add_list_leading_separator()

Add appropriate leading separator before a list.

Lists need different separators depending on context:
- In table cells: <br> tag if there's already content
- Outside lists: blank line (\n\n) if needed
- Inside list items: blank line before nested list

**Signature:**

```c
void htm_add_list_leading_separator(const char* output, HtmContext ctx);
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `output` | `const char*` | Yes | The output destination |
| `ctx` | `HtmContext` | Yes | The context |

**Returns:** `void`


---

### htm_add_nested_list_trailing_separator()

Add appropriate trailing separator after a nested list.

Nested lists inside list items need trailing newlines to separate
from following content. In loose lists, use blank line (\n\n). In tight lists, single newline (\n).

**Signature:**

```c
void htm_add_nested_list_trailing_separator(const char* output, HtmContext ctx);
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `output` | `const char*` | Yes | The output destination |
| `ctx` | `HtmContext` | Yes | The context |

**Returns:** `void`


---

### htm_calculate_list_nesting_depth()

Calculate the nesting depth for a list.

If we're in a list but NOT in a list item, this is incorrectly nested HTML
and we need to increment the depth. If in a list item, the depth was already
incremented by the <li> element.

**Signature:**

```c
uintptr_t htm_calculate_list_nesting_depth(HtmContext ctx);
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `ctx` | `HtmContext` | Yes | The context |

**Returns:** `uintptr_t`


---

### htm_is_list_item()

Check if a node is a list item element.

**Signature:**

```c
bool htm_is_list_item(HtmNodeHandle node_handle, HtmParser parser, HtmDomContext dom_ctx);
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `node_handle` | `HtmNodeHandle` | Yes | The node handle |
| `parser` | `HtmParser` | Yes | The parser |
| `dom_ctx` | `HtmDomContext` | Yes | The dom context |

**Returns:** `bool`


---

### htm_process_list_children()

Process a list's children, tracking which items had block elements.

This is used to determine proper spacing between list items.
Returns true if the last processed item had block children.

**Signature:**

```c
void htm_process_list_children(HtmNodeHandle node_handle, HtmParser parser, const char* output, HtmConversionOptions options, HtmContext ctx, uintptr_t depth, bool is_ordered, bool is_loose, uintptr_t nested_depth, uintptr_t start_counter, HtmDomContext dom_ctx);
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `node_handle` | `HtmNodeHandle` | Yes | The node handle |
| `parser` | `HtmParser` | Yes | The parser |
| `output` | `const char*` | Yes | The output destination |
| `options` | `HtmConversionOptions` | Yes | The options to use |
| `ctx` | `HtmContext` | Yes | The context |
| `depth` | `uintptr_t` | Yes | The depth |
| `is_ordered` | `bool` | Yes | The is ordered |
| `is_loose` | `bool` | Yes | The is loose |
| `nested_depth` | `uintptr_t` | Yes | The nested depth |
| `start_counter` | `uintptr_t` | Yes | The start counter |
| `dom_ctx` | `HtmDomContext` | Yes | The dom context |

**Returns:** `void`


---

### htm_dispatch_list_handler()

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

```c
bool htm_dispatch_list_handler(const char* tag_name, HtmNodeHandle node_handle, HtmHtmlTag tag, HtmParser parser, const char* output, HtmConversionOptions options, HtmContext ctx, uintptr_t depth, HtmDomContext dom_ctx);
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `tag_name` | `const char*` | Yes | The tag name |
| `node_handle` | `HtmNodeHandle` | Yes | The node handle |
| `tag` | `HtmHtmlTag` | Yes | The h t m l tag |
| `parser` | `HtmParser` | Yes | The parser |
| `output` | `const char*` | Yes | The output destination |
| `options` | `HtmConversionOptions` | Yes | The options to use |
| `ctx` | `HtmContext` | Yes | The context |
| `depth` | `uintptr_t` | Yes | The depth |
| `dom_ctx` | `HtmDomContext` | Yes | The dom context |

**Returns:** `bool`


---

### htm_convert_html()

Converts HTML to Markdown using the provided conversion options.

This is the main entry point for HTML to Markdown conversion.

**Signature:**

```c
const char* htm_convert_html(const char* html, HtmConversionOptions options);
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `html` | `const char*` | Yes | The html |
| `options` | `HtmConversionOptions` | Yes | The options to use |

**Returns:** `const char*`

**Errors:** Returns `NULL` on error.


---

### htm_convert_html_with_visitor()

Converts HTML to Markdown with a custom visitor for callbacks during traversal.

This variant allows passing a visitor that will receive callbacks for each node
during the tree walk, enabling custom processing or analysis.

**Signature:**

```c
const char* htm_convert_html_with_visitor(const char* html, HtmConversionOptions options, HtmVisitorHandle visitor);
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `html` | `const char*` | Yes | The html |
| `options` | `HtmConversionOptions` | Yes | The options to use |
| `visitor` | `HtmVisitorHandle*` | No | The visitor handle |

**Returns:** `const char*`

**Errors:** Returns `NULL` on error.


---

### htm_dispatch_media_handler()

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

```c
bool htm_dispatch_media_handler(const char* tag_name, HtmNodeHandle node_handle, HtmParser parser, const char* output, HtmConversionOptions options, HtmContext ctx, uintptr_t depth, HtmDomContext dom_ctx);
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `tag_name` | `const char*` | Yes | The tag name |
| `node_handle` | `HtmNodeHandle` | Yes | The node handle |
| `parser` | `HtmParser` | Yes | The parser |
| `output` | `const char*` | Yes | The output destination |
| `options` | `HtmConversionOptions` | Yes | The options to use |
| `ctx` | `HtmContext` | Yes | The context |
| `depth` | `uintptr_t` | Yes | The depth |
| `dom_ctx` | `HtmDomContext` | Yes | The dom context |

**Returns:** `bool`


---

### htm_extract_plain_text()

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

```c
const char* htm_extract_plain_text(HtmVDom dom, HtmParser parser, HtmConversionOptions options);
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `dom` | `HtmVDom` | Yes | The v dom |
| `parser` | `HtmParser` | Yes | The parser |
| `options` | `HtmConversionOptions` | Yes | The options to use |

**Returns:** `const char*`


---

### htm_handle_dfn()

Handles the `<dfn>` element.

A dfn element marks a term that is being defined. The content represents
the term, and its definition would typically appear in surrounding context.
It is rendered as emphasized (italic) text.

# Behavior

- Content is collected from children
- Non-empty content is wrapped with the configured emphasis symbol (default: `*`)
- Inline suffix handling is applied (e.g., footnote references)

**Signature:**

```c
void htm_handle_dfn(const char* tag_name, HtmNodeHandle node_handle, HtmParser parser, const char* output, HtmConversionOptions options, HtmContext ctx, uintptr_t depth, HtmDomContext dom_ctx);
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `tag_name` | `const char*` | Yes | The  tag name |
| `node_handle` | `HtmNodeHandle` | Yes | The node handle |
| `parser` | `HtmParser` | Yes | The parser |
| `output` | `const char*` | Yes | The output destination |
| `options` | `HtmConversionOptions` | Yes | The options to use |
| `ctx` | `HtmContext` | Yes | The context |
| `depth` | `uintptr_t` | Yes | The depth |
| `dom_ctx` | `HtmDomContext` | Yes | The dom context |

**Returns:** `void`


---

### htm_handle_abbr()

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

```c
void htm_handle_abbr(const char* tag_name, HtmNodeHandle node_handle, HtmParser parser, const char* output, HtmConversionOptions options, HtmContext ctx, uintptr_t depth, HtmDomContext dom_ctx);
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `tag_name` | `const char*` | Yes | The  tag name |
| `node_handle` | `HtmNodeHandle` | Yes | The node handle |
| `parser` | `HtmParser` | Yes | The parser |
| `output` | `const char*` | Yes | The output destination |
| `options` | `HtmConversionOptions` | Yes | The options to use |
| `ctx` | `HtmContext` | Yes | The context |
| `depth` | `uintptr_t` | Yes | The depth |
| `dom_ctx` | `HtmDomContext` | Yes | The dom context |

**Returns:** `void`


---

### htm_handle_time_data()

Handles the `<time>` and `<data>` elements.

Time and data elements contain machine-readable content in their attributes
and human-readable content in their text. For Markdown purposes, we output
only the human-readable text content, as Markdown doesn't have a way to
preserve machine-readable metadata.

# Behavior

- Content is extracted from children and output as-is
- Attributes (datetime, value) are not rendered in Markdown output

**Signature:**

```c
void htm_handle_time_data(const char* tag_name, HtmNodeHandle node_handle, HtmParser parser, const char* output, HtmConversionOptions options, HtmContext ctx, uintptr_t depth, HtmDomContext dom_ctx);
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `tag_name` | `const char*` | Yes | The  tag name |
| `node_handle` | `HtmNodeHandle` | Yes | The node handle |
| `parser` | `HtmParser` | Yes | The parser |
| `output` | `const char*` | Yes | The output destination |
| `options` | `HtmConversionOptions` | Yes | The options to use |
| `ctx` | `HtmContext` | Yes | The context |
| `depth` | `uintptr_t` | Yes | The depth |
| `dom_ctx` | `HtmDomContext` | Yes | The dom context |

**Returns:** `void`


---

### htm_handle_cite()

Handles the `<cite>` element.

A cite element marks the title of a cited work (book, article, website, etc.).
It is rendered as emphasized (italic) text in block mode, or as plain text in inline mode.

# Behavior

- **Block mode**: Content is wrapped with emphasis markers (default: `*`)
- **Inline mode**: Content is output as-is without formatting

**Signature:**

```c
void htm_handle_cite(const char* tag_name, HtmNodeHandle node_handle, HtmParser parser, const char* output, HtmConversionOptions options, HtmContext ctx, uintptr_t depth, HtmDomContext dom_ctx);
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `tag_name` | `const char*` | Yes | The  tag name |
| `node_handle` | `HtmNodeHandle` | Yes | The node handle |
| `parser` | `HtmParser` | Yes | The parser |
| `output` | `const char*` | Yes | The output destination |
| `options` | `HtmConversionOptions` | Yes | The options to use |
| `ctx` | `HtmContext` | Yes | The context |
| `depth` | `uintptr_t` | Yes | The depth |
| `dom_ctx` | `HtmDomContext` | Yes | The dom context |

**Returns:** `void`


---

### htm_handle_q()

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

```c
void htm_handle_q(const char* tag_name, HtmNodeHandle node_handle, HtmParser parser, const char* output, HtmConversionOptions options, HtmContext ctx, uintptr_t depth, HtmDomContext dom_ctx);
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `tag_name` | `const char*` | Yes | The  tag name |
| `node_handle` | `HtmNodeHandle` | Yes | The node handle |
| `parser` | `HtmParser` | Yes | The parser |
| `output` | `const char*` | Yes | The output destination |
| `options` | `HtmConversionOptions` | Yes | The options to use |
| `ctx` | `HtmContext` | Yes | The context |
| `depth` | `uintptr_t` | Yes | The depth |
| `dom_ctx` | `HtmDomContext` | Yes | The dom context |

**Returns:** `void`


---

### htm_handle_hgroup()

Handles the `<hgroup>` element.

An hgroup element groups related headings together (e.g., a title and subtitle).
In Markdown, we simply process all children sequentially, allowing nested
headings to maintain their individual formatting.

# Behavior

- Children are processed sequentially in the current context
- No special formatting is applied at the hgroup level

**Signature:**

```c
void htm_handle_hgroup(const char* tag_name, HtmNodeHandle node_handle, HtmParser parser, const char* output, HtmConversionOptions options, HtmContext ctx, uintptr_t depth, HtmDomContext dom_ctx);
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `tag_name` | `const char*` | Yes | The  tag name |
| `node_handle` | `HtmNodeHandle` | Yes | The node handle |
| `parser` | `HtmParser` | Yes | The parser |
| `output` | `const char*` | Yes | The output destination |
| `options` | `HtmConversionOptions` | Yes | The options to use |
| `ctx` | `HtmContext` | Yes | The context |
| `depth` | `uintptr_t` | Yes | The depth |
| `dom_ctx` | `HtmDomContext` | Yes | The dom context |

**Returns:** `void`


---

### htm_handle_dl()

Handles the `<dl>` element.

A definition list contains terms and their definitions. Terms and definitions
are output as plain blocks without Pandoc-style colon syntax, since standard
Markdown and GFM do not support definition lists.

# Behavior

- **Inline mode**: Children are processed inline without block spacing
- **Block mode**: Content is collected and wrapped with proper spacing

**Signature:**

```c
void htm_handle_dl(const char* tag_name, HtmNodeHandle node_handle, HtmParser parser, const char* output, HtmConversionOptions options, HtmContext ctx, uintptr_t depth, HtmDomContext dom_ctx);
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `tag_name` | `const char*` | Yes | The  tag name |
| `node_handle` | `HtmNodeHandle` | Yes | The node handle |
| `parser` | `HtmParser` | Yes | The parser |
| `output` | `const char*` | Yes | The output destination |
| `options` | `HtmConversionOptions` | Yes | The options to use |
| `ctx` | `HtmContext` | Yes | The context |
| `depth` | `uintptr_t` | Yes | The depth |
| `dom_ctx` | `HtmDomContext` | Yes | The dom context |

**Returns:** `void`


---

### htm_handle_dt()

Handles the `<dt>` element.

A dt element contains a term being defined. Terms are output on their own line,
with definitions following on subsequent lines.

# Behavior

- **Inline mode**: Content is output as-is
- **Block mode**: Content is followed by a newline

**Signature:**

```c
void htm_handle_dt(const char* tag_name, HtmNodeHandle node_handle, HtmParser parser, const char* output, HtmConversionOptions options, HtmContext ctx, uintptr_t depth, HtmDomContext dom_ctx);
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `tag_name` | `const char*` | Yes | The  tag name |
| `node_handle` | `HtmNodeHandle` | Yes | The node handle |
| `parser` | `HtmParser` | Yes | The parser |
| `output` | `const char*` | Yes | The output destination |
| `options` | `HtmConversionOptions` | Yes | The options to use |
| `ctx` | `HtmContext` | Yes | The context |
| `depth` | `uintptr_t` | Yes | The depth |
| `dom_ctx` | `HtmDomContext` | Yes | The dom context |

**Returns:** `void`


---

### htm_handle_dd()

Handles the `<dd>` element.

A dd element contains the definition for a term. It is output as a plain
block since standard Markdown and GFM do not support definition list syntax.

# Behavior

- **Inline mode**: Content is output as-is
- **Block mode**: Content is output as a block

**Signature:**

```c
void htm_handle_dd(const char* tag_name, HtmNodeHandle node_handle, HtmParser parser, const char* output, HtmConversionOptions options, HtmContext ctx, uintptr_t depth, HtmDomContext dom_ctx);
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `tag_name` | `const char*` | Yes | The  tag name |
| `node_handle` | `HtmNodeHandle` | Yes | The node handle |
| `parser` | `HtmParser` | Yes | The parser |
| `output` | `const char*` | Yes | The output destination |
| `options` | `HtmConversionOptions` | Yes | The options to use |
| `ctx` | `HtmContext` | Yes | The context |
| `depth` | `uintptr_t` | Yes | The depth |
| `dom_ctx` | `HtmDomContext` | Yes | The dom context |

**Returns:** `void`


---

### htm_handle_menu()

Handles the `<menu>` element.

A menu element is a semantic list, typically used for command menus or
navigation. It is rendered as an unordered list with dashes.

# Behavior

- **Inline mode**: Children are processed inline without list formatting
- **Block mode**: Content is rendered as an unordered list
- Uses `-` as the list bullet (overrides configured bullets)
- Proper blank-line spacing is maintained

**Signature:**

```c
void htm_handle_menu(const char* tag_name, HtmNodeHandle node_handle, HtmParser parser, const char* output, HtmConversionOptions options, HtmContext ctx, uintptr_t depth, HtmDomContext dom_ctx);
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `tag_name` | `const char*` | Yes | The  tag name |
| `node_handle` | `HtmNodeHandle` | Yes | The node handle |
| `parser` | `HtmParser` | Yes | The parser |
| `output` | `const char*` | Yes | The output destination |
| `options` | `HtmConversionOptions` | Yes | The options to use |
| `ctx` | `HtmContext` | Yes | The context |
| `depth` | `uintptr_t` | Yes | The depth |
| `dom_ctx` | `HtmDomContext` | Yes | The dom context |

**Returns:** `void`


---

### htm_handle_figure()

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

```c
void htm_handle_figure(const char* tag_name, HtmNodeHandle node_handle, HtmParser parser, const char* output, HtmConversionOptions options, HtmContext ctx, uintptr_t depth, HtmDomContext dom_ctx);
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `tag_name` | `const char*` | Yes | The  tag name |
| `node_handle` | `HtmNodeHandle` | Yes | The node handle |
| `parser` | `HtmParser` | Yes | The parser |
| `output` | `const char*` | Yes | The output destination |
| `options` | `HtmConversionOptions` | Yes | The options to use |
| `ctx` | `HtmContext` | Yes | The context |
| `depth` | `uintptr_t` | Yes | The depth |
| `dom_ctx` | `HtmDomContext` | Yes | The dom context |

**Returns:** `void`


---

### htm_handle_figcaption()

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

```c
void htm_handle_figcaption(const char* tag_name, HtmNodeHandle node_handle, HtmParser parser, const char* output, HtmConversionOptions options, HtmContext ctx, uintptr_t depth, HtmDomContext dom_ctx);
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `tag_name` | `const char*` | Yes | The  tag name |
| `node_handle` | `HtmNodeHandle` | Yes | The node handle |
| `parser` | `HtmParser` | Yes | The parser |
| `output` | `const char*` | Yes | The output destination |
| `options` | `HtmConversionOptions` | Yes | The options to use |
| `ctx` | `HtmContext` | Yes | The context |
| `depth` | `uintptr_t` | Yes | The depth |
| `dom_ctx` | `HtmDomContext` | Yes | The dom context |

**Returns:** `void`


---

### htm_handle_details()

Handles the `<details>` element.

A details element represents a disclosure widget that can be toggled
to show/hide additional content. In Markdown, it's rendered as a block
with all content visible.

# Behavior

- **Inline mode**: Children are processed inline without block spacing
- **Block mode**: Content is collected and wrapped with proper blank-line spacing
- **Empty content**: Skipped entirely

**Signature:**

```c
void htm_handle_details(const char* tag_name, HtmNodeHandle node_handle, HtmParser parser, const char* output, HtmConversionOptions options, HtmContext ctx, uintptr_t depth, HtmDomContext dom_ctx);
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `tag_name` | `const char*` | Yes | The  tag name |
| `node_handle` | `HtmNodeHandle` | Yes | The node handle |
| `parser` | `HtmParser` | Yes | The parser |
| `output` | `const char*` | Yes | The output destination |
| `options` | `HtmConversionOptions` | Yes | The options to use |
| `ctx` | `HtmContext` | Yes | The context |
| `depth` | `uintptr_t` | Yes | The depth |
| `dom_ctx` | `HtmDomContext` | Yes | The dom context |

**Returns:** `void`


---

### htm_handle_summary()

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

```c
void htm_handle_summary(const char* tag_name, HtmNodeHandle node_handle, HtmParser parser, const char* output, HtmConversionOptions options, HtmContext ctx, uintptr_t depth, HtmDomContext dom_ctx);
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `tag_name` | `const char*` | Yes | The  tag name |
| `node_handle` | `HtmNodeHandle` | Yes | The node handle |
| `parser` | `HtmParser` | Yes | The parser |
| `output` | `const char*` | Yes | The output destination |
| `options` | `HtmConversionOptions` | Yes | The options to use |
| `ctx` | `HtmContext` | Yes | The context |
| `depth` | `uintptr_t` | Yes | The depth |
| `dom_ctx` | `HtmDomContext` | Yes | The dom context |

**Returns:** `void`


---

### htm_handle_dialog()

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

```c
void htm_handle_dialog(const char* tag_name, HtmNodeHandle node_handle, HtmParser parser, const char* output, HtmConversionOptions options, HtmContext ctx, uintptr_t depth, HtmDomContext dom_ctx);
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `tag_name` | `const char*` | Yes | The  tag name |
| `node_handle` | `HtmNodeHandle` | Yes | The node handle |
| `parser` | `HtmParser` | Yes | The parser |
| `output` | `const char*` | Yes | The output destination |
| `options` | `HtmConversionOptions` | Yes | The options to use |
| `ctx` | `HtmContext` | Yes | The context |
| `depth` | `uintptr_t` | Yes | The depth |
| `dom_ctx` | `HtmDomContext` | Yes | The dom context |

**Returns:** `void`


---

### htm_dispatch_semantic_handler()

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

```c
bool htm_dispatch_semantic_handler(const char* tag_name, HtmNodeHandle node_handle, HtmParser parser, const char* output, HtmConversionOptions options, HtmContext ctx, uintptr_t depth, HtmDomContext dom_ctx);
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `tag_name` | `const char*` | Yes | The tag name |
| `node_handle` | `HtmNodeHandle` | Yes | The node handle |
| `parser` | `HtmParser` | Yes | The parser |
| `output` | `const char*` | Yes | The output destination |
| `options` | `HtmConversionOptions` | Yes | The options to use |
| `ctx` | `HtmContext` | Yes | The context |
| `depth` | `uintptr_t` | Yes | The depth |
| `dom_ctx` | `HtmDomContext` | Yes | The dom context |

**Returns:** `bool`


---

### htm_escape_link_label()

Escape special characters in link labels.

Markdown link labels can contain brackets, which need careful escaping to avoid
being interpreted as nested links. This function escapes unescaped closing brackets
that would break the link syntax.

**Signature:**

```c
const char* htm_escape_link_label(const char* text);
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `text` | `const char*` | Yes | The text |

**Returns:** `const char*`


---

### htm_escape_malformed_angle_brackets()

Escape malformed angle brackets in markdown output.

Markdown uses `<...>` for automatic links. Angle brackets that don't form valid
link syntax should be escaped as `&lt;` to prevent parser confusion.

A valid tag must have:
- `<!` followed by `-` or alphabetic character (for comments/declarations)
- `</` followed by alphabetic character (for closing tags)
- `<?` (for processing instructions)
- `<` followed by alphabetic character (for opening tags)

**Signature:**

```c
HtmStr* htm_escape_malformed_angle_brackets(const char* input);
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `input` | `const char*` | Yes | The input data |

**Returns:** `HtmStr`


---

### htm_trim_line_end_whitespace()

Remove trailing spaces/tabs from every line while preserving newlines.

**Signature:**

```c
void htm_trim_line_end_whitespace(const char* output);
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `output` | `const char*` | Yes | The output destination |

**Returns:** `void`


---

### htm_truncate_at_char_boundary()

Truncate a string at a valid UTF-8 boundary.

**Signature:**

```c
void htm_truncate_at_char_boundary(const char* value, uintptr_t max_len);
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `value` | `const char*` | Yes | The value |
| `max_len` | `uintptr_t` | Yes | The max len |

**Returns:** `void`


---

### htm_normalize_heading_text()

Normalize heading text by replacing newlines and extra whitespace.

Heading text should be on a single line in Markdown. This function collapses
any newlines and multiple spaces into single spaces.

**Signature:**

```c
HtmStr* htm_normalize_heading_text(const char* text);
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `text` | `const char*` | Yes | The text |

**Returns:** `HtmStr`


---

### htm_dedent_code_block()

Remove common leading whitespace from all lines in a code block.

This is useful when HTML authors indent `<pre>` content for readability,
so we can strip the shared indentation without touching meaningful spacing.

**Signature:**

```c
const char* htm_dedent_code_block(const char* content);
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `content` | `const char*` | Yes | The content to process |

**Returns:** `const char*`


---

### htm_floor_char_boundary()

Returns the largest valid char boundary index at or before `index`.

If `index` is already a char boundary it is returned unchanged.
Otherwise it walks backwards to find one.  Returns 0 if no boundary
is found before `index`.

**Signature:**

```c
uintptr_t htm_floor_char_boundary(const char* s, uintptr_t index);
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `s` | `const char*` | Yes | The s |
| `index` | `uintptr_t` | Yes | The index |

**Returns:** `uintptr_t`


---

### htm_handle_visitor_element_start()

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

```c
HtmVisitAction* htm_handle_visitor_element_start(HtmVisitorHandle visitor_handle, const char* tag_name, HtmNodeHandle node_handle, HtmHtmlTag tag, HtmParser parser, const char* output, HtmContext ctx, uintptr_t depth, HtmDomContext dom_ctx);
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `visitor_handle` | `HtmVisitorHandle` | Yes | Reference to the visitor for callbacks |
| `tag_name` | `const char*` | Yes | The normalized tag name being processed |
| `node_handle` | `HtmNodeHandle` | Yes | Handle to the DOM node |
| `tag` | `HtmHtmlTag` | Yes | Reference to the tag object |
| `parser` | `HtmParser` | Yes | Reference to the tl parser |
| `output` | `const char*` | Yes | Mutable reference to output string |
| `ctx` | `HtmContext` | Yes | The context |
| `depth` | `uintptr_t` | Yes | Current tree depth |
| `dom_ctx` | `HtmDomContext` | Yes | Reference to DOM context for tree navigation |

**Returns:** `HtmVisitAction`


---

### htm_handle_visitor_element_end()

Handles visitor callback for element end (after processing).

This function is called when exiting an element after its content has been processed.
The visitor can:
- Accept the output normally (Continue)
- Replace the output with custom content (Custom)
- Remove the output entirely (Skip)
- Signal an error (Error)

**Signature:**

```c
void htm_handle_visitor_element_end(HtmVisitorHandle visitor_handle, const char* tag_name, HtmNodeHandle node_handle, HtmHtmlTag tag, HtmParser parser, const char* output, uintptr_t element_output_start, HtmContext ctx, uintptr_t depth, HtmDomContext dom_ctx);
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `visitor_handle` | `HtmVisitorHandle` | Yes | Reference to the visitor for callbacks |
| `tag_name` | `const char*` | Yes | The normalized tag name that was processed |
| `node_handle` | `HtmNodeHandle` | Yes | Handle to the DOM node |
| `tag` | `HtmHtmlTag` | Yes | Reference to the tag object |
| `parser` | `HtmParser` | Yes | Reference to the tl parser |
| `output` | `const char*` | Yes | Mutable reference to output string |
| `element_output_start` | `uintptr_t` | Yes | Byte position where this element's output started |
| `ctx` | `HtmContext` | Yes | Reference to the conversion context |
| `depth` | `uintptr_t` | Yes | Current tree depth |
| `dom_ctx` | `HtmDomContext` | Yes | Reference to DOM context for tree navigation |

**Returns:** `void`


---

### htm_escape()

Escape Markdown special characters in text.

**Returns:**

Escaped text

**Signature:**

```c
HtmStr* htm_escape(const char* text, bool escape_misc, bool escape_asterisks, bool escape_underscores, bool escape_ascii);
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `text` | `const char*` | Yes | Text to escape |
| `escape_misc` | `bool` | Yes | Escape miscellaneous characters (`\` `&` `<` `` ` `` `[` `>` `~` `#` `=` `+` `\|` `-`) |
| `escape_asterisks` | `bool` | Yes | Escape asterisks (`*`) |
| `escape_underscores` | `bool` | Yes | Escape underscores (`_`) |
| `escape_ascii` | `bool` | Yes | Escape all ASCII punctuation (for `CommonMark` spec compliance) |

**Returns:** `HtmStr`


---

### htm_chomp()

Extract boundary whitespace from text (chomp).

Returns (prefix, suffix, `trimmed_text`) tuple.
Prefix/suffix are " " if original text had leading/trailing whitespace.
However, suffix is "" if the trailing whitespace is only newlines (not spaces/tabs).
This prevents trailing newlines from becoming trailing spaces in the output.
The trimmed text has all leading/trailing whitespace removed.

**Signature:**

```c
HtmStrStrStr* htm_chomp(const char* text);
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `text` | `const char*` | Yes | The text |

**Returns:** `HtmStrStrStr`


---

### htm_normalize_whitespace()

Normalize whitespace by collapsing consecutive spaces and tabs.

Multiple spaces and tabs are replaced with a single space.
Newlines are preserved.
Unicode spaces are normalized to ASCII spaces.

**Returns:**

Normalized text with collapsed spaces/tabs but preserved newlines

**Signature:**

```c
const char* htm_normalize_whitespace(const char* text);
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `text` | `const char*` | Yes | The text to normalize |

**Returns:** `const char*`


---

### htm_normalize_whitespace_cow()

Normalize whitespace in text, returning borrowed or owned result as needed.

This function optimizes memory by returning a borrowed reference when no normalization
is needed, and only allocating a new string when whitespace changes are necessary.

Multiple consecutive spaces, tabs, and Unicode space characters are replaced with
a single ASCII space. Newlines are preserved as-is.

**Returns:**

`Cow.Borrowed` if text is already normalized, or `Cow.Owned` with normalized text

**Signature:**

```c
HtmStr* htm_normalize_whitespace_cow(const char* text);
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `text` | `const char*` | Yes | The text to normalize |

**Returns:** `HtmStr`


---

### htm_decode_html_entities()

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

```c
const char* htm_decode_html_entities(const char* text);
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `text` | `const char*` | Yes | Text containing HTML entities |

**Returns:** `const char*`


---

### htm_decode_html_entities_cow()

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

```c
HtmStr* htm_decode_html_entities_cow(const char* text);
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `text` | `const char*` | Yes | Text potentially containing HTML entities |

**Returns:** `HtmStr`


---

### htm_underline()

Underline text with a character.

**Signature:**

```c
const char* htm_underline(const char* text, const char* pad_char);
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `text` | `const char*` | Yes | The text |
| `pad_char` | `const char*` | Yes | The pad char |

**Returns:** `const char*`


---

### htm_indent()

Indent text with a string prefix.

**Signature:**

```c
const char* htm_indent(const char* text, uintptr_t level, const char* indent_str);
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `text` | `const char*` | Yes | The text |
| `level` | `uintptr_t` | Yes | The level |
| `indent_str` | `const char*` | Yes | The indent str |

**Returns:** `const char*`


---

### htm_build_document_structure()

Build a `DocumentStructure` from an already-parsed `tl.VDom`.

Walks the DOM once, mapping HTML elements to semantic `NodeContent` variants,
tracking parent/child relationships, extracting inline `TextAnnotation`s, and
constructing heading-based `Group` nodes.

**Signature:**

```c
HtmDocumentStructure* htm_build_document_structure(HtmVDom dom);
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `dom` | `HtmVDom` | Yes | The v dom |

**Returns:** `HtmDocumentStructure`


---

### htm_build_node_context()

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

```c
HtmNodeContext* htm_build_node_context(HtmNodeType node_type, const char* tag_name, void* attributes, uintptr_t depth, uintptr_t index_in_parent, const char* parent_tag, bool is_inline);
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `node_type` | `HtmNodeType` | Yes | Coarse-grained classification (Link, Image, Heading, etc.) |
| `tag_name` | `const char*` | Yes | Raw HTML tag name (e.g., "div", "h1", "custom-element") |
| `attributes` | `void*` | Yes | All HTML attributes as key-value pairs |
| `depth` | `uintptr_t` | Yes | Nesting depth in the DOM tree (0 = root) |
| `index_in_parent` | `uintptr_t` | Yes | Zero-based index among siblings |
| `parent_tag` | `const char**` | No | Parent element's tag name (None if root) |
| `is_inline` | `bool` | Yes | Whether this element is treated as inline vs block |

**Returns:** `HtmNodeContext`


---

### htm_convert()

Convert HTML to Markdown, returning a `ConversionResult` with content, metadata, images,
and warnings.

**Errors:**

Returns an error if HTML parsing fails or if the input contains invalid UTF-8.

**Signature:**

```c
HtmConversionResult* htm_convert(const char* html, HtmConversionOptions options);
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `html` | `const char*` | Yes | The HTML string to convert |
| `options` | `HtmConversionOptions*` | No | Optional conversion options (defaults to `default options`) |

**Returns:** `HtmConversionResult`

**Errors:** Returns `NULL` on error.


---

### htm_convert_with_visitor()

Internal: convert with visitor support. Used by FFI crate.
Will be removed when convert() accepts visitor parameter directly.

**Signature:**

```c
const char* htm_convert_with_visitor(const char* html, HtmConversionOptions options, HtmVisitorHandle visitor);
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `html` | `const char*` | Yes | The html |
| `options` | `HtmConversionOptions*` | No | The options to use |
| `visitor` | `HtmVisitorHandle*` | No | The visitor handle |

**Returns:** `const char*`

**Errors:** Returns `NULL` on error.


---

### htm_conversion_options_from_json()

Parse JSON string into `ConversionOptions`.

Deserializes a JSON string into a full set of conversion options.
The JSON can be either a complete or partial options object.

**Returns:**

Fully populated `ConversionOptions` with defaults applied to any unspecified values

**Errors:**

Returns `ConversionError.ConfigError` if JSON parsing fails or contains invalid option values

**Signature:**

```c
HtmConversionOptions* htm_conversion_options_from_json(const char* json);
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `json` | `const char*` | Yes | JSON string representing conversion options |

**Returns:** `HtmConversionOptions`

**Errors:** Returns `NULL` on error.


---

### htm_conversion_options_update_from_json()

Parse JSON string into partial `ConversionOptions` update.

Deserializes a JSON string into a partial set of conversion options.
Only specified options are included; unspecified options are None.

**Returns:**

`ConversionOptionsUpdate` with only specified fields populated

**Errors:**

Returns `ConversionError.ConfigError` if JSON parsing fails or contains invalid option values

**Signature:**

```c
HtmConversionOptionsUpdate* htm_conversion_options_update_from_json(const char* json);
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `json` | `const char*` | Yes | JSON string representing partial conversion options |

**Returns:** `HtmConversionOptionsUpdate`

**Errors:** Returns `NULL` on error.


---

### htm_inline_image_config_from_json()

Parse JSON string into `InlineImageConfig` (requires `inline-images` feature).

Deserializes a JSON string into inline image extraction configuration.
The JSON can be either a complete or partial configuration object.

**Returns:**

Fully populated `InlineImageConfig` with defaults applied to any unspecified values

**Errors:**

Returns `ConversionError.ConfigError` if JSON parsing fails or contains invalid configuration values

**Signature:**

```c
HtmInlineImageConfig* htm_inline_image_config_from_json(const char* json);
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `json` | `const char*` | Yes | JSON string representing inline image configuration |

**Returns:** `HtmInlineImageConfig`

**Errors:** Returns `NULL` on error.


---

### htm_metadata_config_from_json()

Parse JSON string into `MetadataConfig` (requires `metadata` feature).

Deserializes a JSON string into metadata extraction configuration.
The JSON can be either a complete or partial configuration object.

**Returns:**

Fully populated `MetadataConfig` with defaults applied to any unspecified values

**Errors:**

Returns `ConversionError.ConfigError` if JSON parsing fails or contains invalid configuration values

**Signature:**

```c
HtmMetadataConfig* htm_metadata_config_from_json(const char* json);
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `json` | `const char*` | Yes | JSON string representing metadata extraction configuration |

**Returns:** `HtmMetadataConfig`

**Errors:** Returns `NULL` on error.


---

## Types

### HtmConversionOptions

Main conversion options for HTML to Markdown conversion.

Use `ConversionOptions.builder()` to construct, or `the default constructor` for defaults.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `heading_style` | `HtmHeadingStyle` | `HTM_HTM_ATX` | Heading style to use in Markdown output (ATX `#` or Setext underline). |
| `list_indent_type` | `HtmListIndentType` | `HTM_HTM_SPACES` | How to indent nested list items (spaces or tab). |
| `list_indent_width` | `uintptr_t` | `2` | Number of spaces (or tabs) to use for each level of list indentation. |
| `bullets` | `const char*` | `"-*+"` | Bullet character(s) to use for unordered list items (e.g. `"-"`, `"*"`). |
| `strong_em_symbol` | `const char*` | `"*"` | Character used for bold/italic emphasis markers (`*` or `_`). |
| `escape_asterisks` | `bool` | `false` | Escape `*` characters in plain text to avoid unintended bold/italic. |
| `escape_underscores` | `bool` | `false` | Escape `_` characters in plain text to avoid unintended bold/italic. |
| `escape_misc` | `bool` | `false` | Escape miscellaneous Markdown metacharacters (`[]()#` etc.) in plain text. |
| `escape_ascii` | `bool` | `false` | Escape ASCII characters that have special meaning in certain Markdown dialects. |
| `code_language` | `const char*` | `""` | Default language annotation for fenced code blocks that have no language hint. |
| `autolinks` | `bool` | `true` | Automatically convert bare URLs into Markdown autolinks. |
| `default_title` | `bool` | `false` | Emit a default title when no `<title>` tag is present. |
| `br_in_tables` | `bool` | `false` | Render `<br>` elements inside table cells as literal line breaks. |
| `highlight_style` | `HtmHighlightStyle` | `HTM_HTM_DOUBLE_EQUAL` | Style used for `<mark>` / highlighted text (e.g. `==text==`). |
| `extract_metadata` | `bool` | `true` | Extract `<meta>` and `<head>` information into the result metadata. |
| `whitespace_mode` | `HtmWhitespaceMode` | `HTM_HTM_NORMALIZED` | Controls how whitespace is normalised during conversion. |
| `strip_newlines` | `bool` | `false` | Strip all newlines from the output, producing a single-line result. |
| `wrap` | `bool` | `false` | Wrap long lines at `wrap_width` characters. |
| `wrap_width` | `uintptr_t` | `80` | Maximum line width when `wrap` is enabled (default `80`). |
| `convert_as_inline` | `bool` | `false` | Treat the entire document as inline content (no block-level wrappers). |
| `sub_symbol` | `const char*` | `""` | Markdown notation for subscript text (e.g. `"~"`). |
| `sup_symbol` | `const char*` | `""` | Markdown notation for superscript text (e.g. `"^"`). |
| `newline_style` | `HtmNewlineStyle` | `HTM_HTM_SPACES` | How to encode hard line breaks (`<br>`) in Markdown. |
| `code_block_style` | `HtmCodeBlockStyle` | `HTM_HTM_BACKTICKS` | Style used for fenced code blocks (backticks or tilde). |
| `keep_inline_images_in` | `const char**` | `NULL` | HTML tag names whose `<img>` children are kept inline instead of block. |
| `preprocessing` | `HtmPreprocessingOptions` | — | Pre-processing options applied to the HTML before conversion. |
| `encoding` | `const char*` | `"utf-8"` | Expected character encoding of the input HTML (default `"utf-8"`). |
| `debug` | `bool` | `false` | Emit debug information during conversion. |
| `strip_tags` | `const char**` | `NULL` | HTML tag names whose content is stripped from the output entirely. |
| `preserve_tags` | `const char**` | `NULL` | HTML tag names that are preserved verbatim in the output. |
| `skip_images` | `bool` | `false` | Skip conversion of `<img>` elements (omit images from output). |
| `link_style` | `HtmLinkStyle` | `HTM_HTM_INLINE` | Link rendering style (inline or reference). |
| `output_format` | `HtmOutputFormat` | `HTM_HTM_MARKDOWN` | Target output format (Markdown, plain text, etc.). |
| `include_document_structure` | `bool` | `false` | Include structured document tree in result. |
| `extract_images` | `bool` | `false` | Extract inline images from data URIs and SVGs. |
| `max_image_size` | `uint64_t` | `5242880` | Maximum decoded image size in bytes (default 5MB). |
| `capture_svg` | `bool` | `false` | Capture SVG elements as images. |
| `infer_dimensions` | `bool` | `true` | Infer image dimensions from data. |

#### Methods

##### htm_default()

**Signature:**

```c
HtmConversionOptions htm_default();
```

##### htm_builder()

Create a new builder with default values.

**Signature:**

```c
HtmConversionOptionsBuilder htm_builder();
```

##### htm_apply_update()

Apply a partial update to these conversion options.

**Signature:**

```c
void htm_apply_update(HtmConversionOptionsUpdate update);
```

##### htm_from_update()

Create from a partial update, applying to defaults.

**Signature:**

```c
HtmConversionOptions htm_from_update(HtmConversionOptionsUpdate update);
```

##### htm_from()

**Signature:**

```c
HtmConversionOptions htm_from(HtmConversionOptionsUpdate update);
```


---

### HtmConversionResult

The primary result of HTML conversion and extraction.

Contains the converted text output, optional structured document tree,
metadata, extracted tables, images, and processing warnings.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `content` | `const char**` | `NULL` | Converted text output (markdown, djot, or plain text). `None` when `output_format` is set to `OutputFormat.None`, indicating extraction-only mode. |
| `document` | `HtmDocumentStructure*` | `NULL` | Structured document tree with semantic elements. Populated when `include_document_structure` is `True` in options. |
| `metadata` | `HtmHtmlMetadata` | — | Extracted HTML metadata (title, OG, links, images, structured data). |
| `tables` | `HtmTableData*` | `NULL` | Extracted tables with structured cell data and markdown representation. |
| `images` | `HtmInlineImage*` | `NULL` | Extracted inline images (data URIs and SVGs). Populated when `extract_images` is `True` in options. |
| `warnings` | `HtmProcessingWarning*` | `NULL` | Non-fatal processing warnings. |


---

### HtmContext

Conversion context that tracks state during HTML to Markdown conversion.

This context is passed through the recursive tree walker and maintains information
about the current position in the document tree, nesting levels, and enabled features.


---

### HtmConversionOptionsBuilder

Builder for `ConversionOptions`.

All fields start with default values. Call `.build()` to produce the final options.

#### Methods

##### htm_strip_tags()

Set the list of HTML tag names whose content is stripped from output.

**Signature:**

```c
HtmConversionOptionsBuilder htm_strip_tags(const char** tags);
```

##### htm_preserve_tags()

Set the list of HTML tag names that are preserved verbatim in output.

**Signature:**

```c
HtmConversionOptionsBuilder htm_preserve_tags(const char** tags);
```

##### htm_keep_inline_images_in()

Set the list of HTML tag names whose `<img>` children are kept inline.

**Signature:**

```c
HtmConversionOptionsBuilder htm_keep_inline_images_in(const char** tags);
```

##### htm_preprocessing()

Set the pre-processing options applied to the HTML before conversion.

**Signature:**

```c
HtmConversionOptionsBuilder htm_preprocessing(HtmPreprocessingOptions preprocessing);
```

##### htm_build()

Build the final `ConversionOptions`.

**Signature:**

```c
HtmConversionOptions htm_build();
```


---

### HtmDjotRenderer

Renderer for Djot lightweight markup output.

#### Methods

##### htm_emphasis()

**Signature:**

```c
const char* htm_emphasis(const char* content);
```

##### htm_strong()

**Signature:**

```c
const char* htm_strong(const char* content, const char* symbol);
```

##### htm_strikethrough()

**Signature:**

```c
const char* htm_strikethrough(const char* content);
```

##### htm_highlight()

**Signature:**

```c
const char* htm_highlight(const char* content);
```

##### htm_inserted()

**Signature:**

```c
const char* htm_inserted(const char* content);
```

##### htm_subscript()

**Signature:**

```c
const char* htm_subscript(const char* content, const char* custom_symbol);
```

##### htm_superscript()

**Signature:**

```c
const char* htm_superscript(const char* content, const char* custom_symbol);
```

##### htm_span_with_attributes()

**Signature:**

```c
const char* htm_span_with_attributes(const char* content, const char** classes, const char* id);
```

##### htm_div_with_attributes()

**Signature:**

```c
const char* htm_div_with_attributes(const char* content, const char** classes);
```

##### htm_is_djot()

**Signature:**

```c
bool htm_is_djot();
```


---

### HtmDocumentMetadata

Document-level metadata extracted from `<head>` and top-level elements.

Contains all metadata typically used by search engines, social media platforms,
and browsers for document indexing and presentation.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `title` | `const char**` | `NULL` | Document title from `<title>` tag |
| `description` | `const char**` | `NULL` | Document description from `<meta name="description">` tag |
| `keywords` | `const char**` | `NULL` | Document keywords from `<meta name="keywords">` tag, split on commas |
| `author` | `const char**` | `NULL` | Document author from `<meta name="author">` tag |
| `canonical_url` | `const char**` | `NULL` | Canonical URL from `<link rel="canonical">` tag |
| `base_href` | `const char**` | `NULL` | Base URL from `<base href="">` tag for resolving relative URLs |
| `language` | `const char**` | `NULL` | Document language from `lang` attribute |
| `text_direction` | `HtmTextDirection*` | `NULL` | Document text direction from `dir` attribute |
| `open_graph` | `void*` | `NULL` | Open Graph metadata (og:* properties) for social media Keys like "title", "description", "image", "url", etc. |
| `twitter_card` | `void*` | `NULL` | Twitter Card metadata (twitter:* properties) Keys like "card", "site", "creator", "title", "description", "image", etc. |
| `meta_tags` | `void*` | `NULL` | Additional meta tags not covered by specific fields Keys are meta name/property attributes, values are content |


---

### HtmDocumentNode

A single node in the document tree.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `id` | `const char*` | — | Deterministic node identifier. |
| `content` | `HtmNodeContent` | — | The semantic content of this node. |
| `parent` | `uint32_t*` | `NULL` | Index of the parent node (None for root nodes). |
| `children` | `uint32_t*` | — | Indices of child nodes in reading order. |
| `annotations` | `HtmTextAnnotation*` | — | Inline formatting annotations (bold, italic, links, etc.) with byte offsets into the text. |
| `attributes` | `void**` | `NULL` | Format-specific attributes (e.g. class, id, data-* attributes). |


---

### HtmDocumentStructure

A structured document tree representing the semantic content of an HTML document.

Uses a flat node array with index-based parent/child references for efficient traversal.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `nodes` | `HtmDocumentNode*` | — | All nodes in document reading order. |
| `source_format` | `const char**` | `NULL` | The source format (always "html" for this library). |


---

### HtmDomContext

DOM context that provides efficient access to parent/child relationships and text content.

This context is built once during conversion and provides O(1) access to node relationships
via precomputed maps. It also includes an LRU cache for text content extraction.


---

### HtmFormatRenderer

Trait for format-specific rendering of inline elements.

Implementations provide the syntax for emphasis, strong, strikethrough, etc.
in their respective output formats.

#### Methods

##### htm_emphasis()

Render emphasis (em, i elements)

**Signature:**

```c
const char* htm_emphasis(const char* content);
```

##### htm_strong()

Render strong emphasis (strong, b elements)

**Signature:**

```c
const char* htm_strong(const char* content, const char* symbol);
```

##### htm_strikethrough()

Render strikethrough (del, s elements)

**Signature:**

```c
const char* htm_strikethrough(const char* content);
```

##### htm_highlight()

Render highlight (mark element)

**Signature:**

```c
const char* htm_highlight(const char* content);
```

##### htm_inserted()

Render inserted text (ins element)

**Signature:**

```c
const char* htm_inserted(const char* content);
```

##### htm_subscript()

Render subscript (sub element)

**Signature:**

```c
const char* htm_subscript(const char* content, const char* custom_symbol);
```

##### htm_superscript()

Render superscript (sup element)

**Signature:**

```c
const char* htm_superscript(const char* content, const char* custom_symbol);
```

##### htm_span_with_attributes()

Render span with attributes (for Djot: [text]{.class})

**Signature:**

```c
const char* htm_span_with_attributes(const char* content, const char** classes, const char* id);
```

##### htm_div_with_attributes()

Render div with attributes (for Djot: .: class)

**Signature:**

```c
const char* htm_div_with_attributes(const char* content, const char** classes);
```

##### htm_is_djot()

Check if this is Djot format

**Signature:**

```c
bool htm_is_djot();
```


---

### HtmGridCell

A single cell in a table grid.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `content` | `const char*` | — | The text content of the cell. |
| `row` | `uint32_t` | — | 0-indexed row position. |
| `col` | `uint32_t` | — | 0-indexed column position. |
| `row_span` | `uint32_t` | — | Number of rows this cell spans (default 1). |
| `col_span` | `uint32_t` | — | Number of columns this cell spans (default 1). |
| `is_header` | `bool` | — | Whether this is a header cell (`<th>`). |


---

### HtmHeaderMetadata

Header element metadata with hierarchy tracking.

Captures heading elements (h1-h6) with their text content, identifiers,
and position in the document structure.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `level` | `uint8_t` | — | Header level: 1 (h1) through 6 (h6) |
| `text` | `const char*` | — | Normalized text content of the header |
| `id` | `const char**` | `NULL` | HTML id attribute if present |
| `depth` | `uintptr_t` | — | Document tree depth at the header element |
| `html_offset` | `uintptr_t` | — | Byte offset in original HTML document |

#### Methods

##### htm_is_valid()

Validate that the header level is within valid range (1-6).

**Returns:**

`true` if level is 1-6, `false` otherwise.

**Signature:**

```c
bool htm_is_valid();
```


---

### HtmHtmlMetadata

Comprehensive metadata extraction result from HTML document.

Contains all extracted metadata types in a single structure,
suitable for serialization and transmission across language boundaries.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `document` | `HtmDocumentMetadata` | — | Document-level metadata (title, description, canonical, etc.) |
| `headers` | `HtmHeaderMetadata*` | `NULL` | Extracted header elements with hierarchy |
| `links` | `HtmLinkMetadata*` | `NULL` | Extracted hyperlinks with type classification |
| `images` | `HtmImageMetadata*` | `NULL` | Extracted images with source and dimensions |
| `structured_data` | `HtmStructuredData*` | `NULL` | Extracted structured data blocks |


---

### HtmHtmlVisitor

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

##### htm_visit_element_start()

Called before entering any element.

This is the first callback invoked for every HTML element, allowing
visitors to implement generic element handling before tag-specific logic.

**Signature:**

```c
HtmVisitResult htm_visit_element_start(HtmNodeContext ctx);
```

##### htm_visit_element_end()

Called after exiting any element.

Receives the default markdown output that would be generated.
Visitors can inspect or replace this output.

**Signature:**

```c
HtmVisitResult htm_visit_element_end(HtmNodeContext ctx, const char* output);
```

##### htm_visit_text()

Visit text nodes (most frequent callback - ~100+ per document).

**Signature:**

```c
HtmVisitResult htm_visit_text(HtmNodeContext ctx, const char* text);
```

##### htm_visit_link()

Visit anchor links `<a href="...">`.

**Signature:**

```c
HtmVisitResult htm_visit_link(HtmNodeContext ctx, const char* href, const char* text, const char* title);
```

##### htm_visit_image()

Visit images `<img src="...">`.

**Signature:**

```c
HtmVisitResult htm_visit_image(HtmNodeContext ctx, const char* src, const char* alt, const char* title);
```

##### htm_visit_heading()

Visit heading elements `<h1>` through `<h6>`.

**Signature:**

```c
HtmVisitResult htm_visit_heading(HtmNodeContext ctx, uint32_t level, const char* text, const char* id);
```

##### htm_visit_code_block()

Visit code blocks `<pre><code>`.

**Signature:**

```c
HtmVisitResult htm_visit_code_block(HtmNodeContext ctx, const char* lang, const char* code);
```

##### htm_visit_code_inline()

Visit inline code `<code>`.

**Signature:**

```c
HtmVisitResult htm_visit_code_inline(HtmNodeContext ctx, const char* code);
```

##### htm_visit_list_item()

Visit list items `<li>`.

**Signature:**

```c
HtmVisitResult htm_visit_list_item(HtmNodeContext ctx, bool ordered, const char* marker, const char* text);
```

##### htm_visit_list_start()

Called before processing a list `<ul>` or `<ol>`.

**Signature:**

```c
HtmVisitResult htm_visit_list_start(HtmNodeContext ctx, bool ordered);
```

##### htm_visit_list_end()

Called after processing a list `</ul>` or `</ol>`.

**Signature:**

```c
HtmVisitResult htm_visit_list_end(HtmNodeContext ctx, bool ordered, const char* output);
```

##### htm_visit_table_start()

Called before processing a table `<table>`.

**Signature:**

```c
HtmVisitResult htm_visit_table_start(HtmNodeContext ctx);
```

##### htm_visit_table_row()

Visit table rows `<tr>`.

**Signature:**

```c
HtmVisitResult htm_visit_table_row(HtmNodeContext ctx, const char** cells, bool is_header);
```

##### htm_visit_table_end()

Called after processing a table `</table>`.

**Signature:**

```c
HtmVisitResult htm_visit_table_end(HtmNodeContext ctx, const char* output);
```

##### htm_visit_blockquote()

Visit blockquote elements `<blockquote>`.

**Signature:**

```c
HtmVisitResult htm_visit_blockquote(HtmNodeContext ctx, const char* content, uintptr_t depth);
```

##### htm_visit_strong()

Visit strong/bold elements `<strong>`, `<b>`.

**Signature:**

```c
HtmVisitResult htm_visit_strong(HtmNodeContext ctx, const char* text);
```

##### htm_visit_emphasis()

Visit emphasis/italic elements `<em>`, `<i>`.

**Signature:**

```c
HtmVisitResult htm_visit_emphasis(HtmNodeContext ctx, const char* text);
```

##### htm_visit_strikethrough()

Visit strikethrough elements `<s>`, `<del>`, `<strike>`.

**Signature:**

```c
HtmVisitResult htm_visit_strikethrough(HtmNodeContext ctx, const char* text);
```

##### htm_visit_underline()

Visit underline elements `<u>`, `<ins>`.

**Signature:**

```c
HtmVisitResult htm_visit_underline(HtmNodeContext ctx, const char* text);
```

##### htm_visit_subscript()

Visit subscript elements `<sub>`.

**Signature:**

```c
HtmVisitResult htm_visit_subscript(HtmNodeContext ctx, const char* text);
```

##### htm_visit_superscript()

Visit superscript elements `<sup>`.

**Signature:**

```c
HtmVisitResult htm_visit_superscript(HtmNodeContext ctx, const char* text);
```

##### htm_visit_mark()

Visit mark/highlight elements `<mark>`.

**Signature:**

```c
HtmVisitResult htm_visit_mark(HtmNodeContext ctx, const char* text);
```

##### htm_visit_line_break()

Visit line break elements `<br>`.

**Signature:**

```c
HtmVisitResult htm_visit_line_break(HtmNodeContext ctx);
```

##### htm_visit_horizontal_rule()

Visit horizontal rule elements `<hr>`.

**Signature:**

```c
HtmVisitResult htm_visit_horizontal_rule(HtmNodeContext ctx);
```

##### htm_visit_custom_element()

Visit custom elements (web components) or unknown tags.

**Signature:**

```c
HtmVisitResult htm_visit_custom_element(HtmNodeContext ctx, const char* tag_name, const char* html);
```

##### htm_visit_definition_list_start()

Visit definition list `<dl>`.

**Signature:**

```c
HtmVisitResult htm_visit_definition_list_start(HtmNodeContext ctx);
```

##### htm_visit_definition_term()

Visit definition term `<dt>`.

**Signature:**

```c
HtmVisitResult htm_visit_definition_term(HtmNodeContext ctx, const char* text);
```

##### htm_visit_definition_description()

Visit definition description `<dd>`.

**Signature:**

```c
HtmVisitResult htm_visit_definition_description(HtmNodeContext ctx, const char* text);
```

##### htm_visit_definition_list_end()

Called after processing a definition list `</dl>`.

**Signature:**

```c
HtmVisitResult htm_visit_definition_list_end(HtmNodeContext ctx, const char* output);
```

##### htm_visit_form()

Visit form elements `<form>`.

**Signature:**

```c
HtmVisitResult htm_visit_form(HtmNodeContext ctx, const char* action, const char* method);
```

##### htm_visit_input()

Visit input elements `<input>`.

**Signature:**

```c
HtmVisitResult htm_visit_input(HtmNodeContext ctx, const char* input_type, const char* name, const char* value);
```

##### htm_visit_button()

Visit button elements `<button>`.

**Signature:**

```c
HtmVisitResult htm_visit_button(HtmNodeContext ctx, const char* text);
```

##### htm_visit_audio()

Visit audio elements `<audio>`.

**Signature:**

```c
HtmVisitResult htm_visit_audio(HtmNodeContext ctx, const char* src);
```

##### htm_visit_video()

Visit video elements `<video>`.

**Signature:**

```c
HtmVisitResult htm_visit_video(HtmNodeContext ctx, const char* src);
```

##### htm_visit_iframe()

Visit iframe elements `<iframe>`.

**Signature:**

```c
HtmVisitResult htm_visit_iframe(HtmNodeContext ctx, const char* src);
```

##### htm_visit_details()

Visit details elements `<details>`.

**Signature:**

```c
HtmVisitResult htm_visit_details(HtmNodeContext ctx, bool open);
```

##### htm_visit_summary()

Visit summary elements `<summary>`.

**Signature:**

```c
HtmVisitResult htm_visit_summary(HtmNodeContext ctx, const char* text);
```

##### htm_visit_figure_start()

Visit figure elements `<figure>`.

**Signature:**

```c
HtmVisitResult htm_visit_figure_start(HtmNodeContext ctx);
```

##### htm_visit_figcaption()

Visit figcaption elements `<figcaption>`.

**Signature:**

```c
HtmVisitResult htm_visit_figcaption(HtmNodeContext ctx, const char* text);
```

##### htm_visit_figure_end()

Called after processing a figure `</figure>`.

**Signature:**

```c
HtmVisitResult htm_visit_figure_end(HtmNodeContext ctx, const char* output);
```


---

### HtmImageMetadata

Image metadata with source and dimensions.

Captures `<img>` elements and inline `<svg>` elements with metadata
for image analysis and optimization.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `src` | `const char*` | — | Image source (URL, data URI, or SVG content identifier) |
| `alt` | `const char**` | `NULL` | Alternative text from alt attribute (for accessibility) |
| `title` | `const char**` | `NULL` | Title attribute (often shown as tooltip) |
| `dimensions` | `HtmU32U32*` | `NULL` | Image dimensions as (width, height) if available |
| `image_type` | `HtmImageType` | — | Image type classification |
| `attributes` | `void*` | — | Additional HTML attributes |


---

### HtmImageMetadataPayload

Payload type for image metadata extraction.


---

### HtmInlineCollectorHandle

Handle type for inline image collector when feature is enabled.


---

### HtmInlineImageConfig

Inline image configuration that specifies contexts where images remain as markdown links.

This is a wrapper type that provides semantic clarity for the vector of element
names where inline images should be preserved.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `keep_inline_images_in` | `const char**` | `NULL` | HTML elements where images should remain as markdown links (not converted to alt text) |

#### Methods

##### htm_from_elements()

Create a new inline image configuration from a list of element names.

**Signature:**

```c
HtmInlineImageConfig htm_from_elements(const char** elements);
```

##### htm_add_element()

Add an element name to the list of elements where images are kept inline.

**Signature:**

```c
void htm_add_element(const char* element);
```

##### htm_should_keep_images()

Check if a given element should keep images inline.

**Returns:**

`true` if the element is in the configured list, `false` otherwise

**Signature:**

```c
bool htm_should_keep_images(const char* element);
```

##### htm_default()

**Signature:**

```c
HtmInlineImageConfig htm_default();
```


---

### HtmLinkMetadata

Hyperlink metadata with categorization and attributes.

Represents `<a>` elements with parsed href values, text content, and link type classification.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `href` | `const char*` | — | The href URL value |
| `text` | `const char*` | — | Link text content (normalized, concatenated if mixed with elements) |
| `title` | `const char**` | `NULL` | Optional title attribute (often shown as tooltip) |
| `link_type` | `HtmLinkType` | — | Link type classification |
| `rel` | `const char**` | — | Rel attribute values (e.g., "nofollow", "stylesheet", "canonical") |
| `attributes` | `void*` | — | Additional HTML attributes |

#### Methods

##### htm_classify_link()

Classify a link based on href value.

**Returns:**

Appropriate `LinkType` based on protocol and content.

**Signature:**

```c
HtmLinkType htm_classify_link(const char* href);
```


---

### HtmMarkdownRenderer

Renderer for standard Markdown output.

#### Methods

##### htm_emphasis()

**Signature:**

```c
const char* htm_emphasis(const char* content);
```

##### htm_strong()

**Signature:**

```c
const char* htm_strong(const char* content, const char* symbol);
```

##### htm_strikethrough()

**Signature:**

```c
const char* htm_strikethrough(const char* content);
```

##### htm_highlight()

**Signature:**

```c
const char* htm_highlight(const char* content);
```

##### htm_inserted()

**Signature:**

```c
const char* htm_inserted(const char* content);
```

##### htm_subscript()

**Signature:**

```c
const char* htm_subscript(const char* content, const char* custom_symbol);
```

##### htm_superscript()

**Signature:**

```c
const char* htm_superscript(const char* content, const char* custom_symbol);
```

##### htm_span_with_attributes()

**Signature:**

```c
const char* htm_span_with_attributes(const char* content, const char** classes, const char* id);
```

##### htm_div_with_attributes()

**Signature:**

```c
const char* htm_div_with_attributes(const char* content, const char** classes);
```

##### htm_is_djot()

**Signature:**

```c
bool htm_is_djot();
```


---

### HtmMetadataCollector

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

### HtmMetadataConfig

Configuration for metadata extraction granularity.

Controls which metadata types are extracted and size limits for safety.
Enables selective extraction of different metadata categories from HTML documents,
allowing fine-grained control over which types of information to collect during
the HTML-to-Markdown conversion process.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `extract_document` | `bool` | `true` | Extract document-level metadata (title, description, author, etc.). When enabled, collects metadata from `<head>` section including: - `<title>` element content - `<meta name="description">` and other standard meta tags - Open Graph (og:*) properties for social media optimization - Twitter Card (twitter:*) properties - Language and text direction attributes - Canonical URL and base href references |
| `extract_headers` | `bool` | `true` | Extract h1-h6 header elements and their hierarchy. When enabled, collects all heading elements with: - Header level (1-6) - Text content (normalized) - HTML id attribute if present - Document tree depth for hierarchy tracking - Byte offset in original HTML for positioning |
| `extract_links` | `bool` | `true` | Extract anchor (a) elements as links with type classification. When enabled, collects all hyperlinks with: - href attribute value - Link text content - Title attribute (tooltip text) - Automatic link type classification (anchor, internal, external, email, phone, other) - Rel attribute values - Additional custom attributes |
| `extract_images` | `bool` | `true` | Extract image elements and data URIs. When enabled, collects all image elements with: - Source URL or data URI - Alt text for accessibility - Title attribute - Dimensions (width, height) if available - Automatic image type classification (data URI, external, relative, inline SVG) - Additional custom attributes |
| `extract_structured_data` | `bool` | `true` | Extract structured data (JSON-LD, Microdata, RDFa). When enabled, collects machine-readable structured data including: - JSON-LD script blocks with schema detection - Microdata attributes (itemscope, itemtype, itemprop) - RDFa markup - Extracted schema type if detectable |
| `max_structured_data_size` | `uintptr_t` | — | Maximum total size of structured data to collect (bytes). Prevents memory exhaustion attacks on malformed or adversarial documents containing excessively large structured data blocks. When the accumulated size of structured data exceeds this limit, further collection stops. Default: `1_000_000` bytes (1 MB) |

#### Methods

##### htm_default()

Create default metadata configuration.

Defaults to extracting all metadata types with 1MB limit on structured data.

**Signature:**

```c
HtmMetadataConfig htm_default();
```

##### htm_any_enabled()

Check if any metadata extraction is enabled.

Returns `true` if at least one extraction category is enabled, `false` if all are disabled.
This is useful for early exit optimization when the application doesn't need metadata.

**Returns:**

`true` if any of the extraction flags are enabled, `false` if all are disabled.

**Signature:**

```c
bool htm_any_enabled();
```

##### htm_apply_update()

Apply a partial update to this metadata configuration.

Any specified fields in the update (Some values) will override the current values.
Unspecified fields (None) are left unchanged. This allows selective modification
of configuration without affecting unrelated settings.

**Signature:**

```c
void htm_apply_update(HtmMetadataConfigUpdate update);
```

##### htm_from_update()

Create new metadata configuration from a partial update.

Creates a new `MetadataConfig` struct with defaults, then applies the update.
Fields not specified in the update (None) keep their default values.
This is a convenience method for constructing a configuration from a partial specification
without needing to explicitly call `.default()` first.

**Returns:**

New `MetadataConfig` with specified updates applied to defaults

**Signature:**

```c
HtmMetadataConfig htm_from_update(HtmMetadataConfigUpdate update);
```

##### htm_from()

**Signature:**

```c
HtmMetadataConfig htm_from(HtmMetadataConfigUpdate update);
```


---

### HtmNodeContext

Context information passed to all visitor methods.

Provides comprehensive metadata about the current node being visited,
including its type, attributes, position in the DOM tree, and parent context.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `node_type` | `HtmNodeType` | — | Coarse-grained node type classification |
| `tag_name` | `const char*` | — | Raw HTML tag name (e.g., "div", "h1", "custom-element") |
| `attributes` | `void*` | — | All HTML attributes as key-value pairs |
| `depth` | `uintptr_t` | — | Depth in the DOM tree (0 = root) |
| `index_in_parent` | `uintptr_t` | — | Index among siblings (0-based) |
| `parent_tag` | `const char**` | `NULL` | Parent element's tag name (None if root) |
| `is_inline` | `bool` | — | Whether this element is treated as inline vs block |


---

### HtmPreprocessingOptions

HTML preprocessing options for document cleanup before conversion.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `enabled` | `bool` | `true` | Enable HTML preprocessing globally |
| `preset` | `HtmPreprocessingPreset` | `HTM_HTM_STANDARD` | Preprocessing preset level (Minimal, Standard, Aggressive) |
| `remove_navigation` | `bool` | `true` | Remove navigation elements (nav, breadcrumbs, menus, sidebars) |
| `remove_forms` | `bool` | `true` | Remove form elements (forms, inputs, buttons, etc.) |

#### Methods

##### htm_default()

**Signature:**

```c
HtmPreprocessingOptions htm_default();
```

##### htm_apply_update()

Apply a partial update to these preprocessing options.

Any specified fields in the update will override the current values.
Unspecified fields (None) are left unchanged.

**Signature:**

```c
void htm_apply_update(HtmPreprocessingOptionsUpdate update);
```

##### htm_from_update()

Create new preprocessing options from a partial update.

Creates a new `PreprocessingOptions` struct with defaults, then applies the update.
Fields not specified in the update keep their default values.

**Returns:**

New `PreprocessingOptions` with specified updates applied to defaults

**Signature:**

```c
HtmPreprocessingOptions htm_from_update(HtmPreprocessingOptionsUpdate update);
```

##### htm_from()

**Signature:**

```c
HtmPreprocessingOptions htm_from(HtmPreprocessingOptionsUpdate update);
```


---

### HtmProcessingWarning

A non-fatal warning generated during HTML processing.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `message` | `const char*` | — | Human-readable warning message. |
| `kind` | `HtmWarningKind` | — | The category of warning. |


---

### HtmReferenceCollector

Collects link/image references during conversion and produces a reference
definitions section at the end of the document.

#### Methods

##### htm_get_or_insert()

Register a URL (and optional title) and return its 1-based reference number.

If the same URL+title pair was already registered, the existing number is returned.

**Signature:**

```c
uintptr_t htm_get_or_insert(const char* url, const char* title);
```

##### htm_finish()

Produce the reference definitions section.

Returns an empty string when no references were collected.

**Signature:**

```c
const char* htm_finish();
```


---

### HtmReferenceCollectorHandle

Shared handle for passing the collector through the conversion context.


---

### HtmStructureCollector

Incremental builder for `DocumentStructure` during a single DOM walk.

#### Methods

##### htm_push_heading()

Record a heading element.

Creates a `NodeContent.Group` (which owns all subsequent sibling content until a
heading of equal or higher rank closes it) followed by a `NodeContent.Heading` child.

Returns the index of the **heading** node (the group node is one before it).

**Signature:**

```c
uint32_t htm_push_heading(uint8_t level, const char* text, const char* id);
```

##### htm_push_paragraph()

Record a paragraph element.

Returns the node index.

**Signature:**

```c
uint32_t htm_push_paragraph(const char* text);
```

##### htm_push_list_start()

Open a list container.

Returns the node index; call `push_list_end` to close it.

**Signature:**

```c
uint32_t htm_push_list_start(bool ordered);
```

##### htm_push_list_end()

Close the innermost open list container.

**Signature:**

```c
void htm_push_list_end();
```

##### htm_push_list_item()

Record a list item under the current open list.

If there is no open list, the item is parented under the current section/container.
Returns the node index.

**Signature:**

```c
uint32_t htm_push_list_item(const char* text);
```

##### htm_push_table()

Record a table.

Returns the node index.

**Signature:**

```c
uint32_t htm_push_table(HtmTableGrid grid);
```

##### htm_push_image()

Record an image element.

Returns the node index.

**Signature:**

```c
uint32_t htm_push_image(const char* src, const char* alt);
```

##### htm_push_code()

Record a code block.

Returns the node index.

**Signature:**

```c
uint32_t htm_push_code(const char* text, const char* language);
```

##### htm_push_quote_start()

Open a blockquote container.

Returns the node index; call `push_quote_end` to close it.

**Signature:**

```c
uint32_t htm_push_quote_start();
```

##### htm_push_quote_end()

Close the innermost open blockquote container.

**Signature:**

```c
void htm_push_quote_end();
```

##### htm_push_raw_block()

Record a raw block (e.g. preserved `<script>` or `<style>` content).

Returns the node index.

**Signature:**

```c
uint32_t htm_push_raw_block(const char* format, const char* content);
```

##### htm_finish()

Consume the collector and return the completed `DocumentStructure`.

**Signature:**

```c
HtmDocumentStructure htm_finish();
```

##### htm_default()

**Signature:**

```c
HtmStructureCollector htm_default();
```


---

### HtmStructureCollectorHandle

Shared mutable handle used in `crate.converter.Context`.


---

### HtmStructuredData

Structured data block (JSON-LD, Microdata, or RDFa).

Represents machine-readable structured data found in the document.
JSON-LD blocks are collected as raw JSON strings for flexibility.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `data_type` | `HtmStructuredDataType` | — | Type of structured data (JSON-LD, Microdata, RDFa) |
| `raw_json` | `const char*` | — | Raw JSON string (for JSON-LD) or serialized representation |
| `schema_type` | `const char**` | `NULL` | Schema type if detectable (e.g., "Article", "Event", "Product") |


---

### HtmTableData

A top-level extracted table with both structured data and markdown representation.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `grid` | `HtmTableGrid` | — | The structured table grid. |
| `markdown` | `const char*` | — | The markdown rendering of this table. |


---

### HtmTableGrid

A structured table grid with cell-level data including spans.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `rows` | `uint32_t` | — | Number of rows. |
| `cols` | `uint32_t` | — | Number of columns. |
| `cells` | `HtmGridCell*` | `NULL` | All cells in the table (may be fewer than rows*cols due to spans). |


---

### HtmTableScan

Scan results for a table element.

Contains metadata about table structure to determine optimal rendering:
- Row counts for consistency checking
- Presence of headers, captions, and nested tables
- Presence of colspan/rowspan (spanning cells)
- Link and text content counts

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `row_counts` | `uintptr_t*` | `NULL` | Number of cells in each row |
| `has_span` | `bool` | — | Whether any cells have colspan or rowspan attributes |
| `has_header` | `bool` | — | Whether the table has header cells (th elements or role="head") |
| `has_caption` | `bool` | — | Whether the table has a caption element |
| `nested_table_count` | `uintptr_t` | — | Number of nested tables found inside this table |
| `link_count` | `uintptr_t` | — | Count of anchor elements in the table |
| `has_text` | `bool` | — | Whether the table contains text content (not empty) |


---

### HtmTextAnnotation

An inline text annotation with byte-range offsets.

Annotations describe formatting (bold, italic, etc.) and links within a node's text content.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `start` | `uint32_t` | — | Start byte offset (inclusive) into the parent node's text. |
| `end` | `uint32_t` | — | End byte offset (exclusive) into the parent node's text. |
| `kind` | `HtmAnnotationKind` | — | The type of annotation. |


---

### HtmVisitorHandle

Type alias for a visitor handle (Rc-wrapped `RefCell` for interior mutability).

This allows visitors to be passed around and shared while still being mutable.


---

## Enums

### HtmVisitAction

Result of visitor element start callback indicating what should happen next.

| Value | Description |
|-------|-------------|
| `HTM_CONTINUE` | Continue with normal element processing |
| `HTM_SKIP` | Skip the element entirely (don't process children or call visit_element_end) |
| `HTM_CUSTOM` | Custom output was provided, skip normal processing |
| `HTM_ERROR` | Error occurred during visitor callback |


---

### HtmTextDirection

Text directionality of document content.

Corresponds to the HTML `dir` attribute and `bdi` element directionality.

| Value | Description |
|-------|-------------|
| `HTM_LEFT_TO_RIGHT` | Left-to-right text flow (default for Latin scripts) |
| `HTM_RIGHT_TO_LEFT` | Right-to-left text flow (Hebrew, Arabic, Urdu, etc.) |
| `HTM_AUTO` | Automatic directionality detection |


---

### HtmLinkType

Link classification based on href value and document context.

Used to categorize links during extraction for filtering and analysis.

| Value | Description |
|-------|-------------|
| `HTM_ANCHOR` | Anchor link within same document (href starts with #) |
| `HTM_INTERNAL` | Internal link within same domain |
| `HTM_EXTERNAL` | External link to different domain |
| `HTM_EMAIL` | Email link (mailto:) |
| `HTM_PHONE` | Phone link (tel:) |
| `HTM_OTHER` | Other protocol or unclassifiable |


---

### HtmImageType

Image source classification for proper handling and processing.

Determines whether an image is embedded (data URI), inline SVG, external, or relative.

| Value | Description |
|-------|-------------|
| `HTM_DATA_URI` | Data URI embedded image (base64 or other encoding) |
| `HTM_INLINE_SVG` | Inline SVG element |
| `HTM_EXTERNAL` | External image URL (http/https) |
| `HTM_RELATIVE` | Relative image path |


---

### HtmStructuredDataType

Structured data format type.

Identifies the schema/format used for structured data markup.

| Value | Description |
|-------|-------------|
| `HTM_JSON_LD` | JSON-LD (JSON for Linking Data) script blocks |
| `HTM_MICRODATA` | HTML5 Microdata attributes (itemscope, itemtype, itemprop) |
| `HTM_RDFA` | RDF in Attributes (RDFa) markup |


---

### HtmPreprocessingPreset

HTML preprocessing aggressiveness level.

Controls the extent of cleanup performed before conversion. Higher levels remove more elements.

| Value | Description |
|-------|-------------|
| `HTM_MINIMAL` | Minimal cleanup. Remove only essential noise (scripts, styles). |
| `HTM_STANDARD` | Standard cleanup. Default. Removes navigation, forms, and other auxiliary content. |
| `HTM_AGGRESSIVE` | Aggressive cleanup. Remove extensive non-content elements and structure. |


---

### HtmHeadingStyle

Heading style options for Markdown output.

Controls how headings (h1-h6) are rendered in the output Markdown.

| Value | Description |
|-------|-------------|
| `HTM_UNDERLINED` | Underlined style (=== for h1, --- for h2). |
| `HTM_ATX` | ATX style (# for h1, ## for h2, etc.). Default. |
| `HTM_ATX_CLOSED` | ATX closed style (# title #, with closing hashes). |


---

### HtmListIndentType

List indentation character type.

Controls whether list items are indented with spaces or tabs.

| Value | Description |
|-------|-------------|
| `HTM_SPACES` | Use spaces for indentation. Default. Width controlled by `list_indent_width`. |
| `HTM_TABS` | Use tabs for indentation. |


---

### HtmWhitespaceMode

Whitespace handling strategy during conversion.

Determines how sequences of whitespace characters (spaces, tabs, newlines) are processed.

| Value | Description |
|-------|-------------|
| `HTM_NORMALIZED` | Collapse multiple whitespace characters to single spaces. Default. Matches browser behavior. |
| `HTM_STRICT` | Preserve all whitespace exactly as it appears in the HTML. |


---

### HtmNewlineStyle

Line break syntax in Markdown output.

Controls how soft line breaks (from `<br>` or line breaks in source) are rendered.

| Value | Description |
|-------|-------------|
| `HTM_SPACES` | Two trailing spaces at end of line. Default. Standard Markdown syntax. |
| `HTM_BACKSLASH` | Backslash at end of line. Alternative Markdown syntax. |


---

### HtmCodeBlockStyle

Code block fence style in Markdown output.

Determines how code blocks (`<pre><code>`) are rendered in Markdown.

| Value | Description |
|-------|-------------|
| `HTM_INDENTED` | Indented code blocks (4 spaces). `CommonMark` standard. |
| `HTM_BACKTICKS` | Fenced code blocks with backticks (```). Default (GFM). Supports language hints. |
| `HTM_TILDES` | Fenced code blocks with tildes (~~~). Supports language hints. |


---

### HtmHighlightStyle

Highlight rendering style for `<mark>` elements.

Controls how highlighted text is rendered in Markdown output.

| Value | Description |
|-------|-------------|
| `HTM_DOUBLE_EQUAL` | Double equals syntax (==text==). Default. Pandoc-compatible. |
| `HTM_HTML` | Preserve as HTML (==text==). Original HTML tag. |
| `HTM_BOLD` | Render as bold (**text**). Uses strong emphasis. |
| `HTM_NONE` | Strip formatting, render as plain text. No markup. |


---

### HtmLinkStyle

Link rendering style in Markdown output.

Controls whether links and images use inline `[text](url)` syntax or
reference-style `[text][1]` syntax with definitions collected at the end.

| Value | Description |
|-------|-------------|
| `HTM_INLINE` | Inline links: `[text](url)`. Default. |
| `HTM_REFERENCE` | Reference-style links: `[text][1]` with `[1]: url` at end of document. |


---

### HtmOutputFormat

Output format for conversion.

Specifies the target markup language format for the conversion output.

| Value | Description |
|-------|-------------|
| `HTM_MARKDOWN` | Standard Markdown (CommonMark compatible). Default. |
| `HTM_DJOT` | Djot lightweight markup language. |
| `HTM_PLAIN` | Plain text output (no markup, visible text only). |


---

### HtmNodeContent

The semantic content type of a document node.

Uses internally tagged representation (`"node_type": "heading"`) for JSON serialization.

| Value | Description |
|-------|-------------|
| `HTM_HEADING` | A heading element (h1-h6). — Fields: `level`: `uint8_t`, `text`: `const char*` |
| `HTM_PARAGRAPH` | A paragraph of text. — Fields: `text`: `const char*` |
| `HTM_LIST` | A list container (ordered or unordered). Children are `ListItem` nodes. — Fields: `ordered`: `bool` |
| `HTM_LIST_ITEM` | A single list item. — Fields: `text`: `const char*` |
| `HTM_TABLE` | A table with structured cell data. — Fields: `grid`: `HtmTableGrid` |
| `HTM_IMAGE` | An image element. — Fields: `description`: `const char*`, `src`: `const char*`, `image_index`: `uint32_t` |
| `HTM_CODE` | A code block or inline code. — Fields: `text`: `const char*`, `language`: `const char*` |
| `HTM_QUOTE` | A block quote container. |
| `HTM_DEFINITION_LIST` | A definition list container. |
| `HTM_DEFINITION_ITEM` | A definition list entry with term and description. — Fields: `term`: `const char*`, `definition`: `const char*` |
| `HTM_RAW_BLOCK` | A raw block preserved as-is (e.g. `<script>`, `<style>` content). — Fields: `format`: `const char*`, `content`: `const char*` |
| `HTM_METADATA_BLOCK` | A block of key-value metadata pairs (from `<head>` meta tags). — Fields: `entries`: `HtmStringString*` |
| `HTM_GROUP` | A section grouping container (auto-generated from heading hierarchy). — Fields: `label`: `const char*`, `heading_level`: `uint8_t`, `heading_text`: `const char*` |


---

### HtmAnnotationKind

The type of an inline text annotation.

Uses internally tagged representation (`"annotation_type": "bold"`) for JSON serialization.

| Value | Description |
|-------|-------------|
| `HTM_BOLD` | Bold / strong emphasis. |
| `HTM_ITALIC` | Italic / emphasis. |
| `HTM_UNDERLINE` | Underline. |
| `HTM_STRIKETHROUGH` | Strikethrough / deleted text. |
| `HTM_CODE` | Inline code. |
| `HTM_SUBSCRIPT` | Subscript text. |
| `HTM_SUPERSCRIPT` | Superscript text. |
| `HTM_HIGHLIGHT` | Highlighted / marked text. |
| `HTM_LINK` | A hyperlink. — Fields: `url`: `const char*`, `title`: `const char*` |


---

### HtmWarningKind

Categories of processing warnings.

| Value | Description |
|-------|-------------|
| `HTM_IMAGE_EXTRACTION_FAILED` | An image could not be extracted (e.g. invalid data URI, unsupported format). |
| `HTM_ENCODING_FALLBACK` | The input encoding was not recognized; fell back to UTF-8. |
| `HTM_TRUNCATED_INPUT` | The input was truncated due to size limits. |
| `HTM_MALFORMED_HTML` | The HTML was malformed but processing continued with best effort. |
| `HTM_SANITIZATION_APPLIED` | Sanitization was applied to remove potentially unsafe content. |


---

### HtmNodeType

Node type enumeration covering all HTML element types.

This enum categorizes all HTML elements that the converter recognizes,
providing a coarse-grained classification for visitor dispatch.

| Value | Description |
|-------|-------------|
| `HTM_TEXT` | Text node (most frequent - 100+ per document) |
| `HTM_ELEMENT` | Generic element node |
| `HTM_HEADING` | Heading elements (h1-h6) |
| `HTM_PARAGRAPH` | Paragraph element |
| `HTM_DIV` | Generic div container |
| `HTM_BLOCKQUOTE` | Blockquote element |
| `HTM_PRE` | Preformatted text block |
| `HTM_HR` | Horizontal rule |
| `HTM_LIST` | Ordered or unordered list (ul, ol) |
| `HTM_LIST_ITEM` | List item (li) |
| `HTM_DEFINITION_LIST` | Definition list (dl) |
| `HTM_DEFINITION_TERM` | Definition term (dt) |
| `HTM_DEFINITION_DESCRIPTION` | Definition description (dd) |
| `HTM_TABLE` | Table element |
| `HTM_TABLE_ROW` | Table row (tr) |
| `HTM_TABLE_CELL` | Table cell (td, th) |
| `HTM_TABLE_HEADER` | Table header cell (th) |
| `HTM_TABLE_BODY` | Table body (tbody) |
| `HTM_TABLE_HEAD` | Table head (thead) |
| `HTM_TABLE_FOOT` | Table foot (tfoot) |
| `HTM_LINK` | Anchor link (a) |
| `HTM_IMAGE` | Image (img) |
| `HTM_STRONG` | Strong/bold (strong, b) |
| `HTM_EM` | Emphasis/italic (em, i) |
| `HTM_CODE` | Inline code (code) |
| `HTM_STRIKETHROUGH` | Strikethrough (s, del, strike) |
| `HTM_UNDERLINE` | Underline (u, ins) |
| `HTM_SUBSCRIPT` | Subscript (sub) |
| `HTM_SUPERSCRIPT` | Superscript (sup) |
| `HTM_MARK` | Mark/highlight (mark) |
| `HTM_SMALL` | Small text (small) |
| `HTM_BR` | Line break (br) |
| `HTM_SPAN` | Span element |
| `HTM_ARTICLE` | Article element |
| `HTM_SECTION` | Section element |
| `HTM_NAV` | Navigation element |
| `HTM_ASIDE` | Aside element |
| `HTM_HEADER` | Header element |
| `HTM_FOOTER` | Footer element |
| `HTM_MAIN` | Main element |
| `HTM_FIGURE` | Figure element |
| `HTM_FIGCAPTION` | Figure caption |
| `HTM_TIME` | Time element |
| `HTM_DETAILS` | Details element |
| `HTM_SUMMARY` | Summary element |
| `HTM_FORM` | Form element |
| `HTM_INPUT` | Input element |
| `HTM_SELECT` | Select element |
| `HTM_OPTION` | Option element |
| `HTM_BUTTON` | Button element |
| `HTM_TEXTAREA` | Textarea element |
| `HTM_LABEL` | Label element |
| `HTM_FIELDSET` | Fieldset element |
| `HTM_LEGEND` | Legend element |
| `HTM_AUDIO` | Audio element |
| `HTM_VIDEO` | Video element |
| `HTM_PICTURE` | Picture element |
| `HTM_SOURCE` | Source element |
| `HTM_IFRAME` | Iframe element |
| `HTM_SVG` | SVG element |
| `HTM_CANVAS` | Canvas element |
| `HTM_RUBY` | Ruby annotation |
| `HTM_RT` | Ruby text |
| `HTM_RP` | Ruby parenthesis |
| `HTM_ABBR` | Abbreviation |
| `HTM_KBD` | Keyboard input |
| `HTM_SAMP` | Sample output |
| `HTM_VAR` | Variable |
| `HTM_CITE` | Citation |
| `HTM_Q` | Quote |
| `HTM_DEL` | Deleted text |
| `HTM_INS` | Inserted text |
| `HTM_DATA` | Data element |
| `HTM_METER` | Meter element |
| `HTM_PROGRESS` | Progress element |
| `HTM_OUTPUT` | Output element |
| `HTM_TEMPLATE` | Template element |
| `HTM_SLOT` | Slot element |
| `HTM_HTML` | HTML root element |
| `HTM_HEAD` | Head element |
| `HTM_BODY` | Body element |
| `HTM_TITLE` | Title element |
| `HTM_META` | Meta element |
| `HTM_LINK_TAG` | Link element (not anchor) |
| `HTM_STYLE` | Style element |
| `HTM_SCRIPT` | Script element |
| `HTM_BASE` | Base element |
| `HTM_CUSTOM` | Custom element (web components) or unknown tag |


---

### HtmVisitResult

Result of a visitor callback.

Allows visitors to control the conversion flow by either proceeding
with default behavior, providing custom output, skipping elements,
preserving HTML, or signaling errors.

| Value | Description |
|-------|-------------|
| `HTM_CONTINUE` | Continue with default conversion behavior |
| `HTM_CUSTOM` | Replace default output with custom markdown The visitor takes full responsibility for the markdown output of this node and its children. — Fields: `0`: `const char*` |
| `HTM_SKIP` | Skip this element entirely (don't output anything) The element and all its children are ignored in the output. |
| `HTM_PRESERVE_HTML` | Preserve original HTML (don't convert to markdown) The element's raw HTML is included verbatim in the output. |
| `HTM_ERROR` | Stop conversion with an error The conversion process halts and returns this error message. — Fields: `0`: `const char*` |


---

### HtmVisitorDispatch

Result of dispatching a visitor callback.

This enum represents the outcome of a visitor callback dispatch,
providing a more ergonomic interface for control flow than the
raw `VisitResult` type.

| Value | Description |
|-------|-------------|
| `HTM_CONTINUE` | Continue with default conversion behavior |
| `HTM_CUSTOM` | Replace default output with custom markdown — Fields: `0`: `const char*` |
| `HTM_SKIP` | Skip this element entirely (don't output anything) |
| `HTM_PRESERVE_HTML` | Preserve original HTML (don't convert to markdown) |


---

## Errors

### HtmConversionError

Errors that can occur during HTML to Markdown conversion.

| Variant | Description |
|---------|-------------|
| `HTM_PARSE_ERROR` | HTML parsing error |
| `HTM_SANITIZATION_ERROR` | HTML sanitization error |
| `HTM_CONFIG_ERROR` | Invalid configuration |
| `HTM_IO_ERROR` | I/O error |
| `HTM_PANIC` | Internal error caught during conversion |
| `HTM_INVALID_INPUT` | Invalid input data |
| `HTM_OTHER` | Generic conversion error |


---

