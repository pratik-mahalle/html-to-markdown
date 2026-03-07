package dev.kreuzberg.htmltomarkdown;

import com.fasterxml.jackson.annotation.JsonProperty;
import dev.kreuzberg.htmltomarkdown.metadata.ExtendedMetadata;
import java.util.List;

/**
 * Result of HTML to Markdown conversion with table extraction.
 *
 * @param content the converted Markdown string
 * @param metadata optional extended metadata (null if not configured)
 * @param tables list of extracted table data
 */
public record TableExtractionResult(
    @JsonProperty("content") String content,
    @JsonProperty("metadata") ExtendedMetadata metadata,
    @JsonProperty("tables") List<TableData> tables) {}
