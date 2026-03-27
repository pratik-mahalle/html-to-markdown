#![allow(clippy::cast_precision_loss, clippy::cast_sign_loss, clippy::unused_self)]

//! Main conversion options with builder pattern.

use crate::options::preprocessing::PreprocessingOptions;
use crate::options::validation::{
    CodeBlockStyle, HeadingStyle, HighlightStyle, ListIndentType, NewlineStyle, OutputFormat, WhitespaceMode,
};

/// Main conversion options for HTML to Markdown conversion.
///
/// Use [`ConversionOptions::builder()`] to construct, or [`Default::default()`] for defaults.
///
/// # Example
///
/// ```rust,ignore
/// use html_to_markdown_rs::ConversionOptions;
///
/// let options = ConversionOptions::builder()
///     .heading_style(HeadingStyle::Atx)
///     .wrap(true)
///     .wrap_width(100)
///     .build();
/// ```
#[derive(Debug, Clone)]
#[cfg_attr(
    any(feature = "serde", feature = "metadata"),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(
    any(feature = "serde", feature = "metadata"),
    serde(rename_all = "camelCase", default)
)]
pub struct ConversionOptions {
    pub heading_style: HeadingStyle,
    pub list_indent_type: ListIndentType,
    pub list_indent_width: usize,
    pub bullets: String,
    pub strong_em_symbol: char,
    pub escape_asterisks: bool,
    pub escape_underscores: bool,
    pub escape_misc: bool,
    pub escape_ascii: bool,
    pub code_language: String,
    pub autolinks: bool,
    pub default_title: bool,
    pub br_in_tables: bool,
    pub highlight_style: HighlightStyle,
    pub extract_metadata: bool,
    pub whitespace_mode: WhitespaceMode,
    pub strip_newlines: bool,
    pub wrap: bool,
    pub wrap_width: usize,
    pub convert_as_inline: bool,
    pub sub_symbol: String,
    pub sup_symbol: String,
    pub newline_style: NewlineStyle,
    pub code_block_style: CodeBlockStyle,
    pub keep_inline_images_in: Vec<String>,
    pub preprocessing: PreprocessingOptions,
    pub encoding: String,
    pub debug: bool,
    pub strip_tags: Vec<String>,
    pub preserve_tags: Vec<String>,
    pub skip_images: bool,
    pub output_format: OutputFormat,
    /// Include structured document tree in result.
    pub include_document_structure: bool,
    /// Extract inline images from data URIs and SVGs.
    pub extract_images: bool,
    /// Maximum decoded image size in bytes (default 5MB).
    pub max_image_size: u64,
    /// Capture SVG elements as images.
    pub capture_svg: bool,
    /// Infer image dimensions from data.
    pub infer_dimensions: bool,
}

impl Default for ConversionOptions {
    fn default() -> Self {
        Self {
            heading_style: HeadingStyle::default(),
            list_indent_type: ListIndentType::default(),
            list_indent_width: 2,
            bullets: "-".to_string(),
            strong_em_symbol: '*',
            escape_asterisks: false,
            escape_underscores: false,
            escape_misc: false,
            escape_ascii: false,
            code_language: String::new(),
            autolinks: true,
            default_title: false,
            br_in_tables: false,
            highlight_style: HighlightStyle::default(),
            extract_metadata: true,
            whitespace_mode: WhitespaceMode::default(),
            strip_newlines: false,
            wrap: false,
            wrap_width: 80,
            convert_as_inline: false,
            sub_symbol: String::new(),
            sup_symbol: String::new(),
            newline_style: NewlineStyle::Spaces,
            code_block_style: CodeBlockStyle::default(),
            keep_inline_images_in: Vec::new(),
            preprocessing: PreprocessingOptions::default(),
            encoding: "utf-8".to_string(),
            debug: false,
            strip_tags: Vec::new(),
            preserve_tags: Vec::new(),
            skip_images: false,
            output_format: OutputFormat::default(),
            include_document_structure: false,
            extract_images: false,
            max_image_size: 5_242_880,
            capture_svg: false,
            infer_dimensions: true,
        }
    }
}

// ── Public getters ──────────────────────────────────────────────────────────

impl ConversionOptions {
    /// Create a new builder with default values.
    #[must_use]
    pub fn builder() -> ConversionOptionsBuilder {
        ConversionOptionsBuilder(Self::default())
    }

