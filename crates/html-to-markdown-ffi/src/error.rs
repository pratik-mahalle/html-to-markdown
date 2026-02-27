//! Error handling for C FFI.
//!
//! This module provides thread-local error storage and utilities for capturing
//! and reporting errors across the FFI boundary.

use std::cell::RefCell;
use std::ffi::CString;
use std::os::raw::c_char;
use std::ptr;

use html_to_markdown_rs::ConversionError;

/// Error codes returned by FFI functions.
///
/// These codes classify errors into broad categories that C consumers
/// can switch on without parsing error message strings.
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum HtmlToMarkdownErrorCode {
    /// No error.
    Ok = 0,
    /// Input was not valid UTF-8.
    InvalidUtf8 = 1,
    /// HTML parsing or sanitization failed.
    Parse = 2,
    /// A visitor callback returned an error.
    Visitor = 3,
    /// Memory allocation failure.
    Memory = 4,
    /// Internal error (I/O, panic, or other).
    Internal = 5,
}

thread_local! {
    static LAST_ERROR: RefCell<Option<CString>> = const { RefCell::new(None) };
    static LAST_ERROR_CODE: RefCell<HtmlToMarkdownErrorCode> = const { RefCell::new(HtmlToMarkdownErrorCode::Ok) };
}

/// Set the thread-local last error message.
///
/// # Arguments
///
/// * `message` - Optional error message. If `None`, clears the error.
pub fn set_last_error(message: Option<String>) {
    LAST_ERROR.with(|cell| {
        let mut slot = cell.borrow_mut();
        *slot = message.and_then(|msg| CString::new(msg).ok());
    });
}

/// Set the thread-local last error code.
pub fn set_last_error_code(code: HtmlToMarkdownErrorCode) {
    LAST_ERROR_CODE.with(|cell| *cell.borrow_mut() = code);
}

/// Classify a `ConversionError` into an `HtmlToMarkdownErrorCode`.
pub fn classify_conversion_error(err: &ConversionError) -> HtmlToMarkdownErrorCode {
    match err {
        ConversionError::ParseError(_) | ConversionError::SanitizationError(_) => HtmlToMarkdownErrorCode::Parse,
        ConversionError::InvalidInput(_) => HtmlToMarkdownErrorCode::Parse,
        ConversionError::ConfigError(_) => HtmlToMarkdownErrorCode::Internal,
        ConversionError::IoError(_) => HtmlToMarkdownErrorCode::Internal,
        ConversionError::Panic(_) | ConversionError::Other(_) => HtmlToMarkdownErrorCode::Internal,
        // The dependency on html-to-markdown-rs unconditionally enables the visitor
        // feature, so ConversionError::Visitor always exists regardless of this
        // crate's own feature flags.
        ConversionError::Visitor(_) => HtmlToMarkdownErrorCode::Visitor,
    }
}

/// Get a pointer to the last error message.
///
/// # Safety
///
/// Returns a pointer to thread-local storage. The pointer is valid only until
/// the next call to any function that modifies `LAST_ERROR`.
pub fn last_error_ptr() -> *const c_char {
    LAST_ERROR.with(|cell| {
        cell.borrow()
            .as_ref()
            .map_or(ptr::null(), |cstr| cstr.as_ptr().cast::<c_char>())
    })
}

/// Capture a `ConversionError` and store it in thread-local storage.
///
/// # Arguments
///
/// * `err` - The conversion error to capture
pub fn capture_error(err: ConversionError) {
    let code = classify_conversion_error(&err);
    set_last_error(Some(err.to_string()));
    set_last_error_code(code);
}

/// Get the last error message from a failed conversion.
///
/// # Safety
///
/// - Returns a pointer to a thread-local buffer; copy it immediately if needed
/// - Pointer is invalidated by the next call to any `html_to_markdown_*` function
/// - May return NULL if no error has occurred in this thread
#[unsafe(no_mangle)]
pub unsafe extern "C" fn html_to_markdown_last_error() -> *const c_char {
    last_error_ptr()
}

/// Get the error code from the last failed operation.
///
/// Returns 0 (Ok) if no error has occurred.
#[unsafe(no_mangle)]
pub extern "C" fn html_to_markdown_last_error_code() -> u32 {
    LAST_ERROR_CODE.with(|cell| *cell.borrow() as u32)
}

/// Get the name of an error code as a C string.
///
/// Returns a pointer to a static string. Do not free.
/// Returns "unknown" for invalid codes.
#[unsafe(no_mangle)]
pub extern "C" fn html_to_markdown_error_code_name(code: u32) -> *const c_char {
    match code {
        0 => c"ok".as_ptr(),
        1 => c"invalid_utf8".as_ptr(),
        2 => c"parse".as_ptr(),
        3 => c"visitor".as_ptr(),
        4 => c"memory".as_ptr(),
        5 => c"internal".as_ptr(),
        _ => c"unknown".as_ptr(),
    }
}
