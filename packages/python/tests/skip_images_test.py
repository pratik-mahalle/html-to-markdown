"""Comprehensive tests for the skip_images feature.

Tests cover:
1. Basic functionality: images skipped when skip_images=True
2. Default behavior: images included when skip_images=False
3. Simple images with attributes
4. Multiple images in same HTML
5. Images in different contexts (paragraphs, lists, tables)
6. Image with surrounding text
7. Alt text handling
8. Edge cases (empty images, complex attributes)
9. Integration with metadata extraction
10. Options handle usage
"""

from __future__ import annotations

from typing import TYPE_CHECKING

from html_to_markdown import (
    ConversionOptions,
    convert_with_handle,
    convert_with_metadata,
    create_options_handle,
)

if TYPE_CHECKING:
    from collections.abc import Callable


def test_skip_images_true_removes_image_markdown(convert: Callable[..., str]) -> None:
    """Verify that images are skipped when skip_images=True."""
    html = '<img src="https://example.com/image.jpg" alt="Test Image">'
    result = convert(html, skip_images=True)
    assert "![Test Image]" not in result
    assert "https://example.com/image.jpg" not in result


def test_skip_images_false_includes_image_markdown(convert: Callable[..., str]) -> None:
    """Verify that images are included by default (skip_images=False)."""
    html = '<img src="https://example.com/image.jpg" alt="Test Image">'
    result = convert(html, skip_images=False)
    assert "![Test Image](https://example.com/image.jpg)" in result


def test_skip_images_default_includes_image_markdown(convert: Callable[..., str]) -> None:
    """Verify that images are included by default when skip_images not specified."""
    html = '<img src="https://example.com/image.jpg" alt="Test Image">'
    result = convert(html)
    assert "![Test Image](https://example.com/image.jpg)" in result


def test_skip_images_simple_img_tag(convert: Callable[..., str]) -> None:
    """Test skipping a simple <img> with src and alt attributes."""
    html = '<p>Before <img src="image.jpg" alt="Picture"> after</p>'
    result = convert(html, skip_images=True)
    assert "Before" in result
    assert "after" in result
    assert "![Picture]" not in result
    assert "image.jpg" not in result


def test_skip_images_simple_img_tag_default(convert: Callable[..., str]) -> None:
    """Test that simple <img> is included by default."""
    html = '<p>Before <img src="image.jpg" alt="Picture"> after</p>'
    result = convert(html)
    assert "![Picture](image.jpg)" in result
    assert "Before" in result
    assert "after" in result


def test_skip_images_multiple_images_in_same_html(convert: Callable[..., str]) -> None:
    """Test skipping multiple <img> elements in same HTML."""
    html = """
    <p>First <img src="image1.jpg" alt="Image 1"> image</p>
    <p>Second <img src="image2.jpg" alt="Image 2"> image</p>
    <p>Third <img src="image3.jpg" alt="Image 3"> image</p>
    """
    result = convert(html, skip_images=True)
    assert "![Image 1]" not in result
    assert "![Image 2]" not in result
    assert "![Image 3]" not in result
    assert "image1.jpg" not in result
    assert "image2.jpg" not in result
    assert "image3.jpg" not in result
    assert "First" in result
    assert "Second" in result
    assert "Third" in result


def test_skip_images_multiple_images_default(convert: Callable[..., str]) -> None:
    """Test that multiple images are included by default."""
    html = """
    <p>First <img src="image1.jpg" alt="Image 1"> image</p>
    <p>Second <img src="image2.jpg" alt="Image 2"> image</p>
    <p>Third <img src="image3.jpg" alt="Image 3"> image</p>
    """
    result = convert(html)
    assert "![Image 1](image1.jpg)" in result
    assert "![Image 2](image2.jpg)" in result
    assert "![Image 3](image3.jpg)" in result


def test_skip_images_in_paragraph(convert: Callable[..., str]) -> None:
    """Test skipping images inside paragraphs."""
    html = "<p>Text with <img src='pic.jpg' alt='Picture'> in middle</p>"
    result = convert(html, skip_images=True)
    assert "Text with" in result
    assert "in middle" in result
    assert "![Picture]" not in result


