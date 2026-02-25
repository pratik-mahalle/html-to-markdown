//! Profiling FFI functions for C API.
//!
//! This module provides C-compatible profiling API that integrates with the
//! underlying profiling infrastructure in the profiling module.

use std::ffi::CStr;
use std::os::raw::c_char;

use crate::error::{HtmlToMarkdownErrorCode, set_last_error, set_last_error_code};
use crate::profiling;

/// Start Rust-side profiling and write a flamegraph to the specified path.
///
/// Returns 1 on success, 0 on failure. Use `html_to_markdown_last_error` to inspect failures.
///
/// # Safety
///
/// - `output` must be a valid, null-terminated UTF-8 C string for the duration of the call.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn html_to_markdown_profile_start(output: *const c_char, frequency: i32) -> bool {
    if output.is_null() {
        set_last_error(Some("output path was null".to_string()));
        return false;
    }

    let output_str = match unsafe { CStr::from_ptr(output) }.to_str() {
        Ok(s) => s,
        Err(_) => {
            set_last_error(Some("output path must be valid UTF-8".to_string()));
            set_last_error_code(HtmlToMarkdownErrorCode::InvalidUtf8);
            return false;
        }
    };

    match profiling::start(output_str.into(), frequency) {
        Ok(()) => {
            set_last_error(None);
            true
        }
        Err(err) => {
            crate::error::capture_error(err);
            false
        }
    }
}

/// Stop Rust-side profiling and flush the flamegraph.
///
/// Returns 1 on success, 0 on failure. Use `html_to_markdown_last_error` to inspect failures.
///
/// # Safety
///
/// - This must only be called after a successful `html_to_markdown_profile_start`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn html_to_markdown_profile_stop() -> bool {
    match profiling::stop() {
        Ok(()) => {
            set_last_error(None);
            true
        }
        Err(err) => {
            crate::error::capture_error(err);
            false
        }
    }
}
