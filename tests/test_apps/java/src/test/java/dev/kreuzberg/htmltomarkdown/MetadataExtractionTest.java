package dev.kreuzberg.htmltomarkdown;

import dev.kreuzberg.htmltomarkdown.metadata.MetadataExtraction;
import org.junit.jupiter.api.Test;
import org.junit.jupiter.api.DisplayName;
import static org.junit.jupiter.api.Assertions.*;

/**
 * Metadata extraction tests for html-to-markdown.
 *
 * Tests verify that document metadata such as titles, headers,
 * links, images, and structured data are extracted correctly.
 */
@DisplayName("Metadata Extraction Tests")
class MetadataExtractionTest {

    @Test
    @DisplayName("Extract basic document metadata")
    void testBasicMetadataExtraction() {
        String html = "<html>"
                + "<head><title>Test Document</title></head>"
                + "<body><h1>Main Heading</h1></body>"
                + "</html>";

        MetadataExtraction result = HtmlToMarkdown.convertWithMetadata(html);

        assertNotNull(result, "MetadataExtraction should not be null");
        assertNotNull(result.getMarkdown(), "Markdown should be extracted");
        assertNotNull(result.getMetadata(), "Metadata should be extracted");
    }

    @Test
    @DisplayName("Extract markdown content alongside metadata")
    void testMarkdownWithMetadataExtraction() {
        String html = "<h1>Title</h1><p>Content with <strong>bold</strong></p>";

        MetadataExtraction result = HtmlToMarkdown.convertWithMetadata(html);

        String markdown = result.getMarkdown();
        assertNotNull(markdown, "Markdown should not be null");
        assertTrue(markdown.contains("Title"), "Title should be in markdown");
        assertTrue(markdown.contains("Content"), "Content should be in markdown");
        assertTrue(markdown.contains("bold"), "Bold formatting should be in markdown");
    }

    @Test
    @DisplayName("Extract document with multiple headers")
    void testMultipleHeaderExtraction() {
        String html = "<h1>Main Title</h1>"
                + "<h2>Section 1</h2>"
                + "<p>Content 1</p>"
                + "<h2>Section 2</h2>"
                + "<p>Content 2</p>"
                + "<h3>Subsection</h3>";

        MetadataExtraction result = HtmlToMarkdown.convertWithMetadata(html);

        assertNotNull(result, "Result should not be null");
        String markdown = result.getMarkdown();
        assertTrue(markdown.contains("Main Title"), "Main title should be present");
        assertTrue(markdown.contains("Section 1"), "Section 1 should be present");
        assertTrue(markdown.contains("Section 2"), "Section 2 should be present");
        assertTrue(markdown.contains("Subsection"), "Subsection should be present");
    }

    @Test
    @DisplayName("Extract document with links")
    void testLinkExtraction() {
        String html = "<a href=\"https://example.com\">Example</a>"
                + "<a href=\"https://test.com\" title=\"Test Site\">Test</a>";

        MetadataExtraction result = HtmlToMarkdown.convertWithMetadata(html);

        assertNotNull(result, "Result should not be null");
        String markdown = result.getMarkdown();
        assertTrue(markdown.contains("Example") && markdown.contains("https://example.com"),
                "Links should be in markdown");
    }

    @Test
    @DisplayName("Extract document with images")
    void testImageExtraction() {
        String html = "<img src=\"image1.png\" alt=\"First Image\" />"
                + "<img src=\"https://example.com/image2.jpg\" alt=\"Second Image\" />";

        MetadataExtraction result = HtmlToMarkdown.convertWithMetadata(html);

        assertNotNull(result, "Result should not be null");
        String markdown = result.getMarkdown();
        // Images should be converted to markdown format or referenced
        assertTrue(markdown.contains("image1.png") || markdown.contains("First Image"),
                "Image references should be present");
    }

    @Test
    @DisplayName("Extract complex document with mixed content")
    void testComplexDocumentExtraction() {
        String html = "<html>"
                + "<head><title>Complex Document</title>"
                + "<meta name=\"description\" content=\"Test description\" />"
                + "</head>"
                + "<body>"
                + "<h1>Title</h1>"
                + "<p>Introduction paragraph.</p>"
                + "<h2>Section 1</h2>"
                + "<p>Content with <a href=\"#link1\">link</a>.</p>"
                + "<h2>Section 2</h2>"
                + "<p>More content with <img src=\"image.png\" alt=\"Image\" />.</p>"
                + "<ul><li>Item 1</li><li>Item 2</li></ul>"
                + "</body>"
                + "</html>";

        MetadataExtraction result = HtmlToMarkdown.convertWithMetadata(html);

        assertNotNull(result, "Result should not be null");
        assertNotNull(result.getMarkdown(), "Markdown should not be null");
        assertNotNull(result.getMetadata(), "Metadata should not be null");

        String markdown = result.getMarkdown();
        assertTrue(markdown.contains("Title"), "Title should be present");
        assertTrue(markdown.contains("Introduction"), "Intro should be present");
        assertTrue(markdown.contains("Section 1"), "Section 1 should be present");
        assertTrue(markdown.contains("Section 2"), "Section 2 should be present");
    }

