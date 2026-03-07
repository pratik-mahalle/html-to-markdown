---
name: Parser Robustness
priority: high
---
Parsers must handle malformed HTML without panicking

- Both html5ever and tl handle malformed HTML gracefully
- Validate parser output before using
- Test with HTML5 Test Suite examples
- Recover from common malformations (unclosed tags, bad attrs)
- Preserve as much content as possible on parse error
- Return clear error messages for unrecoverable input
