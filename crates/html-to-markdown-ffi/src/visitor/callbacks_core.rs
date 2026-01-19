//! Callback function types for the visitor pattern.
//!
//! This module defines all callback function types that can be implemented
//! by FFI consumers. Each callback type corresponds to a different HTML element
//! or visitor event during the conversion process.

use std::os::raw::c_char;

use super::types::{HtmlToMarkdownNodeContext, HtmlToMarkdownVisitResult};

/// Visitor callback function type for text nodes.
///
/// Called for each text node in the HTML document.
/// This is the most frequently called callback (100+ times per document).
///
/// # Arguments
///
/// - `user_data`: Context pointer passed to `html_to_markdown_visitor_create()`
/// - `ctx`: Node context (valid only for callback duration)
/// - `text`: Text content (NULL-terminated, valid for callback duration)
///
/// # Returns
///
/// `HtmlToMarkdownVisitResult` with `result_type` and optional custom output.
/// If custom, allocate the output string with `malloc()`.
///
/// # Safety
///
/// - `ctx` is valid only during callback; don't store or dereference after
/// - `text` is valid only during callback; make a copy if needed
/// - Returned `custom_output` MUST be malloc'd; will be freed with `free()`
pub type HtmlToMarkdownVisitTextCallback = unsafe extern "C" fn(
    user_data: *mut std::ffi::c_void,
    ctx: *const HtmlToMarkdownNodeContext,
    text: *const c_char,
) -> HtmlToMarkdownVisitResult;

/// Visitor callback function type for start of elements.
///
/// Called before entering any HTML element (pre-order traversal).
/// Generic hook for all element types.
///
/// # Arguments
///
/// - `user_data`: Context pointer from `html_to_markdown_visitor_create()`
/// - `ctx`: Node context (valid only for callback duration)
///
/// # Returns
///
/// `HtmlToMarkdownVisitResult` indicating action to take.
pub type HtmlToMarkdownVisitElementStartCallback = unsafe extern "C" fn(
    user_data: *mut std::ffi::c_void,
    ctx: *const HtmlToMarkdownNodeContext,
) -> HtmlToMarkdownVisitResult;

/// Visitor callback function type for end of elements.
///
/// Called after exiting any HTML element (post-order traversal).
/// Receives the default markdown output for the element.
///
/// # Arguments
///
/// - `user_data`: Context pointer from `html_to_markdown_visitor_create()`
/// - `ctx`: Node context (valid only for callback duration)
/// - `output`: Default markdown output (NULL-terminated, valid for callback duration)
///
/// # Returns
///
/// `HtmlToMarkdownVisitResult` with optional custom replacement output.
pub type HtmlToMarkdownVisitElementEndCallback = unsafe extern "C" fn(
    user_data: *mut std::ffi::c_void,
    ctx: *const HtmlToMarkdownNodeContext,
    output: *const c_char,
) -> HtmlToMarkdownVisitResult;

/// Visitor callback function type for links.
///
/// Called for anchor links `<a href="...">`.
///
/// # Arguments
///
/// - `user_data`: Context pointer from `html_to_markdown_visitor_create()`
/// - `ctx`: Node context with link metadata
/// - `href`: Link URL (NULL-terminated, valid for callback duration)
/// - `text`: Link text (already converted to markdown, NULL-terminated)
/// - `title`: Title attribute, or NULL if not present
///
/// # Returns
///
/// `HtmlToMarkdownVisitResult` indicating action.
pub type HtmlToMarkdownVisitLinkCallback = unsafe extern "C" fn(
    user_data: *mut std::ffi::c_void,
    ctx: *const HtmlToMarkdownNodeContext,
    href: *const c_char,
    text: *const c_char,
    title: *const c_char,
) -> HtmlToMarkdownVisitResult;

/// Visitor callback function type for images.
///
/// Called for image elements `<img src="..." alt="...">`.
///
/// # Arguments
///
/// - `user_data`: Context pointer from `html_to_markdown_visitor_create()`
/// - `ctx`: Node context with image metadata
/// - `src`: Image source URL (NULL-terminated)
/// - `alt`: Alt text (NULL-terminated)
/// - `title`: Title attribute, or NULL if not present
///
/// # Returns
///
/// `HtmlToMarkdownVisitResult` indicating action.
pub type HtmlToMarkdownVisitImageCallback = unsafe extern "C" fn(
    user_data: *mut std::ffi::c_void,
    ctx: *const HtmlToMarkdownNodeContext,
    src: *const c_char,
    alt: *const c_char,
    title: *const c_char,
) -> HtmlToMarkdownVisitResult;

