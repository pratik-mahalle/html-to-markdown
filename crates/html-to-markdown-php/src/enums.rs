use ext_php_rs::prelude::*;
use ext_php_rs::types::Zval;
use html_to_markdown_rs::{
    CodeBlockStyle, HeadingStyle, HighlightStyle, ListIndentType, NewlineStyle, OutputFormat, PreprocessingPreset,
    WhitespaceMode,
};

/// Parse a heading style string into HeadingStyle enum.
pub fn parse_heading_style(value: &Zval, key: &str) -> PhpResult<HeadingStyle> {
    match read_string(value, key)?.as_str() {
        "underlined" => Ok(HeadingStyle::Underlined),
        "atx" => Ok(HeadingStyle::Atx),
        "atx_closed" => Ok(HeadingStyle::AtxClosed),
        other => Err(PhpException::default(format!("Invalid heading_style '{other}'"))),
    }
}

/// Parse a list indent type string into ListIndentType enum.
pub fn parse_list_indent_type(value: &Zval, key: &str) -> PhpResult<ListIndentType> {
    match read_string(value, key)?.as_str() {
        "spaces" => Ok(ListIndentType::Spaces),
        "tabs" => Ok(ListIndentType::Tabs),
        other => Err(PhpException::default(format!("Invalid list_indent_type '{other}'"))),
    }
}

/// Parse a highlight style string into HighlightStyle enum.
pub fn parse_highlight_style(value: &Zval, key: &str) -> PhpResult<HighlightStyle> {
    match read_string(value, key)?.as_str() {
        "double_equal" => Ok(HighlightStyle::DoubleEqual),
        "html" => Ok(HighlightStyle::Html),
        "bold" => Ok(HighlightStyle::Bold),
        "none" => Ok(HighlightStyle::None),
        other => Err(PhpException::default(format!("Invalid highlight_style '{other}'"))),
    }
}

/// Parse a whitespace mode string into WhitespaceMode enum.
pub fn parse_whitespace_mode(value: &Zval, key: &str) -> PhpResult<WhitespaceMode> {
    match read_string(value, key)?.as_str() {
        "normalized" => Ok(WhitespaceMode::Normalized),
        "strict" => Ok(WhitespaceMode::Strict),
        other => Err(PhpException::default(format!("Invalid whitespace_mode '{other}'"))),
    }
}

/// Parse a newline style string into NewlineStyle enum.
pub fn parse_newline_style(value: &Zval, key: &str) -> PhpResult<NewlineStyle> {
    match read_string(value, key)?.as_str() {
        "spaces" => Ok(NewlineStyle::Spaces),
        "backslash" => Ok(NewlineStyle::Backslash),
        other => Err(PhpException::default(format!("Invalid newline_style '{other}'"))),
    }
}

/// Parse a code block style string into CodeBlockStyle enum.
pub fn parse_code_block_style(value: &Zval, key: &str) -> PhpResult<CodeBlockStyle> {
    match read_string(value, key)?.as_str() {
        "indented" => Ok(CodeBlockStyle::Indented),
        "backticks" => Ok(CodeBlockStyle::Backticks),
        "tildes" => Ok(CodeBlockStyle::Tildes),
        other => Err(PhpException::default(format!("Invalid code_block_style '{other}'"))),
    }
}

/// Parse a preprocessing preset string into PreprocessingPreset enum.
pub fn parse_preprocessing_preset(value: &Zval, key: &str) -> PhpResult<PreprocessingPreset> {
    match read_string(value, key)?.as_str() {
        "minimal" => Ok(PreprocessingPreset::Minimal),
        "standard" => Ok(PreprocessingPreset::Standard),
        "aggressive" => Ok(PreprocessingPreset::Aggressive),
        other => Err(PhpException::default(format!("Invalid preprocessing preset '{other}'"))),
    }
}

/// Parse an output format string into OutputFormat enum.
pub fn parse_output_format(value: &Zval, key: &str) -> PhpResult<OutputFormat> {
    match read_string(value, key)?.as_str() {
        "djot" => Ok(OutputFormat::Djot),
        "markdown" => Ok(OutputFormat::Markdown),
        other => Err(PhpException::default(format!("Invalid output_format '{other}'"))),
    }
}

// Helper functions (private)

fn read_string(value: &Zval, key: &str) -> PhpResult<String> {
    value
        .string()
        .ok_or_else(|| PhpException::default(format!("'{key}' must be a string (got {:?})", value.get_type())))
}
