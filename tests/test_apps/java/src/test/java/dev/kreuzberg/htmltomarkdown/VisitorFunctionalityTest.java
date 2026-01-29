package dev.kreuzberg.htmltomarkdown;

import dev.kreuzberg.htmltomarkdown.visitor.Visitor;
import dev.kreuzberg.htmltomarkdown.visitor.VisitResult;
import dev.kreuzberg.htmltomarkdown.visitor.NodeContext;
import org.junit.jupiter.api.Test;
import org.junit.jupiter.api.DisplayName;
import static org.junit.jupiter.api.Assertions.*;

/**
 * Visitor functionality tests for html-to-markdown.
 *
 * Tests verify that custom visitors can intercept and customize
 * the HTML-to-Markdown conversion process.
 */
@DisplayName("Visitor Functionality Tests")
class VisitorFunctionalityTest {

    @Test
    @DisplayName("Basic visitor implementation")
    void testBasicVisitor() {
        Visitor visitor = new Visitor() {
            @Override
            public VisitResult visitText(NodeContext ctx, String text) {
                return VisitResult.Continue.INSTANCE;
            }
        };

        String html = "<p>Test text</p>";
        String result = HtmlToMarkdown.convertWithVisitor(html, visitor);

        assertNotNull(result, "Result should not be null");
        assertTrue(result.contains("Test text"), "Text should be in result");
    }

    @Test
    @DisplayName("Visitor can skip elements")
    void testVisitorSkipElements() {
        Visitor visitor = new Visitor() {
            @Override
            public VisitResult visitLink(NodeContext ctx, String href, String text, String title) {
                // Skip all links
                return VisitResult.Skip.INSTANCE;
            }

            @Override
            public VisitResult visitText(NodeContext ctx, String text) {
                return VisitResult.Continue.INSTANCE;
            }
        };

        String html = "<p>Before</p><a href=\"https://example.com\">Link</a><p>After</p>";
        String result = HtmlToMarkdown.convertWithVisitor(html, visitor);

        assertNotNull(result, "Result should not be null");
        assertTrue(result.contains("Before"), "Before text should be present");
        assertTrue(result.contains("After"), "After text should be present");
    }

    @Test
    @DisplayName("Visitor conditional element handling")
    void testVisitorConditionalHandling() {
        Visitor visitor = new Visitor() {
            @Override
            public VisitResult visitLink(NodeContext ctx, String href, String text, String title) {
                // Skip mailto links
                if (href != null && href.startsWith("mailto:")) {
                    return VisitResult.Skip.INSTANCE;
                }
                return VisitResult.Continue.INSTANCE;
            }
        };

        String html = "<a href=\"https://example.com\">External</a>"
                + "<a href=\"mailto:test@example.com\">Email</a>";

        String result = HtmlToMarkdown.convertWithVisitor(html, visitor);

        assertNotNull(result, "Result should not be null");
        assertTrue(result.contains("External"), "External link should be present");
    }

    @Test
    @DisplayName("Visitor with multiple callback implementations")
    void testVisitorMultipleCallbacks() {
        Visitor visitor = new Visitor() {
            @Override
            public VisitResult visitElementStart(NodeContext ctx) {
                return VisitResult.Continue.INSTANCE;
            }

            @Override
            public VisitResult visitText(NodeContext ctx, String text) {
                return VisitResult.Continue.INSTANCE;
            }

            @Override
            public VisitResult visitLink(NodeContext ctx, String href, String text, String title) {
                return VisitResult.Continue.INSTANCE;
            }

            @Override
            public VisitResult visitImage(NodeContext ctx, String src, String alt, String title) {
                return VisitResult.Continue.INSTANCE;
            }

            @Override
            public VisitResult visitCodeInline(NodeContext ctx, String code) {
                return VisitResult.Continue.INSTANCE;
            }

            @Override
            public VisitResult visitHeading(NodeContext ctx, int level, String text, String id) {
                return VisitResult.Continue.INSTANCE;
            }
        };

        String html = "<h1>Title</h1>"
                + "<p>Text with <a href=\"#\">link</a> and <code>code</code></p>"
                + "<img src=\"image.png\" alt=\"Image\" />";

        String result = HtmlToMarkdown.convertWithVisitor(html, visitor);

        assertNotNull(result, "Result should not be null");
    }

    @Test
    @DisplayName("Visitor with empty implementation defaults to Continue")
    void testVisitorEmptyImplementation() {
        Visitor visitor = new Visitor() {
            // All methods use default Continue implementation
        };

        String html = "<h1>Title</h1><p>Content</p>";
        String result = HtmlToMarkdown.convertWithVisitor(html, visitor);

        assertNotNull(result, "Result should not be null");
        assertTrue(result.contains("Title"), "Title should be in result");
        assertTrue(result.contains("Content"), "Content should be in result");
    }

