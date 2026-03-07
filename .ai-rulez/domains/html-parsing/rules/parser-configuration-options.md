---
name: Parser Configuration Options
priority: high
---
Provide comprehensive parser customization

- Expose ParserConfig struct with options:
  - parser_type: html5ever | tl
  - encoding: auto-detect | specified
  - namespace_handling: enabled | disabled
  - entity_resolution: decode | preserve
- Apply configuration at parse time
- Validate configuration values
- Document configuration impact on output
