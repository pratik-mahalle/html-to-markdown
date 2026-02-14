"""
Test suite for Issue #187: Visitor pattern tagName context field.

This test file verifies that the visitor pattern correctly provides the HTML
tag name in the context dictionary passed to visitor callbacks. Specifically:

1. visit_element_start receives correct tag_name in context
2. Tag names are not "unknown" but actual HTML tag names
3. Filtering by tag name works correctly
4. Attribute access works with proper tag identification

Related Issue: #187 (https://github.com/kreuzberg-dev/html-to-markdown/issues/187)
"""

from __future__ import annotations

from typing import Any

from html_to_markdown import convert_with_visitor


class TestVisitorTagNameContext:
    """Test that visitor callbacks receive correct tag names in context."""

    def test_visit_element_start_receives_div_tag_name(self) -> None:
        """Test that visit_element_start receives 'div' as tag_name."""

        class DivTagVisitor:
            def __init__(self) -> None:
                self.div_contexts: list[dict[str, Any]] = []

            def visit_element_start(self, ctx: dict[str, Any]) -> dict[str, str]:
                if ctx.get("tag_name") == "div":
                    self.div_contexts.append(ctx)
                return {"type": "continue"}

        visitor = DivTagVisitor()
        html = "<div>Content</div>"
        convert_with_visitor(html, visitor=visitor)

        # FAILING ASSERTION: Should have at least one div context
        assert len(visitor.div_contexts) > 0, "Expected visit_element_start to be called for div"
        assert visitor.div_contexts[0]["tag_name"] == "div", "Expected tag_name to be 'div', not 'unknown'"

    def test_visit_element_start_receives_script_tag_name(self) -> None:
        """Test that visit_element_start receives 'script' as tag_name."""

        class ScriptTagVisitor:
            def __init__(self) -> None:
                self.script_contexts: list[dict[str, Any]] = []

            def visit_element_start(self, ctx: dict[str, Any]) -> dict[str, str]:
                if ctx.get("tag_name") == "script":
                    self.script_contexts.append(ctx)
                return {"type": "continue"}

        visitor = ScriptTagVisitor()
        html = "<script>console.log('test');</script>"
        convert_with_visitor(html, visitor=visitor)

        # FAILING ASSERTION: Should capture script tag name
        assert len(visitor.script_contexts) > 0, "Expected visit_element_start to be called for script"
        assert visitor.script_contexts[0]["tag_name"] == "script", "Expected tag_name to be 'script'"

    def test_visit_element_start_receives_style_tag_name(self) -> None:
        """Test that visit_element_start receives 'style' as tag_name."""

        class StyleTagVisitor:
            def __init__(self) -> None:
                self.style_contexts: list[dict[str, Any]] = []

            def visit_element_start(self, ctx: dict[str, Any]) -> dict[str, str]:
                if ctx.get("tag_name") == "style":
                    self.style_contexts.append(ctx)
                return {"type": "continue"}

        visitor = StyleTagVisitor()
        html = "<style>.test { color: red; }</style>"
        convert_with_visitor(html, visitor=visitor)

        # FAILING ASSERTION: Should capture style tag name
        assert len(visitor.style_contexts) > 0, "Expected visit_element_start to be called for style"
        assert visitor.style_contexts[0]["tag_name"] == "style", "Expected tag_name to be 'style'"

    def test_visit_element_start_receives_p_tag_name(self) -> None:
        """Test that visit_element_start receives 'p' as tag_name."""

        class ParagraphTagVisitor:
            def __init__(self) -> None:
                self.p_contexts: list[dict[str, Any]] = []

            def visit_element_start(self, ctx: dict[str, Any]) -> dict[str, str]:
                if ctx.get("tag_name") == "p":
                    self.p_contexts.append(ctx)
                return {"type": "continue"}

        visitor = ParagraphTagVisitor()
        html = "<p>Paragraph content</p>"
        convert_with_visitor(html, visitor=visitor)

        # FAILING ASSERTION: Should capture p tag name
        assert len(visitor.p_contexts) > 0, "Expected visit_element_start to be called for p"
        assert visitor.p_contexts[0]["tag_name"] == "p", "Expected tag_name to be 'p'"

    def test_visit_element_start_never_receives_unknown_tag_name(self) -> None:
        """Test that tag_name is never 'unknown' for recognized HTML elements."""

        class UnknownTagVisitor:
            def __init__(self) -> None:
                self.unknown_tags: list[str] = []
                self.all_tags: list[str] = []

            def visit_element_start(self, ctx: dict[str, Any]) -> dict[str, str]:
                tag_name = ctx.get("tag_name", "unknown")
                self.all_tags.append(tag_name)
                if tag_name == "unknown":
                    self.unknown_tags.append(tag_name)
                return {"type": "continue"}

        visitor = UnknownTagVisitor()
        html = """
            <div id="container">
                <p>Paragraph</p>
                <span>Inline text</span>
            </div>
            <script>console.log('test');</script>
            <style>.class { color: blue; }</style>
        """
        convert_with_visitor(html, visitor=visitor)

        # FAILING ASSERTION: Should not have any "unknown" tag names
        assert len(visitor.unknown_tags) == 0, (
            f"Expected no 'unknown' tag names, but got {visitor.unknown_tags}. All tags seen: {visitor.all_tags}"
        )

    def test_tag_name_filtering_works_for_multiple_elements(self) -> None:
        """Test that tag name filtering correctly identifies multiple element types."""

        class MultiElementFilterVisitor:
            def __init__(self) -> None:
                self.divs: list[dict[str, Any]] = []
                self.spans: list[dict[str, Any]] = []
                self.paragraphs: list[dict[str, Any]] = []

            def visit_element_start(self, ctx: dict[str, Any]) -> dict[str, str]:
                tag_name = ctx.get("tag_name")
                if tag_name == "div":
                    self.divs.append(ctx)
                elif tag_name == "span":
                    self.spans.append(ctx)
                elif tag_name == "p":
                    self.paragraphs.append(ctx)
                return {"type": "continue"}

        visitor = MultiElementFilterVisitor()
        html = """
            <div>
                <p>First paragraph</p>
                <span>Inline</span>
                <p>Second paragraph</p>
            </div>
        """
        convert_with_visitor(html, visitor=visitor)

        # FAILING ASSERTIONS: Each element type should be captured correctly
        assert len(visitor.divs) > 0, "Expected to capture div elements"
        assert len(visitor.paragraphs) >= 2, "Expected to capture at least 2 p elements"
        assert len(visitor.spans) > 0, "Expected to capture span elements"

        # Verify tag names are correct
        assert all(ctx["tag_name"] == "div" for ctx in visitor.divs), "All divs should have tag_name 'div'"
        assert all(ctx["tag_name"] == "p" for ctx in visitor.paragraphs), "All p tags should have tag_name 'p'"
        assert all(ctx["tag_name"] == "span" for ctx in visitor.spans), "All spans should have tag_name 'span'"


