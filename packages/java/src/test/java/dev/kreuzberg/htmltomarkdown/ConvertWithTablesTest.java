package dev.kreuzberg.htmltomarkdown;

import static org.junit.jupiter.api.Assertions.*;

import org.junit.jupiter.api.Test;

class ConvertWithTablesTest {

  @Test
  void extractsSimpleTable() {
    String html =
        "<table><thead><tr><th>Name</th><th>Age</th></tr></thead>"
            + "<tbody><tr><td>Alice</td><td>30</td></tr></tbody></table>";

    var result = HtmlToMarkdown.convertWithTables(html);

    assertNotNull(result);
    assertNotNull(result.content());
    assertFalse(result.content().isEmpty());
    assertEquals(1, result.tables().size());

    var table = result.tables().get(0);
    assertNotNull(table.cells());
    assertTrue(table.cells().size() >= 2);
    assertEquals("Name", table.cells().get(0).get(0));
    assertEquals("Age", table.cells().get(0).get(1));
    assertEquals("Alice", table.cells().get(1).get(0));
    assertEquals("30", table.cells().get(1).get(1));
    assertNotNull(table.markdown());
    assertFalse(table.markdown().isEmpty());
    assertNotNull(table.isHeaderRow());
    assertTrue(table.isHeaderRow().get(0));
  }

  @Test
  void returnsEmptyTablesForNonTableHtml() {
    var result = HtmlToMarkdown.convertWithTables("<p>Hello world</p>");

    assertNotNull(result);
    assertFalse(result.content().isEmpty());
    assertTrue(result.tables().isEmpty());
  }

  @Test
  void extractsMultipleTables() {
    String html =
        "<table><tr><th>A</th></tr><tr><td>1</td></tr></table>"
            + "<p>text</p>"
            + "<table><tr><th>B</th></tr><tr><td>2</td></tr></table>";

    var result = HtmlToMarkdown.convertWithTables(html);

    assertEquals(2, result.tables().size());
  }

  @Test
  void handlesEmptyInput() {
    var result = HtmlToMarkdown.convertWithTables("");

    assertNotNull(result);
    assertEquals("", result.content());
    assertTrue(result.tables().isEmpty());
  }

  @Test
  void handlesNullInput() {
    var result = HtmlToMarkdown.convertWithTables(null);

    assertNotNull(result);
    assertTrue(result.tables().isEmpty());
  }

  @Test
  void contentContainsTableText() {
    String html = "<table><tr><th>Header</th></tr><tr><td>Value</td></tr></table>";
    var result = HtmlToMarkdown.convertWithTables(html);

    assertTrue(result.content().contains("Header"));
    assertTrue(result.content().contains("Value"));
  }

  @Test
  void handlesSpecialCharactersInCells() {
    String html = "<table><tr><td>a &amp; b</td><td>c &lt; d</td></tr></table>";
    var result = HtmlToMarkdown.convertWithTables(html);

    assertEquals(1, result.tables().size());
    var table = result.tables().get(0);
    assertTrue(table.cells().get(0).contains("a & b"));
  }
}
