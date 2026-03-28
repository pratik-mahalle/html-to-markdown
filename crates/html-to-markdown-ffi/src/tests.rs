//! Integration tests for the C FFI API.
//!
//! These tests verify that the entire FFI API works correctly across all modules.

use std::ffi::{CStr, CString};
#[cfg(feature = "metadata")]
use std::os::raw::c_char;
use std::ptr;

use super::*;

#[cfg(feature = "visitor")]
#[test]
fn test_convert_with_tables_basic() {
    unsafe {
        let html = CString::new("<table><tr><th>Name</th><th>Age</th></tr><tr><td>Alice</td><td>30</td></tr></table>")
            .unwrap();
        let result = html_to_markdown_convert_with_tables(html.as_ptr(), ptr::null(), ptr::null());
        assert!(!result.is_null(), "Result should not be null");

        let json_str = CStr::from_ptr(result).to_str().unwrap();
        assert!(json_str.contains("\"content\""), "Should have content field");
        assert!(json_str.contains("\"tables\""), "Should have tables field");
        assert!(json_str.contains("\"cells\""), "Should have cells in table data");
        assert!(json_str.contains("\"markdown\""), "Should have markdown in table data");
        assert!(
            json_str.contains("\"is_header_row\""),
            "Should have is_header_row in table data"
        );
        assert!(json_str.contains("Name"), "Should contain table header text");
        assert!(json_str.contains("Alice"), "Should contain table cell text");

        html_to_markdown_free_string(result);
    }
}

#[cfg(feature = "visitor")]
#[test]
fn test_convert_with_tables_null_html() {
    unsafe {
        let result = html_to_markdown_convert_with_tables(ptr::null(), ptr::null(), ptr::null());
        assert!(result.is_null());
        let err = html_to_markdown_last_error();
        assert!(!err.is_null());
        let msg = CStr::from_ptr(err).to_str().unwrap();
        assert_eq!(msg, "html pointer was null");
    }
}

#[cfg(feature = "visitor")]
#[test]
fn test_convert_with_tables_no_tables() {
    unsafe {
        let html = CString::new("<p>No tables here</p>").unwrap();
        let result = html_to_markdown_convert_with_tables(html.as_ptr(), ptr::null(), ptr::null());
        assert!(!result.is_null(), "Result should not be null");

        let json_str = CStr::from_ptr(result).to_str().unwrap();
        let parsed: serde_json::Value = serde_json::from_str(json_str).unwrap();
        assert!(
            parsed["tables"].as_array().unwrap().is_empty(),
            "Should have empty tables array"
        );
        assert!(!parsed["content"].as_str().unwrap().is_empty(), "Should have content");

        html_to_markdown_free_string(result);
    }
}

