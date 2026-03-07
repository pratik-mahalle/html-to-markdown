---
name: input-validator
description: Validate HTML input before parsing
---
Source: crates/html-to-markdown/src/lib.rs (validate_input)

Key concepts:
- Binary data detection
- Encoding validation
- Size limit enforcement
- Control character detection

Capabilities:
- Detect binary data (gzip, PDF, ZIP, UTF-16)
- Validate character encoding
- Enforce size limits
- Identify malformed input
- Provide clear error messages

Patterns:
- Check magic number prefixes (0x1F8B for gzip)
- Calculate null byte and control character ratios
- Verify valid UTF-8 sequences
- Check document size before parsing
