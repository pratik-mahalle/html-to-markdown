//! Extended callback function types for the visitor pattern.
//!
//! This module defines additional callback types for less common HTML elements
//! and formatting, extending the core callbacks defined in the callbacks module.

use std::os::raw::c_char;

use super::types::{HtmlToMarkdownNodeContext, HtmlToMarkdownVisitResult};

/// Visitor callback function type for line breaks.
///
/// Called for `<br>` elements.
///
/// # Arguments
///
/// - `user_data`: Context pointer from `html_to_markdown_visitor_create()`
/// - `ctx`: Node context with line break metadata
///
/// # Returns
///
/// `HtmlToMarkdownVisitResult` indicating action.
pub type HtmlToMarkdownVisitLineBreakCallback = unsafe extern "C" fn(
    user_data: *mut std::ffi::c_void,
    ctx: *const HtmlToMarkdownNodeContext,
) -> HtmlToMarkdownVisitResult;

/// Visitor callback function type for horizontal rules.
///
/// Called for `<hr>` elements.
///
/// # Arguments
///
/// - `user_data`: Context pointer from `html_to_markdown_visitor_create()`
/// - `ctx`: Node context with horizontal rule metadata
///
/// # Returns
///
/// `HtmlToMarkdownVisitResult` indicating action.
pub type HtmlToMarkdownVisitHorizontalRuleCallback = unsafe extern "C" fn(
    user_data: *mut std::ffi::c_void,
    ctx: *const HtmlToMarkdownNodeContext,
) -> HtmlToMarkdownVisitResult;

/// Visitor callback function type for custom/unknown elements.
///
/// Called for custom elements (web components) or unknown tags.
///
/// # Arguments
///
/// - `user_data`: Context pointer from `html_to_markdown_visitor_create()`
/// - `ctx`: Node context with custom element metadata
/// - `tag_name`: The custom element's tag name (NULL-terminated)
/// - `html`: The raw HTML of this element (NULL-terminated)
///
/// # Returns
///
/// `HtmlToMarkdownVisitResult` indicating action.
pub type HtmlToMarkdownVisitCustomElementCallback = unsafe extern "C" fn(
    user_data: *mut std::ffi::c_void,
    ctx: *const HtmlToMarkdownNodeContext,
    tag_name: *const c_char,
    html: *const c_char,
) -> HtmlToMarkdownVisitResult;

/// Visitor callback function type for definition list start.
///
/// Called before processing a definition list `<dl>`.
///
/// # Arguments
///
/// - `user_data`: Context pointer from `html_to_markdown_visitor_create()`
/// - `ctx`: Node context with definition list metadata
///
/// # Returns
///
/// `HtmlToMarkdownVisitResult` indicating action.
pub type HtmlToMarkdownVisitDefinitionListStartCallback = unsafe extern "C" fn(
    user_data: *mut std::ffi::c_void,
    ctx: *const HtmlToMarkdownNodeContext,
) -> HtmlToMarkdownVisitResult;

/// Visitor callback function type for definition terms.
///
/// Called for definition term elements `<dt>`.
///
/// # Arguments
///
/// - `user_data`: Context pointer from `html_to_markdown_visitor_create()`
/// - `ctx`: Node context with definition term metadata
/// - `text`: The term text content (NULL-terminated)
///
/// # Returns
///
/// `HtmlToMarkdownVisitResult` indicating action.
pub type HtmlToMarkdownVisitDefinitionTermCallback = unsafe extern "C" fn(
    user_data: *mut std::ffi::c_void,
    ctx: *const HtmlToMarkdownNodeContext,
    text: *const c_char,
) -> HtmlToMarkdownVisitResult;

/// Visitor callback function type for definition descriptions.
///
/// Called for definition description elements `<dd>`.
///
/// # Arguments
///
/// - `user_data`: Context pointer from `html_to_markdown_visitor_create()`
/// - `ctx`: Node context with definition description metadata
/// - `text`: The description text content (NULL-terminated)
///
/// # Returns
///
/// `HtmlToMarkdownVisitResult` indicating action.
pub type HtmlToMarkdownVisitDefinitionDescriptionCallback = unsafe extern "C" fn(
    user_data: *mut std::ffi::c_void,
    ctx: *const HtmlToMarkdownNodeContext,
    text: *const c_char,
) -> HtmlToMarkdownVisitResult;

