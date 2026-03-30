# Other Language Bindings

Brief reference for Go, Ruby, PHP, Java, C#, Elixir, R, WASM, and C FFI.

---

## Go

**Module:** `github.com/kreuzberg-dev/html-to-markdown/packages/go/v2`
**Package:** `htmltomarkdown`
**Install:** `go get github.com/kreuzberg-dev/html-to-markdown/packages/go/v2`

Uses cgo with the C FFI layer. Options are passed as JSON strings internally.

```go
import "github.com/kreuzberg-dev/html-to-markdown/packages/go/v2/htmltomarkdown"

// Primary function — returns ExtractionResult
result, err := htmltomarkdown.Convert(html)
if err != nil {
    log.Fatal(err)
}
fmt.Println(result.Content)        // markdown string
fmt.Println(len(result.Tables))    // extracted tables
fmt.Println(len(result.Warnings))  // processing warnings

// Must-or-panic variant
result = htmltomarkdown.MustConvert(html)

// Metadata is in result.Metadata when metadata extraction is enabled
fmt.Println(result.Metadata)

// Tables are always in result.Tables
for _, table := range result.Tables {
    fmt.Println(table.Markdown)
}

// Version
version := htmltomarkdown.Version()
```

### Go ExtractionResult

```go
type ExtractionResult struct {
    Content  string
    Tables   []TableData
    Warnings []ProcessingWarning
    Metadata interface{} // metadata map when metadata extraction is enabled
}
```

### Go ConversionOptions

Options are passed via JSON. See the Rust options for field names (use camelCase in JSON).

---

## Ruby

**Gem:** `html-to-markdown`
**Install:** `gem install html-to-markdown`
**Require:** `require 'html_to_markdown'`

Uses Magnus (native extension via Rust).

```ruby
require 'html_to_markdown'

# Primary function — returns a Hash
result = HtmlToMarkdown.convert(html)
puts result[:content]          # markdown string
puts result[:tables].length    # extracted tables
puts result[:warnings].length  # processing warnings
puts result[:metadata]         # metadata hash (or nil)

# With options (Hash)
result = HtmlToMarkdown.convert(html, {
    heading_style: "atx",
    code_block_style: "backticks",
    autolinks: true,
})

# Metadata — in result[:metadata]
result = HtmlToMarkdown.convert(html)
metadata = result[:metadata]

# Inline images — set extract_images: true
result = HtmlToMarkdown.convert(html, { extract_images: true })
images = result[:images]

# Tables — always in result[:tables]
result[:tables].each { |t| puts t[:markdown] }

# Reusable options handle (performance)
handle = HtmlToMarkdown.options({ heading_style: "atx" })
result = HtmlToMarkdown.convert(html, handle)
```

### Ruby convert() return Hash

```ruby
{
    content: String,              # markdown text
    document: nil,                # not yet wired
    metadata: Hash | nil,         # HtmlMetadata
    tables: Array,                # [{grid: {...}, markdown: "..."}]
    images: Array,                # inline images (if extract_images: true)
    warnings: Array               # [{message: "...", kind: "..."}]
}
```

---

## PHP

**Composer:** `kreuzberg-dev/html-to-markdown`
**Install:** `composer require kreuzberg-dev/html-to-markdown`
**PHP requirement:** 8.4+

Uses ext-php-rs (native PHP extension).

```php
<?php
declare(strict_types=1);

use HtmlToMarkdown\Converter;

// Primary function — returns array with content, metadata, tables, images, warnings
$result = Converter::convert('<h1>Hello</h1>');
$markdown = $result['content'];

// With options (associative array)
$result = Converter::convert('<h1>Hello</h1>', [
    'heading_style' => 'atx',
    'code_block_style' => 'backticks',
    'autolinks' => true,
]);

// Metadata — in $result['metadata']
$metadata = $result['metadata'];
echo $metadata['document']['title'];

// Tables — always in $result['tables']
foreach ($result['tables'] as $table) {
    echo $table['markdown'];
}

// Inline images — set extract_images: true
$result = Converter::convert('<img src="data:..." />', ['extract_images' => true]);
$images = $result['images'];
```

