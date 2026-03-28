using Xunit;

namespace HtmlToMarkdown.Tests;

public class ConverterTests
{
    [Fact]
    public void Convert_SimpleHeading_ReturnsMarkdown()
    {
        var html = "<h1>Hello World</h1>";
        var result = HtmlToMarkdownConverter.Convert(html);
        Assert.Contains("Hello World", result.Content);
    }

    [Fact]
    public void Convert_EmptyString_ReturnsEmptyString()
    {
        var result = HtmlToMarkdownConverter.Convert("");
        Assert.Equal("", result.Content ?? "");
    }

    [Fact]
    public void Convert_NullInput_ThrowsArgumentNullException()
    {
        string? html = null;
        Assert.Throws<ArgumentNullException>(() =>
            HtmlToMarkdownConverter.Convert(html!));
    }

    [Fact]
    public void Convert_Paragraph_ReturnsMarkdown()
    {
        var html = "<p>This is a test.</p>";
        var result = HtmlToMarkdownConverter.Convert(html);
        Assert.Contains("This is a test", result.Content);
    }

    [Fact]
    public void Convert_StrongText_ReturnsMarkdown()
    {
        var html = "<strong>Bold text</strong>";
        var result = HtmlToMarkdownConverter.Convert(html);
        Assert.Contains("Bold text", result.Content);
    }

    [Fact]
    public void Convert_List_ReturnsMarkdown()
    {
        var html = "<ul><li>Item 1</li><li>Item 2</li></ul>";
        var result = HtmlToMarkdownConverter.Convert(html);
        Assert.Contains("Item 1", result.Content);
        Assert.Contains("Item 2", result.Content);
    }

    [Fact]
    public void GetVersion_ReturnsNonEmptyString()
    {
        var version = HtmlToMarkdownConverter.GetVersion();
        Assert.False(string.IsNullOrEmpty(version));
        Assert.NotEqual("unknown", version);
    }
}
