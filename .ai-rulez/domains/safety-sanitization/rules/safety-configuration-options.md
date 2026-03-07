---
name: Safety Configuration Options
priority: high
---
Provide comprehensive safety configuration

- SafetyConfig struct with options:
  - sanitize_html: bool (default: true)
  - max_document_size: usize (default: 50MB)
  - max_nesting_depth: usize (default: 256)
  - allowed_tags: Vec
  - allowed_attributes: Vec
  - allowed_url_schemes: Option<Vec>
  - strip_svg: bool
  - strip_comments: bool
  - strict_mode: bool
- Use builder pattern for configuration
- Validate configuration values