class TestVisitorFilteringByAttribute:
    """Test that visitor can filter elements by attributes based on tag names."""

    def test_filter_divs_by_class_attribute(self) -> None:
        """Test filtering divs by class attribute works correctly."""

        class ClassFilterVisitor:
            def __init__(self, target_class: str) -> None:
                self.target_class = target_class
                self.matched_elements: list[dict[str, Any]] = []

            def visit_element_start(self, ctx: dict[str, Any]) -> dict[str, str]:
                tag_name = ctx.get("tag_name")
                attributes = ctx.get("attributes", {})
                class_attr = attributes.get("class", "")

                if tag_name == "div" and self.target_class in class_attr:
                    self.matched_elements.append(ctx)

                return {"type": "continue"}

        visitor = ClassFilterVisitor("highlight")
        html = """
            <div class="highlight">Should be matched</div>
            <div class="normal">Should not match</div>
            <div class="highlight another-class">Should also match</div>
        """
        convert_with_visitor(html, visitor=visitor)

        # FAILING ASSERTION: Should match exactly the highlighted divs
        assert len(visitor.matched_elements) == 2, f"Expected 2 matched elements, got {len(visitor.matched_elements)}"

    def test_filter_elements_by_id_attribute(self) -> None:
        """Test filtering elements by id attribute based on correct tag_name."""

        class IdFilterVisitor:
            def __init__(self) -> None:
                self.elements_with_ids: dict[str, list[str]] = {}

            def visit_element_start(self, ctx: dict[str, Any]) -> dict[str, str]:
                tag_name = ctx.get("tag_name")
                attributes = ctx.get("attributes", {})
                element_id = attributes.get("id")

                if element_id and tag_name:
                    if tag_name not in self.elements_with_ids:
                        self.elements_with_ids[tag_name] = []
                    self.elements_with_ids[tag_name].append(element_id)

                return {"type": "continue"}

        visitor = IdFilterVisitor()
        html = """
            <div id="main-container">Content</div>
            <p id="intro">Paragraph</p>
            <span id="highlight">Inline</span>
        """
        convert_with_visitor(html, visitor=visitor)

        # FAILING ASSERTIONS: Should correctly map IDs to their tag names
        assert "div" in visitor.elements_with_ids, "Expected to find divs with IDs"
        assert "main-container" in visitor.elements_with_ids.get("div", []), "Expected div with id 'main-container'"

        assert "p" in visitor.elements_with_ids, "Expected to find paragraphs with IDs"
        assert "intro" in visitor.elements_with_ids.get("p", []), "Expected p with id 'intro'"

        assert "span" in visitor.elements_with_ids, "Expected to find spans with IDs"
        assert "highlight" in visitor.elements_with_ids.get("span", []), "Expected span with id 'highlight'"

    def test_conditional_processing_based_on_tag_name(self) -> None:
        """Test that visitors can conditionally process elements based on tag_name."""

        class ConditionalProcessorVisitor:
            def __init__(self) -> None:
                self.skip_tags: list[str] = []
                self.process_tags: list[str] = []

            def visit_element_start(self, ctx: dict[str, Any]) -> dict[str, str]:
                tag_name = ctx.get("tag_name")

                # Skip script and style tags
                if tag_name in ("script", "style"):
                    self.skip_tags.append(tag_name)
                    return {"type": "skip"}

                # Process other tags
                if tag_name:
                    self.process_tags.append(tag_name)

                return {"type": "continue"}

        visitor = ConditionalProcessorVisitor()
        html = """
            <div>
                <p>Keep this</p>
                <script>Skip this</script>
                <style>Skip this too</style>
                <span>Keep this</span>
            </div>
        """
        convert_with_visitor(html, visitor=visitor)

        # FAILING ASSERTIONS: Should correctly skip script/style based on tag_name
        assert "script" in visitor.skip_tags, "Expected to skip script tags"
        assert "style" in visitor.skip_tags, "Expected to skip style tags"

        assert "div" in visitor.process_tags, "Expected to process div tags"
        assert "p" in visitor.process_tags, "Expected to process p tags"
        assert "span" in visitor.process_tags, "Expected to process span tags"


