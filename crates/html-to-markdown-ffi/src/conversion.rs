//! Basic HTML to Markdown conversion functions for C FFI.
//!
//! This module provides simple conversion functions without additional features
//! like metadata extraction or callbacks.

use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use std::ptr;
use std::slice;

use html_to_markdown_rs::convert_to_string as convert;
use html_to_markdown_rs::safety::guard_panic;

use crate::error::{HtmlToMarkdownErrorCode, capture_error, set_last_error, set_last_error_code};
use crate::profiling;
use crate::strings::string_to_c_string;

/// Convert HTML to Markdown using default options.
///
/// # Safety
///
/// - `html` must be a valid null-terminated C string
/// - The returned string must be freed with `html_to_markdown_free_string`
/// - Returns NULL on error
///
/// # Example (C)
///
/// ```c
/// const char* html = "<h1>Hello</h1>";
/// char* markdown = html_to_markdown_convert(html);
/// if (markdown != NULL) {
///     printf("%s\n", markdown);
///     html_to_markdown_free_string(markdown);
/// }
/// ```
#[unsafe(no_mangle)]
pub unsafe extern "C" fn html_to_markdown_convert(html: *const c_char) -> *mut c_char {
    if html.is_null() {
        set_last_error(Some("html pointer was null".to_string()));
        set_last_error_code(HtmlToMarkdownErrorCode::Internal);
        return ptr::null_mut();
    }

    let html_str = match unsafe { CStr::from_ptr(html) }.to_str() {
        Ok(s) => s,
        Err(_) => {
            set_last_error(Some("html must be valid UTF-8".to_string()));
            set_last_error_code(HtmlToMarkdownErrorCode::InvalidUtf8);
            return ptr::null_mut();
        }
    };

    match guard_panic(|| profiling::maybe_profile(|| convert(html_str, None))) {
        Ok(markdown) => {
            set_last_error(None);
            set_last_error_code(HtmlToMarkdownErrorCode::Ok);
            match string_to_c_string(markdown, "markdown result") {
                Ok(c_string) => c_string.into_raw(),
                Err(err) => {
                    set_last_error(Some(format!("failed to build CString for markdown result: {err}")));
                    set_last_error_code(HtmlToMarkdownErrorCode::Internal);
                    ptr::null_mut()
                }
            }
        }
        Err(err) => {
            capture_error(err);
            ptr::null_mut()
        }
    }
}

/// Convert HTML to Markdown using default options, returning the output length.
///
/// # Safety
///
/// - `html` must be a valid null-terminated C string
/// - `len_out` must be a valid pointer to a size_t
/// - The returned string must be freed with `html_to_markdown_free_string`
/// - Returns NULL on error
#[unsafe(no_mangle)]
pub unsafe extern "C" fn html_to_markdown_convert_with_len(html: *const c_char, len_out: *mut usize) -> *mut c_char {
    if html.is_null() {
        set_last_error(Some("html pointer was null".to_string()));
        set_last_error_code(HtmlToMarkdownErrorCode::Internal);
        return ptr::null_mut();
    }

    if len_out.is_null() {
        set_last_error(Some("len_out pointer was null".to_string()));
        set_last_error_code(HtmlToMarkdownErrorCode::Internal);
        return ptr::null_mut();
    }

    let html_str = match unsafe { CStr::from_ptr(html) }.to_str() {
        Ok(s) => s,
        Err(_) => {
            set_last_error(Some("html must be valid UTF-8".to_string()));
            set_last_error_code(HtmlToMarkdownErrorCode::InvalidUtf8);
            return ptr::null_mut();
        }
    };

    match guard_panic(|| profiling::maybe_profile(|| convert(html_str, None))) {
        Ok(markdown) => {
            set_last_error(None);
            set_last_error_code(HtmlToMarkdownErrorCode::Ok);
            match string_to_c_string(markdown, "markdown result") {
                Ok(c_string) => {
                    unsafe {
                        *len_out = c_string.as_bytes().len();
                    }
                    c_string.into_raw()
                }
                Err(err) => {
                    set_last_error(Some(format!("failed to build CString for markdown result: {err}")));
                    set_last_error_code(HtmlToMarkdownErrorCode::Internal);
                    ptr::null_mut()
                }
            }
        }
        Err(err) => {
            capture_error(err);
            ptr::null_mut()
        }
    }
}

