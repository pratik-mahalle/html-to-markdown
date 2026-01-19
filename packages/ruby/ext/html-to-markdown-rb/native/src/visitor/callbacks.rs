//! HtmlVisitor trait implementation for Ruby visitor wrapper.

use super::bridge::RubyVisitorWrapper;
use html_to_markdown_rs::visitor::{HtmlVisitor, NodeContext, VisitResult};
use magnus::Ruby;
use magnus::prelude::*;

impl HtmlVisitor for RubyVisitorWrapper {
    fn visit_element_start(&mut self, ctx: &NodeContext) -> VisitResult {
        if let Ok(ruby) = Ruby::get() {
            if let Ok(node_ctx) = self.ruby_to_node_context(ctx, &ruby) {
                if let Ok(result) = self.call_visitor_method("visit_element_start", &[node_ctx]) {
                    return result;
                }
            }
        }
        VisitResult::Continue
    }

    fn visit_element_end(&mut self, ctx: &NodeContext, output: &str) -> VisitResult {
        if let Ok(ruby) = Ruby::get() {
            if let Ok(node_ctx) = self.ruby_to_node_context(ctx, &ruby) {
                if let Ok(result) = self.call_visitor_method(
                    "visit_element_end",
                    &[node_ctx, ruby.str_from_slice(output.as_bytes()).as_value()],
                ) {
                    return result;
                }
            }
        }
        VisitResult::Continue
    }

    fn visit_text(&mut self, ctx: &NodeContext, text: &str) -> VisitResult {
        if let Ok(ruby) = Ruby::get() {
            if let Ok(node_ctx) = self.ruby_to_node_context(ctx, &ruby) {
                if let Ok(result) = self.call_visitor_method("visit_text", &[node_ctx, self.utf8_str(&ruby, text)]) {
                    return result;
                }
            }
        }
        VisitResult::Continue
    }

    fn visit_link(&mut self, ctx: &NodeContext, href: &str, text: &str, title: Option<&str>) -> VisitResult {
        if let Ok(ruby) = Ruby::get() {
            if let Ok(node_ctx) = self.ruby_to_node_context(ctx, &ruby) {
                let title_val = match title {
                    Some(t) => ruby.str_from_slice(t.as_bytes()).as_value(),
                    None => ruby.qnil().as_value(),
                };
                if let Ok(result) = self.call_visitor_method(
                    "visit_link",
                    &[
                        node_ctx,
                        ruby.str_from_slice(href.as_bytes()).as_value(),
                        ruby.str_from_slice(text.as_bytes()).as_value(),
                        title_val,
                    ],
                ) {
                    return result;
                }
            }
        }
        VisitResult::Continue
    }

    fn visit_image(&mut self, ctx: &NodeContext, src: &str, alt: &str, title: Option<&str>) -> VisitResult {
        if let Ok(ruby) = Ruby::get() {
            if let Ok(node_ctx) = self.ruby_to_node_context(ctx, &ruby) {
                let title_val = match title {
                    Some(t) => ruby.str_from_slice(t.as_bytes()).as_value(),
                    None => ruby.qnil().as_value(),
                };
                if let Ok(result) = self.call_visitor_method(
                    "visit_image",
                    &[
                        node_ctx,
                        ruby.str_from_slice(src.as_bytes()).as_value(),
                        ruby.str_from_slice(alt.as_bytes()).as_value(),
                        title_val,
                    ],
                ) {
                    return result;
                }
            }
        }
        VisitResult::Continue
    }

