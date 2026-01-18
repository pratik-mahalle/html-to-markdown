using System.Text.Json;
using Xunit;

namespace HtmlToMarkdown.Tests;

/// <summary>
/// Tests for OutputFormat enum and ConversionOptions class.
/// </summary>
public class OutputFormatTests
{
    [Fact]
    public void OutputFormat_Markdown_DefaultValue()
    {
        // Arrange & Act
        var options = new ConversionOptions();

        // Assert
        Assert.Equal("markdown", options.OutputFormat);
    }

    [Fact]
    public void OutputFormat_CanBeSetToDjot()
    {
        // Arrange
        var options = new ConversionOptions();

        // Act
        options.OutputFormat = "djot";

        // Assert
        Assert.Equal("djot", options.OutputFormat);
    }

    [Fact]
    public void OutputFormat_SerializesToJson()
    {
        // Arrange
        var options = new ConversionOptions
        {
            OutputFormat = "djot"
        };

        // Act
        var json = JsonSerializer.Serialize(options);

        // Assert
        Assert.Contains("\"outputFormat\":\"djot\"", json);
    }

    [Fact]
    public void ConversionOptions_HasDefaultValues()
    {
        // Arrange & Act
        var options = new ConversionOptions();

        // Assert
        Assert.Equal("atx", options.HeadingStyle);
        Assert.Equal("spaces", options.ListIndentType);
        Assert.Equal(2, options.ListIndentWidth);
        Assert.Equal("-", options.Bullets);
        Assert.Equal('*', options.StrongEmSymbol);
        Assert.False(options.EscapeAsterisks);
        Assert.False(options.EscapeUnderscores);
        Assert.False(options.EscapeMisc);
        Assert.False(options.EscapeAscii);
        Assert.Equal("", options.CodeLanguage);
        Assert.True(options.Autolinks);
        Assert.False(options.DefaultTitle);
        Assert.False(options.BrInTables);
        Assert.True(options.HocrSpatialTables);
        Assert.Equal("double_equal", options.HighlightStyle);
        Assert.True(options.ExtractMetadata);
        Assert.Equal("normalized", options.WhitespaceMode);
        Assert.False(options.StripNewlines);
        Assert.False(options.Wrap);
        Assert.Equal(80, options.WrapWidth);
        Assert.False(options.ConvertAsInline);
        Assert.Equal("", options.SubSymbol);
        Assert.Equal("", options.SupSymbol);
        Assert.Equal("spaces", options.NewlineStyle);
        Assert.Equal("indented", options.CodeBlockStyle);
        Assert.NotNull(options.KeepInlineImagesIn);
        Assert.Empty(options.KeepInlineImagesIn);
        Assert.Equal("utf-8", options.Encoding);
        Assert.False(options.Debug);
        Assert.NotNull(options.StripTags);
        Assert.Empty(options.StripTags);
        Assert.NotNull(options.PreserveTags);
        Assert.Empty(options.PreserveTags);
        Assert.False(options.SkipImages);
        Assert.Equal("markdown", options.OutputFormat);
    }

    [Fact]
    public void ConversionOptions_SupportsOutputFormatEnum()
    {
        // Arrange
        var options = new ConversionOptions();

        // Act & Assert
        options.OutputFormat = "markdown";
        Assert.Equal("markdown", options.OutputFormat);

        options.OutputFormat = "djot";
        Assert.Equal("djot", options.OutputFormat);
    }

    [Fact]
    public void PreprocessingOptions_HasDefaultValues()
    {
        // Arrange & Act
        var options = new ConversionOptions();

        // Assert
        Assert.NotNull(options.Preprocessing);
        Assert.False(options.Preprocessing.Enabled);
        Assert.Equal("standard", options.Preprocessing.Preset);
        Assert.True(options.Preprocessing.RemoveNavigation);
        Assert.True(options.Preprocessing.RemoveForms);
    }

    [Fact]
    public void ConversionOptions_CanBeModified()
    {
        // Arrange
        var options = new ConversionOptions();

        // Act
        options.OutputFormat = "djot";
        options.HeadingStyle = "underlined";
        options.ListIndentWidth = 4;
        options.EscapeAsterisks = true;
        options.SkipImages = true;

        // Assert
        Assert.Equal("djot", options.OutputFormat);
        Assert.Equal("underlined", options.HeadingStyle);
        Assert.Equal(4, options.ListIndentWidth);
        Assert.True(options.EscapeAsterisks);
        Assert.True(options.SkipImages);
    }

    [Fact]
    public void ConversionOptions_SupportsListProperties()
    {
        // Arrange
        var options = new ConversionOptions();

        // Act
        options.StripTags.Add("script");
        options.StripTags.Add("style");
        options.PreserveTags.Add("table");
        options.KeepInlineImagesIn.Add("p");

        // Assert
        Assert.Contains("script", options.StripTags);
        Assert.Contains("style", options.StripTags);
        Assert.Contains("table", options.PreserveTags);
        Assert.Contains("p", options.KeepInlineImagesIn);
    }

    [Fact]
    public void ConversionOptions_SupportsJsonSerialization()
    {
        // Arrange
        var options = new ConversionOptions
        {
            OutputFormat = "djot",
            HeadingStyle = "atx_closed",
            ListIndentWidth = 4,
            EscapeAsterisks = true
        };

        // Act
        var json = JsonSerializer.Serialize(options);
        var deserialized = JsonSerializer.Deserialize<ConversionOptions>(json);

        // Assert
        Assert.NotNull(deserialized);
        Assert.Equal("djot", deserialized!.OutputFormat);
        Assert.Equal("atx_closed", deserialized.HeadingStyle);
        Assert.Equal(4, deserialized.ListIndentWidth);
        Assert.True(deserialized.EscapeAsterisks);
    }
}