/// Convert UTF-8 HTML bytes to Markdown and return the output length.
///
/// # Safety
///
/// - `html` must point to `len` bytes of UTF-8 data
/// - `len_out` must be a valid pointer to a size_t
/// - The returned string must be freed with `html_to_markdown_free_string`
/// - Returns NULL on error
#[unsafe(no_mangle)]
pub unsafe extern "C" fn html_to_markdown_convert_bytes_with_len(
    html: *const u8,
    len: usize,
    len_out: *mut usize,
) -> *mut c_char {
    if html.is_null() {
        set_last_error(Some("html pointer was null".to_string()));
        set_last_error_code(HtmlToMarkdownErrorCode::Internal);
        return ptr::null_mut();
    }

    if len_out.is_null() {
        set_last_error(Some("len_out pointer was null".to_string()));
        set_last_error_code(HtmlToMarkdownErrorCode::Internal);
        return ptr::null_mut();
    }

    let html_bytes = unsafe { slice::from_raw_parts(html, len) };
    let html_str = match std::str::from_utf8(html_bytes) {
        Ok(s) => s,
        Err(_) => {
            set_last_error(Some("html must be valid UTF-8".to_string()));
            set_last_error_code(HtmlToMarkdownErrorCode::InvalidUtf8);
            return ptr::null_mut();
        }
    };

    match guard_panic(|| profiling::maybe_profile(|| convert(html_str, None))) {
        Ok(markdown) => {
            set_last_error(None);
            set_last_error_code(HtmlToMarkdownErrorCode::Ok);
            match string_to_c_string(markdown, "markdown result") {
                Ok(c_string) => {
                    unsafe {
                        *len_out = c_string.as_bytes().len();
                    }
                    c_string.into_raw()
                }
                Err(err) => {
                    set_last_error(Some(format!("failed to build CString for markdown result: {err}")));
                    set_last_error_code(HtmlToMarkdownErrorCode::Internal);
                    ptr::null_mut()
                }
            }
        }
        Err(err) => {
            capture_error(err);
            ptr::null_mut()
        }
    }
}

