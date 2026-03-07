---
name: Heading Conversion Accuracy
priority: high
---
Convert headings to correct Markdown syntax

- Support both ATX (# style) and Setext (underline) formats
- Make heading style configurable via HeadingStyle option
- ATX: "# h1\n## h2\n### h3" etc.
- Setext: "h1\n==\nh2\n--" (preferred for h1, h2 only)
- Preserve heading content with inline formatting
- Test h1-h6 conversion
- Verify proper spacing around headings
