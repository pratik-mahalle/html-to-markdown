//! Regression coverage for issue #190.

use html_to_markdown_rs::{CodeBlockStyle, ConversionOptions, convert};

#[test]
fn test_code_block_dedent_handles_unicode_whitespace() {
    let nbsp = '\u{00A0}';
    let html = format!("<pre><code> msg = String()\n{nbsp}msg = String()\n</code></pre>");
    let options = ConversionOptions {
        code_block_style: CodeBlockStyle::Backticks,
        ..Default::default()
    };

    let markdown = convert(&html, Some(options)).expect("conversion should succeed");

    assert!(markdown.contains("msg = String()"));
    assert!(!markdown.contains(nbsp));
}

#[test]
fn test_convert_strips_nul_bytes() {
    let html = "a\0b";
    let markdown = convert(html, None).expect("conversion should succeed");

    assert_eq!(markdown, "ab\n");
}
