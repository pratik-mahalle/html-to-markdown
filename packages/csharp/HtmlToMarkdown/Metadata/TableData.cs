using System.Collections.Generic;
using System.Text.Json.Serialization;

namespace HtmlToMarkdown.Metadata;

/// <summary>
/// A single table extracted during HTML to Markdown conversion.
/// </summary>
public record TableData
{
    /// <summary>
    /// 2D list of cell string values, in row-major order.
    /// </summary>
    [JsonPropertyName("cells")]
    public List<List<string>> Cells { get; init; } = [];

    /// <summary>
    /// The rendered Markdown representation of the table.
    /// </summary>
    [JsonPropertyName("markdown")]
    public string Markdown { get; init; } = string.Empty;

    /// <summary>
    /// Boolean list indicating which rows are header rows.
    /// </summary>
    [JsonPropertyName("is_header_row")]
    public List<bool> IsHeaderRow { get; init; } = [];
}