def test_skip_images_in_list(convert: Callable[..., str]) -> None:
    """Test skipping images inside list items."""
    html = """
    <ul>
        <li>Item 1 <img src="img1.jpg" alt="Img 1"></li>
        <li>Item 2 <img src="img2.jpg" alt="Img 2"></li>
        <li>Item 3</li>
    </ul>
    """
    result = convert(html, skip_images=True)
    assert "- Item 1" in result or "- Item 1 " in result
    assert "- Item 2" in result or "- Item 2 " in result
    assert "- Item 3" in result
    assert "![Img 1]" not in result
    assert "![Img 2]" not in result


def test_skip_images_in_list_default(convert: Callable[..., str]) -> None:
    """Test that images in list items are included by default."""
    html = """
    <ul>
        <li>Item 1 <img src="img1.jpg" alt="Img 1"></li>
        <li>Item 2 <img src="img2.jpg" alt="Img 2"></li>
        <li>Item 3</li>
    </ul>
    """
    result = convert(html)
    assert "![Img 1](img1.jpg)" in result
    assert "![Img 2](img2.jpg)" in result


def test_skip_images_in_table(convert: Callable[..., str]) -> None:
    """Test skipping images inside table cells."""
    html = """
    <table>
        <tr>
            <th>Header</th>
        </tr>
        <tr>
            <td>Cell with <img src="cell.jpg" alt="Cell Image"></td>
        </tr>
    </table>
    """
    result = convert(html, skip_images=True)
    assert "Header" in result
    assert "Cell with" in result
    assert "![Cell Image]" not in result


def test_skip_images_in_table_default(convert: Callable[..., str]) -> None:
    """Test that images in tables are included by default."""
    html = """
    <table>
        <tr>
            <th>Header</th>
        </tr>
        <tr>
            <td>Cell with <img src="cell.jpg" alt="Cell Image"></td>
        </tr>
    </table>
    """
    result = convert(html)
    assert "![Cell Image](cell.jpg)" in result


def test_skip_images_preserves_surrounding_text(convert: Callable[..., str]) -> None:
    """Verify that surrounding content is preserved when images are skipped."""
    html = "<p>Start of paragraph <img src='image.jpg' alt='Middle Image'> end of paragraph</p>"
    result = convert(html, skip_images=True)
    assert "Start of paragraph" in result
    assert "end of paragraph" in result
    assert "![Middle Image]" not in result


def test_skip_images_preserves_surrounding_formatting(convert: Callable[..., str]) -> None:
    """Verify that surrounding formatting is preserved when images are skipped."""
    html = "<p>Text with <strong>bold</strong> <img src='img.jpg' alt='Img'> and <em>italic</em></p>"
    result = convert(html, skip_images=True)
    assert "**bold**" in result
    assert "*italic*" in result
    assert "![Img]" not in result


def test_skip_images_with_alt_text(convert: Callable[..., str]) -> None:
    """Verify that alt text is not included when images are skipped."""
    html = '<img src="image.jpg" alt="Descriptive Alt Text">'
    result = convert(html, skip_images=True)
    assert "Descriptive Alt Text" not in result
    assert "![Descriptive Alt Text]" not in result


def test_skip_images_with_empty_alt(convert: Callable[..., str]) -> None:
    """Test skipping images with empty alt attribute."""
    html = '<img src="image.jpg" alt="">'
    result = convert(html, skip_images=True)
    assert "image.jpg" not in result


def test_skip_images_with_empty_alt_default(convert: Callable[..., str]) -> None:
    """Test that images with empty alt are included by default."""
    html = '<img src="image.jpg" alt="">'
    result = convert(html)
    assert "![](image.jpg)" in result


def test_skip_images_without_alt(convert: Callable[..., str]) -> None:
    """Test skipping images without alt attribute."""
    html = '<img src="image.jpg">'
    result = convert(html, skip_images=True)
    assert "image.jpg" not in result


def test_skip_images_without_alt_default(convert: Callable[..., str]) -> None:
    """Test that images without alt are included by default."""
    html = '<img src="image.jpg">'
    result = convert(html)
    assert "![](image.jpg)" in result or "image.jpg" in result


def test_skip_images_with_title_attribute(convert: Callable[..., str]) -> None:
    """Test skipping images with title attribute."""
    html = '<img src="image.jpg" alt="Alt Text" title="Title Text">'
    result = convert(html, skip_images=True)
    assert "image.jpg" not in result
    assert "Alt Text" not in result
    assert "Title Text" not in result


def test_skip_images_with_title_attribute_default(convert: Callable[..., str]) -> None:
    """Test that images with title are included by default."""
    html = '<img src="image.jpg" alt="Alt Text" title="Title Text">'
    result = convert(html)
    assert "image.jpg" in result


