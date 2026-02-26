---
title: C# API Reference
description: API reference for the KreuzbergDev.HtmlToMarkdown NuGet package
---

# C# API Reference <span class="version-badge">v2.8.0</span>

**Package:** [`KreuzbergDev.HtmlToMarkdown`](https://www.nuget.org/packages/KreuzbergDev.HtmlToMarkdown) | **Version:** 2.26.0 | **.NET:** 8.0+

---

## Installation

```bash
dotnet add package KreuzbergDev.HtmlToMarkdown
```

---

## Class: `HtmlToMarkdownConverter`

All methods are static on `HtmlToMarkdown.HtmlToMarkdownConverter`.

### `Convert`

Convert HTML to Markdown.

```csharp
public static string Convert(string html)
public static string Convert(ReadOnlySpan<byte> html)
```

**Parameters:**

| Parameter | Type | Description |
|-----------|------|-------------|
| `html` | `string` | The HTML string to convert |

**Returns:** `string` -- the converted Markdown.

**Throws:**

- `ArgumentNullException` when `html` is null
- `HtmlToMarkdownException` on conversion failure

**Example:**

```csharp
using HtmlToMarkdown;

string html = "<h1>Hello</h1><p>World</p>";
string markdown = HtmlToMarkdownConverter.Convert(html);

// From UTF-8 bytes (zero-copy path)
ReadOnlySpan<byte> htmlBytes = System.Text.Encoding.UTF8.GetBytes(html);
string markdown = HtmlToMarkdownConverter.Convert(htmlBytes);
```

---

### `ConvertWithMetadata`

Convert HTML to Markdown with metadata extraction.

```csharp
public static ConversionResult ConvertWithMetadata(string html)
public static ConversionResult ConvertWithMetadata(ReadOnlySpan<byte> html)
```

**Returns:** `ConversionResult` with `Markdown` and `Metadata` properties.

**Example:**

```csharp
var result = HtmlToMarkdownConverter.ConvertWithMetadata(html);
Console.WriteLine(result.Markdown);
Console.WriteLine(result.Metadata.Document.Title);
Console.WriteLine(result.Metadata.Headers.Count);
Console.WriteLine(result.Metadata.Links.Count);
```

---

### `ConvertWithVisitor`

Convert HTML with a custom visitor.

```csharp
public static string ConvertWithVisitor(string html, IHtmlVisitor visitor)
```

**Example:**

```csharp
using HtmlToMarkdown.Visitor;

public class SkipImages : IHtmlVisitor
{
    public VisitResult VisitImage(NodeContext ctx, string src, string alt, string? title)
        => VisitResult.Skip();
}

string markdown = HtmlToMarkdownConverter.ConvertWithVisitor(html, new SkipImages());
```

---

### `Version`

Return the version string of the native library.

```csharp
public static string Version()
```

---

## Types

### `ConversionResult`

```csharp
public class ConversionResult
{
    public string Markdown { get; }
    public ExtendedMetadata Metadata { get; }
}
```

### `ExtendedMetadata`

```csharp
public class ExtendedMetadata
{
    public DocumentMetadata Document { get; set; }
    public List<HeaderMetadata> Headers { get; set; }
    public List<LinkMetadata> Links { get; set; }
    public List<ImageMetadata> Images { get; set; }
    public List<StructuredData> StructuredData { get; set; }
}
```

### `DocumentMetadata`

```csharp
public class DocumentMetadata
{
    public string? Title { get; set; }
    public string? Description { get; set; }
    public List<string> Keywords { get; set; }
    public string? Author { get; set; }
    public string? CanonicalUrl { get; set; }
    public string? Language { get; set; }
    public string? TextDirection { get; set; }
    public Dictionary<string, string> OpenGraph { get; set; }
    public Dictionary<string, string> TwitterCard { get; set; }
    public Dictionary<string, string> MetaTags { get; set; }
}
```

### `HeaderMetadata`

```csharp
public class HeaderMetadata
{
    public int Level { get; set; }
    public string Text { get; set; }
    public string? Id { get; set; }
    public int Depth { get; set; }
    public int HtmlOffset { get; set; }
}
```

---

## Visitor Interface

### `IHtmlVisitor`

```csharp
public interface IHtmlVisitor
{
    VisitResult VisitText(NodeContext ctx, string text) => VisitResult.Continue();
    VisitResult VisitLink(NodeContext ctx, string href, string text, string? title) => VisitResult.Continue();
    VisitResult VisitImage(NodeContext ctx, string src, string alt, string? title) => VisitResult.Continue();
    VisitResult VisitHeading(NodeContext ctx, int level, string text, string? id) => VisitResult.Continue();
    VisitResult VisitCodeBlock(NodeContext ctx, string? language, string code) => VisitResult.Continue();
    VisitResult VisitCodeInline(NodeContext ctx, string code) => VisitResult.Continue();
    // ... and more
}
```

### `VisitResult`

```csharp
public class VisitResult
{
    public static VisitResult Continue();
    public static VisitResult Skip();
    public static VisitResult PreserveHtml();
    public static VisitResult Custom(string output);
    public static VisitResult Error(string message);
}
```

---

## P/Invoke Details

The .NET binding uses P/Invoke (`DllImport`) to call the native C FFI library. Key implementation details:

- Native library is bundled as a runtime-specific NuGet asset
- UTF-8 string marshalling via `Marshal.StringToCoTaskMemUTF8`
- Memory freed via the library's `html_to_markdown_free_string` function
- Supports `ReadOnlySpan<byte>` for zero-copy byte input
- Thread-safe: each call manages its own native memory

---

## See Also

- [Configuration Reference](configuration.md) -- full options documentation
- [Types Reference](types.md) -- cross-language type definitions
- [C# Migration Guide (v2.19.0)](../migration/csharp-2.19.0.md) -- migrating from earlier versions
