using Xunit;

namespace HtmlToMarkdown.Tests;

public class ConvertWithTablesTests
{
    [Fact]
    public void ExtractsSimpleTable()
    {
        var html = "<table><thead><tr><th>Name</th><th>Age</th></tr></thead>" +
            "<tbody><tr><td>Alice</td><td>30</td></tr></tbody></table>";

        var result = HtmlToMarkdownConverter.ConvertWithTables(html);

        Assert.NotNull(result);
        Assert.NotEmpty(result.Content);
        Assert.Single(result.Tables);

        var table = result.Tables[0];
        Assert.True(table.Cells.Count >= 2);
        Assert.Equal("Name", table.Cells[0][0]);
        Assert.Equal("Age", table.Cells[0][1]);
        Assert.Equal("Alice", table.Cells[1][0]);
        Assert.Equal("30", table.Cells[1][1]);
        Assert.NotEmpty(table.Markdown);
        Assert.NotEmpty(table.IsHeaderRow);
        Assert.True(table.IsHeaderRow[0]);
    }

    [Fact]
    public void ReturnsEmptyTablesForNonTableHtml()
    {
        var result = HtmlToMarkdownConverter.ConvertWithTables("<p>Hello world</p>");

        Assert.NotNull(result);
        Assert.NotEmpty(result.Content);
        Assert.Empty(result.Tables);
    }

    [Fact]
    public void ExtractsMultipleTables()
    {
        var html = "<table><tr><th>A</th></tr><tr><td>1</td></tr></table>" +
            "<p>text</p>" +
            "<table><tr><th>B</th></tr><tr><td>2</td></tr></table>";

        var result = HtmlToMarkdownConverter.ConvertWithTables(html);

        Assert.Equal(2, result.Tables.Count);
    }

    [Fact]
    public void HandlesEmptyInput()
    {
        var result = HtmlToMarkdownConverter.ConvertWithTables("");

        Assert.NotNull(result);
        Assert.Empty(result.Tables);
    }

    [Fact]
    public void HandlesNullInput()
    {
        Assert.Throws<ArgumentNullException>(() =>
            HtmlToMarkdownConverter.ConvertWithTables(null!));
    }

    [Fact]
    public void ContentContainsTableText()
    {
        var html = "<table><tr><th>Header</th></tr><tr><td>Value</td></tr></table>";
        var result = HtmlToMarkdownConverter.ConvertWithTables(html);

        Assert.Contains("Header", result.Content);
        Assert.Contains("Value", result.Content);
    }

    [Fact]
    public void HandlesSpecialCharacters()
    {
        var html = "<table><tr><td>a &amp; b</td><td>c &lt; d</td></tr></table>";
        var result = HtmlToMarkdownConverter.ConvertWithTables(html);

        Assert.Single(result.Tables);
    }
}
