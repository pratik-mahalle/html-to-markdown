package dev.kreuzberg.htmltomarkdown;

import org.junit.jupiter.api.Test;
import org.junit.jupiter.api.DisplayName;
import static org.junit.jupiter.api.Assertions.*;

/**
 * Smoke tests for basic html-to-markdown functionality.
 *
 * Tests verify that the published Maven Central package loads correctly
 * and performs basic conversions through JNI/FFI bindings.
 */
@DisplayName("Smoke Tests - Basic Functionality")
class SmokeTest {
    @Test
    @DisplayName("Package and classes load successfully")
    void testPackageLoads() {
        assertNotNull(HtmlToMarkdown.class, "HtmlToMarkdown class should be available");
        assertNotNull(HtmlToMarkdown.ConversionException.class, "ConversionException should be available");
    }

    @Test
    @DisplayName("Convert simple paragraph")
    void testBasicConversion() {
        String html = "<p>Hello World</p>";
        String result = HtmlToMarkdown.convert(html);
        assertNotNull(result, "Result should not be null");
        assertTrue(result.contains("Hello World"), "Result should contain original text");
    }

    @Test
    @DisplayName("Convert heading with ATX style")
    void testHeadingConversion() {
        String html = "<h1>Title</h1>";
        String result = HtmlToMarkdown.convert(html);
        assertTrue(result.startsWith("#"), "ATX-style heading should start with #");
        assertTrue(result.contains("Title"), "Heading should contain text");
    }

    @Test
    @DisplayName("Handle empty input")
    void testEmptyInput() {
        String result = HtmlToMarkdown.convert("");
        assertNotNull(result, "Result should not be null for empty input");
    }

    @Test
    @DisplayName("Handle null input with NullPointerException")
    void testNullInputThrows() {
        assertThrows(NullPointerException.class, () -> HtmlToMarkdown.convert(null),
                "Should throw NullPointerException for null input");
    }

    @Test
    @DisplayName("Get library version")
    void testGetVersion() {
        String version = HtmlToMarkdown.getVersion();
        assertNotNull(version, "Version should not be null");
        assertTrue(version.matches("\\d+\\.\\d+\\.\\d+"), "Version should match semver format");
    }

    @Test
    @DisplayName("Convert strong/bold text")
    void testStrongConversion() {
        String html = "<strong>bold text</strong>";
        String result = HtmlToMarkdown.convert(html);
        assertTrue(result.contains("**bold text**") || result.contains("__bold text__"),
                "Should convert strong to **text** or __text__");
    }

    @Test
    @DisplayName("Convert emphasis/italic text")
    void testEmphasisConversion() {
        String html = "<em>italic text</em>";
        String result = HtmlToMarkdown.convert(html);
        assertTrue(result.contains("*italic text*") || result.contains("_italic text_"),
                "Should convert em to *text* or _text_");
    }

    @Test
    @DisplayName("Convert unordered list")
    void testUnorderedListConversion() {
        String html = "<ul><li>Item 1</li><li>Item 2</li></ul>";
        String result = HtmlToMarkdown.convert(html);
        assertTrue(result.contains("Item 1") && result.contains("Item 2"),
                "List items should be preserved");
        assertTrue(result.contains("-") || result.contains("*"),
                "Should contain list marker");
    }

    @Test
    @DisplayName("Convert ordered list")
    void testOrderedListConversion() {
        String html = "<ol><li>First</li><li>Second</li></ol>";
        String result = HtmlToMarkdown.convert(html);
        assertTrue(result.contains("First") && result.contains("Second"),
                "List items should be preserved");
        assertTrue(result.matches("(?s).*1\\..*2\\..*"),
                "Should contain numbered list markers");
    }

    @Test
    @DisplayName("Convert hyperlink")
    void testLinkConversion() {
        String html = "<a href=\"https://example.com\">Example</a>";
        String result = HtmlToMarkdown.convert(html);
        assertTrue(result.contains("[Example]") && result.contains("(https://example.com)"),
                "Should convert to Markdown link format [text](url)");
    }

    @Test
    @DisplayName("Convert inline code")
    void testInlineCodeConversion() {
        String html = "<code>console.log('hello')</code>";
        String result = HtmlToMarkdown.convert(html);
        assertTrue(result.contains("`"), "Should contain backticks for inline code");
        assertTrue(result.contains("console.log"), "Code content should be preserved");
    }

    @Test
    @DisplayName("Convert blockquote")
    void testBlockquoteConversion() {
        String html = "<blockquote>Quote</blockquote>";
        String result = HtmlToMarkdown.convert(html);
        assertTrue(result.contains(">"), "Should contain blockquote marker");
        assertTrue(result.contains("Quote"), "Quote text should be preserved");
    }

    @Test
    @DisplayName("Convert combined elements")
    void testCombinedElementsConversion() {
        String html = "<h1>Title</h1><p>Content with <strong>bold</strong> text</p>";
        String result = HtmlToMarkdown.convert(html);
        assertTrue(result.contains("Title"), "Title should be present");
        assertTrue(result.contains("Content"), "Content should be present");
        assertTrue(result.contains("bold"), "Bold text should be present");
    }
}
