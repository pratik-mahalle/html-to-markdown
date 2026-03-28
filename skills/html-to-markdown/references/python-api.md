# Python API Reference

Package name: `html-to-markdown`
Import: `from html_to_markdown import ...`
Python requirement: 3.10+

## Primary Function

```python
def convert(
    html: str,
    options: ConversionOptions | None = None,
    preprocessing: PreprocessingOptions | None = None,
) -> ExtractionResult:
    ...
```

Returns an `ExtractionResult` TypedDict (dict) with all extracted data in a single pass.

```python
from html_to_markdown import convert, ConversionOptions, PreprocessingOptions

# Simple
result = convert("<h1>Hello</h1><p>World</p>")
print(result["content"])        # "# Hello\n\nWorld\n"
print(result["tables"])         # []
print(result["warnings"])       # []
print(result["metadata"])       # dict with document, headers, links, images, structured_data

# With options
result = convert(
    html,
    options=ConversionOptions(heading_style="atx", code_block_style="backticks"),
    preprocessing=PreprocessingOptions(enabled=True, preset="aggressive"),
)
print(result["content"])
```

## ExtractionResult (TypedDict)

```python
class ExtractionResult(TypedDict):
    content: str | None                   # Converted markdown/djot/plain text
    document: None                        # Document structure (not yet wired)
    metadata: dict | None                 # HtmlMetadata dict (or None if unavailable)
    tables: list[ExtractedTable]          # Extracted tables
    images: list                          # Extracted inline images (if extract_images=True)
    warnings: list[ProcessingWarning]     # Non-fatal warnings

class ExtractedTable(TypedDict):
    grid: TableGrid
    markdown: str

class TableGrid(TypedDict):
    rows: int
    cols: int
    cells: list[GridCell]

class GridCell(TypedDict):
    content: str
    row: int
    col: int
    row_span: int
    col_span: int
    is_header: bool

class ProcessingWarning(TypedDict):
    message: str
    kind: str           # "image_extraction_failed" | "encoding_fallback" | ...
```

## ConversionOptions (dataclass)

```python
from html_to_markdown import ConversionOptions

@dataclass
class ConversionOptions:
    # Headings
    heading_style: Literal["underlined", "atx", "atx_closed"] = "atx"

    # Lists
    list_indent_type: Literal["spaces", "tabs"] = "spaces"
    list_indent_width: int = 2
    bullets: str = "-*+"

    # Emphasis
    strong_em_symbol: Literal["*", "_"] = "*"

    # Escaping
    escape_asterisks: bool = False
    escape_underscores: bool = False
    escape_misc: bool = False
    escape_ascii: bool = False

    # Code
    code_language: str = ""
    code_block_style: Literal["indented", "backticks", "tildes"] = "backticks"

    # Links
    autolinks: bool = True
    default_title: bool = False

    # Images
    keep_inline_images_in: set[str] | None = None
    skip_images: bool = False
    extract_images: bool = False
    max_image_size: int = 5_242_880     # 5 MiB
    capture_svg: bool = False
    infer_dimensions: bool = True

    # Tables
    br_in_tables: bool = False

    # Highlight
    highlight_style: Literal["double-equal", "html", "bold"] = "double-equal"

    # Metadata
    extract_metadata: bool = True

    # Whitespace
    whitespace_mode: Literal["normalized", "strict"] = "normalized"
    strip_newlines: bool = False

    # Wrapping
    wrap: bool = False
    wrap_width: int = 80

    # Element handling
    strip_tags: set[str] | None = None
    preserve_tags: set[str] | None = None
    convert_as_inline: bool = False

    # Subscript / superscript
    sub_symbol: str = ""
    sup_symbol: str = ""

    # Newlines
    newline_style: Literal["spaces", "backslash"] = "spaces"

    # Output format
    output_format: Literal["markdown", "djot"] = "markdown"

    # Document structure
    include_document_structure: bool = False

    # Encoding and debug
    encoding: str = "utf-8"
    debug: bool = False
```

## PreprocessingOptions (dataclass)

```python
from html_to_markdown import PreprocessingOptions

@dataclass
class PreprocessingOptions:
    enabled: bool = True
    preset: Literal["minimal", "standard", "aggressive"] = "standard"
    remove_navigation: bool = True
    remove_forms: bool = True
```

## Accessing Metadata, Tables, and Images

All structured data is in the `ExtractionResult` dict returned by `convert()`. Use `ConversionOptions` fields to control what is extracted:

```python
from html_to_markdown import convert, ConversionOptions

# Metadata — enabled by default
result = convert(html)
meta = result["metadata"]
print(meta["document"]["title"])
print(meta["headers"])
print(meta["links"])

# Tables — always present in result
for table in result["tables"]:
    print(table["markdown"])
    for cell in table["grid"]["cells"]:
        print(cell["content"])

# Inline images — set extract_images=True
result = convert(html, ConversionOptions(extract_images=True))
for image in result["images"]:
    print(image["format"], image["filename"])

# Document structure — set include_document_structure=True
result = convert(html, ConversionOptions(include_document_structure=True))
doc = result["document"]

# Plain string output
markdown: str = result["content"]
```

## Performance Handles (reuse parsed options)

```python
def create_options_handle(
    options: ConversionOptions | None = None,
    preprocessing: PreprocessingOptions | None = None,
) -> ConversionOptionsHandle:
    ...

def convert_with_handle(html: str, handle: ConversionOptionsHandle) -> str:
    ...
```

`convert_with_handle()` returns a plain `str` (the Markdown text), not an `ExtractionResult`. Use it for high-throughput scenarios where you only need the converted text and want to avoid the overhead of building the full result dict.

Use handles when converting many documents with the same options — avoids re-parsing options on each call.

## MetadataConfig

```python
from html_to_markdown._html_to_markdown import MetadataConfig

config = MetadataConfig(
    extract_document=True,
    extract_headers=True,
    extract_links=True,
    extract_images=True,
    extract_structured_data=True,
    max_structured_data_size=10_000,
)
```

## InlineImageConfig

```python
from html_to_markdown._html_to_markdown import InlineImageConfig

config = InlineImageConfig(
    max_decoded_size_bytes=5_242_880,   # 5 MiB
    filename_prefix=None,
    capture_svg=False,
    infer_dimensions=True,
)
```

## InlineImage (TypedDict)

```python
class InlineImage(TypedDict):
    data: bytes
    format: str              # "png" | "jpg" | "gif" | "webp" | "svg"
    filename: str | None
    description: str | None  # alt text
    dimensions: tuple[int, int] | None
    source: Literal["img_data_uri", "svg_element"]
    attributes: dict[str, str]
```

## Error Handling

```python
from html_to_markdown import convert
from html_to_markdown.exceptions import ConversionError

try:
    result = convert(html)
except ConversionError as e:
    print(f"Conversion failed: {e}")
except ValueError as e:
    print(f"Invalid input: {e}")
```

## Async Tip

The `convert()` function is synchronous but releases the GIL (Python GVL) during the Rust computation. For CPU-bound workloads, use `asyncio.to_thread()` or a thread pool:

```python
import asyncio
from html_to_markdown import convert

async def convert_async(html: str):
    return await asyncio.to_thread(convert, html)
```
