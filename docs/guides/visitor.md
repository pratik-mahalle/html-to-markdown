# Visitor Pattern Guide <span class="version-badge new">v2.23.0</span>

This guide shows how to create custom visitors that modify HTML-to-Markdown conversion behavior. Visitors let you intercept specific HTML elements and control their output without modifying the library.

For the conceptual overview, see [Visitor Pattern Concepts](../concepts/visitor-pattern.md).

---

## Basic Visitor Example

The simplest visitor implements one or more `visit_*` methods. Each method receives context about the current element and returns a result indicating how to handle it.

=== "Python"

    --8<-- "docs/snippets/python/visitor/basic_visitor.md"

=== "TypeScript"

    --8<-- "docs/snippets/typescript/visitor/basic_visitor.md"

=== "Ruby"

    --8<-- "docs/snippets/ruby/visitor/basic_visitor.md"

=== "PHP"

    --8<-- "docs/snippets/php/visitor/basic_visitor.md"

=== "C"

    --8<-- "docs/snippets/c/visitor/basic_visitor.md"

=== "Elixir"

    ```elixir
    defmodule MyVisitor do
      use HtmlToMarkdown.Visitor

      @impl true
      def handle_link(_context, _href, text, _title) do
        {:custom, text}
      end

      @impl true
      def handle_image(_context, _src, _alt, _title) do
        :skip
      end
    end

    html = "<p><a href='https://example.com'>Link</a> <img src='pic.png'></p>"
    {:ok, markdown} = HtmlToMarkdown.Visitor.convert_with_visitor(html, MyVisitor, nil)
    ```

=== "R"

    ```r
    library(htmltomarkdown)

    html <- "<p>Visit <a href='https://example.com'>our site</a></p>"
    markdown <- convert_with_visitor(html)
    cat(markdown)
    ```

=== "Rust"

    ```rust
    use html_to_markdown_rs::convert_with_visitor;
    use html_to_markdown_rs::visitor::{HtmlVisitor, NodeContext, VisitResult};

    #[derive(Debug)]
    struct StripLinksVisitor;

    impl HtmlVisitor for StripLinksVisitor {
        fn visit_link(
            &mut self,
            _ctx: &NodeContext,
            _href: &str,
            text: &str,
            _title: Option<&str>,
        ) -> VisitResult {
            // Convert links to plain text
            VisitResult::Custom(text.to_string())
        }

        fn visit_image(
            &mut self,
            _ctx: &NodeContext,
            _src: &str,
            _alt: &str,
            _title: Option<&str>,
        ) -> VisitResult {
            VisitResult::Skip
        }
    }
    ```

---

## Return Values

Every visitor callback must return one of these result types:

### Continue

Use the default conversion for this element. This is the default when a callback is not implemented.

```python
def visit_link(self, ctx, href, text, title):
    return {"type": "continue"}
```

### Custom

Replace the element's output with your own Markdown string:

```python
def visit_link(self, ctx, href, text, title):
    return {"type": "custom", "output": f"[{text}]({href})"}
```

### Skip

Remove the element entirely from the output:

```python
def visit_image(self, ctx, src, alt, title):
    return {"type": "skip"}
```

### Preserve HTML

Keep the raw HTML in the Markdown output:

```python
def visit_link(self, ctx, href, text, title):
    return {"type": "preserve_html"}
```

### Error

Stop conversion and return an error:

```python
def visit_link(self, ctx, href, text, title):
    if "javascript:" in href:
        return {"type": "error", "reason": "JavaScript URLs not allowed"}
    return {"type": "continue"}
```

---

## Common Use Cases

### Remove All Images

```python
class NoImagesVisitor:
    def visit_image(self, ctx, src, alt, title):
        return {"type": "skip"}

markdown = convert_with_visitor(html, visitor=NoImagesVisitor())
```

### Strip Links (Keep Text)

Convert links to plain text:

```python
class PlainTextLinksVisitor:
    def visit_link(self, ctx, href, text, title):
        return {"type": "custom", "output": text}

markdown = convert_with_visitor(html, visitor=PlainTextLinksVisitor())
```

### URL Rewriting

Rewrite relative URLs to absolute URLs:

=== "Python"

    ```python
    from urllib.parse import urljoin

    class AbsoluteUrlVisitor:
        def __init__(self, base_url):
            self.base_url = base_url

        def visit_link(self, ctx, href, text, title):
            absolute = urljoin(self.base_url, href)
            title_attr = f' "{title}"' if title else ""
            return {"type": "custom", "output": f"[{text}]({absolute}{title_attr})"}

        def visit_image(self, ctx, src, alt, title):
            absolute = urljoin(self.base_url, src)
            title_attr = f' "{title}"' if title else ""
            return {"type": "custom", "output": f"![{alt}]({absolute}{title_attr})"}

    visitor = AbsoluteUrlVisitor("https://example.com")
    markdown = convert_with_visitor(html, visitor=visitor)
    ```

