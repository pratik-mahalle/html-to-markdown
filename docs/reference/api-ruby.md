---
title: "Ruby API Reference"
---

# Ruby API Reference <span class="version-badge">v3.2.0</span>

## Functions

### table_total_columns()

Calculate total columns in a table.

Scans all rows and cells to determine the maximum column count,
accounting for colspan values.

**Returns:**
Maximum column count (minimum 1, maximum MAX_TABLE_COLS)

**Signature:**

```ruby
def self.table_total_columns(node_handle, parser, dom_ctx)
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `node_handle` | `NodeHandle` | Yes | Handle to the table element |
| `parser` | `Parser` | Yes | HTML parser instance |
| `dom_ctx` | `DomContext` | Yes | DOM context for tag name resolution |

**Returns:** `Integer`


---

### handle_table()

Convert an entire table element to Markdown.

Main entry point for table conversion. Analyzes table structure to determine
if it should be rendered as a Markdown table or converted to list format.
Handles layout tables, blank tables, and tables with semantic meaning.
Integrates with visitor pattern for custom table handling.

**Signature:**

```ruby
def self.handle_table(node_handle, parser, output, options, ctx, dom_ctx, depth)
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `node_handle` | `NodeHandle` | Yes | Handle to the table element |
| `parser` | `Parser` | Yes | HTML parser instance |
| `output` | `String` | Yes | Mutable string to append table content |
| `options` | `ConversionOptions` | Yes | Conversion options |
| `ctx` | `Context` | Yes | Conversion context (visitor, etc) |
| `dom_ctx` | `DomContext` | Yes | DOM context |
| `depth` | `Integer` | Yes | Nesting depth |

**Returns:** `nil`


---

### handle_caption()

Handles caption elements within tables.

Extracts text content from the caption and formats it as italicized text
with escaped hyphens to prevent Markdown table separator interpretation.

**Signature:**

```ruby
def self.handle_caption(node_handle, parser, output, options, ctx, depth, dom_ctx)
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `node_handle` | `NodeHandle` | Yes | Handle to the caption element |
| `parser` | `Parser` | Yes | HTML parser instance |
| `output` | `String` | Yes | Output string to append caption text to |
| `options` | `ConversionOptions` | Yes | Conversion options |
| `ctx` | `Context` | Yes | Conversion context |
| `depth` | `Integer` | Yes | Current recursion depth |
| `dom_ctx` | `DomContext` | Yes | DOM context for tag name resolution |

**Returns:** `nil`


---

### get_colspan()

Get colspan attribute value from an element.

Reads the colspan attribute from a table cell, with bounds checking
to prevent memory exhaustion attacks.

**Returns:**
The colspan value (minimum 1, maximum MAX_TABLE_COLS)

**Signature:**

```ruby
def self.get_colspan(node_handle, parser)
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `node_handle` | `NodeHandle` | Yes | Handle to the cell element |
| `parser` | `Parser` | Yes | HTML parser instance |

**Returns:** `Integer`


---

### get_colspan_rowspan()

Get both colspan and rowspan in a single lookup.

More efficient than calling get_colspan and a separate rowspan lookup.

**Returns:**
A tuple of (colspan, rowspan), both minimum 1 and maximum MAX_TABLE_COLS

**Signature:**

```ruby
def self.get_colspan_rowspan(node_handle, parser)
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

```ruby
def self.collect_table_cells(node_handle, parser, dom_ctx, cells)
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `node_handle` | `NodeHandle` | Yes | Handle to the row element |
| `parser` | `Parser` | Yes | HTML parser instance |
| `dom_ctx` | `DomContext` | Yes | DOM context for tag name resolution |
| `cells` | `Array<NodeHandle>` | Yes | Mutable vector to populate with cell handles |

**Returns:** `nil`


---

### convert_table_cell()

Convert a table cell (td or th) to Markdown format.

Processes cell content and renders it with pipe delimiters for Markdown tables.
Handles colspan by adding extra pipes, and escapes pipes in cell content.

**Signature:**

```ruby
def self.convert_table_cell(node_handle, parser, output, options, ctx, tag_name, dom_ctx)
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `node_handle` | `NodeHandle` | Yes | Handle to the cell element |
| `parser` | `Parser` | Yes | HTML parser instance |
| `output` | `String` | Yes | Mutable string to append cell content |
| `options` | `ConversionOptions` | Yes | Conversion options (escape settings, br_in_tables) |
| `ctx` | `Context` | Yes | Conversion context (visitor, etc) |
| `tag_name` | `String` | Yes | Tag name (for consistency, not used) |
| `dom_ctx` | `DomContext` | Yes | DOM context for content extraction |

**Returns:** `nil`


---

### append_layout_row()

Append a layout table row as a list item.

For tables used for visual layout, converts rows to list items
instead of table format for better readability.

**Signature:**

```ruby
def self.append_layout_row(row_handle, parser, output, options, ctx, dom_ctx)
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `row_handle` | `NodeHandle` | Yes | Handle to the row element |
| `parser` | `Parser` | Yes | HTML parser instance |
| `output` | `String` | Yes | Mutable string to append content |
| `options` | `ConversionOptions` | Yes | Conversion options |
| `ctx` | `Context` | Yes | Conversion context |
| `dom_ctx` | `DomContext` | Yes | DOM context |

**Returns:** `nil`


---

### convert_table_row()

Convert a table row (tr) to Markdown format.

Processes all cells in a row, handling colspan and rowspan for proper
column alignment. Renders header separator row after the first row.
Integrates with visitor pattern for custom row handling.

**Signature:**

```ruby
def self.convert_table_row(node_handle, parser, output, options, ctx, row_index, has_span, rowspan_tracker, total_cols, header_cols, dom_ctx, depth, is_header)
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `node_handle` | `NodeHandle` | Yes | Handle to the row element |
| `parser` | `Parser` | Yes | HTML parser instance |
| `output` | `String` | Yes | Mutable string to append row content |
| `options` | `ConversionOptions` | Yes | Conversion options |
| `ctx` | `Context` | Yes | Conversion context (visitor, etc) |
| `row_index` | `Integer` | Yes | Index of this row in the table |
| `has_span` | `Boolean` | Yes | Whether table has colspan/rowspan |
| `rowspan_tracker` | `Array<Integer?>` | Yes | Mutable array tracking rowspan remainder for each column |
| `total_cols` | `Integer` | Yes | Total columns in the table |
| `header_cols` | `Integer` | Yes | Columns to render in separator row |
| `dom_ctx` | `DomContext` | Yes | DOM context |
| `depth` | `Integer` | Yes | Nesting depth |
| `is_header` | `Boolean` | Yes | Whether this is a header row |

**Returns:** `nil`


---

### scan_table()

Scan a table element for structural metadata.

Analyzes the table to determine characteristics that influence rendering:
- Whether to render as a Markdown table or layout table
- If spanning cells are present
- If the table has semantic meaning (headers, captions)

**Signature:**

```ruby
def self.scan_table(node_handle, parser, dom_ctx)
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

```ruby
def self.dispatch_table_handler(tag_name, node_handle, parser, output, options, ctx, depth, dom_ctx)
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `tag_name` | `String` | Yes | The tag name |
| `node_handle` | `NodeHandle` | Yes | The node handle |
| `parser` | `Parser` | Yes | The parser |
| `output` | `String` | Yes | The output destination |
| `options` | `ConversionOptions` | Yes | The options to use |
| `ctx` | `Context` | Yes | The context |
| `depth` | `Integer` | Yes | The depth |
| `dom_ctx` | `DomContext` | Yes | The dom context |

**Returns:** `Boolean`


---

### dispatch_block_handler()

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

```ruby
def self.dispatch_block_handler(tag_name, node_handle, parser, output, options, ctx, depth, dom_ctx)
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `tag_name` | `String` | Yes | The tag name |
| `node_handle` | `NodeHandle` | Yes | The node handle |
| `parser` | `Parser` | Yes | The parser |
| `output` | `String` | Yes | The output destination |
| `options` | `ConversionOptions` | Yes | The options to use |
| `ctx` | `Context` | Yes | The context |
| `depth` | `Integer` | Yes | The depth |
| `dom_ctx` | `DomContext` | Yes | The dom context |

**Returns:** `Boolean`


---

### handle()

Dispatcher for form elements.

Routes all form-related elements to their respective handlers.

**Signature:**

```ruby
def self.handle(tag_name, node_handle, parser, output, options, ctx, depth, dom_ctx)
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `tag_name` | `String` | Yes | The tag name |
| `node_handle` | `NodeHandle` | Yes | The node handle |
| `parser` | `Parser` | Yes | The parser |
| `output` | `String` | Yes | The output destination |
| `options` | `ConversionOptions` | Yes | The options to use |
| `ctx` | `Context` | Yes | The context |
| `depth` | `Integer` | Yes | The depth |
| `dom_ctx` | `DomContext` | Yes | The dom context |

**Returns:** `nil`


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

Returns `true` if the tag was successfully handled by a form handler,
`false` if the tag is not a form element and requires other handling.

**Signature:**

```ruby
def self.dispatch_form_handler(tag_name, node_handle, parser, output, options, ctx, depth, dom_ctx)
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `tag_name` | `String` | Yes | The tag name |
| `node_handle` | `NodeHandle` | Yes | The node handle |
| `parser` | `Parser` | Yes | The parser |
| `output` | `String` | Yes | The output destination |
| `options` | `ConversionOptions` | Yes | The options to use |
| `ctx` | `Context` | Yes | The context |
| `depth` | `Integer` | Yes | The depth |
| `dom_ctx` | `DomContext` | Yes | The dom context |

**Returns:** `Boolean`


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