---

## Java

**Maven:** `dev.kreuzberg:html-to-markdown`
**GroupId:** `dev.kreuzberg`
**ArtifactId:** `html-to-markdown`
**Java requirement:** 21+ (uses Panama FFM API)

```xml
<dependency>
  <groupId>dev.kreuzberg</groupId>
  <artifactId>html-to-markdown</artifactId>
  <version>3.0.0</version>
</dependency>
```

```java
import dev.kreuzberg.htmltomarkdown.HtmlToMarkdown;
import dev.kreuzberg.htmltomarkdown.HtmlToMarkdown.ConversionResult;
import dev.kreuzberg.htmltomarkdown.HtmlToMarkdown.ConversionException;

// Primary function — returns ConversionResult
try {
    ConversionResult result = HtmlToMarkdown.convert("<h1>Hello</h1>");
    System.out.println(result.content());    // markdown string
    System.out.println(result.tables());     // List<TableData>
    System.out.println(result.warnings());   // List<ProcessingWarning>
    System.out.println(result.metadata());   // metadata map (when enabled)
} catch (ConversionException e) {
    System.err.println("Conversion failed: " + e.getMessage());
}

// Tables — always in result.tables()
for (var table : result.tables()) {
    System.out.println(table.markdown());
}

// Metadata — in result.metadata() when metadata extraction is enabled
var meta = result.metadata();

// Version
String version = HtmlToMarkdown.getVersion();
```

### ConversionOptions (Java)

```java
import dev.kreuzberg.htmltomarkdown.ConversionOptions;

ConversionOptions options = new ConversionOptions();
options.setHeadingStyle("atx");
options.setCodeBlockStyle("backticks");
options.setAutolinks(true);

// Pass as JSON string to FFI — options are serialized internally
```

---

## C# (.NET)

**NuGet:** `KreuzbergDev.HtmlToMarkdown`
**Install:** `dotnet add package KreuzbergDev.HtmlToMarkdown`
**.NET requirement:** 6+

```csharp
using HtmlToMarkdown;

// Primary function
var result = HtmlToMarkdownConverter.Convert("<h1>Hello</h1>");
Console.WriteLine(result.Content);         // markdown string
Console.WriteLine(result.Tables.Count);    // table count
Console.WriteLine(result.Warnings.Count);  // warning count
Console.WriteLine(result.Metadata?.Document?.Title);  // metadata (when enabled)

// ReadOnlySpan<byte> overload (avoids string allocation)
var bytes = System.Text.Encoding.UTF8.GetBytes(html);
var result2 = HtmlToMarkdownConverter.Convert(bytes.AsSpan());

// Tables — always in result.Tables
foreach (var table in result.Tables) {
    Console.WriteLine(table.Markdown);
}

// Inline images — set ExtractImages = true in options
var options = new ConversionOptions { ExtractImages = true };
var result3 = HtmlToMarkdownConverter.Convert(html, options);
foreach (var image in result3.Images) {
    Console.WriteLine(image.Format);
}
```

### HtmlToMarkdownException

```csharp
try {
    var result = HtmlToMarkdownConverter.Convert(html);
} catch (HtmlToMarkdownException e) {
    Console.Error.WriteLine($"Conversion failed: {e.Message}");
}
```

---

## Elixir

**Hex:** `html_to_markdown`
**Module:** `HtmlToMarkdown`
**Elixir requirement:** 1.14+ (uses Rustler NIFs)

```elixir
# mix.exs
{:html_to_markdown, "~> 3.0"}
```

