---
title: R API Reference
description: API reference for the htmltomarkdown R package
---

# R API Reference

**Package:** [`htmltomarkdown`](https://kreuzberg-dev.r-universe.dev/htmltomarkdown) | **Version:** 2.26.0 | **R:** 4.3+

---

## Installation

```r
install.packages("htmltomarkdown", repos = "https://kreuzberg-dev.r-universe.dev")
```

The package uses extendr to bind to the Rust core library.

---

## Functions

### `convert`

Convert HTML to Markdown.

```r
convert(html)
```

**Arguments:**

| Parameter | Type | Description |
|-----------|------|-------------|
| `html` | `character` | A character string of HTML content |

**Returns:** `character` -- the converted Markdown string.

**Example:**

```r
library(htmltomarkdown)

html <- "<h1>Hello</h1><p>World</p>"
markdown <- convert(html)
cat(markdown)
```

---

### `convert_with_options`

Convert HTML to Markdown with options provided as a named list.

```r
convert_with_options(html, options)
```

**Arguments:**

| Parameter | Type | Description |
|-----------|------|-------------|
| `html` | `character` | HTML content |
| `options` | `list` | Named list of conversion options |

**Returns:** `character` -- the converted Markdown string.

**Example:**

```r
options <- list(
  heading_style = "atx",
  wrap = TRUE,
  wrap_width = 80
)
markdown <- convert_with_options(html, options)
```

---

### `convert_with_metadata`

Convert HTML to Markdown and extract document metadata.

```r
convert_with_metadata(html, options = NULL, config = NULL)
```

**Arguments:**

| Parameter | Type | Description |
|-----------|------|-------------|
| `html` | `character` | HTML content |
| `options` | `list` or `NULL` | Optional conversion options |
| `config` | `list` or `NULL` | Optional metadata extraction configuration |

**Returns:** `list` with `markdown` and `metadata` elements.

**Example:**

```r
html <- '<html lang="en"><head><title>Article</title></head>
         <body><h1>Title</h1><a href="https://example.com">Link</a></body></html>'

result <- convert_with_metadata(html)
cat(result$markdown)
print(result$metadata$document$title)    # "Article"
print(length(result$metadata$headers))   # 1
print(length(result$metadata$links))     # 1

# Selective extraction
config <- list(
  extract_headers = TRUE,
  extract_links = TRUE,
  extract_images = FALSE
)
result <- convert_with_metadata(html, config = config)
```

---

### `convert_with_inline_images`

Convert HTML and extract inline images.

```r
convert_with_inline_images(html, options = NULL, config = NULL)
```

**Returns:** `list` with `markdown`, `images`, and `warnings` elements.

---

### `convert_with_visitor`

Convert HTML with a visitor object (reserved for future use).

```r
convert_with_visitor(html, visitor = NULL, options = NULL)
```

---

### `create_options_handle`

Create a reusable options handle for repeated conversions.

```r
create_options_handle(options)
```

### `convert_with_options_handle`

Convert using a pre-created options handle.

```r
convert_with_options_handle(html, handle)
```

**Example:**

```r
handle <- create_options_handle(list(heading_style = "atx"))

for (html in html_documents) {
  markdown <- convert_with_options_handle(html, handle)
}
```

---

### `version`

Get the version of the html-to-markdown Rust core.

```r
version()
```

---

## Options List

Options are passed as named R lists. All fields are optional.

```r
options <- list(
  heading_style = "atx",           # "underlined", "atx", "atx_closed"
  list_indent_type = "spaces",     # "spaces", "tabs"
  list_indent_width = 2,
  bullets = "-",
  code_block_style = "indented",   # "indented", "backticks", "tildes"
  whitespace_mode = "normalized",  # "normalized", "strict"
  wrap = FALSE,
  wrap_width = 80,
  newline_style = "spaces",        # "spaces", "backslash"
  preserve_tags = c(),
  strip_tags = c(),
  skip_images = FALSE,
  output_format = "markdown"       # "markdown", "djot"
)
```

See the [Configuration Reference](configuration.md) for detailed descriptions.

---

## Metadata Config List

```r
config <- list(
  extract_document = TRUE,
  extract_headers = TRUE,
  extract_links = TRUE,
  extract_images = TRUE,
  extract_structured_data = TRUE,
  max_structured_data_size = 1000000
)
```

---

## Extendr Details

The R binding uses extendr to compile Rust code into a shared library loaded by R:

- Rust source in `packages/r/src/rust/`
- Compiled during package installation
- R wrappers auto-generated in `R/extendr-wrappers.R`
- All errors surfaced as R conditions

---

## See Also

- [Configuration Reference](configuration.md) -- full options documentation
- [Types Reference](types.md) -- cross-language type definitions
