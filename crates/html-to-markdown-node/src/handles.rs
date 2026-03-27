use crate::options::{JsConversionOptions, JsInlineImageConfig};
use crate::profiling;
use crate::types::{JsHtmlExtraction, JsInlineImage, JsInlineImageWarning};
#[cfg(feature = "metadata")]
use crate::types::{JsMetadataConfig, JsMetadataExtraction, convert_metadata};
#[cfg(feature = "visitor")]
use crate::types::{JsTableExtraction, tables::convert_tables};
use html_to_markdown_bindings_common::error::error_message;
#[cfg(feature = "metadata")]
use html_to_markdown_rs::metadata::MetadataConfig as RustMetadataConfig;
use html_to_markdown_rs::safety::guard_panic;
use html_to_markdown_rs::{
    ConversionError, ConversionOptions as RustConversionOptions, DEFAULT_INLINE_IMAGE_LIMIT,
    InlineImageConfig as RustInlineImageConfig,
};
use napi::bindgen_prelude::*;
use napi_derive::napi;
use std::str;

fn build_extraction_json(result: html_to_markdown_rs::ConversionResult) -> Result<String> {
    use serde_json::{Value, json};

    let document = match result.document {
        Some(doc) => serde_json::to_value(&doc).map_err(|e| Error::new(Status::GenericFailure, e.to_string()))?,
        None => Value::Null,
    };

    let tables = serde_json::to_value(&result.tables).map_err(|e| Error::new(Status::GenericFailure, e.to_string()))?;

    let warnings: Vec<Value> = result
        .warnings
        .into_iter()
        .map(|w| {
            json!({
                "message": w.message,
                "kind": format!("{:?}", w.kind),
            })
        })
        .collect();

    #[cfg(feature = "metadata")]
    let metadata =
        serde_json::to_value(&result.metadata).map_err(|e| Error::new(Status::GenericFailure, e.to_string()))?;
    #[cfg(not(feature = "metadata"))]
    let metadata = Value::Null;

    let output = json!({
        "content": result.content,
        "document": document,
        "metadata": metadata,
        "tables": tables,
        "warnings": warnings,
    });

    serde_json::to_string(&output).map_err(|e| Error::new(Status::GenericFailure, e.to_string()))
}

fn to_js_error(err: ConversionError) -> Error {
    Error::new(Status::GenericFailure, error_message(&err))
}

fn buffer_to_str(html: &Buffer) -> Result<&str> {
    str::from_utf8(html.as_ref()).map_err(|e| Error::new(Status::InvalidArg, format!("HTML must be valid UTF-8: {e}")))
}

/// const { convert } = require('html-to-markdown');
///
/// const html = '<h1>Hello World</h1>';
/// const markdown = convert(html);
/// console.log(markdown); // # Hello World
/// ```
#[napi]
pub fn convert(html: String, options: Option<JsConversionOptions>, visitor: Option<Object>) -> Result<String> {
    let rust_options = options.map(Into::into);

    #[cfg(feature = "visitor")]
    if visitor.is_some() {
        // Visitor support for synchronous conversion would require a separate implementation
        return Err(Error::new(
            Status::GenericFailure,
            "Use convertWithVisitor for async visitor support",
        ));
    }
    #[cfg(not(feature = "visitor"))]
    let _ = visitor;

    guard_panic(|| profiling::maybe_profile(|| html_to_markdown_rs::convert_to_string(&html, rust_options.clone())))
        .map_err(to_js_error)
}

#[napi]
pub fn start_profiling(output_path: String, frequency: Option<i32>) -> Result<()> {
    let freq = frequency.unwrap_or(1000);
    profiling::start(output_path.into(), freq).map_err(to_js_error)
}

#[napi]
pub fn stop_profiling() -> Result<()> {
    profiling::stop().map_err(to_js_error)
}

/// Convert HTML to Markdown from a Buffer/Uint8Array without creating intermediate JS strings.
#[napi(js_name = "convertBuffer")]
pub fn convert_buffer(html: Buffer, options: Option<JsConversionOptions>) -> Result<String> {
    let html = buffer_to_str(&html)?;
    let rust_options = options.map(Into::into);
    guard_panic(|| profiling::maybe_profile(|| html_to_markdown_rs::convert_to_string(html, rust_options.clone())))
        .map_err(to_js_error)
}

/// Create a reusable `ConversionOptions` handle.
#[napi]
pub fn create_conversion_options_handle(options: Option<JsConversionOptions>) -> External<RustConversionOptions> {
    External::new(options.map(Into::into).unwrap_or_default())
}

/// Create a reusable `MetadataConfig` handle.
#[cfg(feature = "metadata")]
#[napi]
pub fn create_metadata_config_handle(metadata_config: Option<JsMetadataConfig>) -> External<RustMetadataConfig> {
    External::new(metadata_config.map(Into::into).unwrap_or_default())
}

