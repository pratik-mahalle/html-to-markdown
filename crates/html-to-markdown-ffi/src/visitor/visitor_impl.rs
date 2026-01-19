//! Visitor trait implementation for C FFI callbacks.
//!
//! This module implements the complete `HtmlVisitor` trait for `CVisitorWrapper`,
//! handling all HTML element types and visitor events.

use std::os::raw::c_char;
use std::ptr;

use html_to_markdown_rs::visitor::{HtmlVisitor, NodeContext, VisitResult};

use super::wrapper::CVisitorWrapper;

impl HtmlVisitor for CVisitorWrapper {
    fn visit_text(&mut self, ctx: &NodeContext, text: &str) -> VisitResult {
        if let Some(callback) = self.callbacks.visit_text {
            let c_text_string = std::ffi::CString::new(text).unwrap_or_else(|_| std::ffi::CString::new("").unwrap());
            let c_text = c_text_string.as_ptr();

            let c_ctx = self.build_node_context(ctx);

            let result = unsafe { callback(self.callbacks.user_data, &raw const c_ctx, c_text) };
            let visit_result = self.process_result(result);
            self.clear_temp_strings();
            visit_result
        } else {
            VisitResult::Continue
        }
    }

    fn visit_element_start(&mut self, ctx: &NodeContext) -> VisitResult {
        if let Some(callback) = self.callbacks.visit_element_start {
            let c_ctx = self.build_node_context(ctx);
            let result = unsafe { callback(self.callbacks.user_data, &raw const c_ctx) };
            let visit_result = self.process_result(result);
            self.clear_temp_strings();
            visit_result
        } else {
            VisitResult::Continue
        }
    }

    fn visit_element_end(&mut self, ctx: &NodeContext, output: &str) -> VisitResult {
        if let Some(callback) = self.callbacks.visit_element_end {
            let c_output_string =
                std::ffi::CString::new(output).unwrap_or_else(|_| std::ffi::CString::new("").unwrap());
            let c_output = c_output_string.as_ptr();
            let c_ctx = self.build_node_context(ctx);
            let result = unsafe { callback(self.callbacks.user_data, &raw const c_ctx, c_output) };
            let visit_result = self.process_result(result);
            self.clear_temp_strings();
            visit_result
        } else {
            VisitResult::Continue
        }
    }

    fn visit_link(&mut self, ctx: &NodeContext, href: &str, text: &str, title: Option<&str>) -> VisitResult {
        if let Some(callback) = self.callbacks.visit_link {
            let c_href_string = std::ffi::CString::new(href).unwrap_or_else(|_| std::ffi::CString::new("").unwrap());
            let c_text_string = std::ffi::CString::new(text).unwrap_or_else(|_| std::ffi::CString::new("").unwrap());
            let c_title_string = title.and_then(|t| std::ffi::CString::new(t).ok());

            let c_href = c_href_string.as_ptr();
            let c_text = c_text_string.as_ptr();
            let c_title = c_title_string.as_ref().map_or(ptr::null(), |s| s.as_ptr());

            let c_ctx = self.build_node_context(ctx);
            let result = unsafe { callback(self.callbacks.user_data, &raw const c_ctx, c_href, c_text, c_title) };
            let visit_result = self.process_result(result);
            self.clear_temp_strings();
            visit_result
        } else {
            VisitResult::Continue
        }
    }

    fn visit_image(&mut self, ctx: &NodeContext, src: &str, alt: &str, title: Option<&str>) -> VisitResult {
        if let Some(callback) = self.callbacks.visit_image {
            let c_src_string = std::ffi::CString::new(src).unwrap_or_else(|_| std::ffi::CString::new("").unwrap());
            let c_alt_string = std::ffi::CString::new(alt).unwrap_or_else(|_| std::ffi::CString::new("").unwrap());
            let c_title_string = title.and_then(|t| std::ffi::CString::new(t).ok());

            let c_src = c_src_string.as_ptr();
            let c_alt = c_alt_string.as_ptr();
            let c_title = c_title_string.as_ref().map_or(ptr::null(), |s| s.as_ptr());

            let c_ctx = self.build_node_context(ctx);
            let result = unsafe { callback(self.callbacks.user_data, &raw const c_ctx, c_src, c_alt, c_title) };
            let visit_result = self.process_result(result);
            self.clear_temp_strings();
            visit_result
        } else {
            VisitResult::Continue
        }
    }