#[cfg(all(feature = "visitor", feature = "metadata"))]
#[test]
fn test_convert_with_tables_with_metadata() {
    unsafe {
        let html = CString::new(
            "<html><head><title>Table Page</title></head><body><table><tr><td>Cell</td></tr></table></body></html>",
        )
        .unwrap();
        let metadata_cfg = CString::new(r#"{"extract_document": true}"#).unwrap();
        let result = html_to_markdown_convert_with_tables(html.as_ptr(), ptr::null(), metadata_cfg.as_ptr());
        assert!(!result.is_null(), "Result should not be null");

        let json_str = CStr::from_ptr(result).to_str().unwrap();
        let parsed: serde_json::Value = serde_json::from_str(json_str).unwrap();
        assert!(
            parsed["metadata"].is_object(),
            "Should have metadata when config provided"
        );
        assert!(!parsed["tables"].as_array().unwrap().is_empty(), "Should have tables");

        html_to_markdown_free_string(result);
    }
}

#[test]
fn test_basic_conversion() {
    unsafe {
        let html = CString::new("<h1>Hello World</h1>").unwrap();
        let result = html_to_markdown_convert_to_string(html.as_ptr());
        assert!(!result.is_null());

        let markdown = CStr::from_ptr(result).to_str().unwrap();
        assert!(markdown.contains("Hello World"));

        html_to_markdown_free_string(result);
    }
}

#[test]
fn test_null_html() {
    unsafe {
        let result = html_to_markdown_convert_to_string(ptr::null());
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
        let _ = html_to_markdown_convert_to_string(ptr::null());
        let err = html_to_markdown_last_error();
        assert!(!err.is_null());

        let html = CString::new("<p>ok</p>").unwrap();
        let result = html_to_markdown_convert_to_string(html.as_ptr());
        assert!(!result.is_null());
        html_to_markdown_free_string(result);

        let cleared = html_to_markdown_last_error();
        assert!(cleared.is_null());
    }
}

#[cfg(feature = "metadata")]
#[test]
fn test_convert_with_metadata_basic() {
    unsafe {
        let html = CString::new("<html><head><title>Test Page</title></head><body><h1>Hello World</h1></body></html>")
            .unwrap();
        let mut metadata_json: *mut c_char = ptr::null_mut();
        let result = html_to_markdown_convert_with_metadata(html.as_ptr(), &mut metadata_json);

        assert!(!result.is_null(), "Markdown result should not be null");
        assert!(!metadata_json.is_null(), "Metadata JSON should not be null");

        let markdown = CStr::from_ptr(result).to_str().unwrap();
        assert!(markdown.contains("Hello World"));

        let metadata = CStr::from_ptr(metadata_json).to_str().unwrap();
        assert!(metadata.contains("\"title\""));
        assert!(metadata.contains("Test Page"));

        html_to_markdown_free_string(result);
        html_to_markdown_free_string(metadata_json);
    }
}

#[cfg(feature = "metadata")]
#[test]
fn test_convert_with_metadata_json_structure() {
    unsafe {
        let html = CString::new("<html lang=\"en\"><head><title>Test</title></head><body><h1 id=\"heading\">Title</h1><a href=\"https://example.com\">Link</a></body></html>").unwrap();
        let mut metadata_json: *mut c_char = ptr::null_mut();
        let result = html_to_markdown_convert_with_metadata(html.as_ptr(), &mut metadata_json);

        assert!(!result.is_null());
        assert!(!metadata_json.is_null());

        let metadata_str = CStr::from_ptr(metadata_json).to_str().unwrap();

        assert!(metadata_str.contains("\"document\""), "Should have document field");
        assert!(metadata_str.contains("\"headers\""), "Should have headers field");
        assert!(metadata_str.contains("\"links\""), "Should have links field");
        assert!(metadata_str.contains("\"language\":\"en\""), "Should have language");
        assert!(metadata_str.contains("\"level\":1"), "Should have header level");

        html_to_markdown_free_string(result);
        html_to_markdown_free_string(metadata_json);
    }
}

#[test]
fn test_convert_with_len_reports_length() {
    unsafe {
        let html = CString::new("<p>hello</p>").unwrap();
        let mut len: usize = 0;
        let result = html_to_markdown_convert_to_string_with_len(html.as_ptr(), &mut len);
        assert!(!result.is_null());
        assert!(len > 0);
        html_to_markdown_free_string(result);
    }
}

#[test]
fn test_convert_bytes_with_len_reports_length() {
    unsafe {
        let html = b"<p>hello</p>";
        let mut len: usize = 0;
        let result = html_to_markdown_convert_to_string_bytes_with_len(html.as_ptr(), html.len(), &mut len);
        assert!(!result.is_null());
        assert!(len > 0);
        html_to_markdown_free_string(result);
    }
}

#[cfg(feature = "metadata")]
#[test]
fn test_convert_with_metadata_null_html() {
    unsafe {
        let mut metadata_json: *mut c_char = ptr::null_mut();
        let result = html_to_markdown_convert_with_metadata(ptr::null(), &mut metadata_json);

        assert!(result.is_null());
        let err = html_to_markdown_last_error();
        assert!(!err.is_null());
        let msg = CStr::from_ptr(err).to_str().unwrap();
        assert_eq!(msg, "html pointer was null");
    }
}

#[cfg(feature = "metadata")]
#[test]
fn test_convert_with_metadata_null_output_ptr() {
    unsafe {
        let html = CString::new("<p>test</p>").unwrap();
        let result = html_to_markdown_convert_with_metadata(html.as_ptr(), ptr::null_mut());

        assert!(result.is_null());
        let err = html_to_markdown_last_error();
        assert!(!err.is_null());
        let msg = CStr::from_ptr(err).to_str().unwrap();
        assert_eq!(msg, "metadata_json_out pointer was null");
    }
}

#[cfg(feature = "metadata")]
#[test]
fn test_convert_with_metadata_empty_html() {
    unsafe {
        let html = CString::new("").unwrap();
        let mut metadata_json: *mut c_char = ptr::null_mut();
        let result = html_to_markdown_convert_with_metadata(html.as_ptr(), &mut metadata_json);

        assert!(!result.is_null());
        assert!(!metadata_json.is_null());

        html_to_markdown_free_string(result);
        html_to_markdown_free_string(metadata_json);
    }
}

#[cfg(feature = "metadata")]
#[test]
fn test_convert_with_metadata_complex_document() {
    unsafe {
        let html = CString::new(
            r#"
                <html>
                <head>
                    <title>Complex Document</title>
                    <meta name="description" content="Test description">
                    <meta name="author" content="Test Author">
                </head>
                <body>
                    <h1>Main Title</h1>
                    <h2>Subtitle</h2>
                    <p>Content with <a href="https://example.com">external link</a></p>
                    <p>And <a href="/internal">internal link</a></p>
                    <img src="https://example.com/image.png" alt="Test Image">
                </body>
                </html>
            "#,
        )
        .unwrap();
        let mut metadata_json: *mut c_char = ptr::null_mut();
        let result = html_to_markdown_convert_with_metadata(html.as_ptr(), &mut metadata_json);

        assert!(!result.is_null());
        assert!(!metadata_json.is_null());

        let metadata_str = CStr::from_ptr(metadata_json).to_str().unwrap();

        assert!(metadata_str.contains("Complex Document"));
        assert!(metadata_str.contains("Test description"));
        assert!(metadata_str.contains("Test Author"));
        assert!(metadata_str.contains("Main Title"));
        assert!(metadata_str.contains("Subtitle"));
        assert!(metadata_str.contains("https://example.com"));
        assert!(metadata_str.contains("Test Image"));

        html_to_markdown_free_string(result);
        html_to_markdown_free_string(metadata_json);
    }
}

#[cfg(feature = "metadata")]
#[test]
fn test_convert_with_metadata_error_clears_both_pointers() {
    unsafe {
        let mut metadata_json: *mut c_char = ptr::null_mut();

        let result = html_to_markdown_convert_with_metadata(ptr::null(), &mut metadata_json);
        assert!(result.is_null(), "Markdown pointer should be null on error");
        assert!(metadata_json.is_null(), "Metadata pointer should remain null on error");

        let html = CString::new("<p>test</p>").unwrap();
        let result2 = html_to_markdown_convert_with_metadata(html.as_ptr(), ptr::null_mut());
        assert!(result2.is_null(), "Markdown pointer should be null on error");
    }
}

#[cfg(feature = "metadata")]
#[test]
fn test_convert_with_metadata_ensures_both_null_on_error() {
    unsafe {
        let mut metadata_json: *mut c_char = ptr::null_mut();

        let result = html_to_markdown_convert_with_metadata(ptr::null(), &mut metadata_json);

        assert!(result.is_null(), "markdown should be null on error");
        assert!(
            metadata_json.is_null(),
            "metadata should be null on error (not partially written)"
        );

        let err = html_to_markdown_last_error();
        assert!(!err.is_null(), "error should be set");
    }
}

#[cfg(feature = "metadata")]
#[test]
fn test_convert_with_metadata_null_pointer_safety() {
    unsafe {
        let html = CString::new("<html><head><title>Test</title></head></html>").unwrap();
        let result = html_to_markdown_convert_with_metadata(html.as_ptr(), ptr::null_mut());

        assert!(result.is_null(), "markdown should be null when output ptr is null");
    }
}

#[cfg(feature = "metadata")]
#[test]
fn test_convert_with_metadata_valid_json_output() {
    unsafe {
        let html = CString::new("<html><head><title>JSON Output Test</title></head></html>").unwrap();
        let mut metadata_json: *mut c_char = ptr::null_mut();
        let result = html_to_markdown_convert_with_metadata(html.as_ptr(), &mut metadata_json);

        assert!(!result.is_null(), "markdown should not be null");
        assert!(!metadata_json.is_null(), "metadata should not be null");

        let metadata_str = CStr::from_ptr(metadata_json).to_str().unwrap();

        assert!(metadata_str.contains("{"), "Should contain JSON object");
        assert!(metadata_str.contains("}"), "Should contain JSON closing brace");
        assert!(metadata_str.contains("\""), "Should contain JSON quotes");

        html_to_markdown_free_string(result);
        html_to_markdown_free_string(metadata_json);
    }
}

#[cfg(feature = "metadata")]
#[test]
fn test_convert_with_metadata_no_memory_leak_on_success() {
    unsafe {
        let html =
            CString::new("<html><head><title>Memory Test</title></head><body><h1>Header</h1></body></html>").unwrap();
        let mut metadata_json: *mut c_char = ptr::null_mut();

        for _ in 0..10 {
            let result = html_to_markdown_convert_with_metadata(html.as_ptr(), &mut metadata_json);

            assert!(!result.is_null());
            assert!(!metadata_json.is_null());

            html_to_markdown_free_string(result);
            html_to_markdown_free_string(metadata_json);
            metadata_json = ptr::null_mut();
        }
    }
}

#[cfg(feature = "metadata")]
#[test]
fn test_convert_with_metadata_with_len_reports_lengths() {
    unsafe {
        let html = CString::new("<html><body><p>hello</p></body></html>").unwrap();
        let mut metadata_json: *mut c_char = ptr::null_mut();
        let mut markdown_len: usize = 0;
        let mut metadata_len: usize = 0;
        let result = html_to_markdown_convert_with_metadata_with_len(
            html.as_ptr(),
            &mut metadata_json,
            &mut markdown_len,
            &mut metadata_len,
        );
        assert!(!result.is_null());
        assert!(!metadata_json.is_null());
        assert!(markdown_len > 0);
        assert!(metadata_len > 0);
        html_to_markdown_free_string(result);
        html_to_markdown_free_string(metadata_json);
    }
}

#[cfg(feature = "metadata")]
#[test]
fn test_convert_with_metadata_bytes_with_len_reports_lengths() {
    unsafe {
        let html = b"<html><body><p>hello</p></body></html>";
        let mut metadata_json: *mut c_char = ptr::null_mut();
        let mut markdown_len: usize = 0;
        let mut metadata_len: usize = 0;
        let result = html_to_markdown_convert_with_metadata_bytes_with_len(
            html.as_ptr(),
            html.len(),
            &mut metadata_json,
            &mut markdown_len,
            &mut metadata_len,
        );
        assert!(!result.is_null());
        assert!(!metadata_json.is_null());
        assert!(markdown_len > 0);
        assert!(metadata_len > 0);
        html_to_markdown_free_string(result);
        html_to_markdown_free_string(metadata_json);
    }
}

#[cfg(feature = "metadata")]
#[test]
fn test_convert_with_metadata_unicode_handling() {
    unsafe {
        let html = CString::new("<html><head><title>Unicode: 你好 мир 🦀</title></head></html>").unwrap();
        let mut metadata_json: *mut c_char = ptr::null_mut();
        let result = html_to_markdown_convert_with_metadata(html.as_ptr(), &mut metadata_json);

        assert!(!result.is_null());
        assert!(!metadata_json.is_null());

        let metadata_str = CStr::from_ptr(metadata_json).to_str().unwrap();
        assert!(
            metadata_str.contains("你好") || metadata_str.contains("\\u"),
            "Should handle Unicode properly"
        );

        html_to_markdown_free_string(result);
        html_to_markdown_free_string(metadata_json);
    }
}

#[cfg(feature = "metadata")]
#[test]
fn test_convert_with_metadata_all_fields_present() {
    unsafe {
        let html = CString::new(
            r#"<html lang="en">
                <head>
                    <title>All Fields</title>
                    <meta name="description" content="Test description">
                </head>
                <body>
                    <h1>Header</h1>
                    <a href="https://example.com">Link</a>
                    <img src="test.jpg" alt="Image">
                </body>
            </html>"#,
        )
        .unwrap();
        let mut metadata_json: *mut c_char = ptr::null_mut();
        let result = html_to_markdown_convert_with_metadata(html.as_ptr(), &mut metadata_json);

        assert!(!result.is_null());
        assert!(!metadata_json.is_null());

        let metadata_str = CStr::from_ptr(metadata_json).to_str().unwrap();

        assert!(metadata_str.contains("document"), "Should have document section");
        assert!(metadata_str.contains("headers"), "Should have headers section");
        assert!(metadata_str.contains("links"), "Should have links section");
        assert!(metadata_str.contains("images"), "Should have images section");

        html_to_markdown_free_string(result);
        html_to_markdown_free_string(metadata_json);
    }
}
