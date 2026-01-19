use crate::options::{JsConversionOptions, JsInlineImageConfig};
use crate::profiling;
use crate::types::{JsHtmlExtraction, JsInlineImage, JsInlineImageWarning};
#[cfg(feature = "metadata")]
use crate::types::{JsMetadataConfig, JsMetadataExtraction, convert_metadata};
#[cfg(feature = "async-visitor")]
use crate::visitor::JsVisitorBridge;
#[cfg(feature = "metadata")]
use html_to_markdown_bindings_common::parse_metadata_config;
use html_to_markdown_bindings_common::{error::error_message, parse_conversion_options, parse_inline_image_config};
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
use std::sync::Arc;

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

    guard_panic(|| profiling::maybe_profile(|| html_to_markdown_rs::convert(&html, rust_options.clone())))
        .map_err(to_js_error)
}

/// Convert HTML to Markdown with an async visitor object.
///
/// # Async Visitor Support
///
/// This function enables full async visitor pattern support for Node.js:
/// - JavaScript visitor callbacks are invoked asynchronously via NAPI `ThreadsafeFunction`
/// - All 30+ visitor methods are supported (links, images, headings, code, lists, tables, etc.)
/// - Callback errors gracefully default to `VisitResult::Continue`
/// - Powered by tokio async runtime for seamless JS-Rust cooperation
///
/// # Visitor Methods
///
/// Implement any combination of these optional async methods in your visitor:
/// - `visitText(ctx, text) -> { type: string, output?: string }`
/// - `visitLink(ctx, href, text, title) -> VisitResult`
/// - `visitImage(ctx, src, alt, title) -> VisitResult`
/// - `visitHeading(ctx, level, text, id) -> VisitResult`
/// - `visitCodeBlock(ctx, lang, code) -> VisitResult`
/// - `visitCodeInline(ctx, code) -> VisitResult`
/// - `visitListItem(ctx, ordered, marker, text) -> VisitResult`
/// - `visitTableRow(ctx, cells, isHeader) -> VisitResult`
/// - `visitBlockquote(ctx, content, depth) -> VisitResult`
/// - And 20+ more semantic and inline element callbacks
///
/// # `VisitResult` Types
///
/// Each callback should return an object with:
/// - `type: 'continue' | 'skip' | 'custom' | 'preservehtml' | 'error'`
/// - `output?: string` (required for 'custom' and 'error' types)
///
/// # Arguments
///
/// * `html` - The HTML string to convert
/// * `options` - Optional conversion options
/// * `visitor` - Visitor object with optional async callback methods
///
/// # Example
///
/// ```javascript
/// const { convertWithVisitor } = require('@kreuzberg/html-to-markdown-node');
///
/// const html = '<a href="https://example.com">Click me</a>';
/// const visitor = {
///   visitLink: async (ctx, href, text, title) => {
///     console.log(`Found link: ${href}`);
///     return { type: 'continue' };  // Use default markdown conversion
///   }
/// };
///
/// const markdown = await convertWithVisitor(html, undefined, visitor);
/// console.log(markdown); // [Click me](https://example.com)
/// ```
///
/// @deprecated Use the optional visitor parameter in convert() instead
#[cfg(feature = "async-visitor")]
async fn convert_with_visitor_async_impl(
    html: String,
    options: Option<JsConversionOptions>,
    bridge: JsVisitorBridge,
) -> napi::Result<String> {
    let rust_options = options.map(Into::into);

    // Wrap the async visitor in an Arc<Mutex<>> for thread-safe async access
    let async_visitor_handle = std::sync::Arc::new(tokio::sync::Mutex::new(bridge));

    // Create the AsyncToSyncVisitorBridge - this spawns an async task
    let sync_bridge = html_to_markdown_rs::visitor_helpers::AsyncToSyncVisitorBridge::new(async_visitor_handle.clone());

    // Run the conversion on a blocking thread pool
    // This allows the async visitor task to process JavaScript callbacks on the async executor
    // while the conversion runs synchronously on a blocking thread
    let result = tokio::task::spawn_blocking(move || {
        // Wrap in Rc<RefCell<>> to create a VisitorHandle
        let visitor_handle = std::rc::Rc::new(std::cell::RefCell::new(sync_bridge));

        // Run the synchronous conversion
        html_to_markdown_rs::convert_with_visitor(&html, rust_options, Some(visitor_handle))
    })
    .await
    .map_err(|e| napi::Error::from_reason(format!("Task join error: {}", e)))?
    .map_err(to_js_error)?;

    Ok(result)
}

