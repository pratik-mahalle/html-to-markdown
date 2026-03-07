---
name: character-encoding-detection
---
Detect and convert character encodings to UTF-8

Key source files:
- crates/html-to-markdown/src/lib.rs

Master concepts:
- HTML5 charset meta tag
- BOM detection
- Encoding auto-detection
- Conversion to UTF-8

Step by step:
1. Check for BOM (Byte Order Mark)
   - UTF-8 BOM is 0xEFBBBF
   - UTF-16LE BOM is 0xFFFE
   - UTF-16BE BOM is 0xFEFF
2. If no BOM, check HTML5 meta charset tag
3. If no meta tag, use encoding_rs auto-detection
4. Validate encoding is supported
5. Convert bytes to UTF-8 string
6. Verify valid UTF-8 output