```ruby
def self.handle_blockquote(node_handle, tag, parser, output, options, ctx, depth, dom_ctx)
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `node_handle` | `NodeHandle` | Yes | The node handle |
| `tag` | `HtmlTag` | Yes | The h t m l tag |
| `parser` | `Parser` | Yes | The parser |
| `output` | `String` | Yes | The output destination |
| `options` | `ConversionOptions` | Yes | The options to use |
| `ctx` | `Context` | Yes | The context |
| `depth` | `Integer` | Yes | The depth |
| `dom_ctx` | `DomContext` | Yes | The dom context |

**Returns:** `nil`


---

### handle_code()

Handle an inline `<code>` element and convert to Markdown.

This handler processes inline code elements including:
- Extracting code content and applying backtick delimiters
- Handling backticks in content by using multiple delimiters
- Invoking visitor callbacks when the visitor feature is enabled
- Generating appropriate markdown output with proper escaping

**Signature:**

```ruby
def self.handle_code(node_handle, tag, parser, output, options, ctx, depth, dom_ctx)
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `node_handle` | `NodeHandle` | Yes | The node handle |
| `tag` | `HtmlTag` | Yes | The h t m l tag |
| `parser` | `Parser` | Yes | The parser |
| `output` | `String` | Yes | The output destination |
| `options` | `ConversionOptions` | Yes | The options to use |
| `ctx` | `Context` | Yes | The context |
| `depth` | `Integer` | Yes | The depth |
| `dom_ctx` | `DomContext` | Yes | The dom context |

**Returns:** `nil`


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

```ruby
def self.handle_pre(node_handle, tag, parser, output, options, ctx, depth, dom_ctx)
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `node_handle` | `NodeHandle` | Yes | The node handle |
| `tag` | `HtmlTag` | Yes | The h t m l tag |
| `parser` | `Parser` | Yes | The parser |
| `output` | `String` | Yes | The output destination |
| `options` | `ConversionOptions` | Yes | The options to use |
| `ctx` | `Context` | Yes | The context |
| `depth` | `Integer` | Yes | The depth |
| `dom_ctx` | `DomContext` | Yes | The dom context |

**Returns:** `nil`


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

```ruby
def self.handle_graphic(node_handle, tag, parser, output, options, ctx, depth, dom_ctx)
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `node_handle` | `NodeHandle` | Yes | The node handle |
| `tag` | `HtmlTag` | Yes | The h t m l tag |
| `parser` | `Parser` | Yes | The parser |
| `output` | `String` | Yes | The output destination |
| `options` | `ConversionOptions` | Yes | The options to use |
| `ctx` | `Context` | Yes | The context |
| `depth` | `Integer` | Yes | The depth |
| `dom_ctx` | `DomContext` | Yes | The dom context |

**Returns:** `nil`


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

```ruby
def self.handle_img(node_handle, tag, parser, output, options, ctx, depth, dom_ctx)
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `node_handle` | `NodeHandle` | Yes | The node handle |
| `tag` | `HtmlTag` | Yes | The h t m l tag |
| `parser` | `Parser` | Yes | The parser |
| `output` | `String` | Yes | The output destination |
| `options` | `ConversionOptions` | Yes | The options to use |
| `ctx` | `Context` | Yes | The context |
| `depth` | `Integer` | Yes | The depth |
| `dom_ctx` | `DomContext` | Yes | The dom context |

**Returns:** `nil`


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

```ruby
def self.handle_link(node_handle, tag, parser, output, options, ctx, depth, dom_ctx)
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `node_handle` | `NodeHandle` | Yes | The node handle |
| `tag` | `HtmlTag` | Yes | The h t m l tag |
| `parser` | `Parser` | Yes | The parser |
| `output` | `String` | Yes | The output destination |
| `options` | `ConversionOptions` | Yes | The options to use |
| `ctx` | `Context` | Yes | The context |
| `depth` | `Integer` | Yes | The depth |
| `dom_ctx` | `DomContext` | Yes | The dom context |

**Returns:** `nil`


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

```ruby
def self.dispatch_inline_handler(tag_name, node_handle, parser, output, options, ctx, depth, dom_ctx)
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `tag_name` | `String` | Yes | The normalized HTML tag name (lowercase) |
| `node_handle` | `NodeHandle` | Yes | The DOM node handle from the parser |
| `parser` | `Parser` | Yes | Reference to the tl HTML parser |
| `output` | `String` | Yes | Output buffer to write converted content to |
| `options` | `ConversionOptions` | Yes | Conversion configuration options |
| `ctx` | `Context` | Yes | Processing context with state tracking |
| `depth` | `Integer` | Yes | Current DOM tree depth for recursion tracking |
| `dom_ctx` | `DomContext` | Yes | DOM context for accessing tree structure |

**Returns:** `Boolean`


---

### calculate_list_continuation_indent()

Calculate indentation level for list item continuations.

Returns the number of 4-space indent groups needed for list continuations.

List continuations (block elements inside list items) need special indentation:
- Base indentation: (depth - 1) groups (for the nesting level)
- Content indentation: depth groups (for the list item content)
- Combined formula: (2 * depth - 1) groups of 4 spaces each

**Signature:**

```ruby
def self.calculate_list_continuation_indent(depth)
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `depth` | `Integer` | Yes | The depth |

**Returns:** `Integer`


---

### is_loose_list()

Check if a list (ul or ol) is "loose".

A loose list is one where any list item contains block-level elements
like paragraphs (<p>). In loose lists, all items should have blank line
separation (ending with \n\n) regardless of their own content.

**Signature:**

```ruby
def self.is_loose_list(node_handle, parser, dom_ctx)
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `node_handle` | `NodeHandle` | Yes | The node handle |
| `parser` | `Parser` | Yes | The parser |
| `dom_ctx` | `DomContext` | Yes | The dom context |

**Returns:** `Boolean`


---

### add_list_continuation_indent()

Add list continuation indentation to output.

Used when block elements (like <p> or <div>) appear inside list items.
Adds appropriate line separation and indentation to continue the list item.

**Signature:**

```ruby
def self.add_list_continuation_indent(output, list_depth, blank_line, options)
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `output` | `String` | Yes | The output string to append to |
| `list_depth` | `Integer` | Yes | Current list nesting depth |
| `blank_line` | `Boolean` | Yes | If true, adds blank line separation (\n\n); if false, single newline (\n) |
| `options` | `ConversionOptions` | Yes | The options to use |

**Returns:** `nil`


---

### continuation_indent_string()

Calculate the indentation string for list continuations based on depth and options.

**Signature:**

```ruby
def self.continuation_indent_string(list_depth, options)
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `list_depth` | `Integer` | Yes | The list depth |
| `options` | `ConversionOptions` | Yes | The options to use |

**Returns:** `String?`


---

### add_list_leading_separator()

Add appropriate leading separator before a list.

Lists need different separators depending on context:
- In table cells: <br> tag if there's already content
- Outside lists: blank line (\n\n) if needed
- Inside list items: blank line before nested list

**Signature:**

```ruby
def self.add_list_leading_separator(output, ctx)
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `output` | `String` | Yes | The output destination |
| `ctx` | `Context` | Yes | The context |

**Returns:** `nil`


---

### add_nested_list_trailing_separator()

Add appropriate trailing separator after a nested list.

Nested lists inside list items need trailing newlines to separate
from following content. In loose lists, use blank line (\n\n). In tight lists, single newline (\n).

**Signature:**

```ruby
def self.add_nested_list_trailing_separator(output, ctx)
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `output` | `String` | Yes | The output destination |
| `ctx` | `Context` | Yes | The context |

**Returns:** `nil`


---

### calculate_list_nesting_depth()

Calculate the nesting depth for a list.

If we're in a list but NOT in a list item, this is incorrectly nested HTML
and we need to increment the depth. If in a list item, the depth was already
incremented by the <li> element.

**Signature:**

```ruby
def self.calculate_list_nesting_depth(ctx)
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `ctx` | `Context` | Yes | The context |

**Returns:** `Integer`


---

### is_list_item()

Check if a node is a list item element.

**Signature:**

```ruby
def self.is_list_item(node_handle, parser, dom_ctx)
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `node_handle` | `NodeHandle` | Yes | The node handle |
| `parser` | `Parser` | Yes | The parser |
| `dom_ctx` | `DomContext` | Yes | The dom context |

**Returns:** `Boolean`


---

### process_list_children()

Process a list's children, tracking which items had block elements.

This is used to determine proper spacing between list items.
Returns true if the last processed item had block children.

**Signature:**

```ruby
def self.process_list_children(node_handle, parser, output, options, ctx, depth, is_ordered, is_loose, nested_depth, start_counter, dom_ctx)
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `node_handle` | `NodeHandle` | Yes | The node handle |
| `parser` | `Parser` | Yes | The parser |
| `output` | `String` | Yes | The output destination |
| `options` | `ConversionOptions` | Yes | The options to use |
| `ctx` | `Context` | Yes | The context |
| `depth` | `Integer` | Yes | The depth |
| `is_ordered` | `Boolean` | Yes | The is ordered |
| `is_loose` | `Boolean` | Yes | The is loose |
| `nested_depth` | `Integer` | Yes | The nested depth |
| `start_counter` | `Integer` | Yes | The start counter |
| `dom_ctx` | `DomContext` | Yes | The dom context |

**Returns:** `nil`


---

### dispatch_list_handler()

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

```ruby
def self.dispatch_list_handler(tag_name, node_handle, tag, parser, output, options, ctx, depth, dom_ctx)
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `tag_name` | `String` | Yes | The tag name |
| `node_handle` | `NodeHandle` | Yes | The node handle |
| `tag` | `HtmlTag` | Yes | The h t m l tag |
| `parser` | `Parser` | Yes | The parser |
| `output` | `String` | Yes | The output destination |
| `options` | `ConversionOptions` | Yes | The options to use |
| `ctx` | `Context` | Yes | The context |
| `depth` | `Integer` | Yes | The depth |
| `dom_ctx` | `DomContext` | Yes | The dom context |

**Returns:** `Boolean`


---

### convert_html()

Converts HTML to Markdown using the provided conversion options.

This is the main entry point for HTML to Markdown conversion.

**Signature:**

```ruby
def self.convert_html(html, options)
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `html` | `String` | Yes | The html |
| `options` | `ConversionOptions` | Yes | The options to use |

**Returns:** `String`

**Errors:** Raises `Error`.


---

### convert_html_with_visitor()

Converts HTML to Markdown with a custom visitor for callbacks during traversal.

This variant allows passing a visitor that will receive callbacks for each node
during the tree walk, enabling custom processing or analysis.

**Signature:**