class TestVisitorContextIntegrity:
    """Test that context dictionary maintains integrity across different tag types."""

    def test_context_has_all_required_fields_for_all_tags(self) -> None:
        """Test that all tag contexts have required fields."""

        class ContextValidatorVisitor:
            def __init__(self) -> None:
                self.contexts: list[dict[str, Any]] = []

            def visit_element_start(self, ctx: dict[str, Any]) -> dict[str, str]:
                self.contexts.append(ctx)
                return {"type": "continue"}

        visitor = ContextValidatorVisitor()
        html = """
            <div id="test" class="container">
                <p>Text</p>
                <span data-value="123">Inline</span>
            </div>
        """
        convert_with_visitor(html, visitor=visitor)

        # FAILING ASSERTIONS: All contexts should have required fields
        assert len(visitor.contexts) > 0, "Expected to capture element contexts"

        required_fields = ["tag_name", "attributes", "depth", "node_type", "is_inline"]
        for ctx in visitor.contexts:
            for field in required_fields:
                assert field in ctx, f"Expected field '{field}' in context for tag '{ctx.get('tag_name')}'"

    def test_tag_name_is_lowercase_string(self) -> None:
        """Test that tag_name is always a lowercase string."""

        class TagNameTypeVisitor:
            def __init__(self) -> None:
                self.tag_names: list[str] = []

            def visit_element_start(self, ctx: dict[str, Any]) -> dict[str, str]:
                tag_name = ctx.get("tag_name")
                if tag_name is not None:
                    self.tag_names.append(tag_name)
                return {"type": "continue"}

        visitor = TagNameTypeVisitor()
        html = """
            <DIV>Case sensitive</DIV>
            <P>Paragraph</P>
            <span>Inline</span>
        """
        convert_with_visitor(html, visitor=visitor)

        # FAILING ASSERTIONS: All tag names should be strings and lowercase
        assert len(visitor.tag_names) > 0, "Expected to capture tag names"
        for tag_name in visitor.tag_names:
            assert isinstance(tag_name, str), f"Expected tag_name to be string, got {type(tag_name)}"
            assert tag_name == tag_name.lower(), f"Expected tag_name '{tag_name}' to be lowercase"