    pub fn heading_style(&self) -> HeadingStyle {
        self.heading_style
    }
    pub fn list_indent_type(&self) -> ListIndentType {
        self.list_indent_type
    }
    pub fn list_indent_width(&self) -> usize {
        self.list_indent_width
    }
    pub fn bullets(&self) -> &str {
        &self.bullets
    }
    pub fn strong_em_symbol(&self) -> char {
        self.strong_em_symbol
    }
    pub fn escape_asterisks(&self) -> bool {
        self.escape_asterisks
    }
    pub fn escape_underscores(&self) -> bool {
        self.escape_underscores
    }
    pub fn escape_misc(&self) -> bool {
        self.escape_misc
    }
    pub fn escape_ascii(&self) -> bool {
        self.escape_ascii
    }
    pub fn code_language(&self) -> &str {
        &self.code_language
    }
    pub fn autolinks(&self) -> bool {
        self.autolinks
    }
    pub fn default_title(&self) -> bool {
        self.default_title
    }
    pub fn br_in_tables(&self) -> bool {
        self.br_in_tables
    }
    pub fn highlight_style(&self) -> HighlightStyle {
        self.highlight_style
    }
    pub fn extract_metadata(&self) -> bool {
        self.extract_metadata
    }
    pub fn whitespace_mode(&self) -> WhitespaceMode {
        self.whitespace_mode
    }
    pub fn strip_newlines(&self) -> bool {
        self.strip_newlines
    }
    pub fn wrap(&self) -> bool {
        self.wrap
    }
    pub fn wrap_width(&self) -> usize {
        self.wrap_width
    }
    pub fn convert_as_inline(&self) -> bool {
        self.convert_as_inline
    }
    pub fn sub_symbol(&self) -> &str {
        &self.sub_symbol
    }
    pub fn sup_symbol(&self) -> &str {
        &self.sup_symbol
    }
    pub fn newline_style(&self) -> NewlineStyle {
        self.newline_style
    }
    pub fn code_block_style(&self) -> CodeBlockStyle {
        self.code_block_style
    }
    pub fn keep_inline_images_in(&self) -> &[String] {
        &self.keep_inline_images_in
    }
    pub fn preprocessing(&self) -> &PreprocessingOptions {
        &self.preprocessing
    }
    pub fn encoding(&self) -> &str {
        &self.encoding
    }
    pub fn debug(&self) -> bool {
        self.debug
    }
    pub fn strip_tags(&self) -> &[String] {
        &self.strip_tags
    }
    pub fn preserve_tags(&self) -> &[String] {
        &self.preserve_tags
    }
    pub fn skip_images(&self) -> bool {
        self.skip_images
    }
    pub fn output_format(&self) -> OutputFormat {
        self.output_format
    }
    pub fn include_document_structure(&self) -> bool {
        self.include_document_structure
    }
    pub fn extract_images(&self) -> bool {
        self.extract_images
    }
    pub fn max_image_size(&self) -> u64 {
        self.max_image_size
    }
    pub fn capture_svg(&self) -> bool {
        self.capture_svg
    }
    pub fn infer_dimensions(&self) -> bool {
        self.infer_dimensions
    }
}

// ── Builder ─────────────────────────────────────────────────────────────────

/// Builder for [`ConversionOptions`].
///
/// All fields start with default values. Call `.build()` to produce the final options.
#[derive(Debug, Clone)]
pub struct ConversionOptionsBuilder(ConversionOptions);

macro_rules! builder_setter {
    ($name:ident, $ty:ty) => {
        #[must_use]
        pub fn $name(mut self, value: $ty) -> Self {
            self.0.$name = value;
            self
        }
    };
}

macro_rules! builder_setter_into {
    ($name:ident, $ty:ty) => {
        #[must_use]
        pub fn $name(mut self, value: impl Into<$ty>) -> Self {
            self.0.$name = value.into();
            self
        }
    };
}

impl ConversionOptionsBuilder {
    // Output control
    builder_setter!(output_format, OutputFormat);
    builder_setter!(include_document_structure, bool);
    builder_setter!(extract_metadata, bool);
    builder_setter!(extract_images, bool);

