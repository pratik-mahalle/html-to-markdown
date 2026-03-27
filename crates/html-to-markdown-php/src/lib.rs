#![allow(clippy::all, clippy::pedantic, clippy::nursery, missing_docs)]
#![cfg_attr(windows, feature(abi_vectorcall))]
#![deny(clippy::all)]

use ext_php_rs::boxed::ZBox;
use ext_php_rs::prelude::*;
use ext_php_rs::types::{ZendHashTable, Zval};
use html_to_markdown_rs::safety::guard_panic;
use std::path::PathBuf;

// Module declarations
mod build;
mod enums;
mod options;
mod profiling;
mod types;
#[cfg(feature = "visitor")]
mod visitor_support;

// Import conversion helper functions
use build::build_html_extraction;
#[cfg(feature = "metadata")]
use build::build_metadata_extraction;
#[cfg(feature = "visitor")]
use build::build_tables_extraction;
#[cfg(feature = "metadata")]
use options::parse_metadata_config;
use options::{parse_conversion_options, parse_inline_image_config};
use types::to_php_exception;

// Convert HTML to Markdown
#[php_function]
#[php(name = "html_to_markdown_convert")]
pub fn convert_html(html: String, options: Option<&ZendHashTable>, _visitor: Option<&Zval>) -> PhpResult<String> {
    let rust_options = match options {
        Some(table) => Some(parse_conversion_options(table)?),
        None => None,
    };

    // NOTE: PHP visitor support is not yet fully implemented with ext-php-rs.
    // The visitor parameter is accepted for API compatibility but is currently ignored.
    convert_impl(&html, rust_options)
}

// Convert HTML to Markdown with inline images
#[php_function]
#[php(name = "html_to_markdown_convert_with_inline_images")]
pub fn convert_html_with_inline_images(
    html: String,
    options: Option<&ZendHashTable>,
    image_config: Option<&ZendHashTable>,
    _visitor: Option<&Zval>,
) -> PhpResult<ZBox<ZendHashTable>> {
    let rust_options = match options {
        Some(table) => Some(parse_conversion_options(table)?),
        None => None,
    };

    let config = match image_config {
        Some(table) => parse_inline_image_config(table)?,
        None => html_to_markdown_rs::InlineImageConfig::new(html_to_markdown_rs::DEFAULT_INLINE_IMAGE_LIMIT),
    };

    // NOTE: PHP visitor support is not yet fully implemented with ext-php-rs.
    // The visitor parameter is accepted for API compatibility but is currently ignored.
    let extraction = convert_with_inline_images_impl(&html, rust_options, config)?;

    build_html_extraction(extraction)
}

/// Convert HTML to Markdown with metadata extraction (requires metadata feature).
#[cfg(feature = "metadata")]
#[php_function]
#[php(name = "html_to_markdown_convert_with_metadata")]
pub fn convert_html_with_metadata(
    html: String,
    options: Option<&ZendHashTable>,
    metadata_config: Option<&ZendHashTable>,
    _visitor: Option<&Zval>,
) -> PhpResult<ZBox<ZendHashTable>> {
    let rust_options = match options {
        Some(table) => Some(parse_conversion_options(table)?),
        None => None,
    };

    let config = match metadata_config {
        Some(table) => parse_metadata_config(table)?,
        None => html_to_markdown_rs::MetadataConfig::default(),
    };

    // NOTE: PHP visitor support is not yet fully implemented with ext-php-rs.
    // The visitor parameter is accepted for API compatibility but is currently ignored.
    let (markdown, metadata) = convert_with_metadata_impl(&html, rust_options, config)?;

    build_metadata_extraction(markdown, metadata)
}

/// Convert HTML to Markdown with visitor support (requires visitor feature).
#[cfg(feature = "visitor")]
#[php_function]
#[php(name = "html_to_markdown_convert_with_visitor")]
pub fn convert_html_with_visitor(
    html: String,
    options: Option<&ZendHashTable>,
    visitor: Option<&Zval>,
) -> PhpResult<String> {
    use html_to_markdown_rs::visitor::HtmlVisitor;
    use std::cell::RefCell;
    use std::panic::AssertUnwindSafe;
    use std::rc::Rc;
    use visitor_support::PhpVisitorBridge;

    let rust_options = match options {
        Some(table) => Some(parse_conversion_options(table)?),
        None => None,
    };

    // Create visitor bridge if a PHP visitor object is provided
    let visitor_opt: Option<Rc<RefCell<dyn HtmlVisitor>>> = visitor.and_then(|v| {
        if v.is_object() {
            Some(Rc::new(RefCell::new(PhpVisitorBridge::new(v.shallow_clone()))) as Rc<RefCell<dyn HtmlVisitor>>)
        } else {
            None
        }
    });

    guard_panic(AssertUnwindSafe(|| {
        profiling::maybe_profile(|| {
            html_to_markdown_rs::convert_with_visitor(&html, rust_options.clone(), visitor_opt.clone())
        })
    }))
    .map_err(to_php_exception)
}

