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
pub mod profiling;
pub mod strings;
pub mod version;

// Feature-gated modules
#[cfg(feature = "metadata")]
pub mod metadata;

// Visitor pattern support
pub mod visitor;

// FFI-specific modules
mod profiling_ffi;

// Tests module
#[cfg(test)]
mod tests;

// Re-export public FFI functions for easy access
pub use conversion::{html_to_markdown_convert, html_to_markdown_free_string};

#[cfg(feature = "visitor")]
pub use conversion::html_to_markdown_convert_with_tables;
pub use error::{html_to_markdown_error_code_name, html_to_markdown_last_error, html_to_markdown_last_error_code};
pub use profiling_ffi::{html_to_markdown_profile_start, html_to_markdown_profile_stop};
pub use version::html_to_markdown_version;

#[cfg(feature = "metadata")]
pub use metadata::{
    html_to_markdown_convert_with_metadata, html_to_markdown_convert_with_metadata_bytes_with_len,
    html_to_markdown_convert_with_metadata_with_len,
};
