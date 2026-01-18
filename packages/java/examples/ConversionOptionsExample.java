import dev.kreuzberg.htmltomarkdown.ConversionOptions;
import dev.kreuzberg.htmltomarkdown.HtmlToMarkdown;
import dev.kreuzberg.htmltomarkdown.OutputFormat;

/**
 * Example demonstrating ConversionOptions usage.
 *
 * <p>Shows how to create and configure conversion options with various settings including output
 * format selection.
 *
 * <p><b>Note:</b> Options integration with the conversion process is coming in a future version.
 * Currently, options can be configured but the convert methods only use default options.
 */
public class ConversionOptionsExample {

  /** Default wrap width for examples. */
  private static final int DEFAULT_WRAP_WIDTH = 100;

  /** Run the example. */
  public static void main(final String[] args) {
    // Example 1: Create options with Markdown output (default)
    System.out.println("Example 1: Markdown format (default)");
    ConversionOptions markdownOptions =
        new ConversionOptions()
            .setHeadingStyle("atx")
            .setOutputFormat(OutputFormat.MARKDOWN)
            .setListIndentWidth(2);
    System.out.println("Output format: " + markdownOptions.getOutputFormat());
    System.out.println("Heading style: " + markdownOptions.getHeadingStyle());
    System.out.println();

    // Example 2: Create options with Djot output
    System.out.println("Example 2: Djot format");
    ConversionOptions djotOptions =
        new ConversionOptions().setOutputFormat(OutputFormat.DJOT).setHeadingStyle("underlined");
    System.out.println("Output format: " + djotOptions.getOutputFormat());
    System.out.println("Heading style: " + djotOptions.getHeadingStyle());
    System.out.println();

    // Example 3: Extensive configuration
    System.out.println("Example 3: Extensive configuration");
    ConversionOptions complexOptions =
        new ConversionOptions()
            .setHeadingStyle("atx_closed")
            .setListIndentType("tabs")
            .setListIndentWidth(1)
            .setOutputFormat(OutputFormat.DJOT)
            .setEscapeAsterisks(true)
            .setEscapeUnderscores(false)
            .setWrap(true)
            .setWrapWidth(DEFAULT_WRAP_WIDTH)
            .setSkipImages(false)
            .setDebug(true);
    System.out.println("Options configured: " + complexOptions);
    System.out.println();

    // Example 4: Parse OutputFormat from string
    System.out.println("Example 4: Parse OutputFormat");
    try {
      OutputFormat parsed = OutputFormat.parse("djot");
      System.out.println("Parsed format: " + parsed);
      System.out.println("Value: " + parsed.getValue());
    } catch (IllegalArgumentException e) {
      System.err.println("Failed to parse format: " + e.getMessage());
    }
    System.out.println();

    // Example 5: Current convert with default options
    System.out.println("Example 5: Convert HTML (using default options)");
    String html = "<h1>Hello World</h1><p>This is a <strong>bold</strong> example.</p>";
    try {
      String markdown = HtmlToMarkdown.convert(html);
      System.out.println("Input HTML:");
      System.out.println(html);
      System.out.println("\nConverted Markdown:");
      System.out.println(markdown);
    } catch (HtmlToMarkdown.ConversionException e) {
      System.err.println("Conversion failed: " + e.getMessage());
    }
  }
}