```ruby
def self.convert_html_with_visitor(html, options, visitor: nil)
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `html` | `String` | Yes | The html |
| `options` | `ConversionOptions` | Yes | The options to use |
| `visitor` | `VisitorHandle?` | No | The visitor handle |

**Returns:** `String`

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

Returns `true` if the tag was recognized and handled, `false` otherwise.

**Signature:**

```ruby
def self.dispatch_media_handler(tag_name, node_handle, parser, output, options, ctx, depth, dom_ctx)
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `tag_name` | `String` | Yes | The tag name |
| `node_handle` | `NodeHandle` | Yes | The node handle |
| `parser` | `Parser` | Yes | The parser |
| `output` | `String` | Yes | The output destination |
| `options` | `ConversionOptions` | Yes | The options to use |
| `ctx` | `Context` | Yes | The context |
| `depth` | `Integer` | Yes | The depth |
| `dom_ctx` | `DomContext` | Yes | The dom context |

**Returns:** `Boolean`


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

```ruby
def self.extract_plain_text(dom, parser, options)
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `dom` | `VDom` | Yes | The v dom |
| `parser` | `Parser` | Yes | The parser |
| `options` | `ConversionOptions` | Yes | The options to use |

**Returns:** `String`


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

```ruby
def self.handle_dfn(tag_name, node_handle, parser, output, options, ctx, depth, dom_ctx)
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `tag_name` | `String` | Yes | The  tag name |
| `node_handle` | `NodeHandle` | Yes | The node handle |
| `parser` | `Parser` | Yes | The parser |
| `output` | `String` | Yes | The output destination |
| `options` | `ConversionOptions` | Yes | The options to use |
| `ctx` | `Context` | Yes | The context |
| `depth` | `Integer` | Yes | The depth |
| `dom_ctx` | `DomContext` | Yes | The dom context |

**Returns:** `nil`


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

```ruby
def self.handle_abbr(tag_name, node_handle, parser, output, options, ctx, depth, dom_ctx)
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `tag_name` | `String` | Yes | The  tag name |
| `node_handle` | `NodeHandle` | Yes | The node handle |
| `parser` | `Parser` | Yes | The parser |
| `output` | `String` | Yes | The output destination |
| `options` | `ConversionOptions` | Yes | The options to use |
| `ctx` | `Context` | Yes | The context |
| `depth` | `Integer` | Yes | The depth |
| `dom_ctx` | `DomContext` | Yes | The dom context |

**Returns:** `nil`


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

```ruby
def self.handle_time_data(tag_name, node_handle, parser, output, options, ctx, depth, dom_ctx)
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `tag_name` | `String` | Yes | The  tag name |
| `node_handle` | `NodeHandle` | Yes | The node handle |
| `parser` | `Parser` | Yes | The parser |
| `output` | `String` | Yes | The output destination |
| `options` | `ConversionOptions` | Yes | The options to use |
| `ctx` | `Context` | Yes | The context |
| `depth` | `Integer` | Yes | The depth |
| `dom_ctx` | `DomContext` | Yes | The dom context |

**Returns:** `nil`


---

### handle_cite()

Handles the `<cite>` element.

A cite element marks the title of a cited work (book, article, website, etc.).
It is rendered as emphasized (italic) text in block mode, or as plain text in inline mode.

# Behavior

- **Block mode**: Content is wrapped with emphasis markers (default: `*`)
- **Inline mode**: Content is output as-is without formatting

**Signature:**

```ruby
def self.handle_cite(tag_name, node_handle, parser, output, options, ctx, depth, dom_ctx)
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `tag_name` | `String` | Yes | The  tag name |
| `node_handle` | `NodeHandle` | Yes | The node handle |
| `parser` | `Parser` | Yes | The parser |
| `output` | `String` | Yes | The output destination |
| `options` | `ConversionOptions` | Yes | The options to use |
| `ctx` | `Context` | Yes | The context |
| `depth` | `Integer` | Yes | The depth |
| `dom_ctx` | `DomContext` | Yes | The dom context |

**Returns:** `nil`


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

```ruby
def self.handle_q(tag_name, node_handle, parser, output, options, ctx, depth, dom_ctx)
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `tag_name` | `String` | Yes | The  tag name |
| `node_handle` | `NodeHandle` | Yes | The node handle |
| `parser` | `Parser` | Yes | The parser |
| `output` | `String` | Yes | The output destination |
| `options` | `ConversionOptions` | Yes | The options to use |
| `ctx` | `Context` | Yes | The context |
| `depth` | `Integer` | Yes | The depth |
| `dom_ctx` | `DomContext` | Yes | The dom context |

**Returns:** `nil`


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

```ruby
def self.handle_hgroup(tag_name, node_handle, parser, output, options, ctx, depth, dom_ctx)
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `tag_name` | `String` | Yes | The  tag name |
| `node_handle` | `NodeHandle` | Yes | The node handle |
| `parser` | `Parser` | Yes | The parser |
| `output` | `String` | Yes | The output destination |
| `options` | `ConversionOptions` | Yes | The options to use |
| `ctx` | `Context` | Yes | The context |
| `depth` | `Integer` | Yes | The depth |
| `dom_ctx` | `DomContext` | Yes | The dom context |

**Returns:** `nil`


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

```ruby
def self.handle_dl(tag_name, node_handle, parser, output, options, ctx, depth, dom_ctx)
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `tag_name` | `String` | Yes | The  tag name |
| `node_handle` | `NodeHandle` | Yes | The node handle |
| `parser` | `Parser` | Yes | The parser |
| `output` | `String` | Yes | The output destination |
| `options` | `ConversionOptions` | Yes | The options to use |
| `ctx` | `Context` | Yes | The context |
| `depth` | `Integer` | Yes | The depth |
| `dom_ctx` | `DomContext` | Yes | The dom context |

**Returns:** `nil`


---

### handle_dt()

Handles the `<dt>` element.

A dt element contains a term being defined. Terms are output on their own line,
with definitions following on subsequent lines.

# Behavior

- **Inline mode**: Content is output as-is
- **Block mode**: Content is followed by a newline

**Signature:**

```ruby
def self.handle_dt(tag_name, node_handle, parser, output, options, ctx, depth, dom_ctx)
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `tag_name` | `String` | Yes | The  tag name |
| `node_handle` | `NodeHandle` | Yes | The node handle |
| `parser` | `Parser` | Yes | The parser |
| `output` | `String` | Yes | The output destination |
| `options` | `ConversionOptions` | Yes | The options to use |
| `ctx` | `Context` | Yes | The context |
| `depth` | `Integer` | Yes | The depth |
| `dom_ctx` | `DomContext` | Yes | The dom context |

**Returns:** `nil`


---

### handle_dd()

Handles the `<dd>` element.

A dd element contains the definition for a term. It is output as a plain
block since standard Markdown and GFM do not support definition list syntax.

# Behavior

- **Inline mode**: Content is output as-is
- **Block mode**: Content is output as a block

**Signature:**

```ruby
def self.handle_dd(tag_name, node_handle, parser, output, options, ctx, depth, dom_ctx)
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `tag_name` | `String` | Yes | The  tag name |
| `node_handle` | `NodeHandle` | Yes | The node handle |
| `parser` | `Parser` | Yes | The parser |
| `output` | `String` | Yes | The output destination |
| `options` | `ConversionOptions` | Yes | The options to use |
| `ctx` | `Context` | Yes | The context |
| `depth` | `Integer` | Yes | The depth |
| `dom_ctx` | `DomContext` | Yes | The dom context |

**Returns:** `nil`


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

```ruby
def self.handle_menu(tag_name, node_handle, parser, output, options, ctx, depth, dom_ctx)
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `tag_name` | `String` | Yes | The  tag name |
| `node_handle` | `NodeHandle` | Yes | The node handle |
| `parser` | `Parser` | Yes | The parser |
| `output` | `String` | Yes | The output destination |
| `options` | `ConversionOptions` | Yes | The options to use |
| `ctx` | `Context` | Yes | The context |
| `depth` | `Integer` | Yes | The depth |
| `dom_ctx` | `DomContext` | Yes | The dom context |

**Returns:** `nil`


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

```ruby
def self.handle_figure(tag_name, node_handle, parser, output, options, ctx, depth, dom_ctx)
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `tag_name` | `String` | Yes | The  tag name |
| `node_handle` | `NodeHandle` | Yes | The node handle |
| `parser` | `Parser` | Yes | The parser |
| `output` | `String` | Yes | The output destination |
| `options` | `ConversionOptions` | Yes | The options to use |
| `ctx` | `Context` | Yes | The context |
| `depth` | `Integer` | Yes | The depth |
| `dom_ctx` | `DomContext` | Yes | The dom context |

**Returns:** `nil`


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

```ruby
def self.handle_figcaption(tag_name, node_handle, parser, output, options, ctx, depth, dom_ctx)
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `tag_name` | `String` | Yes | The  tag name |
| `node_handle` | `NodeHandle` | Yes | The node handle |
| `parser` | `Parser` | Yes | The parser |
| `output` | `String` | Yes | The output destination |
| `options` | `ConversionOptions` | Yes | The options to use |
| `ctx` | `Context` | Yes | The context |
| `depth` | `Integer` | Yes | The depth |
| `dom_ctx` | `DomContext` | Yes | The dom context |

**Returns:** `nil`


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

```ruby
def self.handle_details(tag_name, node_handle, parser, output, options, ctx, depth, dom_ctx)
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `tag_name` | `String` | Yes | The  tag name |
| `node_handle` | `NodeHandle` | Yes | The node handle |
| `parser` | `Parser` | Yes | The parser |
| `output` | `String` | Yes | The output destination |
| `options` | `ConversionOptions` | Yes | The options to use |
| `ctx` | `Context` | Yes | The context |
| `depth` | `Integer` | Yes | The depth |
| `dom_ctx` | `DomContext` | Yes | The dom context |

**Returns:** `nil`


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

```ruby
def self.handle_summary(tag_name, node_handle, parser, output, options, ctx, depth, dom_ctx)
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `tag_name` | `String` | Yes | The  tag name |
| `node_handle` | `NodeHandle` | Yes | The node handle |
| `parser` | `Parser` | Yes | The parser |
| `output` | `String` | Yes | The output destination |
| `options` | `ConversionOptions` | Yes | The options to use |
| `ctx` | `Context` | Yes | The context |
| `depth` | `Integer` | Yes | The depth |
| `dom_ctx` | `DomContext` | Yes | The dom context |

**Returns:** `nil`


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

```ruby
def self.handle_dialog(tag_name, node_handle, parser, output, options, ctx, depth, dom_ctx)
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `tag_name` | `String` | Yes | The  tag name |
| `node_handle` | `NodeHandle` | Yes | The node handle |
| `parser` | `Parser` | Yes | The parser |
| `output` | `String` | Yes | The output destination |
| `options` | `ConversionOptions` | Yes | The options to use |
| `ctx` | `Context` | Yes | The context |
| `depth` | `Integer` | Yes | The depth |
| `dom_ctx` | `DomContext` | Yes | The dom context |

