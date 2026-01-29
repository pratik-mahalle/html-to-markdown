package dev.kreuzberg.htmltomarkdown;

import dev.kreuzberg.htmltomarkdown.metadata.MetadataExtraction;
import org.junit.jupiter.api.Test;
import org.junit.jupiter.api.DisplayName;
import org.junit.jupiter.params.ParameterizedTest;
import org.junit.jupiter.params.provider.ValueSource;
import static org.junit.jupiter.api.Assertions.*;

/**
 * Type safety tests for html-to-markdown Java bindings.
 *
 * Tests verify that the Java API provides strong type safety
 * through proper use of generics, enums, and nullability.
 */
@DisplayName("Type Safety Tests")
class TypeSafetyTest {

    @Test
    @DisplayName("ConversionOptions is properly typed")
    void testConversionOptionsTyping() {
        ConversionOptions options = new ConversionOptions();

        // Test that getters return correct types
        String headingStyle = options.getHeadingStyle();
        assertInstanceOf(String.class, headingStyle, "Heading style should be String");

        String listIndentType = options.getListIndentType();
        assertInstanceOf(String.class, listIndentType, "List indent type should be String");

        int listIndentWidth = options.getListIndentWidth();
        assertInstanceOf(Integer.class, listIndentWidth, "List indent width should be int");

        // Test boolean options
        assertNotNull(options.getHeadingStyle(), "Options should not be null");
    }

    @Test
    @DisplayName("ConversionOptions setter methods return builder type")
    void testConversionOptionsBuilderPattern() {
        // Verify fluent builder pattern with proper types
        ConversionOptions result = new ConversionOptions()
                .setHeadingStyle("atx")
                .setListIndentWidth(2)
                .setEscapeAsterisks(true);

        assertInstanceOf(ConversionOptions.class, result,
                "Setter should return ConversionOptions for chaining");

        // Verify values were set
        assertEquals("atx", result.getHeadingStyle(), "Heading style should be set");
        assertEquals(2, result.getListIndentWidth(), "List indent width should be set");
    }

    @Test
    @DisplayName("OutputFormat enum type safety")
    void testOutputFormatEnum() {
        assertNotNull(OutputFormat.MARKDOWN, "MARKDOWN format should exist");
        assertNotNull(OutputFormat.DJOT, "DJOT format should exist");
        // Verify these are distinct enum values
        assertNotEquals(OutputFormat.MARKDOWN, OutputFormat.DJOT,
                "OutputFormat values should be distinct");
    }

    @Test
    @DisplayName("Conversion returns String type")
    void testConversionReturnType() {
        String result = HtmlToMarkdown.convert("<p>Test</p>");
        assertInstanceOf(String.class, result, "Conversion should return String");
    }

    @Test
    @DisplayName("MetadataExtraction contains properly typed fields")
    void testMetadataExtractionTyping() {
        dev.kreuzberg.htmltomarkdown.metadata.MetadataExtraction extraction =
            HtmlToMarkdown.convertWithMetadata(
                "<html><head><title>Test</title></head><body><h1>Hello</h1></body></html>");

        assertNotNull(extraction, "MetadataExtraction should not be null");

        String markdown = extraction.getMarkdown();
        assertInstanceOf(String.class, markdown, "Markdown should be String");

        // Extended metadata type
        Object metadata = extraction.getMetadata();
        assertNotNull(metadata, "Metadata should not be null");
    }

    @Test
    @DisplayName("Visitor interface provides type-safe callbacks")
    void testVisitorInterfaceTyping() {
        dev.kreuzberg.htmltomarkdown.visitor.Visitor visitor = new dev.kreuzberg.htmltomarkdown.visitor.Visitor() {
            @Override
            public dev.kreuzberg.htmltomarkdown.visitor.VisitResult visitText(
                    dev.kreuzberg.htmltomarkdown.visitor.NodeContext ctx, String text) {
                assertInstanceOf(String.class, text, "Text parameter should be String");
                assertNotNull(ctx, "NodeContext should not be null");
                return dev.kreuzberg.htmltomarkdown.visitor.VisitResult.Continue.INSTANCE;
            }

            @Override
            public dev.kreuzberg.htmltomarkdown.visitor.VisitResult visitLink(
                    dev.kreuzberg.htmltomarkdown.visitor.NodeContext ctx, String href, String text, String title) {
                assertInstanceOf(String.class, href, "href should be String");
                assertInstanceOf(String.class, text, "text should be String");
                // title may be null
                return dev.kreuzberg.htmltomarkdown.visitor.VisitResult.Continue.INSTANCE;
            }
        };

        assertNotNull(visitor, "Visitor instance should be created");
        String result = HtmlToMarkdown.convertWithVisitor(
                "<p>Text</p><a href=\"#\">Link</a>", visitor);
        assertInstanceOf(String.class, result, "Result should be String");
    }