/// Convert HTML to Markdown with table extraction, returning a JSON string.
///
/// The returned JSON has the shape:
/// ```json
/// {
///   "content": "...",
///   "metadata": {...} | null,
///   "tables": [{"cells": [[...]], "markdown": "...", "is_header_row": [...]}]
/// }
/// ```
///
/// # Safety
///
/// - `html` must be a valid null-terminated C string
/// - `options_json` may be NULL (uses defaults) or a valid null-terminated JSON C string
/// - `metadata_config_json` may be NULL (uses defaults) or a valid null-terminated JSON C string
/// - The returned string must be freed with `html_to_markdown_free_string`
/// - Returns NULL on error (check error with `html_to_markdown_last_error`)
///
/// # Example (C)
///
/// ```c
/// const char* html = "<table><tr><th>Name</th></tr><tr><td>Alice</td></tr></table>";
/// char* json = html_to_markdown_convert_with_tables(html, NULL, NULL);
/// if (json != NULL) {
///     printf("%s\n", json);
///     html_to_markdown_free_string(json);
/// }
/// ```
#[cfg(feature = "visitor")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn html_to_markdown_convert_with_tables(
    html: *const c_char,
    options_json: *const c_char,
    metadata_config_json: *const c_char,
) -> *mut c_char {
    if html.is_null() {
        set_last_error(Some("html pointer was null".to_string()));
        set_last_error_code(HtmlToMarkdownErrorCode::Internal);
        return ptr::null_mut();
    }

    let html_str = match unsafe { CStr::from_ptr(html) }.to_str() {
        Ok(s) => s,
        Err(_) => {
            set_last_error(Some("html must be valid UTF-8".to_string()));
            set_last_error_code(HtmlToMarkdownErrorCode::InvalidUtf8);
            return ptr::null_mut();
        }
    };

    let options = if options_json.is_null() {
        None
    } else {
        match unsafe { CStr::from_ptr(options_json) }.to_str() {
            Ok("") => None,
            Ok(s) => match html_to_markdown_rs::conversion_options_from_json(s) {
                Ok(opts) => Some(opts),
                Err(e) => {
                    set_last_error(Some(format!("failed to parse options JSON: {e}")));
                    set_last_error_code(HtmlToMarkdownErrorCode::Internal);
                    return ptr::null_mut();
                }
            },
            Err(_) => {
                set_last_error(Some("options_json must be valid UTF-8".to_string()));
                set_last_error_code(HtmlToMarkdownErrorCode::InvalidUtf8);
                return ptr::null_mut();
            }
        }
    };

    #[cfg(feature = "metadata")]
    let metadata_cfg = if metadata_config_json.is_null() {
        None
    } else {
        match unsafe { CStr::from_ptr(metadata_config_json) }.to_str() {
            Ok("") => None,
            Ok(s) => match html_to_markdown_rs::metadata_config_from_json(s) {
                Ok(cfg) => Some(cfg),
                Err(e) => {
                    set_last_error(Some(format!("failed to parse metadata config JSON: {e}")));
                    set_last_error_code(HtmlToMarkdownErrorCode::Internal);
                    return ptr::null_mut();
                }
            },
            Err(_) => {
                set_last_error(Some("metadata_config_json must be valid UTF-8".to_string()));
                set_last_error_code(HtmlToMarkdownErrorCode::InvalidUtf8);
                return ptr::null_mut();
            }
        }
    };

    #[cfg(not(feature = "metadata"))]
    let metadata_cfg: Option<()> = None;

    match guard_panic(|| {
        profiling::maybe_profile(|| {
            html_to_markdown_rs::convert_with_tables(html_str, options.clone(), metadata_cfg.clone())
        })
    }) {
        Ok(result) => {
            set_last_error(None);
            set_last_error_code(HtmlToMarkdownErrorCode::Ok);

            let json = match serde_json::to_string(&result) {
                Ok(j) => j,
                Err(e) => {
                    set_last_error(Some(format!("failed to serialize result to JSON: {e}")));
                    set_last_error_code(HtmlToMarkdownErrorCode::Internal);
                    return ptr::null_mut();
                }
            };

            match string_to_c_string(json, "tables JSON result") {
                Ok(c_string) => c_string.into_raw(),
                Err(err) => {
                    set_last_error(Some(format!("failed to build CString for tables JSON result: {err}")));
                    set_last_error_code(HtmlToMarkdownErrorCode::Internal);
                    ptr::null_mut()
                }
            }
        }
        Err(err) => {
            capture_error(err);
            ptr::null_mut()
        }
    }
}

/// Free a string returned by html_to_markdown_convert.
///
/// Passing NULL is a safe no-op (similar to `free(NULL)` in C).
///
/// # Safety
///
/// - `s` must be a string previously returned by `html_to_markdown_convert`, or NULL
/// - `s` must not be used after this call
///
/// # Example (C)
///
/// ```c
/// char* markdown = html_to_markdown_convert("<p>text</p>");
/// html_to_markdown_free_string(markdown);
/// // markdown is now invalid
/// ```
#[unsafe(no_mangle)]
pub unsafe extern "C" fn html_to_markdown_free_string(s: *mut c_char) {
    if !s.is_null() {
        unsafe { drop(CString::from_raw(s)) };
    }
}
