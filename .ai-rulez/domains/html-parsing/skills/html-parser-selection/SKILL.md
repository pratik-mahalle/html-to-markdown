---
description: "Html Parser Selection"
name: html-parser-selection
---

Choose and configure optimal HTML parser for use case

Key source files:

- crates/html-to-markdown/src/wrapper.rs
- crates/html-to-markdown/src/converter.rs

Master concepts:

- html5ever vs tl/astral-tl trade-offs
- Parser configuration options
- Parser selection heuristics
- Fallback behavior

Step by step:

1. Evaluate input HTML characteristics
2. Consider standards compliance need (html5ever for high confidence)
3. Consider performance requirements (tl for known-good HTML)
4. Configure selected parser
5. Set encoding detection
6. Apply namespace handling options
7. Configure entity resolution