/// Convert HTML using a previously-created `ConversionOptions` handle.
#[napi]
pub fn convert_with_options_handle(html: String, options: &External<RustConversionOptions>) -> Result<String> {
    guard_panic(|| {
        profiling::maybe_profile(|| html_to_markdown_rs::convert_to_string(&html, Some((**options).clone())))
    })
    .map_err(to_js_error)
}

/// Convert HTML Buffer data using a previously-created `ConversionOptions` handle.
#[napi(js_name = "convertBufferWithOptionsHandle")]
pub fn convert_buffer_with_options_handle(html: Buffer, options: &External<RustConversionOptions>) -> Result<String> {
    let html = buffer_to_str(&html)?;
    guard_panic(|| profiling::maybe_profile(|| html_to_markdown_rs::convert_to_string(html, Some((**options).clone()))))
        .map_err(to_js_error)
}

/// Extract structured content, metadata, and images from HTML in a single pass.
///
/// This is the v3 API entry point. Returns a JSON string encoding a `ConversionResult` object
/// with `content`, `document`, `metadata`, `tables`, and `warnings` fields.
///
/// Use `JSON.parse()` on the result to obtain a JavaScript object. This approach avoids the
/// overhead of NAPI type conversion for deeply-nested structures.
///
/// # Example
///
/// ```javascript
/// const { extract } = require('html-to-markdown');
///
/// const html = '<h1>Hello</h1><p>World</p>';
/// const result = JSON.parse(extract(html));
/// console.log(result.content);   // '# Hello\n\nWorld'
/// console.log(result.document);  // DocumentStructure object or null
/// console.log(result.tables);    // []
/// console.log(result.warnings);  // []
/// ```
#[napi]
pub fn extract(html: String, options: Option<JsConversionOptions>) -> Result<String> {
    let rust_options = options.map(Into::into);
    let result = guard_panic(|| profiling::maybe_profile(|| html_to_markdown_rs::extract(&html, rust_options.clone())))
        .map_err(to_js_error)?;
    build_extraction_json(result)
}

fn build_js_extraction(extraction: html_to_markdown_rs::HtmlExtraction) -> JsHtmlExtraction {
    let inline_images = extraction
        .inline_images
        .into_iter()
        .map(|img| JsInlineImage {
            data: img.data.into(),
            format: img.format.to_string(),
            filename: img.filename,
            description: img.description,
            dimensions: img.dimensions.map(|(w, h)| vec![w, h]),
            source: img.source.to_string(),
            attributes: img.attributes.into_iter().collect(),
        })
        .collect();

    let warnings = extraction
        .warnings
        .into_iter()
        .map(|w| JsInlineImageWarning {
            index: w.index as u32,
            message: w.message,
        })
        .collect();

    JsHtmlExtraction {
        markdown: extraction.markdown,
        inline_images,
        warnings,
    }
}

fn convert_inline_images_impl(
    html: &str,
    options: Option<JsConversionOptions>,
    image_config: Option<JsInlineImageConfig>,
) -> Result<JsHtmlExtraction> {
    let rust_options = options.map(Into::into);
    let rust_config = image_config.map_or_else(|| RustInlineImageConfig::new(DEFAULT_INLINE_IMAGE_LIMIT), Into::into);

    let extraction =
        guard_panic(|| html_to_markdown_rs::convert_with_inline_images(html, rust_options, rust_config, None))
            .map_err(to_js_error)?;

    Ok(build_js_extraction(extraction))
}

fn convert_inline_images_with_handle_impl(
    html: &str,
    options: &External<RustConversionOptions>,
    image_config: Option<JsInlineImageConfig>,
) -> Result<JsHtmlExtraction> {
    let rust_options = Some((**options).clone());
    let rust_config = image_config.map_or_else(|| RustInlineImageConfig::new(DEFAULT_INLINE_IMAGE_LIMIT), Into::into);

    let extraction =
        guard_panic(|| html_to_markdown_rs::convert_with_inline_images(html, rust_options, rust_config, None))
            .map_err(to_js_error)?;

    Ok(build_js_extraction(extraction))
}

/// Convert HTML to Markdown while collecting inline images
///
/// # Arguments
///
/// * `html` - The HTML string to convert
/// * `options` - Optional conversion options
/// * `image_config` - Configuration for inline image extraction
/// * `visitor` - Optional visitor object (when visitor feature is enabled)
#[napi]
pub fn convert_with_inline_images(
    html: String,
    options: Option<JsConversionOptions>,
    image_config: Option<JsInlineImageConfig>,
    visitor: Option<Object>,
) -> Result<JsHtmlExtraction> {
    #[cfg(feature = "visitor")]
    if visitor.is_some() {
        return Err(Error::new(
            Status::GenericFailure,
            "Use convertWithVisitor for async visitor support",
        ));
    }
    #[cfg(not(feature = "visitor"))]
    let _ = visitor;

    convert_inline_images_impl(&html, options, image_config)
}

