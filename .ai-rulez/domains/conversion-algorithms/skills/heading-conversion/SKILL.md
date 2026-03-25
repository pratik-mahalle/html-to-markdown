---
description: "Heading Conversion"
name: heading-conversion
---
Convert HTML heading tags to Markdown

Key source files:
- crates/html-to-markdown/src/converter.rs (convert_heading)

Master concepts:
- ATX style (#)
- Setext style (underline)
- HeadingStyle configuration
- Content preservation

Step by step:
1. Extract heading level from h1, h2, etc.
2. Extract heading text content
3. Check HeadingStyle option
   a. ATX mode generates # h1 or ## h2 etc.
   b. Setext mode generates underlines for h1 and h2
4. Preserve inline formatting (bold, italic, links)
5. Add blank line after heading
