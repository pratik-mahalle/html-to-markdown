using System.Text.Json.Serialization;

namespace HtmlToMarkdown.Metadata;

/// <summary>
/// A single table extracted via Extract(), containing structured cell data and Markdown.
/// </summary>
public record ExtractTable
{
    /// <summary>
    /// The structured table grid with row/column/cell data.
    /// </summary>
    [JsonPropertyName("grid")]
    public TableGrid Grid { get; init; } = new();

    /// <summary>
    /// The rendered Markdown representation of the table.
    /// </summary>
    [JsonPropertyName("markdown")]
    public string Markdown { get; init; } = string.Empty;
}
