#[cfg(feature = "js-bindings")]
use html_to_markdown_rs::ConversionError;
#[cfg(feature = "js-bindings")]
use wasm_bindgen::prelude::*;

#[cfg(feature = "js-bindings")]
pub fn to_js_error(err: ConversionError) -> JsValue {
    JsValue::from_str(&html_to_markdown_bindings_common::error::error_message(&err))
}

#[cfg(feature = "js-bindings")]
pub fn parse_wasm_options(options: JsValue) -> Result<Option<html_to_markdown_rs::ConversionOptions>, JsValue> {
    if options.is_undefined() || options.is_null() {
        return Ok(None);
    }

    if let Some(obj) = options.dyn_ref::<js_sys::Object>() {
        if js_sys::Object::keys(obj).length() == 0 {
            return Ok(None);
        }
    }

    let update: html_to_markdown_rs::ConversionOptionsUpdate = serde_wasm_bindgen::from_value(options)
        .map_err(|e| JsValue::from_str(&format!("Failed to parse options: {}", e)))?;
    Ok(Some(update.into()))
}

#[cfg(feature = "js-bindings")]
pub fn bytes_to_string(bytes: js_sys::Uint8Array) -> Result<String, JsValue> {
    let mut buffer = vec![0u8; bytes.length() as usize];
    bytes.copy_to(&mut buffer);
    String::from_utf8(buffer).map_err(|e| JsValue::from_str(&format!("HTML must be valid UTF-8: {}", e)))
}

/// Convert HTML to Markdown
///
/// # Arguments
///
/// * `html` - The HTML string to convert
/// * `options` - Optional conversion options (as a JavaScript object)
///
/// # Example
///
/// ```javascript
/// import { convert } from 'html-to-markdown-wasm';
///
/// const html = '<h1>Hello World</h1>';
/// const markdown = convert(html);
/// console.log(markdown); // # Hello World
/// ```
#[cfg(feature = "js-bindings")]
#[wasm_bindgen]
pub fn convert(html: String, options: JsValue) -> Result<String, JsValue> {
    let rust_options = parse_wasm_options(options)?;

    #[cfg(feature = "visitor")]
    {
        html_to_markdown_rs::safety::guard_panic(|| html_to_markdown_rs::convert(&html, rust_options))
            .map_err(to_js_error)
    }
    #[cfg(not(feature = "visitor"))]
    {
        html_to_markdown_rs::safety::guard_panic(|| html_to_markdown_rs::convert(&html, rust_options))
            .map_err(to_js_error)
    }
}

#[cfg(feature = "js-bindings")]
#[wasm_bindgen(js_name = convertBytes)]
pub fn convert_bytes(html: js_sys::Uint8Array, options: JsValue) -> Result<String, JsValue> {
    let html = bytes_to_string(html)?;
    let rust_options = parse_wasm_options(options)?;

    #[cfg(feature = "visitor")]
    {
        html_to_markdown_rs::safety::guard_panic(|| html_to_markdown_rs::convert(&html, rust_options))
            .map_err(to_js_error)
    }
    #[cfg(not(feature = "visitor"))]
    {
        html_to_markdown_rs::safety::guard_panic(|| html_to_markdown_rs::convert(&html, rust_options))
            .map_err(to_js_error)
    }
}

#[cfg(feature = "js-bindings")]
#[wasm_bindgen(js_name = createConversionOptionsHandle)]
pub fn create_conversion_options_handle(
    options: JsValue,
) -> Result<crate::options::WasmConversionOptionsHandle, JsValue> {
    crate::options::WasmConversionOptionsHandle::new(options)
}

#[cfg(feature = "js-bindings")]
#[wasm_bindgen(js_name = convertWithOptionsHandle)]
pub fn convert_with_options_handle(
    html: String,
    handle: &crate::options::WasmConversionOptionsHandle,
) -> Result<String, JsValue> {
    #[cfg(feature = "visitor")]
    {
        html_to_markdown_rs::safety::guard_panic(|| html_to_markdown_rs::convert(&html, Some(handle.inner.clone())))
            .map_err(to_js_error)
    }
    #[cfg(not(feature = "visitor"))]
    {
        html_to_markdown_rs::safety::guard_panic(|| html_to_markdown_rs::convert(&html, Some(handle.inner.clone())))
            .map_err(to_js_error)
    }
}

#[cfg(feature = "js-bindings")]
#[wasm_bindgen(js_name = convertBytesWithOptionsHandle)]
pub fn convert_bytes_with_options_handle(
    html: js_sys::Uint8Array,
    handle: &crate::options::WasmConversionOptionsHandle,
) -> Result<String, JsValue> {
    let html = bytes_to_string(html)?;

    #[cfg(feature = "visitor")]
    {
        html_to_markdown_rs::safety::guard_panic(|| html_to_markdown_rs::convert(&html, Some(handle.inner.clone())))
            .map_err(to_js_error)
    }
    #[cfg(not(feature = "visitor"))]
    {
        html_to_markdown_rs::safety::guard_panic(|| html_to_markdown_rs::convert(&html, Some(handle.inner.clone())))
            .map_err(to_js_error)
    }
}

