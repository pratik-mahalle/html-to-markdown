---
title: "Python API Reference"
---

# Python API Reference <span class="version-badge">v3.2.0</span>

## Functions

### table_total_columns()

Calculate total columns in a table.

Scans all rows and cells to determine the maximum column count,
accounting for colspan values.

**Returns:**
Maximum column count (minimum 1, maximum MAX_TABLE_COLS)

**Signature:**

```python
def table_total_columns(node_handle: NodeHandle, parser: Parser, dom_ctx: DomContext) -> int
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `node_handle` | `NodeHandle` | Yes | Handle to the table element |
| `parser` | `Parser` | Yes | HTML parser instance |
| `dom_ctx` | `DomContext` | Yes | DOM context for tag name resolution |

**Returns:** `int`


---

### handle_table()

Convert an entire table element to Markdown.

Main entry point for table conversion. Analyzes table structure to determine
if it should be rendered as a Markdown table or converted to list format.
Handles layout tables, blank tables, and tables with semantic meaning.
Integrates with visitor pattern for custom table handling.

**Signature:**

```python
def handle_table(node_handle: NodeHandle, parser: Parser, output: str, options: ConversionOptions, ctx: Context, dom_ctx: DomContext, depth: int) -> None
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `node_handle` | `NodeHandle` | Yes | Handle to the table element |
| `parser` | `Parser` | Yes | HTML parser instance |
| `output` | `str` | Yes | Mutable string to append table content |
| `options` | `ConversionOptions` | Yes | Conversion options |
| `ctx` | `Context` | Yes | Conversion context (visitor, etc) |
| `dom_ctx` | `DomContext` | Yes | DOM context |
| `depth` | `int` | Yes | Nesting depth |

**Returns:** `None`


---

### handle_caption()

Handles caption elements within tables.

Extracts text content from the caption and formats it as italicized text
with escaped hyphens to prevent Markdown table separator interpretation.

**Signature:**

```python
def handle_caption(node_handle: NodeHandle, parser: Parser, output: str, options: ConversionOptions, ctx: Context, depth: int, dom_ctx: DomContext) -> None
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `node_handle` | `NodeHandle` | Yes | Handle to the caption element |
| `parser` | `Parser` | Yes | HTML parser instance |
| `output` | `str` | Yes | Output string to append caption text to |
| `options` | `ConversionOptions` | Yes | Conversion options |
| `ctx` | `Context` | Yes | Conversion context |
| `depth` | `int` | Yes | Current recursion depth |
| `dom_ctx` | `DomContext` | Yes | DOM context for tag name resolution |

**Returns:** `None`


---

### get_colspan()

Get colspan attribute value from an element.

Reads the colspan attribute from a table cell, with bounds checking
to prevent memory exhaustion attacks.

**Returns:**
The colspan value (minimum 1, maximum MAX_TABLE_COLS)

**Signature:**

```python
def get_colspan(node_handle: NodeHandle, parser: Parser) -> int
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `node_handle` | `NodeHandle` | Yes | Handle to the cell element |
| `parser` | `Parser` | Yes | HTML parser instance |

**Returns:** `int`


---

### get_colspan_rowspan()

Get both colspan and rowspan in a single lookup.

More efficient than calling get_colspan and a separate rowspan lookup.

**Returns:**
A tuple of (colspan, rowspan), both minimum 1 and maximum MAX_TABLE_COLS

**Signature:**

```python
def get_colspan_rowspan(node_handle: NodeHandle, parser: Parser) -> UsizeUsize
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `node_handle` | `NodeHandle` | Yes | Handle to the cell element |
| `parser` | `Parser` | Yes | HTML parser instance |

**Returns:** `UsizeUsize`


---

### collect_table_cells()

Collect table cells (td/th) from a row element.

Extracts only the direct cell children of a row, filtering by tag name.

**Signature:**

```python
def collect_table_cells(node_handle: NodeHandle, parser: Parser, dom_ctx: DomContext, cells: list[NodeHandle]) -> None
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `node_handle` | `NodeHandle` | Yes | Handle to the row element |
| `parser` | `Parser` | Yes | HTML parser instance |
| `dom_ctx` | `DomContext` | Yes | DOM context for tag name resolution |
| `cells` | `list[NodeHandle]` | Yes | Mutable vector to populate with cell handles |

**Returns:** `None`


---

### convert_table_cell()

Convert a table cell (td or th) to Markdown format.

Processes cell content and renders it with pipe delimiters for Markdown tables.
Handles colspan by adding extra pipes, and escapes pipes in cell content.

**Signature:**

```python
def convert_table_cell(node_handle: NodeHandle, parser: Parser, output: str, options: ConversionOptions, ctx: Context, tag_name: str, dom_ctx: DomContext) -> None
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `node_handle` | `NodeHandle` | Yes | Handle to the cell element |
| `parser` | `Parser` | Yes | HTML parser instance |
| `output` | `str` | Yes | Mutable string to append cell content |
| `options` | `ConversionOptions` | Yes | Conversion options (escape settings, br_in_tables) |
| `ctx` | `Context` | Yes | Conversion context (visitor, etc) |
| `tag_name` | `str` | Yes | Tag name (for consistency, not used) |
| `dom_ctx` | `DomContext` | Yes | DOM context for content extraction |

**Returns:** `None`


---

### append_layout_row()

Append a layout table row as a list item.

For tables used for visual layout, converts rows to list items
instead of table format for better readability.

**Signature:**

```python
def append_layout_row(row_handle: NodeHandle, parser: Parser, output: str, options: ConversionOptions, ctx: Context, dom_ctx: DomContext) -> None
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `row_handle` | `NodeHandle` | Yes | Handle to the row element |
| `parser` | `Parser` | Yes | HTML parser instance |
| `output` | `str` | Yes | Mutable string to append content |
| `options` | `ConversionOptions` | Yes | Conversion options |
| `ctx` | `Context` | Yes | Conversion context |
| `dom_ctx` | `DomContext` | Yes | DOM context |

**Returns:** `None`


---

### convert_table_row()

Convert a table row (tr) to Markdown format.

Processes all cells in a row, handling colspan and rowspan for proper
column alignment. Renders header separator row after the first row.
Integrates with visitor pattern for custom row handling.

**Signature:**

```python
def convert_table_row(node_handle: NodeHandle, parser: Parser, output: str, options: ConversionOptions, ctx: Context, row_index: int, has_span: bool, rowspan_tracker: list[int | None], total_cols: int, header_cols: int, dom_ctx: DomContext, depth: int, is_header: bool) -> None
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `node_handle` | `NodeHandle` | Yes | Handle to the row element |
| `parser` | `Parser` | Yes | HTML parser instance |
| `output` | `str` | Yes | Mutable string to append row content |
| `options` | `ConversionOptions` | Yes | Conversion options |
| `ctx` | `Context` | Yes | Conversion context (visitor, etc) |
| `row_index` | `int` | Yes | Index of this row in the table |
| `has_span` | `bool` | Yes | Whether table has colspan/rowspan |
| `rowspan_tracker` | `list[int | None]` | Yes | Mutable array tracking rowspan remainder for each column |
| `total_cols` | `int` | Yes | Total columns in the table |
| `header_cols` | `int` | Yes | Columns to render in separator row |
| `dom_ctx` | `DomContext` | Yes | DOM context |
| `depth` | `int` | Yes | Nesting depth |
| `is_header` | `bool` | Yes | Whether this is a header row |

**Returns:** `None`


---

### scan_table()

Scan a table element for structural metadata.

Analyzes the table to determine characteristics that influence rendering:
- Whether to render as a Markdown table or layout table
- If spanning cells are present
- If the table has semantic meaning (headers, captions)

**Signature:**

```python
def scan_table(node_handle: NodeHandle, parser: Parser, dom_ctx: DomContext) -> TableScan
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `node_handle` | `NodeHandle` | Yes | Handle to the table element |
| `parser` | `Parser` | Yes | HTML parser instance |
| `dom_ctx` | `DomContext` | Yes | DOM context for tag name resolution |

**Returns:** `TableScan`


---

### dispatch_table_handler()

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

```python
def dispatch_table_handler(tag_name: str, node_handle: NodeHandle, parser: Parser, output: str, options: ConversionOptions, ctx: Context, depth: int, dom_ctx: DomContext) -> bool
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `tag_name` | `str` | Yes | The tag name |
| `node_handle` | `NodeHandle` | Yes | The node handle |
| `parser` | `Parser` | Yes | The parser |
| `output` | `str` | Yes | The output destination |
| `options` | `ConversionOptions` | Yes | The options to use |
| `ctx` | `Context` | Yes | The context |
| `depth` | `int` | Yes | The depth |
| `dom_ctx` | `DomContext` | Yes | The dom context |

**Returns:** `bool`


---

### dispatch_block_handler()

Dispatches block element handling to the appropriate handler.

This function is designed to be called from the main walk_node function
in converter.rs once the module is refactored. It returns `True` if the
element was handled, `False` otherwise.

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

```python
def dispatch_block_handler(tag_name: str, node_handle: NodeHandle, parser: Parser, output: str, options: ConversionOptions, ctx: Context, depth: int, dom_ctx: DomContext) -> bool
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `tag_name` | `str` | Yes | The tag name |
| `node_handle` | `NodeHandle` | Yes | The node handle |
| `parser` | `Parser` | Yes | The parser |
| `output` | `str` | Yes | The output destination |
| `options` | `ConversionOptions` | Yes | The options to use |
| `ctx` | `Context` | Yes | The context |
| `depth` | `int` | Yes | The depth |
| `dom_ctx` | `DomContext` | Yes | The dom context |

**Returns:** `bool`


---

### handle()