**Returns:** `nil`


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

Returns `true` if the tag was successfully handled by a semantic handler,
`false` if the tag is not a semantic element and requires other handling.

**Signature:**

```ruby
def self.dispatch_semantic_handler(tag_name, node_handle, parser, output, options, ctx, depth, dom_ctx)
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `tag_name` | `String` | Yes | The tag name |
| `node_handle` | `NodeHandle` | Yes | The node handle |
| `parser` | `Parser` | Yes | The parser |
| `output` | `String` | Yes | The output destination |
| `options` | `ConversionOptions` | Yes | The options to use |
| `ctx` | `Context` | Yes | The context |
| `depth` | `Integer` | Yes | The depth |
| `dom_ctx` | `DomContext` | Yes | The dom context |

**Returns:** `Boolean`


---

### escape_link_label()

Escape special characters in link labels.

Markdown link labels can contain brackets, which need careful escaping to avoid
being interpreted as nested links. This function escapes unescaped closing brackets
that would break the link syntax.

**Signature:**

```ruby
def self.escape_link_label(text)
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `text` | `String` | Yes | The text |

**Returns:** `String`


---

### escape_malformed_angle_brackets()

Escape malformed angle brackets in markdown output.

Markdown uses `<...>` for automatic links. Angle brackets that don't form valid
link syntax should be escaped as `&lt;` to prevent parser confusion.

A valid tag must have:
- `<!` followed by `-` or alphabetic character (for comments/declarations)
- `</` followed by alphabetic character (for closing tags)
- `<?` (for processing instructions)
- `<` followed by alphabetic character (for opening tags)

**Signature:**

```ruby
def self.escape_malformed_angle_brackets(input)
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `input` | `String` | Yes | The input data |

**Returns:** `Str`


---

### trim_line_end_whitespace()

Remove trailing spaces/tabs from every line while preserving newlines.

**Signature:**

```ruby
def self.trim_line_end_whitespace(output)
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `output` | `String` | Yes | The output destination |

**Returns:** `nil`


---

### truncate_at_char_boundary()

Truncate a string at a valid UTF-8 boundary.

**Signature:**

```ruby
def self.truncate_at_char_boundary(value, max_len)
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `value` | `String` | Yes | The value |
| `max_len` | `Integer` | Yes | The max len |

**Returns:** `nil`


---

### normalize_heading_text()

Normalize heading text by replacing newlines and extra whitespace.

Heading text should be on a single line in Markdown. This function collapses
any newlines and multiple spaces into single spaces.

**Signature:**

```ruby
def self.normalize_heading_text(text)
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `text` | `String` | Yes | The text |

**Returns:** `Str`


---

### dedent_code_block()

Remove common leading whitespace from all lines in a code block.

This is useful when HTML authors indent `<pre>` content for readability,
so we can strip the shared indentation without touching meaningful spacing.

**Signature:**

```ruby
def self.dedent_code_block(content)
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `content` | `String` | Yes | The content to process |

**Returns:** `String`


---

### floor_char_boundary()

Returns the largest valid char boundary index at or before `index`.

If `index` is already a char boundary it is returned unchanged.
Otherwise it walks backwards to find one.  Returns 0 if no boundary
is found before `index`.

**Signature:**

```ruby
def self.floor_char_boundary(s, index)
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `s` | `String` | Yes | The s |
| `index` | `Integer` | Yes | The index |

**Returns:** `Integer`


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

```ruby
def self.handle_visitor_element_start(visitor_handle, tag_name, node_handle, tag, parser, output, ctx, depth, dom_ctx)
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `visitor_handle` | `VisitorHandle` | Yes | Reference to the visitor for callbacks |
| `tag_name` | `String` | Yes | The normalized tag name being processed |
| `node_handle` | `NodeHandle` | Yes | Handle to the DOM node |
| `tag` | `HtmlTag` | Yes | Reference to the tag object |
| `parser` | `Parser` | Yes | Reference to the tl parser |
| `output` | `String` | Yes | Mutable reference to output string |
| `ctx` | `Context` | Yes | The context |
| `depth` | `Integer` | Yes | Current tree depth |
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

```ruby
def self.handle_visitor_element_end(visitor_handle, tag_name, node_handle, tag, parser, output, element_output_start, ctx, depth, dom_ctx)
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `visitor_handle` | `VisitorHandle` | Yes | Reference to the visitor for callbacks |
| `tag_name` | `String` | Yes | The normalized tag name that was processed |
| `node_handle` | `NodeHandle` | Yes | Handle to the DOM node |
| `tag` | `HtmlTag` | Yes | Reference to the tag object |
| `parser` | `Parser` | Yes | Reference to the tl parser |
| `output` | `String` | Yes | Mutable reference to output string |
| `element_output_start` | `Integer` | Yes | Byte position where this element's output started |
| `ctx` | `Context` | Yes | Reference to the conversion context |
| `depth` | `Integer` | Yes | Current tree depth |
| `dom_ctx` | `DomContext` | Yes | Reference to DOM context for tree navigation |

**Returns:** `nil`


---

### escape()

Escape Markdown special characters in text.

**Returns:**

Escaped text

**Signature:**

```ruby
def self.escape(text, escape_misc, escape_asterisks, escape_underscores, escape_ascii)
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `text` | `String` | Yes | Text to escape |
| `escape_misc` | `Boolean` | Yes | Escape miscellaneous characters (`\` `&` `<` `` ` `` `[` `>` `~` `#` `=` `+` `\|` `-`) |
| `escape_asterisks` | `Boolean` | Yes | Escape asterisks (`*`) |
| `escape_underscores` | `Boolean` | Yes | Escape underscores (`_`) |
| `escape_ascii` | `Boolean` | Yes | Escape all ASCII punctuation (for `CommonMark` spec compliance) |

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

```ruby
def self.chomp(text)
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `text` | `String` | Yes | The text |

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

```ruby
def self.normalize_whitespace(text)
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `text` | `String` | Yes | The text to normalize |

**Returns:** `String`


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

```ruby
def self.normalize_whitespace_cow(text)
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `text` | `String` | Yes | The text to normalize |

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

```ruby
def self.decode_html_entities(text)
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `text` | `String` | Yes | Text containing HTML entities |

**Returns:** `String`


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

```ruby
def self.decode_html_entities_cow(text)
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

```ruby
def self.underline(text, pad_char)
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `text` | `String` | Yes | The text |
| `pad_char` | `String` | Yes | The pad char |

**Returns:** `String`


---

### indent()

Indent text with a string prefix.

**Signature:**

```ruby
def self.indent(text, level, indent_str)
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `text` | `String` | Yes | The text |
| `level` | `Integer` | Yes | The level |
| `indent_str` | `String` | Yes | The indent str |

**Returns:** `String`


---

### build_document_structure()

Build a `DocumentStructure` from an already-parsed `tl.VDom`.

Walks the DOM once, mapping HTML elements to semantic `NodeContent` variants,
tracking parent/child relationships, extracting inline `TextAnnotation`s, and
constructing heading-based `Group` nodes.

**Signature:**

```ruby
def self.build_document_structure(dom)
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

```ruby
def self.build_node_context(node_type, tag_name, attributes, depth, index_in_parent, parent_tag: nil, is_inline)
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `node_type` | `NodeType` | Yes | Coarse-grained classification (Link, Image, Heading, etc.) |
| `tag_name` | `String` | Yes | Raw HTML tag name (e.g., "div", "h1", "custom-element") |
| `attributes` | `Hash{String=>String}` | Yes | All HTML attributes as key-value pairs |
| `depth` | `Integer` | Yes | Nesting depth in the DOM tree (0 = root) |
| `index_in_parent` | `Integer` | Yes | Zero-based index among siblings |
| `parent_tag` | `String?` | No | Parent element's tag name (None if root) |
| `is_inline` | `Boolean` | Yes | Whether this element is treated as inline vs block |

**Returns:** `NodeContext`


---

### convert()

Convert HTML to Markdown, returning a `ConversionResult` with content, metadata, images,
and warnings.

**Errors:**

Returns an error if HTML parsing fails or if the input contains invalid UTF-8.

**Signature:**

```ruby
def self.convert(html, options: nil)
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `html` | `String` | Yes | The HTML string to convert |
| `options` | `ConversionOptions?` | No | Optional conversion options (defaults to `default options`) |

**Returns:** `ConversionResult`

**Errors:** Raises `Error`.


---

### convert_with_visitor()

Internal: convert with visitor support. Used by FFI crate.
Will be removed when convert() accepts visitor parameter directly.

**Signature:**

```ruby
def self.convert_with_visitor(html, options: nil, visitor: nil)
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `html` | `String` | Yes | The html |
| `options` | `ConversionOptions?` | No | The options to use |
| `visitor` | `VisitorHandle?` | No | The visitor handle |

**Returns:** `String`

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

```ruby
def self.conversion_options_from_json(json)
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `json` | `String` | Yes | JSON string representing conversion options |

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

```ruby
def self.conversion_options_update_from_json(json)
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `json` | `String` | Yes | JSON string representing partial conversion options |

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

```ruby
def self.inline_image_config_from_json(json)
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `json` | `String` | Yes | JSON string representing inline image configuration |

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