    fn visit_heading(&mut self, ctx: &NodeContext, level: u32, text: &str, id: Option<&str>) -> VisitResult {
        if let Some(callback) = self.callbacks.visit_heading {
            let c_text_string = std::ffi::CString::new(text).unwrap_or_else(|_| std::ffi::CString::new("").unwrap());
            let c_id_string = id.and_then(|i| std::ffi::CString::new(i).ok());

            let c_text = c_text_string.as_ptr();
            let c_id = c_id_string.as_ref().map_or(ptr::null(), |s| s.as_ptr());

            let c_ctx = self.build_node_context(ctx);
            let result = unsafe { callback(self.callbacks.user_data, &raw const c_ctx, level, c_text, c_id) };
            let visit_result = self.process_result(result);
            self.clear_temp_strings();
            visit_result
        } else {
            VisitResult::Continue
        }
    }

    fn visit_code_block(&mut self, ctx: &NodeContext, lang: Option<&str>, code: &str) -> VisitResult {
        if let Some(callback) = self.callbacks.visit_code_block {
            let c_lang_string = lang.and_then(|l| std::ffi::CString::new(l).ok());
            let c_code_string = std::ffi::CString::new(code).unwrap_or_else(|_| std::ffi::CString::new("").unwrap());

            let c_lang = c_lang_string.as_ref().map_or(ptr::null(), |s| s.as_ptr());
            let c_code = c_code_string.as_ptr();

            let c_ctx = self.build_node_context(ctx);
            let result = unsafe { callback(self.callbacks.user_data, &raw const c_ctx, c_lang, c_code) };
            let visit_result = self.process_result(result);
            self.clear_temp_strings();
            visit_result
        } else {
            VisitResult::Continue
        }
    }

    fn visit_code_inline(&mut self, ctx: &NodeContext, code: &str) -> VisitResult {
        if let Some(callback) = self.callbacks.visit_code_inline {
            let c_code_string = std::ffi::CString::new(code).unwrap_or_else(|_| std::ffi::CString::new("").unwrap());
            let c_code = c_code_string.as_ptr();

            let c_ctx = self.build_node_context(ctx);
            let result = unsafe { callback(self.callbacks.user_data, &raw const c_ctx, c_code) };
            let visit_result = self.process_result(result);
            self.clear_temp_strings();
            visit_result
        } else {
            VisitResult::Continue
        }
    }

    fn visit_list_item(&mut self, ctx: &NodeContext, ordered: bool, marker: &str, text: &str) -> VisitResult {
        if let Some(callback) = self.callbacks.visit_list_item {
            let c_marker_string =
                std::ffi::CString::new(marker).unwrap_or_else(|_| std::ffi::CString::new("").unwrap());
            let c_text_string = std::ffi::CString::new(text).unwrap_or_else(|_| std::ffi::CString::new("").unwrap());

            let c_marker = c_marker_string.as_ptr();
            let c_text = c_text_string.as_ptr();

            let c_ctx = self.build_node_context(ctx);
            let result = unsafe { callback(self.callbacks.user_data, &raw const c_ctx, ordered, c_marker, c_text) };
            let visit_result = self.process_result(result);
            self.clear_temp_strings();
            visit_result
        } else {
            VisitResult::Continue
        }
    }

    fn visit_list_start(&mut self, ctx: &NodeContext, ordered: bool) -> VisitResult {
        if let Some(callback) = self.callbacks.visit_list_start {
            let c_ctx = self.build_node_context(ctx);
            let result = unsafe { callback(self.callbacks.user_data, &raw const c_ctx, ordered) };
            let visit_result = self.process_result(result);
            self.clear_temp_strings();
            visit_result
        } else {
            VisitResult::Continue
        }
    }