/// Visitor callback function type for headings.
///
/// Called for heading elements `<h1>` through `<h6>`.
///
/// # Arguments
///
/// - `user_data`: Context pointer from `html_to_markdown_visitor_create()`
/// - `ctx`: Node context with heading metadata
/// - `level`: Heading level (1-6)
/// - `text`: Heading text content (NULL-terminated)
/// - `id`: ID attribute, or NULL if not present
///
/// # Returns
///
/// `HtmlToMarkdownVisitResult` indicating action.
pub type HtmlToMarkdownVisitHeadingCallback = unsafe extern "C" fn(
    user_data: *mut std::ffi::c_void,
    ctx: *const HtmlToMarkdownNodeContext,
    level: u32,
    text: *const c_char,
    id: *const c_char,
) -> HtmlToMarkdownVisitResult;

/// Visitor callback function type for code blocks.
///
/// Called for code block elements `<pre><code>`.
///
/// # Arguments
///
/// - `user_data`: Context pointer from `html_to_markdown_visitor_create()`
/// - `ctx`: Node context with code block metadata
/// - `lang`: Optional language specifier (NULL-terminated, or NULL if not present)
/// - `code`: Code content (NULL-terminated)
///
/// # Returns
///
/// `HtmlToMarkdownVisitResult` indicating action.
pub type HtmlToMarkdownVisitCodeBlockCallback = unsafe extern "C" fn(
    user_data: *mut std::ffi::c_void,
    ctx: *const HtmlToMarkdownNodeContext,
    lang: *const c_char,
    code: *const c_char,
) -> HtmlToMarkdownVisitResult;

/// Visitor callback function type for inline code.
///
/// Called for inline code elements `<code>`.
///
/// # Arguments
///
/// - `user_data`: Context pointer from `html_to_markdown_visitor_create()`
/// - `ctx`: Node context with code metadata
/// - `code`: Code content (NULL-terminated)
///
/// # Returns
///
/// `HtmlToMarkdownVisitResult` indicating action.
pub type HtmlToMarkdownVisitCodeInlineCallback = unsafe extern "C" fn(
    user_data: *mut std::ffi::c_void,
    ctx: *const HtmlToMarkdownNodeContext,
    code: *const c_char,
) -> HtmlToMarkdownVisitResult;

/// Visitor callback function type for list start.
///
/// Called before processing a list element `<ul>` or `<ol>`.
///
/// # Arguments
///
/// - `user_data`: Context pointer from `html_to_markdown_visitor_create()`
/// - `ctx`: Node context with list metadata
/// - `ordered`: Whether this is an ordered list (true) or unordered (false)
///
/// # Returns
///
/// `HtmlToMarkdownVisitResult` indicating action.
pub type HtmlToMarkdownVisitListStartCallback = unsafe extern "C" fn(
    user_data: *mut std::ffi::c_void,
    ctx: *const HtmlToMarkdownNodeContext,
    ordered: bool,
) -> HtmlToMarkdownVisitResult;

/// Visitor callback function type for list items.
///
/// Called for list item elements `<li>`.
///
/// # Arguments
///
/// - `user_data`: Context pointer from `html_to_markdown_visitor_create()`
/// - `ctx`: Node context with list item metadata
/// - `ordered`: Whether this is in an ordered list
/// - `marker`: The list marker string (e.g., "-", "1.", "a)") (NULL-terminated)
/// - `text`: The list item content (NULL-terminated)
///
/// # Returns
///
/// `HtmlToMarkdownVisitResult` indicating action.
pub type HtmlToMarkdownVisitListItemCallback = unsafe extern "C" fn(
    user_data: *mut std::ffi::c_void,
    ctx: *const HtmlToMarkdownNodeContext,
    ordered: bool,
    marker: *const c_char,
    text: *const c_char,
) -> HtmlToMarkdownVisitResult;

