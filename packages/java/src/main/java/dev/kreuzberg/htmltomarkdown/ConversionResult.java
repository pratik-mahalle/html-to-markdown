package dev.kreuzberg.htmltomarkdown;

import com.fasterxml.jackson.annotation.JsonProperty;
import com.fasterxml.jackson.databind.JsonNode;
import dev.kreuzberg.htmltomarkdown.metadata.HtmlMetadata;
import java.util.List;

/**
 * The primary result of {@code extract()}, containing all extracted content and structured data.
 *
 * @param content the converted Markdown string, or {@code null} in extraction-only mode
 * @param metadata extracted HTML metadata (title, links, images, etc.)
 * @param tables extracted tables with structured grid data
 * @param document structured document tree (raw JSON), or {@code null} if not requested
 * @param images extracted inline images (data URIs and SVGs)
 * @param warnings non-fatal processing warnings
 */
public record ConversionResult(
    @JsonProperty("content") String content,
    @JsonProperty("metadata") HtmlMetadata metadata,
    @JsonProperty("tables") List<ExtractTable> tables,
    @JsonProperty("document") JsonNode document,
    @JsonProperty("images") List<InlineImage> images,
    @JsonProperty("warnings") List<ProcessingWarning> warnings) { }
