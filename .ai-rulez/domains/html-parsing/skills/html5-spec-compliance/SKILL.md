---
description: "Html5 Spec Compliance"
name: html5-spec-compliance
---

Ensure HTML5 spec compliance in parsing

Key source files:

- crates/html-to-markdown/src/converter.rs

Master concepts:

- HTML5 spec recovery rules
- Malformed HTML handling
- Entity handling per spec
- Namespace support

Step by step:

1. Test against HTML5 spec test suite
2. Verify recovery rules for common errors
   - Unclosed tags auto-close per algorithm
   - Missing required tags insert implicitly
   - Mismatched tags fix and close properly
3. Test entity handling including numeric and named entities
4. Test namespace support for SVG and MathML
5. Document deviations from spec
