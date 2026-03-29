//! C FFI bindings for html-to-markdown.
//!
//! Provides a C-compatible API that can be consumed by Java (Panama FFM),
//! Go (cgo), C# (P/Invoke), Zig, and other languages with C FFI support.
#![allow(clippy::all, clippy::pedantic, clippy::nursery)]
#![deny(clippy::correctness, clippy::suspicious)]
#![warn(clippy::all)]

// Core modules
pub mod conversion;
pub mod error;
pub mod metadata;
pub mod strings;
pub mod version;

// Visitor pattern support
pub mod visitor;

// Tests module
#[cfg(test)]
mod tests;

// Re-export public FFI functions for easy access
pub use conversion::{html_to_markdown_convert, html_to_markdown_free_string};
pub use error::{html_to_markdown_error_code_name, html_to_markdown_last_error, html_to_markdown_last_error_code};
pub use metadata::{
    html_to_markdown_convert_with_metadata, html_to_markdown_convert_with_metadata_bytes_with_len,
    html_to_markdown_convert_with_metadata_with_len,
};
pub use version::html_to_markdown_version;
