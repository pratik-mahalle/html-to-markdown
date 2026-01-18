package dev.kreuzberg.htmltomarkdown;

import static org.junit.jupiter.api.Assertions.*;

import java.util.Collections;
import java.util.Set;
import org.junit.jupiter.api.DisplayName;
import org.junit.jupiter.api.Test;

/** Test suite for ConversionOptions class. */
class ConversionOptionsTest {

  @Test
  @DisplayName("ConversionOptions default values")
  void testDefaultValues() {
    ConversionOptions opts = new ConversionOptions();
    assertEquals("atx", opts.getHeadingStyle());
    assertEquals("spaces", opts.getListIndentType());
    assertEquals(2, opts.getListIndentWidth());
    assertEquals("-*+", opts.getBullets());
    assertEquals("*", opts.getStrongEmSymbol());
    assertFalse(opts.isEscapeAsterisks());
    assertFalse(opts.isEscapeUnderscores());
    assertFalse(opts.isEscapeMisc());
    assertFalse(opts.isEscapeAscii());
    assertEquals("", opts.getCodeLanguage());
    assertEquals("utf-8", opts.getEncoding());
    assertTrue(opts.isAutolinks());
    assertFalse(opts.isDefaultTitle());
    assertEquals(Collections.emptySet(), opts.getKeepInlineImagesIn());
    assertFalse(opts.isBrInTables());
    assertTrue(opts.isHocrSpatialTables());
    assertEquals("double-equal", opts.getHighlightStyle());
    assertTrue(opts.isExtractMetadata());
    assertEquals("normalized", opts.getWhitespaceMode());
    assertFalse(opts.isStripNewlines());
    assertFalse(opts.isWrap());
    assertEquals(80, opts.getWrapWidth());
    assertEquals(Collections.emptySet(), opts.getStripTags());
    assertEquals(Collections.emptySet(), opts.getPreserveTags());
    assertFalse(opts.isSkipImages());
    assertFalse(opts.isConvertAsInline());
    assertEquals("", opts.getSubSymbol());
    assertEquals("", opts.getSupSymbol());
    assertEquals("spaces", opts.getNewlineStyle());
    assertEquals("backticks", opts.getCodeBlockStyle());
    assertEquals(OutputFormat.MARKDOWN, opts.getOutputFormat());
    assertFalse(opts.isDebug());
  }

  @Test
  @DisplayName("ConversionOptions builder pattern")
  void testBuilderPattern() {
    ConversionOptions opts =
        new ConversionOptions()
            .setHeadingStyle("underlined")
            .setOutputFormat(OutputFormat.DJOT)
            .setListIndentWidth(4)
            .setEscapeAsterisks(true);

    assertEquals("underlined", opts.getHeadingStyle());
    assertEquals(OutputFormat.DJOT, opts.getOutputFormat());
    assertEquals(4, opts.getListIndentWidth());
    assertTrue(opts.isEscapeAsterisks());
  }

  @Test
  @DisplayName("ConversionOptions setHeadingStyle")
  void testSetHeadingStyle() {
    ConversionOptions opts = new ConversionOptions().setHeadingStyle("underlined");
    assertEquals("underlined", opts.getHeadingStyle());
  }

  @Test
  @DisplayName("ConversionOptions setHeadingStyle null throws exception")
  void testSetHeadingStyleNull() {
    ConversionOptions opts = new ConversionOptions();
    assertThrows(NullPointerException.class, () -> opts.setHeadingStyle(null));
  }

  @Test
  @DisplayName("ConversionOptions setOutputFormat")
  void testSetOutputFormat() {
    ConversionOptions opts1 = new ConversionOptions().setOutputFormat(OutputFormat.DJOT);
    assertEquals(OutputFormat.DJOT, opts1.getOutputFormat());

    ConversionOptions opts2 = new ConversionOptions().setOutputFormat(OutputFormat.MARKDOWN);
    assertEquals(OutputFormat.MARKDOWN, opts2.getOutputFormat());
  }

