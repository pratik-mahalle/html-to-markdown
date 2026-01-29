"""Smoke tests for html-to-markdown Python package."""

import html_to_markdown
from html_to_markdown import (
    ConversionOptions,
    ConversionOptionsHandle,
    OutputFormat,
    convert,
    convert_with_handle,
    create_options_handle,
)


def test_package_imports() -> None:
    """Test that package can be imported."""
    assert html_to_markdown is not None  # noqa: S101
    # Version is dynamic, just verify it's a string
    assert isinstance(html_to_markdown.__version__, str)  # noqa: S101
    assert len(html_to_markdown.__version__) > 0  # noqa: S101


def test_basic_conversion() -> None:
    """Test basic HTML to markdown conversion."""
    html = "<p>Hello World</p>"
    result = convert(html)
    assert "Hello World" in result  # noqa: S101


def test_conversion_with_options_dataclass() -> None:
    """Test conversion with ConversionOptions dataclass."""
    html = "<h1>Title</h1>"
    options = ConversionOptions(output_format=OutputFormat.MARKDOWN)
    result = convert(html, options)
    assert "#" in result or "Title" in result  # noqa: S101


def test_conversion_with_djot_format() -> None:
    """Test conversion with DJOT format."""
    html = "<h1>Title</h1>"
    options = ConversionOptions(output_format=OutputFormat.DJOT)
    result = convert(html, options)
    assert "Title" in result  # noqa: S101


def test_conversion_with_options_handle() -> None:
    """Test conversion with ConversionOptionsHandle."""
    html = "<h1>Title</h1><p>Content</p>"
    handle = ConversionOptionsHandle()
    result = convert_with_handle(html, handle)
    assert "#" in result or "Title" in result  # noqa: S101


def test_create_options_handle() -> None:
    """Test creating options handle via factory function."""
    html = "<h1>Test</h1>"
    handle = create_options_handle()
    result = convert_with_handle(html, handle)
    assert "Test" in result  # noqa: S101


def test_empty_string_handling() -> None:
    """Test error handling for empty string."""
    result = convert("")
    assert result == ""  # noqa: S101


def test_malformed_html() -> None:
    """Test handling of malformed HTML."""
    html = "<p>Unclosed paragraph"
    result = convert(html)
    assert "Unclosed paragraph" in result  # noqa: S101


def test_nested_elements() -> None:
    """Test nested HTML elements."""
    html = "<div><p><strong>Bold</strong> text</p></div>"
    result = convert(html)
    assert "**Bold**" in result  # noqa: S101
    assert "text" in result  # noqa: S101