Dispatcher for form elements.

Routes all form-related elements to their respective handlers.

**Signature:**

```python
def handle(tag_name: str, node_handle: NodeHandle, parser: Parser, output: str, options: ConversionOptions, ctx: Context, depth: int, dom_ctx: DomContext) -> None
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `tag_name` | `str` | Yes | The tag name |
| `node_handle` | `NodeHandle` | Yes | The node handle |
| `parser` | `Parser` | Yes | The parser |
| `output` | `str` | Yes | The output destination |
| `options` | `ConversionOptions` | Yes | The options to use |
| `ctx` | `Context` | Yes | The context |
| `depth` | `int` | Yes | The depth |
| `dom_ctx` | `DomContext` | Yes | The dom context |

**Returns:** `None`


---

### dispatch_form_handler()

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

Returns `True` if the tag was successfully handled by a form handler,
`False` if the tag is not a form element and requires other handling.

**Signature:**

```python
def dispatch_form_handler(tag_name: str, node_handle: NodeHandle, parser: Parser, output: str, options: ConversionOptions, ctx: Context, depth: int, dom_ctx: DomContext) -> bool
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `tag_name` | `str` | Yes | The tag name |
| `node_handle` | `NodeHandle` | Yes | The node handle |
| `parser` | `Parser` | Yes | The parser |
| `output` | `str` | Yes | The output destination |
| `options` | `ConversionOptions` | Yes | The options to use |
| `ctx` | `Context` | Yes | The context |
| `depth` | `int` | Yes | The depth |
| `dom_ctx` | `DomContext` | Yes | The dom context |

**Returns:** `bool`


---

### handle_blockquote()

Handle a `<blockquote>` element and convert to Markdown.

This handler processes blockquote elements including:
- Converting inline blockquotes by processing children as inline
- Handling nested blockquotes via blockquote_depth tracking
- Processing citation URLs from cite attribute
- Invoking visitor callbacks when the visitor feature is enabled
- Adding proper spacing and blockquote prefix formatting

**Signature:**

```python
def handle_blockquote(node_handle: NodeHandle, tag: HtmlTag, parser: Parser, output: str, options: ConversionOptions, ctx: Context, depth: int, dom_ctx: DomContext) -> None
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `node_handle` | `NodeHandle` | Yes | The node handle |
| `tag` | `HtmlTag` | Yes | The h t m l tag |
| `parser` | `Parser` | Yes | The parser |
| `output` | `str` | Yes | The output destination |
| `options` | `ConversionOptions` | Yes | The options to use |
| `ctx` | `Context` | Yes | The context |
| `depth` | `int` | Yes | The depth |
| `dom_ctx` | `DomContext` | Yes | The dom context |

**Returns:** `None`


---

### handle_code()

Handle an inline `<code>` element and convert to Markdown.

This handler processes inline code elements including:
- Extracting code content and applying backtick delimiters
- Handling backticks in content by using multiple delimiters
- Invoking visitor callbacks when the visitor feature is enabled
- Generating appropriate markdown output with proper escaping

**Signature:**

```python
def handle_code(node_handle: NodeHandle, tag: HtmlTag, parser: Parser, output: str, options: ConversionOptions, ctx: Context, depth: int, dom_ctx: DomContext) -> None
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `node_handle` | `NodeHandle` | Yes | The node handle |
| `tag` | `HtmlTag` | Yes | The h t m l tag |
| `parser` | `Parser` | Yes | The parser |
| `output` | `str` | Yes | The output destination |
| `options` | `ConversionOptions` | Yes | The options to use |
| `ctx` | `Context` | Yes | The context |
| `depth` | `int` | Yes | The depth |
| `dom_ctx` | `DomContext` | Yes | The dom context |

**Returns:** `None`


---

### handle_pre()

Handle a `<pre>` element and convert to Markdown.

This handler processes code block elements including:
- Extracting language information from class attributes
- Processing whitespace and dedenting code content
- Supporting multiple code block styles (indented, backticks, tildes)
- Invoking visitor callbacks when the visitor feature is enabled
- Generating appropriate markdown output

**Signature:**

```python
def handle_pre(node_handle: NodeHandle, tag: HtmlTag, parser: Parser, output: str, options: ConversionOptions, ctx: Context, depth: int, dom_ctx: DomContext) -> None
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `node_handle` | `NodeHandle` | Yes | The node handle |
| `tag` | `HtmlTag` | Yes | The h t m l tag |
| `parser` | `Parser` | Yes | The parser |
| `output` | `str` | Yes | The output destination |
| `options` | `ConversionOptions` | Yes | The options to use |
| `ctx` | `Context` | Yes | The context |
| `depth` | `int` | Yes | The depth |
| `dom_ctx` | `DomContext` | Yes | The dom context |

**Returns:** `None`


---

### handle_graphic()

Handle a `<graphic>` element and convert to Markdown.

This handler processes graphic elements including:
- Extracting source from url, href, xlink:href, or src attributes
- Using alt attribute, with fallback to filename
- Collecting metadata when the metadata feature is enabled
- Invoking visitor callbacks when the visitor feature is enabled
- Generating appropriate markdown output

**Signature:**

```python
def handle_graphic(node_handle: NodeHandle, tag: HtmlTag, parser: Parser, output: str, options: ConversionOptions, ctx: Context, depth: int, dom_ctx: DomContext) -> None
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `node_handle` | `NodeHandle` | Yes | The node handle |
| `tag` | `HtmlTag` | Yes | The h t m l tag |
| `parser` | `Parser` | Yes | The parser |
| `output` | `str` | Yes | The output destination |
| `options` | `ConversionOptions` | Yes | The options to use |
| `ctx` | `Context` | Yes | The context |
| `depth` | `int` | Yes | The depth |
| `dom_ctx` | `DomContext` | Yes | The dom context |

**Returns:** `None`


---

### handle_img()

Handle an `<img>` element and convert to Markdown.

This handler processes image elements including:
- Extracting src, alt, and title attributes
- Collecting metadata when the metadata feature is enabled
- Handling inline data URIs when the inline-images feature is enabled
- Invoking visitor callbacks when the visitor feature is enabled
- Generating appropriate markdown output

**Signature:**

```python
def handle_img(node_handle: NodeHandle, tag: HtmlTag, parser: Parser, output: str, options: ConversionOptions, ctx: Context, depth: int, dom_ctx: DomContext) -> None
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `node_handle` | `NodeHandle` | Yes | The node handle |
| `tag` | `HtmlTag` | Yes | The h t m l tag |
| `parser` | `Parser` | Yes | The parser |
| `output` | `str` | Yes | The output destination |
| `options` | `ConversionOptions` | Yes | The options to use |
| `ctx` | `Context` | Yes | The context |
| `depth` | `int` | Yes | The depth |
| `dom_ctx` | `DomContext` | Yes | The dom context |

**Returns:** `None`


---

### handle_link()

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

```python
def handle_link(node_handle: NodeHandle, tag: HtmlTag, parser: Parser, output: str, options: ConversionOptions, ctx: Context, depth: int, dom_ctx: DomContext) -> None
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `node_handle` | `NodeHandle` | Yes | The node handle |
| `tag` | `HtmlTag` | Yes | The h t m l tag |
| `parser` | `Parser` | Yes | The parser |
| `output` | `str` | Yes | The output destination |
| `options` | `ConversionOptions` | Yes | The options to use |
| `ctx` | `Context` | Yes | The context |
| `depth` | `int` | Yes | The depth |
| `dom_ctx` | `DomContext` | Yes | The dom context |

**Returns:** `None`


---

### dispatch_inline_handler()

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

Returns `True` if the tag was recognized and handled, `False` otherwise.
This allows the caller to distinguish between:
- Handled inline elements (return `True`)
- Unhandled elements (return `False`) that should be processed as text or passed through

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
3. Returns `True`
4. Emphasis handler outputs `**Bold text**` to output buffer

For `<span>Normal text</span>`, the dispatcher:
1. Fails to recognize "span" tag
2. Returns `False`
3. Caller processes as default inline content

**Signature:**

```python
def dispatch_inline_handler(tag_name: str, node_handle: NodeHandle, parser: Parser, output: str, options: ConversionOptions, ctx: Context, depth: int, dom_ctx: DomContext) -> bool
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `tag_name` | `str` | Yes | The normalized HTML tag name (lowercase) |
| `node_handle` | `NodeHandle` | Yes | The DOM node handle from the parser |
| `parser` | `Parser` | Yes | Reference to the tl HTML parser |
| `output` | `str` | Yes | Output buffer to write converted content to |
| `options` | `ConversionOptions` | Yes | Conversion configuration options |
| `ctx` | `Context` | Yes | Processing context with state tracking |
| `depth` | `int` | Yes | Current DOM tree depth for recursion tracking |
| `dom_ctx` | `DomContext` | Yes | DOM context for accessing tree structure |

**Returns:** `bool`


---

### calculate_list_continuation_indent()

Calculate indentation level for list item continuations.

Returns the number of 4-space indent groups needed for list continuations.

List continuations (block elements inside list items) need special indentation:
- Base indentation: (depth - 1) groups (for the nesting level)
- Content indentation: depth groups (for the list item content)
- Combined formula: (2 * depth - 1) groups of 4 spaces each

**Signature:**

```python
def calculate_list_continuation_indent(depth: int) -> int
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `depth` | `int` | Yes | The depth |

**Returns:** `int`


---

### is_loose_list()

Check if a list (ul or ol) is "loose".

A loose list is one where any list item contains block-level elements
like paragraphs (<p>). In loose lists, all items should have blank line
separation (ending with \n\n) regardless of their own content.

**Signature:**

