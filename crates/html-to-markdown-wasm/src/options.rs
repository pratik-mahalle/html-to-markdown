use crate::enums::{
    WasmCodeBlockStyle, WasmHeadingStyle, WasmHighlightStyle, WasmListIndentType, WasmNewlineStyle, WasmOutputFormat,
    WasmPreprocessingPreset, WasmWhitespaceMode,
};
use html_to_markdown_rs::{ConversionOptionsUpdate, PreprocessingOptionsUpdate};
use serde::{Deserialize, Serialize};

#[cfg(feature = "js-bindings")]
use wasm_bindgen::prelude::*;

/// HTML preprocessing options
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WasmPreprocessingOptions {
    /// Enable preprocessing
    #[serde(default)]
    pub enabled: bool,
    /// Preprocessing preset
    #[serde(default)]
    pub preset: Option<WasmPreprocessingPreset>,
    /// Remove navigation elements
    #[serde(default = "default_true")]
    pub remove_navigation: bool,
    /// Remove form elements
    #[serde(default = "default_true")]
    pub remove_forms: bool,
}

fn default_true() -> bool {
    true
}

impl From<WasmPreprocessingOptions> for PreprocessingOptionsUpdate {
    fn from(val: WasmPreprocessingOptions) -> Self {
        Self {
            enabled: Some(val.enabled),
            preset: val.preset.map(Into::into),
            remove_navigation: Some(val.remove_navigation),
            remove_forms: Some(val.remove_forms),
        }
    }
}

/// Main conversion options
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WasmConversionOptions {
    /// Heading style
    pub heading_style: Option<WasmHeadingStyle>,
    /// List indentation type
    pub list_indent_type: Option<WasmListIndentType>,
    /// List indentation width (spaces)
    pub list_indent_width: Option<usize>,
    /// Bullet characters for unordered lists
    pub bullets: Option<String>,
    /// Symbol for strong/emphasis (* or _)
    pub strong_em_symbol: Option<char>,
    /// Escape asterisks in text
    pub escape_asterisks: Option<bool>,
    /// Escape underscores in text
    pub escape_underscores: Option<bool>,
    /// Escape misc markdown characters
    pub escape_misc: Option<bool>,
    /// Escape all ASCII punctuation
    pub escape_ascii: Option<bool>,
    /// Default code language
    pub code_language: Option<String>,
    /// Use autolinks for bare URLs
    pub autolinks: Option<bool>,
    /// Add default title if none exists
    pub default_title: Option<bool>,
    /// Use <br> in tables instead of spaces
    pub br_in_tables: Option<bool>,
    /// Enable spatial table reconstruction in hOCR documents.
    ///
    /// Deprecated since 2.30.0: hOCR support will be removed in v3.
    pub hocr_spatial_tables: Option<bool>,
    /// Highlight style for `<mark>` elements
    pub highlight_style: Option<WasmHighlightStyle>,
    /// Extract metadata from HTML
    pub extract_metadata: Option<bool>,
    /// Whitespace handling mode
    pub whitespace_mode: Option<WasmWhitespaceMode>,
    /// Strip newlines from HTML before processing
    pub strip_newlines: Option<bool>,
    /// Enable text wrapping
    pub wrap: Option<bool>,
    /// Text wrap width
    pub wrap_width: Option<usize>,
    /// Treat block elements as inline
    pub convert_as_inline: Option<bool>,
    /// Subscript symbol
    pub sub_symbol: Option<String>,
    /// Superscript symbol
    pub sup_symbol: Option<String>,
    /// Newline style
    pub newline_style: Option<WasmNewlineStyle>,
    /// Code block style
    pub code_block_style: Option<WasmCodeBlockStyle>,
    /// Elements where images should remain as markdown
    pub keep_inline_images_in: Option<Vec<String>>,
    /// Skip images during conversion
    pub skip_images: Option<bool>,
    /// Preprocessing options
    pub preprocessing: Option<WasmPreprocessingOptions>,
    /// Source encoding (informational)
    pub encoding: Option<String>,
    /// Enable debug mode with diagnostic warnings
    pub debug: Option<bool>,
    /// List of HTML tags to strip
    pub strip_tags: Option<Vec<String>>,
    /// List of HTML tags to preserve as-is in the output
    pub preserve_tags: Option<Vec<String>>,
    /// Output format for conversion
    pub output_format: Option<WasmOutputFormat>,
}

