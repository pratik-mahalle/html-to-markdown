//! C FFI functions for visitor creation and lifetime management.
//!
//! This module provides the public C FFI entry points for creating, using,
//! and freeing visitor instances during HTML to Markdown conversion.

use std::cell::RefCell;
use std::ffi::CStr;
use std::os::raw::c_char;
use std::ptr;
use std::rc::Rc;

use html_to_markdown_rs::convert_with_visitor;
use html_to_markdown_rs::visitor::HtmlVisitor;

use crate::error::{capture_error, set_last_error};
use crate::strings::string_to_c_string;

use super::registry::HtmlToMarkdownVisitorCallbacks;
use super::types::{HtmlToMarkdownVisitResult, HtmlToMarkdownVisitResultType, HtmlToMarkdownVisitor};
use super::wrapper::CVisitorWrapper;

/// Create a new visitor instance from a callback table.
///
/// Allocates a visitor handle that can be used with `html_to_markdown_convert_with_visitor()`.
/// The visitor is NOT thread-safe; each thread must create its own visitor.
///
/// # Arguments
///
/// - `callbacks`: Pointer to callback table. Must be valid for the entire lifetime
///   of the returned visitor handle.
///
/// # Returns
///
/// - Non-NULL: Opaque visitor handle (pass to convert functions)
/// - NULL: Failed to create visitor; call `html_to_markdown_last_error()` for details
///
/// # Safety
///
/// - `callbacks` must point to a valid `HtmlToMarkdownVisitorCallbacks` struct
/// - `callbacks` must remain valid until visitor is freed
/// - Returned handle must be freed with `html_to_markdown_visitor_free()`
/// - Returned handle is NOT thread-safe; don't share across threads
///
/// # Example
///
/// ```c
/// html_to_markdown_visitor_callbacks_t callbacks = {0};
/// callbacks.visit_text = my_visit_text;
/// void *visitor = html_to_markdown_visitor_create(&callbacks);
/// if (visitor == NULL) {
///     fprintf(stderr, "Failed: %s\n", html_to_markdown_last_error());
///     return 1;
/// }
/// // Use visitor...
/// html_to_markdown_visitor_free(visitor);
/// ```
#[unsafe(no_mangle)]
pub unsafe extern "C" fn html_to_markdown_visitor_create(
    callbacks: *const HtmlToMarkdownVisitorCallbacks,
) -> HtmlToMarkdownVisitor {
    if callbacks.is_null() {
        set_last_error(Some("callbacks pointer was null".to_string()));
        return ptr::null_mut();
    }

    let callbacks = unsafe { (*callbacks).clone() };
    let wrapper = CVisitorWrapper::new(callbacks);
    let handle = Rc::new(RefCell::new(wrapper));
    set_last_error(None);
    Box::into_raw(Box::new(handle)) as HtmlToMarkdownVisitor
}

/// Free a visitor instance created by `html_to_markdown_visitor_create()`.
///
/// Deallocates all resources associated with the visitor.
/// After this call, the visitor handle is invalid and must not be used.
///
/// # Arguments
///
/// - `visitor`: Visitor handle from `html_to_markdown_visitor_create()`.
///   NULL pointers are safe (no-op).
///
/// # Safety
///
/// - `visitor` must be a handle previously returned by `html_to_markdown_visitor_create()`
/// - `visitor` must not be used after this call
/// - Calling with NULL is safe but unnecessary
///
/// # Example
///
/// ```c
/// void *visitor = html_to_markdown_visitor_create(&callbacks);
/// // Use visitor in conversions...
/// html_to_markdown_visitor_free(visitor);
/// visitor = NULL; // Good practice to avoid use-after-free
/// ```
#[unsafe(no_mangle)]
pub unsafe extern "C" fn html_to_markdown_visitor_free(visitor: HtmlToMarkdownVisitor) {
    if visitor.is_null() {
        return;
    }
    let _handle = unsafe { Box::from_raw(visitor.cast::<Rc<RefCell<CVisitorWrapper>>>()) };
}

