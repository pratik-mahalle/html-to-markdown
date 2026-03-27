using System.Text.Json.Serialization;

namespace HtmlToMarkdown.Metadata;

/// <summary>
/// A single cell within a table grid, with position and span information.
/// </summary>
public record GridCell
{
    /// <summary>
    /// The text content of the cell.
    /// </summary>
    [JsonPropertyName("content")]
    public string Content { get; init; } = string.Empty;

    /// <summary>
    /// Zero-based row index of the cell.
    /// </summary>
    [JsonPropertyName("row")]
    public uint Row { get; init; }

    /// <summary>
    /// Zero-based column index of the cell.
    /// </summary>
    [JsonPropertyName("col")]
    public uint Col { get; init; }

    /// <summary>
    /// Number of rows this cell spans (1 = no span).
    /// </summary>
    [JsonPropertyName("row_span")]
    public uint RowSpan { get; init; } = 1;

    /// <summary>
    /// Number of columns this cell spans (1 = no span).
    /// </summary>
    [JsonPropertyName("col_span")]
    public uint ColSpan { get; init; } = 1;

    /// <summary>
    /// Whether this cell is a header cell.
    /// </summary>
    [JsonPropertyName("is_header")]
    public bool IsHeader { get; init; }
}