/// Convert HTML to Markdown with structured table extraction (requires visitor feature).
#[cfg(feature = "visitor")]
#[php_function]
#[php(name = "html_to_markdown_convert_with_tables")]
pub fn convert_html_with_tables(
    html: String,
    options: Option<&ZendHashTable>,
    metadata_config: Option<&ZendHashTable>,
) -> PhpResult<ZBox<ZendHashTable>> {
    let rust_options = match options {
        Some(table) => Some(parse_conversion_options(table)?),
        None => None,
    };

    #[cfg(feature = "metadata")]
    let config = match metadata_config {
        Some(table) => Some(parse_metadata_config(table)?),
        None => None,
    };

    #[cfg(not(feature = "metadata"))]
    let _ = metadata_config;

    let result = guard_panic(|| {
        profiling::maybe_profile(|| {
            html_to_markdown_rs::convert_with_tables(
                &html,
                rust_options.clone(),
                #[cfg(feature = "metadata")]
                config.clone(),
                #[cfg(not(feature = "metadata"))]
                None,
            )
        })
    })
    .map_err(to_php_exception)?;

    build_tables_extraction(result)
}

/// Profiling function: start profiling to a file.
#[php_function]
#[php(name = "html_to_markdown_profile_start")]
pub fn profile_start(output_path: String, frequency: Option<i64>) -> PhpResult<bool> {
    let freq = frequency.unwrap_or(1000) as i32;
    profiling::start(PathBuf::from(output_path), freq).map_err(to_php_exception)?;
    Ok(true)
}

/// Profiling function: stop profiling and write results.
#[php_function]
#[php(name = "html_to_markdown_profile_stop")]
pub fn profile_stop() -> PhpResult<bool> {
    profiling::stop().map_err(to_php_exception)?;
    Ok(true)
}

/// PHP module initialization.
#[php_module]
pub fn module(module: ModuleBuilder) -> ModuleBuilder {
    let mut builder = module
        .name("html_to_markdown")
        .version(env!("CARGO_PKG_VERSION"))
        .function(wrap_function!(convert_html))
        .function(wrap_function!(convert_html_with_inline_images))
        .function(wrap_function!(profile_start))
        .function(wrap_function!(profile_stop));

    #[cfg(feature = "metadata")]
    {
        builder = builder.function(wrap_function!(convert_html_with_metadata));
    }

    #[cfg(feature = "visitor")]
    {
        builder = builder
            .function(wrap_function!(convert_html_with_visitor))
            .function(wrap_function!(convert_html_with_tables));
    }

    builder
}

// Helper functions (private)

#[inline]
fn convert_impl(html: &str, options: Option<html_to_markdown_rs::ConversionOptions>) -> PhpResult<String> {
    guard_panic(|| profiling::maybe_profile(|| html_to_markdown_rs::convert_to_string(html, options.clone())))
        .map_err(to_php_exception)
}

#[inline]
fn convert_with_inline_images_impl(
    html: &str,
    options: Option<html_to_markdown_rs::ConversionOptions>,
    config: html_to_markdown_rs::InlineImageConfig,
) -> PhpResult<html_to_markdown_rs::HtmlExtraction> {
    guard_panic(|| html_to_markdown_rs::convert_with_inline_images(html, options, config, None))
        .map_err(to_php_exception)
}

#[cfg(feature = "metadata")]
#[inline]
fn convert_with_metadata_impl(
    html: &str,
    options: Option<html_to_markdown_rs::ConversionOptions>,
    config: html_to_markdown_rs::MetadataConfig,
) -> PhpResult<(String, html_to_markdown_rs::HtmlMetadata)> {
    guard_panic(|| html_to_markdown_rs::convert_with_metadata(html, options, config, None)).map_err(to_php_exception)
}