/// Visitor callback function type for list end.
///
/// Called after processing a list element `</ul>` or `</ol>`.
///
/// # Arguments
///
/// - `user_data`: Context pointer from `html_to_markdown_visitor_create()`
/// - `ctx`: Node context with list metadata
/// - `ordered`: Whether this is an ordered list
/// - `output`: The default markdown output for the list (NULL-terminated)
///
/// # Returns
///
/// `HtmlToMarkdownVisitResult` indicating action.
pub type HtmlToMarkdownVisitListEndCallback = unsafe extern "C" fn(
    user_data: *mut std::ffi::c_void,
    ctx: *const HtmlToMarkdownNodeContext,
    ordered: bool,
    output: *const c_char,
) -> HtmlToMarkdownVisitResult;

/// Visitor callback function type for table start.
///
/// Called before processing a table element `<table>`.
///
/// # Arguments
///
/// - `user_data`: Context pointer from `html_to_markdown_visitor_create()`
/// - `ctx`: Node context with table metadata
///
/// # Returns
///
/// `HtmlToMarkdownVisitResult` indicating action.
pub type HtmlToMarkdownVisitTableStartCallback = unsafe extern "C" fn(
    user_data: *mut std::ffi::c_void,
    ctx: *const HtmlToMarkdownNodeContext,
) -> HtmlToMarkdownVisitResult;

/// Visitor callback function type for table rows.
///
/// Called for table row elements `<tr>`.
///
/// # Arguments
///
/// - `user_data`: Context pointer from `html_to_markdown_visitor_create()`
/// - `ctx`: Node context with row metadata
/// - `cells`: Array of cell contents (NULL-terminated array of NULL-terminated strings)
/// - `cell_count`: Number of cells in the row
/// - `is_header`: Whether this row is in a header section (true) or body (false)
///
/// # Returns
///
/// `HtmlToMarkdownVisitResult` indicating action.
pub type HtmlToMarkdownVisitTableRowCallback = unsafe extern "C" fn(
    user_data: *mut std::ffi::c_void,
    ctx: *const HtmlToMarkdownNodeContext,
    cells: *const *const c_char,
    cell_count: usize,
    is_header: bool,
) -> HtmlToMarkdownVisitResult;

/// Visitor callback function type for table end.
///
/// Called after processing a table element `</table>`.
///
/// # Arguments
///
/// - `user_data`: Context pointer from `html_to_markdown_visitor_create()`
/// - `ctx`: Node context with table metadata
/// - `output`: The default markdown output for the table (NULL-terminated)
///
/// # Returns
///
/// `HtmlToMarkdownVisitResult` indicating action.
pub type HtmlToMarkdownVisitTableEndCallback = unsafe extern "C" fn(
    user_data: *mut std::ffi::c_void,
    ctx: *const HtmlToMarkdownNodeContext,
    output: *const c_char,
) -> HtmlToMarkdownVisitResult;

/// Visitor callback function type for blockquotes.
///
/// Called for blockquote elements `<blockquote>`.
///
/// # Arguments
///
/// - `user_data`: Context pointer from `html_to_markdown_visitor_create()`
/// - `ctx`: Node context with blockquote metadata
/// - `content`: The blockquote content (NULL-terminated)
/// - `depth`: Nesting depth for nested blockquotes
///
/// # Returns
///
/// `HtmlToMarkdownVisitResult` indicating action.
pub type HtmlToMarkdownVisitBlockquoteCallback = unsafe extern "C" fn(
    user_data: *mut std::ffi::c_void,
    ctx: *const HtmlToMarkdownNodeContext,
    content: *const c_char,
    depth: usize,
) -> HtmlToMarkdownVisitResult;

/// Visitor callback function type for strong/bold elements.
///
/// Called for `<strong>` and `<b>` elements.
///
/// # Arguments
///
/// - `user_data`: Context pointer from `html_to_markdown_visitor_create()`
/// - `ctx`: Node context with strong element metadata
/// - `text`: The text content (NULL-terminated)
///
/// # Returns
///
/// `HtmlToMarkdownVisitResult` indicating action.
pub type HtmlToMarkdownVisitStrongCallback = unsafe extern "C" fn(
    user_data: *mut std::ffi::c_void,
    ctx: *const HtmlToMarkdownNodeContext,
    text: *const c_char,
) -> HtmlToMarkdownVisitResult;