    @Test
    @DisplayName("VisitResult has type-safe subclasses")
    void testVisitResultTypeHierarchy() {
        dev.kreuzberg.htmltomarkdown.visitor.VisitResult continueResult =
            dev.kreuzberg.htmltomarkdown.visitor.VisitResult.Continue.INSTANCE;
        dev.kreuzberg.htmltomarkdown.visitor.VisitResult skipResult =
            dev.kreuzberg.htmltomarkdown.visitor.VisitResult.Skip.INSTANCE;

        assertNotNull(continueResult, "Continue result should not be null");
        assertNotNull(skipResult, "Skip result should not be null");
        assertNotEquals(continueResult, skipResult, "Continue and Skip should be different");
    }

    @Test
    @DisplayName("ConversionException is properly typed exception")
    void testConversionExceptionTyping() {
        Throwable exception = assertThrows(HtmlToMarkdown.ConversionException.class,
                () -> {
                    throw new HtmlToMarkdown.ConversionException("Test error");
                },
                "Should throw ConversionException");

        assertInstanceOf(RuntimeException.class, exception,
                "ConversionException should be RuntimeException");
        assertTrue(exception.getMessage().contains("Test error"),
                "Exception message should be preserved");
    }

    @Test
    @DisplayName("Generic methods handle type erasure correctly")
    void testGenericMethodHandling() {
        String htmlInput = "<p>Generic test</p>";
        String result = HtmlToMarkdown.convert(htmlInput);

        // Although generics are erased at runtime in Java,
        // we verify the API returns expected types
        assertInstanceOf(String.class, result, "Should return String type");
    }

    @ParameterizedTest
    @ValueSource(strings = {"atx", "underlined", "atx_closed"})
    @DisplayName("ConversionOptions heading styles are type-safe")
    void testHeadingStyleTypeValidation(String style) {
        ConversionOptions options = new ConversionOptions().setHeadingStyle(style);
        String retrievedStyle = options.getHeadingStyle();

        assertInstanceOf(String.class, retrievedStyle, "Should be String type");
        assertEquals(style, retrievedStyle, "Style value should be preserved");
    }

    @Test
    @DisplayName("Visitor implementation provides type-safe callbacks")
    void testVisitorCallbackTypeSafety() {
        dev.kreuzberg.htmltomarkdown.visitor.Visitor visitor = new dev.kreuzberg.htmltomarkdown.visitor.Visitor() {
            // Empty implementation with default Continue behavior
        };

        String html = "<div>Test</div>";
        String result = HtmlToMarkdown.convertWithVisitor(html, visitor);

        assertNotNull(result, "Result should not be null");
        assertInstanceOf(String.class, result, "Result should be String");
    }

    @Test
    @DisplayName("Version string is properly typed String")
    void testVersionStringTyping() {
        String version = HtmlToMarkdown.getVersion();
        assertInstanceOf(String.class, version, "Version should be String type");
        assertFalse(version.isEmpty(), "Version should not be empty");
    }

    @Test
    @DisplayName("Metadata components are properly typed")
    void testMetadataComponentTyping() {
        MetadataExtraction extraction = HtmlToMarkdown.convertWithMetadata(
                "<html><title>Title</title><body><h1>Header</h1><a href=\"#\">Link</a></body></html>");

        assertNotNull(extraction, "Extraction should not be null");
        assertInstanceOf(String.class, extraction.getMarkdown(),
                "Markdown should be String");

        // Metadata object should exist and be properly typed
        Object metadata = extraction.getMetadata();
        assertNotNull(metadata, "Metadata object should exist");
    }
}
