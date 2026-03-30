package dev.kreuzberg.htmltomarkdown;

import com.fasterxml.jackson.annotation.JsonProperty;

/**
 * A single table extracted via {@code extract()}, containing structured cell data and markdown.
 *
 * @param grid the structured table grid with row/column/cell data
 * @param markdown the rendered Markdown representation of the table
 */
public record ExtractTable(
    @JsonProperty("grid") TableGrid grid, @JsonProperty("markdown") String markdown) { }
