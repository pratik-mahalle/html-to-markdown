import { test, expect, describe } from "bun:test";
import { convert, convertWithMetadata, hasMetadataSupport } from "@kreuzberg/html-to-markdown";

describe("html-to-markdown Bun smoke tests", () => {
  describe("Basic conversion", () => {
    test("converts simple HTML to Markdown", () => {
      const html = "<h1>Hello World</h1>";
      const markdown = convert(html);
      expect(markdown).toContain("# Hello World");
      expect(typeof markdown).toBe("string");
    });

    test("converts paragraph to text", () => {
      const html = "<p>This is a test paragraph.</p>";
      const markdown = convert(html);
      expect(markdown).toContain("This is a test paragraph.");
    });

    test("handles empty input", () => {
      const markdown = convert("");
      expect(markdown).toBe("");
    });

    test("handles malformed HTML gracefully", () => {
      const html = "<p>Unclosed paragraph";
      expect(() => convert(html)).not.toThrow();
      const result = convert(html);
      expect(typeof result).toBe("string");
    });
  });

  describe("Metadata extraction", () => {
    test("metadata support is available", () => {
      const supported = hasMetadataSupport();
      expect(typeof supported).toBe("boolean");
    });

    test("extracts basic metadata", () => {
      const html = "<h1>Test</h1><p>Content</p>";
      const result = convertWithMetadata(html);
      expect(result).toBeDefined();
      expect(result.markdown).toBeTruthy();
      expect(result.metadata).toBeDefined();
    });

    test("extracts headers from HTML", () => {
      const html = "<h1>Title</h1><h2>Subtitle</h2>";
      const result = convertWithMetadata(html);
      expect(result.metadata).toBeDefined();
      if (result.metadata.headers) {
        expect(result.metadata.headers.length).toBeGreaterThan(0);
      }
    });
  });

  describe("Options handling", () => {
    test("accepts conversion options", () => {
      const html = "<p>Test</p>";
      const options = { hardBreaks: true };
      expect(() => convert(html, options)).not.toThrow();
      const result = convert(html, options);
      expect(typeof result).toBe("string");
    });

    test("handles null options", () => {
      const html = "<p>Test</p>";
      expect(() => convert(html, null)).not.toThrow();
    });
  });

  describe("Complex HTML", () => {
    test("handles lists", () => {
      const html = "<ul><li>Item 1</li><li>Item 2</li></ul>";
      const markdown = convert(html);
      expect(markdown).toContain("Item 1");
      expect(markdown).toContain("Item 2");
    });

    test("handles links", () => {
      const html = '<a href="https://example.com">Link Text</a>';
      const markdown = convert(html);
      expect(markdown).toContain("Link Text");
      expect(markdown).toContain("https://example.com");
    });

    test("handles images", () => {
      const html = '<img src="image.jpg" alt="Test Image">';
      const markdown = convert(html);
      expect(markdown).toContain("Test Image");
      expect(markdown).toContain("image.jpg");
    });
  });
});
