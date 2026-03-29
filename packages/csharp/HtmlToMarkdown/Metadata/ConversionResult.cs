using System.Collections.Generic;
using System.Text.Json;
using System.Text.Json.Serialization;

namespace HtmlToMarkdown.Metadata;

/// <summary>
/// The primary result of <c>Extract()</c>, containing all extracted content and structured data.
/// </summary>
public record ConversionResult
{
    /// <summary>
    /// The converted Markdown string, or <c>null</c> in extraction-only mode.
    /// </summary>
    [JsonPropertyName("content")]
    public string? Content { get; init; }

    /// <summary>
    /// Extracted HTML metadata (title, links, images, etc.).
    /// </summary>
    [JsonPropertyName("metadata")]
    public HtmlMetadata? Metadata { get; init; }

    /// <summary>
    /// Extracted tables with structured grid data.
    /// </summary>
    [JsonPropertyName("tables")]
    public List<ExtractTable> Tables { get; init; } = [];

    /// <summary>
    /// Structured document tree (raw JSON), or <c>null</c> if not requested.
    /// </summary>
    [JsonPropertyName("document")]
    public JsonElement? Document { get; init; }

    /// <summary>
    /// Extracted inline images (data URIs and SVGs).
    /// </summary>
    [JsonPropertyName("images")]
    public List<InlineImage> Images { get; init; } = [];

    /// <summary>
    /// Non-fatal processing warnings.
    /// </summary>
    [JsonPropertyName("warnings")]
    public List<ProcessingWarning> Warnings { get; init; } = [];
}
