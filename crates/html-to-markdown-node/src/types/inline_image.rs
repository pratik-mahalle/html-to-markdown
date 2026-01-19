use napi::bindgen_prelude::*;
use napi_derive::napi;
use std::collections::HashMap;

/// Inline image data
#[napi(object)]
pub struct JsInlineImage {
    /// Raw image data
    pub data: Buffer,
    /// Image format (png, jpeg, gif, etc.)
    pub format: String,
    /// Generated or provided filename
    pub filename: Option<String>,
    /// Alt text / description
    pub description: Option<String>,
    /// Image dimensions (width, height) if available
    pub dimensions: Option<Vec<u32>>,
    /// Source type (`img_data_uri` or `svg_element`)
    pub source: String,
    /// HTML attributes from the source element
    pub attributes: HashMap<String, String>,
}

/// Warning about inline image processing
#[napi(object)]
pub struct JsInlineImageWarning {
    /// Index of the image that caused the warning
    pub index: u32,
    /// Warning message
    pub message: String,
}

/// Result of HTML extraction with inline images
#[napi(object)]
pub struct JsHtmlExtraction {
    /// Converted markdown
    pub markdown: String,
    /// Extracted inline images
    pub inline_images: Vec<JsInlineImage>,
    /// Warnings encountered during extraction
    pub warnings: Vec<JsInlineImageWarning>,
}