#[cfg(feature = "async-visitor")]
#[napi(js_name = "convertWithVisitor", ts_return_type = "Promise<string>")]
pub fn convert_with_visitor<'env>(
    env: &'env Env,
    html: String,
    options: Option<JsConversionOptions>,
    visitor: Object<'_>,
) -> napi::Result<PromiseRaw<'env, String>> {
    let mut bridge = JsVisitorBridge::new();

    // Extract visitor functions from the JavaScript object
    // Each function is optional - we only store it if it exists
    //
    // IMPORTANT: ThreadsafeFunctions expect functions with signature:
    //   (jsonString: string) => Promise<string>
    // where jsonString is JSON-serialized context and return value is JSON-serialized result.
    //
    // For user convenience, a TypeScript wrapper utility is provided to automatically
    // handle JSON.parse/stringify so users can write:
    //   (ctx: NodeContext) => Promise<{type: string}>
    macro_rules! extract_fn {
        ($method_name:literal, $field:ident) => {
            if let Ok(Some(func)) = visitor.get::<Function<String, Promise<String>>>($method_name) {
                match func.build_threadsafe_function().build() {
                    Ok(tsfn) => {
                        bridge.$field = Some(Arc::new(tsfn));
                    }
                    Err(_) => {
                        // Silently ignore - visitor method is optional
                    }
                }
            }
        };
    }

    extract_fn!("visitElementStart", visit_element_start_fn);
    extract_fn!("visitElementEnd", visit_element_end_fn);
    extract_fn!("visitText", visit_text_fn);
    extract_fn!("visitLink", visit_link_fn);
    extract_fn!("visitImage", visit_image_fn);
    extract_fn!("visitHeading", visit_heading_fn);
    extract_fn!("visitCodeBlock", visit_code_block_fn);
    extract_fn!("visitCodeInline", visit_code_inline_fn);
    extract_fn!("visitListItem", visit_list_item_fn);
    extract_fn!("visitListStart", visit_list_start_fn);
    extract_fn!("visitListEnd", visit_list_end_fn);
    extract_fn!("visitTableStart", visit_table_start_fn);
    extract_fn!("visitTableRow", visit_table_row_fn);
    extract_fn!("visitTableEnd", visit_table_end_fn);
    extract_fn!("visitBlockquote", visit_blockquote_fn);
    extract_fn!("visitStrong", visit_strong_fn);
    extract_fn!("visitEmphasis", visit_emphasis_fn);
    extract_fn!("visitStrikethrough", visit_strikethrough_fn);
    extract_fn!("visitUnderline", visit_underline_fn);
    extract_fn!("visitSubscript", visit_subscript_fn);
    extract_fn!("visitSuperscript", visit_superscript_fn);
    extract_fn!("visitMark", visit_mark_fn);
    extract_fn!("visitLineBreak", visit_line_break_fn);
    extract_fn!("visitHorizontalRule", visit_horizontal_rule_fn);
    extract_fn!("visitCustomElement", visit_custom_element_fn);
    extract_fn!("visitDefinitionListStart", visit_definition_list_start_fn);
    extract_fn!("visitDefinitionTerm", visit_definition_term_fn);
    extract_fn!("visitDefinitionDescription", visit_definition_description_fn);
    extract_fn!("visitDefinitionListEnd", visit_definition_list_end_fn);
    extract_fn!("visitForm", visit_form_fn);
    extract_fn!("visitInput", visit_input_fn);
    extract_fn!("visitButton", visit_button_fn);
    extract_fn!("visitAudio", visit_audio_fn);
    extract_fn!("visitVideo", visit_video_fn);
    extract_fn!("visitIframe", visit_iframe_fn);
    extract_fn!("visitDetails", visit_details_fn);
    extract_fn!("visitSummary", visit_summary_fn);
    extract_fn!("visitFigureStart", visit_figure_start_fn);
    extract_fn!("visitFigcaption", visit_figcaption_fn);
    extract_fn!("visitFigureEnd", visit_figure_end_fn);

    // Spawn the async work and return a Promise
    // env.spawn_future() spawns the async task and returns a PromiseRaw
    env.spawn_future(convert_with_visitor_async_impl(html, options, bridge))
}

#[napi(js_name = "convertJson")]
pub fn convert_json(html: String, options_json: Option<String>) -> Result<String> {
    let rust_options = parse_conversion_options(options_json.as_deref()).map_err(to_js_error)?;
    guard_panic(|| profiling::maybe_profile(|| html_to_markdown_rs::convert(&html, rust_options.clone())))
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
    guard_panic(|| profiling::maybe_profile(|| html_to_markdown_rs::convert(html, rust_options.clone())))
        .map_err(to_js_error)
}

#[napi(js_name = "convertBufferJson")]
pub fn convert_buffer_json(html: Buffer, options_json: Option<String>) -> Result<String> {
    let html = buffer_to_str(&html)?;
    let rust_options = parse_conversion_options(options_json.as_deref()).map_err(to_js_error)?;
    guard_panic(|| profiling::maybe_profile(|| html_to_markdown_rs::convert(html, rust_options.clone())))
        .map_err(to_js_error)
}

