package dev.kreuzberg.htmltomarkdown;

import com.fasterxml.jackson.databind.ObjectMapper;
import java.lang.foreign.Arena;
import java.lang.foreign.MemorySegment;

/**
 * High-performance HTML to Markdown converter with Rust core.
 *
 * <p>This class provides a Java interface to the native html-to-markdown library using Java's
 * Foreign Function &amp; Memory API (Panama FFI).
 *
 * <p><b>Example usage:</b>
 *
 * <pre>{@code
 * String html = "<h1>Hello World</h1><p>This is a <strong>test</strong>.</p>";
 * ConversionResult result = HtmlToMarkdown.convert(html);
 * System.out.println(result.content());
 * // Output:
 * // # Hello World
 * //
 * // This is a **test**.
 * }</pre>
 *
 * <p><b>Thread Safety:</b> This class is thread-safe. Multiple threads can safely call {@link
 * #convert(String)} concurrently.
 *
 * <p><b>Performance:</b> The underlying Rust implementation provides 60-80x higher throughput
 * compared to pure Java HTML-to-Markdown converters.
 *
 * @since 2.7.3
 */
public final class HtmlToMarkdown {

  /** Shared ObjectMapper for JSON deserialization with snake_case support. */
  private static final ObjectMapper MAPPER =
      new ObjectMapper()
          .setPropertyNamingStrategy(
              com.fasterxml.jackson.databind.PropertyNamingStrategies.SNAKE_CASE)
          .configure(
              com.fasterxml.jackson.databind.DeserializationFeature.FAIL_ON_UNKNOWN_PROPERTIES,
              false);

  /**
   * Convert HTML to Markdown in a single pass using default options.
   *
   * <p>Equivalent to calling {@link #convert(String, String)} with {@code null} options.
   *
   * @param html the HTML string to convert
   * @return a {@code ConversionResult} with content and all extracted data
   * @throws NullPointerException if html is null
   * @throws ConversionException if the conversion or JSON parsing fails
   * @since 3.0.0
   */
  public static ConversionResult convert(final String html) {
    return convert(html, null);
  }

  /**
   * Convert HTML to Markdown in a single pass, returning a full {@code ConversionResult}.
   *
   * <p>Returns a {@code ConversionResult} containing:
   *
   * <ul>
   *   <li>The converted Markdown string (or {@code null} in extraction-only mode)
   *   <li>Extracted HTML metadata (title, links, images, structured data)
   *   <li>Extracted tables with structured grid data
   *   <li>Non-fatal processing warnings
   * </ul>
   *
   * @param html the HTML string to convert
   * @param optionsJson optional JSON string for conversion options, or {@code null} for defaults
   * @return a {@code ConversionResult} with content and all extracted data
   * @throws NullPointerException if html is null
   * @throws ConversionException if the conversion or JSON parsing fails
   * @since 3.0.0
   */
  public static ConversionResult convert(final String html, final String optionsJson) {
    if (html == null) {
      throw new NullPointerException("HTML cannot be null");
    }

    if (html.isEmpty()) {
      return new ConversionResult(
          "", null, java.util.List.of(), null, java.util.List.of(), java.util.List.of());
    }

    try (Arena arena = Arena.ofConfined()) {
      MemorySegment htmlSegment = HtmlToMarkdownFFI.toCString(arena, html);
      MemorySegment optionsSegment =
          (optionsJson != null)
              ? HtmlToMarkdownFFI.toCString(arena, optionsJson)
              : MemorySegment.NULL;

      MemorySegment resultSegment;
      try {
        resultSegment =
            (MemorySegment)
                HtmlToMarkdownFFI.html_to_markdown_convert.invoke(htmlSegment, optionsSegment);
      } catch (Throwable e) {
        throw new ConversionException("FFI call to convert failed: " + e.getMessage(), e);
      }

      if (resultSegment == null || resultSegment.address() == 0) {
        String errorMsg = getLastError();
        throw new ConversionException(errorMsg != null ? errorMsg : "convert failed");
      }

      try {
        String jsonStr = HtmlToMarkdownFFI.fromCString(resultSegment);
        return MAPPER.readValue(jsonStr, ConversionResult.class);
      } catch (ConversionException e) {
        throw e;
      } catch (Exception e) {
        throw new ConversionException("failed to parse conversion JSON: " + e.getMessage(), e);
      } finally {
        try {
          HtmlToMarkdownFFI.html_to_markdown_free_string.invoke(resultSegment);
        } catch (Throwable ignored) {
        }
      }
    } catch (ConversionException e) {
      throw e;
    } catch (Throwable e) {
      throw new ConversionException("Failed to convert HTML content", e);
    }
  }

  /**
   * Get the last error message from a failed conversion.
   *
   * @return the last error message, or a descriptive error string if retrieval failed
   */
  private static String getLastError() {
    try {
      MemorySegment errorSegment =
          (MemorySegment) HtmlToMarkdownFFI.html_to_markdown_last_error.invoke();
      return HtmlToMarkdownFFI.fromCString(errorSegment);
    } catch (Throwable e) {
      System.err.println("Failed to retrieve FFI error message: " + e.getMessage());
      e.printStackTrace(System.err);
      return "FFI error retrieval failed: " + e.getClass().getSimpleName();
    }
  }

  private HtmlToMarkdown() {
    throw new UnsupportedOperationException("Utility class");
  }

  /** Exception thrown when HTML-to-Markdown conversion fails. */
  public static class ConversionException extends RuntimeException {
    public ConversionException(String message) {
      super(message);
    }

    public ConversionException(String message, Throwable cause) {
      super(message, cause);
    }
  }
}
