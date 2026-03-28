//! Integration tests for the C FFI API.
//!
//! These tests verify that the entire FFI API works correctly across all modules.

use std::ffi::{CStr, CString};
use std::ptr;

use super::*;

#[test]
fn test_basic_conversion() {
    unsafe {
        let html = CString::new("<h1>Hello World</h1>").unwrap();
        let result = html_to_markdown_convert(html.as_ptr(), ptr::null());
        assert!(!result.is_null());

        let json_str = CStr::from_ptr(result).to_str().unwrap();
        let parsed: serde_json::Value = serde_json::from_str(json_str).unwrap();
        let content = parsed["content"].as_str().unwrap_or("");
        assert!(content.contains("Hello World"));

        html_to_markdown_free_string(result);
    }
}

#[test]
fn test_null_html() {
    unsafe {
        let result = html_to_markdown_convert(ptr::null(), ptr::null());
        assert!(result.is_null());
        let err = html_to_markdown_last_error();
        assert!(!err.is_null());
        let msg = CStr::from_ptr(err).to_str().unwrap();
        assert_eq!(msg, "html pointer was null");
    }
}

#[test]
fn test_version() {
    unsafe {
        let version = html_to_markdown_version();
        assert!(!version.is_null());
        let version_str = CStr::from_ptr(version).to_str().unwrap();
        assert!(!version_str.is_empty());
    }
}

#[test]
fn test_last_error_clears_after_success() {
    unsafe {
        let _ = html_to_markdown_convert(ptr::null(), ptr::null());
        let err = html_to_markdown_last_error();
        assert!(!err.is_null());

        let html = CString::new("<p>ok</p>").unwrap();
        let result = html_to_markdown_convert(html.as_ptr(), ptr::null());
        assert!(!result.is_null());
        html_to_markdown_free_string(result);

        let cleared = html_to_markdown_last_error();
        assert!(cleared.is_null());
    }
}
