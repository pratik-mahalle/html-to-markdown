# Conversion with Options - R

Configure HTML to Markdown conversion with reusable options.

## Using conversion_options()

Build options with the helper function:

```r
library(htmltomarkdown)

opts <- conversion_options(
  heading_style = "atx",
  wrap = TRUE,
  wrap_width = 80L
)

markdown <- convert_with_options("<h1>Hello</h1><p>World</p>", opts)
cat(markdown)
```

## Pre-built Options Handle

Create a reusable options handle for repeated conversions:

```r
handle <- create_options_handle(conversion_options(wrap = TRUE, wrap_width = 40L))

# Reuse the handle across multiple conversions
md1 <- convert_with_options_handle("<p>First document</p>", handle)
md2 <- convert_with_options_handle("<p>Second document</p>", handle)
```

## Supported Options

The `conversion_options()` function supports:

- **Heading Style**: `heading_style` - `"atx"`, `"atx_closed"`, or `"underlined"`
- **List Formatting**: `list_indent_type` (`"spaces"` / `"tabs"`), `list_indent_width`
- **Output Format**: `newline_style`, `code_block_style`, `highlight_style`
- **Text Wrapping**: `wrap` (logical) and `wrap_width` (integer)
- **Tag Control**: `strip_tags`, `preserve_tags` (character vectors)
- **Preprocessing**: `preprocessing` - named list with `enabled`, `preset`, `remove_navigation`, `remove_forms`
- **Debug**: `debug` - enable verbose tracing from the Rust core