    fn visit_list_end(&mut self, ctx: &NodeContext, ordered: bool, output: &str) -> VisitResult {
        if let Some(callback) = self.callbacks.visit_list_end {
            let c_output_string =
                std::ffi::CString::new(output).unwrap_or_else(|_| std::ffi::CString::new("").unwrap());
            let c_output = c_output_string.as_ptr();

            let c_ctx = self.build_node_context(ctx);
            let result = unsafe { callback(self.callbacks.user_data, &raw const c_ctx, ordered, c_output) };
            let visit_result = self.process_result(result);
            self.clear_temp_strings();
            visit_result
        } else {
            VisitResult::Continue
        }
    }

    fn visit_table_start(&mut self, ctx: &NodeContext) -> VisitResult {
        if let Some(callback) = self.callbacks.visit_table_start {
            let c_ctx = self.build_node_context(ctx);
            let result = unsafe { callback(self.callbacks.user_data, &raw const c_ctx) };
            let visit_result = self.process_result(result);
            self.clear_temp_strings();
            visit_result
        } else {
            VisitResult::Continue
        }
    }

    fn visit_table_row(&mut self, ctx: &NodeContext, cells: &[String], is_header: bool) -> VisitResult {
        if let Some(callback) = self.callbacks.visit_table_row {
            let c_cells: Vec<std::ffi::CString> = cells
                .iter()
                .map(|cell| {
                    std::ffi::CString::new(cell.clone()).unwrap_or_else(|_| std::ffi::CString::new("").unwrap())
                })
                .collect();

            let c_cell_ptrs: Vec<*const c_char> = c_cells.iter().map(|s| s.as_ptr()).collect();
            let c_cells_ptr = c_cell_ptrs.as_ptr();
            let cells_len = c_cell_ptrs.len();

            let c_ctx = self.build_node_context(ctx);
            let result = unsafe {
                callback(
                    self.callbacks.user_data,
                    &raw const c_ctx,
                    c_cells_ptr,
                    cells_len,
                    is_header,
                )
            };
            let visit_result = self.process_result(result);
            self.clear_temp_strings();
            visit_result
        } else {
            VisitResult::Continue
        }
    }

    fn visit_table_end(&mut self, ctx: &NodeContext, output: &str) -> VisitResult {
        if let Some(callback) = self.callbacks.visit_table_end {
            let c_output_string =
                std::ffi::CString::new(output).unwrap_or_else(|_| std::ffi::CString::new("").unwrap());
            let c_output = c_output_string.as_ptr();

            let c_ctx = self.build_node_context(ctx);
            let result = unsafe { callback(self.callbacks.user_data, &raw const c_ctx, c_output) };
            let visit_result = self.process_result(result);
            self.clear_temp_strings();
            visit_result
        } else {
            VisitResult::Continue
        }
    }

    fn visit_blockquote(&mut self, ctx: &NodeContext, content: &str, depth: usize) -> VisitResult {
        if let Some(callback) = self.callbacks.visit_blockquote {
            let c_content_string =
                std::ffi::CString::new(content).unwrap_or_else(|_| std::ffi::CString::new("").unwrap());
            let c_content = c_content_string.as_ptr();

            let c_ctx = self.build_node_context(ctx);
            let result = unsafe { callback(self.callbacks.user_data, &raw const c_ctx, c_content, depth) };
            let visit_result = self.process_result(result);
            self.clear_temp_strings();
            visit_result
        } else {
            VisitResult::Continue
        }
    }