```elixir
# Primary function — returns {:ok, map()} | {:error, term()}
{:ok, result} = HtmlToMarkdown.convert("<h1>Hello</h1>")
IO.puts result.content      # markdown string
IO.inspect result.tables    # list of table maps
IO.inspect result.warnings  # list of warning maps
IO.inspect result.metadata  # metadata map (when enabled)

# Bang variant (raises on error)
result = HtmlToMarkdown.convert!("<h1>Hello</h1>")

# With options
{:ok, result} = HtmlToMarkdown.convert(html, %{
    heading_style: "atx",
    code_block_style: "backticks",
})

# Metadata — in result.metadata
{:ok, result} = HtmlToMarkdown.convert(html)
metadata = result.metadata

# Tables — always in result.tables
Enum.each(result.tables, fn table -> IO.puts table.markdown end)

# Inline images — set extract_images: true
{:ok, result} = HtmlToMarkdown.convert(html, %{extract_images: true})
images = result.images

# Options handle (reuse for performance)
{:ok, handle} = HtmlToMarkdown.create_options_handle(%{heading_style: "atx"})
{:ok, result} = HtmlToMarkdown.convert(html, handle)
```

---

## R

**CRAN:** `htmltomarkdown`
**Install:** `install.packages("htmltomarkdown")`
**R requirement:** 4.1+

Uses extendr (Rust bindings for R).

```r
library(htmltomarkdown)

# Primary function
result <- convert("<h1>Hello</h1>")
cat(result$content)          # markdown string
length(result$tables)        # table count

# With options (named list)
result <- convert("<h1>Hello</h1>", list(
    heading_style = "atx",
    code_block_style = "backticks"
))

# Metadata — in result$metadata
result <- convert("<h1>Hello</h1>")
metadata <- result$metadata

# Tables — always in result$tables
for (table in result$tables) {
    cat(table$markdown)
}

# Inline images — set extract_images = TRUE
result <- convert("<img src='data:...' />", list(extract_images = TRUE))
images <- result$images

# Options handle (performance)
handle <- create_options_handle(list(heading_style = "atx"))
result <- convert_handle("<h1>Hello</h1>", handle)
```

---

## WASM

**Package:** `@kreuzberg/html-to-markdown-wasm` (built with wasm-pack)

```javascript
import init, { convert, convertBytes, createConversionOptionsHandle, convertWithOptionsHandle } from '@kreuzberg/html-to-markdown-wasm';

await init(); // initialize WASM module

// convert() — returns JSON string, always JSON.parse() the result
const result = JSON.parse(convert('<h1>Hello</h1>', {}));
console.log(result.content);    // markdown string
console.log(result.tables);     // extracted tables
console.log(result.metadata);   // metadata (when enabled)

// convertBytes() — accepts Uint8Array
const encoder = new TextEncoder();
const bytes = encoder.encode('<h1>Hello</h1>');
const result2 = JSON.parse(convertBytes(bytes, {}));

// Metadata — in result.metadata when extract_metadata is enabled
const result3 = JSON.parse(convert('<h1>Hello</h1>', { extractMetadata: true }));
console.log(result3.metadata);

// Tables — always in result.tables
for (const table of result.tables) {
    console.log(table.markdown);
}

// Options handle (reuse for performance)
const handle = createConversionOptionsHandle({ headingStyle: 'atx' });
const json = convertWithOptionsHandle('<h1>Hello</h1>', handle);
```

---

## C FFI

**Crate:** `html-to-markdown-ffi`
**Header:** `html_to_markdown_ffi.h` (generated by cbindgen)

Used internally by Go, Java, and C# bindings. Direct C usage:

```c
#include "html_to_markdown_ffi.h"

// convert() — returns malloc'd JSON string containing the full ConversionResult
// JSON has keys: content, tables, metadata, images, warnings
char* result_json = html_to_markdown_convert(html_cstr, options_json_cstr);
if (result_json) {
    // parse JSON result — content, tables, metadata, images, warnings are all here
    html_to_markdown_free_string(result_json);
}

// Always free returned strings with html_to_markdown_free_string()
// Never free with stdlib free() — use the library's free function
```

**Key FFI contract:** Every string returned by the library must be freed with `html_to_markdown_free_string()`. Never use the system `free()`.
