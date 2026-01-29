package dev.kreuzberg.htmltomarkdown;

import dev.kreuzberg.htmltomarkdown.visitor.Visitor;
import org.junit.jupiter.api.Test;
import org.junit.jupiter.api.DisplayName;
import org.junit.jupiter.params.ParameterizedTest;
import org.junit.jupiter.params.provider.ValueSource;
import static org.junit.jupiter.api.Assertions.*;

/**
 * Error handling tests for html-to-markdown.
 *
 * Tests verify that the package properly handles edge cases,
 * invalid inputs, and error conditions.
 */
@DisplayName("Error Handling Tests")
class ErrorHandlingTest {

    @Test
    @DisplayName("Null input throws NullPointerException")
    void testNullInputThrowsNullPointerException() {
        assertThrows(NullPointerException.class, () -> HtmlToMarkdown.convert(null),
                "Should throw NullPointerException for null HTML");
    }

    @Test
    @DisplayName("Empty string converts to empty or whitespace")
    void testEmptyStringHandling() {
        String result = HtmlToMarkdown.convert("");
        assertNotNull(result, "Result should not be null");
        assertTrue(result.isEmpty() || result.trim().isEmpty(),
                "Empty HTML should produce empty or whitespace-only output");
    }

    @ParameterizedTest
    @ValueSource(strings = {
            "<p></p>",
            "<div></div>",
            "<span></span>",
            "<ul></ul>",
            "<table></table>"
    })
    @DisplayName("Empty HTML elements are handled gracefully")
    void testEmptyElementsHandling(String html) {
        String result = HtmlToMarkdown.convert(html);
        assertNotNull(result, "Should not throw exception for empty elements");
        // Result may be empty or contain minimal whitespace
    }

    @Test
    @DisplayName("Malformed HTML is handled")
    void testMalformedHtmlHandling() {
        String malformedHtml = "<p>Unclosed paragraph<div>Nested unclosed</div>";
        String result = HtmlToMarkdown.convert(malformedHtml);
        assertNotNull(result, "Should handle malformed HTML");
        assertTrue(result.contains("Unclosed paragraph") || result.contains("Nested"),
                "Content should be extracted even from malformed HTML");
    }

    @Test
    @DisplayName("HTML with special characters")
    void testSpecialCharactersHandling() {
        String html = "<p>Special &amp; characters &lt;tag&gt; &quot;quoted&quot;</p>";
        String result = HtmlToMarkdown.convert(html);
        assertNotNull(result, "Should handle HTML entities");
        assertTrue(result.contains("&") || result.contains("Special"),
                "Special characters should be handled");
    }

    @Test
    @DisplayName("HTML with Unicode characters")
    void testUnicodeCharactersHandling() {
        String html = "<p>Unicode: 你好 مرحبا 🚀 ñoño</p>";
        String result = HtmlToMarkdown.convert(html);
        assertNotNull(result, "Should handle Unicode");
        assertTrue(result.contains("你好") || result.contains("مرحبا") || result.contains("🚀"),
                "Unicode characters should be preserved");
    }

    @Test
    @DisplayName("Very large HTML input")
    void testLargeInputHandling() {
        StringBuilder sb = new StringBuilder("<html><body>");
        for (int i = 0; i < 1000; i++) {
            sb.append("<p>Paragraph ").append(i).append("</p>");
        }
        sb.append("</body></html>");

        String result = HtmlToMarkdown.convert(sb.toString());
        assertNotNull(result, "Should handle large input");
        assertTrue(result.contains("Paragraph"), "Content should be processed");
    }

    @Test
    @DisplayName("Deep HTML nesting")
    void testDeepNestingHandling() {
        StringBuilder sb = new StringBuilder("<div>");
        for (int i = 0; i < 50; i++) {
            sb.append("<div>");
        }
        sb.append("Content");
        for (int i = 0; i < 50; i++) {
            sb.append("</div>");
        }
        sb.append("</div>");

        String result = HtmlToMarkdown.convert(sb.toString());
        assertNotNull(result, "Should handle deep nesting");
        assertTrue(result.contains("Content"), "Content should be extracted");
    }

    @Test
    @DisplayName("Mixed content with text and HTML")
    void testMixedContentHandling() {
        String html = "Just text <p>and paragraph</p> more text <strong>bold</strong>";
        String result = HtmlToMarkdown.convert(html);
        assertNotNull(result, "Should handle mixed content");
        assertTrue(result.contains("text") && result.contains("paragraph") && result.contains("bold"),
                "All content parts should be preserved");
    }

    @Test
    @DisplayName("HTML comments are handled")
    void testHtmlCommentsHandling() {
        String html = "<p>Visible</p><!-- This is a comment --><p>Also visible</p>";
        String result = HtmlToMarkdown.convert(html);
        assertNotNull(result, "Should handle HTML comments");
        assertTrue(result.contains("Visible") && result.contains("Also visible"),
                "Visible content should be preserved");
        assertFalse(result.contains("comment"), "Comments should be removed");
    }

    @Test
    @DisplayName("Script and style tags are handled")
    void testScriptAndStyleTagsHandling() {
        String html = "<p>Text</p><script>alert('xss')</script><style>.foo{}</style><p>More</p>";
        String result = HtmlToMarkdown.convert(html);
        assertNotNull(result, "Should handle script/style tags");
        assertTrue(result.contains("Text") && result.contains("More"),
                "Text content should be preserved");
    }

    @Test
    @DisplayName("Data URIs and unusual protocols")
    void testDataUrisHandling() {
        String html = "<a href=\"data:text/html,<h1>XSS</h1>\">Link</a>";
        String result = HtmlToMarkdown.convert(html);
        assertNotNull(result, "Should handle data URIs");
        assertTrue(result.contains("Link"), "Link text should be preserved");
    }

    @Test
    @DisplayName("Conversion with null visitor throws NullPointerException")
    void testNullVisitorThrows() {
        assertThrows(NullPointerException.class,
                () -> HtmlToMarkdown.convertWithVisitor("<p>Test</p>", null),
                "Should throw NullPointerException for null visitor");
    }

    @Test
    @DisplayName("Conversion with visitor on null HTML throws NullPointerException")
    void testNullHtmlWithVisitorThrows() {
        Visitor visitor = new Visitor() {
            // Empty implementation
        };
        assertThrows(NullPointerException.class,
                () -> HtmlToMarkdown.convertWithVisitor(null, visitor),
                "Should throw NullPointerException for null HTML");
    }

    @Test
    @DisplayName("Conversion with metadata on null HTML throws NullPointerException")
    void testNullHtmlWithMetadataThrows() {
        assertThrows(NullPointerException.class,
                () -> HtmlToMarkdown.convertWithMetadata(null),
                "Should throw NullPointerException for null HTML");
    }
}
