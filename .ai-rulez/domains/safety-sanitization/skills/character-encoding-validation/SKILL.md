---
name: character-encoding-validation
---
Detect and convert character encodings safely

Key source files:
- crates/html-to-markdown/src/lib.rs

Master concepts:
- BOM detection
- HTML5 charset meta tag
- Auto-detection via encoding_rs
- Safe UTF-8 conversion

Step by step:
1. Check for BOM (Byte Order Mark)
   - 0xEFBBBF indicates UTF-8 BOM
   - 0xFFFE indicates UTF-16LE BOM
   - 0xFEFF indicates UTF-16BE BOM
2. If no BOM, check HTML5 meta charset tag
   - Look for meta charset UTF-8
   - Look for meta http-equiv Content-Type
3. If no meta, auto-detect encoding
   - Use encoding_rs crate
   - Analyze byte patterns
4. Validate encoding is supported
5. Convert to UTF-8
   - Use encoding_rs for conversion
   - Verify valid UTF-8 output
6. Handle errors gracefully