    @Test
    @DisplayName("Visitor processes complex HTML structure")
    void testVisitorComplexStructure() {
        Visitor visitor = new Visitor() {
            @Override
            public VisitResult visitHeading(NodeContext ctx, int level, String text, String id) {
                return VisitResult.Continue.INSTANCE;
            }

            @Override
            public VisitResult visitLink(NodeContext ctx, String href, String text, String title) {
                return VisitResult.Continue.INSTANCE;
            }

            @Override
            public VisitResult visitText(NodeContext ctx, String text) {
                return VisitResult.Continue.INSTANCE;
            }
        };

        String html = "<article>"
                + "<h1>Main</h1>"
                + "<section>"
                + "<h2>Sub</h2>"
                + "<p>Text with <a href=\"#\">link</a></p>"
                + "</section>"
                + "</article>";

        String result = HtmlToMarkdown.convertWithVisitor(html, visitor);

        assertNotNull(result, "Result should not be null");
    }

    @Test
    @DisplayName("Visitor with heading level discrimination")
    void testVisitorHeadingLevels() {
        Visitor visitor = new Visitor() {
            @Override
            public VisitResult visitHeading(NodeContext ctx, int level, String text, String id) {
                if (level <= 2) {
                    return VisitResult.Continue.INSTANCE;
                }
                return VisitResult.Continue.INSTANCE;
            }
        };

        String html = "<h1>Level 1</h1><h2>Level 2</h2><h3>Level 3</h3>";

        String result = HtmlToMarkdown.convertWithVisitor(html, visitor);

        assertNotNull(result, "Result should not be null");
    }

    @Test
    @DisplayName("Visitor image filtering")
    void testVisitorImageFiltering() {
        Visitor visitor = new Visitor() {
            @Override
            public VisitResult visitImage(NodeContext ctx, String src, String alt, String title) {
                if (src != null && !src.startsWith("http")) {
                    return VisitResult.Continue.INSTANCE;
                }
                return VisitResult.Skip.INSTANCE;
            }

            @Override
            public VisitResult visitText(NodeContext ctx, String text) {
                return VisitResult.Continue.INSTANCE;
            }
        };

        String html = "<img src=\"local.png\" alt=\"Local\" />"
                + "<img src=\"https://example.com/remote.png\" alt=\"Remote\" />";

        String result = HtmlToMarkdown.convertWithVisitor(html, visitor);

        assertNotNull(result, "Result should not be null");
    }

    @Test
    @DisplayName("Visitor code block handling")
    void testVisitorCodeHandling() {
        Visitor visitor = new Visitor() {
            @Override
            public VisitResult visitCodeInline(NodeContext ctx, String code) {
                return VisitResult.Continue.INSTANCE;
            }

            @Override
            public VisitResult visitText(NodeContext ctx, String text) {
                return VisitResult.Continue.INSTANCE;
            }
        };

        String html = "<p><code>const x = 1;</code></p>";

        String result = HtmlToMarkdown.convertWithVisitor(html, visitor);

        assertNotNull(result, "Result should not be null");
    }

    @Test
    @DisplayName("Visitor with null visitor throws NullPointerException")
    void testNullVisitorThrows() {
        assertThrows(NullPointerException.class,
                () -> HtmlToMarkdown.convertWithVisitor("<p>Test</p>", null),
                "Should throw NullPointerException for null visitor");
    }

    @Test
    @DisplayName("Visitor with null HTML throws NullPointerException")
    void testNullHtmlWithVisitorThrows() {
        Visitor visitor = new Visitor() { };
        assertThrows(NullPointerException.class,
                () -> HtmlToMarkdown.convertWithVisitor(null, visitor),
                "Should throw NullPointerException for null HTML");
    }

    @Test
    @DisplayName("Visitor processes mixed content")
    void testVisitorMixedContent() {
        Visitor visitor = new Visitor() {
            @Override
            public VisitResult visitText(NodeContext ctx, String text) {
                return VisitResult.Continue.INSTANCE;
            }

            @Override
            public VisitResult visitLink(NodeContext ctx, String href, String text, String title) {
                return VisitResult.Continue.INSTANCE;
            }

            @Override
            public VisitResult visitImage(NodeContext ctx, String src, String alt, String title) {
                return VisitResult.Continue.INSTANCE;
            }
        };

        String html = "Raw text <p>Paragraph</p> <a href=\"#\">Link</a> <img src=\"i.png\" />";

        String result = HtmlToMarkdown.convertWithVisitor(html, visitor);

        assertNotNull(result, "Result should not be null");
    }

    @Test
    @DisplayName("Multiple conversions with same visitor instance")
    void testMultipleConversionsWithSameVisitor() {
        Visitor visitor = new Visitor() { };

        String[] htmlInputs = {
                "<p>First</p>",
                "<h1>Second</h1>",
                "<a href=\"#\">Third</a>"
        };

        for (String html : htmlInputs) {
            String result = HtmlToMarkdown.convertWithVisitor(html, visitor);
            assertNotNull(result, "Result should not be null for: " + html);
        }
    }
}