def test_skip_images_with_width_height_attributes(convert: Callable[..., str]) -> None:
    """Test skipping images with width and height attributes."""
    html = '<img src="image.jpg" alt="Image" width="100" height="100">'
    result = convert(html, skip_images=True)
    assert "image.jpg" not in result
    assert "width" not in result
    assert "height" not in result


def test_skip_images_with_data_attributes(convert: Callable[..., str]) -> None:
    """Test skipping images with data attributes."""
    html = '<img src="image.jpg" alt="Image" data-id="123" data-type="featured">'
    result = convert(html, skip_images=True)
    assert "image.jpg" not in result
    assert "data-id" not in result
    assert "data-type" not in result


def test_skip_images_with_class_attribute(convert: Callable[..., str]) -> None:
    """Test skipping images with class attribute."""
    html = '<img src="image.jpg" alt="Image" class="featured-image">'
    result = convert(html, skip_images=True)
    assert "image.jpg" not in result
    assert "featured-image" not in result


def test_skip_images_with_id_attribute(convert: Callable[..., str]) -> None:
    """Test skipping images with id attribute."""
    html = '<img src="image.jpg" alt="Image" id="main-image">'
    result = convert(html, skip_images=True)
    assert "image.jpg" not in result
    assert "main-image" not in result


def test_skip_images_consecutive_images(convert: Callable[..., str]) -> None:
    """Test skipping consecutive images without text between them."""
    html = '<img src="img1.jpg" alt="First"><img src="img2.jpg" alt="Second"><img src="img3.jpg" alt="Third">'
    result = convert(html, skip_images=True)
    assert "![First]" not in result
    assert "![Second]" not in result
    assert "![Third]" not in result
    assert "img1.jpg" not in result
    assert "img2.jpg" not in result
    assert "img3.jpg" not in result


def test_skip_images_consecutive_images_default(convert: Callable[..., str]) -> None:
    """Test that consecutive images are included by default."""
    html = '<img src="img1.jpg" alt="First"><img src="img2.jpg" alt="Second"><img src="img3.jpg" alt="Third">'
    result = convert(html)
    assert "![First](img1.jpg)" in result
    assert "![Second](img2.jpg)" in result
    assert "![Third](img3.jpg)" in result


def test_skip_images_nested_in_link(convert: Callable[..., str]) -> None:
    """Test skipping images nested inside link elements."""
    html = '<a href="https://example.com"><img src="image.jpg" alt="Click me"></a>'
    result = convert(html, skip_images=True)
    # The link should still be present but without the image
    assert "![Click me]" not in result
    assert "image.jpg" not in result


def test_skip_images_nested_in_link_default(convert: Callable[..., str]) -> None:
    """Test that images nested in links are included by default."""
    html = '<a href="https://example.com"><img src="image.jpg" alt="Click me"></a>'
    result = convert(html)
    assert "![Click me](image.jpg)" in result


def test_skip_images_with_srcset_attribute(convert: Callable[..., str]) -> None:
    """Test skipping images with srcset attribute."""
    html = """<img
        src="image.jpg"
        alt="Responsive Image"
        srcset="image-small.jpg 480w, image-medium.jpg 800w, image-large.jpg 1200w"
    >"""
    result = convert(html, skip_images=True)
    assert "image.jpg" not in result
    assert "![Responsive Image]" not in result
    assert "srcset" not in result


def test_skip_images_with_picture_element(convert: Callable[..., str]) -> None:
    """Test skipping images inside picture elements."""
    html = """
    <picture>
        <source media="(min-width:800px)" srcset="large.jpg">
        <img src="small.jpg" alt="Responsive">
    </picture>
    """
    result = convert(html, skip_images=True)
    assert "![Responsive]" not in result
    assert "small.jpg" not in result
    assert "large.jpg" not in result


def test_skip_images_in_figure(convert: Callable[..., str]) -> None:
    """Test skipping images inside figure elements."""
    html = """
    <figure>
        <img src="image.jpg" alt="Figure Image">
        <figcaption>This is a figure caption</figcaption>
    </figure>
    """
    result = convert(html, skip_images=True)
    assert "![Figure Image]" not in result
    assert "image.jpg" not in result
    assert "This is a figure caption" in result


