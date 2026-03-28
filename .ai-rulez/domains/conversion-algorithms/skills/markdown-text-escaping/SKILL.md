---
description: "Markdown Text Escaping"
name: markdown-text-escaping
---

Escape Markdown metacharacters in text

Key source files:

- crates/html-to-markdown/src/text.rs

Master concepts:

- Metacharacter identification
- Escape rules
- Context-aware escaping
- Double-escape prevention

Step by step:

1. Identify Markdown metacharacters
   backslash, asterisk, underscore, backtick, brackets, parens, hash, plus, minus, dot, exclamation, pipe, greater-than, tilde, equals
2. For each metachar, check if escape needed
   - Before Markdown syntax means escape
   - In code block means no escape
   - In link text means escape
3. Escape with backslash for literal characters
4. Verify no double-escaping occurs
5. Preserve content accuracy
