//! Version information for the C FFI library.
//!
//! Provides version string compatible with the package version.

use std::os::raw::c_char;

/// Get the library version string.
///
/// # Safety
///
/// - Returns a static string that does not need to be freed
#[unsafe(no_mangle)]
pub const unsafe extern "C" fn html_to_markdown_version() -> *const c_char {
    concat!(env!("CARGO_PKG_VERSION"), "\0").as_ptr().cast::<c_char>()
}
