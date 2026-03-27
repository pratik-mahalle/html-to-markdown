package dev.kreuzberg.htmltomarkdown;

import com.fasterxml.jackson.annotation.JsonProperty;
import java.util.List;

/**
 * A structured table grid with row/column dimensions and cell-level data.
 *
 * @param rows number of rows
 * @param cols number of columns
 * @param cells all cells in the table (may be fewer than rows*cols due to spans)
 */
public record TableGrid(
    @JsonProperty("rows") int rows,
    @JsonProperty("cols") int cols,
    @JsonProperty("cells") List<GridCell> cells) { }
