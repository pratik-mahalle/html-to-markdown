---
description: "Semantic Html5 Element Conversion"
name: semantic-html5-element-conversion
---

Convert semantic HTML5 elements to Markdown

Key source files:

- crates/html-to-markdown/src/converter.rs

Master concepts:

- Semantic elements (article, section, nav, aside)
- Conversion strategy (heading vs section marker)
- Content preservation

Step by step:

1. Identify semantic element type
2. Choose conversion strategy
   - article elements convert to optional heading or section
   - section elements convert to optional heading or section
   - nav elements convert to labeled list or section
   - aside elements convert to blockquote or indented section
   - header elements convert to optional heading
   - footer elements convert to optional separator
3. Apply strategy based on content
4. Preserve all content