def test_skip_images_in_figure_default(convert: Callable[..., str]) -> None:
    """Test that images in figures are included by default."""
    html = """
    <figure>
        <img src="image.jpg" alt="Figure Image">
        <figcaption>This is a figure caption</figcaption>
    </figure>
    """
    result = convert(html)
    assert "![Figure Image](image.jpg)" in result
    assert "This is a figure caption" in result


def test_skip_images_complex_document(convert: Callable[..., str]) -> None:
    """Test skipping images in a complex document with mixed content."""
    html = """
    <article>
        <h1>Article Title</h1>
        <img src="header.jpg" alt="Header Image">
        <p>Introduction paragraph with <img src="inline.jpg" alt="Inline"> image.</p>
        <section>
            <h2>Section</h2>
            <img src="section.jpg" alt="Section Image">
            <p>More content</p>
        </section>
        <footer>
            <p>Footer with <img src="footer.jpg" alt="Footer"> image</p>
        </footer>
    </article>
    """
    result = convert(html, skip_images=True)
    assert "Article Title" in result
    assert "Introduction paragraph with" in result
    assert "image" in result.lower()
    assert "Section" in result
    assert "More content" in result
    assert "Footer with" in result
    assert "![Header Image]" not in result
    assert "![Inline]" not in result
    assert "![Section Image]" not in result
    assert "![Footer]" not in result
    assert "header.jpg" not in result
    assert "inline.jpg" not in result
    assert "section.jpg" not in result
    assert "footer.jpg" not in result


def test_skip_images_complex_document_default(convert: Callable[..., str]) -> None:
    """Test that images in complex documents are included by default."""
    html = """
    <article>
        <h1>Article Title</h1>
        <img src="header.jpg" alt="Header Image">
        <p>Introduction paragraph.</p>
    </article>
    """
    result = convert(html)
    assert "Article Title" in result
    assert "![Header Image](header.jpg)" in result


def test_skip_images_with_relative_urls(convert: Callable[..., str]) -> None:
    """Test skipping images with relative URLs."""
    html = """
    <p>Image 1: <img src="./images/pic1.jpg" alt="Pic1"></p>
    <p>Image 2: <img src="../images/pic2.jpg" alt="Pic2"></p>
    <p>Image 3: <img src="/images/pic3.jpg" alt="Pic3"></p>
    """
    result = convert(html, skip_images=True)
    assert "![Pic1]" not in result
    assert "![Pic2]" not in result
    assert "![Pic3]" not in result
    assert "images/pic1.jpg" not in result
    assert "images/pic2.jpg" not in result
    assert "images/pic3.jpg" not in result


def test_skip_images_with_relative_urls_default(convert: Callable[..., str]) -> None:
    """Test that images with relative URLs are included by default."""
    html = '<img src="./images/pic.jpg" alt="Picture">'
    result = convert(html)
    assert "![Picture](./images/pic.jpg)" in result


def test_skip_images_with_absolute_urls(convert: Callable[..., str]) -> None:
    """Test skipping images with absolute URLs."""
    html = '<img src="https://example.com/images/pic.jpg" alt="Picture">'
    result = convert(html, skip_images=True)
    assert "![Picture]" not in result
    assert "https://example.com/images/pic.jpg" not in result


def test_skip_images_with_absolute_urls_default(convert: Callable[..., str]) -> None:
    """Test that images with absolute URLs are included by default."""
    html = '<img src="https://example.com/images/pic.jpg" alt="Picture">'
    result = convert(html)
    assert "![Picture](https://example.com/images/pic.jpg)" in result


def test_skip_images_with_data_uri(convert: Callable[..., str]) -> None:
    """Test skipping images with data URI."""
    html = '<img src="data:image/png;base64,iVBORw0KGgo..." alt="Embedded">'
    result = convert(html, skip_images=True)
    assert "![Embedded]" not in result
    assert "data:image" not in result


def test_skip_images_whitespace_around_image(convert: Callable[..., str]) -> None:
    """Test skipping images with whitespace around them."""
    html = "<p>Text   <img src='image.jpg' alt='Image'>   more text</p>"
    result = convert(html, skip_images=True)
    assert "Text" in result
    assert "more text" in result
    assert "![Image]" not in result


def test_skip_images_newlines_around_image(convert: Callable[..., str]) -> None:
    """Test skipping images with newlines around them."""
    html = """<p>
        Before
        <img src="image.jpg" alt="Image">
        After
    </p>"""
    result = convert(html, skip_images=True)
    assert "Before" in result
    assert "After" in result
    assert "![Image]" not in result


