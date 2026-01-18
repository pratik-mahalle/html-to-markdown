"""Tests for OutputFormat enum and output_format option."""

from html_to_markdown import (
    ConversionOptions,
    OutputFormat,
    convert,
)

OutputFormatImported = OutputFormat


def test_output_format_enum_values() -> None:
    """Test that OutputFormat enum has correct values."""
    assert OutputFormat.MARKDOWN.value == "markdown"
    assert OutputFormat.DJOT.value == "djot"


def test_output_format_enum_string_conversion() -> None:
    """Test that OutputFormat enum can be converted to string."""
    assert str(OutputFormat.MARKDOWN) == "OutputFormat.MARKDOWN"
    assert str(OutputFormat.DJOT) == "OutputFormat.DJOT"


def test_conversion_options_with_output_format_markdown() -> None:
    """Test ConversionOptions accepts output_format with markdown value."""
    options = ConversionOptions(output_format="markdown")
    assert options.output_format == "markdown"


def test_conversion_options_with_output_format_djot() -> None:
    """Test ConversionOptions accepts output_format with djot value."""
    options = ConversionOptions(output_format="djot")
    assert options.output_format == "djot"


def test_conversion_options_default_output_format() -> None:
    """Test that ConversionOptions defaults to markdown format."""
    options = ConversionOptions()
    assert options.output_format == "markdown"


def test_conversion_options_with_output_format_enum() -> None:
    """Test ConversionOptions accepts OutputFormat enum."""
    options = ConversionOptions(output_format=OutputFormat.DJOT.value)
    assert options.output_format == "djot"


def test_convert_with_markdown_output_format() -> None:
    """Test conversion with markdown output format."""
    html = "<h1>Hello</h1><p>World</p>"
    options = ConversionOptions(output_format="markdown")
    markdown = convert(html, options)
    # Basic markdown output should be present
    assert len(markdown) > 0
    assert "Hello" in markdown
    assert "World" in markdown


def test_convert_with_djot_output_format() -> None:
    """Test conversion with djot output format."""
    html = "<h1>Hello</h1><p>World</p>"
    options = ConversionOptions(output_format="djot")
    markdown = convert(html, options)
    # Basic output should be present (djot is also a valid format)
    assert len(markdown) > 0
    assert "Hello" in markdown
    assert "World" in markdown


def test_output_format_is_exportable() -> None:
    """Test that OutputFormat is properly exported from main module."""
    assert OutputFormatImported.MARKDOWN == OutputFormat.MARKDOWN
    assert OutputFormatImported.DJOT == OutputFormat.DJOT