/// Create a reusable `ConversionOptions` handle.
#[napi]
pub fn create_conversion_options_handle(options: Option<JsConversionOptions>) -> External<RustConversionOptions> {
    External::new(options.map(Into::into).unwrap_or_default())
}

#[napi(js_name = "createConversionOptionsHandleJson")]
pub fn create_conversion_options_handle_json(options_json: Option<String>) -> Result<External<RustConversionOptions>> {
    let rust_options = parse_conversion_options(options_json.as_deref()).map_err(to_js_error)?;
    Ok(External::new(rust_options.unwrap_or_default()))
}

/// Create a reusable `MetadataConfig` handle.
#[cfg(feature = "metadata")]
#[napi]
pub fn create_metadata_config_handle(metadata_config: Option<JsMetadataConfig>) -> External<RustMetadataConfig> {
    External::new(metadata_config.map(Into::into).unwrap_or_default())
}

#[cfg(feature = "metadata")]
#[napi(js_name = "createMetadataConfigHandleJson")]
pub fn create_metadata_config_handle_json(
    metadata_config_json: Option<String>,
) -> Result<External<RustMetadataConfig>> {
    let rust_config = parse_metadata_config(metadata_config_json.as_deref()).map_err(to_js_error)?;
    Ok(External::new(rust_config))
}

/// Convert HTML using a previously-created `ConversionOptions` handle.
#[napi]
pub fn convert_with_options_handle(html: String, options: &External<RustConversionOptions>) -> Result<String> {
    guard_panic(|| profiling::maybe_profile(|| html_to_markdown_rs::convert(&html, Some((**options).clone()))))
        .map_err(to_js_error)
}

/// Convert HTML Buffer data using a previously-created `ConversionOptions` handle.
#[napi(js_name = "convertBufferWithOptionsHandle")]
pub fn convert_buffer_with_options_handle(html: Buffer, options: &External<RustConversionOptions>) -> Result<String> {
    let html = buffer_to_str(&html)?;
    guard_panic(|| profiling::maybe_profile(|| html_to_markdown_rs::convert(html, Some((**options).clone()))))
        .map_err(to_js_error)
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

fn convert_inline_images_json_impl(
    html: &str,
    options_json: Option<String>,
    image_config_json: Option<String>,
) -> Result<JsHtmlExtraction> {
    let rust_options = parse_conversion_options(options_json.as_deref()).map_err(to_js_error)?;
    let rust_config = parse_inline_image_config(image_config_json.as_deref()).map_err(to_js_error)?;

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

#[napi(js_name = "convertWithInlineImagesJson")]
pub fn convert_with_inline_images_json(
    html: String,
    options_json: Option<String>,
    image_config_json: Option<String>,
) -> Result<JsHtmlExtraction> {
    convert_inline_images_json_impl(&html, options_json, image_config_json)
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

#[napi(js_name = "convertInlineImagesBufferJson")]
pub fn convert_inline_images_buffer_json(
    html: Buffer,
    options_json: Option<String>,
    image_config_json: Option<String>,
) -> Result<JsHtmlExtraction> {
    let html = buffer_to_str(&html)?;
    convert_inline_images_json_impl(html, options_json, image_config_json)
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

#[cfg(feature = "metadata")]
#[napi(js_name = "convertWithMetadataJson")]
pub fn convert_with_metadata_json(
    html: String,
    options_json: Option<String>,
    metadata_config_json: Option<String>,
) -> Result<JsMetadataExtraction> {
    convert_metadata_json_impl(&html, options_json, metadata_config_json)
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

/// Convert HTML from Buffer/Uint8Array with metadata extraction using JSON config.
#[cfg(feature = "metadata")]
#[napi(js_name = "convertWithMetadataBufferJson")]
pub fn convert_with_metadata_buffer_json(
    html: Buffer,
    options_json: Option<String>,
    metadata_config_json: Option<String>,
) -> Result<JsMetadataExtraction> {
    let html = buffer_to_str(&html)?;
    convert_metadata_json_impl(html, options_json, metadata_config_json)
}

#[cfg(feature = "metadata")]
fn convert_metadata_json_impl(
    html: &str,
    options_json: Option<String>,
    metadata_config_json: Option<String>,
) -> Result<JsMetadataExtraction> {
    let rust_options = parse_conversion_options(options_json.as_deref()).map_err(to_js_error)?;
    let rust_config = parse_metadata_config(metadata_config_json.as_deref()).map_err(to_js_error)?;

    let (markdown, metadata) =
        guard_panic(|| html_to_markdown_rs::convert_with_metadata(html, rust_options, rust_config, None))
            .map_err(to_js_error)?;

    Ok(JsMetadataExtraction {
        markdown,
        metadata: convert_metadata(metadata),
    })
}
