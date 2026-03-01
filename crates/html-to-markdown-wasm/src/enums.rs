use html_to_markdown_rs::{
    CodeBlockStyle, HeadingStyle, HighlightStyle, ListIndentType, NewlineStyle, OutputFormat, WhitespaceMode,
};
use serde::{Deserialize, Serialize};

/// Heading style options
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum WasmHeadingStyle {
    /// Underlined style (=== for h1, --- for h2)
    Underlined,
    /// ATX style (# for h1, ## for h2, etc.)
    Atx,
    /// ATX closed style (# title #)
    AtxClosed,
}

impl From<WasmHeadingStyle> for HeadingStyle {
    fn from(val: WasmHeadingStyle) -> Self {
        match val {
            WasmHeadingStyle::Underlined => HeadingStyle::Underlined,
            WasmHeadingStyle::Atx => HeadingStyle::Atx,
            WasmHeadingStyle::AtxClosed => HeadingStyle::AtxClosed,
        }
    }
}

/// List indentation type
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum WasmListIndentType {
    Spaces,
    Tabs,
}

impl From<WasmListIndentType> for ListIndentType {
    fn from(val: WasmListIndentType) -> Self {
        match val {
            WasmListIndentType::Spaces => ListIndentType::Spaces,
            WasmListIndentType::Tabs => ListIndentType::Tabs,
        }
    }
}

/// Whitespace handling mode
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum WasmWhitespaceMode {
    Normalized,
    Strict,
}

impl From<WasmWhitespaceMode> for WhitespaceMode {
    fn from(val: WasmWhitespaceMode) -> Self {
        match val {
            WasmWhitespaceMode::Normalized => WhitespaceMode::Normalized,
            WasmWhitespaceMode::Strict => WhitespaceMode::Strict,
        }
    }
}

/// Newline style
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum WasmNewlineStyle {
    /// Two spaces at end of line
    Spaces,
    /// Backslash at end of line
    Backslash,
}

impl From<WasmNewlineStyle> for NewlineStyle {
    fn from(val: WasmNewlineStyle) -> Self {
        match val {
            WasmNewlineStyle::Spaces => NewlineStyle::Spaces,
            WasmNewlineStyle::Backslash => NewlineStyle::Backslash,
        }
    }
}

/// Code block style
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum WasmCodeBlockStyle {
    /// Indented code blocks (4 spaces) - CommonMark default
    Indented,
    /// Fenced code blocks with backticks (```)
    Backticks,
    /// Fenced code blocks with tildes (~~~)
    Tildes,
}

impl From<WasmCodeBlockStyle> for CodeBlockStyle {
    fn from(val: WasmCodeBlockStyle) -> Self {
        match val {
            WasmCodeBlockStyle::Indented => CodeBlockStyle::Indented,
            WasmCodeBlockStyle::Backticks => CodeBlockStyle::Backticks,
            WasmCodeBlockStyle::Tildes => CodeBlockStyle::Tildes,
        }
    }
}

/// Highlight style for `<mark>` elements
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum WasmHighlightStyle {
    /// ==text==
    DoubleEqual,
    /// <mark>text</mark>
    Html,
    /// **text**
    Bold,
    /// Plain text (no formatting)
    None,
}

impl From<WasmHighlightStyle> for HighlightStyle {
    fn from(val: WasmHighlightStyle) -> Self {
        match val {
            WasmHighlightStyle::DoubleEqual => HighlightStyle::DoubleEqual,
            WasmHighlightStyle::Html => HighlightStyle::Html,
            WasmHighlightStyle::Bold => HighlightStyle::Bold,
            WasmHighlightStyle::None => HighlightStyle::None,
        }
    }
}

/// Preprocessing preset levels
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum WasmPreprocessingPreset {
    Minimal,
    Standard,
    Aggressive,
}

impl From<WasmPreprocessingPreset> for html_to_markdown_rs::PreprocessingPreset {
    fn from(val: WasmPreprocessingPreset) -> Self {
        match val {
            WasmPreprocessingPreset::Minimal => html_to_markdown_rs::PreprocessingPreset::Minimal,
            WasmPreprocessingPreset::Standard => html_to_markdown_rs::PreprocessingPreset::Standard,
            WasmPreprocessingPreset::Aggressive => html_to_markdown_rs::PreprocessingPreset::Aggressive,
        }
    }
}

/// Output format for conversion
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum WasmOutputFormat {
    /// Standard Markdown (CommonMark compatible)
    Markdown,
    /// Djot lightweight markup language
    Djot,
    /// Plain text (no markup)
    Plain,
}

impl From<WasmOutputFormat> for OutputFormat {
    fn from(val: WasmOutputFormat) -> Self {
        match val {
            WasmOutputFormat::Markdown => OutputFormat::Markdown,
            WasmOutputFormat::Djot => OutputFormat::Djot,
            WasmOutputFormat::Plain => OutputFormat::Plain,
        }
    }
}
