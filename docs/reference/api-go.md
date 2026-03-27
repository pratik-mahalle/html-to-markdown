---
title: Go API Reference
description: API reference for the htmltomarkdown Go package
---

# Go API Reference <span class="version-badge">v2.8.0</span>

**Package:** [`htmltomarkdown`](https://pkg.go.dev/github.com/kreuzberg-dev/html-to-markdown/packages/go/v2/htmltomarkdown) | **Version:** 2.28.1 | **Go:** 1.21+

---

## Installation

```bash
go get github.com/kreuzberg-dev/html-to-markdown/packages/go/v2/htmltomarkdown
```

The package uses CGO to call the C FFI interface exposed by the Rust library. A pre-built shared library is downloaded automatically during installation.

---

## Functions

### `Convert`

Convert HTML to Markdown using default options.

```go
func Convert(html string) (string, error)
```

**Parameters:**

| Parameter | Type | Description |
|-----------|------|-------------|
| `html` | `string` | The HTML string to convert |

**Returns:** `(string, error)` -- the converted Markdown string and any error.

**Example:**

```go
import "github.com/kreuzberg-dev/html-to-markdown/packages/go/v2/htmltomarkdown"

func main() {
    html := "<h1>Hello World</h1><p>This is a paragraph.</p>"
    markdown, err := htmltomarkdown.Convert(html)
    if err != nil {
        log.Fatal(err)
    }
    fmt.Println(markdown)
}
```

---

### `MustConvert`

Like `Convert` but panics on error. Useful when conversion errors are unexpected.

```go
func MustConvert(html string) string
```

**Example:**

```go
markdown := htmltomarkdown.MustConvert("<h1>Title</h1>")
fmt.Println(markdown)
```

---

### `ConvertWithMetadata`

Convert HTML to Markdown with metadata extraction.

```go
func ConvertWithMetadata(html string) (string, *HtmlMetadata, error)
```

**Parameters:**

| Parameter | Type | Description |
|-----------|------|-------------|
| `html` | `string` | The HTML string to convert |

**Returns:** `(string, *HtmlMetadata, error)` -- Markdown string, metadata, and any error.

**Example:**

```go
markdown, metadata, err := htmltomarkdown.ConvertWithMetadata(html)
if err != nil {
    log.Fatal(err)
}

fmt.Println(metadata.Document.Title)
fmt.Println(len(metadata.Headers))
fmt.Println(len(metadata.Links))
```

---

### `Version`

Return the version string of the underlying html-to-markdown library.

```go
func Version() string
```

---

### `StartProfiling` / `StopProfiling`

Control Rust-side CPU profiling (flamegraph output).

```go
func StartProfiling(outputPath string, frequency int) error
func StopProfiling() error
```

---

## Types

### `HtmlMetadata`

```go
type HtmlMetadata struct {
    Document       DocumentMetadata   `json:"document"`
    Headers        []HeaderMetadata   `json:"headers"`
    Links          []LinkMetadata     `json:"links"`
    Images         []ImageMetadata    `json:"images"`
    StructuredData []StructuredData   `json:"structured_data"`
}
```

### `DocumentMetadata`

```go
type DocumentMetadata struct {
    Title         *string           `json:"title"`
    Description   *string           `json:"description"`
    Keywords      []string          `json:"keywords"`
    Author        *string           `json:"author"`
    CanonicalURL  *string           `json:"canonical_url"`
    Language      *string           `json:"language"`
    TextDirection *TextDirection     `json:"text_direction"`
    OpenGraph     map[string]string `json:"open_graph"`
    TwitterCard   map[string]string `json:"twitter_card"`
    MetaTags      map[string]string `json:"meta_tags"`
}
```

### `HeaderMetadata`

```go
type HeaderMetadata struct {
    Level      int     `json:"level"`
    Text       string  `json:"text"`
    ID         *string `json:"id"`
    Depth      int     `json:"depth"`
    HTMLOffset int     `json:"html_offset"`
}
```

### `LinkMetadata`

```go
type LinkMetadata struct {
    Href       string            `json:"href"`
    Text       string            `json:"text"`
    Title      *string           `json:"title"`
    LinkType   LinkType          `json:"link_type"`
    Rel        []string          `json:"rel"`
    Attributes map[string]string `json:"attributes"`
}
```

### `ImageMetadata`

```go
type ImageMetadata struct {
    Src        string            `json:"src"`
    Alt        *string           `json:"alt"`
    Title      *string           `json:"title"`
    Dimensions *[2]uint32        `json:"dimensions"`
    ImageType  ImageType         `json:"image_type"`
    Attributes map[string]string `json:"attributes"`
}
```

### Enum Types

```go
type TextDirection string  // "ltr", "rtl", "auto"
type LinkType string       // "anchor", "internal", "external", "email", "phone", "other"
type ImageType string      // "data_uri", "inline_svg", "external", "relative"
type StructuredDataType string // "json_ld", "microdata", "rdfa"
```

---

## Error Handling

All functions return `error` as the last return value. Errors originate from the Rust core and are surfaced as Go `error` values.

```go
markdown, err := htmltomarkdown.Convert(html)
if err != nil {
    log.Printf("conversion failed: %v", err)
}
```

---

## See Also

- [C API Reference](api-c.md) -- the underlying C FFI that Go binds to
- [Configuration Reference](configuration.md) -- full options documentation
- [Types Reference](types.md) -- cross-language type definitions
