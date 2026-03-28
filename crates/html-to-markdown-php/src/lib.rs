#![allow(clippy::pedantic, clippy::nursery, missing_docs)]
#![cfg_attr(windows, feature(abi_vectorcall))]
#![deny(clippy::correctness, clippy::suspicious)]

use ext_php_rs::boxed::ZBox;
use ext_php_rs::prelude::*;
use ext_php_rs::types::ZendHashTable;
use html_to_markdown_rs::safety::guard_panic;

// Module declarations
mod build;
mod enums;
mod options;
mod types;

use build::build_conversion_result;
use options::parse_conversion_options;
use types::to_php_exception;

// Convert HTML to Markdown, returning an associative array with:
// content, document, metadata, tables, images, warnings
#[php_function]
#[php(name = "html_to_markdown_convert")]
pub fn convert_html_full(html: String, options: Option<&ZendHashTable>) -> PhpResult<ZBox<ZendHashTable>> {
    let rust_options = match options {
        Some(table) => Some(parse_conversion_options(table)?),
        None => None,
    };

    let result = guard_panic(|| html_to_markdown_rs::convert(&html, rust_options.clone())).map_err(to_php_exception)?;

    build_conversion_result(result)
}

/// PHP module initialization.
#[php_module]
pub fn module(module: ModuleBuilder) -> ModuleBuilder {
    module
        .name("html_to_markdown")
        .version(env!("CARGO_PKG_VERSION"))
        .function(wrap_function!(convert_html_full))
}