impl From<WasmConversionOptions> for ConversionOptionsUpdate {
    fn from(val: WasmConversionOptions) -> Self {
        Self {
            heading_style: val.heading_style.map(Into::into),
            list_indent_type: val.list_indent_type.map(Into::into),
            list_indent_width: val.list_indent_width,
            bullets: val.bullets,
            strong_em_symbol: val.strong_em_symbol,
            escape_asterisks: val.escape_asterisks,
            escape_underscores: val.escape_underscores,
            escape_misc: val.escape_misc,
            escape_ascii: val.escape_ascii,
            code_language: val.code_language,
            autolinks: val.autolinks,
            default_title: val.default_title,
            br_in_tables: val.br_in_tables,
            hocr_spatial_tables: val.hocr_spatial_tables,
            highlight_style: val.highlight_style.map(Into::into),
            extract_metadata: val.extract_metadata,
            whitespace_mode: val.whitespace_mode.map(Into::into),
            strip_newlines: val.strip_newlines,
            wrap: val.wrap,
            wrap_width: val.wrap_width,
            convert_as_inline: val.convert_as_inline,
            sub_symbol: val.sub_symbol,
            sup_symbol: val.sup_symbol,
            newline_style: val.newline_style.map(Into::into),
            code_block_style: val.code_block_style.map(Into::into),
            keep_inline_images_in: val.keep_inline_images_in,
            skip_images: val.skip_images,
            preprocessing: val.preprocessing.map(Into::into),
            encoding: val.encoding,
            debug: val.debug,
            strip_tags: val.strip_tags,
            preserve_tags: val.preserve_tags,
            output_format: val.output_format.map(Into::into),
        }
    }
}

impl From<WasmConversionOptions> for html_to_markdown_rs::ConversionOptions {
    fn from(val: WasmConversionOptions) -> Self {
        html_to_markdown_rs::ConversionOptions::from(ConversionOptionsUpdate::from(val))
    }
}

#[cfg(feature = "js-bindings")]
#[wasm_bindgen]
pub struct WasmConversionOptionsHandle {
    pub(crate) inner: html_to_markdown_rs::ConversionOptions,
}

#[cfg(feature = "js-bindings")]
#[wasm_bindgen]
impl WasmConversionOptionsHandle {
    #[wasm_bindgen(constructor)]
    pub fn new(options: JsValue) -> Result<WasmConversionOptionsHandle, JsValue> {
        let inner = crate::convert::parse_wasm_options(options)?
            .unwrap_or_else(html_to_markdown_rs::ConversionOptions::default);
        Ok(Self { inner })
    }
}

#[cfg(all(feature = "js-bindings", feature = "metadata"))]
#[wasm_bindgen]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WasmMetadataConfig {
    pub(crate) extract_document: bool,
    pub(crate) extract_headers: bool,
    pub(crate) extract_links: bool,
    pub(crate) extract_images: bool,
    pub(crate) extract_structured_data: bool,
    pub(crate) max_structured_data_size: usize,
}

#[cfg(all(feature = "js-bindings", feature = "metadata"))]
#[wasm_bindgen]
impl WasmMetadataConfig {
    /// Create a new metadata configuration with defaults
    ///
    /// All extraction types enabled by default with 1MB structured data limit
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {
            extract_document: true,
            extract_headers: true,
            extract_links: true,
            extract_images: true,
            extract_structured_data: true,
            max_structured_data_size: html_to_markdown_rs::DEFAULT_MAX_STRUCTURED_DATA_SIZE,
        }
    }

    #[wasm_bindgen(getter)]
    pub fn extract_document(&self) -> bool {
        self.extract_document
    }

    #[wasm_bindgen(setter)]
    pub fn set_extract_document(&mut self, value: bool) {
        self.extract_document = value;
    }

    #[wasm_bindgen(getter)]
    pub fn extract_headers(&self) -> bool {
        self.extract_headers
    }

    #[wasm_bindgen(setter)]
    pub fn set_extract_headers(&mut self, value: bool) {
        self.extract_headers = value;
    }

    #[wasm_bindgen(getter)]
    pub fn extract_links(&self) -> bool {
        self.extract_links
    }

    #[wasm_bindgen(setter)]
    pub fn set_extract_links(&mut self, value: bool) {
        self.extract_links = value;
    }

    #[wasm_bindgen(getter)]
    pub fn extract_images(&self) -> bool {
        self.extract_images
    }

    #[wasm_bindgen(setter)]
    pub fn set_extract_images(&mut self, value: bool) {
        self.extract_images = value;
    }

    #[wasm_bindgen(getter)]
    pub fn extract_structured_data(&self) -> bool {
        self.extract_structured_data
    }

    #[wasm_bindgen(setter)]
    pub fn set_extract_structured_data(&mut self, value: bool) {
        self.extract_structured_data = value;
    }

    #[wasm_bindgen(getter)]
    pub fn max_structured_data_size(&self) -> usize {
        self.max_structured_data_size
    }

    #[wasm_bindgen(setter)]
    pub fn set_max_structured_data_size(&mut self, value: usize) {
        self.max_structured_data_size = value;
    }
}

#[cfg(all(feature = "js-bindings", feature = "metadata"))]
impl Default for WasmMetadataConfig {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(all(feature = "js-bindings", feature = "metadata"))]
impl From<WasmMetadataConfig> for html_to_markdown_rs::MetadataConfig {
    fn from(cfg: WasmMetadataConfig) -> Self {
        let update = html_to_markdown_rs::MetadataConfigUpdate {
            extract_document: Some(cfg.extract_document),
            extract_headers: Some(cfg.extract_headers),
            extract_links: Some(cfg.extract_links),
            extract_images: Some(cfg.extract_images),
            extract_structured_data: Some(cfg.extract_structured_data),
            max_structured_data_size: Some(cfg.max_structured_data_size),
        };
        html_to_markdown_rs::MetadataConfig::from(update)
    }
}