    fn visit_strong(&mut self, ctx: &NodeContext, text: &str) -> VisitResult {
        if let Some(callback) = self.callbacks.visit_strong {
            let c_text_string = std::ffi::CString::new(text).unwrap_or_else(|_| std::ffi::CString::new("").unwrap());
            let c_text = c_text_string.as_ptr();

            let c_ctx = self.build_node_context(ctx);
            let result = unsafe { callback(self.callbacks.user_data, &raw const c_ctx, c_text) };
            let visit_result = self.process_result(result);
            self.clear_temp_strings();
            visit_result
        } else {
            VisitResult::Continue
        }
    }

    fn visit_emphasis(&mut self, ctx: &NodeContext, text: &str) -> VisitResult {
        if let Some(callback) = self.callbacks.visit_emphasis {
            let c_text_string = std::ffi::CString::new(text).unwrap_or_else(|_| std::ffi::CString::new("").unwrap());
            let c_text = c_text_string.as_ptr();

            let c_ctx = self.build_node_context(ctx);
            let result = unsafe { callback(self.callbacks.user_data, &raw const c_ctx, c_text) };
            let visit_result = self.process_result(result);
            self.clear_temp_strings();
            visit_result
        } else {
            VisitResult::Continue
        }
    }

    fn visit_strikethrough(&mut self, ctx: &NodeContext, text: &str) -> VisitResult {
        if let Some(callback) = self.callbacks.visit_strikethrough {
            let c_text_string = std::ffi::CString::new(text).unwrap_or_else(|_| std::ffi::CString::new("").unwrap());
            let c_text = c_text_string.as_ptr();

            let c_ctx = self.build_node_context(ctx);
            let result = unsafe { callback(self.callbacks.user_data, &raw const c_ctx, c_text) };
            let visit_result = self.process_result(result);
            self.clear_temp_strings();
            visit_result
        } else {
            VisitResult::Continue
        }
    }

    fn visit_underline(&mut self, ctx: &NodeContext, text: &str) -> VisitResult {
        if let Some(callback) = self.callbacks.visit_underline {
            let c_text_string = std::ffi::CString::new(text).unwrap_or_else(|_| std::ffi::CString::new("").unwrap());
            let c_text = c_text_string.as_ptr();

            let c_ctx = self.build_node_context(ctx);
            let result = unsafe { callback(self.callbacks.user_data, &raw const c_ctx, c_text) };
            let visit_result = self.process_result(result);
            self.clear_temp_strings();
            visit_result
        } else {
            VisitResult::Continue
        }
    }

    fn visit_subscript(&mut self, ctx: &NodeContext, text: &str) -> VisitResult {
        if let Some(callback) = self.callbacks.visit_subscript {
            let c_text_string = std::ffi::CString::new(text).unwrap_or_else(|_| std::ffi::CString::new("").unwrap());
            let c_text = c_text_string.as_ptr();

            let c_ctx = self.build_node_context(ctx);
            let result = unsafe { callback(self.callbacks.user_data, &raw const c_ctx, c_text) };
            let visit_result = self.process_result(result);
            self.clear_temp_strings();
            visit_result
        } else {
            VisitResult::Continue
        }
    }
    fn visit_superscript(&mut self, ctx: &NodeContext, text: &str) -> VisitResult {
        if let Some(callback) = self.callbacks.visit_superscript {
            let c_text_string = std::ffi::CString::new(text).unwrap_or_else(|_| std::ffi::CString::new("").unwrap());
            let c_text = c_text_string.as_ptr();

            let c_ctx = self.build_node_context(ctx);
            let result = unsafe { callback(self.callbacks.user_data, &raw const c_ctx, c_text) };
            let visit_result = self.process_result(result);
            self.clear_temp_strings();
            visit_result
        } else {
            VisitResult::Continue
        }
    }

    fn visit_mark(&mut self, ctx: &NodeContext, text: &str) -> VisitResult {
        if let Some(callback) = self.callbacks.visit_mark {
            let c_text_string = std::ffi::CString::new(text).unwrap_or_else(|_| std::ffi::CString::new("").unwrap());
            let c_text = c_text_string.as_ptr();

            let c_ctx = self.build_node_context(ctx);
            let result = unsafe { callback(self.callbacks.user_data, &raw const c_ctx, c_text) };
            let visit_result = self.process_result(result);
            self.clear_temp_strings();
            visit_result
        } else {
            VisitResult::Continue
        }
    }