```ruby
def self.metadata_config_from_json(json)
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `json` | `String` | Yes | JSON string representing metadata extraction configuration |

**Returns:** `MetadataConfig`

**Errors:** Raises `Error`.


---

## Types

### ConversionOptions

Main conversion options for HTML to Markdown conversion.

Use `ConversionOptions.builder()` to construct, or `the default constructor` for defaults.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `heading_style` | `HeadingStyle` | `:atx` | Heading style to use in Markdown output (ATX `#` or Setext underline). |
| `list_indent_type` | `ListIndentType` | `:spaces` | How to indent nested list items (spaces or tab). |
| `list_indent_width` | `Integer` | `2` | Number of spaces (or tabs) to use for each level of list indentation. |
| `bullets` | `String` | `"-*+"` | Bullet character(s) to use for unordered list items (e.g. `"-"`, `"*"`). |
| `strong_em_symbol` | `String` | `"*"` | Character used for bold/italic emphasis markers (`*` or `_`). |
| `escape_asterisks` | `Boolean` | `false` | Escape `*` characters in plain text to avoid unintended bold/italic. |
| `escape_underscores` | `Boolean` | `false` | Escape `_` characters in plain text to avoid unintended bold/italic. |
| `escape_misc` | `Boolean` | `false` | Escape miscellaneous Markdown metacharacters (`[]()#` etc.) in plain text. |
| `escape_ascii` | `Boolean` | `false` | Escape ASCII characters that have special meaning in certain Markdown dialects. |
| `code_language` | `String` | `""` | Default language annotation for fenced code blocks that have no language hint. |
| `autolinks` | `Boolean` | `true` | Automatically convert bare URLs into Markdown autolinks. |
| `default_title` | `Boolean` | `false` | Emit a default title when no `<title>` tag is present. |
| `br_in_tables` | `Boolean` | `false` | Render `<br>` elements inside table cells as literal line breaks. |
| `highlight_style` | `HighlightStyle` | `:double_equal` | Style used for `<mark>` / highlighted text (e.g. `==text==`). |
| `extract_metadata` | `Boolean` | `true` | Extract `<meta>` and `<head>` information into the result metadata. |
| `whitespace_mode` | `WhitespaceMode` | `:normalized` | Controls how whitespace is normalised during conversion. |
| `strip_newlines` | `Boolean` | `false` | Strip all newlines from the output, producing a single-line result. |
| `wrap` | `Boolean` | `false` | Wrap long lines at `wrap_width` characters. |
| `wrap_width` | `Integer` | `80` | Maximum line width when `wrap` is enabled (default `80`). |
| `convert_as_inline` | `Boolean` | `false` | Treat the entire document as inline content (no block-level wrappers). |
| `sub_symbol` | `String` | `""` | Markdown notation for subscript text (e.g. `"~"`). |
| `sup_symbol` | `String` | `""` | Markdown notation for superscript text (e.g. `"^"`). |
| `newline_style` | `NewlineStyle` | `:spaces` | How to encode hard line breaks (`<br>`) in Markdown. |
| `code_block_style` | `CodeBlockStyle` | `:backticks` | Style used for fenced code blocks (backticks or tilde). |
| `keep_inline_images_in` | `Array<String>` | `[]` | HTML tag names whose `<img>` children are kept inline instead of block. |
| `preprocessing` | `PreprocessingOptions` | — | Pre-processing options applied to the HTML before conversion. |
| `encoding` | `String` | `"utf-8"` | Expected character encoding of the input HTML (default `"utf-8"`). |
| `debug` | `Boolean` | `false` | Emit debug information during conversion. |
| `strip_tags` | `Array<String>` | `[]` | HTML tag names whose content is stripped from the output entirely. |
| `preserve_tags` | `Array<String>` | `[]` | HTML tag names that are preserved verbatim in the output. |
| `skip_images` | `Boolean` | `false` | Skip conversion of `<img>` elements (omit images from output). |
| `link_style` | `LinkStyle` | `:inline` | Link rendering style (inline or reference). |
| `output_format` | `OutputFormat` | `:markdown` | Target output format (Markdown, plain text, etc.). |
| `include_document_structure` | `Boolean` | `false` | Include structured document tree in result. |
| `extract_images` | `Boolean` | `false` | Extract inline images from data URIs and SVGs. |
| `max_image_size` | `Integer` | `5242880` | Maximum decoded image size in bytes (default 5MB). |
| `capture_svg` | `Boolean` | `false` | Capture SVG elements as images. |
| `infer_dimensions` | `Boolean` | `true` | Infer image dimensions from data. |

#### Methods

##### default()

**Signature:**

```ruby
def self.default()
```

##### builder()

Create a new builder with default values.

**Signature:**

```ruby
def self.builder()
```

##### apply_update()

Apply a partial update to these conversion options.

**Signature:**

```ruby
def apply_update(update)
```

##### from_update()

Create from a partial update, applying to defaults.

**Signature:**

```ruby
def self.from_update(update)
```

##### from()

**Signature:**

```ruby
def self.from(update)
```


---

### ConversionResult

The primary result of HTML conversion and extraction.

Contains the converted text output, optional structured document tree,
metadata, extracted tables, images, and processing warnings.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `content` | `String?` | `nil` | Converted text output (markdown, djot, or plain text). `None` when `output_format` is set to `OutputFormat.None`, indicating extraction-only mode. |
| `document` | `DocumentStructure?` | `nil` | Structured document tree with semantic elements. Populated when `include_document_structure` is `True` in options. |
| `metadata` | `HtmlMetadata` | — | Extracted HTML metadata (title, OG, links, images, structured data). |
| `tables` | `Array<TableData>` | `[]` | Extracted tables with structured cell data and markdown representation. |
| `images` | `Array<InlineImage>` | `[]` | Extracted inline images (data URIs and SVGs). Populated when `extract_images` is `True` in options. |
| `warnings` | `Array<ProcessingWarning>` | `[]` | Non-fatal processing warnings. |


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

```ruby
def strip_tags(tags)
```

##### preserve_tags()

Set the list of HTML tag names that are preserved verbatim in output.

**Signature:**

```ruby
def preserve_tags(tags)
```

##### keep_inline_images_in()

Set the list of HTML tag names whose `<img>` children are kept inline.

**Signature:**

```ruby
def keep_inline_images_in(tags)
```

##### preprocessing()

Set the pre-processing options applied to the HTML before conversion.

**Signature:**

```ruby
def preprocessing(preprocessing)
```

##### build()

Build the final `ConversionOptions`.

**Signature:**

```ruby
def build()
```


---

### DjotRenderer

Renderer for Djot lightweight markup output.

#### Methods

##### emphasis()

**Signature:**

```ruby
def emphasis(content)
```

##### strong()

**Signature:**

```ruby
def strong(content, symbol)
```

##### strikethrough()

**Signature:**

```ruby
def strikethrough(content)
```

##### highlight()

**Signature:**

```ruby
def highlight(content)
```

##### inserted()

**Signature:**

```ruby
def inserted(content)
```

##### subscript()

**Signature:**

```ruby
def subscript(content, custom_symbol)
```

##### superscript()

**Signature:**

```ruby
def superscript(content, custom_symbol)
```

##### span_with_attributes()

**Signature:**

```ruby
def span_with_attributes(content, classes, id)
```

##### div_with_attributes()

**Signature:**

```ruby
def div_with_attributes(content, classes)
```

##### is_djot()

**Signature:**

```ruby
def is_djot()
```


---

### DocumentMetadata

Document-level metadata extracted from `<head>` and top-level elements.

Contains all metadata typically used by search engines, social media platforms,
and browsers for document indexing and presentation.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `title` | `String?` | `nil` | Document title from `<title>` tag |
| `description` | `String?` | `nil` | Document description from `<meta name="description">` tag |
| `keywords` | `Array<String>` | `[]` | Document keywords from `<meta name="keywords">` tag, split on commas |
| `author` | `String?` | `nil` | Document author from `<meta name="author">` tag |
| `canonical_url` | `String?` | `nil` | Canonical URL from `<link rel="canonical">` tag |
| `base_href` | `String?` | `nil` | Base URL from `<base href="">` tag for resolving relative URLs |
| `language` | `String?` | `nil` | Document language from `lang` attribute |
| `text_direction` | `TextDirection?` | `nil` | Document text direction from `dir` attribute |
| `open_graph` | `Hash{String=>String}` | `{}` | Open Graph metadata (og:* properties) for social media Keys like "title", "description", "image", "url", etc. |
| `twitter_card` | `Hash{String=>String}` | `{}` | Twitter Card metadata (twitter:* properties) Keys like "card", "site", "creator", "title", "description", "image", etc. |
| `meta_tags` | `Hash{String=>String}` | `{}` | Additional meta tags not covered by specific fields Keys are meta name/property attributes, values are content |


---

### DocumentNode

A single node in the document tree.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `id` | `String` | — | Deterministic node identifier. |
| `content` | `NodeContent` | — | The semantic content of this node. |
| `parent` | `Integer?` | `nil` | Index of the parent node (None for root nodes). |
| `children` | `Array<Integer>` | — | Indices of child nodes in reading order. |
| `annotations` | `Array<TextAnnotation>` | — | Inline formatting annotations (bold, italic, links, etc.) with byte offsets into the text. |
| `attributes` | `Hash{String=>String}?` | `nil` | Format-specific attributes (e.g. class, id, data-* attributes). |


---

### DocumentStructure

A structured document tree representing the semantic content of an HTML document.

Uses a flat node array with index-based parent/child references for efficient traversal.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `nodes` | `Array<DocumentNode>` | — | All nodes in document reading order. |
| `source_format` | `String?` | `nil` | The source format (always "html" for this library). |


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

```ruby
def emphasis(content)
```

##### strong()

Render strong emphasis (strong, b elements)

**Signature:**

```ruby
def strong(content, symbol)
```

##### strikethrough()

Render strikethrough (del, s elements)

**Signature:**

```ruby
def strikethrough(content)
```

##### highlight()

Render highlight (mark element)

**Signature:**

```ruby
def highlight(content)
```

##### inserted()

Render inserted text (ins element)

**Signature:**

```ruby
def inserted(content)
```

##### subscript()

Render subscript (sub element)

**Signature:**

```ruby
def subscript(content, custom_symbol)
```

##### superscript()

Render superscript (sup element)

**Signature:**

```ruby
def superscript(content, custom_symbol)
```

##### span_with_attributes()

Render span with attributes (for Djot: [text]{.class})

**Signature:**

```ruby
def span_with_attributes(content, classes, id)
```

##### div_with_attributes()

Render div with attributes (for Djot: .: class)

**Signature:**

```ruby
def div_with_attributes(content, classes)
```

##### is_djot()

Check if this is Djot format

**Signature:**

```ruby
def is_djot()
```


---

### GridCell