```python
def is_loose_list(node_handle: NodeHandle, parser: Parser, dom_ctx: DomContext) -> bool
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `node_handle` | `NodeHandle` | Yes | The node handle |
| `parser` | `Parser` | Yes | The parser |
| `dom_ctx` | `DomContext` | Yes | The dom context |

**Returns:** `bool`


---

### add_list_continuation_indent()

Add list continuation indentation to output.

Used when block elements (like <p> or <div>) appear inside list items.
Adds appropriate line separation and indentation to continue the list item.

**Signature:**

```python
def add_list_continuation_indent(output: str, list_depth: int, blank_line: bool, options: ConversionOptions) -> None
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `output` | `str` | Yes | The output string to append to |
| `list_depth` | `int` | Yes | Current list nesting depth |
| `blank_line` | `bool` | Yes | If true, adds blank line separation (\n\n); if false, single newline (\n) |
| `options` | `ConversionOptions` | Yes | The options to use |

**Returns:** `None`


---

### continuation_indent_string()

Calculate the indentation string for list continuations based on depth and options.

**Signature:**

```python
def continuation_indent_string(list_depth: int, options: ConversionOptions) -> str | None
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `list_depth` | `int` | Yes | The list depth |
| `options` | `ConversionOptions` | Yes | The options to use |

**Returns:** `str | None`


---

### add_list_leading_separator()

Add appropriate leading separator before a list.

Lists need different separators depending on context:
- In table cells: <br> tag if there's already content
- Outside lists: blank line (\n\n) if needed
- Inside list items: blank line before nested list

**Signature:**

```python
def add_list_leading_separator(output: str, ctx: Context) -> None
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `output` | `str` | Yes | The output destination |
| `ctx` | `Context` | Yes | The context |

**Returns:** `None`


---

### add_nested_list_trailing_separator()

Add appropriate trailing separator after a nested list.

Nested lists inside list items need trailing newlines to separate
from following content. In loose lists, use blank line (\n\n). In tight lists, single newline (\n).

**Signature:**

```python
def add_nested_list_trailing_separator(output: str, ctx: Context) -> None
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `output` | `str` | Yes | The output destination |
| `ctx` | `Context` | Yes | The context |

**Returns:** `None`


---

### calculate_list_nesting_depth()

Calculate the nesting depth for a list.

If we're in a list but NOT in a list item, this is incorrectly nested HTML
and we need to increment the depth. If in a list item, the depth was already
incremented by the <li> element.

**Signature:**

```python
def calculate_list_nesting_depth(ctx: Context) -> int
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `ctx` | `Context` | Yes | The context |

**Returns:** `int`


---

### is_list_item()

Check if a node is a list item element.

**Signature:**

```python
def is_list_item(node_handle: NodeHandle, parser: Parser, dom_ctx: DomContext) -> bool
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `node_handle` | `NodeHandle` | Yes | The node handle |
| `parser` | `Parser` | Yes | The parser |
| `dom_ctx` | `DomContext` | Yes | The dom context |

**Returns:** `bool`


---

### process_list_children()

Process a list's children, tracking which items had block elements.

This is used to determine proper spacing between list items.
Returns true if the last processed item had block children.

**Signature:**

```python
def process_list_children(node_handle: NodeHandle, parser: Parser, output: str, options: ConversionOptions, ctx: Context, depth: int, is_ordered: bool, is_loose: bool, nested_depth: int, start_counter: int, dom_ctx: DomContext) -> None
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `node_handle` | `NodeHandle` | Yes | The node handle |
| `parser` | `Parser` | Yes | The parser |
| `output` | `str` | Yes | The output destination |
| `options` | `ConversionOptions` | Yes | The options to use |
| `ctx` | `Context` | Yes | The context |
| `depth` | `int` | Yes | The depth |
| `is_ordered` | `bool` | Yes | The is ordered |
| `is_loose` | `bool` | Yes | The is loose |
| `nested_depth` | `int` | Yes | The nested depth |
| `start_counter` | `int` | Yes | The start counter |
| `dom_ctx` | `DomContext` | Yes | The dom context |

**Returns:** `None`


---

### dispatch_list_handler()

Dispatches list element handling to the appropriate handler.

Returns `True` if the element was handled, `False` otherwise.

# Supported Elements

- `ol`: Ordered list - routed to `ordered.handle`
- `ul`: Unordered list - routed to `unordered.handle`
- `li`: List item - routed to `item.handle_li`
- `dl`: Definition list - routed to `definition.handle_dl`
- `dt`: Definition term - routed to `definition.handle_dt`
- `dd`: Definition description - routed to `definition.handle_dd`

**Signature:**

```python
def dispatch_list_handler(tag_name: str, node_handle: NodeHandle, tag: HtmlTag, parser: Parser, output: str, options: ConversionOptions, ctx: Context, depth: int, dom_ctx: DomContext) -> bool
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `tag_name` | `str` | Yes | The tag name |
| `node_handle` | `NodeHandle` | Yes | The node handle |
| `tag` | `HtmlTag` | Yes | The h t m l tag |
| `parser` | `Parser` | Yes | The parser |
| `output` | `str` | Yes | The output destination |
| `options` | `ConversionOptions` | Yes | The options to use |
| `ctx` | `Context` | Yes | The context |
| `depth` | `int` | Yes | The depth |
| `dom_ctx` | `DomContext` | Yes | The dom context |

**Returns:** `bool`


---

### convert_html()

Converts HTML to Markdown using the provided conversion options.

This is the main entry point for HTML to Markdown conversion.

**Signature:**

```python
def convert_html(html: str, options: ConversionOptions) -> str
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `html` | `str` | Yes | The html |
| `options` | `ConversionOptions` | Yes | The options to use |

**Returns:** `str`

**Errors:** Raises `Error`.


---

### convert_html_with_visitor()

Converts HTML to Markdown with a custom visitor for callbacks during traversal.

This variant allows passing a visitor that will receive callbacks for each node
during the tree walk, enabling custom processing or analysis.

**Signature:**

```python
def convert_html_with_visitor(html: str, options: ConversionOptions, visitor: VisitorHandle = None) -> str
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `html` | `str` | Yes | The html |
| `options` | `ConversionOptions` | Yes | The options to use |
| `visitor` | `VisitorHandle | None` | No | The visitor handle |

**Returns:** `str`

**Errors:** Raises `Error`.


---

### dispatch_media_handler()

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

Returns `True` if the tag was recognized and handled, `False` otherwise.

**Signature:**

```python
def dispatch_media_handler(tag_name: str, node_handle: NodeHandle, parser: Parser, output: str, options: ConversionOptions, ctx: Context, depth: int, dom_ctx: DomContext) -> bool
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `tag_name` | `str` | Yes | The tag name |
| `node_handle` | `NodeHandle` | Yes | The node handle |
| `parser` | `Parser` | Yes | The parser |
| `output` | `str` | Yes | The output destination |
| `options` | `ConversionOptions` | Yes | The options to use |
| `ctx` | `Context` | Yes | The context |
| `depth` | `int` | Yes | The depth |
| `dom_ctx` | `DomContext` | Yes | The dom context |

**Returns:** `bool`


---

### extract_plain_text()

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

```python
def extract_plain_text(dom: VDom, parser: Parser, options: ConversionOptions) -> str
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `dom` | `VDom` | Yes | The v dom |
| `parser` | `Parser` | Yes | The parser |
| `options` | `ConversionOptions` | Yes | The options to use |

**Returns:** `str`


---

### handle_dfn()

Handles the `<dfn>` element.

A dfn element marks a term that is being defined. The content represents
the term, and its definition would typically appear in surrounding context.
It is rendered as emphasized (italic) text.

# Behavior

- Content is collected from children
- Non-empty content is wrapped with the configured emphasis symbol (default: `*`)
- Inline suffix handling is applied (e.g., footnote references)

**Signature:**

```python
def handle_dfn(tag_name: str, node_handle: NodeHandle, parser: Parser, output: str, options: ConversionOptions, ctx: Context, depth: int, dom_ctx: DomContext) -> None
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `tag_name` | `str` | Yes | The  tag name |
| `node_handle` | `NodeHandle` | Yes | The node handle |
| `parser` | `Parser` | Yes | The parser |
| `output` | `str` | Yes | The output destination |
| `options` | `ConversionOptions` | Yes | The options to use |
| `ctx` | `Context` | Yes | The context |
| `depth` | `int` | Yes | The depth |
| `dom_ctx` | `DomContext` | Yes | The dom context |

**Returns:** `None`


---

### handle_abbr()

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

```python
def handle_abbr(tag_name: str, node_handle: NodeHandle, parser: Parser, output: str, options: ConversionOptions, ctx: Context, depth: int, dom_ctx: DomContext) -> None
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `tag_name` | `str` | Yes | The  tag name |
| `node_handle` | `NodeHandle` | Yes | The node handle |
| `parser` | `Parser` | Yes | The parser |
| `output` | `str` | Yes | The output destination |
| `options` | `ConversionOptions` | Yes | The options to use |
| `ctx` | `Context` | Yes | The context |
| `depth` | `int` | Yes | The depth |
| `dom_ctx` | `DomContext` | Yes | The dom context |

**Returns:** `None`


---

### handle_time_data()

Handles the `<time>` and `<data>` elements.

Time and data elements contain machine-readable content in their attributes
and human-readable content in their text. For Markdown purposes, we output
only the human-readable text content, as Markdown doesn't have a way to
preserve machine-readable metadata.

# Behavior

- Content is extracted from children and output as-is
- Attributes (datetime, value) are not rendered in Markdown output

**Signature:**

```python
def handle_time_data(tag_name: str, node_handle: NodeHandle, parser: Parser, output: str, options: ConversionOptions, ctx: Context, depth: int, dom_ctx: DomContext) -> None
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `tag_name` | `str` | Yes | The  tag name |
| `node_handle` | `NodeHandle` | Yes | The node handle |
| `parser` | `Parser` | Yes | The parser |
| `output` | `str` | Yes | The output destination |
| `options` | `ConversionOptions` | Yes | The options to use |
| `ctx` | `Context` | Yes | The context |
| `depth` | `int` | Yes | The depth |
| `dom_ctx` | `DomContext` | Yes | The dom context |