/// Visitor callback function type for definition list end.
///
/// Called after processing a definition list `</dl>`.
///
/// # Arguments
///
/// - `user_data`: Context pointer from `html_to_markdown_visitor_create()`
/// - `ctx`: Node context with definition list metadata
/// - `output`: The default markdown output for the definition list (NULL-terminated)
///
/// # Returns
///
/// `HtmlToMarkdownVisitResult` indicating action.
pub type HtmlToMarkdownVisitDefinitionListEndCallback = unsafe extern "C" fn(
    user_data: *mut std::ffi::c_void,
    ctx: *const HtmlToMarkdownNodeContext,
    output: *const c_char,
) -> HtmlToMarkdownVisitResult;

/// Visitor callback function type for form elements.
///
/// Called for `<form>` elements.
///
/// # Arguments
///
/// - `user_data`: Context pointer from `html_to_markdown_visitor_create()`
/// - `ctx`: Node context with form metadata
/// - `action`: Form action attribute (NULL-terminated, or NULL if not present)
/// - `method`: Form method attribute (NULL-terminated, or NULL if not present)
///
/// # Returns
///
/// `HtmlToMarkdownVisitResult` indicating action.
pub type HtmlToMarkdownVisitFormCallback = unsafe extern "C" fn(
    user_data: *mut std::ffi::c_void,
    ctx: *const HtmlToMarkdownNodeContext,
    action: *const c_char,
    method: *const c_char,
) -> HtmlToMarkdownVisitResult;

/// Visitor callback function type for input elements.
///
/// Called for `<input>` elements.
///
/// # Arguments
///
/// - `user_data`: Context pointer from `html_to_markdown_visitor_create()`
/// - `ctx`: Node context with input metadata
/// - `input_type`: Input type attribute (NULL-terminated)
/// - `name`: Name attribute (NULL-terminated, or NULL if not present)
/// - `value`: Value attribute (NULL-terminated, or NULL if not present)
///
/// # Returns
///
/// `HtmlToMarkdownVisitResult` indicating action.
pub type HtmlToMarkdownVisitInputCallback = unsafe extern "C" fn(
    user_data: *mut std::ffi::c_void,
    ctx: *const HtmlToMarkdownNodeContext,
    input_type: *const c_char,
    name: *const c_char,
    value: *const c_char,
) -> HtmlToMarkdownVisitResult;

/// Visitor callback function type for button elements.
///
/// Called for `<button>` elements.
///
/// # Arguments
///
/// - `user_data`: Context pointer from `html_to_markdown_visitor_create()`
/// - `ctx`: Node context with button metadata
/// - `text`: The button text content (NULL-terminated)
///
/// # Returns
///
/// `HtmlToMarkdownVisitResult` indicating action.
pub type HtmlToMarkdownVisitButtonCallback = unsafe extern "C" fn(
    user_data: *mut std::ffi::c_void,
    ctx: *const HtmlToMarkdownNodeContext,
    text: *const c_char,
) -> HtmlToMarkdownVisitResult;

/// Visitor callback function type for audio elements.
///
/// Called for `<audio>` elements.
///
/// # Arguments
///
/// - `user_data`: Context pointer from `html_to_markdown_visitor_create()`
/// - `ctx`: Node context with audio metadata
/// - `src`: Source URL (NULL-terminated, or NULL if not present)
///
/// # Returns
///
/// `HtmlToMarkdownVisitResult` indicating action.
pub type HtmlToMarkdownVisitAudioCallback = unsafe extern "C" fn(
    user_data: *mut std::ffi::c_void,
    ctx: *const HtmlToMarkdownNodeContext,
    src: *const c_char,
) -> HtmlToMarkdownVisitResult;

/// Visitor callback function type for video elements.
///
/// Called for `<video>` elements.
///
/// # Arguments
///
/// - `user_data`: Context pointer from `html_to_markdown_visitor_create()`
/// - `ctx`: Node context with video metadata
/// - `src`: Source URL (NULL-terminated, or NULL if not present)
///
/// # Returns
///
/// `HtmlToMarkdownVisitResult` indicating action.
pub type HtmlToMarkdownVisitVideoCallback = unsafe extern "C" fn(
    user_data: *mut std::ffi::c_void,
    ctx: *const HtmlToMarkdownNodeContext,
    src: *const c_char,
) -> HtmlToMarkdownVisitResult;

