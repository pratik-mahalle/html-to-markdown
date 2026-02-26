# Performance

html-to-markdown is designed for high-throughput HTML to Markdown conversion. The Rust core delivers performance characteristics that are difficult to achieve in interpreted languages, making it suitable for batch processing, real-time pipelines, and resource-constrained environments.

---

## Benchmarks

### Throughput

The core conversion engine processes HTML at **150--280 MB/s** on modern hardware (Apple M-series, Intel 12th gen+), depending on HTML complexity:

| Document Type | Throughput | Notes |
|--------------|------------|-------|
| Simple HTML (paragraphs, headings) | ~280 MB/s | Fast-path optimization for simple structures |
| Mixed content (lists, links, images) | ~200 MB/s | Typical web page content |
| Complex HTML (nested tables, forms) | ~150 MB/s | Deep nesting and table reconstruction |
| Plain text (no HTML tags) | ~400 MB/s | Bypasses parser entirely via fast path |

!!! info "Benchmark environment"
    Benchmarks use Criterion.rs with statistical analysis. Results measured on Apple M2 Pro, single-threaded. Real-world performance may vary based on hardware, document structure, and enabled features.

### Comparison with Python Alternatives

When accessed through the Python binding, html-to-markdown is **10--80x faster** than pure-Python alternatives:

| Library | 100 KB Document | Relative Speed |
|---------|-----------------|----------------|
| **html-to-markdown** (PyO3) | ~0.5 ms | **1x** (baseline) |
| markdownify | ~8 ms | ~16x slower |
| html2text | ~15 ms | ~30x slower |
| inscriptis | ~40 ms | ~80x slower |

The gap widens with larger documents because the Rust core's memory allocation patterns scale more efficiently than Python's garbage-collected heap.

---

## Why It Is Fast

### html5ever Parser

The HTML parser is [html5ever](https://crates.io/crates/html5ever), originally built for Mozilla's Servo browser engine. It is compiled to native code and implements the WHATWG HTML5 spec in a streaming, zero-copy manner where possible.

### Single-Pass Architecture

All operations -- Markdown generation, metadata extraction, inline image collection, and visitor callbacks -- happen in a single depth-first traversal of the DOM tree. There is no second pass, no intermediate representation, and no re-parsing.

### Fast Path for Plain Text

When the input contains no `<` characters, the parser is bypassed entirely. The text goes through entity decoding, whitespace normalization, and optional escaping -- all of which are simple string operations.

### Minimal Allocations

The converter pre-allocates output buffers based on input size and reuses them across elements. String operations use `Cow<str>` (clone-on-write) to avoid unnecessary copying when the input can be used directly.

### Compiled to Native Code

All language bindings (Python, TypeScript, Ruby, PHP, etc.) call directly into compiled Rust code. There is no interpretation overhead for the core conversion logic -- only thin FFI wrapper costs at the language boundary.

---

## Memory Efficiency

### Predictable Memory Usage

Memory consumption is proportional to:

- **Input size**: The DOM tree holds a reference-counted representation of the parsed HTML
- **Tree depth**: Stack usage grows with nesting depth (bounded by recursion limits)
- **Output size**: The Markdown output buffer is pre-allocated

For a typical web page (50--200 KB HTML), peak memory usage is approximately **2--3x the input size**.

### No Unbounded Buffers

- Structured data extraction is size-limited (`max_structured_data_size`, default 100 KB) to prevent memory exhaustion from large JSON-LD blocks
- Inline image extraction has configurable limits on the number of images collected
- The wrapper module processes output in chunks rather than buffering the entire document

---

## Streaming Strategies

For very large documents or high-throughput pipelines, consider these approaches:

### Batch Processing

Process multiple documents in parallel using your language's concurrency primitives. The converter is thread-safe and has no global state:

=== "Python"

    ```python
    from concurrent.futures import ThreadPoolExecutor
    from html_to_markdown import convert

    documents = [...]  # List of HTML strings

    with ThreadPoolExecutor(max_workers=8) as pool:
        results = list(pool.map(convert, documents))
    ```

=== "TypeScript"

    ```typescript
    import { convert } from '@kreuzberg/html-to-markdown';

    const documents: string[] = [...];
    const results = documents.map(convert);
    ```

=== "Rust"

    ```rust
    use rayon::prelude::*;
    use html_to_markdown_rs::convert;

    let documents: Vec<String> = vec![...];
    let results: Vec<String> = documents
        .par_iter()
        .map(|html| convert(html, None).unwrap())
        .collect();
    ```

### Chunked Input

For extremely large HTML files (100+ MB), consider splitting the HTML into logical sections before conversion. The converter handles fragments gracefully -- you do not need to provide a complete `<html>` document.

### Process Pool

For CPU-bound batch workloads in Python, use `ProcessPoolExecutor` to bypass the GIL and utilize multiple cores:

```python
from concurrent.futures import ProcessPoolExecutor
from html_to_markdown import convert

with ProcessPoolExecutor(max_workers=4) as pool:
    results = list(pool.map(convert, large_document_list))
```

---

## Optimization Tips

### 1. Disable Unused Features

If you do not need metadata extraction, visitor callbacks, or inline image extraction, disable those features to reduce overhead. In Rust, use feature flags. In language bindings, simply avoid calling `convert_with_metadata()` or `convert_with_visitor()` -- the plain `convert()` function skips all optional collectors.

### 2. Skip Wrapping When Not Needed

Text wrapping (`wrap: true`) adds a post-processing pass over the output. If your downstream consumer handles line wrapping, disable it:

```python
options = ConversionOptions(wrap=False)  # Default is False
```

### 3. Use the Right Heading Style

ATX headings (`# Heading`) are slightly faster to generate than Setext/underlined headings (`Heading\n=======`) because they do not require measuring the heading text length.

### 4. Avoid Unnecessary Escaping

The `escape_ascii` option escapes all ASCII punctuation for strict CommonMark compliance tests. It is rarely needed in production and adds overhead:

```python
# Only enable what you actually need
options = ConversionOptions(
    escape_asterisks=False,   # Default
    escape_underscores=False, # Default
    escape_misc=False,        # Default
    escape_ascii=False,       # Default -- leave this off
)
```

### 5. Preprocessing for Web Content

When converting scraped web pages, enable preprocessing to strip navigation, ads, and boilerplate before conversion. This reduces the HTML size and produces cleaner output:

```python
options = ConversionOptions(
    preprocessing=PreprocessingOptions(
        enabled=True,
        preset="aggressive",
    )
)
```

### 6. Reuse Options Objects

In hot loops, create the `ConversionOptions` object once and reuse it across calls to avoid repeated construction:

```python
options = ConversionOptions(heading_style="atx", list_indent_width=2)

for html in documents:
    markdown = convert(html, options)  # Reuse options
```

---

## Profiling

For Rust development, the project includes Criterion.rs benchmarks:

```bash
# Run all benchmarks
task bench

# Run specific benchmark
cargo bench --bench conversion

# Generate flamegraph (requires cargo-flamegraph)
cargo flamegraph --bench conversion
```

Benchmark results are stored as CI artifacts for regression detection. A slowdown of more than 5% from the baseline triggers a CI failure.

---

## Further Reading

- [Architecture](architecture.md) -- how the Rust core and bindings are structured
- [Conversion Pipeline](conversion-pipeline.md) -- detailed breakdown of each processing stage
- [kreuzberg](https://docs.kreuzberg.dev) -- document intelligence library that uses html-to-markdown for high-throughput HTML processing
