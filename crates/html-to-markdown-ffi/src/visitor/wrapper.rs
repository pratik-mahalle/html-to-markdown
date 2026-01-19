//! Visitor wrapper implementation for C FFI callbacks.
//!
//! This module contains the `CVisitorWrapper` struct and its core methods
//! for managing callback execution and memory safety.

use std::cell::RefCell;
use std::ffi::{CStr, CString};
use std::ptr;

use html_to_markdown_rs::visitor::{NodeContext, VisitResult};

use super::registry::HtmlToMarkdownVisitorCallbacks;
use super::types::{
    HtmlToMarkdownAttribute, HtmlToMarkdownNodeContext, HtmlToMarkdownVisitResult, HtmlToMarkdownVisitResultType,
};

/// Internal wrapper implementing `HtmlVisitor` trait from C callbacks.
///
/// Bridges the gap between:
/// - C callback functions (function pointers with void* `user_data`)
/// - Rust `HtmlVisitor` trait (method receivers with &mut self)
///
/// This wrapper is allocated in a Box and stored as an opaque pointer
/// in the `HtmlToMarkdownVisitor` handle.
///
/// # Design Notes
///
/// - Stores callback function pointers and `user_data` (owned by caller)
/// - Attributes and temporary strings are stored in wrapper, valid only during callback
/// - Callback string parameters are C string pointers (valid only during callback)
/// - Returned custom output/error strings are malloc'd; we take ownership and free them
pub struct CVisitorWrapper {
    /// C callback table with function pointers and `user_data`
    pub callbacks: HtmlToMarkdownVisitorCallbacks,
    /// Temporary storage for tag name `CString` (valid only during callback)
    pub temp_tag_name: RefCell<Option<CString>>,
    /// Temporary storage for parent tag `CString` (valid only during callback)
    pub temp_parent_tag: RefCell<Option<CString>>,
    /// Temporary storage for attribute array and key/value `CStrings`
    pub temp_attributes: RefCell<Vec<HtmlToMarkdownAttribute>>,
    /// Temporary storage for attribute key/value `CStrings` (kept alive during callback)
    pub temp_attribute_strings: RefCell<Vec<CString>>,
}

impl CVisitorWrapper {
    /// Create a new visitor wrapper from C callbacks.
    pub const fn new(callbacks: HtmlToMarkdownVisitorCallbacks) -> Self {
        Self {
            callbacks,
            temp_tag_name: RefCell::new(None),
            temp_parent_tag: RefCell::new(None),
            temp_attributes: RefCell::new(Vec::new()),
            temp_attribute_strings: RefCell::new(Vec::new()),
        }
    }

    /// Build a C `NodeContext` from a Rust `NodeContext`.
    ///
    /// Note: The returned context contains pointers to temporary strings stored in `RefCell` fields.
    /// These strings remain valid only during the callback execution. After the callback returns,
    /// the strings are cleared and the pointers become invalid.
    pub fn build_node_context(&self, ctx: &NodeContext) -> HtmlToMarkdownNodeContext {
        let c_tag_name = CString::new(ctx.tag_name.clone()).unwrap_or_else(|_| CString::new("").unwrap());
        let tag_name_ptr = c_tag_name.as_ptr();
        *self.temp_tag_name.borrow_mut() = Some(c_tag_name);

        let parent_tag_ptr = if let Some(parent) = &ctx.parent_tag {
            let c_parent = CString::new(parent.clone()).unwrap_or_else(|_| CString::new("").unwrap());
            let ptr = c_parent.as_ptr();
            *self.temp_parent_tag.borrow_mut() = Some(c_parent);
            ptr
        } else {
            ptr::null()
        };

        let mut attrs = Vec::new();
        let mut attr_strings = Vec::new();

        for (key, value) in &ctx.attributes {
            let c_key = CString::new(key.clone()).unwrap_or_default();
            let c_value = CString::new(value.clone()).unwrap_or_default();
            let key_ptr = c_key.as_ptr();
            let value_ptr = c_value.as_ptr();

            attrs.push(HtmlToMarkdownAttribute {
                key: key_ptr,
                value: value_ptr,
            });

            attr_strings.push(c_key);
            attr_strings.push(c_value);
        }

        attrs.push(HtmlToMarkdownAttribute {
            key: ptr::null(),
            value: ptr::null(),
        });

        *self.temp_attribute_strings.borrow_mut() = attr_strings;
        let attributes_ptr = if attrs.is_empty() {
            ptr::null()
        } else {
            *self.temp_attributes.borrow_mut() = attrs;
            self.temp_attributes.borrow().as_ptr()
        };

        HtmlToMarkdownNodeContext {
            node_type: ctx.node_type.into(),
            tag_name: tag_name_ptr,
            attributes: attributes_ptr,
            depth: ctx.depth,
            index_in_parent: ctx.index_in_parent,
            parent_tag: parent_tag_ptr,
            is_inline: ctx.is_inline,
            reserved: 0,
        }
    }

    /// Clear temporary strings after callback execution.
    ///
    /// This should be called after each callback to ensure memory is not leaked.
    pub fn clear_temp_strings(&self) {
        *self.temp_tag_name.borrow_mut() = None;
        *self.temp_parent_tag.borrow_mut() = None;
        *self.temp_attributes.borrow_mut() = Vec::new();
        *self.temp_attribute_strings.borrow_mut() = Vec::new();
    }

    /// Process a `VisitResult` from a C callback, handling memory cleanup.
    pub fn process_result(&self, result: HtmlToMarkdownVisitResult) -> VisitResult {
        match result.result_type {
            HtmlToMarkdownVisitResultType::Continue => VisitResult::Continue,
            HtmlToMarkdownVisitResultType::Custom => {
                if result.custom_output.is_null() {
                    VisitResult::Continue
                } else {
                    let output = unsafe { CStr::from_ptr(result.custom_output).to_string_lossy().into_owned() };
                    unsafe { libc::free(result.custom_output.cast::<std::ffi::c_void>()) };
                    VisitResult::Custom(output)
                }
            }
            HtmlToMarkdownVisitResultType::Skip => VisitResult::Skip,
            HtmlToMarkdownVisitResultType::PreserveHtml => VisitResult::PreserveHtml,
            HtmlToMarkdownVisitResultType::Error => {
                if result.error_message.is_null() {
                    VisitResult::Error("unknown error".to_string())
                } else {
                    let msg = unsafe { CStr::from_ptr(result.error_message).to_string_lossy().into_owned() };
                    unsafe { libc::free(result.error_message.cast::<std::ffi::c_void>()) };
                    VisitResult::Error(msg)
                }
            }
        }
    }
}

impl std::fmt::Debug for CVisitorWrapper {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CVisitorWrapper")
            .field("user_data", &(self.callbacks.user_data as usize))
            .finish()
    }
}