/// Visitor callback function type for iframe elements.
///
/// Called for `<iframe>` elements.
///
/// # Arguments
///
/// - `user_data`: Context pointer from `html_to_markdown_visitor_create()`
/// - `ctx`: Node context with iframe metadata
/// - `src`: Source URL (NULL-terminated, or NULL if not present)
///
/// # Returns
///
/// `HtmlToMarkdownVisitResult` indicating action.
pub type HtmlToMarkdownVisitIframeCallback = unsafe extern "C" fn(
    user_data: *mut std::ffi::c_void,
    ctx: *const HtmlToMarkdownNodeContext,
    src: *const c_char,
) -> HtmlToMarkdownVisitResult;

/// Visitor callback function type for details elements.
///
/// Called for `<details>` elements.
///
/// # Arguments
///
/// - `user_data`: Context pointer from `html_to_markdown_visitor_create()`
/// - `ctx`: Node context with details metadata
/// - `open`: Whether the details element is open (true) or closed (false)
///
/// # Returns
///
/// `HtmlToMarkdownVisitResult` indicating action.
pub type HtmlToMarkdownVisitDetailsCallback = unsafe extern "C" fn(
    user_data: *mut std::ffi::c_void,
    ctx: *const HtmlToMarkdownNodeContext,
    open: bool,
) -> HtmlToMarkdownVisitResult;

/// Visitor callback function type for summary elements.
///
/// Called for `<summary>` elements.
///
/// # Arguments
///
/// - `user_data`: Context pointer from `html_to_markdown_visitor_create()`
/// - `ctx`: Node context with summary metadata
/// - `text`: The summary text content (NULL-terminated)
///
/// # Returns
///
/// `HtmlToMarkdownVisitResult` indicating action.
pub type HtmlToMarkdownVisitSummaryCallback = unsafe extern "C" fn(
    user_data: *mut std::ffi::c_void,
    ctx: *const HtmlToMarkdownNodeContext,
    text: *const c_char,
) -> HtmlToMarkdownVisitResult;

/// Visitor callback function type for figure start.
///
/// Called before processing a figure element `<figure>`.
///
/// # Arguments
///
/// - `user_data`: Context pointer from `html_to_markdown_visitor_create()`
/// - `ctx`: Node context with figure metadata
///
/// # Returns
///
/// `HtmlToMarkdownVisitResult` indicating action.
pub type HtmlToMarkdownVisitFigureStartCallback = unsafe extern "C" fn(
    user_data: *mut std::ffi::c_void,
    ctx: *const HtmlToMarkdownNodeContext,
) -> HtmlToMarkdownVisitResult;

/// Visitor callback function type for figcaption elements.
///
/// Called for `<figcaption>` elements.
///
/// # Arguments
///
/// - `user_data`: Context pointer from `html_to_markdown_visitor_create()`
/// - `ctx`: Node context with figcaption metadata
/// - `text`: The caption text content (NULL-terminated)
///
/// # Returns
///
/// `HtmlToMarkdownVisitResult` indicating action.
pub type HtmlToMarkdownVisitFigcaptionCallback = unsafe extern "C" fn(
    user_data: *mut std::ffi::c_void,
    ctx: *const HtmlToMarkdownNodeContext,
    text: *const c_char,
) -> HtmlToMarkdownVisitResult;

/// Visitor callback function type for figure end.
///
/// Called after processing a figure element `</figure>`.
///
/// # Arguments
///
/// - `user_data`: Context pointer from `html_to_markdown_visitor_create()`
/// - `ctx`: Node context with figure metadata
/// - `output`: The default markdown output for the figure (NULL-terminated)
///
/// # Returns
///
/// `HtmlToMarkdownVisitResult` indicating action.
pub type HtmlToMarkdownVisitFigureEndCallback = unsafe extern "C" fn(
    user_data: *mut std::ffi::c_void,
    ctx: *const HtmlToMarkdownNodeContext,
    output: *const c_char,
) -> HtmlToMarkdownVisitResult;
