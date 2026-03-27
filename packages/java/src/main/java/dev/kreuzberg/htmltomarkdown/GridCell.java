package dev.kreuzberg.htmltomarkdown;

import com.fasterxml.jackson.annotation.JsonProperty;

/**
 * A single cell in an extracted table grid.
 *
 * @param content the text content of the cell
 * @param row 0-indexed row position
 * @param col 0-indexed column position
 * @param rowSpan number of rows this cell spans (default 1)
 * @param colSpan number of columns this cell spans (default 1)
 * @param isHeader whether this is a header cell
 */
public record GridCell(
    @JsonProperty("content") String content,
    @JsonProperty("row") int row,
    @JsonProperty("col") int col,
    @JsonProperty("row_span") int rowSpan,
    @JsonProperty("col_span") int colSpan,
    @JsonProperty("is_header") boolean isHeader) { }
