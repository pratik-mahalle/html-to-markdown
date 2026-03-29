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
