# CLI Usage

html-to-markdown includes a command-line tool for converting HTML files, URLs, and stdin to Markdown.

---

## Installation

### Via Cargo (Rust toolchain)

```bash
cargo install html-to-markdown-cli
```

### Via Homebrew (macOS / Linux)

```bash
brew install kreuzberg-dev/tap/html-to-markdown
```

### From Source

```bash
git clone https://github.com/kreuzberg-dev/html-to-markdown.git
cd html-to-markdown
cargo build --release --package html-to-markdown-cli
# Binary at target/release/html-to-markdown
```

---

## Basic Usage

### Convert from stdin

Pipe HTML into the command:

```bash
echo '<h1>Title</h1><p>Content</p>' | html-to-markdown
```

Output:

```
# Title

Content
```

### Convert a file

```bash
html-to-markdown input.html
```

### Convert and save to file

```bash
html-to-markdown input.html -o output.md
```

### Fetch and convert a URL

```bash
html-to-markdown --url https://example.com
```

Save the result:

```bash
html-to-markdown --url https://example.com -o page.md
```

Specify a custom User-Agent:

```bash
html-to-markdown --url https://example.com --user-agent "MyBot/1.0"
```

---

## Options

### Heading Options

```bash
# ATX style (default): # Heading
html-to-markdown input.html --heading-style atx

# Underlined style: Heading followed by ======
html-to-markdown input.html --heading-style underlined

# ATX closed style: # Heading #
html-to-markdown input.html --heading-style atx-closed
```

### List Options

```bash
# Custom bullet characters (cycle through nesting levels)
html-to-markdown input.html --bullets '*+-'

# List indent width (default 2)
html-to-markdown input.html --list-indent-width 4

# Tab indentation instead of spaces
html-to-markdown input.html --list-indent-type tabs
```

### Code Block Options

```bash
# Fenced with backticks (default)
html-to-markdown input.html --code-block-style backticks

# Fenced with tildes
html-to-markdown input.html --code-block-style tildes

# Indented (4-space)
html-to-markdown input.html --code-block-style indented

# Default language for unlabeled code blocks
html-to-markdown input.html --code-language python
```

### Text Formatting

```bash
# Use underscores for bold/italic (_italic_ instead of *italic*)
html-to-markdown input.html --strong-em-symbol '_'

# Escape special characters
html-to-markdown input.html --escape-asterisks --escape-underscores

# Custom subscript/superscript symbols
html-to-markdown input.html --sub-symbol '~' --sup-symbol '^'

# Newline style for <br> tags
html-to-markdown input.html --newline-style backslash  # default
html-to-markdown input.html --newline-style spaces
```

### Link Options

```bash
# Convert matching links to autolinks: <https://example.com>
html-to-markdown input.html --autolinks

# Add URL as default title when no title attribute exists
html-to-markdown input.html --default-title
```

### Wrapping

```bash
# Enable wrapping at 80 columns (default width)
html-to-markdown input.html --wrap

# Custom wrap width
html-to-markdown input.html --wrap --wrap-width 120
```

### Whitespace

```bash
# Strict whitespace preservation
html-to-markdown input.html --whitespace-mode strict

# Strip newlines before processing (useful for minified HTML)
html-to-markdown input.html --strip-newlines
```

### Metadata Extraction

Extract metadata alongside conversion. When `--with-metadata` is used, the output is JSON with `markdown` and `metadata` fields:

```bash
# Extract all metadata
html-to-markdown input.html --with-metadata

# Extract specific metadata categories
html-to-markdown input.html --with-metadata --extract-headers --extract-links

# Extract everything
html-to-markdown input.html --with-metadata \
    --extract-document --extract-headers --extract-links \
    --extract-images --extract-structured-data

# Save metadata output to file
html-to-markdown input.html --with-metadata -o output.json
```

The JSON output structure:

```json
{
  "markdown": "# Title\n\nContent...",
  "metadata": {
    "document": {"title": "Page Title", "description": "..."},
    "headers": [{"level": 1, "text": "Title", "id": null}],
    "links": [{"href": "https://...", "text": "...", "link_type": "external"}],
    "images": [{"src": "https://...", "alt": "...", "image_type": "external"}],
    "structured_data": []
  }
}
```

### Preprocessing

Clean up web content before conversion:

```bash
# Standard preprocessing
html-to-markdown input.html --preprocess

# Aggressive mode (maximum cleanup for web scraping)
html-to-markdown input.html --preprocess --preset aggressive

# Keep specific elements during preprocessing
html-to-markdown input.html --preprocess --keep-navigation --keep-forms
```

### Element Handling

```bash
# Treat block elements as inline
html-to-markdown input.html --convert-as-inline

# Strip specific HTML tags (extract text only)
html-to-markdown input.html --strip-tags 'script,style,nav'
```

### Highlight Style

```bash
# Double equal signs (default): ==highlighted==
html-to-markdown input.html --highlight-style double-equal

# Raw HTML: <mark>highlighted</mark>
html-to-markdown input.html --highlight-style html

# Bold: **highlighted**
html-to-markdown input.html --highlight-style bold
```

### Output Format

```bash
# Markdown (default)
html-to-markdown input.html --output-format markdown

# Djot
html-to-markdown input.html --output-format djot
```

### Encoding

```bash
# Specify input encoding (default: utf-8)
html-to-markdown input.html --encoding latin-1
```

### Debug Mode

```bash
# Show diagnostic information
html-to-markdown input.html --debug
```

---

## Shell Completions

Generate shell completion scripts for your shell:

```bash
# Bash
html-to-markdown --generate-completion bash > html-to-markdown.bash
source html-to-markdown.bash

# Zsh
html-to-markdown --generate-completion zsh > _html-to-markdown
# Place in your $fpath

# Fish
html-to-markdown --generate-completion fish > html-to-markdown.fish
source html-to-markdown.fish

# PowerShell
html-to-markdown --generate-completion powershell > html-to-markdown.ps1

# Elvish
html-to-markdown --generate-completion elvish > html-to-markdown.elv
```

---

## Man Page

Generate a man page:

```bash
html-to-markdown --generate-man > html-to-markdown.1
man ./html-to-markdown.1
```

---

## Examples

### Web Scraping Pipeline

Fetch a page, preprocess it, and extract clean Markdown:

```bash
html-to-markdown --url https://example.com/article \
    --preprocess --preset aggressive \
    --wrap --wrap-width 80 \
    -o article.md
```

### Batch File Conversion

Convert all HTML files in a directory:

```bash
for f in *.html; do
    html-to-markdown "$f" -o "${f%.html}.md"
done
```

### Extract Metadata for Indexing

```bash
html-to-markdown --url https://example.com \
    --with-metadata \
    --extract-document --extract-headers --extract-links \
    -o page-index.json
```

### Discord/Slack-Friendly Output

Use compact indentation and simple formatting:

```bash
html-to-markdown input.html \
    --list-indent-width 2 \
    --bullets '-' \
    --code-block-style backticks
```

### Pipe with curl

```bash
curl -s https://example.com | html-to-markdown --preprocess
```

---

## Further Reading

- [Basic Conversion](basic-conversion.md) -- programmatic API usage
- [Configuration Options](configuration.md) -- all options explained in detail
- [Metadata Extraction](metadata.md) -- metadata API guide
