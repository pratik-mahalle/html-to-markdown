package dev.kreuzberg.htmltomarkdown;

import static org.junit.jupiter.api.Assertions.*;

import org.junit.jupiter.api.DisplayName;
import org.junit.jupiter.api.Test;

/** Test suite for OutputFormat enum. */
class OutputFormatTest {

  @Test
  @DisplayName("OutputFormat.MARKDOWN has correct value")
  void testMarkdownValue() {
    assertEquals("markdown", OutputFormat.MARKDOWN.getValue());
    assertEquals("markdown", OutputFormat.MARKDOWN.toString());
  }

  @Test
  @DisplayName("OutputFormat.DJOT has correct value")
  void testDjotValue() {
    assertEquals("djot", OutputFormat.DJOT.getValue());
    assertEquals("djot", OutputFormat.DJOT.toString());
  }

  @Test
  @DisplayName("Parse markdown format")
  void testParseMarkdown() {
    assertEquals(OutputFormat.MARKDOWN, OutputFormat.parse("markdown"));
    assertEquals(OutputFormat.MARKDOWN, OutputFormat.parse("MARKDOWN"));
    assertEquals(OutputFormat.MARKDOWN, OutputFormat.parse("Markdown"));
  }

  @Test
  @DisplayName("Parse djot format")
  void testParseDjot() {
    assertEquals(OutputFormat.DJOT, OutputFormat.parse("djot"));
    assertEquals(OutputFormat.DJOT, OutputFormat.parse("DJOT"));
    assertEquals(OutputFormat.DJOT, OutputFormat.parse("Djot"));
  }

  @Test
  @DisplayName("Parse invalid format throws exception")
  void testParseInvalid() {
    assertThrows(IllegalArgumentException.class, () -> OutputFormat.parse("invalid"));
    assertThrows(IllegalArgumentException.class, () -> OutputFormat.parse(""));
  }

  @Test
  @DisplayName("Parse null throws NullPointerException")
  void testParseNull() {
    assertThrows(IllegalArgumentException.class, () -> OutputFormat.parse(null));
  }
}