A single cell in a table grid.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `content` | `String` | — | The text content of the cell. |
| `row` | `Integer` | — | 0-indexed row position. |
| `col` | `Integer` | — | 0-indexed column position. |
| `row_span` | `Integer` | — | Number of rows this cell spans (default 1). |
| `col_span` | `Integer` | — | Number of columns this cell spans (default 1). |
| `is_header` | `Boolean` | — | Whether this is a header cell (`<th>`). |


---

### HeaderMetadata

Header element metadata with hierarchy tracking.

Captures heading elements (h1-h6) with their text content, identifiers,
and position in the document structure.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `level` | `Integer` | — | Header level: 1 (h1) through 6 (h6) |
| `text` | `String` | — | Normalized text content of the header |
| `id` | `String?` | `nil` | HTML id attribute if present |
| `depth` | `Integer` | — | Document tree depth at the header element |
| `html_offset` | `Integer` | — | Byte offset in original HTML document |

#### Methods

##### is_valid()

Validate that the header level is within valid range (1-6).

**Returns:**

`true` if level is 1-6, `false` otherwise.

**Signature:**

```ruby
def is_valid()
```


---

### HtmlMetadata

Comprehensive metadata extraction result from HTML document.

Contains all extracted metadata types in a single structure,
suitable for serialization and transmission across language boundaries.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `document` | `DocumentMetadata` | — | Document-level metadata (title, description, canonical, etc.) |
| `headers` | `Array<HeaderMetadata>` | `[]` | Extracted header elements with hierarchy |
| `links` | `Array<LinkMetadata>` | `[]` | Extracted hyperlinks with type classification |
| `images` | `Array<ImageMetadata>` | `[]` | Extracted images with source and dimensions |
| `structured_data` | `Array<StructuredData>` | `[]` | Extracted structured data blocks |


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

##### visit_element_start()

Called before entering any element.

This is the first callback invoked for every HTML element, allowing
visitors to implement generic element handling before tag-specific logic.

**Signature:**

```ruby
def visit_element_start(ctx)
```

##### visit_element_end()

Called after exiting any element.

Receives the default markdown output that would be generated.
Visitors can inspect or replace this output.

**Signature:**

```ruby
def visit_element_end(ctx, output)
```

##### visit_text()

Visit text nodes (most frequent callback - ~100+ per document).

**Signature:**

```ruby
def visit_text(ctx, text)
```

##### visit_link()

Visit anchor links `<a href="...">`.

**Signature:**

```ruby
def visit_link(ctx, href, text, title)
```

##### visit_image()

Visit images `<img src="...">`.

**Signature:**

```ruby
def visit_image(ctx, src, alt, title)
```

##### visit_heading()

Visit heading elements `<h1>` through `<h6>`.

**Signature:**

```ruby
def visit_heading(ctx, level, text, id)
```

##### visit_code_block()

Visit code blocks `<pre><code>`.

**Signature:**

```ruby
def visit_code_block(ctx, lang, code)
```

##### visit_code_inline()

Visit inline code `<code>`.

**Signature:**

```ruby
def visit_code_inline(ctx, code)
```

##### visit_list_item()

Visit list items `<li>`.

**Signature:**

```ruby
def visit_list_item(ctx, ordered, marker, text)
```

##### visit_list_start()

Called before processing a list `<ul>` or `<ol>`.

**Signature:**

```ruby
def visit_list_start(ctx, ordered)
```

##### visit_list_end()

Called after processing a list `</ul>` or `</ol>`.

**Signature:**

```ruby
def visit_list_end(ctx, ordered, output)
```

##### visit_table_start()

Called before processing a table `<table>`.

**Signature:**

```ruby
def visit_table_start(ctx)
```

##### visit_table_row()

Visit table rows `<tr>`.

**Signature:**

```ruby
def visit_table_row(ctx, cells, is_header)
```

##### visit_table_end()

Called after processing a table `</table>`.

**Signature:**

```ruby
def visit_table_end(ctx, output)
```

##### visit_blockquote()

Visit blockquote elements `<blockquote>`.

**Signature:**

```ruby
def visit_blockquote(ctx, content, depth)
```

##### visit_strong()

Visit strong/bold elements `<strong>`, `<b>`.

**Signature:**

```ruby
def visit_strong(ctx, text)
```

##### visit_emphasis()

Visit emphasis/italic elements `<em>`, `<i>`.

**Signature:**

```ruby
def visit_emphasis(ctx, text)
```

##### visit_strikethrough()

Visit strikethrough elements `<s>`, `<del>`, `<strike>`.

**Signature:**

```ruby
def visit_strikethrough(ctx, text)
```

##### visit_underline()

Visit underline elements `<u>`, `<ins>`.

**Signature:**

```ruby
def visit_underline(ctx, text)
```

##### visit_subscript()

Visit subscript elements `<sub>`.

**Signature:**

```ruby
def visit_subscript(ctx, text)
```

##### visit_superscript()

Visit superscript elements `<sup>`.

**Signature:**

```ruby
def visit_superscript(ctx, text)
```

##### visit_mark()

Visit mark/highlight elements `<mark>`.

**Signature:**

```ruby
def visit_mark(ctx, text)
```

##### visit_line_break()

Visit line break elements `<br>`.

**Signature:**

```ruby
def visit_line_break(ctx)
```

##### visit_horizontal_rule()

Visit horizontal rule elements `<hr>`.

**Signature:**

```ruby
def visit_horizontal_rule(ctx)
```

##### visit_custom_element()

Visit custom elements (web components) or unknown tags.

**Signature:**

```ruby
def visit_custom_element(ctx, tag_name, html)
```

##### visit_definition_list_start()

Visit definition list `<dl>`.

**Signature:**

```ruby
def visit_definition_list_start(ctx)
```

##### visit_definition_term()

Visit definition term `<dt>`.

**Signature:**

```ruby
def visit_definition_term(ctx, text)
```

##### visit_definition_description()

Visit definition description `<dd>`.

**Signature:**

```ruby
def visit_definition_description(ctx, text)
```

##### visit_definition_list_end()

Called after processing a definition list `</dl>`.

**Signature:**

```ruby
def visit_definition_list_end(ctx, output)
```

##### visit_form()

Visit form elements `<form>`.

**Signature:**

```ruby
def visit_form(ctx, action, method)
```

##### visit_input()

Visit input elements `<input>`.

**Signature:**

```ruby
def visit_input(ctx, input_type, name, value)
```

##### visit_button()

Visit button elements `<button>`.

**Signature:**

```ruby
def visit_button(ctx, text)
```

##### visit_audio()

Visit audio elements `<audio>`.

**Signature:**

```ruby
def visit_audio(ctx, src)
```

##### visit_video()

Visit video elements `<video>`.

**Signature:**

```ruby
def visit_video(ctx, src)
```

##### visit_iframe()

Visit iframe elements `<iframe>`.

**Signature:**

```ruby
def visit_iframe(ctx, src)
```

##### visit_details()

Visit details elements `<details>`.

**Signature:**

```ruby
def visit_details(ctx, open)
```

##### visit_summary()

Visit summary elements `<summary>`.

**Signature:**

```ruby
def visit_summary(ctx, text)
```

##### visit_figure_start()

Visit figure elements `<figure>`.

**Signature:**

```ruby
def visit_figure_start(ctx)
```

##### visit_figcaption()

Visit figcaption elements `<figcaption>`.

**Signature:**

```ruby
def visit_figcaption(ctx, text)
```

##### visit_figure_end()

Called after processing a figure `</figure>`.

**Signature:**

```ruby
def visit_figure_end(ctx, output)
```


---

### ImageMetadata

Image metadata with source and dimensions.

Captures `<img>` elements and inline `<svg>` elements with metadata
for image analysis and optimization.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `src` | `String` | — | Image source (URL, data URI, or SVG content identifier) |
| `alt` | `String?` | `nil` | Alternative text from alt attribute (for accessibility) |
| `title` | `String?` | `nil` | Title attribute (often shown as tooltip) |
| `dimensions` | `U32U32?` | `nil` | Image dimensions as (width, height) if available |
| `image_type` | `ImageType` | — | Image type classification |
| `attributes` | `Hash{String=>String}` | — | Additional HTML attributes |


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
| `keep_inline_images_in` | `Array<String>` | `[]` | HTML elements where images should remain as markdown links (not converted to alt text) |

#### Methods

##### from_elements()

Create a new inline image configuration from a list of element names.

**Signature:**

```ruby
def self.from_elements(elements)
```

##### add_element()

Add an element name to the list of elements where images are kept inline.

**Signature:**

```ruby
def add_element(element)
```

##### should_keep_images()

Check if a given element should keep images inline.

**Returns:**

`true` if the element is in the configured list, `false` otherwise

**Signature:**

```ruby
def should_keep_images(element)
```

##### default()

**Signature:**

```ruby
def self.default()
```


---

### LinkMetadata

Hyperlink metadata with categorization and attributes.

Represents `<a>` elements with parsed href values, text content, and link type classification.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `href` | `String` | — | The href URL value |
| `text` | `String` | — | Link text content (normalized, concatenated if mixed with elements) |
| `title` | `String?` | `nil` | Optional title attribute (often shown as tooltip) |
| `link_type` | `LinkType` | — | Link type classification |
| `rel` | `Array<String>` | — | Rel attribute values (e.g., "nofollow", "stylesheet", "canonical") |
| `attributes` | `Hash{String=>String}` | — | Additional HTML attributes |

#### Methods

##### classify_link()

Classify a link based on href value.

**Returns:**

Appropriate `LinkType` based on protocol and content.

**Signature:**

```ruby
def self.classify_link(href)
```


---

### MarkdownRenderer

Renderer for standard Markdown output.

#### Methods

##### emphasis()

**Signature:**

```ruby
def emphasis(content)
```

##### strong()

**Signature:**

```ruby
def strong(content, symbol)
```

##### strikethrough()

**Signature:**

```ruby
def strikethrough(content)
```

##### highlight()

**Signature:**

```ruby
def highlight(content)
```

##### inserted()

**Signature:**

```ruby
def inserted(content)
```

##### subscript()

**Signature:**

```ruby
def subscript(content, custom_symbol)
```

##### superscript()

**Signature:**

```ruby
def superscript(content, custom_symbol)
```

##### span_with_attributes()

