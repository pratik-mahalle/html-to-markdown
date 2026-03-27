using System.Text.Json.Serialization;
using HtmlToMarkdown.Metadata;

namespace HtmlToMarkdown.Serialization;

[JsonSourceGenerationOptions(PropertyNameCaseInsensitive = true)]
[JsonSerializable(typeof(HtmlMetadata))]
[JsonSerializable(typeof(TableExtractionResult))]
[JsonSerializable(typeof(TableData))]
[JsonSerializable(typeof(ConversionResult))]
[JsonSerializable(typeof(ExtractTable))]
[JsonSerializable(typeof(TableGrid))]
[JsonSerializable(typeof(GridCell))]
[JsonSerializable(typeof(ProcessingWarning))]
internal partial class MetadataJsonContext : JsonSerializerContext
{
}