/// Convert HTML to Markdown while collecting inline images
///
/// # Arguments
///
/// * `html` - The HTML string to convert
/// * `options` - Optional conversion options (as a JavaScript object)
/// * `image_config` - Configuration for inline image extraction
///
/// # Example
///
/// ```javascript
/// import { convertWithInlineImages, WasmInlineImageConfig } from 'html-to-markdown-wasm';
///
/// const html = '<img src="data:image/png;base64,..." alt="test">';
/// const config = new WasmInlineImageConfig(1024 * 1024);
/// config.inferDimensions = true;
///
/// const result = convertWithInlineImages(html, null, config);
/// console.log(result.markdown);
/// console.log(result.inlineImages.length);
/// ```
#[cfg(feature = "js-bindings")]
fn convert_with_inline_images_internal(
    html: &str,
    options: JsValue,
    image_config: Option<crate::inline_images::WasmInlineImageConfig>,
) -> Result<crate::inline_images::WasmHtmlExtraction, JsValue> {
    let rust_options = parse_wasm_options(options)?;

    let rust_config = image_config.map(Into::into).unwrap_or_else(|| {
        html_to_markdown_rs::InlineImageConfig::new(html_to_markdown_rs::DEFAULT_INLINE_IMAGE_LIMIT)
    });

    let extraction = {
        #[cfg(feature = "visitor")]
        {
            html_to_markdown_rs::safety::guard_panic(|| {
                html_to_markdown_rs::convert_with_inline_images(html, rust_options, rust_config, None)
            })
        }
        #[cfg(not(feature = "visitor"))]
        {
            html_to_markdown_rs::safety::guard_panic(|| {
                html_to_markdown_rs::convert_with_inline_images(html, rust_options, rust_config, None)
            })
        }
    }
    .map_err(to_js_error)?;

    Ok(extraction.into())
}

#[cfg(feature = "js-bindings")]
#[wasm_bindgen(js_name = convertWithInlineImages)]
pub fn convert_with_inline_images(
    html: String,
    options: JsValue,
    image_config: Option<crate::inline_images::WasmInlineImageConfig>,
) -> Result<crate::inline_images::WasmHtmlExtraction, JsValue> {
    convert_with_inline_images_internal(&html, options, image_config)
}

#[cfg(feature = "js-bindings")]
#[wasm_bindgen(js_name = convertBytesWithInlineImages)]
pub fn convert_bytes_with_inline_images(
    html: js_sys::Uint8Array,
    options: JsValue,
    image_config: Option<crate::inline_images::WasmInlineImageConfig>,
) -> Result<crate::inline_images::WasmHtmlExtraction, JsValue> {
    let html = bytes_to_string(html)?;
    convert_with_inline_images_internal(&html, options, image_config)
}

/// Convert HTML to Markdown with metadata extraction
///
/// # Arguments
///
/// * `html` - The HTML string to convert
/// * `options` - Optional conversion options (as a JavaScript object)
/// * `metadata_config` - Metadata extraction configuration
///
/// # Returns
///
/// JavaScript object with `markdown` (string) and `metadata` (object) fields
///
/// # Example
///
/// ```javascript
/// import { convertWithMetadata, WasmMetadataConfig } from 'html-to-markdown-wasm';
///
/// const html = '<h1>Hello World</h1><a href="https://example.com">Link</a>';
/// const config = new WasmMetadataConfig();
/// config.extractHeaders = true;
/// config.extractLinks = true;
///
/// const result = convertWithMetadata(html, null, config);
/// console.log(result.markdown); // # Hello World\n\n[Link](https://example.com)
/// console.log(result.metadata.headers); // [{ level: 1, text: "Hello World", ... }]
/// console.log(result.metadata.links); // [{ href: "https://example.com", text: "Link", ... }]
/// ```
#[cfg(all(feature = "js-bindings", feature = "metadata"))]
#[wasm_bindgen(js_name = convertWithMetadata)]
pub fn convert_with_metadata(
    html: String,
    options: JsValue,
    metadata_config: Option<crate::options::WasmMetadataConfig>,
) -> Result<JsValue, JsValue> {
    let rust_options = parse_wasm_options(options)?;
    let rust_metadata_config = metadata_config.map(Into::into).unwrap_or_default();

    let (markdown, metadata) = {
        #[cfg(feature = "visitor")]
        {
            html_to_markdown_rs::safety::guard_panic(|| {
                html_to_markdown_rs::convert_with_metadata(&html, rust_options, rust_metadata_config, None)
            })
        }
        #[cfg(not(feature = "visitor"))]
        {
            html_to_markdown_rs::safety::guard_panic(|| {
                html_to_markdown_rs::convert_with_metadata(&html, rust_options, rust_metadata_config, None)
            })
        }
    }
    .map_err(to_js_error)?;

    let metadata_js = serde_wasm_bindgen::to_value(&metadata).map_err(|e| JsValue::from_str(&e.to_string()))?;

    let result = js_sys::Object::new();
    js_sys::Reflect::set(&result, &JsValue::from_str("markdown"), &JsValue::from_str(&markdown))
        .map_err(|_| JsValue::from_str("failed to set markdown property"))?;
    js_sys::Reflect::set(&result, &JsValue::from_str("metadata"), &metadata_js)
        .map_err(|_| JsValue::from_str("failed to set metadata property"))?;

    Ok(result.into())
}

/// Convert HTML bytes to Markdown with metadata extraction
///
/// # Arguments
///
/// * `html` - The HTML bytes to convert
/// * `options` - Optional conversion options (as a JavaScript object)
/// * `metadata_config` - Metadata extraction configuration
///
/// # Returns
///
/// JavaScript object with `markdown` (string) and `metadata` (object) fields
#[cfg(all(feature = "js-bindings", feature = "metadata"))]
#[wasm_bindgen(js_name = convertBytesWithMetadata)]
pub fn convert_bytes_with_metadata(
    html: js_sys::Uint8Array,
    options: JsValue,
    metadata_config: Option<crate::options::WasmMetadataConfig>,
) -> Result<JsValue, JsValue> {
    let html = bytes_to_string(html)?;
    convert_with_metadata(html, options, metadata_config)
}