/// Convert HTML to Markdown using a custom visitor.
///
/// Performs HTML→Markdown conversion with visitor callbacks invoked at each element.
/// Returns the length of the output markdown.
///
/// # Arguments
///
/// - `html`: Null-terminated UTF-8 C string containing HTML
/// - `visitor`: Visitor handle from `html_to_markdown_visitor_create()`
/// - `len_out`: Pointer to `size_t` where output length will be written.
///   Can be NULL if length is not needed.
///
/// # Returns
///
/// - Non-NULL: Pointer to malloc'd markdown string (NULL-terminated).
///   Length written to *`len_out` if `len_out` is not NULL.
///   Must be freed with `html_to_markdown_free_string()`.
/// - NULL: Conversion failed; call `html_to_markdown_last_error()` for details
///
/// # Safety
///
/// - `html` must be a valid null-terminated UTF-8 C string
/// - `visitor` must be a handle from `html_to_markdown_visitor_create()`
/// - `len_out` (if not NULL) must be a valid pointer to `size_t`
/// - Returned string must be freed with `html_to_markdown_free_string()`
/// - Visitor callbacks are invoked on the calling thread (must not block or panic)
///
/// # Errors
///
/// - HTML parsing errors (malformed HTML)
/// - Visitor callback returning Error result
/// - Memory allocation failures
/// - UTF-8 encoding errors
///
/// # Example
///
/// ```c
/// const char *html = "<h1>Title</h1><p>Content</p>";
/// void *visitor = html_to_markdown_visitor_create(&callbacks);
/// size_t out_len = 0;
/// char *md = html_to_markdown_convert_with_visitor(html, visitor, &out_len);
/// if (md == NULL) {
///     fprintf(stderr, "Failed: %s\n", html_to_markdown_last_error());
/// } else {
///     printf("Output length: %zu\n%s\n", out_len, md);
///     html_to_markdown_free_string(md);
/// }
/// html_to_markdown_visitor_free(visitor);
/// ```
#[unsafe(no_mangle)]
pub unsafe extern "C" fn html_to_markdown_convert_with_visitor(
    html: *const c_char,
    visitor: HtmlToMarkdownVisitor,
    len_out: *mut usize,
) -> *mut c_char {
    if html.is_null() {
        set_last_error(Some("html pointer was null".to_string()));
        return ptr::null_mut();
    }

    if visitor.is_null() {
        set_last_error(Some("visitor handle was null".to_string()));
        return ptr::null_mut();
    }

    let html_str = if let Ok(s) = unsafe { CStr::from_ptr(html).to_str() } {
        s
    } else {
        set_last_error(Some("html must be valid UTF-8".to_string()));
        return ptr::null_mut();
    };

    let visitor_wrapper = unsafe { &*(visitor as *const Rc<RefCell<CVisitorWrapper>>) };
    let visitor_rc: Rc<RefCell<dyn HtmlVisitor>> = Rc::clone(visitor_wrapper) as Rc<RefCell<dyn HtmlVisitor>>;

    match convert_with_visitor(html_str, None, Some(visitor_rc)) {
        Ok(markdown) => {
            set_last_error(None);
            match string_to_c_string(markdown.clone(), "markdown result") {
                Ok(c_string) => {
                    if !len_out.is_null() {
                        unsafe { *len_out = markdown.len() };
                    }
                    c_string.into_raw()
                }
                Err(err) => {
                    set_last_error(Some(format!("failed to build CString for markdown result: {err}")));
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

/// Convert UTF-8 HTML bytes to Markdown using a custom visitor.
///
/// Variant of `html_to_markdown_convert_with_visitor()` that accepts raw byte pointers
/// instead of null-terminated C strings. Useful for data with embedded nulls or
/// when length is already known.
///
/// # Arguments
///
/// - `html`: Pointer to UTF-8 bytes (NOT null-terminated)
/// - `len`: Number of bytes pointed to by html
/// - `visitor`: Visitor handle from `html_to_markdown_visitor_create()`
/// - `len_out`: Pointer to `size_t` where output length will be written (can be NULL)
///
/// # Returns
///
/// - Non-NULL: Pointer to malloc'd markdown string (NULL-terminated).
///   Must be freed with `html_to_markdown_free_string()`.
/// - NULL: Conversion failed; call `html_to_markdown_last_error()` for details
///
/// # Safety
///
/// - `html` must point to at least `len` bytes of valid data
/// - Data must be valid UTF-8
/// - `len` must be accurate (not larger than allocated buffer)
/// - `visitor` must be a handle from `html_to_markdown_visitor_create()`
/// - `len_out` (if not NULL) must point to a valid `size_t`
///
/// # Example
///
/// ```c
/// const uint8_t *html_bytes = (const uint8_t *)input_data;
/// size_t html_len = 1024;
/// void *visitor = html_to_markdown_visitor_create(&callbacks);
/// size_t out_len = 0;
/// char *md = html_to_markdown_convert_bytes_with_visitor(html_bytes, html_len, visitor, &out_len);
/// if (md != NULL) {
///     printf("%s\n", md);
///     html_to_markdown_free_string(md);
/// }
/// html_to_markdown_visitor_free(visitor);
/// ```
#[unsafe(no_mangle)]
pub unsafe extern "C" fn html_to_markdown_convert_bytes_with_visitor(
    html: *const u8,
    len: usize,
    visitor: HtmlToMarkdownVisitor,
    len_out: *mut usize,
) -> *mut c_char {
    if html.is_null() {
        set_last_error(Some("html pointer was null".to_string()));
        return ptr::null_mut();
    }

    if visitor.is_null() {
        set_last_error(Some("visitor handle was null".to_string()));
        return ptr::null_mut();
    }

    let html_bytes = unsafe { std::slice::from_raw_parts(html, len) };
    let html_str = if let Ok(s) = std::str::from_utf8(html_bytes) {
        s
    } else {
        set_last_error(Some("html must be valid UTF-8".to_string()));
        return ptr::null_mut();
    };

    let visitor_wrapper = unsafe { &*(visitor as *const Rc<RefCell<CVisitorWrapper>>) };
    let visitor_rc: Rc<RefCell<dyn HtmlVisitor>> = Rc::clone(visitor_wrapper) as Rc<RefCell<dyn HtmlVisitor>>;

    match convert_with_visitor(html_str, None, Some(visitor_rc)) {
        Ok(markdown) => {
            set_last_error(None);
            match string_to_c_string(markdown.clone(), "markdown result") {
                Ok(c_string) => {
                    if !len_out.is_null() {
                        unsafe { *len_out = markdown.len() };
                    }
                    c_string.into_raw()
                }
                Err(err) => {
                    set_last_error(Some(format!("failed to build CString for markdown result: {err}")));
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

/// Create a `VisitResult` with Continue action.
///
/// Helper function to construct a Continue result without custom output.
/// Equivalent to: `result = {0}; result.result_type = HTML_TO_MARKDOWN_VISIT_CONTINUE;`
///
/// # Returns
///
/// `HtmlToMarkdownVisitResult` with Continue action.
///
/// # Example
///
/// ```c
/// HtmlToMarkdownVisitResult result = html_to_markdown_visit_result_continue();
/// return result;
/// ```
#[unsafe(no_mangle)]
pub const extern "C" fn html_to_markdown_visit_result_continue() -> HtmlToMarkdownVisitResult {
    HtmlToMarkdownVisitResult {
        result_type: HtmlToMarkdownVisitResultType::Continue,
        custom_output: ptr::null_mut(),
        error_message: ptr::null_mut(),
    }
}

/// Create a `VisitResult` with Custom action.
///
/// Helper function to construct a Custom result. The `output` string should be
/// allocated with `malloc()` and will be freed by the FFI layer after use.
///
/// # Arguments
///
/// - `output`: malloc'd null-terminated C string with custom markdown
///
/// # Returns
///
/// `HtmlToMarkdownVisitResult` with Custom action.
///
/// # Safety
///
/// - `output` must be malloc'd (will be freed with `free()`)
/// - `output` must be NULL-terminated
///
/// # Example
///
/// ```c
/// char *custom = (char *)malloc(100);
/// snprintf(custom, 100, "Custom markdown here");
/// HtmlToMarkdownVisitResult result = html_to_markdown_visit_result_custom(custom);
/// return result;
/// ```
#[unsafe(no_mangle)]
pub const extern "C" fn html_to_markdown_visit_result_custom(output: *mut c_char) -> HtmlToMarkdownVisitResult {
    HtmlToMarkdownVisitResult {
        result_type: HtmlToMarkdownVisitResultType::Custom,
        custom_output: output,
        error_message: ptr::null_mut(),
    }
}

/// Create a `VisitResult` with Skip action.
///
/// Helper function to construct a Skip result (element and children omitted from output).
///
/// # Returns
///
/// `HtmlToMarkdownVisitResult` with Skip action.
///
/// # Example
///
/// ```c
/// HtmlToMarkdownVisitResult result = html_to_markdown_visit_result_skip();
/// return result;
/// ```
#[unsafe(no_mangle)]
pub const extern "C" fn html_to_markdown_visit_result_skip() -> HtmlToMarkdownVisitResult {
    HtmlToMarkdownVisitResult {
        result_type: HtmlToMarkdownVisitResultType::Skip,
        custom_output: ptr::null_mut(),
        error_message: ptr::null_mut(),
    }
}

/// Create a `VisitResult` with `PreserveHtml` action.
///
/// Helper function to construct a `PreserveHtml` result (raw HTML included verbatim).
///
/// # Returns
///
/// `HtmlToMarkdownVisitResult` with `PreserveHtml` action.
///
/// # Example
///
/// ```c
/// HtmlToMarkdownVisitResult result = html_to_markdown_visit_result_preserve_html();
/// return result;
/// ```
#[unsafe(no_mangle)]
pub const extern "C" fn html_to_markdown_visit_result_preserve_html() -> HtmlToMarkdownVisitResult {
    HtmlToMarkdownVisitResult {
        result_type: HtmlToMarkdownVisitResultType::PreserveHtml,
        custom_output: ptr::null_mut(),
        error_message: ptr::null_mut(),
    }
}

/// Create a `VisitResult` with Error action.
///
/// Helper function to construct an Error result. The `message` string should be
/// allocated with `malloc()` and will be freed by the FFI layer after use.
///
/// # Arguments
///
/// - `message`: malloc'd null-terminated C string with error message
///
/// # Returns
///
/// `HtmlToMarkdownVisitResult` with Error action.
///
/// # Safety
///
/// - `message` must be malloc'd (will be freed with `free()`)
/// - `message` must be NULL-terminated
///
/// # Example
///
/// ```c
/// char *error_msg = (char *)malloc(100);
/// snprintf(error_msg, 100, "Unexpected element type");
/// HtmlToMarkdownVisitResult result = html_to_markdown_visit_result_error(error_msg);
/// return result;
/// ```
#[unsafe(no_mangle)]
pub const extern "C" fn html_to_markdown_visit_result_error(message: *mut c_char) -> HtmlToMarkdownVisitResult {
    HtmlToMarkdownVisitResult {
        result_type: HtmlToMarkdownVisitResultType::Error,
        custom_output: ptr::null_mut(),
        error_message: message,
    }
}
