#![allow(clippy::let_unit_value, deprecated)]

use extendr_api::prelude::*;
use html_to_markdown_rs::convert_with_metadata as convert_with_metadata_inner;
use html_to_markdown_rs::convert_with_tables as convert_with_tables_inner;
use html_to_markdown_rs::{
    ConversionOptions, convert as convert_rs,
    convert_with_inline_images as convert_with_inline_images_inner,
};

fn convert_inner(html: &str, options: Option<ConversionOptions>) -> html_to_markdown_rs::error::Result<String> {
    convert_rs(html, options).map(|r| r.content.unwrap_or_default())
}
use std::path::PathBuf;

mod options;
mod profiling;
mod types;
mod visitor;

use options::{decode_inline_image_config, decode_metadata_config, decode_options};
use types::{
    conversion_result_to_robj, inline_image_to_robj, inline_image_warning_to_robj, metadata_to_robj,
    table_extraction_to_robj,
};

struct OptionsHandle(ConversionOptions);

/// Convert HTML to Markdown.
/// @param html A character string of HTML content.
/// @return A character string of Markdown content.
/// @export
#[extendr]
fn convert(html: &str) -> Result<String> {
    profiling::maybe_profile(|| convert_inner(html, None)).map_err(|e| Error::Other(e.to_string()))
}

/// Convert HTML to Markdown with options provided as a named list.
/// @param html A character string of HTML content.
/// @param options A named list of conversion options, or NULL for defaults.
/// @return A character string of Markdown content.
/// @export
#[extendr]
fn convert_with_options(html: &str, options: Robj) -> Result<String> {
    let opts = decode_options(options).map_err(|e| Error::Other(e))?;
    profiling::maybe_profile(|| convert_inner(html, Some(opts.clone()))).map_err(|e| Error::Other(e.to_string()))
}

/// Convert HTML to Markdown using a pre-created options handle.
/// @param html A character string of HTML content.
/// @param handle An options handle created by create_options_handle().
/// @return A character string of Markdown content.
/// @export
#[extendr]
fn convert_with_options_handle(html: &str, handle: ExternalPtr<OptionsHandle>) -> Result<String> {
    profiling::maybe_profile(|| convert_inner(html, Some(handle.0.clone()))).map_err(|e| Error::Other(e.to_string()))
}

/// Create a reusable options handle from a named list of options.
/// @param options A named list of conversion options.
/// @return An opaque options handle for use with convert_with_options_handle().
/// @export
#[extendr]
fn create_options_handle(options: Robj) -> Result<ExternalPtr<OptionsHandle>> {
    let opts = decode_options(options).map_err(|e| Error::Other(e))?;
    Ok(ExternalPtr::new(OptionsHandle(opts)))
}

/// Convert HTML to Markdown and extract inline images.
/// @param html A character string of HTML content.
/// @param options A named list of conversion options, or NULL for defaults.
/// @param config A named list of inline image config, or NULL for defaults.
/// @return A named list with markdown, images, and warnings.
/// @export
#[extendr]
fn convert_with_inline_images(html: &str, options: Robj, config: Robj) -> Result<Robj> {
    let opts = decode_options(options).map_err(|e| Error::Other(e))?;
    let img_config = decode_inline_image_config(config).map_err(|e| Error::Other(e))?;

    let result = profiling::maybe_profile(|| {
        convert_with_inline_images_inner(html, Some(opts.clone()), img_config.clone(), None)
    })
    .map_err(|e| Error::Other(e.to_string()))?;

    let images: Vec<Robj> = result.inline_images.into_iter().map(inline_image_to_robj).collect();

    let warnings: Vec<Robj> = result
        .warnings
        .into_iter()
        .map(|w| inline_image_warning_to_robj(w.index, w.message))
        .collect();

    Ok(list!(
        markdown = result.markdown,
        images = List::from_values(images),
        warnings = List::from_values(warnings)
    )
    .into())
}

