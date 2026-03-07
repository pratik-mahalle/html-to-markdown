---
name: Binary Data Detection
priority: high
---
Reject binary and non-UTF-8 input

- Check for magic number prefixes (gzip, zstd, ZIP, PDF)
- Scan first 8KB for binary indicators
- Detect UTF-16 via null byte patterns (>20% threshold)
- Calculate control character ratio (>30% threshold)
- Verify valid UTF-8 byte sequences
- Return clear error messages indicating detected format
- Never attempt to parse binary data
