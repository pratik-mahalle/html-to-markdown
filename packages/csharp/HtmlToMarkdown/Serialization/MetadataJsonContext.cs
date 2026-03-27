using System.Text.Json.Serialization;
using HtmlToMarkdown.Metadata;

namespace HtmlToMarkdown.Serialization;

[JsonSourceGenerationOptions(PropertyNameCaseInsensitive = true)]
[JsonSerializable(typeof(HtmlMetadata))]
[JsonSerializable(typeof(TableExtractionResult))]
[JsonSerializable(typeof(TableData))]
internal partial class MetadataJsonContext : JsonSerializerContext
{
}
