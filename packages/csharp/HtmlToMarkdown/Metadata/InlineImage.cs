using System.Collections.Generic;
using System.Text.Json.Serialization;

namespace HtmlToMarkdown.Metadata;

/// <summary>
/// An inline image extracted from an HTML document (e.g. data URIs or SVG elements).
/// This is different from <see cref="ImageMetadata"/>, which captures metadata about images
/// found in the HTML. <c>InlineImage</c> contains the actual decoded image data bytes.
/// </summary>
public record InlineImage
{
    /// <summary>
    /// Raw image data bytes (encoded in its original format).
    /// </summary>
    [JsonPropertyName("data")]
    public byte[] Data { get; init; } = [];

    /// <summary>
    /// Image format (e.g. "png", "jpeg", "gif", "svg").
    /// </summary>
    [JsonPropertyName("format")]
    public string Format { get; init; } = string.Empty;

    /// <summary>
    /// Generated or extracted filename for the image.
    /// </summary>
    [JsonPropertyName("filename")]
    public string? Filename { get; init; }

    /// <summary>
    /// Alt text or other descriptive metadata from the source HTML.
    /// </summary>
    [JsonPropertyName("description")]
    public string? Description { get; init; }

    /// <summary>
    /// Image dimensions as [width, height] if available.
    /// </summary>
    [JsonPropertyName("dimensions")]
    public List<uint>? Dimensions { get; init; }

    /// <summary>
    /// Where the image originated ("img_data_uri" or "svg_element").
    /// </summary>
    [JsonPropertyName("source")]
    public string Source { get; init; } = string.Empty;

    /// <summary>
    /// Additional HTML attributes from the source element.
    /// </summary>
    [JsonPropertyName("attributes")]
    public Dictionary<string, string> Attributes { get; init; } = [];
}