**Signature:**

```ruby
def span_with_attributes(content, classes, id)
```

##### div_with_attributes()

**Signature:**

```ruby
def div_with_attributes(content, classes)
```

##### is_djot()

**Signature:**

```ruby
def is_djot()
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
| `extract_document` | `Boolean` | `true` | Extract document-level metadata (title, description, author, etc.). When enabled, collects metadata from `<head>` section including: - `<title>` element content - `<meta name="description">` and other standard meta tags - Open Graph (og:*) properties for social media optimization - Twitter Card (twitter:*) properties - Language and text direction attributes - Canonical URL and base href references |
| `extract_headers` | `Boolean` | `true` | Extract h1-h6 header elements and their hierarchy. When enabled, collects all heading elements with: - Header level (1-6) - Text content (normalized) - HTML id attribute if present - Document tree depth for hierarchy tracking - Byte offset in original HTML for positioning |
| `extract_links` | `Boolean` | `true` | Extract anchor (a) elements as links with type classification. When enabled, collects all hyperlinks with: - href attribute value - Link text content - Title attribute (tooltip text) - Automatic link type classification (anchor, internal, external, email, phone, other) - Rel attribute values - Additional custom attributes |
| `extract_images` | `Boolean` | `true` | Extract image elements and data URIs. When enabled, collects all image elements with: - Source URL or data URI - Alt text for accessibility - Title attribute - Dimensions (width, height) if available - Automatic image type classification (data URI, external, relative, inline SVG) - Additional custom attributes |
| `extract_structured_data` | `Boolean` | `true` | Extract structured data (JSON-LD, Microdata, RDFa). When enabled, collects machine-readable structured data including: - JSON-LD script blocks with schema detection - Microdata attributes (itemscope, itemtype, itemprop) - RDFa markup - Extracted schema type if detectable |
| `max_structured_data_size` | `Integer` | — | Maximum total size of structured data to collect (bytes). Prevents memory exhaustion attacks on malformed or adversarial documents containing excessively large structured data blocks. When the accumulated size of structured data exceeds this limit, further collection stops. Default: `1_000_000` bytes (1 MB) |

#### Methods

##### default()

Create default metadata configuration.

Defaults to extracting all metadata types with 1MB limit on structured data.

**Signature:**

```ruby
def self.default()
```

##### any_enabled()

Check if any metadata extraction is enabled.

Returns `true` if at least one extraction category is enabled, `false` if all are disabled.
This is useful for early exit optimization when the application doesn't need metadata.

**Returns:**

`true` if any of the extraction flags are enabled, `false` if all are disabled.

**Signature:**

```ruby
def any_enabled()
```

##### apply_update()

Apply a partial update to this metadata configuration.

Any specified fields in the update (Some values) will override the current values.
Unspecified fields (None) are left unchanged. This allows selective modification
of configuration without affecting unrelated settings.

**Signature:**

```ruby
def apply_update(update)
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

```ruby
def self.from_update(update)
```

##### from()

**Signature:**

```ruby
def self.from(update)
```


---

### NodeContext

Context information passed to all visitor methods.

Provides comprehensive metadata about the current node being visited,
including its type, attributes, position in the DOM tree, and parent context.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `node_type` | `NodeType` | — | Coarse-grained node type classification |
| `tag_name` | `String` | — | Raw HTML tag name (e.g., "div", "h1", "custom-element") |
| `attributes` | `Hash{String=>String}` | — | All HTML attributes as key-value pairs |
| `depth` | `Integer` | — | Depth in the DOM tree (0 = root) |
| `index_in_parent` | `Integer` | — | Index among siblings (0-based) |
| `parent_tag` | `String?` | `nil` | Parent element's tag name (None if root) |
| `is_inline` | `Boolean` | — | Whether this element is treated as inline vs block |


---

### PreprocessingOptions

HTML preprocessing options for document cleanup before conversion.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `enabled` | `Boolean` | `true` | Enable HTML preprocessing globally |
| `preset` | `PreprocessingPreset` | `:standard` | Preprocessing preset level (Minimal, Standard, Aggressive) |
| `remove_navigation` | `Boolean` | `true` | Remove navigation elements (nav, breadcrumbs, menus, sidebars) |
| `remove_forms` | `Boolean` | `true` | Remove form elements (forms, inputs, buttons, etc.) |

#### Methods

##### default()

**Signature:**

```ruby
def self.default()
```

##### apply_update()

Apply a partial update to these preprocessing options.

Any specified fields in the update will override the current values.
Unspecified fields (None) are left unchanged.

**Signature:**

```ruby
def apply_update(update)
```

##### from_update()

Create new preprocessing options from a partial update.

Creates a new `PreprocessingOptions` struct with defaults, then applies the update.
Fields not specified in the update keep their default values.

**Returns:**

New `PreprocessingOptions` with specified updates applied to defaults

**Signature:**

```ruby
def self.from_update(update)
```

##### from()

**Signature:**

```ruby
def self.from(update)
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

##### get_or_insert()

Register a URL (and optional title) and return its 1-based reference number.

If the same URL+title pair was already registered, the existing number is returned.

**Signature:**

```ruby
def get_or_insert(url, title)
```

##### finish()

Produce the reference definitions section.

Returns an empty string when no references were collected.

**Signature:**

```ruby
def finish()
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

```ruby
def push_heading(level, text, id)
```

##### push_paragraph()

Record a paragraph element.

Returns the node index.

**Signature:**

```ruby
def push_paragraph(text)
```

##### push_list_start()

Open a list container.

Returns the node index; call `push_list_end` to close it.

**Signature:**

```ruby
def push_list_start(ordered)
```

##### push_list_end()

Close the innermost open list container.

**Signature:**

```ruby
def push_list_end()
```

##### push_list_item()

Record a list item under the current open list.

If there is no open list, the item is parented under the current section/container.
Returns the node index.

**Signature:**

```ruby
def push_list_item(text)
```

##### push_table()

Record a table.

Returns the node index.

**Signature:**

```ruby
def push_table(grid)
```

##### push_image()

Record an image element.

Returns the node index.

**Signature:**

```ruby
def push_image(src, alt)
```

##### push_code()

Record a code block.

Returns the node index.

**Signature:**

```ruby
def push_code(text, language)
```

##### push_quote_start()

Open a blockquote container.

Returns the node index; call `push_quote_end` to close it.

**Signature:**

```ruby
def push_quote_start()
```

##### push_quote_end()

Close the innermost open blockquote container.

**Signature:**

```ruby
def push_quote_end()
```

##### push_raw_block()

Record a raw block (e.g. preserved `<script>` or `<style>` content).

Returns the node index.

**Signature:**

```ruby
def push_raw_block(format, content)
```

##### finish()

Consume the collector and return the completed `DocumentStructure`.

**Signature:**

```ruby
def finish()
```

##### default()

**Signature:**

```ruby
def self.default()
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
| `raw_json` | `String` | — | Raw JSON string (for JSON-LD) or serialized representation |
| `schema_type` | `String?` | `nil` | Schema type if detectable (e.g., "Article", "Event", "Product") |


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
| `rows` | `Integer` | — | Number of rows. |
| `cols` | `Integer` | — | Number of columns. |
| `cells` | `Array<GridCell>` | `[]` | All cells in the table (may be fewer than rows*cols due to spans). |


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
| `row_counts` | `Array<Integer>` | `[]` | Number of cells in each row |
| `has_span` | `Boolean` | — | Whether any cells have colspan or rowspan attributes |
| `has_header` | `Boolean` | — | Whether the table has header cells (th elements or role="head") |
| `has_caption` | `Boolean` | — | Whether the table has a caption element |
| `nested_table_count` | `Integer` | — | Number of nested tables found inside this table |
| `link_count` | `Integer` | — | Count of anchor elements in the table |
| `has_text` | `Boolean` | — | Whether the table contains text content (not empty) |


---

### TextAnnotation

An inline text annotation with byte-range offsets.

Annotations describe formatting (bold, italic, etc.) and links within a node's text content.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `start` | `Integer` | — | Start byte offset (inclusive) into the parent node's text. |
| `end` | `Integer` | — | End byte offset (exclusive) into the parent node's text. |
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
| `continue` | Continue with normal element processing |
| `skip` | Skip the element entirely (don't process children or call visit_element_end) |
| `custom` | Custom output was provided, skip normal processing |
| `error` | Error occurred during visitor callback |


---

### TextDirection

Text directionality of document content.

Corresponds to the HTML `dir` attribute and `bdi` element directionality.

| Value | Description |
|-------|-------------|
| `left_to_right` | Left-to-right text flow (default for Latin scripts) |
| `right_to_left` | Right-to-left text flow (Hebrew, Arabic, Urdu, etc.) |
| `auto` | Automatic directionality detection |


---

### LinkType

Link classification based on href value and document context.

Used to categorize links during extraction for filtering and analysis.

| Value | Description |
|-------|-------------|
| `anchor` | Anchor link within same document (href starts with #) |
| `internal` | Internal link within same domain |
| `external` | External link to different domain |
| `email` | Email link (mailto:) |
| `phone` | Phone link (tel:) |
| `other` | Other protocol or unclassifiable |


---

### ImageType

Image source classification for proper handling and processing.

Determines whether an image is embedded (data URI), inline SVG, external, or relative.

| Value | Description |
|-------|-------------|
| `data_uri` | Data URI embedded image (base64 or other encoding) |
| `inline_svg` | Inline SVG element |
| `external` | External image URL (http/https) |
| `relative` | Relative image path |


---

### StructuredDataType

Structured data format type.

Identifies the schema/format used for structured data markup.

| Value | Description |
|-------|-------------|
| `json_ld` | JSON-LD (JSON for Linking Data) script blocks |
| `microdata` | HTML5 Microdata attributes (itemscope, itemtype, itemprop) |
| `rdfa` | RDF in Attributes (RDFa) markup |


---

### PreprocessingPreset

HTML preprocessing aggressiveness level.

Controls the extent of cleanup performed before conversion. Higher levels remove more elements.

| Value | Description |
|-------|-------------|
| `minimal` | Minimal cleanup. Remove only essential noise (scripts, styles). |
| `standard` | Standard cleanup. Default. Removes navigation, forms, and other auxiliary content. |
| `aggressive` | Aggressive cleanup. Remove extensive non-content elements and structure. |


---

### HeadingStyle

Heading style options for Markdown output.

Controls how headings (h1-h6) are rendered in the output Markdown.

| Value | Description |
|-------|-------------|
| `underlined` | Underlined style (=== for h1, --- for h2). |
| `atx` | ATX style (# for h1, ## for h2, etc.). Default. |
| `atx_closed` | ATX closed style (# title #, with closing hashes). |


---

### ListIndentType

List indentation character type.

Controls whether list items are indented with spaces or tabs.

| Value | Description |
|-------|-------------|
| `spaces` | Use spaces for indentation. Default. Width controlled by `list_indent_width`. |
| `tabs` | Use tabs for indentation. |


---

### WhitespaceMode

Whitespace handling strategy during conversion.

Determines how sequences of whitespace characters (spaces, tabs, newlines) are processed.

| Value | Description |
|-------|-------------|
| `normalized` | Collapse multiple whitespace characters to single spaces. Default. Matches browser behavior. |
| `strict` | Preserve all whitespace exactly as it appears in the HTML. |


---

### NewlineStyle

Line break syntax in Markdown output.

Controls how soft line breaks (from `<br>` or line breaks in source) are rendered.

| Value | Description |
|-------|-------------|
| `spaces` | Two trailing spaces at end of line. Default. Standard Markdown syntax. |
| `backslash` | Backslash at end of line. Alternative Markdown syntax. |


---

### CodeBlockStyle

Code block fence style in Markdown output.

Determines how code blocks (`<pre><code>`) are rendered in Markdown.

| Value | Description |
|-------|-------------|
| `indented` | Indented code blocks (4 spaces). `CommonMark` standard. |
| `backticks` | Fenced code blocks with backticks (```). Default (GFM). Supports language hints. |
| `tildes` | Fenced code blocks with tildes (~~~). Supports language hints. |