/// Convert HTML to Markdown and extract document metadata.
/// @param html A character string of HTML content.
/// @param options A named list of conversion options, or NULL for defaults.
/// @param config A named list of metadata config, or NULL for defaults.
/// @return A named list with markdown and metadata.
/// @export
#[extendr]
fn convert_with_metadata(html: &str, options: Robj, config: Robj) -> Result<Robj> {
    let opts = decode_options(options).map_err(|e| Error::Other(e))?;
    let meta_config = decode_metadata_config(config).map_err(|e| Error::Other(e))?;

    let (markdown, metadata) =
        profiling::maybe_profile(|| convert_with_metadata_inner(html, Some(opts.clone()), meta_config.clone(), None))
            .map_err(|e| Error::Other(e.to_string()))?;

    Ok(list!(markdown = markdown, metadata = metadata_to_robj(metadata)).into())
}

/// Convert HTML to Markdown and extract tables as structured data.
/// @param html A character string of HTML content.
/// @param options A named list of conversion options, or NULL for defaults.
/// @param config A named list of metadata config, or NULL for defaults.
/// @return A named list with content, metadata, and tables.
/// @export
#[extendr]
fn convert_with_tables(html: &str, options: Robj, config: Robj) -> Result<Robj> {
    let opts = decode_options(options).map_err(|e| Error::Other(e))?;
    let meta_config = decode_metadata_config(config).map_err(|e| Error::Other(e))?;

    let result =
        profiling::maybe_profile(|| convert_with_tables_inner(html, Some(opts.clone()), Some(meta_config.clone())))
            .map_err(|e| Error::Other(e.to_string()))?;

    Ok(table_extraction_to_robj(result))
}

/// Extract structured content from HTML, returning a named list with:
///   content, metadata, tables, warnings.
/// @param html A character string of HTML content.
/// @param options A named list of conversion options, or NULL for defaults.
/// @return A named list with content (character or NULL), metadata (list), tables (list), warnings (list).
/// @export
#[extendr]
fn extract(html: &str, options: Robj) -> Result<Robj> {
    let opts = decode_options(options).map_err(|e| Error::Other(e))?;
    let result = profiling::maybe_profile(|| html_to_markdown_rs::convert(html, Some(opts.clone())))
        .map_err(|e| Error::Other(e.to_string()))?;
    Ok(conversion_result_to_robj(result))
}

/// Convert HTML to Markdown with a visitor (simplified: standard conversion).
/// @param html A character string of HTML content.
/// @param visitor A visitor object (currently unused, reserved for future use).
/// @param options A named list of conversion options, or NULL for defaults.
/// @return A character string of Markdown content.
/// @export
#[extendr]
fn convert_with_visitor(html: &str, visitor: Robj, options: Robj) -> Result<String> {
    let _ = visitor; // reserved for future callback-based visitor
    let opts = decode_options(options).map_err(|e| Error::Other(e))?;
    visitor::convert_with_visitor(html, opts).map_err(|e| Error::Other(e.to_string()))
}

/// Start CPU profiling. Requires the 'profiling' feature.
/// @param output Path to write the flamegraph SVG.
/// @param frequency Sampling frequency in Hz (default: 1000).
/// @export
#[extendr]
fn start_profiling(output: &str, frequency: Nullable<i32>) -> Result<()> {
    let freq = match frequency {
        Nullable::NotNull(f) => f,
        Nullable::Null => 1000,
    };
    profiling::start(PathBuf::from(output), freq).map_err(|e| Error::Other(e.to_string()))
}

/// Stop CPU profiling and write the flamegraph.
/// @export
#[extendr]
fn stop_profiling() -> Result<()> {
    profiling::stop().map_err(|e| Error::Other(e.to_string()))
}

/// Get the version of the html-to-markdown Rust core.
/// @return A character string with the version.
/// @export
#[extendr]
fn version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

#[extendr]
impl OptionsHandle {}

extendr_module! {
    mod htmltomarkdown;
    fn convert;
    fn extract;
    fn convert_with_options;
    fn convert_with_options_handle;
    fn create_options_handle;
    fn convert_with_inline_images;
    fn convert_with_metadata;
    fn convert_with_tables;
    fn convert_with_visitor;
    fn start_profiling;
    fn stop_profiling;
    fn version;
    impl OptionsHandle;
}