=== "TypeScript"

    ```typescript
    import { convertWithVisitor, Visitor, NodeContext, VisitResult } from '@kreuzberg/html-to-markdown';

    const baseUrl = 'https://example.com';

    const visitor: Visitor = {
      visitLink(ctx: NodeContext, href: string, text: string): VisitResult {
        const absolute = new URL(href, baseUrl).toString();
        return { type: 'custom', output: `[${text}](${absolute})` };
      },
      visitImage(ctx: NodeContext, src: string, alt: string): VisitResult {
        const absolute = new URL(src, baseUrl).toString();
        return { type: 'custom', output: `![${alt}](${absolute})` };
      },
    };

    const markdown = convertWithVisitor(html, { visitor });
    ```

### Security Filtering

Block dangerous URLs:

```python
class SafeLinksVisitor:
    BLOCKED_SCHEMES = {"javascript:", "data:", "vbscript:"}

    def visit_link(self, ctx, href, text, title):
        href_lower = href.lower().strip()
        for scheme in self.BLOCKED_SCHEMES:
            if href_lower.startswith(scheme):
                return {"type": "custom", "output": text}  # Strip the link
        return {"type": "continue"}

markdown = convert_with_visitor(html, visitor=SafeLinksVisitor())
```

### Custom Heading Anchors

Add Markdown anchor syntax to headings:

```python
import re

class AnchoredHeadingsVisitor:
    def visit_heading(self, ctx, level, text, heading_id):
        slug = heading_id or re.sub(r'[^\w-]', '', text.lower().replace(' ', '-'))
        hashes = "#" * level
        return {"type": "custom", "output": f"{hashes} {text} {{#{slug}}}"}

markdown = convert_with_visitor(html, visitor=AnchoredHeadingsVisitor())
```

### Platform-Specific Formatting

Generate Slack-compatible Markdown:

```python
class SlackVisitor:
    def visit_strong(self, ctx, text):
        return {"type": "custom", "output": f"*{text}*"}

    def visit_emphasis(self, ctx, text):
        return {"type": "custom", "output": f"_{text}_"}

    def visit_strikethrough(self, ctx, text):
        return {"type": "custom", "output": f"~{text}~"}

    def visit_code_block(self, ctx, language, code):
        return {"type": "custom", "output": f"```\n{code}\n```"}
```

---

## Async Visitors

Python and TypeScript support async visitors for callbacks that need to perform I/O operations:

=== "Python"

    ```python
    import asyncio
    from html_to_markdown import convert_with_async_visitor

    class AsyncUrlValidator:
        async def visit_link(self, ctx, href, text, title):
            # Could check URL validity with an HTTP request
            if href.startswith("http"):
                return {"type": "continue"}
            return {"type": "custom", "output": text}

    markdown = convert_with_async_visitor(html, visitor=AsyncUrlValidator())
    ```

=== "TypeScript"

    ```typescript
    import { convertWithVisitor } from '@kreuzberg/html-to-markdown';

    const asyncVisitor = {
      async visitLink(ctx, href, text) {
        // Could perform async URL validation
        return { type: 'continue' };
      },
    };

    const markdown = await convertWithVisitor(html, { visitor: asyncVisitor });
    ```

!!! note "Async overhead"
    Async visitors have slightly more overhead than sync visitors due to the runtime bridging. Use sync visitors when your callbacks do not need I/O.

---

## Node Context

Every visitor callback receives a context object with metadata about the current element:

```python
def visit_link(self, ctx, href, text, title):
    print(ctx.tag_name)        # "a"
    print(ctx.depth)           # Nesting depth in DOM
    print(ctx.parent_tag)      # Parent element tag (e.g., "p")
    print(ctx.is_inline)       # True for inline elements
    print(ctx.attributes)      # Dict of HTML attributes
    print(ctx.index_in_parent) # Sibling index
    return {"type": "continue"}
```

You can use context to make conditional decisions. For example, only modify links inside a specific parent element:

```python
def visit_link(self, ctx, href, text, title):
    if ctx.parent_tag == "nav":
        return {"type": "skip"}  # Skip navigation links
    return {"type": "continue"}
```

---

## Combining Visitors with Options

Visitors work alongside `ConversionOptions`. Options control the default behavior, while visitors override specific elements:

```python
from html_to_markdown import ConversionOptions, convert_with_visitor

class MyVisitor:
    def visit_image(self, ctx, src, alt, title):
        return {"type": "skip"}

options = ConversionOptions(
    heading_style="atx",
    wrap=True,
    wrap_width=80,
)

markdown = convert_with_visitor(html, options, visitor=MyVisitor())
```

---

## Combining Visitors with Metadata

In Rust, the `convert_with_metadata` function accepts an optional visitor, allowing both metadata extraction and custom conversion in a single pass:

```rust
let visitor = Some(visitor_handle);
let (markdown, metadata) = convert_with_metadata(html, Some(options), config, visitor)?;
```

---

## Further Reading

- [Visitor Pattern Concepts](../concepts/visitor-pattern.md) -- complete callback reference and support matrix
- [Configuration Options](configuration.md) -- options that work alongside visitors
- [Security Guide](security.md) -- using visitors for security filtering