    fn visit_heading(&mut self, ctx: &NodeContext, level: u32, text: &str, id: Option<&str>) -> VisitResult {
        if let Ok(ruby) = Ruby::get() {
            if let Ok(node_ctx) = self.ruby_to_node_context(ctx, &ruby) {
                let id_val = match id {
                    Some(i) => ruby.str_from_slice(i.as_bytes()).as_value(),
                    None => ruby.qnil().as_value(),
                };
                if let Ok(result) = self.call_visitor_method(
                    "visit_heading",
                    &[
                        node_ctx,
                        ruby.integer_from_i64(i64::from(level)).as_value(),
                        ruby.str_from_slice(text.as_bytes()).as_value(),
                        id_val,
                    ],
                ) {
                    return result;
                }
            }
        }
        VisitResult::Continue
    }

    fn visit_code_block(&mut self, ctx: &NodeContext, lang: Option<&str>, code: &str) -> VisitResult {
        if let Ok(ruby) = Ruby::get() {
            if let Ok(node_ctx) = self.ruby_to_node_context(ctx, &ruby) {
                let lang_val = match lang {
                    Some(l) => ruby.str_from_slice(l.as_bytes()).as_value(),
                    None => ruby.qnil().as_value(),
                };
                if let Ok(result) = self.call_visitor_method(
                    "visit_code_block",
                    &[node_ctx, lang_val, ruby.str_from_slice(code.as_bytes()).as_value()],
                ) {
                    return result;
                }
            }
        }
        VisitResult::Continue
    }

    fn visit_code_inline(&mut self, ctx: &NodeContext, code: &str) -> VisitResult {
        if let Ok(ruby) = Ruby::get() {
            if let Ok(node_ctx) = self.ruby_to_node_context(ctx, &ruby) {
                if let Ok(result) = self.call_visitor_method(
                    "visit_code_inline",
                    &[node_ctx, ruby.str_from_slice(code.as_bytes()).as_value()],
                ) {
                    return result;
                }
            }
        }
        VisitResult::Continue
    }

    fn visit_list_item(&mut self, ctx: &NodeContext, ordered: bool, marker: &str, text: &str) -> VisitResult {
        if let Ok(ruby) = Ruby::get() {
            if let Ok(node_ctx) = self.ruby_to_node_context(ctx, &ruby) {
                let ordered_val = if ordered {
                    ruby.qtrue().as_value()
                } else {
                    ruby.qfalse().as_value()
                };
                if let Ok(result) = self.call_visitor_method(
                    "visit_list_item",
                    &[
                        node_ctx,
                        ordered_val,
                        ruby.str_from_slice(marker.as_bytes()).as_value(),
                        ruby.str_from_slice(text.as_bytes()).as_value(),
                    ],
                ) {
                    return result;
                }
            }
        }
        VisitResult::Continue
    }

    fn visit_list_start(&mut self, ctx: &NodeContext, ordered: bool) -> VisitResult {
        if let Ok(ruby) = Ruby::get() {
            if let Ok(node_ctx) = self.ruby_to_node_context(ctx, &ruby) {
                let ordered_val = if ordered {
                    ruby.qtrue().as_value()
                } else {
                    ruby.qfalse().as_value()
                };
                if let Ok(result) = self.call_visitor_method("visit_list_start", &[node_ctx, ordered_val]) {
                    return result;
                }
            }
        }
        VisitResult::Continue
    }

    fn visit_list_end(&mut self, ctx: &NodeContext, ordered: bool, output: &str) -> VisitResult {
        if let Ok(ruby) = Ruby::get() {
            if let Ok(node_ctx) = self.ruby_to_node_context(ctx, &ruby) {
                let ordered_val = if ordered {
                    ruby.qtrue().as_value()
                } else {
                    ruby.qfalse().as_value()
                };
                if let Ok(result) = self.call_visitor_method(
                    "visit_list_end",
                    &[node_ctx, ordered_val, ruby.str_from_slice(output.as_bytes()).as_value()],
                ) {
                    return result;
                }
            }
        }
        VisitResult::Continue
    }

    fn visit_table_start(&mut self, ctx: &NodeContext) -> VisitResult {
        if let Ok(ruby) = Ruby::get() {
            if let Ok(node_ctx) = self.ruby_to_node_context(ctx, &ruby) {
                if let Ok(result) = self.call_visitor_method("visit_table_start", &[node_ctx]) {
                    return result;
                }
            }
        }
        VisitResult::Continue
    }