**Returns:** `None`


---

### handle_cite()

Handles the `<cite>` element.

A cite element marks the title of a cited work (book, article, website, etc.).
It is rendered as emphasized (italic) text in block mode, or as plain text in inline mode.

# Behavior

- **Block mode**: Content is wrapped with emphasis markers (default: `*`)
- **Inline mode**: Content is output as-is without formatting

**Signature:**

```python
def handle_cite(tag_name: str, node_handle: NodeHandle, parser: Parser, output: str, options: ConversionOptions, ctx: Context, depth: int, dom_ctx: DomContext) -> None
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `tag_name` | `str` | Yes | The  tag name |
| `node_handle` | `NodeHandle` | Yes | The node handle |
| `parser` | `Parser` | Yes | The parser |
| `output` | `str` | Yes | The output destination |
| `options` | `ConversionOptions` | Yes | The options to use |
| `ctx` | `Context` | Yes | The context |
| `depth` | `int` | Yes | The depth |
| `dom_ctx` | `DomContext` | Yes | The dom context |

**Returns:** `None`


---

### handle_q()

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

```python
def handle_q(tag_name: str, node_handle: NodeHandle, parser: Parser, output: str, options: ConversionOptions, ctx: Context, depth: int, dom_ctx: DomContext) -> None
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `tag_name` | `str` | Yes | The  tag name |
| `node_handle` | `NodeHandle` | Yes | The node handle |
| `parser` | `Parser` | Yes | The parser |
| `output` | `str` | Yes | The output destination |
| `options` | `ConversionOptions` | Yes | The options to use |
| `ctx` | `Context` | Yes | The context |
| `depth` | `int` | Yes | The depth |
| `dom_ctx` | `DomContext` | Yes | The dom context |

**Returns:** `None`


---

### handle_hgroup()

Handles the `<hgroup>` element.

An hgroup element groups related headings together (e.g., a title and subtitle).
In Markdown, we simply process all children sequentially, allowing nested
headings to maintain their individual formatting.

# Behavior

- Children are processed sequentially in the current context
- No special formatting is applied at the hgroup level

**Signature:**

```python
def handle_hgroup(tag_name: str, node_handle: NodeHandle, parser: Parser, output: str, options: ConversionOptions, ctx: Context, depth: int, dom_ctx: DomContext) -> None
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `tag_name` | `str` | Yes | The  tag name |
| `node_handle` | `NodeHandle` | Yes | The node handle |
| `parser` | `Parser` | Yes | The parser |
| `output` | `str` | Yes | The output destination |
| `options` | `ConversionOptions` | Yes | The options to use |
| `ctx` | `Context` | Yes | The context |
| `depth` | `int` | Yes | The depth |
| `dom_ctx` | `DomContext` | Yes | The dom context |

**Returns:** `None`


---

### handle_dl()

Handles the `<dl>` element.

A definition list contains terms and their definitions. Terms and definitions
are output as plain blocks without Pandoc-style colon syntax, since standard
Markdown and GFM do not support definition lists.

# Behavior

- **Inline mode**: Children are processed inline without block spacing
- **Block mode**: Content is collected and wrapped with proper spacing

**Signature:**

```python
def handle_dl(tag_name: str, node_handle: NodeHandle, parser: Parser, output: str, options: ConversionOptions, ctx: Context, depth: int, dom_ctx: DomContext) -> None
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `tag_name` | `str` | Yes | The  tag name |
| `node_handle` | `NodeHandle` | Yes | The node handle |
| `parser` | `Parser` | Yes | The parser |
| `output` | `str` | Yes | The output destination |
| `options` | `ConversionOptions` | Yes | The options to use |
| `ctx` | `Context` | Yes | The context |
| `depth` | `int` | Yes | The depth |
| `dom_ctx` | `DomContext` | Yes | The dom context |

**Returns:** `None`


---

### handle_dt()

Handles the `<dt>` element.

A dt element contains a term being defined. Terms are output on their own line,
with definitions following on subsequent lines.

# Behavior

- **Inline mode**: Content is output as-is
- **Block mode**: Content is followed by a newline

**Signature:**

```python
def handle_dt(tag_name: str, node_handle: NodeHandle, parser: Parser, output: str, options: ConversionOptions, ctx: Context, depth: int, dom_ctx: DomContext) -> None
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `tag_name` | `str` | Yes | The  tag name |
| `node_handle` | `NodeHandle` | Yes | The node handle |
| `parser` | `Parser` | Yes | The parser |
| `output` | `str` | Yes | The output destination |
| `options` | `ConversionOptions` | Yes | The options to use |
| `ctx` | `Context` | Yes | The context |
| `depth` | `int` | Yes | The depth |
| `dom_ctx` | `DomContext` | Yes | The dom context |

**Returns:** `None`


---

### handle_dd()

Handles the `<dd>` element.

A dd element contains the definition for a term. It is output as a plain
block since standard Markdown and GFM do not support definition list syntax.

# Behavior

- **Inline mode**: Content is output as-is
- **Block mode**: Content is output as a block

**Signature:**

```python
def handle_dd(tag_name: str, node_handle: NodeHandle, parser: Parser, output: str, options: ConversionOptions, ctx: Context, depth: int, dom_ctx: DomContext) -> None
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `tag_name` | `str` | Yes | The  tag name |
| `node_handle` | `NodeHandle` | Yes | The node handle |
| `parser` | `Parser` | Yes | The parser |
| `output` | `str` | Yes | The output destination |
| `options` | `ConversionOptions` | Yes | The options to use |
| `ctx` | `Context` | Yes | The context |
| `depth` | `int` | Yes | The depth |
| `dom_ctx` | `DomContext` | Yes | The dom context |

**Returns:** `None`


---

### handle_menu()

Handles the `<menu>` element.

A menu element is a semantic list, typically used for command menus or
navigation. It is rendered as an unordered list with dashes.

# Behavior

- **Inline mode**: Children are processed inline without list formatting
- **Block mode**: Content is rendered as an unordered list
- Uses `-` as the list bullet (overrides configured bullets)
- Proper blank-line spacing is maintained

**Signature:**

```python
def handle_menu(tag_name: str, node_handle: NodeHandle, parser: Parser, output: str, options: ConversionOptions, ctx: Context, depth: int, dom_ctx: DomContext) -> None
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `tag_name` | `str` | Yes | The  tag name |
| `node_handle` | `NodeHandle` | Yes | The node handle |
| `parser` | `Parser` | Yes | The parser |
| `output` | `str` | Yes | The output destination |
| `options` | `ConversionOptions` | Yes | The options to use |
| `ctx` | `Context` | Yes | The context |
| `depth` | `int` | Yes | The depth |
| `dom_ctx` | `DomContext` | Yes | The dom context |

**Returns:** `None`


---

### handle_figure()

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

```python
def handle_figure(tag_name: str, node_handle: NodeHandle, parser: Parser, output: str, options: ConversionOptions, ctx: Context, depth: int, dom_ctx: DomContext) -> None
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `tag_name` | `str` | Yes | The  tag name |
| `node_handle` | `NodeHandle` | Yes | The node handle |
| `parser` | `Parser` | Yes | The parser |
| `output` | `str` | Yes | The output destination |
| `options` | `ConversionOptions` | Yes | The options to use |
| `ctx` | `Context` | Yes | The context |
| `depth` | `int` | Yes | The depth |
| `dom_ctx` | `DomContext` | Yes | The dom context |

**Returns:** `None`


---

### handle_figcaption()

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

```python
def handle_figcaption(tag_name: str, node_handle: NodeHandle, parser: Parser, output: str, options: ConversionOptions, ctx: Context, depth: int, dom_ctx: DomContext) -> None
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `tag_name` | `str` | Yes | The  tag name |
| `node_handle` | `NodeHandle` | Yes | The node handle |
| `parser` | `Parser` | Yes | The parser |
| `output` | `str` | Yes | The output destination |
| `options` | `ConversionOptions` | Yes | The options to use |
| `ctx` | `Context` | Yes | The context |
| `depth` | `int` | Yes | The depth |
| `dom_ctx` | `DomContext` | Yes | The dom context |

**Returns:** `None`


---

### handle_details()

Handles the `<details>` element.

A details element represents a disclosure widget that can be toggled
to show/hide additional content. In Markdown, it's rendered as a block
with all content visible.

# Behavior

- **Inline mode**: Children are processed inline without block spacing
- **Block mode**: Content is collected and wrapped with proper blank-line spacing
- **Empty content**: Skipped entirely

**Signature:**

```python
def handle_details(tag_name: str, node_handle: NodeHandle, parser: Parser, output: str, options: ConversionOptions, ctx: Context, depth: int, dom_ctx: DomContext) -> None
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `tag_name` | `str` | Yes | The  tag name |
| `node_handle` | `NodeHandle` | Yes | The node handle |
| `parser` | `Parser` | Yes | The parser |
| `output` | `str` | Yes | The output destination |
| `options` | `ConversionOptions` | Yes | The options to use |
| `ctx` | `Context` | Yes | The context |
| `depth` | `int` | Yes | The depth |
| `dom_ctx` | `DomContext` | Yes | The dom context |

**Returns:** `None`


---

### handle_summary()

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