/// Visitor callback function type for emphasis/italic elements.
///
/// Called for `<em>` and `<i>` elements.
///
/// # Arguments
///
/// - `user_data`: Context pointer from `html_to_markdown_visitor_create()`
/// - `ctx`: Node context with emphasis element metadata
/// - `text`: The text content (NULL-terminated)
///
/// # Returns
///
/// `HtmlToMarkdownVisitResult` indicating action.
pub type HtmlToMarkdownVisitEmphasisCallback = unsafe extern "C" fn(
    user_data: *mut std::ffi::c_void,
    ctx: *const HtmlToMarkdownNodeContext,
    text: *const c_char,
) -> HtmlToMarkdownVisitResult;

/// Visitor callback function type for strikethrough elements.
///
/// Called for `<s>`, `<del>`, and `<strike>` elements.
///
/// # Arguments
///
/// - `user_data`: Context pointer from `html_to_markdown_visitor_create()`
/// - `ctx`: Node context with strikethrough element metadata
/// - `text`: The text content (NULL-terminated)
///
/// # Returns
///
/// `HtmlToMarkdownVisitResult` indicating action.
pub type HtmlToMarkdownVisitStrikethroughCallback = unsafe extern "C" fn(
    user_data: *mut std::ffi::c_void,
    ctx: *const HtmlToMarkdownNodeContext,
    text: *const c_char,
) -> HtmlToMarkdownVisitResult;

/// Visitor callback function type for underline elements.
///
/// Called for `<u>` and `<ins>` elements.
///
/// # Arguments
///
/// - `user_data`: Context pointer from `html_to_markdown_visitor_create()`
/// - `ctx`: Node context with underline element metadata
/// - `text`: The text content (NULL-terminated)
///
/// # Returns
///
/// `HtmlToMarkdownVisitResult` indicating action.
pub type HtmlToMarkdownVisitUnderlineCallback = unsafe extern "C" fn(
    user_data: *mut std::ffi::c_void,
    ctx: *const HtmlToMarkdownNodeContext,
    text: *const c_char,
) -> HtmlToMarkdownVisitResult;

/// Visitor callback function type for subscript elements.
///
/// Called for `<sub>` elements.
///
/// # Arguments
///
/// - `user_data`: Context pointer from `html_to_markdown_visitor_create()`
/// - `ctx`: Node context with subscript element metadata
/// - `text`: The text content (NULL-terminated)
///
/// # Returns
///
/// `HtmlToMarkdownVisitResult` indicating action.
pub type HtmlToMarkdownVisitSubscriptCallback = unsafe extern "C" fn(
    user_data: *mut std::ffi::c_void,
    ctx: *const HtmlToMarkdownNodeContext,
    text: *const c_char,
) -> HtmlToMarkdownVisitResult;

/// Visitor callback function type for superscript elements.
///
/// Called for `<sup>` elements.
///
/// # Arguments
///
/// - `user_data`: Context pointer from `html_to_markdown_visitor_create()`
/// - `ctx`: Node context with superscript element metadata
/// - `text`: The text content (NULL-terminated)
///
/// # Returns
///
/// `HtmlToMarkdownVisitResult` indicating action.
pub type HtmlToMarkdownVisitSuperscriptCallback = unsafe extern "C" fn(
    user_data: *mut std::ffi::c_void,
    ctx: *const HtmlToMarkdownNodeContext,
    text: *const c_char,
) -> HtmlToMarkdownVisitResult;

/// Visitor callback function type for mark/highlight elements.
///
/// Called for `<mark>` elements.
///
/// # Arguments
///
/// - `user_data`: Context pointer from `html_to_markdown_visitor_create()`
/// - `ctx`: Node context with mark element metadata
/// - `text`: The text content (NULL-terminated)
///
/// # Returns
///
/// `HtmlToMarkdownVisitResult` indicating action.
pub type HtmlToMarkdownVisitMarkCallback = unsafe extern "C" fn(
    user_data: *mut std::ffi::c_void,
    ctx: *const HtmlToMarkdownNodeContext,
    text: *const c_char,
) -> HtmlToMarkdownVisitResult;