    fn visit_table_row(&mut self, ctx: &NodeContext, cells: &[String], is_header: bool) -> VisitResult {
        if let Ok(ruby) = Ruby::get() {
            if let Ok(node_ctx) = self.ruby_to_node_context(ctx, &ruby) {
                let cells_array = ruby.ary_new();
                for cell in cells {
                    let _ = cells_array.push(ruby.str_from_slice(cell.as_bytes()).as_value());
                }
                let is_header_val = if is_header {
                    ruby.qtrue().as_value()
                } else {
                    ruby.qfalse().as_value()
                };
                if let Ok(result) =
                    self.call_visitor_method("visit_table_row", &[node_ctx, cells_array.as_value(), is_header_val])
                {
                    return result;
                }
            }
        }
        VisitResult::Continue
    }

    fn visit_table_end(&mut self, ctx: &NodeContext, output: &str) -> VisitResult {
        if let Ok(ruby) = Ruby::get() {
            if let Ok(node_ctx) = self.ruby_to_node_context(ctx, &ruby) {
                if let Ok(result) = self.call_visitor_method(
                    "visit_table_end",
                    &[node_ctx, ruby.str_from_slice(output.as_bytes()).as_value()],
                ) {
                    return result;
                }
            }
        }
        VisitResult::Continue
    }

    fn visit_blockquote(&mut self, ctx: &NodeContext, content: &str, depth: usize) -> VisitResult {
        if let Ok(ruby) = Ruby::get() {
            if let Ok(node_ctx) = self.ruby_to_node_context(ctx, &ruby) {
                if let Ok(result) = self.call_visitor_method(
                    "visit_blockquote",
                    &[
                        node_ctx,
                        ruby.str_from_slice(content.as_bytes()).as_value(),
                        ruby.integer_from_i64(depth as i64).as_value(),
                    ],
                ) {
                    return result;
                }
            }
        }
        VisitResult::Continue
    }

    fn visit_strong(&mut self, ctx: &NodeContext, text: &str) -> VisitResult {
        if let Ok(ruby) = Ruby::get() {
            if let Ok(node_ctx) = self.ruby_to_node_context(ctx, &ruby) {
                if let Ok(result) = self.call_visitor_method(
                    "visit_strong",
                    &[node_ctx, ruby.str_from_slice(text.as_bytes()).as_value()],
                ) {
                    return result;
                }
            }
        }
        VisitResult::Continue
    }

    fn visit_emphasis(&mut self, ctx: &NodeContext, text: &str) -> VisitResult {
        if let Ok(ruby) = Ruby::get() {
            if let Ok(node_ctx) = self.ruby_to_node_context(ctx, &ruby) {
                if let Ok(result) = self.call_visitor_method(
                    "visit_emphasis",
                    &[node_ctx, ruby.str_from_slice(text.as_bytes()).as_value()],
                ) {
                    return result;
                }
            }
        }
        VisitResult::Continue
    }

    fn visit_strikethrough(&mut self, ctx: &NodeContext, text: &str) -> VisitResult {
        if let Ok(ruby) = Ruby::get() {
            if let Ok(node_ctx) = self.ruby_to_node_context(ctx, &ruby) {
                if let Ok(result) = self.call_visitor_method(
                    "visit_strikethrough",
                    &[node_ctx, ruby.str_from_slice(text.as_bytes()).as_value()],
                ) {
                    return result;
                }
            }
        }
        VisitResult::Continue
    }

    fn visit_underline(&mut self, ctx: &NodeContext, text: &str) -> VisitResult {
        if let Ok(ruby) = Ruby::get() {
            if let Ok(node_ctx) = self.ruby_to_node_context(ctx, &ruby) {
                if let Ok(result) = self.call_visitor_method(
                    "visit_underline",
                    &[node_ctx, ruby.str_from_slice(text.as_bytes()).as_value()],
                ) {
                    return result;
                }
            }
        }
        VisitResult::Continue
    }