    fn visit_line_break(&mut self, ctx: &NodeContext) -> VisitResult {
        if let Some(callback) = self.callbacks.visit_line_break {
            let c_ctx = self.build_node_context(ctx);
            let result = unsafe { callback(self.callbacks.user_data, &raw const c_ctx) };
            let visit_result = self.process_result(result);
            self.clear_temp_strings();
            visit_result
        } else {
            VisitResult::Continue
        }
    }

    fn visit_horizontal_rule(&mut self, ctx: &NodeContext) -> VisitResult {
        if let Some(callback) = self.callbacks.visit_horizontal_rule {
            let c_ctx = self.build_node_context(ctx);
            let result = unsafe { callback(self.callbacks.user_data, &raw const c_ctx) };
            let visit_result = self.process_result(result);
            self.clear_temp_strings();
            visit_result
        } else {
            VisitResult::Continue
        }
    }

    fn visit_custom_element(&mut self, ctx: &NodeContext, tag_name: &str, html: &str) -> VisitResult {
        if let Some(callback) = self.callbacks.visit_custom_element {
            let c_tag_name_string =
                std::ffi::CString::new(tag_name).unwrap_or_else(|_| std::ffi::CString::new("").unwrap());
            let c_html_string = std::ffi::CString::new(html).unwrap_or_else(|_| std::ffi::CString::new("").unwrap());

            let c_tag_name = c_tag_name_string.as_ptr();
            let c_html = c_html_string.as_ptr();

            let c_ctx = self.build_node_context(ctx);
            let result = unsafe { callback(self.callbacks.user_data, &raw const c_ctx, c_tag_name, c_html) };
            let visit_result = self.process_result(result);
            self.clear_temp_strings();
            visit_result
        } else {
            VisitResult::Continue
        }
    }

    fn visit_definition_list_start(&mut self, ctx: &NodeContext) -> VisitResult {
        if let Some(callback) = self.callbacks.visit_definition_list_start {
            let c_ctx = self.build_node_context(ctx);
            let result = unsafe { callback(self.callbacks.user_data, &raw const c_ctx) };
            let visit_result = self.process_result(result);
            self.clear_temp_strings();
            visit_result
        } else {
            VisitResult::Continue
        }
    }

    fn visit_definition_term(&mut self, ctx: &NodeContext, text: &str) -> VisitResult {
        if let Some(callback) = self.callbacks.visit_definition_term {
            let c_text_string = std::ffi::CString::new(text).unwrap_or_else(|_| std::ffi::CString::new("").unwrap());
            let c_text = c_text_string.as_ptr();

            let c_ctx = self.build_node_context(ctx);
            let result = unsafe { callback(self.callbacks.user_data, &raw const c_ctx, c_text) };
            let visit_result = self.process_result(result);
            self.clear_temp_strings();
            visit_result
        } else {
            VisitResult::Continue
        }
    }

    fn visit_definition_description(&mut self, ctx: &NodeContext, text: &str) -> VisitResult {
        if let Some(callback) = self.callbacks.visit_definition_description {
            let c_text_string = std::ffi::CString::new(text).unwrap_or_else(|_| std::ffi::CString::new("").unwrap());
            let c_text = c_text_string.as_ptr();

            let c_ctx = self.build_node_context(ctx);
            let result = unsafe { callback(self.callbacks.user_data, &raw const c_ctx, c_text) };
            let visit_result = self.process_result(result);
            self.clear_temp_strings();
            visit_result
        } else {
            VisitResult::Continue
        }
    }