/// Convert HTML to Markdown while collecting inline images using a pre-created options handle.
#[napi(js_name = "convertWithInlineImagesHandle")]
pub fn convert_with_inline_images_handle(
    html: String,
    options: &External<RustConversionOptions>,
    image_config: Option<JsInlineImageConfig>,
) -> Result<JsHtmlExtraction> {
    convert_inline_images_with_handle_impl(&html, options, image_config)
}

/// Convert inline images from Buffer/Uint8Array input without an intermediate string allocation.
#[napi(js_name = "convertInlineImagesBuffer")]
pub fn convert_inline_images_buffer(
    html: Buffer,
    options: Option<JsConversionOptions>,
    image_config: Option<JsInlineImageConfig>,
) -> Result<JsHtmlExtraction> {
    let html = buffer_to_str(&html)?;
    convert_inline_images_impl(html, options, image_config)
}

/// Convert inline images from Buffer/Uint8Array input using a pre-created options handle.
#[napi(js_name = "convertInlineImagesBufferWithOptionsHandle")]
pub fn convert_inline_images_buffer_with_options_handle(
    html: Buffer,
    options: &External<RustConversionOptions>,
    image_config: Option<JsInlineImageConfig>,
) -> Result<JsHtmlExtraction> {
    let html = buffer_to_str(&html)?;
    convert_inline_images_with_handle_impl(html, options, image_config)
}

/// Convert HTML to Markdown with metadata extraction.
///
/// # Arguments
///
/// * `html` - The HTML string to convert
/// * `options` - Optional conversion options
/// * `metadata_config` - Optional metadata extraction configuration
/// * `visitor` - Optional visitor object (when visitor feature is enabled)
///
/// # Example
///
/// ```javascript
/// const { convertWithMetadata } = require('html-to-markdown');
///
/// const html = '<html lang="en"><head><title>Test</title></head><body><h1>Hello</h1></body></html>';
/// const config = { extractHeaders: true, extractLinks: true };
/// const result = convertWithMetadata(html, undefined, config);
/// console.log(result.markdown);
/// console.log(result.metadata.document.title);
/// ```
#[cfg(feature = "metadata")]
#[napi(js_name = "convertWithMetadata")]
pub fn convert_with_metadata(
    html: String,
    options: Option<JsConversionOptions>,
    metadata_config: Option<JsMetadataConfig>,
    visitor: Option<Object>,
) -> Result<JsMetadataExtraction> {
    #[cfg(feature = "visitor")]
    if visitor.is_some() {
        return Err(Error::new(
            Status::GenericFailure,
            "Use convertWithVisitor for async visitor support",
        ));
    }
    #[cfg(not(feature = "visitor"))]
    let _ = visitor;

    let rust_options = options.map(Into::into);
    let rust_config = metadata_config.map(Into::into).unwrap_or_default();

    let (markdown, metadata) =
        guard_panic(|| html_to_markdown_rs::convert_with_metadata(&html, rust_options, rust_config, None))
            .map_err(to_js_error)?;

    Ok(JsMetadataExtraction {
        markdown,
        metadata: convert_metadata(metadata),
    })
}

/// Convert HTML to Markdown with metadata extraction using a pre-created options handle.
#[cfg(feature = "metadata")]
#[napi(js_name = "convertWithMetadataHandle")]
pub fn convert_with_metadata_handle(
    html: String,
    options: &External<RustConversionOptions>,
    metadata_config: Option<JsMetadataConfig>,
) -> Result<JsMetadataExtraction> {
    let rust_config = metadata_config.map(Into::into).unwrap_or_default();
    let (markdown, metadata) =
        guard_panic(|| html_to_markdown_rs::convert_with_metadata(&html, Some((**options).clone()), rust_config, None))
            .map_err(to_js_error)?;

    Ok(JsMetadataExtraction {
        markdown,
        metadata: convert_metadata(metadata),
    })
}

/// Convert HTML from Buffer/Uint8Array with metadata extraction without intermediate string allocation.
#[cfg(feature = "metadata")]
#[napi(js_name = "convertWithMetadataBuffer")]
pub fn convert_with_metadata_buffer(
    html: Buffer,
    options: Option<JsConversionOptions>,
    metadata_config: Option<JsMetadataConfig>,
) -> Result<JsMetadataExtraction> {
    let html = buffer_to_str(&html)?;
    let rust_options = options.map(Into::into);
    let rust_config = metadata_config.map(Into::into).unwrap_or_default();

    let (markdown, metadata) =
        guard_panic(|| html_to_markdown_rs::convert_with_metadata(html, rust_options, rust_config, None))
            .map_err(to_js_error)?;

    Ok(JsMetadataExtraction {
        markdown,
        metadata: convert_metadata(metadata),
    })
}

