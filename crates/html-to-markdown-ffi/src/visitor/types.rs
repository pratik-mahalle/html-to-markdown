//! Core types and structures for the visitor pattern FFI.
//!
//! This module contains all the C-compatible types used by the visitor pattern,
//! including result types, node types, attributes, and node context structures.

use std::os::raw::c_char;
use std::ptr;

use html_to_markdown_rs::visitor::NodeType;

/// underlying `CVisitorWrapper`.
pub type HtmlToMarkdownVisitor = *mut std::ffi::c_void;

/// Result type enumeration for visitor callbacks.
///
/// Indicates what action the conversion engine should take after a callback.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HtmlToMarkdownVisitResultType {
    /// Continue with default conversion behavior
    Continue = 0,
    /// Replace default output with custom markdown (requires `custom_output` set)
    Custom = 1,
    /// Skip this element entirely
    Skip = 2,
    /// Preserve original HTML verbatim
    PreserveHtml = 3,
    /// Stop conversion with error (requires `error_message` set)
    Error = 4,
}

/// Result of a visitor callback (C-compatible).
///
/// Returned by all visitor callback functions. Only certain fields are meaningful
/// depending on `result_type`:
///
/// - **Continue**: No additional fields needed
/// - **Custom**: `custom_output` must point to a malloc'd string (NULL-terminated)
/// - **Skip**: No additional fields needed
/// - **`PreserveHtml`**: No additional fields needed
/// - **Error**: `error_message` must point to a malloc'd string (NULL-terminated)
///
/// # Memory Ownership
///
/// - `custom_output` and `error_message` are OWNED BY THE CALLBACK
/// - The FFI layer will free these strings after processing
/// - MUST be allocated with malloc/calloc; will be freed with `free()`
///
/// # Safety
///
/// The conversion engine guarantees:
/// - This struct is only valid during the callback and for the return statement
/// - Allocated strings are freed immediately after callback returns
/// - No references to callback data persist after return
#[repr(C)]
#[derive(Debug, Clone)]
pub struct HtmlToMarkdownVisitResult {
    /// The action to take (Continue, Custom, Skip, etc.)
    pub result_type: HtmlToMarkdownVisitResultType,

    /// Custom markdown output (only if `result_type` == Custom)
    /// Must be malloc'd NULL-terminated string; ownership transfers to FFI layer
    pub custom_output: *mut c_char,

    /// Error message (only if `result_type` == Error)
    /// Must be malloc'd NULL-terminated string; ownership transfers to FFI layer
    pub error_message: *mut c_char,
}

impl Default for HtmlToMarkdownVisitResult {
    fn default() -> Self {
        Self {
            result_type: HtmlToMarkdownVisitResultType::Continue,
            custom_output: ptr::null_mut(),
            error_message: ptr::null_mut(),
        }
    }
}

/// Node type enumeration (mirrors Rust `NodeType`).
///
/// All HTML element types recognized by the converter are represented here.
/// Used in `NodeContext` to classify elements during visitor callbacks.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HtmlToMarkdownNodeType {
    /// Text node (character data).
    Text = 0,
    /// Generic element node.
    Element = 1,
    /// Heading element (h1-h6).
    Heading = 2,
    /// Paragraph element.
    Paragraph = 3,
    /// Division element.
    Div = 4,
    /// Blockquote element.
    Blockquote = 5,
    /// Preformatted text element.
    Pre = 6,
    /// Horizontal rule element.
    Hr = 7,
    /// Unordered or ordered list element.
    List = 8,
    /// List item element.
    ListItem = 9,
    /// Table element.
    Table = 10,
    /// Table row element.
    TableRow = 11,
    /// Table cell element.
    TableCell = 12,
    /// Anchor/hyperlink element.
    Link = 13,
    /// Image element.
    Image = 14,
    /// Inline code element.
    Code = 15,
    /// Strong/bold element.
    Strong = 16,
    /// Emphasis/italic element.
    Em = 17,
    /// Custom or unknown element type.
    Custom = 255,
}

impl From<NodeType> for HtmlToMarkdownNodeType {
    fn from(nt: NodeType) -> Self {
        use HtmlToMarkdownNodeType::{
            Blockquote, Code, Custom, Div, Element, Em, Heading, Hr, Image, Link, List, ListItem, Paragraph, Pre,
            Strong, Table, TableCell, TableRow, Text,
        };
        match nt {
            NodeType::Text => Text,
            NodeType::Element => Element,
            NodeType::Heading => Heading,
            NodeType::Paragraph => Paragraph,
            NodeType::Div => Div,
            NodeType::Blockquote => Blockquote,
            NodeType::Pre => Pre,
            NodeType::Hr => Hr,
            NodeType::List => List,
            NodeType::ListItem => ListItem,
            NodeType::Table => Table,
            NodeType::TableRow => TableRow,
            NodeType::TableCell => TableCell,
            NodeType::Link => Link,
            NodeType::Image => Image,
            NodeType::Code => Code,
            NodeType::Strong => Strong,
            NodeType::Em => Em,
            _ => Custom,
        }
    }
}

/// Key-value attribute pair (C-compatible).
///
/// Used in attribute arrays passed to callbacks. Both key and value
/// are pointers to NULL-terminated C strings (valid for callback duration only).
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct HtmlToMarkdownAttribute {
    /// Attribute name (e.g., "href", "class", "id")
    pub key: *const c_char,
    /// Attribute value (e.g., "<https://example.com>", "container", "header-1")
    pub value: *const c_char,
}

/// Node context for visitor callbacks (C-compatible).
///
/// Provides comprehensive metadata about the current HTML node being visited.
/// All string pointers are valid only for the duration of the callback.
///
/// # Memory Safety
///
/// This struct is allocated by the conversion engine and remains valid only
/// during the callback. DO NOT store or dereference pointers after return.
///
/// # String Ownership
///
/// All string fields (`tag_name`, `parent_tag`, attribute keys/values)
/// are owned by the conversion engine and freed after callback returns.
/// Make copies if you need to preserve them.
#[repr(C)]
#[derive(Debug, Clone)]
pub struct HtmlToMarkdownNodeContext {
    /// Node type classification (coarse-grained)
    pub node_type: HtmlToMarkdownNodeType,

    /// Raw HTML tag name (e.g., "div", "h1", "custom-element")
    /// NULL-terminated C string
    pub tag_name: *const c_char,

    /// HTML attributes array, NULL-terminated
    /// Access: attributes[0], attributes[1], ..., until key == NULL
    pub attributes: *const HtmlToMarkdownAttribute,

    /// Depth in DOM tree (0 = root)
    pub depth: usize,

    /// Index among siblings (0-based)
    pub index_in_parent: usize,

    /// Parent element's tag name, or NULL if root
    pub parent_tag: *const c_char,

    /// Whether element is inline vs block (true = inline)
    pub is_inline: bool,

    /// RESERVED for future expansion; set to 0
    pub reserved: usize,
}