```python
def handle_summary(tag_name: str, node_handle: NodeHandle, parser: Parser, output: str, options: ConversionOptions, ctx: Context, depth: int, dom_ctx: DomContext) -> None
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `tag_name` | `str` | Yes | The  tag name |
| `node_handle` | `NodeHandle` | Yes | The node handle |
| `parser` | `Parser` | Yes | The parser |
| `output` | `str` | Yes | The output destination |
| `options` | `ConversionOptions` | Yes | The options to use |
| `ctx` | `Context` | Yes | The context |
| `depth` | `int` | Yes | The depth |
| `dom_ctx` | `DomContext` | Yes | The dom context |

**Returns:** `None`


---

### handle_dialog()

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

```python
def handle_dialog(tag_name: str, node_handle: NodeHandle, parser: Parser, output: str, options: ConversionOptions, ctx: Context, depth: int, dom_ctx: DomContext) -> None
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `tag_name` | `str` | Yes | The  tag name |
| `node_handle` | `NodeHandle` | Yes | The node handle |
| `parser` | `Parser` | Yes | The parser |
| `output` | `str` | Yes | The output destination |
| `options` | `ConversionOptions` | Yes | The options to use |
| `ctx` | `Context` | Yes | The context |
| `depth` | `int` | Yes | The depth |
| `dom_ctx` | `DomContext` | Yes | The dom context |

**Returns:** `None`


---

### dispatch_semantic_handler()

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

Returns `True` if the tag was successfully handled by a semantic handler,
`False` if the tag is not a semantic element and requires other handling.

**Signature:**

```python
def dispatch_semantic_handler(tag_name: str, node_handle: NodeHandle, parser: Parser, output: str, options: ConversionOptions, ctx: Context, depth: int, dom_ctx: DomContext) -> bool
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `tag_name` | `str` | Yes | The tag name |
| `node_handle` | `NodeHandle` | Yes | The node handle |
| `parser` | `Parser` | Yes | The parser |
| `output` | `str` | Yes | The output destination |
| `options` | `ConversionOptions` | Yes | The options to use |
| `ctx` | `Context` | Yes | The context |
| `depth` | `int` | Yes | The depth |
| `dom_ctx` | `DomContext` | Yes | The dom context |

**Returns:** `bool`


---

### floor_char_boundary()

Returns the largest valid char boundary index at or before `index`.

If `index` is already a char boundary it is returned unchanged.
Otherwise it walks backwards to find one.  Returns 0 if no boundary
is found before `index`.

**Signature:**

```python
def floor_char_boundary(s: str, index: int) -> int
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `s` | `str` | Yes | The s |
| `index` | `int` | Yes | The index |

**Returns:** `int`


---

### handle_visitor_element_start()

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

```python
def handle_visitor_element_start(visitor_handle: VisitorHandle, tag_name: str, node_handle: NodeHandle, tag: HtmlTag, parser: Parser, output: str, ctx: Context, depth: int, dom_ctx: DomContext) -> VisitAction
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `visitor_handle` | `VisitorHandle` | Yes | Reference to the visitor for callbacks |
| `tag_name` | `str` | Yes | The normalized tag name being processed |
| `node_handle` | `NodeHandle` | Yes | Handle to the DOM node |
| `tag` | `HtmlTag` | Yes | Reference to the tag object |
| `parser` | `Parser` | Yes | Reference to the tl parser |
| `output` | `str` | Yes | Mutable reference to output string |
| `ctx` | `Context` | Yes | The context |
| `depth` | `int` | Yes | Current tree depth |
| `dom_ctx` | `DomContext` | Yes | Reference to DOM context for tree navigation |

**Returns:** `VisitAction`


---

### handle_visitor_element_end()

Handles visitor callback for element end (after processing).

This function is called when exiting an element after its content has been processed.
The visitor can:
- Accept the output normally (Continue)
- Replace the output with custom content (Custom)
- Remove the output entirely (Skip)
- Signal an error (Error)

**Signature:**

```python
def handle_visitor_element_end(visitor_handle: VisitorHandle, tag_name: str, node_handle: NodeHandle, tag: HtmlTag, parser: Parser, output: str, element_output_start: int, ctx: Context, depth: int, dom_ctx: DomContext) -> None
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `visitor_handle` | `VisitorHandle` | Yes | Reference to the visitor for callbacks |
| `tag_name` | `str` | Yes | The normalized tag name that was processed |
| `node_handle` | `NodeHandle` | Yes | Handle to the DOM node |
| `tag` | `HtmlTag` | Yes | Reference to the tag object |
| `parser` | `Parser` | Yes | Reference to the tl parser |
| `output` | `str` | Yes | Mutable reference to output string |
| `element_output_start` | `int` | Yes | Byte position where this element's output started |
| `ctx` | `Context` | Yes | Reference to the conversion context |
| `depth` | `int` | Yes | Current tree depth |
| `dom_ctx` | `DomContext` | Yes | Reference to DOM context for tree navigation |

**Returns:** `None`


---

### escape()

Escape Markdown special characters in text.

**Returns:**

Escaped text

**Signature:**

```python
def escape(text: str, escape_misc: bool, escape_asterisks: bool, escape_underscores: bool, escape_ascii: bool) -> Str
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `text` | `str` | Yes | Text to escape |
| `escape_misc` | `bool` | Yes | Escape miscellaneous characters (`\` `&` `<` `` ` `` `[` `>` `~` `#` `=` `+` `\|` `-`) |
| `escape_asterisks` | `bool` | Yes | Escape asterisks (`*`) |
| `escape_underscores` | `bool` | Yes | Escape underscores (`_`) |
| `escape_ascii` | `bool` | Yes | Escape all ASCII punctuation (for `CommonMark` spec compliance) |

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

```python
def chomp(text: str) -> StrStrStr
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `text` | `str` | Yes | The text |

**Returns:** `StrStrStr`


---

### normalize_whitespace()

Normalize whitespace by collapsing consecutive spaces and tabs.

Multiple spaces and tabs are replaced with a single space.
Newlines are preserved.
Unicode spaces are normalized to ASCII spaces.

**Returns:**

Normalized text with collapsed spaces/tabs but preserved newlines

**Signature:**

```python
def normalize_whitespace(text: str) -> str
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `text` | `str` | Yes | The text to normalize |

**Returns:** `str`


---

### normalize_whitespace_cow()

Normalize whitespace in text, returning borrowed or owned result as needed.

This function optimizes memory by returning a borrowed reference when no normalization
is needed, and only allocating a new string when whitespace changes are necessary.

Multiple consecutive spaces, tabs, and Unicode space characters are replaced with
a single ASCII space. Newlines are preserved as-is.

**Returns:**

`Cow.Borrowed` if text is already normalized, or `Cow.Owned` with normalized text

**Signature:**

```python
def normalize_whitespace_cow(text: str) -> Str
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `text` | `str` | Yes | The text to normalize |

**Returns:** `Str`


---

### decode_html_entities()

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

```python
def decode_html_entities(text: str) -> str
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `text` | `str` | Yes | Text containing HTML entities |

**Returns:** `str`


---

### decode_html_entities_cow()

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

```python
def decode_html_entities_cow(text: str) -> Str
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `text` | `str` | Yes | Text potentially containing HTML entities |

**Returns:** `Str`


---

### underline()

Underline text with a character.

**Signature:**

```python
def underline(text: str, pad_char: str) -> str
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `text` | `str` | Yes | The text |
| `pad_char` | `str` | Yes | The pad char |

**Returns:** `str`


---

### indent()

Indent text with a string prefix.

**Signature:**

```python
def indent(text: str, level: int, indent_str: str) -> str
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `text` | `str` | Yes | The text |
| `level` | `int` | Yes | The level |
| `indent_str` | `str` | Yes | The indent str |

**Returns:** `str`


---

### build_document_structure()

Build a `DocumentStructure` from an already-parsed `tl.VDom`.

Walks the DOM once, mapping HTML elements to semantic `NodeContent` variants,
tracking parent/child relationships, extracting inline `TextAnnotation`s, and
constructing heading-based `Group` nodes.

**Signature:**

```python
def build_document_structure(dom: VDom) -> DocumentStructure
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `dom` | `VDom` | Yes | The v dom |

**Returns:** `DocumentStructure`


---

### build_node_context()

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

```python
def build_node_context(node_type: NodeType, tag_name: str, attributes: dict[str, str], depth: int, index_in_parent: int, parent_tag: str = None, is_inline: bool) -> NodeContext
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `node_type` | `NodeType` | Yes | Coarse-grained classification (Link, Image, Heading, etc.) |
| `tag_name` | `str` | Yes | Raw HTML tag name (e.g., "div", "h1", "custom-element") |
| `attributes` | `dict[str, str]` | Yes | All HTML attributes as key-value pairs |
| `depth` | `int` | Yes | Nesting depth in the DOM tree (0 = root) |
| `index_in_parent` | `int` | Yes | Zero-based index among siblings |
| `parent_tag` | `str | None` | No | Parent element's tag name (None if root) |
| `is_inline` | `bool` | Yes | Whether this element is treated as inline vs block |

**Returns:** `NodeContext`


---

### convert()

Convert HTML to Markdown, returning a `ConversionResult` with content, metadata, images,
and warnings.

**Errors:**

Returns an error if HTML parsing fails or if the input contains invalid UTF-8.

**Signature:**

```python
def convert(html: str, options: ConversionOptions = None) -> ConversionResult
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `html` | `str` | Yes | The HTML string to convert |
| `options` | `ConversionOptions | None` | No | Optional conversion options (defaults to `default options`) |

**Returns:** `ConversionResult`

**Errors:** Raises `Error`.


---

### convert_with_visitor()