    fn visit_subscript(&mut self, ctx: &NodeContext, text: &str) -> VisitResult {
        if let Ok(ruby) = Ruby::get() {
            if let Ok(node_ctx) = self.ruby_to_node_context(ctx, &ruby) {
                if let Ok(result) = self.call_visitor_method(
                    "visit_subscript",
                    &[node_ctx, ruby.str_from_slice(text.as_bytes()).as_value()],
                ) {
                    return result;
                }
            }
        }
        VisitResult::Continue
    }

    fn visit_superscript(&mut self, ctx: &NodeContext, text: &str) -> VisitResult {
        if let Ok(ruby) = Ruby::get() {
            if let Ok(node_ctx) = self.ruby_to_node_context(ctx, &ruby) {
                if let Ok(result) = self.call_visitor_method(
                    "visit_superscript",
                    &[node_ctx, ruby.str_from_slice(text.as_bytes()).as_value()],
                ) {
                    return result;
                }
            }
        }
        VisitResult::Continue
    }

    fn visit_mark(&mut self, ctx: &NodeContext, text: &str) -> VisitResult {
        if let Ok(ruby) = Ruby::get() {
            if let Ok(node_ctx) = self.ruby_to_node_context(ctx, &ruby) {
                if let Ok(result) = self.call_visitor_method(
                    "visit_mark",
                    &[node_ctx, ruby.str_from_slice(text.as_bytes()).as_value()],
                ) {
                    return result;
                }
            }
        }
        VisitResult::Continue
    }

    fn visit_line_break(&mut self, ctx: &NodeContext) -> VisitResult {
        if let Ok(ruby) = Ruby::get() {
            if let Ok(node_ctx) = self.ruby_to_node_context(ctx, &ruby) {
                if let Ok(result) = self.call_visitor_method("visit_line_break", &[node_ctx]) {
                    return result;
                }
            }
        }
        VisitResult::Continue
    }

    fn visit_horizontal_rule(&mut self, ctx: &NodeContext) -> VisitResult {
        if let Ok(ruby) = Ruby::get() {
            if let Ok(node_ctx) = self.ruby_to_node_context(ctx, &ruby) {
                if let Ok(result) = self.call_visitor_method("visit_horizontal_rule", &[node_ctx]) {
                    return result;
                }
            }
        }
        VisitResult::Continue
    }

    fn visit_custom_element(&mut self, ctx: &NodeContext, tag_name: &str, html: &str) -> VisitResult {
        if let Ok(ruby) = Ruby::get() {
            if let Ok(node_ctx) = self.ruby_to_node_context(ctx, &ruby) {
                if let Ok(result) = self.call_visitor_method(
                    "visit_custom_element",
                    &[
                        node_ctx,
                        ruby.str_from_slice(tag_name.as_bytes()).as_value(),
                        ruby.str_from_slice(html.as_bytes()).as_value(),
                    ],
                ) {
                    return result;
                }
            }
        }
        VisitResult::Continue
    }

    fn visit_definition_list_start(&mut self, ctx: &NodeContext) -> VisitResult {
        if let Ok(ruby) = Ruby::get() {
            if let Ok(node_ctx) = self.ruby_to_node_context(ctx, &ruby) {
                if let Ok(result) = self.call_visitor_method("visit_definition_list_start", &[node_ctx]) {
                    return result;
                }
            }
        }
        VisitResult::Continue
    }

    fn visit_definition_term(&mut self, ctx: &NodeContext, text: &str) -> VisitResult {
        if let Ok(ruby) = Ruby::get() {
            if let Ok(node_ctx) = self.ruby_to_node_context(ctx, &ruby) {
                if let Ok(result) = self.call_visitor_method(
                    "visit_definition_term",
                    &[node_ctx, ruby.str_from_slice(text.as_bytes()).as_value()],
                ) {
                    return result;
                }
            }
        }
        VisitResult::Continue
    }

