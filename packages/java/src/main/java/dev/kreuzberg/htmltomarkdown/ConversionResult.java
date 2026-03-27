package dev.kreuzberg.htmltomarkdown;

import com.fasterxml.jackson.annotation.JsonProperty;
import dev.kreuzberg.htmltomarkdown.metadata.HtmlMetadata;
import java.util.List;

/**
 * The primary result of {@code extract()}, containing all extracted content and structured data.
 *
 * @param content the converted Markdown string, or {@code null} in extraction-only mode
 * @param metadata extracted HTML metadata (title, links, images, etc.)
 * @param tables extracted tables with structured grid data
 * @param warnings non-fatal processing warnings
 */
public record ConversionResult(
    @JsonProperty("content") String content,
    @JsonProperty("metadata") HtmlMetadata metadata,
    @JsonProperty("tables") List<ExtractTable> tables,
    @JsonProperty("warnings") List<ProcessingWarning> warnings) { }