Internal: convert with visitor support. Used by FFI crate.
Will be removed when convert() accepts visitor parameter directly.

**Signature:**

```python
def convert_with_visitor(html: str, options: ConversionOptions = None, visitor: VisitorHandle = None) -> str
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `html` | `str` | Yes | The html |
| `options` | `ConversionOptions | None` | No | The options to use |
| `visitor` | `VisitorHandle | None` | No | The visitor handle |

**Returns:** `str`

**Errors:** Raises `Error`.


---

### conversion_options_from_json()

Parse JSON string into `ConversionOptions`.

Deserializes a JSON string into a full set of conversion options.
The JSON can be either a complete or partial options object.

**Returns:**

Fully populated `ConversionOptions` with defaults applied to any unspecified values

**Errors:**

Returns `ConversionError.ConfigError` if JSON parsing fails or contains invalid option values

**Signature:**

```python
def conversion_options_from_json(json: str) -> ConversionOptions
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `json` | `str` | Yes | JSON string representing conversion options |

**Returns:** `ConversionOptions`

**Errors:** Raises `Error`.


---

### conversion_options_update_from_json()

Parse JSON string into partial `ConversionOptions` update.

Deserializes a JSON string into a partial set of conversion options.
Only specified options are included; unspecified options are None.

**Returns:**

`ConversionOptionsUpdate` with only specified fields populated

**Errors:**

Returns `ConversionError.ConfigError` if JSON parsing fails or contains invalid option values

**Signature:**

```python
def conversion_options_update_from_json(json: str) -> ConversionOptionsUpdate
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `json` | `str` | Yes | JSON string representing partial conversion options |

**Returns:** `ConversionOptionsUpdate`

**Errors:** Raises `Error`.


---

### inline_image_config_from_json()

Parse JSON string into `InlineImageConfig` (requires `inline-images` feature).

Deserializes a JSON string into inline image extraction configuration.
The JSON can be either a complete or partial configuration object.

**Returns:**

Fully populated `InlineImageConfig` with defaults applied to any unspecified values

**Errors:**

Returns `ConversionError.ConfigError` if JSON parsing fails or contains invalid configuration values

**Signature:**

```python
def inline_image_config_from_json(json: str) -> InlineImageConfig
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `json` | `str` | Yes | JSON string representing inline image configuration |

**Returns:** `InlineImageConfig`

**Errors:** Raises `Error`.


---

### metadata_config_from_json()

Parse JSON string into `MetadataConfig` (requires `metadata` feature).

Deserializes a JSON string into metadata extraction configuration.
The JSON can be either a complete or partial configuration object.

**Returns:**

Fully populated `MetadataConfig` with defaults applied to any unspecified values

**Errors:**

Returns `ConversionError.ConfigError` if JSON parsing fails or contains invalid configuration values

**Signature:**

```python
def metadata_config_from_json(json: str) -> MetadataConfig
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `json` | `str` | Yes | JSON string representing metadata extraction configuration |

**Returns:** `MetadataConfig`

**Errors:** Raises `Error`.


---

## Types

### ConversionOptions

Main conversion options for HTML to Markdown conversion.

Use `ConversionOptions.builder()` to construct, or `the default constructor` for defaults.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `heading_style` | `HeadingStyle` | `HeadingStyle.ATX` | Heading style to use in Markdown output (ATX `#` or Setext underline). |
| `list_indent_type` | `ListIndentType` | `ListIndentType.SPACES` | How to indent nested list items (spaces or tab). |
| `list_indent_width` | `int` | `2` | Number of spaces (or tabs) to use for each level of list indentation. |
| `bullets` | `str` | `"-*+"` | Bullet character(s) to use for unordered list items (e.g. `"-"`, `"*"`). |
| `strong_em_symbol` | `str` | `"*"` | Character used for bold/italic emphasis markers (`*` or `_`). |
| `escape_asterisks` | `bool` | `False` | Escape `*` characters in plain text to avoid unintended bold/italic. |
| `escape_underscores` | `bool` | `False` | Escape `_` characters in plain text to avoid unintended bold/italic. |
| `escape_misc` | `bool` | `False` | Escape miscellaneous Markdown metacharacters (`[]()#` etc.) in plain text. |
| `escape_ascii` | `bool` | `False` | Escape ASCII characters that have special meaning in certain Markdown dialects. |
| `code_language` | `str` | `""` | Default language annotation for fenced code blocks that have no language hint. |
| `autolinks` | `bool` | `True` | Automatically convert bare URLs into Markdown autolinks. |
| `default_title` | `bool` | `False` | Emit a default title when no `<title>` tag is present. |
| `br_in_tables` | `bool` | `False` | Render `<br>` elements inside table cells as literal line breaks. |
| `highlight_style` | `HighlightStyle` | `HighlightStyle.DOUBLE_EQUAL` | Style used for `<mark>` / highlighted text (e.g. `==text==`). |
| `extract_metadata` | `bool` | `True` | Extract `<meta>` and `<head>` information into the result metadata. |
| `whitespace_mode` | `WhitespaceMode` | `WhitespaceMode.NORMALIZED` | Controls how whitespace is normalised during conversion. |
| `strip_newlines` | `bool` | `False` | Strip all newlines from the output, producing a single-line result. |
| `wrap` | `bool` | `False` | Wrap long lines at `wrap_width` characters. |
| `wrap_width` | `int` | `80` | Maximum line width when `wrap` is enabled (default `80`). |
| `convert_as_inline` | `bool` | `False` | Treat the entire document as inline content (no block-level wrappers). |
| `sub_symbol` | `str` | `""` | Markdown notation for subscript text (e.g. `"~"`). |
| `sup_symbol` | `str` | `""` | Markdown notation for superscript text (e.g. `"^"`). |
| `newline_style` | `NewlineStyle` | `NewlineStyle.SPACES` | How to encode hard line breaks (`<br>`) in Markdown. |
| `code_block_style` | `CodeBlockStyle` | `CodeBlockStyle.BACKTICKS` | Style used for fenced code blocks (backticks or tilde). |
| `keep_inline_images_in` | `list[str]` | `[]` | HTML tag names whose `<img>` children are kept inline instead of block. |
| `preprocessing` | `PreprocessingOptions` | — | Pre-processing options applied to the HTML before conversion. |
| `encoding` | `str` | `"utf-8"` | Expected character encoding of the input HTML (default `"utf-8"`). |
| `debug` | `bool` | `False` | Emit debug information during conversion. |
| `strip_tags` | `list[str]` | `[]` | HTML tag names whose content is stripped from the output entirely. |
| `preserve_tags` | `list[str]` | `[]` | HTML tag names that are preserved verbatim in the output. |
| `skip_images` | `bool` | `False` | Skip conversion of `<img>` elements (omit images from output). |
| `link_style` | `LinkStyle` | `LinkStyle.INLINE` | Link rendering style (inline or reference). |
| `output_format` | `OutputFormat` | `OutputFormat.MARKDOWN` | Target output format (Markdown, plain text, etc.). |
| `include_document_structure` | `bool` | `False` | Include structured document tree in result. |
| `extract_images` | `bool` | `False` | Extract inline images from data URIs and SVGs. |
| `max_image_size` | `int` | `5242880` | Maximum decoded image size in bytes (default 5MB). |
| `capture_svg` | `bool` | `False` | Capture SVG elements as images. |
| `infer_dimensions` | `bool` | `True` | Infer image dimensions from data. |

#### Methods

##### default()

**Signature:**

```python
@staticmethod
def default() -> ConversionOptions
```

##### builder()

Create a new builder with default values.

**Signature:**

```python
@staticmethod
def builder() -> ConversionOptionsBuilder
```

##### apply_update()

Apply a partial update to these conversion options.

**Signature:**

```python
def apply_update(self, update: ConversionOptionsUpdate) -> None
```

##### from_update()

Create from a partial update, applying to defaults.

**Signature:**

```python
@staticmethod
def from_update(update: ConversionOptionsUpdate) -> ConversionOptions
```

##### from()

**Signature:**

```python
@staticmethod
def from(update: ConversionOptionsUpdate) -> ConversionOptions
```


---

### ConversionResult

The primary result of HTML conversion and extraction.

Contains the converted text output, optional structured document tree,
metadata, extracted tables, images, and processing warnings.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `content` | `str | None` | `None` | Converted text output (markdown, djot, or plain text). `None` when `output_format` is set to `OutputFormat.None`, indicating extraction-only mode. |
| `document` | `DocumentStructure | None` | `None` | Structured document tree with semantic elements. Populated when `include_document_structure` is `True` in options. |
| `metadata` | `HtmlMetadata` | — | Extracted HTML metadata (title, OG, links, images, structured data). |
| `tables` | `list[TableData]` | `[]` | Extracted tables with structured cell data and markdown representation. |
| `images` | `list[InlineImage]` | `[]` | Extracted inline images (data URIs and SVGs). Populated when `extract_images` is `True` in options. |
| `warnings` | `list[ProcessingWarning]` | `[]` | Non-fatal processing warnings. |


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

##### strip_tags()

Set the list of HTML tag names whose content is stripped from output.

**Signature:**

```python
def strip_tags(self, tags: list[str]) -> ConversionOptionsBuilder
```

##### preserve_tags()

Set the list of HTML tag names that are preserved verbatim in output.

**Signature:**

```python
def preserve_tags(self, tags: list[str]) -> ConversionOptionsBuilder
```

##### keep_inline_images_in()

Set the list of HTML tag names whose `<img>` children are kept inline.

**Signature:**

```python
def keep_inline_images_in(self, tags: list[str]) -> ConversionOptionsBuilder
```

##### preprocessing()

Set the pre-processing options applied to the HTML before conversion.