class TestVisitorNestedElementTagNames:
    """Test tag name correctness in nested element structures."""

    def test_nested_divs_have_correct_tag_names(self) -> None:
        """Test that nested divs are all correctly identified as 'div'."""

        class NestedDivVisitor:
            def __init__(self) -> None:
                self.div_depths: list[int] = []

            def visit_element_start(self, ctx: dict[str, Any]) -> dict[str, str]:
                if ctx.get("tag_name") == "div":
                    self.div_depths.append(ctx.get("depth", 0))
                return {"type": "continue"}

        visitor = NestedDivVisitor()
        html = """
            <div id="outer">
                <div id="middle">
                    <div id="inner">
                        Content
                    </div>
                </div>
            </div>
        """
        convert_with_visitor(html, visitor=visitor)

        # FAILING ASSERTION: Should identify all divs correctly
        assert len(visitor.div_depths) == 3, f"Expected 3 divs, found {len(visitor.div_depths)}"

    def test_mixed_nested_elements_have_correct_tag_names(self) -> None:
        """Test that mixed nested elements maintain correct tag identification."""

        class MixedNestedVisitor:
            def __init__(self) -> None:
                self.tag_hierarchy: list[tuple[str, int]] = []

            def visit_element_start(self, ctx: dict[str, Any]) -> dict[str, str]:
                tag_name = ctx.get("tag_name")
                depth = ctx.get("depth", 0)
                if tag_name:
                    self.tag_hierarchy.append((tag_name, depth))
                return {"type": "continue"}

        visitor = MixedNestedVisitor()
        html = """
            <div>
                <p>
                    <span>Text</span>
                </p>
                <section>
                    <h2>Heading</h2>
                    <p>More text</p>
                </section>
            </div>
        """
        convert_with_visitor(html, visitor=visitor)

        # FAILING ASSERTIONS: Should preserve correct tag names at each level
        tag_names = [tag for tag, _ in visitor.tag_hierarchy]
        assert "div" in tag_names, "Expected to find div in hierarchy"
        assert tag_names.count("p") == 2, "Expected to find exactly 2 p elements"
        assert "span" in tag_names, "Expected to find span in hierarchy"
        assert "section" in tag_names, "Expected to find section in hierarchy"


class TestVisitorTagNameUseCase:
    """Test real-world use cases for tag_name in visitor pattern."""

    def test_collect_all_headings_by_level(self) -> None:
        """Test collecting headings only by correctly identifying h1-h6 tags."""

        class HeadingCollectorVisitor:
            def __init__(self) -> None:
                self.headings_by_level: dict[str, list[str]] = {}

            def visit_element_start(self, ctx: dict[str, Any]) -> dict[str, str]:
                tag_name = ctx.get("tag_name")

                # Only process heading tags
                if (
                    tag_name
                    and tag_name.startswith("h")
                    and len(tag_name) == 2
                    and tag_name[1].isdigit()
                    and tag_name not in self.headings_by_level
                ):
                    # This would normally be done in visit_heading, but demonstrates tag_name usage
                    self.headings_by_level[tag_name] = []

                return {"type": "continue"}

        visitor = HeadingCollectorVisitor()
        html = """
            <h1>Title</h1>
            <h2>Section</h2>
            <h3>Subsection</h3>
            <div>Not a heading</div>
        """
        convert_with_visitor(html, visitor=visitor)

        # FAILING ASSERTION: Should identify all heading levels
        assert len(visitor.headings_by_level) >= 3, "Expected to identify h1, h2, h3"
        assert "h1" in visitor.headings_by_level, "Expected to find h1"
        assert "h2" in visitor.headings_by_level, "Expected to find h2"
        assert "h3" in visitor.headings_by_level, "Expected to find h3"
        assert "div" not in visitor.headings_by_level, "Expected not to classify div as heading"

    def test_extract_metadata_from_specific_elements(self) -> None:
        """Test extracting metadata by relying on correct tag_name identification."""

        class MetadataExtractorVisitor:
            def __init__(self) -> None:
                self.metadata: dict[str, Any] = {
                    "title": None,
                    "links": [],
                    "images": [],
                }

            def visit_element_start(self, ctx: dict[str, Any]) -> dict[str, str]:
                tag_name = ctx.get("tag_name")
                attributes = ctx.get("attributes", {})

                if tag_name == "title":
                    self.metadata["title"] = attributes
                elif tag_name == "a":
                    self.metadata["links"].append(attributes.get("href"))
                elif tag_name == "img":
                    self.metadata["images"].append(attributes.get("src"))

                return {"type": "continue"}

        visitor = MetadataExtractorVisitor()
        html = """
            <div>
                <title>Page Title</title>
                <a href="https://example.com">Link</a>
                <img src="/image.jpg" alt="Image" />
            </div>
        """
        convert_with_visitor(html, visitor=visitor)

        # FAILING ASSERTIONS: Should extract metadata based on correct tag identification
        assert visitor.metadata["links"] != [], "Expected to find links"
        assert "https://example.com" in visitor.metadata["links"], "Expected to extract link href"
        assert visitor.metadata["images"] != [], "Expected to find images"
        assert "/image.jpg" in visitor.metadata["images"], "Expected to extract image src"
