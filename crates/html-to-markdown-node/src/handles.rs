use crate::options::JsConversionOptions;
use crate::types::{JsConversionResult, build_conversion_result};
use html_to_markdown_bindings_common::error::error_message;
use html_to_markdown_rs::ConversionError;
use html_to_markdown_rs::safety::guard_panic;
use napi::bindgen_prelude::*;
use napi_derive::napi;

fn to_js_error(err: ConversionError) -> Error {
    Error::new(Status::GenericFailure, error_message(&err))
}

/// Convert HTML to Markdown, returning structured content, metadata, tables, and warnings.
///
/// This is the primary API entry point. Returns a `JsConversionResult` object with
/// `content`, `document`, `metadata`, `tables`, and `warnings` fields.
///
/// # Example
///
/// ```javascript
/// const { convert } = require('html-to-markdown');
///
/// const html = '<h1>Hello</h1><p>World</p>';
/// const result = convert(html);
/// console.log(result.content);   // '# Hello\n\nWorld'
/// console.log(result.tables);    // []
/// console.log(result.warnings);  // []
/// ```
#[napi]
pub fn convert(html: String, options: Option<JsConversionOptions>) -> Result<JsConversionResult> {
    let rust_options = options.map(Into::into);
    let result = guard_panic(|| html_to_markdown_rs::convert(&html, rust_options.clone())).map_err(to_js_error)?;
    build_conversion_result(result)
}
