using System.Collections.Generic;
using System.Text.Json.Serialization;

namespace HtmlToMarkdown.Metadata;

/// <summary>
/// Result of HTML to Markdown conversion with table extraction.
/// </summary>
public record TableExtractionResult
{
    /// <summary>
    /// The converted Markdown string.
    /// </summary>
    [JsonPropertyName("content")]
    public string Content { get; init; } = string.Empty;

    /// <summary>
    /// Optional extended metadata extracted during conversion.
    /// </summary>
    [JsonPropertyName("metadata")]
    public ExtendedMetadata? Metadata { get; init; }

    /// <summary>
    /// List of tables extracted during conversion.
    /// </summary>
    [JsonPropertyName("tables")]
    public List<TableData> Tables { get; init; } = [];
}
