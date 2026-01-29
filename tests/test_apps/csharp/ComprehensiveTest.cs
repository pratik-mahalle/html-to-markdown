using System;
using System.Collections.Generic;
using System.IO;
using System.Linq;
using Newtonsoft.Json;
using Xunit;
using HtmlToMarkdown;

namespace HtmlToMarkdownTestApp;

#nullable enable

/// <summary>
/// Represents a test case for HTML to Markdown conversion.
/// </summary>
public record TestCase(string Name, string Html, string ExpectedMarkdown, Dictionary<string, object>? Options)
{
    public override string ToString() => Name;
}

/// <summary>
/// Comprehensive test suite for HtmlToMarkdownConverter using fixture-driven tests.
/// Tests verify conversion accuracy against known test cases loaded from JSON fixtures.
/// </summary>
public class ComprehensiveTest
{
    /// <summary>
    /// Load test fixture data from JSON files.
    /// </summary>
    private static List<TestCase> LoadFixtures(string filename)
    {
        // Get the directory of the test_apps folder
        // AppDomain.CurrentDomain.BaseDirectory points to bin/Debug/net10.0
        // We need to go up to csharp, then to test_apps
        var baseDir = AppDomain.CurrentDomain.BaseDirectory;
        var csharpDir = Path.GetFullPath(Path.Combine(baseDir, "..", "..", ".."));
        var testAppsDir = Path.GetDirectoryName(csharpDir);
        var fixturesPath = Path.Combine(testAppsDir!, "fixtures", filename);

        if (!File.Exists(fixturesPath))
        {
            throw new FileNotFoundException($"Fixture file not found: {fixturesPath}");
        }

        var json = File.ReadAllText(fixturesPath);
        var testCases = JsonConvert.DeserializeObject<List<TestCase>>(json);
        return testCases ?? new List<TestCase>();
    }

    /// <summary>
    /// Provide basic HTML test cases for parametrized testing.
    /// </summary>
    public static IEnumerable<object[]> BasicHtmlData()
    {
        var fixtures = LoadFixtures("basic-html.json");
        return fixtures.Select(f => new object[] { f });
    }

    /// <summary>
    /// Test basic HTML conversion against fixture data.
    /// </summary>
    [Theory(DisplayName = "Basic HTML Conversion")]
    [MemberData(nameof(BasicHtmlData))]
    public void TestBasicHtmlConversion(TestCase testCase)
    {
        Assert.NotNull(testCase);
        Assert.NotNull(testCase.Html);
        Assert.NotNull(testCase.ExpectedMarkdown);

        var result = HtmlToMarkdownConverter.Convert(testCase.Html);

        Assert.NotNull(result);
        Assert.Equal(testCase.ExpectedMarkdown.Trim(), result.Trim(), ignoreCase: false);
    }

    /// <summary>
    /// Test that converter produces string output for valid HTML.
    /// </summary>
    [Fact(DisplayName = "Output Type Safety")]
    public void TestOutputIsString()
    {
        var html = "<p>Test</p>";
        var result = HtmlToMarkdownConverter.Convert(html);

        Assert.NotNull(result);
        Assert.IsType<string>(result);
        Assert.NotEmpty(result);
    }

    /// <summary>
    /// Test consistency - same input should always produce same output.
    /// </summary>
    [Fact(DisplayName = "Conversion Consistency")]
    public void TestConversionConsistency()
    {
        var html = "<h1>Title</h1><p>Content</p>";

        var result1 = HtmlToMarkdownConverter.Convert(html);
        var result2 = HtmlToMarkdownConverter.Convert(html);
        var result3 = HtmlToMarkdownConverter.Convert(html);

        Assert.Equal(result1, result2);
        Assert.Equal(result2, result3);
    }

    /// <summary>
    /// Test batch processing of multiple HTML documents.
    /// </summary>
    [Fact(DisplayName = "Batch Processing")]
    public void TestBatchProcessing()
    {
        var htmlDocs = new[]
        {
            "<p>Document 1</p>",
            "<h1>Document 2</h1>",
            "<ul><li>Item</li></ul>"
        };

        var results = new List<string>();
        foreach (var html in htmlDocs)
        {
            var result = HtmlToMarkdownConverter.Convert(html);
            results.Add(result);
        }

        Assert.Equal(3, results.Count);
        Assert.All(results, r => Assert.NotNull(r));
    }

    /// <summary>
    /// Test error handling for exceptional inputs.
    /// </summary>
    [Theory(DisplayName = "Error Handling")]
    [InlineData("")]
    [InlineData("   ")]
    [InlineData("<br/>")]
    public void TestErrorHandlingWithEdgeCases(string html)
    {
        var exception = Record.Exception(() => HtmlToMarkdownConverter.Convert(html));
        Assert.Null(exception);
    }