    // Markdown formatting
    builder_setter!(heading_style, HeadingStyle);
    builder_setter!(list_indent_type, ListIndentType);
    builder_setter!(list_indent_width, usize);
    builder_setter_into!(bullets, String);
    builder_setter!(strong_em_symbol, char);
    builder_setter!(code_block_style, CodeBlockStyle);
    builder_setter!(newline_style, NewlineStyle);
    builder_setter!(highlight_style, HighlightStyle);
    builder_setter_into!(code_language, String);
    builder_setter!(autolinks, bool);
    builder_setter!(default_title, bool);
    builder_setter!(br_in_tables, bool);
    builder_setter_into!(sub_symbol, String);
    builder_setter_into!(sup_symbol, String);

    // Escaping
    builder_setter!(escape_asterisks, bool);
    builder_setter!(escape_underscores, bool);
    builder_setter!(escape_misc, bool);
    builder_setter!(escape_ascii, bool);

    // Whitespace / wrapping
    builder_setter!(whitespace_mode, WhitespaceMode);
    builder_setter!(strip_newlines, bool);
    builder_setter!(wrap, bool);
    builder_setter!(wrap_width, usize);

    // Element handling
    builder_setter!(convert_as_inline, bool);
    builder_setter!(skip_images, bool);

    #[must_use]
    pub fn strip_tags(mut self, tags: Vec<String>) -> Self {
        self.0.strip_tags = tags;
        self
    }

    #[must_use]
    pub fn preserve_tags(mut self, tags: Vec<String>) -> Self {
        self.0.preserve_tags = tags;
        self
    }

    #[must_use]
    pub fn keep_inline_images_in(mut self, tags: Vec<String>) -> Self {
        self.0.keep_inline_images_in = tags;
        self
    }

    // Image extraction config
    builder_setter!(max_image_size, u64);
    builder_setter!(capture_svg, bool);
    builder_setter!(infer_dimensions, bool);

    // Preprocessing
    #[must_use]
    pub fn preprocessing(mut self, preprocessing: PreprocessingOptions) -> Self {
        self.0.preprocessing = preprocessing;
        self
    }

    // Encoding
    builder_setter_into!(encoding, String);

    // Debug
    builder_setter!(debug, bool);

    /// Build the final [`ConversionOptions`].
    #[must_use]
    pub fn build(self) -> ConversionOptions {
        self.0
    }
}

// ── ConversionOptionsUpdate (for binding crate compatibility) ────────────

use crate::options::preprocessing::PreprocessingOptionsUpdate;

/// Partial update for `ConversionOptions`.
///
/// Uses `Option<T>` fields for selective updates. Bindings use this to construct
/// options from language-native types. Prefer [`ConversionOptionsBuilder`] for Rust code.
#[derive(Debug, Clone, Default)]
#[cfg_attr(
    any(feature = "serde", feature = "metadata"),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", feature = "metadata"), serde(rename_all = "camelCase"))]
pub struct ConversionOptionsUpdate {
    pub heading_style: Option<HeadingStyle>,
    pub list_indent_type: Option<ListIndentType>,
    pub list_indent_width: Option<usize>,
    pub bullets: Option<String>,
    pub strong_em_symbol: Option<char>,
    pub escape_asterisks: Option<bool>,
    pub escape_underscores: Option<bool>,
    pub escape_misc: Option<bool>,
    pub escape_ascii: Option<bool>,
    pub code_language: Option<String>,
    pub autolinks: Option<bool>,
    pub default_title: Option<bool>,
    pub br_in_tables: Option<bool>,
    pub highlight_style: Option<HighlightStyle>,
    pub extract_metadata: Option<bool>,
    pub whitespace_mode: Option<WhitespaceMode>,
    pub strip_newlines: Option<bool>,
    pub wrap: Option<bool>,
    pub wrap_width: Option<usize>,
    pub convert_as_inline: Option<bool>,
    pub sub_symbol: Option<String>,
    pub sup_symbol: Option<String>,
    pub newline_style: Option<NewlineStyle>,
    pub code_block_style: Option<CodeBlockStyle>,
    pub keep_inline_images_in: Option<Vec<String>>,
    pub preprocessing: Option<PreprocessingOptionsUpdate>,
    pub encoding: Option<String>,
    pub debug: Option<bool>,
    pub strip_tags: Option<Vec<String>>,
    pub preserve_tags: Option<Vec<String>>,
    pub skip_images: Option<bool>,
    pub output_format: Option<OutputFormat>,
    pub include_document_structure: Option<bool>,
    pub extract_images: Option<bool>,
    pub max_image_size: Option<u64>,
    pub capture_svg: Option<bool>,
    pub infer_dimensions: Option<bool>,
}

