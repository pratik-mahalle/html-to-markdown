# Conversion Algorithms Domain

## Purpose

Core HTML-to-Markdown transformation logic. Converts parsed DOM trees into well-formatted Markdown output for 60+ HTML element types.

## Key Areas

- **Block elements**: headings, paragraphs, blockquotes, lists, tables, code blocks, horizontal rules, semantic HTML5 elements
- **Inline elements**: bold, italic, strikethrough, inline code, links, images, abbreviations
- **Tables**: GFM pipe tables with alignment, colspan/rowspan handling, complex table fallbacks
- **Lists**: ordered, unordered, nested, task lists, definition lists, tight vs loose detection
- **Forms & media**: input fields, textareas, selects, audio, video, iframes, embeds
- **Special elements**: line breaks, comments, SVG text extraction, ruby annotations

## Architecture

Visitor pattern in `visitor.rs` dispatches to per-element converter functions. Conversion behavior is controlled by `ConversionOptions` (heading style, list indent, code block style, newline style, table format).

## Dependencies

- Upstream: HTML Parsing domain (DOM tree), Safety-Sanitization domain (attribute validation)
- Downstream: Output formatting, metadata extraction