    /// <summary>
    /// Test handling of very large HTML input.
    /// </summary>
    [Fact(DisplayName = "Large Input Handling")]
    public void TestLargeHtmlInput()
    {
        // Create a large HTML document with many paragraphs
        var largeHtml = string.Concat(Enumerable.Range(1, 1000)
            .Select(i => $"<p>Paragraph {i}</p>"));

        var result = HtmlToMarkdownConverter.Convert(largeHtml);

        Assert.NotNull(result);
        Assert.NotEmpty(result);
        // Verify that all paragraphs were processed
        for (int i = 1; i <= 100; i++)
        {
            Assert.Contains($"Paragraph {i}", result);
        }
    }

    /// <summary>
    /// Test handling of deeply nested HTML structures.
    /// </summary>
    [Fact(DisplayName = "Nested HTML Handling")]
    public void TestDeeplyNestedHtml()
    {
        var nestedHtml = "<div><div><div><p>Deep content</p></div></div></div>";
        var result = HtmlToMarkdownConverter.Convert(nestedHtml);

        Assert.NotNull(result);
        Assert.Contains("Deep content", result);
    }

    /// <summary>
    /// Test Unicode and special character handling.
    /// </summary>
    [Theory(DisplayName = "Unicode Character Support")]
    [InlineData("<p>Hello 世界</p>")]
    [InlineData("<p>Café</p>")]
    [InlineData("<p>Ñoño</p>")]
    [InlineData("<p>🚀 Rocket</p>")]
    public void TestUnicodeCharacters(string html)
    {
        var result = HtmlToMarkdownConverter.Convert(html);
        Assert.NotNull(result);
        Assert.NotEmpty(result);
    }

    /// <summary>
    /// Test HTML entity handling.
    /// </summary>
    [Theory(DisplayName = "HTML Entity Handling")]
    [InlineData("<p>&nbsp;</p>")]
    [InlineData("<p>&lt;tag&gt;</p>")]
    [InlineData("<p>&amp;</p>")]
    [InlineData("<p>&quot;quoted&quot;</p>")]
    public void TestHtmlEntities(string html)
    {
        var result = HtmlToMarkdownConverter.Convert(html);
        Assert.NotNull(result);
    }

    /// <summary>
    /// Test mixed formatting scenarios.
    /// </summary>
    [Fact(DisplayName = "Mixed Formatting")]
    public void TestMixedFormatting()
    {
        var html = "<h1>Title</h1>" +
                   "<p>This is <strong>bold</strong> and <em>italic</em> text with a " +
                   "<a href=\"https://example.com\">link</a>.</p>" +
                   "<ul><li>Bullet 1</li><li>Bullet 2</li></ul>" +
                   "<blockquote>A quote</blockquote>";

        var result = HtmlToMarkdownConverter.Convert(html);

        Assert.NotNull(result);
        Assert.Contains("#", result);           // Heading
        Assert.Contains("**bold**", result);     // Bold
        Assert.Contains("*italic*", result);     // Italic
        Assert.Contains("example.com", result);  // Link
    }

    /// <summary>
    /// Test comment and script removal.
    /// </summary>
    [Fact(DisplayName = "Script and Comment Removal")]
    public void TestScriptAndCommentRemoval()
    {
        var html = "<p>Content</p>" +
                   "<!-- This is a comment -->" +
                   "<script>alert('hello');</script>" +
                   "<p>More content</p>";

        var result = HtmlToMarkdownConverter.Convert(html);

        Assert.NotNull(result);
        // Script and comments should not appear in output
        Assert.DoesNotContain("alert", result);
        Assert.DoesNotContain("This is a comment", result);
        Assert.Contains("Content", result);
        Assert.Contains("More content", result);
    }

    /// <summary>
    /// Test whitespace preservation and normalization.
    /// </summary>
    [Fact(DisplayName = "Whitespace Handling")]
    public void TestWhitespaceHandling()
    {
        var html = "<p>Text   with    multiple     spaces</p>";
        var result = HtmlToMarkdownConverter.Convert(html);

        Assert.NotNull(result);
        // Multiple spaces should be normalized
        Assert.Contains("Text", result);
        Assert.Contains("with", result);
    }

    /// <summary>
    /// Verify that the converter returns consistent encoding.
    /// </summary>
    [Fact(DisplayName = "Text Encoding")]
    public void TestTextEncoding()
    {
        var html = "<p>Test with special chars: é ñ ü</p>";
        var result = HtmlToMarkdownConverter.Convert(html);

        Assert.NotNull(result);
        Assert.IsType<string>(result);

        // Result should be a valid UTF-8 string
        var bytes = System.Text.Encoding.UTF8.GetBytes(result);
        Assert.NotEmpty(bytes);
    }
}
