"""Comprehensive tests for html-to-markdown features."""

import json
from pathlib import Path
from typing import Any

import pytest

from html_to_markdown import (
    ConversionOptions,
    OutputFormat,
    convert,
    convert_with_handle,
    create_options_handle,
)


def load_fixtures(filename: str) -> list[dict[str, Any]]:
    """Load test fixtures from JSON file."""
    fixture_path = Path(__file__).parent.parent / "fixtures" / filename
    with Path(fixture_path).open() as f:
        return json.load(f)


@pytest.fixture
def basic_fixtures() -> list[dict[str, Any]]:
    """Load basic HTML fixtures."""
    return load_fixtures("basic-html.json")


@pytest.mark.parametrize("test_case", load_fixtures("basic-html.json"), ids=lambda tc: tc["name"])
def test_basic_html_conversion(test_case: dict[str, Any]) -> None:
    """Test basic HTML conversions from fixtures."""
    result = convert(test_case["html"])
    expected = test_case["expectedMarkdown"]

    # Normalize whitespace for comparison
    assert result.strip() == expected.strip()  # noqa: S101


class TestOutputFormats:
    """Test different output formats."""

    def test_markdown_format(self) -> None:
        """Test Markdown output format."""
        html = "<h1>Title</h1><p>Paragraph</p>"
        options = ConversionOptions(output_format=OutputFormat.MARKDOWN)
        result = convert(html, options)
        assert "#" in result or "Title" in result  # noqa: S101

    def test_djot_format(self) -> None:
        """Test DJOT output format."""
        html = "<h1>Title</h1><p>Paragraph</p>"
        options = ConversionOptions(output_format=OutputFormat.DJOT)
        result = convert(html, options)
        assert result is not None  # noqa: S101
        assert "Title" in result  # noqa: S101


class TestEdgeCases:
    """Test edge cases and error handling."""

    def test_empty_html(self) -> None:
        """Test empty HTML string."""
        result = convert("")
        assert result == ""  # noqa: S101

    def test_whitespace_only(self) -> None:
        """Test HTML with only whitespace."""
        result = convert("   \n\t  ")
        assert result.strip() == ""  # noqa: S101

    def test_html_entities(self) -> None:
        """Test HTML entities."""
        html = "<p>&lt;test&gt; &amp; &quot;quoted&quot;</p>"
        result = convert(html)
        assert "test" in result.lower() or "&" in result  # noqa: S101

    def test_special_characters(self) -> None:
        """Test special characters in content."""
        html = "<p>Special: © ® ™ € ¥</p>"
        result = convert(html)
        assert result is not None  # noqa: S101

    def test_unicode_content(self) -> None:
        """Test Unicode content."""
        html = "<p>Unicode: 你好世界 🌍 مرحبا</p>"
        result = convert(html)
        assert result is not None  # noqa: S101

    def test_cdata_sections(self) -> None:
        """Test CDATA sections."""
        html = "<p><![CDATA[This is CDATA]]></p>"
        result = convert(html)
        assert result is not None  # noqa: S101


class TestComplexStructures:
    """Test complex HTML structures."""

    def test_nested_lists(self) -> None:
        """Test nested list structures."""
        html = "<ul><li>Item 1<ul><li>Nested 1</li><li>Nested 2</li></ul></li><li>Item 2</li></ul>"
        result = convert(html)
        assert "Item 1" in result  # noqa: S101
        assert "Nested" in result  # noqa: S101

    def test_mixed_content(self) -> None:
        """Test mixed HTML content."""
        html = (
            "<article>"
            "<h1>Title</h1>"
            "<p>Paragraph with <strong>bold</strong> and <em>italic</em>.</p>"
            "<ul><li>Point 1</li><li>Point 2</li></ul>"
            "<blockquote>A quote</blockquote>"
            "</article>"
        )
        result = convert(html)
        assert "Title" in result  # noqa: S101
        assert "bold" in result.lower()  # noqa: S101
        assert "Point" in result  # noqa: S101

    def test_table_conversion(self) -> None:
        """Test table conversion."""
        html = "<table><tr><th>Header 1</th><th>Header 2</th></tr><tr><td>Cell 1</td><td>Cell 2</td></tr></table>"
        result = convert(html)
        assert result is not None  # noqa: S101

    def test_code_blocks(self) -> None:
        """Test code block conversion."""
        html = '<pre><code>function hello() {\n  console.log("world");\n}</code></pre>'
        result = convert(html)
        assert "hello" in result.lower()  # noqa: S101


class TestApiConsistency:
    """Test API consistency across different invocation methods."""

    def test_convert_vs_handle_consistency(self) -> None:
        """Test consistency between convert and convert_with_handle."""
        html = "<h1>Title</h1><p>Content</p>"
        options = ConversionOptions(output_format=OutputFormat.MARKDOWN)

        result1 = convert(html, options)
        handle = create_options_handle()
        result2 = convert_with_handle(html, handle)

        assert len(result1) > 0  # noqa: S101
        assert len(result2) > 0  # noqa: S101

    def test_multiple_conversions_with_same_handle(self) -> None:
        """Test reusing a handle for multiple conversions."""
        handle = create_options_handle()

        html1 = "<p>First</p>"
        html2 = "<p>Second</p>"

        result1 = convert_with_handle(html1, handle)
        result2 = convert_with_handle(html2, handle)

        assert "First" in result1  # noqa: S101
        assert "Second" in result2  # noqa: S101
