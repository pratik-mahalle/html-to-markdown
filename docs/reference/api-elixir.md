---
title: Elixir API Reference
description: API reference for the html_to_markdown Elixir package
---

# Elixir API Reference

**Package:** [`html_to_markdown`](https://hex.pm/packages/html_to_markdown) | **Version:** 2.26.0 | **Elixir:** 1.14+ | **OTP:** 25+

---

## Installation

Add to your `mix.exs`:

```elixir
def deps do
  [
    {:html_to_markdown, "~> 2.26"}
  ]
end
```

The package uses Rustler NIF bindings for native Rust integration.

---

## Functions

### `HtmlToMarkdown.convert/2`

Convert HTML to Markdown.

```elixir
@spec convert(String.t(), options_input()) :: {:ok, String.t()} | {:error, term()}
```

**Parameters:**

| Parameter | Type | Description |
|-----------|------|-------------|
| `html` | `String.t()` | The HTML string to convert |
| `options` | `Options.t() \| map() \| keyword() \| nil` | Optional conversion options |

**Returns:** `{:ok, markdown}` on success, `{:error, reason}` on failure.

**Example:**

```elixir
{:ok, markdown} = HtmlToMarkdown.convert("<h1>Hello</h1><p>World</p>")

# With options
{:ok, markdown} = HtmlToMarkdown.convert(html, %{heading_style: "atx"})

# With keyword list
{:ok, markdown} = HtmlToMarkdown.convert(html, heading_style: "atx", wrap: true)
```

---

### `HtmlToMarkdown.convert!/2`

Convert HTML to Markdown, raising on failure.

```elixir
@spec convert!(String.t(), options_input()) :: String.t()
```

**Raises:** `HtmlToMarkdown.Error` on failure.

**Example:**

```elixir
markdown = HtmlToMarkdown.convert!("<h1>Hello</h1>")
```

---

### `HtmlToMarkdown.convert_with_metadata/3`

Convert HTML to Markdown with metadata extraction.

```elixir
@spec convert_with_metadata(String.t(), options_input(), metadata_config_input()) ::
        {:ok, String.t(), map()} | {:error, term()}
```

**Parameters:**

| Parameter | Type | Description |
|-----------|------|-------------|
| `html` | `String.t()` | The HTML string to convert |
| `options` | `options_input()` | Optional conversion options |
| `metadata_config` | `MetadataConfig.t() \| map() \| keyword() \| nil` | Metadata config |

**Returns:** `{:ok, markdown, metadata}` on success.

**Example:**

```elixir
html = """
<html lang="en">
  <head><title>My Article</title></head>
  <body>
    <h1 id="intro">Introduction</h1>
    <p>Visit <a href="https://example.com">our site</a></p>
  </body>
</html>
"""

{:ok, markdown, metadata} = HtmlToMarkdown.convert_with_metadata(html)
metadata["document"]["title"]    # => "My Article"
metadata["document"]["language"] # => "en"
length(metadata["headers"])      # => 1
length(metadata["links"])        # => 1

# Selective extraction
config = %{extract_headers: true, extract_links: false, extract_images: false}
{:ok, markdown, metadata} = HtmlToMarkdown.convert_with_metadata(html, nil, config)
```

---

### `HtmlToMarkdown.convert_with_metadata!/3`

Bang variant that raises on failure.

```elixir
@spec convert_with_metadata!(String.t(), options_input(), metadata_config_input()) ::
        {String.t(), map()}
```

**Returns:** `{markdown, metadata}` tuple.

---

### `HtmlToMarkdown.convert_with_inline_images/3`

Convert HTML while extracting inline image assets.

```elixir
@spec convert_with_inline_images(String.t(), options_input(), inline_config_input()) ::
        {:ok, String.t(), [InlineImage.t()], [InlineImageWarning.t()]} | {:error, term()}
```

---

### `HtmlToMarkdown.options/1`

Create a reusable options handle (opaque reference).

```elixir
@spec options(options_input()) :: reference()
```

### `HtmlToMarkdown.convert_with_options/2`

Convert using a pre-compiled options handle.

```elixir
@spec convert_with_options(String.t(), reference()) :: {:ok, String.t()} | {:error, term()}
```

**Example:**

```elixir
handle = HtmlToMarkdown.options(%{heading_style: "atx", wrap: true})

Enum.map(html_documents, fn html ->
  {:ok, markdown} = HtmlToMarkdown.convert_with_options(html, handle)
  markdown
end)
```

---

## Types

### `HtmlToMarkdown.Options`

Options struct for conversion configuration.

```elixir
%HtmlToMarkdown.Options{
  heading_style: "atx",           # "underlined", "atx", "atx_closed"
  list_indent_type: "spaces",     # "spaces", "tabs"
  list_indent_width: 2,
  bullets: "-",
  code_block_style: "indented",   # "indented", "backticks", "tildes"
  wrap: false,
  wrap_width: 80,
  preserve_tags: [],
  strip_tags: [],
  skip_images: false,
  output_format: "markdown",      # "markdown", "djot"
  # ... and more
}
```

Options can also be passed as maps or keyword lists.

### `HtmlToMarkdown.MetadataConfig`

```elixir
%HtmlToMarkdown.MetadataConfig{
  extract_document: true,
  extract_headers: true,
  extract_links: true,
  extract_images: true,
  extract_structured_data: true,
  max_structured_data_size: 1_000_000,
}
```

---

## Error Handling

All non-bang functions return `{:ok, result}` or `{:error, reason}` tuples following Elixir conventions:

```elixir
case HtmlToMarkdown.convert(html) do
  {:ok, markdown} -> IO.puts(markdown)
  {:error, reason} -> IO.puts("Failed: #{inspect(reason)}")
end

# Or use pattern matching with `with`
with {:ok, markdown} <- HtmlToMarkdown.convert(html),
     {:ok, _} <- File.write("output.md", markdown) do
  :ok
end
```

---

## Rustler NIF Details

The Elixir binding uses Rustler to compile and load Rust code as a NIF (Native Implemented Function):

- NIF crate located at `crates/html-to-markdown-elixir/`
- Compiled automatically by Mix during `mix compile`
- NIF functions are safe (no panics propagate to BEAM)
- All errors returned as `{:error, reason}` tuples
- Thread-safe: NIFs run on BEAM scheduler dirty threads

---

## See Also

- [Configuration Reference](configuration.md) -- full options documentation
- [Types Reference](types.md) -- cross-language type definitions
