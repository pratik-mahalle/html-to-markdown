---
name: converter-configuration-manager
description: Manage conversion options and settings
---
Source: crates/html-to-markdown/src/options.rs

Key concepts:
- HeadingStyle options
- ListIndentType options
- CodeBlockStyle options
- NewlineStyle options

Capabilities:
- Apply ConversionOptions to all converters
- Select heading style (ATX vs Setext)
- Configure list indentation
- Configure code block style
- Configure line break style
- Validate option values
- Provide sensible defaults

Patterns:
- Builder pattern for constructing options
- Applying options per-converter instance
- Validation of configuration values
- Composing configurations from defaults and overrides
