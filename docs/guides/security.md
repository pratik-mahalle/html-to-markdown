# Security

html-to-markdown is designed with security in mind. This guide covers the built-in safety mechanisms, how to configure them, and security considerations for different deployment scenarios.

---

## Built-in Input Validation

Before any parsing occurs, html-to-markdown validates input to reject non-HTML content that could cause unexpected behavior:

### Binary Data Rejection

The converter detects and rejects binary input by checking for:

- **PDF magic bytes** (`%PDF-`) -- prevents accidentally passing PDF files
- **High null-byte density** -- detects binary files, images, and other non-text content
- **Other binary signatures** -- common file format headers

```python
from html_to_markdown import convert, ConversionError

try:
    # This will raise ConversionError::InvalidInput
    markdown = convert(pdf_bytes.decode("utf-8", errors="replace"))
except ConversionError:
    print("Input is not valid HTML")
```

### UTF-16 Recovery

Rather than failing on UTF-16 encoded input (which appears as a mix of text and null bytes when decoded as UTF-8), the converter detects both UTF-16 LE and UTF-16 BE encodings (with or without BOM) and transparently converts them to UTF-8 before parsing.

### Null Byte Stripping

Stray null bytes in otherwise valid HTML (common in data from legacy systems) are silently stripped rather than causing parser failures.

---

## HTML Parsing Safety

html-to-markdown uses [html5ever](https://crates.io/crates/html5ever), the WHATWG-compliant HTML5 parser originally built for Mozilla Servo. This provides:

- **Standards-compliant error recovery** -- malformed HTML is handled the same way browsers handle it, rather than through custom (potentially vulnerable) error handling
- **No custom parsing** -- there is no regex-based or ad-hoc HTML parsing that could be exploited with crafted input
- **Bounded recursion** -- deeply nested HTML does not cause stack overflows

---

## Script and Style Handling

By default, `<script>` and `<style>` elements are **completely ignored** during conversion. Their content never appears in the Markdown output.

```python
html = """
<script>alert('xss')</script>
<style>body { display: none; }</style>
<p>Safe content</p>
"""

markdown = convert(html)
# Output: Safe content
```

!!! warning "Preserve tags and scripts"
    If you use `preserve_tags` to keep certain elements as raw HTML, be careful not to preserve `<script>` or `<style>` tags, as their content would then appear in the output.

---

## Link Security

### JavaScript URLs

html-to-markdown converts all links, including those with `javascript:` URLs, into Markdown link syntax. The link is preserved as-is in the Markdown output.

If your downstream Markdown renderer executes JavaScript URLs, use a [visitor](visitor.md) to filter them:

```python
class SafeLinksVisitor:
    BLOCKED_SCHEMES = {"javascript:", "data:", "vbscript:"}

    def visit_link(self, ctx, href, text, title):
        href_lower = href.lower().strip()
        for scheme in self.BLOCKED_SCHEMES:
            if href_lower.startswith(scheme):
                return {"type": "custom", "output": text}
        return {"type": "continue"}

markdown = convert_with_visitor(html, visitor=SafeLinksVisitor())
```

### Data URLs

Similarly, `data:` URLs in images are converted to standard Markdown image syntax. If you need to block data URIs:

```python
class NoDataUriImages:
    def visit_image(self, ctx, src, alt, title):
        if src.lower().startswith("data:"):
            return {"type": "skip"}
        return {"type": "continue"}
```

---

## Preprocessing for Untrusted HTML

When converting HTML from untrusted sources (web scraping, user input, email), use preprocessing to strip potentially dangerous or irrelevant content:

```python
from html_to_markdown import ConversionOptions, PreprocessingOptions, convert

options = ConversionOptions(
    preprocessing=PreprocessingOptions(
        enabled=True,
        preset="aggressive",
    ),
    strip_tags=["script", "style", "iframe", "object", "embed"],
)

markdown = convert(untrusted_html, options)
```

The `aggressive` preset removes:

- Navigation elements (`<nav>`, menus)
- Sidebars and footers
- Advertising containers
- Social media widgets
- Cookie consent banners
- Form elements
- Embedded objects

---

## Structured Data Size Limits

When using metadata extraction, the `max_structured_data_size` configuration prevents memory exhaustion from extremely large JSON-LD blocks:

```python
from html_to_markdown import MetadataConfig, convert_with_metadata

config = MetadataConfig(
    extract_structured_data=True,
    max_structured_data_size=50000,  # 50 KB limit (default is 100 KB)
)

markdown, metadata = convert_with_metadata(html, metadata_config=config)
```

!!! info "Why this matters"
    Some web pages embed megabytes of JSON-LD data (product catalogs, event listings). Without a size limit, extracting this data could consume excessive memory.

---

## Memory Safety

The Rust core of html-to-markdown enforces memory safety at the language level:

- **No unsafe code in the core crate** -- the workspace-level Cargo.toml sets `unsafe_code = "forbid"`
- **No buffer overflows** -- Rust's borrow checker prevents out-of-bounds access
- **No use-after-free** -- ownership rules prevent dangling pointer dereferences
- **No data races** -- thread safety is enforced at compile time

The FFI bindings (C, Go, Java, C#) do contain minimal `unsafe` code at the boundary between languages, which is necessary for FFI. Each `unsafe` block is documented with a `// SAFETY:` comment explaining why it is safe.

---

## Deployment Considerations

### Web Applications

When using html-to-markdown in a web application to convert user-submitted HTML:

1. **Always sanitize output** if the Markdown will be rendered back to HTML. html-to-markdown converts HTML to Markdown; it does not sanitize for XSS. Your Markdown renderer should handle sanitization.
2. **Use preprocessing** with the `aggressive` preset to strip irrelevant content.
3. **Set size limits** on input HTML to prevent denial-of-service via very large documents.
4. **Use visitors** to filter dangerous URL schemes.

### Content Migration

When migrating content between CMS platforms:

1. **Validate link integrity** using metadata extraction to catalog all links before and after conversion.
2. **Preserve important HTML** using `preserve_tags` for elements that cannot be represented in Markdown.
3. **Review structured data** extraction to ensure SEO metadata is maintained.

### Batch Processing

When processing large volumes of documents:

1. **Set timeouts** on conversion calls to prevent hangs on pathological input.
2. **Monitor memory usage** -- while the converter is memory-efficient, very large documents (100+ MB) can consume significant memory.
3. **Use process isolation** for untrusted input to limit the blast radius of unexpected behavior.

---

## Security Checklist

For production deployments processing untrusted HTML:

- [ ] Enable preprocessing with an appropriate preset
- [ ] Strip `<script>`, `<style>`, `<iframe>`, `<object>`, and `<embed>` tags
- [ ] Filter `javascript:`, `data:`, and `vbscript:` URL schemes via a visitor
- [ ] Set `max_structured_data_size` for metadata extraction
- [ ] Limit input size before passing to the converter
- [ ] Sanitize the Markdown output before rendering back to HTML
- [ ] Set conversion timeouts for user-facing applications
- [ ] Keep html-to-markdown updated to receive security fixes

---

## Reporting Security Issues

If you discover a security vulnerability in html-to-markdown, please report it responsibly. See the project's [security policy](https://github.com/kreuzberg-dev/html-to-markdown/security) for disclosure procedures.

---

## Further Reading

- [Visitor Pattern Guide](visitor.md) -- using visitors for security filtering
- [Configuration Options](configuration.md) -- preprocessing and strip_tags options
- [Architecture](../concepts/architecture.md) -- how the Rust core ensures memory safety
- [kreuzberg security](https://docs.kreuzberg.dev) -- security considerations for the broader document intelligence platform