  @Test
  @DisplayName("ConversionOptions setOutputFormat null throws exception")
  void testSetOutputFormatNull() {
    ConversionOptions opts = new ConversionOptions();
    assertThrows(NullPointerException.class, () -> opts.setOutputFormat(null));
  }

  @Test
  @DisplayName("ConversionOptions setListIndentWidth")
  void testSetListIndentWidth() {
    ConversionOptions opts = new ConversionOptions().setListIndentWidth(4);
    assertEquals(4, opts.getListIndentWidth());
  }

  @Test
  @DisplayName("ConversionOptions setListIndentWidth negative throws exception")
  void testSetListIndentWidthNegative() {
    ConversionOptions opts = new ConversionOptions();
    assertThrows(IllegalArgumentException.class, () -> opts.setListIndentWidth(-1));
  }

  @Test
  @DisplayName("ConversionOptions setWrapWidth")
  void testSetWrapWidth() {
    ConversionOptions opts = new ConversionOptions().setWrapWidth(120);
    assertEquals(120, opts.getWrapWidth());
  }

  @Test
  @DisplayName("ConversionOptions setWrapWidth zero or negative throws exception")
  void testSetWrapWidthInvalid() {
    ConversionOptions opts = new ConversionOptions();
    assertThrows(IllegalArgumentException.class, () -> opts.setWrapWidth(0));
    assertThrows(IllegalArgumentException.class, () -> opts.setWrapWidth(-1));
  }

  @Test
  @DisplayName("ConversionOptions setSkipImages")
  void testSetSkipImages() {
    ConversionOptions opts = new ConversionOptions().setSkipImages(true);
    assertTrue(opts.isSkipImages());
  }

  @Test
  @DisplayName("ConversionOptions setStripTags")
  void testSetStripTags() {
    Set<String> tags = Set.of("script", "style");
    ConversionOptions opts = new ConversionOptions().setStripTags(tags);
    assertEquals(tags, opts.getStripTags());
  }

  @Test
  @DisplayName("ConversionOptions setStripTags with null")
  void testSetStripTagsNull() {
    ConversionOptions opts = new ConversionOptions().setStripTags(null);
    assertEquals(Collections.emptySet(), opts.getStripTags());
  }

  @Test
  @DisplayName("ConversionOptions setPreserveTags")
  void testSetPreserveTags() {
    Set<String> tags = Set.of("table", "math");
    ConversionOptions opts = new ConversionOptions().setPreserveTags(tags);
    assertEquals(tags, opts.getPreserveTags());
  }

  @Test
  @DisplayName("ConversionOptions setPreserveTags with null")
  void testSetPreserveTagsNull() {
    ConversionOptions opts = new ConversionOptions().setPreserveTags(null);
    assertEquals(Collections.emptySet(), opts.getPreserveTags());
  }

  @Test
  @DisplayName("ConversionOptions toString")
  void testToString() {
    ConversionOptions opts =
        new ConversionOptions()
            .setHeadingStyle("underlined")
            .setOutputFormat(OutputFormat.DJOT)
            .setListIndentWidth(4)
            .setSkipImages(true);

    String str = opts.toString();
    assertNotNull(str);
    assertTrue(str.length() > 0);
    assertTrue(str.contains("ConversionOptions"));
  }

  @Test
  @DisplayName("ConversionOptions method chaining")
  void testMethodChaining() {
    ConversionOptions opts =
        new ConversionOptions()
            .setHeadingStyle("atx_closed")
            .setListIndentType("tabs")
            .setWrap(true)
            .setWrapWidth(100)
            .setDebug(true);

    assertEquals("atx_closed", opts.getHeadingStyle());
    assertEquals("tabs", opts.getListIndentType());
    assertTrue(opts.isWrap());
    assertEquals(100, opts.getWrapWidth());
    assertTrue(opts.isDebug());
  }
}
