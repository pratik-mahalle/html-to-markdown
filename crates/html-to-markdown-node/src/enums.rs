use html_to_markdown_rs::{
    CodeBlockStyle, HeadingStyle, HighlightStyle, LinkStyle, ListIndentType, NewlineStyle, OutputFormat,
    PreprocessingPreset, WhitespaceMode,
};
use napi_derive::napi;

/// Heading style options
#[napi(string_enum)]
pub enum JsHeadingStyle {
    /// Underlined style (=== for h1, --- for h2)
    Underlined,
    /// ATX style (# for h1, ## for h2, etc.)
    Atx,
    /// ATX closed style (# title #)
    AtxClosed,
}

impl From<JsHeadingStyle> for HeadingStyle {
    fn from(val: JsHeadingStyle) -> Self {
        match val {
            JsHeadingStyle::Underlined => Self::Underlined,
            JsHeadingStyle::Atx => Self::Atx,
            JsHeadingStyle::AtxClosed => Self::AtxClosed,
        }
    }
}

/// List indentation type
#[napi(string_enum)]
pub enum JsListIndentType {
    Spaces,
    Tabs,
}

impl From<JsListIndentType> for ListIndentType {
    fn from(val: JsListIndentType) -> Self {
        match val {
            JsListIndentType::Spaces => Self::Spaces,
            JsListIndentType::Tabs => Self::Tabs,
        }
    }
}

/// Whitespace handling mode
#[napi(string_enum)]
pub enum JsWhitespaceMode {
    Normalized,
    Strict,
}

impl From<JsWhitespaceMode> for WhitespaceMode {
    fn from(val: JsWhitespaceMode) -> Self {
        match val {
            JsWhitespaceMode::Normalized => Self::Normalized,
            JsWhitespaceMode::Strict => Self::Strict,
        }
    }
}

/// Newline style
#[napi(string_enum)]
pub enum JsNewlineStyle {
    /// Two spaces at end of line
    Spaces,
    /// Backslash at end of line
    Backslash,
}

impl From<JsNewlineStyle> for NewlineStyle {
    fn from(val: JsNewlineStyle) -> Self {
        match val {
            JsNewlineStyle::Spaces => Self::Spaces,
            JsNewlineStyle::Backslash => Self::Backslash,
        }
    }
}

/// Code block style
#[napi(string_enum)]
pub enum JsCodeBlockStyle {
    /// Indented code blocks (4 spaces) - `CommonMark` default
    Indented,
    /// Fenced code blocks with backticks (```)
    Backticks,
    /// Fenced code blocks with tildes (~~~)
    Tildes,
}

impl From<JsCodeBlockStyle> for CodeBlockStyle {
    fn from(val: JsCodeBlockStyle) -> Self {
        match val {
            JsCodeBlockStyle::Indented => Self::Indented,
            JsCodeBlockStyle::Backticks => Self::Backticks,
            JsCodeBlockStyle::Tildes => Self::Tildes,
        }
    }
}

/// Highlight style for `<mark>` elements
#[napi(string_enum)]
pub enum JsHighlightStyle {
    /// ==text==
    DoubleEqual,
    /// <mark>text</mark>
    Html,
    /// **text**
    Bold,
    /// Plain text (no formatting)
    None,
}

impl From<JsHighlightStyle> for HighlightStyle {
    fn from(val: JsHighlightStyle) -> Self {
        match val {
            JsHighlightStyle::DoubleEqual => Self::DoubleEqual,
            JsHighlightStyle::Html => Self::Html,
            JsHighlightStyle::Bold => Self::Bold,
            JsHighlightStyle::None => Self::None,
        }
    }
}

/// Link rendering style
#[napi(string_enum)]
pub enum JsLinkStyle {
    /// Inline links: [text](url)
    Inline,
    /// Reference-style links: [text][1] with definitions at end
    Reference,
}

impl From<JsLinkStyle> for LinkStyle {
    fn from(val: JsLinkStyle) -> Self {
        match val {
            JsLinkStyle::Inline => Self::Inline,
            JsLinkStyle::Reference => Self::Reference,
        }
    }
}

/// Preprocessing preset levels
#[napi(string_enum)]
pub enum JsPreprocessingPreset {
    Minimal,
    Standard,
    Aggressive,
}

impl From<JsPreprocessingPreset> for PreprocessingPreset {
    fn from(val: JsPreprocessingPreset) -> Self {
        match val {
            JsPreprocessingPreset::Minimal => Self::Minimal,
            JsPreprocessingPreset::Standard => Self::Standard,
            JsPreprocessingPreset::Aggressive => Self::Aggressive,
        }
    }
}

/// Output format for conversion
#[napi(string_enum)]
pub enum JsOutputFormat {
    /// Standard Markdown (CommonMark compatible)
    Markdown,
    /// Djot lightweight markup language
    Djot,
    /// Plain text (no markup)
    Plain,
}

impl From<JsOutputFormat> for OutputFormat {
    fn from(val: JsOutputFormat) -> Self {
        match val {
            JsOutputFormat::Markdown => OutputFormat::Markdown,
            JsOutputFormat::Djot => OutputFormat::Djot,
            JsOutputFormat::Plain => OutputFormat::Plain,
        }
    }
}
