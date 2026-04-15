# Safety & Sanitization Domain

## Purpose

Protects the conversion pipeline from malicious or malformed input. Ensures converted Markdown output cannot be exploited for XSS, code injection, or data exfiltration.

## Key Areas

- **Input validation**: binary data detection (magic numbers, null byte ratios, control char ratios), encoding detection, size/depth limits
- **XSS prevention**: dangerous element removal (script, style, iframe, object, embed), event handler stripping, javascript:/data:/vbscript: URL blocking
- **URL sanitization**: scheme whitelist (http, https, mailto, ftp), protocol normalization, URL-encoded payload detection, case-insensitive scheme matching
- **Attribute filtering**: event handler removal, safe attribute whitelist (id, class, title, alt, href, src), style sanitization
- **SVG handling**: script/style removal within SVG, event handler stripping, xlink:href validation, text extraction fallback
- **Runtime safety**: stack overflow prevention (max nesting depth), memory bounds enforcement, ReDoS prevention

## Architecture

Multi-layer defense: validate_input() -> sanitize -> parse -> convert with URL/attribute sanitization at each element. Configuration via `SafetyConfig` (max document size, max nesting depth, allowed tags/attributes/schemes, strip options).

## Dependencies

- Upstream: url, encoding_rs
- Downstream: HTML Parsing domain (operates on validated input), Conversion Algorithms domain (safe elements only)