**Signature:**

```python
def preprocessing(self, preprocessing: PreprocessingOptions) -> ConversionOptionsBuilder
```

##### build()

Build the final `ConversionOptions`.

**Signature:**

```python
def build(self) -> ConversionOptions
```


---

### DocumentMetadata

Document-level metadata extracted from `<head>` and top-level elements.

Contains all metadata typically used by search engines, social media platforms,
and browsers for document indexing and presentation.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `title` | `str | None` | `None` | Document title from `<title>` tag |
| `description` | `str | None` | `None` | Document description from `<meta name="description">` tag |
| `keywords` | `list[str]` | `[]` | Document keywords from `<meta name="keywords">` tag, split on commas |
| `author` | `str | None` | `None` | Document author from `<meta name="author">` tag |
| `canonical_url` | `str | None` | `None` | Canonical URL from `<link rel="canonical">` tag |
| `base_href` | `str | None` | `None` | Base URL from `<base href="">` tag for resolving relative URLs |
| `language` | `str | None` | `None` | Document language from `lang` attribute |
| `text_direction` | `TextDirection | None` | `None` | Document text direction from `dir` attribute |
| `open_graph` | `dict[str, str]` | `{}` | Open Graph metadata (og:* properties) for social media Keys like "title", "description", "image", "url", etc. |
| `twitter_card` | `dict[str, str]` | `{}` | Twitter Card metadata (twitter:* properties) Keys like "card", "site", "creator", "title", "description", "image", etc. |
| `meta_tags` | `dict[str, str]` | `{}` | Additional meta tags not covered by specific fields Keys are meta name/property attributes, values are content |


---

### DocumentNode

A single node in the document tree.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `id` | `str` | — | Deterministic node identifier. |
| `content` | `NodeContent` | — | The semantic content of this node. |
| `parent` | `int | None` | `None` | Index of the parent node (None for root nodes). |
| `children` | `list[int]` | — | Indices of child nodes in reading order. |
| `annotations` | `list[TextAnnotation]` | — | Inline formatting annotations (bold, italic, links, etc.) with byte offsets into the text. |
| `attributes` | `dict[str, str] | None` | `None` | Format-specific attributes (e.g. class, id, data-* attributes). |


---

### DocumentStructure

A structured document tree representing the semantic content of an HTML document.

Uses a flat node array with index-based parent/child references for efficient traversal.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `nodes` | `list[DocumentNode]` | — | All nodes in document reading order. |
| `source_format` | `str | None` | `None` | The source format (always "html" for this library). |


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

```python
def emphasis(self, content: str) -> str
```

##### strong()

Render strong emphasis (strong, b elements)

**Signature:**

```python
def strong(self, content: str, symbol: str) -> str
```

##### strikethrough()

Render strikethrough (del, s elements)

**Signature:**

```python
def strikethrough(self, content: str) -> str
```

##### highlight()

Render highlight (mark element)

**Signature:**

```python
def highlight(self, content: str) -> str
```

##### inserted()

Render inserted text (ins element)

**Signature:**

```python
def inserted(self, content: str) -> str
```

##### subscript()

Render subscript (sub element)

**Signature:**

```python
def subscript(self, content: str, custom_symbol: str) -> str
```

##### superscript()

Render superscript (sup element)

**Signature:**

```python
def superscript(self, content: str, custom_symbol: str) -> str
```

##### span_with_attributes()

Render span with attributes (for Djot: [text]{.class})

**Signature:**

```python
def span_with_attributes(self, content: str, classes: list[str], id: str) -> str
```

##### div_with_attributes()

Render div with attributes (for Djot: .: class)

**Signature:**

```python
def div_with_attributes(self, content: str, classes: list[str]) -> str
```

##### is_djot()

Check if this is Djot format

**Signature:**

```python
def is_djot(self) -> bool
```


---

### GridCell

A single cell in a table grid.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `content` | `str` | — | The text content of the cell. |
| `row` | `int` | — | 0-indexed row position. |
| `col` | `int` | — | 0-indexed column position. |
| `row_span` | `int` | — | Number of rows this cell spans (default 1). |
| `col_span` | `int` | — | Number of columns this cell spans (default 1). |
| `is_header` | `bool` | — | Whether this is a header cell (`<th>`). |


---

### HeaderMetadata

Header element metadata with hierarchy tracking.

Captures heading elements (h1-h6) with their text content, identifiers,
and position in the document structure.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `level` | `int` | — | Header level: 1 (h1) through 6 (h6) |
| `text` | `str` | — | Normalized text content of the header |
| `id` | `str | None` | `None` | HTML id attribute if present |
| `depth` | `int` | — | Document tree depth at the header element |
| `html_offset` | `int` | — | Byte offset in original HTML document |

#### Methods

##### is_valid()

Validate that the header level is within valid range (1-6).

**Returns:**

`True` if level is 1-6, `False` otherwise.

**Signature:**

```python
def is_valid(self) -> bool
```


---

### HtmlMetadata

Comprehensive metadata extraction result from HTML document.

Contains all extracted metadata types in a single structure,
suitable for serialization and transmission across language boundaries.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `document` | `DocumentMetadata` | — | Document-level metadata (title, description, canonical, etc.) |
| `headers` | `list[HeaderMetadata]` | `[]` | Extracted header elements with hierarchy |
| `links` | `list[LinkMetadata]` | `[]` | Extracted hyperlinks with type classification |
| `images` | `list[ImageMetadata]` | `[]` | Extracted images with source and dimensions |
| `structured_data` | `list[StructuredData]` | `[]` | Extracted structured data blocks |


---

### ImageMetadata

Image metadata with source and dimensions.

Captures `<img>` elements and inline `<svg>` elements with metadata
for image analysis and optimization.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `src` | `str` | — | Image source (URL, data URI, or SVG content identifier) |
| `alt` | `str | None` | `None` | Alternative text from alt attribute (for accessibility) |
| `title` | `str | None` | `None` | Title attribute (often shown as tooltip) |
| `dimensions` | `U32U32 | None` | `None` | Image dimensions as (width, height) if available |
| `image_type` | `ImageType` | — | Image type classification |
| `attributes` | `dict[str, str]` | — | Additional HTML attributes |


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
| `keep_inline_images_in` | `list[str]` | `[]` | HTML elements where images should remain as markdown links (not converted to alt text) |

#### Methods

##### from_elements()

Create a new inline image configuration from a list of element names.

**Signature:**

```python
@staticmethod
def from_elements(elements: list[str]) -> InlineImageConfig
```

##### add_element()

Add an element name to the list of elements where images are kept inline.

**Signature:**

```python
def add_element(self, element: str) -> None
```

##### should_keep_images()

Check if a given element should keep images inline.

**Returns:**

`True` if the element is in the configured list, `False` otherwise

**Signature:**

```python
def should_keep_images(self, element: str) -> bool
```

##### default()

**Signature:**

```python
@staticmethod
def default() -> InlineImageConfig
```


---

### LinkMetadata

Hyperlink metadata with categorization and attributes.

Represents `<a>` elements with parsed href values, text content, and link type classification.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `href` | `str` | — | The href URL value |
| `text` | `str` | — | Link text content (normalized, concatenated if mixed with elements) |
| `title` | `str | None` | `None` | Optional title attribute (often shown as tooltip) |
| `link_type` | `LinkType` | — | Link type classification |
| `rel` | `list[str]` | — | Rel attribute values (e.g., "nofollow", "stylesheet", "canonical") |
| `attributes` | `dict[str, str]` | — | Additional HTML attributes |

#### Methods

##### classify_link()

Classify a link based on href value.

**Returns:**

Appropriate `LinkType` based on protocol and content.

**Signature:**