def test_skip_images_in_heading(convert: Callable[..., str]) -> None:
    """Test skipping images inside headings."""
    html = '<h1>Title <img src="badge.jpg" alt="Badge"> here</h1>'
    result = convert(html, skip_images=True)
    assert "Title" in result
    assert "here" in result
    assert "![Badge]" not in result


def test_skip_images_in_heading_default(convert: Callable[..., str]) -> None:
    """Test that images in headings use alt text by default."""
    html = '<h1>Title <img src="badge.jpg" alt="Badge"> here</h1>'
    result = convert(html)
    assert "Title" in result
    assert "Badge" in result
    # In headings, images are typically not converted to markdown syntax
    # but their alt text is preserved
    assert "here" in result


def test_skip_images_integration_with_metadata(convert: Callable[..., str]) -> None:
    """Test integration with convert_with_metadata.

    Images should be skipped in markdown but still captured in metadata.
    """
    html = """
    <html>
        <head><title>Test</title></head>
        <body>
            <p>Content <img src="image.jpg" alt="Image"> here</p>
        </body>
    </html>
    """

    options = ConversionOptions(skip_images=True)
    markdown, _metadata = convert_with_metadata(html, options=options)

    # Image should not appear in markdown
    assert "![Image]" not in markdown
    assert "image.jpg" not in markdown

    # But content should be present
    assert "Content" in markdown
    assert "here" in markdown


def test_skip_images_with_options_handle(convert: Callable[..., str]) -> None:
    """Test skip_images with create_options_handle."""
    html = '<p>Text with <img src="image.jpg" alt="Image"> image</p>'

    # Create handle with skip_images=True
    handle = create_options_handle(ConversionOptions(skip_images=True))
    result = convert_with_handle(html, handle)

    assert "Text with" in result
    assert "![Image]" not in result
    assert "image.jpg" not in result


def test_skip_images_with_options_handle_default(convert: Callable[..., str]) -> None:
    """Test that images are included by default with options handle."""
    html = '<p>Text with <img src="image.jpg" alt="Image"> image</p>'

    # Create handle without skip_images (defaults to False)
    handle = create_options_handle(ConversionOptions())
    result = convert_with_handle(html, handle)

    assert "![Image](image.jpg)" in result


def test_skip_images_empty_document(convert: Callable[..., str]) -> None:
    """Test skip_images with empty document."""
    html = ""
    result = convert(html, skip_images=True)
    assert result == ""


def test_skip_images_only_images(convert: Callable[..., str]) -> None:
    """Test skip_images when document contains only images."""
    html = '<img src="img1.jpg" alt="1"><img src="img2.jpg" alt="2">'
    result = convert(html, skip_images=True)
    # Should result in empty or whitespace-only markdown
    assert "![1]" not in result
    assert "![2]" not in result
    assert "img1.jpg" not in result
    assert "img2.jpg" not in result


def test_skip_images_mixed_with_other_options(convert: Callable[..., str]) -> None:
    """Test skip_images in combination with other conversion options."""
    html = """
    <h1>Title</h1>
    <p>Text with **bold** and <img src="image.jpg" alt="Image"></p>
    """
    result = convert(
        html,
        skip_images=True,
        heading_style="atx_closed",
        strong_em_symbol="*",
    )
    assert "# Title #" in result
    assert "**bold**" in result
    assert "![Image]" not in result


def test_skip_images_preserves_surrounding_links(convert: Callable[..., str]) -> None:
    """Test that surrounding links are preserved when images are skipped."""
    html = '<p>Visit <a href="https://example.com">our site</a> and see <img src="img.jpg" alt="Picture"></p>'
    result = convert(html, skip_images=True)
    assert "[our site](https://example.com)" in result
    assert "![Picture]" not in result


def test_skip_images_with_multiple_attributes_complex(convert: Callable[..., str]) -> None:
    """Test skipping images with complex multiple attributes."""
    html = """<img
        id="main-img"
        class="responsive featured"
        src="https://example.com/images/photo.jpg?v=1"
        alt="A photo with many attributes"
        title="Hover text"
        width="800"
        height="600"
        loading="lazy"
        decoding="async"
        data-src="backup.jpg"
        data-id="12345"
    >"""
    result = convert(html, skip_images=True)
    assert "https://example.com/images/photo.jpg" not in result
    assert "![A photo with many attributes]" not in result
    assert "main-img" not in result
    assert "responsive" not in result