    fn visit_definition_list_end(&mut self, ctx: &NodeContext, output: &str) -> VisitResult {
        if let Some(callback) = self.callbacks.visit_definition_list_end {
            let c_output_string =
                std::ffi::CString::new(output).unwrap_or_else(|_| std::ffi::CString::new("").unwrap());
            let c_output = c_output_string.as_ptr();

            let c_ctx = self.build_node_context(ctx);
            let result = unsafe { callback(self.callbacks.user_data, &raw const c_ctx, c_output) };
            let visit_result = self.process_result(result);
            self.clear_temp_strings();
            visit_result
        } else {
            VisitResult::Continue
        }
    }

    fn visit_form(&mut self, ctx: &NodeContext, action: Option<&str>, method: Option<&str>) -> VisitResult {
        if let Some(callback) = self.callbacks.visit_form {
            let c_action_string = action.and_then(|a| std::ffi::CString::new(a).ok());
            let c_method_string = method.and_then(|m| std::ffi::CString::new(m).ok());

            let c_action = c_action_string.as_ref().map_or(ptr::null(), |s| s.as_ptr());
            let c_method = c_method_string.as_ref().map_or(ptr::null(), |s| s.as_ptr());

            let c_ctx = self.build_node_context(ctx);
            let result = unsafe { callback(self.callbacks.user_data, &raw const c_ctx, c_action, c_method) };
            let visit_result = self.process_result(result);
            self.clear_temp_strings();
            visit_result
        } else {
            VisitResult::Continue
        }
    }

    fn visit_input(
        &mut self,
        ctx: &NodeContext,
        input_type: &str,
        name: Option<&str>,
        value: Option<&str>,
    ) -> VisitResult {
        if let Some(callback) = self.callbacks.visit_input {
            let c_input_type_string =
                std::ffi::CString::new(input_type).unwrap_or_else(|_| std::ffi::CString::new("").unwrap());
            let c_name_string = name.and_then(|n| std::ffi::CString::new(n).ok());
            let c_value_string = value.and_then(|v| std::ffi::CString::new(v).ok());

            let c_input_type = c_input_type_string.as_ptr();
            let c_name = c_name_string.as_ref().map_or(ptr::null(), |s| s.as_ptr());
            let c_value = c_value_string.as_ref().map_or(ptr::null(), |s| s.as_ptr());

            let c_ctx = self.build_node_context(ctx);
            let result = unsafe {
                callback(
                    self.callbacks.user_data,
                    &raw const c_ctx,
                    c_input_type,
                    c_name,
                    c_value,
                )
            };
            let visit_result = self.process_result(result);
            self.clear_temp_strings();
            visit_result
        } else {
            VisitResult::Continue
        }
    }

    fn visit_button(&mut self, ctx: &NodeContext, text: &str) -> VisitResult {
        if let Some(callback) = self.callbacks.visit_button {
            let c_text_string = std::ffi::CString::new(text).unwrap_or_else(|_| std::ffi::CString::new("").unwrap());
            let c_text = c_text_string.as_ptr();

            let c_ctx = self.build_node_context(ctx);
            let result = unsafe { callback(self.callbacks.user_data, &raw const c_ctx, c_text) };
            let visit_result = self.process_result(result);
            self.clear_temp_strings();
            visit_result
        } else {
            VisitResult::Continue
        }
    }

    fn visit_audio(&mut self, ctx: &NodeContext, src: Option<&str>) -> VisitResult {
        if let Some(callback) = self.callbacks.visit_audio {
            let c_src_string = src.and_then(|s| std::ffi::CString::new(s).ok());
            let c_src = c_src_string.as_ref().map_or(ptr::null(), |s| s.as_ptr());

            let c_ctx = self.build_node_context(ctx);
            let result = unsafe { callback(self.callbacks.user_data, &raw const c_ctx, c_src) };
            let visit_result = self.process_result(result);
            self.clear_temp_strings();
            visit_result
        } else {
            VisitResult::Continue
        }
    }