```python
@staticmethod
def classify_link(href: str) -> LinkType
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
| `extract_document` | `bool` | `True` | Extract document-level metadata (title, description, author, etc.). When enabled, collects metadata from `<head>` section including: - `<title>` element content - `<meta name="description">` and other standard meta tags - Open Graph (og:*) properties for social media optimization - Twitter Card (twitter:*) properties - Language and text direction attributes - Canonical URL and base href references |
| `extract_headers` | `bool` | `True` | Extract h1-h6 header elements and their hierarchy. When enabled, collects all heading elements with: - Header level (1-6) - Text content (normalized) - HTML id attribute if present - Document tree depth for hierarchy tracking - Byte offset in original HTML for positioning |
| `extract_links` | `bool` | `True` | Extract anchor (a) elements as links with type classification. When enabled, collects all hyperlinks with: - href attribute value - Link text content - Title attribute (tooltip text) - Automatic link type classification (anchor, internal, external, email, phone, other) - Rel attribute values - Additional custom attributes |
| `extract_images` | `bool` | `True` | Extract image elements and data URIs. When enabled, collects all image elements with: - Source URL or data URI - Alt text for accessibility - Title attribute - Dimensions (width, height) if available - Automatic image type classification (data URI, external, relative, inline SVG) - Additional custom attributes |
| `extract_structured_data` | `bool` | `True` | Extract structured data (JSON-LD, Microdata, RDFa). When enabled, collects machine-readable structured data including: - JSON-LD script blocks with schema detection - Microdata attributes (itemscope, itemtype, itemprop) - RDFa markup - Extracted schema type if detectable |
| `max_structured_data_size` | `int` | — | Maximum total size of structured data to collect (bytes). Prevents memory exhaustion attacks on malformed or adversarial documents containing excessively large structured data blocks. When the accumulated size of structured data exceeds this limit, further collection stops. Default: `1_000_000` bytes (1 MB) |

#### Methods

##### default()

Create default metadata configuration.

Defaults to extracting all metadata types with 1MB limit on structured data.

**Signature:**

```python
@staticmethod
def default() -> MetadataConfig
```

##### any_enabled()

Check if any metadata extraction is enabled.

Returns `True` if at least one extraction category is enabled, `False` if all are disabled.
This is useful for early exit optimization when the application doesn't need metadata.

**Returns:**

`True` if any of the extraction flags are enabled, `False` if all are disabled.

**Signature:**

```python
def any_enabled(self) -> bool
```

##### apply_update()

Apply a partial update to this metadata configuration.

Any specified fields in the update (Some values) will override the current values.
Unspecified fields (None) are left unchanged. This allows selective modification
of configuration without affecting unrelated settings.

**Signature:**

```python
def apply_update(self, update: MetadataConfigUpdate) -> None
```

##### from_update()

Create new metadata configuration from a partial update.

Creates a new `MetadataConfig` struct with defaults, then applies the update.
Fields not specified in the update (None) keep their default values.
This is a convenience method for constructing a configuration from a partial specification
without needing to explicitly call `.default()` first.

**Returns:**

New `MetadataConfig` with specified updates applied to defaults

**Signature:**

```python
@staticmethod
def from_update(update: MetadataConfigUpdate) -> MetadataConfig
```

##### from()

**Signature:**

```python
@staticmethod
def from(update: MetadataConfigUpdate) -> MetadataConfig
```


---

### PreprocessingOptions

HTML preprocessing options for document cleanup before conversion.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `enabled` | `bool` | `True` | Enable HTML preprocessing globally |
| `preset` | `PreprocessingPreset` | `PreprocessingPreset.STANDARD` | Preprocessing preset level (Minimal, Standard, Aggressive) |
| `remove_navigation` | `bool` | `True` | Remove navigation elements (nav, breadcrumbs, menus, sidebars) |
| `remove_forms` | `bool` | `True` | Remove form elements (forms, inputs, buttons, etc.) |

#### Methods

##### default()

**Signature:**

```python
@staticmethod
def default() -> PreprocessingOptions
```

##### apply_update()

Apply a partial update to these preprocessing options.

Any specified fields in the update will override the current values.
Unspecified fields (None) are left unchanged.

**Signature:**

```python
def apply_update(self, update: PreprocessingOptionsUpdate) -> None
```

##### from_update()

Create new preprocessing options from a partial update.

Creates a new `PreprocessingOptions` struct with defaults, then applies the update.
Fields not specified in the update keep their default values.

**Returns:**

New `PreprocessingOptions` with specified updates applied to defaults

**Signature:**

```python
@staticmethod
def from_update(update: PreprocessingOptionsUpdate) -> PreprocessingOptions
```

##### from()

**Signature:**

```python
@staticmethod
def from(update: PreprocessingOptionsUpdate) -> PreprocessingOptions
```


---

### ProcessingWarning

A non-fatal warning generated during HTML processing.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `message` | `str` | — | Human-readable warning message. |
| `kind` | `WarningKind` | — | The category of warning. |


---

### ReferenceCollector

Collects link/image references during conversion and produces a reference
definitions section at the end of the document.

#### Methods

##### get_or_insert()

Register a URL (and optional title) and return its 1-based reference number.

If the same URL+title pair was already registered, the existing number is returned.

**Signature:**

```python
def get_or_insert(self, url: str, title: str) -> int
```

##### finish()

Produce the reference definitions section.

Returns an empty string when no references were collected.

**Signature:**

```python
def finish(self) -> str
```


---

### ReferenceCollectorHandle

Shared handle for passing the collector through the conversion context.


---

### StructureCollector

Incremental builder for `DocumentStructure` during a single DOM walk.

#### Methods

##### push_heading()

Record a heading element.

Creates a `NodeContent.Group` (which owns all subsequent sibling content until a
heading of equal or higher rank closes it) followed by a `NodeContent.Heading` child.

Returns the index of the **heading** node (the group node is one before it).

**Signature:**

```python
def push_heading(self, level: int, text: str, id: str) -> int
```

##### push_paragraph()

Record a paragraph element.

Returns the node index.

**Signature:**

```python
def push_paragraph(self, text: str) -> int
```

##### push_list_start()

Open a list container.

Returns the node index; call `push_list_end` to close it.

**Signature:**

```python
def push_list_start(self, ordered: bool) -> int
```

##### push_list_end()

Close the innermost open list container.

**Signature:**

```python
def push_list_end(self) -> None
```

##### push_list_item()

Record a list item under the current open list.

If there is no open list, the item is parented under the current section/container.
Returns the node index.

**Signature:**

```python
def push_list_item(self, text: str) -> int
```

##### push_table()

Record a table.

Returns the node index.

**Signature:**

```python
def push_table(self, grid: TableGrid) -> int
```

##### push_image()

Record an image element.

Returns the node index.

**Signature:**

```python
def push_image(self, src: str, alt: str) -> int
```

##### push_code()

Record a code block.

Returns the node index.

**Signature:**

```python
def push_code(self, text: str, language: str) -> int
```

##### push_quote_start()

Open a blockquote container.

Returns the node index; call `push_quote_end` to close it.

**Signature:**

```python
def push_quote_start(self) -> int
```

##### push_quote_end()

Close the innermost open blockquote container.

**Signature:**

```python
def push_quote_end(self) -> None
```

##### push_raw_block()

Record a raw block (e.g. preserved `<script>` or `<style>` content).

Returns the node index.

**Signature:**

```python
def push_raw_block(self, format: str, content: str) -> int
```

##### finish()

Consume the collector and return the completed `DocumentStructure`.

**Signature:**

```python
def finish(self) -> DocumentStructure
```

##### default()

**Signature:**

```python
@staticmethod
def default() -> StructureCollector
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
| `data_type` | `StructuredDataType` | — | Type of structured data (JSON-LD, Microdata, RDFa) |
| `raw_json` | `str` | — | Raw JSON string (for JSON-LD) or serialized representation |
| `schema_type` | `str | None` | `None` | Schema type if detectable (e.g., "Article", "Event", "Product") |


---

### TableData

A top-level extracted table with both structured data and markdown representation.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `grid` | `TableGrid` | — | The structured table grid. |
| `markdown` | `str` | — | The markdown rendering of this table. |


---

### TableGrid

A structured table grid with cell-level data including spans.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `rows` | `int` | — | Number of rows. |
| `cols` | `int` | — | Number of columns. |
| `cells` | `list[GridCell]` | `[]` | All cells in the table (may be fewer than rows*cols due to spans). |


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
| `row_counts` | `list[int]` | `[]` | Number of cells in each row |
| `has_span` | `bool` | — | Whether any cells have colspan or rowspan attributes |
| `has_header` | `bool` | — | Whether the table has header cells (th elements or role="head") |
| `has_caption` | `bool` | — | Whether the table has a caption element |
| `nested_table_count` | `int` | — | Number of nested tables found inside this table |
| `link_count` | `int` | — | Count of anchor elements in the table |
| `has_text` | `bool` | — | Whether the table contains text content (not empty) |


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

### VisitorDispatch

Result of dispatching a visitor callback.

This enum represents the outcome of a visitor callback dispatch,
providing a more ergonomic interface for control flow than the
raw `VisitResult` type.

| Value | Description |
|-------|-------------|
| `CONTINUE` | Continue with default conversion behavior |
| `CUSTOM` | Replace default output with custom markdown — Fields: `0`: `str` |
| `SKIP` | Skip this element entirely (don't output anything) |
| `PRESERVE_HTML` | Preserve original HTML (don't convert to markdown) |


---

### NodeContent

The semantic content type of a document node.

Uses internally tagged representation (`"node_type": "heading"`) for JSON serialization.

| Value | Description |
|-------|-------------|
| `HEADING` | A heading element (h1-h6). — Fields: `level`: `int`, `text`: `str` |
| `PARAGRAPH` | A paragraph of text. — Fields: `text`: `str` |
| `LIST` | A list container (ordered or unordered). Children are `ListItem` nodes. — Fields: `ordered`: `bool` |
| `LIST_ITEM` | A single list item. — Fields: `text`: `str` |
| `TABLE` | A table with structured cell data. — Fields: `grid`: `TableGrid` |
| `IMAGE` | An image element. — Fields: `description`: `str`, `src`: `str`, `image_index`: `int` |
| `CODE` | A code block or inline code. — Fields: `text`: `str`, `language`: `str` |
| `QUOTE` | A block quote container. |
| `DEFINITION_LIST` | A definition list container. |
| `DEFINITION_ITEM` | A definition list entry with term and description. — Fields: `term`: `str`, `definition`: `str` |
| `RAW_BLOCK` | A raw block preserved as-is (e.g. `<script>`, `<style>` content). — Fields: `format`: `str`, `content`: `str` |
| `METADATA_BLOCK` | A block of key-value metadata pairs (from `<head>` meta tags). — Fields: `entries`: `list[StringString]` |
| `GROUP` | A section grouping container (auto-generated from heading hierarchy). — Fields: `label`: `str`, `heading_level`: `int`, `heading_text`: `str` |


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
| `LINK` | A hyperlink. — Fields: `url`: `str`, `title`: `str` |


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

## Errors

### ConversionError

Errors that can occur during HTML to Markdown conversion.

**Base class:** `ConversionError(Exception)`

| Exception | Description |
|-----------|-------------|
| `ParseError(ConversionError)` | HTML parsing error |
| `SanitizationError(ConversionError)` | HTML sanitization error |
| `ConfigError(ConversionError)` | Invalid configuration |
| `IoError(ConversionError)` | I/O error |
| `Panic(ConversionError)` | Internal error caught during conversion |
| `InvalidInput(ConversionError)` | Invalid input data |
| `Other(ConversionError)` | Generic conversion error |


---

