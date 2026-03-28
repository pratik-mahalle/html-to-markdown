---
name: input-security-validator
description: Validate and sanitize HTML input before parsing
---

Source: crates/html-to-markdown/src/lib.rs (validate_input, detect_binary_*)

Key concepts:

- Binary data detection (magic numbers, null bytes)
- Character encoding validation
- Size limit enforcement
- Control character detection

Capabilities:

- Detect binary data formats (gzip, PDF, ZIP, UTF-16)
- Validate character encoding and convert to UTF-8
- Enforce size limits (document and output)
- Detect UTF-16 via null byte patterns
- Calculate control character ratio
- Provide clear error messages

Patterns:

- Magic number check first (0x1F8B for gzip)
- Scan first 8KB for binary indicators
- Detect UTF-16 patterns (even/odd null bytes)
- Validate UTF-8 byte sequences
