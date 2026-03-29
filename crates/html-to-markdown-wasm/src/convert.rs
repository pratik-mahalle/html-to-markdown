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

/// Convert HTML to Markdown, returning a JavaScript object with structured content, metadata,
/// images, and warnings in a single pass.
///
/// This is the primary API entry point. Returns a JavaScript object with:
/// - `content`: converted text (string or null)
/// - `document`: structured document tree (object or null)
/// - `metadata`: extracted HTML metadata (object or null)
/// - `tables`: array of extracted table data
/// - `warnings`: array of non-fatal processing warnings
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
/// const html = '<h1>Hello World</h1><p>Some text.</p>';
/// const result = convert(html, null);
/// console.log(result.content);   // '# Hello World\n\nSome text.'
/// console.log(result.tables);    // []
/// console.log(result.warnings);  // []
/// ```
#[cfg(feature = "js-bindings")]
#[wasm_bindgen]
pub fn convert(html: String, options: JsValue) -> Result<JsValue, JsValue> {
    let rust_options = parse_wasm_options(options)?;

    let result = html_to_markdown_rs::safety::guard_panic(|| html_to_markdown_rs::convert(&html, rust_options))
        .map_err(to_js_error)?;

    let js_result = js_sys::Object::new();

    // content
    let content_val = match result.content {
        Some(ref s) => JsValue::from_str(s),
        None => JsValue::NULL,
    };
    js_sys::Reflect::set(&js_result, &JsValue::from_str("content"), &content_val)
        .map_err(|_| JsValue::from_str("failed to set content"))?;

    // document
    let document_val = match result.document {
        Some(ref doc) => serde_wasm_bindgen::to_value(doc).map_err(|e| JsValue::from_str(&e.to_string()))?,
        None => JsValue::NULL,
    };
    js_sys::Reflect::set(&js_result, &JsValue::from_str("document"), &document_val)
        .map_err(|_| JsValue::from_str("failed to set document"))?;

    // metadata
    #[cfg(feature = "metadata")]
    {
        let metadata_val =
            serde_wasm_bindgen::to_value(&result.metadata).map_err(|e| JsValue::from_str(&e.to_string()))?;
        js_sys::Reflect::set(&js_result, &JsValue::from_str("metadata"), &metadata_val)
            .map_err(|_| JsValue::from_str("failed to set metadata"))?;
    }
    #[cfg(not(feature = "metadata"))]
    {
        js_sys::Reflect::set(&js_result, &JsValue::from_str("metadata"), &JsValue::NULL)
            .map_err(|_| JsValue::from_str("failed to set metadata"))?;
    }

    // tables
    let tables_arr = js_sys::Array::new();
    for table in result.tables {
        let t = serde_wasm_bindgen::to_value(&table).map_err(|e| JsValue::from_str(&e.to_string()))?;
        tables_arr.push(&t);
    }
    js_sys::Reflect::set(&js_result, &JsValue::from_str("tables"), &tables_arr)
        .map_err(|_| JsValue::from_str("failed to set tables"))?;

    // images
    let images_arr = js_sys::Array::new();
    for image in result.images {
        let img_obj = js_sys::Object::new();

        // data as Uint8Array
        let data = js_sys::Uint8Array::from(image.data.as_slice());
        js_sys::Reflect::set(&img_obj, &JsValue::from_str("data"), &data)
            .map_err(|_| JsValue::from_str("failed to set image data"))?;

        js_sys::Reflect::set(
            &img_obj,
            &JsValue::from_str("format"),
            &JsValue::from_str(&image.format.to_string()),
        )
        .map_err(|_| JsValue::from_str("failed to set image format"))?;

        let filename_val = match image.filename {
            Some(ref s) => JsValue::from_str(s),
            None => JsValue::NULL,
        };
        js_sys::Reflect::set(&img_obj, &JsValue::from_str("filename"), &filename_val)
            .map_err(|_| JsValue::from_str("failed to set image filename"))?;

        let desc_val = match image.description {
            Some(ref s) => JsValue::from_str(s),
            None => JsValue::NULL,
        };
        js_sys::Reflect::set(&img_obj, &JsValue::from_str("description"), &desc_val)
            .map_err(|_| JsValue::from_str("failed to set image description"))?;

        let (width_val, height_val) = match image.dimensions {
            Some((w, h)) => (JsValue::from_f64(f64::from(w)), JsValue::from_f64(f64::from(h))),
            None => (JsValue::NULL, JsValue::NULL),
        };
        js_sys::Reflect::set(&img_obj, &JsValue::from_str("width"), &width_val)
            .map_err(|_| JsValue::from_str("failed to set image width"))?;
        js_sys::Reflect::set(&img_obj, &JsValue::from_str("height"), &height_val)
            .map_err(|_| JsValue::from_str("failed to set image height"))?;

        js_sys::Reflect::set(
            &img_obj,
            &JsValue::from_str("source"),
            &JsValue::from_str(&image.source.to_string()),
        )
        .map_err(|_| JsValue::from_str("failed to set image source"))?;

        let attrs_obj = js_sys::Object::new();
        for (key, value) in &image.attributes {
            js_sys::Reflect::set(&attrs_obj, &JsValue::from_str(key), &JsValue::from_str(value))
                .map_err(|_| JsValue::from_str("failed to set image attribute"))?;
        }
        js_sys::Reflect::set(&img_obj, &JsValue::from_str("attributes"), &attrs_obj)
            .map_err(|_| JsValue::from_str("failed to set image attributes"))?;

        images_arr.push(&img_obj);
    }
    js_sys::Reflect::set(&js_result, &JsValue::from_str("images"), &images_arr)
        .map_err(|_| JsValue::from_str("failed to set images"))?;

    // warnings
    let warnings_arr = js_sys::Array::new();
    for warning in result.warnings {
        let w = serde_wasm_bindgen::to_value(&warning).map_err(|e| JsValue::from_str(&e.to_string()))?;
        warnings_arr.push(&w);
    }
    js_sys::Reflect::set(&js_result, &JsValue::from_str("warnings"), &warnings_arr)
        .map_err(|_| JsValue::from_str("failed to set warnings"))?;

    Ok(js_result.into())
}