    fn visit_definition_description(&mut self, ctx: &NodeContext, text: &str) -> VisitResult {
        if let Ok(ruby) = Ruby::get() {
            if let Ok(node_ctx) = self.ruby_to_node_context(ctx, &ruby) {
                if let Ok(result) = self.call_visitor_method(
                    "visit_definition_description",
                    &[node_ctx, ruby.str_from_slice(text.as_bytes()).as_value()],
                ) {
                    return result;
                }
            }
        }
        VisitResult::Continue
    }

    fn visit_definition_list_end(&mut self, ctx: &NodeContext, output: &str) -> VisitResult {
        if let Ok(ruby) = Ruby::get() {
            if let Ok(node_ctx) = self.ruby_to_node_context(ctx, &ruby) {
                if let Ok(result) = self.call_visitor_method(
                    "visit_definition_list_end",
                    &[node_ctx, ruby.str_from_slice(output.as_bytes()).as_value()],
                ) {
                    return result;
                }
            }
        }
        VisitResult::Continue
    }

    fn visit_form(&mut self, ctx: &NodeContext, action: Option<&str>, method: Option<&str>) -> VisitResult {
        if let Ok(ruby) = Ruby::get() {
            if let Ok(node_ctx) = self.ruby_to_node_context(ctx, &ruby) {
                let action_val = match action {
                    Some(a) => ruby.str_from_slice(a.as_bytes()).as_value(),
                    None => ruby.qnil().as_value(),
                };
                let method_val = match method {
                    Some(m) => ruby.str_from_slice(m.as_bytes()).as_value(),
                    None => ruby.qnil().as_value(),
                };
                if let Ok(result) = self.call_visitor_method("visit_form", &[node_ctx, action_val, method_val]) {
                    return result;
                }
            }
        }
        VisitResult::Continue
    }

    fn visit_input(
        &mut self,
        ctx: &NodeContext,
        input_type: &str,
        name: Option<&str>,
        value: Option<&str>,
    ) -> VisitResult {
        if let Ok(ruby) = Ruby::get() {
            if let Ok(node_ctx) = self.ruby_to_node_context(ctx, &ruby) {
                let name_val = match name {
                    Some(n) => ruby.str_from_slice(n.as_bytes()).as_value(),
                    None => ruby.qnil().as_value(),
                };
                let value_val = match value {
                    Some(v) => ruby.str_from_slice(v.as_bytes()).as_value(),
                    None => ruby.qnil().as_value(),
                };
                if let Ok(result) = self.call_visitor_method(
                    "visit_input",
                    &[
                        node_ctx,
                        ruby.str_from_slice(input_type.as_bytes()).as_value(),
                        name_val,
                        value_val,
                    ],
                ) {
                    return result;
                }
            }
        }
        VisitResult::Continue
    }

    fn visit_button(&mut self, ctx: &NodeContext, text: &str) -> VisitResult {
        if let Ok(ruby) = Ruby::get() {
            if let Ok(node_ctx) = self.ruby_to_node_context(ctx, &ruby) {
                if let Ok(result) = self.call_visitor_method(
                    "visit_button",
                    &[node_ctx, ruby.str_from_slice(text.as_bytes()).as_value()],
                ) {
                    return result;
                }
            }
        }
        VisitResult::Continue
    }

    fn visit_audio(&mut self, ctx: &NodeContext, src: Option<&str>) -> VisitResult {
        if let Ok(ruby) = Ruby::get() {
            if let Ok(node_ctx) = self.ruby_to_node_context(ctx, &ruby) {
                let src_val = match src {
                    Some(s) => ruby.str_from_slice(s.as_bytes()).as_value(),
                    None => ruby.qnil().as_value(),
                };
                if let Ok(result) = self.call_visitor_method("visit_audio", &[node_ctx, src_val]) {
                    return result;
                }
            }
        }
        VisitResult::Continue
    }

