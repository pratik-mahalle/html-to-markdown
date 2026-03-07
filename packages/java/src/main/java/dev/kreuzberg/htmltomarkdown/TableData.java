package dev.kreuzberg.htmltomarkdown;

import com.fasterxml.jackson.annotation.JsonProperty;
import java.util.List;

/**
 * A single table extracted during HTML to Markdown conversion.
 *
 * @param cells 2D list of cell string values, row-major order
 * @param markdown the rendered Markdown representation of the table
 * @param isHeaderRow boolean list indicating which rows are header rows
 */
public record TableData(
    @JsonProperty("cells") List<List<String>> cells,
    @JsonProperty("markdown") String markdown,
    @JsonProperty("is_header_row") List<Boolean> isHeaderRow) {}
