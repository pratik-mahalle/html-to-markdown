---
description: "Input Binary Detection"
name: input-binary-detection
---

Detect binary and non-UTF-8 input before parsing

Key source files:

- crates/html-to-markdown/src/lib.rs (validate_input, detect_binary_*)

Master concepts:

- Magic number detection
- UTF-16 heuristics
- Control character ratio
- Encoding validation

Step by step:

1. Get input bytes
2. Check magic number prefixes
   - 0x1F8B indicates gzip
   - 0x28B52FFD indicates zstd
   - 0x504B0304 indicates ZIP
   - 0x25504446 indicates PDF
3. If no magic match, scan for binary indicators
   a. Count null bytes for UTF-16 detection
   b. Count control characters
   c. Calculate ratios
4. Detect UTF-16
   - Even null bytes indicate UTF-16LE
   - Odd null bytes indicate UTF-16BE
5. If binary detected, return specific error
