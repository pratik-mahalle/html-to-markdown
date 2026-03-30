package dev.kreuzberg.htmltomarkdown;

/**
 * Output format for HTML to Markdown conversion.
 *
 * <p>Specifies the target markup language format for the conversion output.
 *
 * <p><b>Example usage:</b>
 *
 * <pre>{@code
 * // Use default Markdown format
 * String markdown = HtmlToMarkdown.convert("<h1>Hello</h1>");
 *
 * // Note: Full options support coming in future versions
 * // ConversionOptions opts = new ConversionOptions().setOutputFormat(OutputFormat.DJOT);
 * // String djot = HtmlToMarkdown.convert("<h1>Hello</h1>", opts);
 * }</pre>
 *
 * @since 2.20.0
 */
public enum OutputFormat {
  /**
   * Standard Markdown (CommonMark compatible). Default.
   *
   * <p>Generates RFC 7764 CommonMark-compliant Markdown output with standard syntax for headings,
   * lists, emphasis, links, images, and code blocks.
   */
  MARKDOWN("markdown"),

  /**
   * Djot lightweight markup language.
   *
   * <p>Generates output in Djot format, a lightweight markup language that is similar to Markdown
   * but with additional features and improved syntax in some areas.
   */
  DJOT("djot");

  private final String value;

  OutputFormat(final String value) {
    this.value = value;
  }

  /**
   * Get the string representation of this output format.
   *
   * @return the format string ("markdown" or "djot")
   */
  public String getValue() {
    return value;
  }

  /**
   * Parse an output format from string value.
   *
   * @param s the format string ("markdown" or "djot")
   * @return the {@code OutputFormat}
   * @throws IllegalArgumentException if the string is null or unknown
   *     <p>For example, {@code OutputFormat.parse("djot")} returns {@code OutputFormat.DJOT}.
   */
  public static OutputFormat parse(final String s) {
    if (s == null) {
      throw new IllegalArgumentException("OutputFormat string cannot be null");
    }
    return switch (s.toLowerCase()) {
      case "markdown" -> MARKDOWN;
      case "djot" -> DJOT;
      default -> throw new IllegalArgumentException("Unknown OutputFormat: " + s);
    };
  }

  @Override
  public String toString() {
    return value;
  }
}
