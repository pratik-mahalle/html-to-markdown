package dev.kreuzberg.htmltomarkdown;

import com.fasterxml.jackson.annotation.JsonProperty;
import java.util.Collections;
import java.util.List;
import java.util.Map;

/**
 * An inline image extracted from an HTML document (e.g. data URIs or SVG elements).
 *
 * <p>This is different from {@link dev.kreuzberg.htmltomarkdown.metadata.ImageMetadata}, which
 * captures metadata about images found in the HTML. {@code InlineImage} contains the actual decoded
 * image data bytes.
 *
 * @param data raw image data bytes (encoded in its original format)
 * @param format image format (e.g. "png", "jpeg", "gif", "svg")
 * @param filename generated or extracted filename for the image
 * @param description alt text or other descriptive metadata from the source HTML
 * @param dimensions image dimensions as [width, height] if available
 * @param source where the image originated ("img_data_uri" or "svg_element")
 * @param attributes additional HTML attributes from the source element
 * @since 3.0.0
 */
public record InlineImage(
    @JsonProperty("data") byte[] data,
    @JsonProperty("format") String format,
    @JsonProperty("filename") String filename,
    @JsonProperty("description") String description,
    @JsonProperty("dimensions") List<Integer> dimensions,
    @JsonProperty("source") String source,
    @JsonProperty("attributes") Map<String, String> attributes) {

  /**
   * Construct an InlineImage record with default values for nulls.
   *
   * @param data the raw image bytes
   * @param format the image format string
   * @param filename the optional filename
   * @param description the optional description/alt text
   * @param dimensions the optional dimensions [width, height]
   * @param source the image source type
   * @param attributes the HTML attributes
   */
  public InlineImage {
    if (data == null) {
      data = new byte[0];
    }
    if (format == null) {
      format = "";
    }
    if (source == null) {
      source = "";
    }
    if (attributes == null) {
      attributes = Collections.emptyMap();
    }
  }

  /**
   * Check if the image has dimensions specified.
   *
   * @return true if dimensions are present
   */
  public boolean hasDimensions() {
    return dimensions != null && dimensions.size() == 2;
  }

  /**
   * Get the image width if available.
   *
   * @return the width in pixels, or -1 if not specified
   */
  public int getWidth() {
    return hasDimensions() ? dimensions.get(0) : -1;
  }

  /**
   * Get the image height if available.
   *
   * @return the height in pixels, or -1 if not specified
   */
  public int getHeight() {
    return hasDimensions() ? dimensions.get(1) : -1;
  }
}
