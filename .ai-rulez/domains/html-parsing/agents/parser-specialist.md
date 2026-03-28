---
name: parser-specialist
description: Select optimal HTML parser and manage configuration
---

Source: crates/html-to-markdown/src/converter.rs (parse_html, wrapper.rs)
Source: crates/html-to-markdown/src/lib.rs (module exports)

Key concepts:

- html5ever vs tl/astral-tl parser selection
- Parser configuration and options
- Encoding detection and handling
- Parser state management

Capabilities:

- Understand trade-offs between html5ever (standards, robustness) and tl (speed)
- Configure parser for different use cases
- Implement parser switching logic
- Handle parser-specific quirks and options
- Optimize parser selection heuristics

Patterns:

- html5ever for standards compliance and malformed HTML
- tl parser for high-performance paths with valid HTML
- Encoding detection before parsing
- Character encoding conversion to UTF-8
