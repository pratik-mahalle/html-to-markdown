using System.Text.Json.Serialization;
using HtmlToMarkdown.Serialization;

namespace HtmlToMarkdown;

/// <summary>
/// Output format for HTML to Markdown conversion.
///
/// Specifies the target markup language format for the conversion output.
/// </summary>
[JsonConverter(typeof(JsonPropertyNameEnumConverter<OutputFormat>))]
public enum OutputFormat
{
    /// <summary>
    /// Standard Markdown (CommonMark compatible). Default.
    /// </summary>
    [JsonPropertyName("markdown")]
    Markdown,

    /// <summary>
    /// Djot lightweight markup language.
    /// </summary>
    [JsonPropertyName("djot")]
    Djot,
}