---

### HighlightStyle

Highlight rendering style for `<mark>` elements.

Controls how highlighted text is rendered in Markdown output.

| Value | Description |
|-------|-------------|
| `double_equal` | Double equals syntax (==text==). Default. Pandoc-compatible. |
| `html` | Preserve as HTML (==text==). Original HTML tag. |
| `bold` | Render as bold (**text**). Uses strong emphasis. |
| `none` | Strip formatting, render as plain text. No markup. |


---

### LinkStyle

Link rendering style in Markdown output.

Controls whether links and images use inline `[text](url)` syntax or
reference-style `[text][1]` syntax with definitions collected at the end.

| Value | Description |
|-------|-------------|
| `inline` | Inline links: `[text](url)`. Default. |
| `reference` | Reference-style links: `[text][1]` with `[1]: url` at end of document. |


---

### OutputFormat

Output format for conversion.

Specifies the target markup language format for the conversion output.

| Value | Description |
|-------|-------------|
| `markdown` | Standard Markdown (CommonMark compatible). Default. |
| `djot` | Djot lightweight markup language. |
| `plain` | Plain text output (no markup, visible text only). |


---

### NodeContent

The semantic content type of a document node.

Uses internally tagged representation (`"node_type": "heading"`) for JSON serialization.

| Value | Description |
|-------|-------------|
| `heading` | A heading element (h1-h6). — Fields: `level`: `Integer`, `text`: `String` |
| `paragraph` | A paragraph of text. — Fields: `text`: `String` |
| `list` | A list container (ordered or unordered). Children are `ListItem` nodes. — Fields: `ordered`: `Boolean` |
| `list_item` | A single list item. — Fields: `text`: `String` |
| `table` | A table with structured cell data. — Fields: `grid`: `TableGrid` |
| `image` | An image element. — Fields: `description`: `String`, `src`: `String`, `image_index`: `Integer` |
| `code` | A code block or inline code. — Fields: `text`: `String`, `language`: `String` |
| `quote` | A block quote container. |
| `definition_list` | A definition list container. |
| `definition_item` | A definition list entry with term and description. — Fields: `term`: `String`, `definition`: `String` |
| `raw_block` | A raw block preserved as-is (e.g. `<script>`, `<style>` content). — Fields: `format`: `String`, `content`: `String` |
| `metadata_block` | A block of key-value metadata pairs (from `<head>` meta tags). — Fields: `entries`: `Array<StringString>` |
| `group` | A section grouping container (auto-generated from heading hierarchy). — Fields: `label`: `String`, `heading_level`: `Integer`, `heading_text`: `String` |


---

### AnnotationKind

The type of an inline text annotation.

Uses internally tagged representation (`"annotation_type": "bold"`) for JSON serialization.

| Value | Description |
|-------|-------------|
| `bold` | Bold / strong emphasis. |
| `italic` | Italic / emphasis. |
| `underline` | Underline. |
| `strikethrough` | Strikethrough / deleted text. |
| `code` | Inline code. |
| `subscript` | Subscript text. |
| `superscript` | Superscript text. |
| `highlight` | Highlighted / marked text. |
| `link` | A hyperlink. — Fields: `url`: `String`, `title`: `String` |


---

### WarningKind

Categories of processing warnings.

| Value | Description |
|-------|-------------|
| `image_extraction_failed` | An image could not be extracted (e.g. invalid data URI, unsupported format). |
| `encoding_fallback` | The input encoding was not recognized; fell back to UTF-8. |
| `truncated_input` | The input was truncated due to size limits. |
| `malformed_html` | The HTML was malformed but processing continued with best effort. |
| `sanitization_applied` | Sanitization was applied to remove potentially unsafe content. |


---

### NodeType

Node type enumeration covering all HTML element types.

This enum categorizes all HTML elements that the converter recognizes,
providing a coarse-grained classification for visitor dispatch.

| Value | Description |
|-------|-------------|
| `text` | Text node (most frequent - 100+ per document) |
| `element` | Generic element node |
| `heading` | Heading elements (h1-h6) |
| `paragraph` | Paragraph element |
| `div` | Generic div container |
| `blockquote` | Blockquote element |
| `pre` | Preformatted text block |
| `hr` | Horizontal rule |
| `list` | Ordered or unordered list (ul, ol) |
| `list_item` | List item (li) |
| `definition_list` | Definition list (dl) |
| `definition_term` | Definition term (dt) |
| `definition_description` | Definition description (dd) |
| `table` | Table element |
| `table_row` | Table row (tr) |
| `table_cell` | Table cell (td, th) |
| `table_header` | Table header cell (th) |
| `table_body` | Table body (tbody) |
| `table_head` | Table head (thead) |
| `table_foot` | Table foot (tfoot) |
| `link` | Anchor link (a) |
| `image` | Image (img) |
| `strong` | Strong/bold (strong, b) |
| `em` | Emphasis/italic (em, i) |
| `code` | Inline code (code) |
| `strikethrough` | Strikethrough (s, del, strike) |
| `underline` | Underline (u, ins) |
| `subscript` | Subscript (sub) |
| `superscript` | Superscript (sup) |
| `mark` | Mark/highlight (mark) |
| `small` | Small text (small) |
| `br` | Line break (br) |
| `span` | Span element |
| `article` | Article element |
| `section` | Section element |
| `nav` | Navigation element |
| `aside` | Aside element |
| `header` | Header element |
| `footer` | Footer element |
| `main` | Main element |
| `figure` | Figure element |
| `figcaption` | Figure caption |
| `time` | Time element |
| `details` | Details element |
| `summary` | Summary element |
| `form` | Form element |
| `input` | Input element |
| `select` | Select element |
| `option` | Option element |
| `button` | Button element |
| `textarea` | Textarea element |
| `label` | Label element |
| `fieldset` | Fieldset element |
| `legend` | Legend element |
| `audio` | Audio element |
| `video` | Video element |
| `picture` | Picture element |
| `source` | Source element |
| `iframe` | Iframe element |
| `svg` | SVG element |
| `canvas` | Canvas element |
| `ruby` | Ruby annotation |
| `rt` | Ruby text |
| `rp` | Ruby parenthesis |
| `abbr` | Abbreviation |
| `kbd` | Keyboard input |
| `samp` | Sample output |
| `var` | Variable |
| `cite` | Citation |
| `q` | Quote |
| `del` | Deleted text |
| `ins` | Inserted text |
| `data` | Data element |
| `meter` | Meter element |
| `progress` | Progress element |
| `output` | Output element |
| `template` | Template element |
| `slot` | Slot element |
| `html` | HTML root element |
| `head` | Head element |
| `body` | Body element |
| `title` | Title element |
| `meta` | Meta element |
| `link_tag` | Link element (not anchor) |
| `style` | Style element |
| `script` | Script element |
| `base` | Base element |
| `custom` | Custom element (web components) or unknown tag |


---

### VisitResult

Result of a visitor callback.

Allows visitors to control the conversion flow by either proceeding
with default behavior, providing custom output, skipping elements,
preserving HTML, or signaling errors.

| Value | Description |
|-------|-------------|
| `continue` | Continue with default conversion behavior |
| `custom` | Replace default output with custom markdown The visitor takes full responsibility for the markdown output of this node and its children. — Fields: `0`: `String` |
| `skip` | Skip this element entirely (don't output anything) The element and all its children are ignored in the output. |
| `preserve_html` | Preserve original HTML (don't convert to markdown) The element's raw HTML is included verbatim in the output. |
| `error` | Stop conversion with an error The conversion process halts and returns this error message. — Fields: `0`: `String` |


---

### VisitorDispatch

Result of dispatching a visitor callback.

This enum represents the outcome of a visitor callback dispatch,
providing a more ergonomic interface for control flow than the
raw `VisitResult` type.

| Value | Description |
|-------|-------------|
| `continue` | Continue with default conversion behavior |
| `custom` | Replace default output with custom markdown — Fields: `0`: `String` |
| `skip` | Skip this element entirely (don't output anything) |
| `preserve_html` | Preserve original HTML (don't convert to markdown) |


---

## Errors

### ConversionError

Errors that can occur during HTML to Markdown conversion.

| Variant | Description |
|---------|-------------|
| `parse_error` | HTML parsing error |
| `sanitization_error` | HTML sanitization error |
| `config_error` | Invalid configuration |
| `io_error` | I/O error |
| `panic` | Internal error caught during conversion |
| `invalid_input` | Invalid input data |
| `other` | Generic conversion error |


---

