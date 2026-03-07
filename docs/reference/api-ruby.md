---
title: Ruby API Reference
description: API reference for the html-to-markdown Ruby gem
---

# Ruby API Reference <span class="version-badge">v2.5.1</span>

**Gem:** [`html-to-markdown`](https://rubygems.org/gems/html-to-markdown) | **Version:** 2.28.1 | **Ruby:** 3.2+

---

## Installation

```bash
gem install html-to-markdown
```

Or in your `Gemfile`:

```ruby
gem 'html-to-markdown'
```

---

## Module Methods

All methods are available on the `HtmlToMarkdown` module.

### `HtmlToMarkdown.convert`

Convert HTML to Markdown.

```ruby
HtmlToMarkdown.convert(html, options = nil, visitor = nil) -> String
```

**Parameters:**

| Parameter | Type | Description |
|-----------|------|-------------|
| `html` | `String` | The HTML string to convert |
| `options` | `Hash, nil` | Optional conversion options as a Hash |
| `visitor` | `Object, nil` | Optional visitor for custom conversion logic |

**Returns:** `String` -- the converted Markdown.

**Raises:** `StandardError` if conversion fails.

**Example:**

```ruby
require 'html_to_markdown'

html = "<h1>Hello</h1><p>World</p>"
markdown = HtmlToMarkdown.convert(html)

# With options
markdown = HtmlToMarkdown.convert(html, { heading_style: "atx" })

# With visitor
class SkipImages
  def visit_image(ctx, src, alt, title)
    { "type" => "skip" }
  end
end

markdown = HtmlToMarkdown.convert(html, nil, SkipImages.new)
```

---

### `HtmlToMarkdown.convert_with_metadata`

Convert HTML to Markdown with metadata extraction.

```ruby
HtmlToMarkdown.convert_with_metadata(html, options = nil, metadata_config = nil) -> Array
```

**Parameters:**

| Parameter | Type | Description |
|-----------|------|-------------|
| `html` | `String` | The HTML string to convert |
| `options` | `Hash, nil` | Optional conversion options |
| `metadata_config` | `Hash, nil` | Metadata extraction configuration |

**Returns:** `Array(String, Hash)` -- a two-element array of `[markdown_string, metadata_hash]`.

**Example:**

```ruby
html = <<~HTML
  <html lang="en">
    <head><title>My Article</title></head>
    <body>
      <h1 id="intro">Introduction</h1>
      <p>Visit <a href="https://example.com">our site</a></p>
      <img src="photo.jpg" alt="Landscape">
    </body>
  </html>
HTML

markdown, metadata = HtmlToMarkdown.convert_with_metadata(html)

puts metadata[:document][:title]      # => "My Article"
puts metadata[:document][:language]   # => "en"
puts metadata[:headers].length        # => 1
puts metadata[:headers][0][:text]     # => "Introduction"
puts metadata[:links].length          # => 1
puts metadata[:images].length         # => 1

# Selective extraction
config = {
  extract_headers: true,
  extract_links: true,
  extract_images: false,
  extract_structured_data: false,
}
markdown, metadata = HtmlToMarkdown.convert_with_metadata(html, nil, config)
```

---

### `HtmlToMarkdown.convert_with_visitor`

Convert HTML with a visitor object. The visitor is passed as the third argument to `convert`.

```ruby
# Preferred: use convert() with visitor parameter
HtmlToMarkdown.convert(html, options, visitor)
```

---

### `HtmlToMarkdown.options`

Create a reusable, pre-compiled options handle.

```ruby
HtmlToMarkdown.options(options_hash = nil) -> Object
```

### `HtmlToMarkdown.convert_with_options`

Convert using a pre-compiled options handle.

```ruby
HtmlToMarkdown.convert_with_options(html, options_handle) -> String
```

**Example:**

```ruby
handle = HtmlToMarkdown.options({ heading_style: "atx", wrap: true })

html_documents.each do |html|
  markdown = HtmlToMarkdown.convert_with_options(html, handle)
end
```

---

## Options Hash

Options are passed as a Ruby Hash with symbol keys. All fields are optional.

```ruby
options = {
  heading_style: "atx",           # "underlined", "atx", "atx_closed"
  list_indent_type: "spaces",     # "spaces", "tabs"
  list_indent_width: 4,
  bullets: "-",
  strong_em_symbol: "*",
  escape_asterisks: false,
  escape_underscores: false,
  escape_misc: false,
  code_language: "",
  autolinks: true,
  whitespace_mode: "normalized",  # "normalized", "strict"
  strip_newlines: false,
  wrap: false,
  wrap_width: 80,
  newline_style: "spaces",        # "spaces", "backslash"
  code_block_style: "indented",   # "indented", "backticks", "tildes"
  preserve_tags: [],
  strip_tags: [],
  skip_images: false,
  output_format: "markdown",      # "markdown", "djot", "plain"
}
```

See the [Configuration Reference](configuration.md) for detailed descriptions.

---

## Metadata Config Hash

```ruby
metadata_config = {
  extract_document: true,
  extract_headers: true,
  extract_links: true,
  extract_images: true,
  extract_structured_data: true,
  max_structured_data_size: 1_000_000,
}
```

---

## Metadata Result Structure

The metadata Hash returned by `convert_with_metadata` has the following structure:

```ruby
{
  document: {
    title: String | nil,
    description: String | nil,
    keywords: Array<String>,
    author: String | nil,
    language: String | nil,
    text_direction: String | nil,    # "ltr", "rtl", "auto"
    canonical_url: String | nil,
    open_graph: Hash<String, String>,
    twitter_card: Hash<String, String>,
    meta_tags: Hash<String, String>,
  },
  headers: [
    { level: Integer, text: String, id: String | nil, depth: Integer, html_offset: Integer },
  ],
  links: [
    { href: String, text: String, title: String | nil, link_type: String, rel: Array<String>, attributes: Hash },
  ],
  images: [
    { src: String, alt: String | nil, title: String | nil, dimensions: Array | nil, image_type: String, attributes: Hash },
  ],
  structured_data: [
    { data_type: String, raw_json: String, schema_type: String | nil },
  ],
}
```

---

## Visitor Protocol

Visitor objects are plain Ruby objects. Define methods matching the callbacks you need. Each method should return a Hash with a `"type"` key.

```ruby
class MyVisitor
  def visit_link(ctx, href, text, title)
    { "type" => "custom", "output" => "#{text} (#{href})" }
  end

  def visit_image(ctx, src, alt, title)
    { "type" => "skip" }
  end
end
```

See the [Visitor Pattern Guide](../guides/visitor.md) for available callbacks.

---

## See Also

- [Configuration Reference](configuration.md) -- full options documentation
- [Types Reference](types.md) -- cross-language type definitions