impl ConversionOptions {
    /// Apply a partial update to these conversion options.
    pub fn apply_update(&mut self, update: ConversionOptionsUpdate) {
        macro_rules! apply {
            ($field:ident) => {
                if let Some(v) = update.$field {
                    self.$field = v;
                }
            };
        }
        apply!(heading_style);
        apply!(list_indent_type);
        apply!(list_indent_width);
        apply!(bullets);
        apply!(strong_em_symbol);
        apply!(escape_asterisks);
        apply!(escape_underscores);
        apply!(escape_misc);
        apply!(escape_ascii);
        apply!(code_language);
        apply!(autolinks);
        apply!(default_title);
        apply!(br_in_tables);
        apply!(highlight_style);
        apply!(extract_metadata);
        apply!(whitespace_mode);
        apply!(strip_newlines);
        apply!(wrap);
        apply!(wrap_width);
        apply!(convert_as_inline);
        apply!(sub_symbol);
        apply!(sup_symbol);
        apply!(newline_style);
        apply!(code_block_style);
        apply!(keep_inline_images_in);
        apply!(encoding);
        apply!(debug);
        apply!(strip_tags);
        apply!(preserve_tags);
        apply!(skip_images);
        apply!(output_format);
        apply!(include_document_structure);
        apply!(extract_images);
        apply!(max_image_size);
        apply!(capture_svg);
        apply!(infer_dimensions);
        if let Some(preprocessing) = update.preprocessing {
            self.preprocessing.apply_update(preprocessing);
        }
    }

    /// Create from a partial update, applying to defaults.
    #[must_use]
    pub fn from_update(update: ConversionOptionsUpdate) -> Self {
        let mut options = Self::default();
        options.apply_update(update);
        options
    }
}

impl From<ConversionOptionsUpdate> for ConversionOptions {
    fn from(update: ConversionOptionsUpdate) -> Self {
        Self::from_update(update)
    }
}

// ── Tests ───────────────────────────────────────────────────────────────────

#[cfg(all(test, any(feature = "serde", feature = "metadata")))]
mod tests {
    use super::*;

    #[test]
    fn test_conversion_options_serde() {
        let options = ConversionOptions::builder()
            .heading_style(HeadingStyle::AtxClosed)
            .list_indent_width(4)
            .bullets("*")
            .escape_asterisks(true)
            .whitespace_mode(WhitespaceMode::Strict)
            .build();

        let json = serde_json::to_string(&options).expect("Failed to serialize");
        let deserialized: ConversionOptions = serde_json::from_str(&json).expect("Failed to deserialize");

        assert_eq!(deserialized.list_indent_width, 4);
        assert_eq!(deserialized.bullets, "*");
        assert!(deserialized.escape_asterisks);
        assert_eq!(deserialized.heading_style, HeadingStyle::AtxClosed);
        assert_eq!(deserialized.whitespace_mode, WhitespaceMode::Strict);
    }

    #[test]
    fn test_conversion_options_partial_deserialization() {
        let partial_json = r#"{
            "headingStyle": "atxClosed",
            "listIndentWidth": 4,
            "bullets": "*"
        }"#;

        let deserialized: ConversionOptions =
            serde_json::from_str(partial_json).expect("Failed to deserialize partial JSON");

        assert_eq!(deserialized.heading_style, HeadingStyle::AtxClosed);
        assert_eq!(deserialized.list_indent_width, 4);
        assert_eq!(deserialized.bullets, "*");
        assert!(!deserialized.escape_asterisks);
        assert!(!deserialized.escape_underscores);
        assert_eq!(deserialized.list_indent_type, ListIndentType::Spaces);
    }

    #[test]
    fn test_builder_pattern() {
        let options = ConversionOptions::builder()
            .heading_style(HeadingStyle::Underlined)
            .wrap(true)
            .wrap_width(100)
            .include_document_structure(true)
            .extract_images(true)
            .build();

        assert_eq!(options.heading_style(), HeadingStyle::Underlined);
        assert!(options.wrap());
        assert_eq!(options.wrap_width(), 100);
        assert!(options.include_document_structure());
        assert!(options.extract_images());
    }
}