    fn visit_video(&mut self, ctx: &NodeContext, src: Option<&str>) -> VisitResult {
        if let Ok(ruby) = Ruby::get() {
            if let Ok(node_ctx) = self.ruby_to_node_context(ctx, &ruby) {
                let src_val = match src {
                    Some(s) => ruby.str_from_slice(s.as_bytes()).as_value(),
                    None => ruby.qnil().as_value(),
                };
                if let Ok(result) = self.call_visitor_method("visit_video", &[node_ctx, src_val]) {
                    return result;
                }
            }
        }
        VisitResult::Continue
    }

    fn visit_iframe(&mut self, ctx: &NodeContext, src: Option<&str>) -> VisitResult {
        if let Ok(ruby) = Ruby::get() {
            if let Ok(node_ctx) = self.ruby_to_node_context(ctx, &ruby) {
                let src_val = match src {
                    Some(s) => ruby.str_from_slice(s.as_bytes()).as_value(),
                    None => ruby.qnil().as_value(),
                };
                if let Ok(result) = self.call_visitor_method("visit_iframe", &[node_ctx, src_val]) {
                    return result;
                }
            }
        }
        VisitResult::Continue
    }

    fn visit_details(&mut self, ctx: &NodeContext, open: bool) -> VisitResult {
        if let Ok(ruby) = Ruby::get() {
            if let Ok(node_ctx) = self.ruby_to_node_context(ctx, &ruby) {
                let open_val = if open {
                    ruby.qtrue().as_value()
                } else {
                    ruby.qfalse().as_value()
                };
                if let Ok(result) = self.call_visitor_method("visit_details", &[node_ctx, open_val]) {
                    return result;
                }
            }
        }
        VisitResult::Continue
    }

    fn visit_summary(&mut self, ctx: &NodeContext, text: &str) -> VisitResult {
        if let Ok(ruby) = Ruby::get() {
            if let Ok(node_ctx) = self.ruby_to_node_context(ctx, &ruby) {
                if let Ok(result) = self.call_visitor_method(
                    "visit_summary",
                    &[node_ctx, ruby.str_from_slice(text.as_bytes()).as_value()],
                ) {
                    return result;
                }
            }
        }
        VisitResult::Continue
    }

    fn visit_figure_start(&mut self, ctx: &NodeContext) -> VisitResult {
        if let Ok(ruby) = Ruby::get() {
            if let Ok(node_ctx) = self.ruby_to_node_context(ctx, &ruby) {
                if let Ok(result) = self.call_visitor_method("visit_figure_start", &[node_ctx]) {
                    return result;
                }
            }
        }
        VisitResult::Continue
    }

    fn visit_figcaption(&mut self, ctx: &NodeContext, text: &str) -> VisitResult {
        if let Ok(ruby) = Ruby::get() {
            if let Ok(node_ctx) = self.ruby_to_node_context(ctx, &ruby) {
                if let Ok(result) = self.call_visitor_method(
                    "visit_figcaption",
                    &[node_ctx, ruby.str_from_slice(text.as_bytes()).as_value()],
                ) {
                    return result;
                }
            }
        }
        VisitResult::Continue
    }

    fn visit_figure_end(&mut self, ctx: &NodeContext, output: &str) -> VisitResult {
        if let Ok(ruby) = Ruby::get() {
            if let Ok(node_ctx) = self.ruby_to_node_context(ctx, &ruby) {
                if let Ok(result) = self.call_visitor_method(
                    "visit_figure_end",
                    &[node_ctx, ruby.str_from_slice(output.as_bytes()).as_value()],
                ) {
                    return result;
                }
            }
        }
        VisitResult::Continue
    }
}