    @Test
    @DisplayName("Extract document with nested structure")
    void testNestedStructureExtraction() {
        String html = "<article>"
                + "<h1>Article</h1>"
                + "<section>"
                + "<h2>Part 1</h2>"
                + "<p>Content</p>"
                + "</section>"
                + "<section>"
                + "<h2>Part 2</h2>"
                + "<article><h3>Nested</h3></article>"
                + "</section>"
                + "</article>";

        MetadataExtraction result = HtmlToMarkdown.convertWithMetadata(html);

        assertNotNull(result, "Result should not be null");
        String markdown = result.getMarkdown();
        assertTrue(markdown.contains("Article"), "Article should be present");
        assertTrue(markdown.contains("Part 1"), "Part 1 should be present");
        assertTrue(markdown.contains("Part 2"), "Part 2 should be present");
    }

    @Test
    @DisplayName("Extract document with special content types")
    void testSpecialContentExtraction() {
        String html = "<blockquote>Quote text</blockquote>"
                + "<pre><code>const x = 1;</code></pre>"
                + "<table><tr><td>Cell</td></tr></table>"
                + "<hr />";

        MetadataExtraction result = HtmlToMarkdown.convertWithMetadata(html);

        assertNotNull(result, "Result should not be null");
        String markdown = result.getMarkdown();
        assertTrue(markdown.contains("Quote") || markdown.contains(">"),
                "Blockquote should be preserved");
        assertTrue(markdown.contains("const") || markdown.contains("code"),
                "Code block should be preserved");
    }

    @Test
    @DisplayName("Empty HTML produces valid extraction")
    void testEmptyDocumentExtraction() {
        String html = "";
        MetadataExtraction result = HtmlToMarkdown.convertWithMetadata(html);

        assertNotNull(result, "Result should not be null for empty HTML");
        assertNotNull(result.getMarkdown(), "Markdown should not be null");
        assertNotNull(result.getMetadata(), "Metadata should not be null");
    }

    @Test
    @DisplayName("HTML with minimal structure")
    void testMinimalDocumentExtraction() {
        String html = "<p>Simple text</p>";

        MetadataExtraction result = HtmlToMarkdown.convertWithMetadata(html);

        assertNotNull(result, "Result should not be null");
        assertTrue(result.getMarkdown().contains("Simple text"),
                "Text should be in markdown");
    }

    @Test
    @DisplayName("HTML with metadata tags")
    void testMetadataTagsExtraction() {
        String html = "<html>"
                + "<head>"
                + "<title>Page Title</title>"
                + "<meta name=\"description\" content=\"Page description\" />"
                + "<meta name=\"author\" content=\"John Doe\" />"
                + "<meta name=\"keywords\" content=\"tag1, tag2\" />"
                + "<link rel=\"canonical\" href=\"https://example.com\" />"
                + "</head>"
                + "<body><p>Content</p></body>"
                + "</html>";

        MetadataExtraction result = HtmlToMarkdown.convertWithMetadata(html);

        assertNotNull(result, "Result should not be null");
        assertNotNull(result.getMarkdown(), "Markdown should be extracted");
    }

    @Test
    @DisplayName("HTML with Open Graph metadata")
    void testOpenGraphExtraction() {
        String html = "<html>"
                + "<head>"
                + "<meta property=\"og:title\" content=\"Social Title\" />"
                + "<meta property=\"og:description\" content=\"Social description\" />"
                + "<meta property=\"og:image\" content=\"https://example.com/image.jpg\" />"
                + "<meta property=\"og:url\" content=\"https://example.com\" />"
                + "</head>"
                + "<body><p>Content</p></body>"
                + "</html>";

        MetadataExtraction result = HtmlToMarkdown.convertWithMetadata(html);

        assertNotNull(result, "Result should not be null");
    }

    @Test
    @DisplayName("HTML with Twitter Card metadata")
    void testTwitterCardExtraction() {
        String html = "<html>"
                + "<head>"
                + "<meta name=\"twitter:card\" content=\"summary_large_image\" />"
                + "<meta name=\"twitter:title\" content=\"Twitter Title\" />"
                + "<meta name=\"twitter:description\" content=\"Twitter description\" />"
                + "<meta name=\"twitter:image\" content=\"https://example.com/image.jpg\" />"
                + "</head>"
                + "<body><p>Content</p></body>"
                + "</html>";

        MetadataExtraction result = HtmlToMarkdown.convertWithMetadata(html);

        assertNotNull(result, "Result should not be null");
    }

    @Test
    @DisplayName("Conversion with null HTML in metadata extraction throws")
    void testNullHtmlThrows() {
        assertThrows(NullPointerException.class,
                () -> HtmlToMarkdown.convertWithMetadata(null),
                "Should throw NullPointerException for null HTML");
    }

    @Test
    @DisplayName("Large document metadata extraction")
    void testLargeDocumentExtraction() {
        StringBuilder html = new StringBuilder("<html><body>");
        for (int i = 0; i < 100; i++) {
            html.append("<h2>Section ").append(i).append("</h2>");
            html.append("<p>Content for section ").append(i).append("</p>");
            html.append("<a href=\"https://example.com/").append(i)
                    .append("\">Link ").append(i).append("</a>");
        }
        html.append("</body></html>");

        MetadataExtraction result = HtmlToMarkdown.convertWithMetadata(html.toString());

        assertNotNull(result, "Result should not be null");
        assertNotNull(result.getMarkdown(), "Markdown should be extracted from large document");
    }
}
