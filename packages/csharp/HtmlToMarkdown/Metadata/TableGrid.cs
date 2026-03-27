using System.Collections.Generic;
using System.Text.Json.Serialization;

namespace HtmlToMarkdown.Metadata;

/// <summary>
/// A structured table grid with row/column dimensions and cell-level data.
/// </summary>
public record TableGrid
{
    /// <summary>
    /// Number of rows in the table.
    /// </summary>
    [JsonPropertyName("rows")]
    public uint Rows { get; init; }

    /// <summary>
    /// Number of columns in the table.
    /// </summary>
    [JsonPropertyName("cols")]
    public uint Cols { get; init; }

    /// <summary>
    /// All cells in the table (may be fewer than rows*cols due to spans).
    /// </summary>
    [JsonPropertyName("cells")]
    public List<GridCell> Cells { get; init; } = [];
}
