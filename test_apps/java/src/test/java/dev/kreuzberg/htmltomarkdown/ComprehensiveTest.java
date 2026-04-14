package dev.kreuzberg.htmltomarkdown;

import com.fasterxml.jackson.databind.ObjectMapper;
import org.junit.jupiter.api.DisplayName;
import org.junit.jupiter.params.ParameterizedTest;
import org.junit.jupiter.params.provider.MethodSource;
import org.junit.jupiter.api.Test;

import java.io.File;
import java.io.IOException;
import java.util.Arrays;
import java.util.stream.Stream;

import static org.junit.jupiter.api.Assertions.*;

/**
 * Comprehensive tests for html-to-markdown using fixture files.
 *
 * These tests validate conversion accuracy against fixture files
 * containing expected HTML-to-Markdown mappings.
 */
@DisplayName("Comprehensive Tests - Fixture-based Validation")
class ComprehensiveTest {
    record TestCase(String name, String html, String expectedMarkdown, Object options) { }

    /**
     * Load test fixtures from JSON files.
     * Tries multiple paths to find fixture files.
     */
    private static File findFixtureFile(String filename) {
        // Try relative paths from different locations
        String[] possiblePaths = {
                "tests/test_apps/fixtures/" + filename,
                "../fixtures/" + filename,
                "../../fixtures/" + filename,
                "fixtures/" + filename,
                filename
        };

        for (String path : possiblePaths) {
            File file = new File(path);
            if (file.exists()) {
                return file;
            }
        }

        return null;
    }

    static Stream<TestCase> basicHtmlProvider() throws IOException {
        File fixtureFile = findFixtureFile("basic-html.json");
        if (fixtureFile == null || !fixtureFile.exists()) {
            return Stream.empty();
        }

        ObjectMapper mapper = new ObjectMapper();
        TestCase[] cases = mapper.readValue(fixtureFile, TestCase[].class);
        return Arrays.stream(cases);
    }

    @ParameterizedTest(name = "{0}")
    @MethodSource("basicHtmlProvider")
    @DisplayName("Basic HTML conversions from fixture file")
    void testBasicHtmlConversion(TestCase testCase) {
        assertNotNull(testCase.name(), "Test case name should not be null");
        assertNotNull(testCase.html(), "HTML input should not be null");
        assertNotNull(testCase.expectedMarkdown(), "Expected markdown should not be null");

        String result = HtmlToMarkdown.convert(testCase.html());

        assertNotNull(result, "Result should not be null for: " + testCase.name());

        // Normalize whitespace for comparison
        String normalizedResult = result.trim();
        String normalizedExpected = testCase.expectedMarkdown().trim();

        assertEquals(normalizedExpected, normalizedResult,
                "Failed for test case: " + testCase.name());
    }

    @Test
    @DisplayName("Batch conversion of multiple HTML inputs")
    void testBatchConversion() {
        String[] inputs = {
                "<h1>Heading</h1>",
                "<p>Paragraph</p>",
                "<ul><li>Item</li></ul>",
                "<strong>Bold</strong>",
                "<em>Italic</em>"
        };

        for (String html : inputs) {
            String result = HtmlToMarkdown.convert(html);
            assertNotNull(result, "Result should not be null for: " + html);
            assertFalse(result.isEmpty() || result.trim().isEmpty(),
                    "Result should not be empty for: " + html);
        }
    }

    @Test
    @DisplayName("Type safety - ConversionOptions creation and usage")
    void testConversionOptionsTypeSafety() {
        ConversionOptions options = new ConversionOptions();
        assertNotNull(options, "ConversionOptions should be instantiable");
        assertNotNull(options.getHeadingStyle(), "Heading style should not be null");

        // Test builder pattern
        ConversionOptions configured = new ConversionOptions()
                .setHeadingStyle("atx")
                .setListIndentWidth(2)
                .setEscapeAsterisks(false);

        assertNotNull(configured, "Configured options should not be null");
    }

    @Test
    @DisplayName("Verify JNI/FFI functionality through version checking")
    void testFFIFunctionality() {
        // This test verifies that FFI bindings are working
        String version = HtmlToMarkdown.getVersion();
        assertNotNull(version, "Version should be retrievable via FFI");
        assertTrue(version.matches("\\d+\\.\\d+\\.\\d+"),
                "Version should follow semantic versioning");
    }

    @Test
    @DisplayName("Complex HTML with multiple formatting")
    void testComplexHtmlFormatting() {
        String html = "<article>"
                + "<h1>Article Title</h1>"
                + "<p>Introduction paragraph with <strong>bold</strong> and <em>italic</em>.</p>"
                + "<h2>Section 1</h2>"
                + "<p>Content with <a href=\"/link\">link</a>.</p>"
                + "<ul>"
                + "<li>List item 1</li>"
                + "<li>List item 2 with <code>code</code></li>"
                + "</ul>"
                + "<blockquote>A quote</blockquote>"
                + "</article>";

        String result = HtmlToMarkdown.convert(html);
        assertNotNull(result, "Complex HTML result should not be null");
        assertTrue(result.contains("Article Title"), "Title should be preserved");
        assertTrue(result.contains("bold"), "Bold text should be preserved");
        assertTrue(result.contains("italic"), "Italic text should be preserved");
        assertTrue(result.contains("link"), "Link should be preserved");
    }

    @Test
    @DisplayName("Table conversion")
    void testTableConversion() {
        String html = "<table><tr><th>Header 1</th><th>Header 2</th></tr>"
                + "<tr><td>Cell 1</td><td>Cell 2</td></tr></table>";

        String result = HtmlToMarkdown.convert(html);
        assertNotNull(result, "Table result should not be null");
        // Tables may be converted to pipe format or text
        assertTrue(result.contains("Header") || result.contains("Cell"),
                "Table content should be preserved");
    }

    @Test
    @DisplayName("Code block conversion")
    void testCodeBlockConversion() {
        String html = "<pre><code>function test() {\n  return true;\n}</code></pre>";

        String result = HtmlToMarkdown.convert(html);
        assertNotNull(result, "Code block result should not be null");
        assertTrue(result.contains("function") || result.contains("test"),
                "Code content should be preserved");
    }

    @Test
    @DisplayName("HTML with attributes preservation")
    void testAttributePreservation() {
        String html = "<a href=\"https://example.com\" title=\"Example Site\">Link</a>";

        String result = HtmlToMarkdown.convert(html);
        assertNotNull(result, "Link with attributes result should not be null");
        assertTrue(result.contains("Link"), "Link text should be preserved");
    }
}
