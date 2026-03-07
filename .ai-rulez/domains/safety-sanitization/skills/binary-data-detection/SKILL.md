---
name: binary-data-detection
---
Detect and reject binary input before parsing

Key source files:
- crates/html-to-markdown/src/lib.rs (validate_input, detect_binary_magic)

Master concepts:
- Magic number identification
- UTF-16 detection via null bytes
- Control character ratio analysis
- Format-specific detection

Step by step:
1. Get input bytes
2. Check for magic number prefixes
   - 0x1F8B indicates gzip compressed
   - 0x28B52FFD indicates zstd compressed
   - 0x504B0304/0505/0708 indicates ZIP archive
   - 0x25504446 indicates PDF document
3. If no magic match, scan first 8KB:
   a. Count null bytes
   b. Count control characters
4. Calculate ratios:
   - Null byte ratio > 20%: UTF-16 detected
   - Control char ratio > 30%: binary detected
5. For UTF-16, check even/odd null distribution:
   - Even nulls (0, 2, 4...): UTF-16LE
   - Odd nulls (1, 3, 5...): UTF-16BE
6. Return specific error message