/// Convert HTML from Buffer/Uint8Array with metadata extraction using a pre-created options handle.
#[cfg(feature = "metadata")]
#[napi(js_name = "convertWithMetadataBufferWithOptionsHandle")]
pub fn convert_with_metadata_buffer_with_options_handle(
    html: Buffer,
    options: &External<RustConversionOptions>,
    metadata_config: Option<JsMetadataConfig>,
) -> Result<JsMetadataExtraction> {
    let html = buffer_to_str(&html)?;
    let rust_config = metadata_config.map(Into::into).unwrap_or_default();

    let (markdown, metadata) =
        guard_panic(|| html_to_markdown_rs::convert_with_metadata(html, Some((**options).clone()), rust_config, None))
            .map_err(to_js_error)?;

    Ok(JsMetadataExtraction {
        markdown,
        metadata: convert_metadata(metadata),
    })
}

/// Convert HTML from Buffer/Uint8Array with metadata extraction using a metadata handle.
#[cfg(feature = "metadata")]
#[napi(js_name = "convertWithMetadataBufferWithMetadataHandle")]
pub fn convert_with_metadata_buffer_with_metadata_handle(
    html: Buffer,
    metadata_config: &External<RustMetadataConfig>,
) -> Result<JsMetadataExtraction> {
    let html = buffer_to_str(&html)?;
    let rust_config = (**metadata_config).clone();
    let (markdown, metadata) =
        guard_panic(|| html_to_markdown_rs::convert_with_metadata(html, None, rust_config, None))
            .map_err(to_js_error)?;

    Ok(JsMetadataExtraction {
        markdown,
        metadata: convert_metadata(metadata),
    })
}

/// Convert HTML from Buffer/Uint8Array with metadata extraction using options + metadata handles.
#[cfg(feature = "metadata")]
#[napi(js_name = "convertWithMetadataBufferWithOptionsAndMetadataHandle")]
pub fn convert_with_metadata_buffer_with_options_and_metadata_handle(
    html: Buffer,
    options: &External<RustConversionOptions>,
    metadata_config: &External<RustMetadataConfig>,
) -> Result<JsMetadataExtraction> {
    let html = buffer_to_str(&html)?;
    let rust_config = (**metadata_config).clone();
    let (markdown, metadata) =
        guard_panic(|| html_to_markdown_rs::convert_with_metadata(html, Some((**options).clone()), rust_config, None))
            .map_err(to_js_error)?;

    Ok(JsMetadataExtraction {
        markdown,
        metadata: convert_metadata(metadata),
    })
}

/// Convert HTML to Markdown with structured table extraction.
///
/// Returns converted content alongside all tables found in the HTML.
/// When the metadata feature is enabled, metadata extraction is also performed.
///
/// # Arguments
///
/// * `html` - The HTML string to convert
/// * `options` - Optional conversion options
/// * `metadata_config` - Optional metadata extraction configuration (when metadata feature is enabled)
///
/// # Example
///
/// ```javascript
/// const { convertWithTables } = require('html-to-markdown');
///
/// const html = '<table><tr><th>Name</th></tr><tr><td>Alice</td></tr></table>';
/// const result = convertWithTables(html);
/// console.log(result.tables[0].cells);
/// ```
#[cfg(all(feature = "visitor", feature = "metadata"))]
#[napi(js_name = "convertWithTables")]
pub fn convert_with_tables(
    html: String,
    options: Option<JsConversionOptions>,
    metadata_config: Option<JsMetadataConfig>,
) -> Result<JsTableExtraction> {
    let rust_options = options.map(Into::into);
    let rust_metadata_cfg = metadata_config.map(Into::into);

    let result = guard_panic(|| html_to_markdown_rs::convert_with_tables(&html, rust_options, rust_metadata_cfg))
        .map_err(to_js_error)?;

    Ok(JsTableExtraction {
        content: result.content,
        metadata: result.metadata.map(convert_metadata),
        tables: convert_tables(result.tables),
    })
}

#[cfg(all(feature = "visitor", not(feature = "metadata")))]
#[napi(js_name = "convertWithTables")]
pub fn convert_with_tables(html: String, options: Option<JsConversionOptions>) -> Result<JsTableExtraction> {
    let rust_options = options.map(Into::into);

    let result =
        guard_panic(|| html_to_markdown_rs::convert_with_tables(&html, rust_options, None)).map_err(to_js_error)?;

    Ok(JsTableExtraction {
        content: result.content,
        tables: convert_tables(result.tables),
    })
}
