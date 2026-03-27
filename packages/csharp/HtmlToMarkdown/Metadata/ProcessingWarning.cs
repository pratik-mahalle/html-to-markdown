using System.Text.Json.Serialization;

namespace HtmlToMarkdown.Metadata;

/// <summary>
/// A non-fatal processing warning produced during conversion.
/// </summary>
public record ProcessingWarning
{
    /// <summary>
    /// Human-readable description of the warning.
    /// </summary>
    [JsonPropertyName("message")]
    public string Message { get; init; } = string.Empty;

    /// <summary>
    /// The warning category (e.g., "malformed_html", "encoding_fallback").
    /// </summary>
    [JsonPropertyName("kind")]
    public string Kind { get; init; } = string.Empty;
}
