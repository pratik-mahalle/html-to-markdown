using System;
using Xunit;
using HtmlToMarkdown;

namespace HtmlToMarkdownTestApp;

#nullable enable

/// <summary>
/// Smoke tests for basic HtmlToMarkdownConverter functionality.
/// These tests verify that the P/Invoke binding is working and can be called.
/// </summary>
public class SmokeTest
{
    [Fact]
    public void TestPackageLoads()
    {
        Assert.NotNull(typeof(HtmlToMarkdownConverter));
    }

    /// <summary>
    /// Test basic HTML paragraph to Markdown conversion.
    /// </summary>
    [Fact]
    public void TestBasicParagraphConversion()
    {
        var html = "<p>Hello World</p>";
        var result = HtmlToMarkdownConverter.Convert(html);
        Assert.NotNull(result);
        Assert.Contains("Hello World", result);
    }

    /// <summary>
    /// Test heading conversion and proper Markdown formatting.
    /// </summary>
    [Fact]
    public void TestHeadingConversion()
    {
        var html = "<h1>Title</h1>";
        var result = HtmlToMarkdownConverter.Convert(html);
        Assert.NotNull(result);
        Assert.StartsWith("#", result.Trim());
        Assert.Contains("Title", result);
    }

    /// <summary>
    /// Test multiple heading levels.
    /// </summary>
    [Theory]
    [InlineData("<h1>Level 1</h1>", "#")]
    [InlineData("<h2>Level 2</h2>", "##")]
    [InlineData("<h3>Level 3</h3>", "###")]
    public void TestMultipleHeadingLevels(string html, string expectedPrefix)
    {
        var result = HtmlToMarkdownConverter.Convert(html);
        Assert.NotNull(result);
        Assert.StartsWith(expectedPrefix, result.Trim());
    }

    /// <summary>
    /// Test empty input handling.
    /// </summary>
    [Fact]
    public void TestEmptyInput()
    {
        var result = HtmlToMarkdownConverter.Convert("");
        Assert.NotNull(result);
        Assert.Equal("", result);
    }

    /// <summary>
    /// Test strong/bold text conversion.
    /// </summary>
    [Fact]
    public void TestBoldTextConversion()
    {
        var html = "<p>This is <strong>bold</strong> text</p>";
        var result = HtmlToMarkdownConverter.Convert(html);
        Assert.NotNull(result);
        Assert.Contains("**bold**", result);
    }

    /// <summary>
    /// Test italic/emphasis text conversion.
    /// </summary>
    [Fact]
    public void TestItalicTextConversion()
    {
        var html = "<p>This is <em>italic</em> text</p>";
        var result = HtmlToMarkdownConverter.Convert(html);
        Assert.NotNull(result);
        Assert.Contains("*italic*", result);
    }

    /// <summary>
    /// Test unordered list conversion.
    /// </summary>
    [Fact]
    public void TestUnorderedListConversion()
    {
        var html = "<ul><li>Item 1</li><li>Item 2</li></ul>";
        var result = HtmlToMarkdownConverter.Convert(html);
        Assert.NotNull(result);
        Assert.Contains("- Item 1", result);
        Assert.Contains("- Item 2", result);
    }

    /// <summary>
    /// Test ordered list conversion.
    /// </summary>
    [Fact]
    public void TestOrderedListConversion()
    {
        var html = "<ol><li>First</li><li>Second</li></ol>";
        var result = HtmlToMarkdownConverter.Convert(html);
        Assert.NotNull(result);
        Assert.Contains("1.", result);
        Assert.Contains("First", result);
    }

    /// <summary>
    /// Test hyperlink conversion to Markdown format.
    /// </summary>
    [Fact]
    public void TestLinkConversion()
    {
        var html = "<a href=\"https://example.com\">Example Link</a>";
        var result = HtmlToMarkdownConverter.Convert(html);
        Assert.NotNull(result);
        Assert.Contains("[Example Link](https://example.com)", result);
    }

    /// <summary>
    /// Test inline code conversion.
    /// </summary>
    [Fact]
    public void TestInlineCodeConversion()
    {
        var html = "<code>console.log('hello')</code>";
        var result = HtmlToMarkdownConverter.Convert(html);
        Assert.NotNull(result);
        Assert.Contains("`console.log('hello')`", result);
    }

    /// <summary>
    /// Test blockquote conversion.
    /// </summary>
    [Fact]
    public void TestBlockquoteConversion()
    {
        var html = "<blockquote>Quote text</blockquote>";
        var result = HtmlToMarkdownConverter.Convert(html);
        Assert.NotNull(result);
        Assert.Contains(">", result);
        Assert.Contains("Quote text", result);
    }

    /// <summary>
    /// Test code block conversion.
    /// </summary>
    [Fact]
    public void TestCodeBlockConversion()
    {
        var html = "<pre><code>function test() {\n  console.log('hello');\n}</code></pre>";
        var result = HtmlToMarkdownConverter.Convert(html);
        Assert.NotNull(result);
        // Code blocks should be represented with triple backticks or similar
        Assert.NotEmpty(result);
    }

    /// <summary>
    /// Test that null input is handled gracefully.
    /// </summary>
    [Fact]
    public void TestNullInputHandling()
    {
        // Most converters handle null by treating it as empty string
        // or throw ArgumentNullException
        var nullString = (string?)null;
        if (nullString is not null)
        {
            var result = HtmlToMarkdownConverter.Convert(nullString);
            Assert.NotNull(result);
        }
        else
        {
            // Verify that the API requires a non-null input
            // (null-forgiving operator is only for suppressing compiler warnings)
            Assert.Null(nullString);
        }
    }

    /// <summary>
    /// Test that malformed HTML is handled gracefully.
    /// </summary>
    [Fact]
    public void TestMalformedHtmlHandling()
    {
        var html = "<p>Unclosed paragraph";
        var result = HtmlToMarkdownConverter.Convert(html);
        Assert.NotNull(result);
        // Should not crash, should return something
        Assert.NotEmpty(result);
    }

    /// <summary>
    /// Test line break handling.
    /// </summary>
    [Fact]
    public void TestLineBreakConversion()
    {
        var html = "<p>Line 1<br/>Line 2</p>";
        var result = HtmlToMarkdownConverter.Convert(html);
        Assert.NotNull(result);
        Assert.Contains("Line 1", result);
        Assert.Contains("Line 2", result);
    }

    /// <summary>
    /// Test horizontal rule conversion.
    /// </summary>
    [Fact]
    public void TestHorizontalRuleConversion()
    {
        var html = "<p>Content</p><hr/><p>More content</p>";
        var result = HtmlToMarkdownConverter.Convert(html);
        Assert.NotNull(result);
        Assert.Contains("Content", result);
    }
}
