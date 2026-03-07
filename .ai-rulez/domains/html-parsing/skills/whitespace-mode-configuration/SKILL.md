---
name: whitespace-mode-configuration
---
Apply configurable whitespace handling strategy

Key source files:
- crates/html-to-markdown/src/options.rs (WhitespaceMode)

Master concepts:
- Preserve mode (keep all whitespace)
- Minimal mode (trim/normalize)
- Collapse mode (HTML5-style)
- Mode application per element

Step by step:
1. Get WhitespaceMode from ConversionOptions
2. For text extraction apply mode
   - Preserve means return text as-is
   - Minimal means trim and normalize internal
   - Collapse means apply HTML5 whitespace rules
3. HTML5 collapse rules
   a. Replace newline and spaces with single space
   b. Collapse multiple spaces to one
   c. Preserve leading and trailing in some contexts
4. Apply mode consistently across elements