    fn visit_video(&mut self, ctx: &NodeContext, src: Option<&str>) -> VisitResult {
        if let Some(callback) = self.callbacks.visit_video {
            let c_src_string = src.and_then(|s| std::ffi::CString::new(s).ok());
            let c_src = c_src_string.as_ref().map_or(ptr::null(), |s| s.as_ptr());

            let c_ctx = self.build_node_context(ctx);
            let result = unsafe { callback(self.callbacks.user_data, &raw const c_ctx, c_src) };
            let visit_result = self.process_result(result);
            self.clear_temp_strings();
            visit_result
        } else {
            VisitResult::Continue
        }
    }

    fn visit_iframe(&mut self, ctx: &NodeContext, src: Option<&str>) -> VisitResult {
        if let Some(callback) = self.callbacks.visit_iframe {
            let c_src_string = src.and_then(|s| std::ffi::CString::new(s).ok());
            let c_src = c_src_string.as_ref().map_or(ptr::null(), |s| s.as_ptr());

            let c_ctx = self.build_node_context(ctx);
            let result = unsafe { callback(self.callbacks.user_data, &raw const c_ctx, c_src) };
            let visit_result = self.process_result(result);
            self.clear_temp_strings();
            visit_result
        } else {
            VisitResult::Continue
        }
    }

    fn visit_details(&mut self, ctx: &NodeContext, open: bool) -> VisitResult {
        if let Some(callback) = self.callbacks.visit_details {
            let c_ctx = self.build_node_context(ctx);
            let result = unsafe { callback(self.callbacks.user_data, &raw const c_ctx, open) };
            let visit_result = self.process_result(result);
            self.clear_temp_strings();
            visit_result
        } else {
            VisitResult::Continue
        }
    }

    fn visit_summary(&mut self, ctx: &NodeContext, text: &str) -> VisitResult {
        if let Some(callback) = self.callbacks.visit_summary {
            let c_text_string = std::ffi::CString::new(text).unwrap_or_else(|_| std::ffi::CString::new("").unwrap());
            let c_text = c_text_string.as_ptr();

            let c_ctx = self.build_node_context(ctx);
            let result = unsafe { callback(self.callbacks.user_data, &raw const c_ctx, c_text) };
            let visit_result = self.process_result(result);
            self.clear_temp_strings();
            visit_result
        } else {
            VisitResult::Continue
        }
    }

    fn visit_figure_start(&mut self, ctx: &NodeContext) -> VisitResult {
        if let Some(callback) = self.callbacks.visit_figure_start {
            let c_ctx = self.build_node_context(ctx);
            let result = unsafe { callback(self.callbacks.user_data, &raw const c_ctx) };
            let visit_result = self.process_result(result);
            self.clear_temp_strings();
            visit_result
        } else {
            VisitResult::Continue
        }
    }

    fn visit_figcaption(&mut self, ctx: &NodeContext, text: &str) -> VisitResult {
        if let Some(callback) = self.callbacks.visit_figcaption {
            let c_text_string = std::ffi::CString::new(text).unwrap_or_else(|_| std::ffi::CString::new("").unwrap());
            let c_text = c_text_string.as_ptr();

            let c_ctx = self.build_node_context(ctx);
            let result = unsafe { callback(self.callbacks.user_data, &raw const c_ctx, c_text) };
            let visit_result = self.process_result(result);
            self.clear_temp_strings();
            visit_result
        } else {
            VisitResult::Continue
        }
    }

    fn visit_figure_end(&mut self, ctx: &NodeContext, output: &str) -> VisitResult {
        if let Some(callback) = self.callbacks.visit_figure_end {
            let c_output_string =
                std::ffi::CString::new(output).unwrap_or_else(|_| std::ffi::CString::new("").unwrap());
            let c_output = c_output_string.as_ptr();

            let c_ctx = self.build_node_context(ctx);
            let result = unsafe { callback(self.callbacks.user_data, &raw const c_ctx, c_output) };
            let visit_result = self.process_result(result);
            self.clear_temp_strings();
            visit_result
        } else {
            VisitResult::Continue
        }
    }
}
