use ext_php_rs::prelude::*;
use ext_php_rs::types::Zval;
use html_to_markdown_rs::visitor::{HtmlVisitor, NodeContext, VisitResult};

/// PHP visitor bridge for custom HTML traversal callbacks.
///
/// This struct bridges PHP callbacks to the Rust HtmlVisitor trait,
/// allowing PHP code to customize HTML-to-Markdown conversion.
#[derive(Debug)]
pub struct PhpVisitorBridge {
    visitor: Zval,
}

impl PhpVisitorBridge {
    /// Create a new bridge wrapping a PHP visitor object.
    pub fn new(visitor: Zval) -> Self {
        Self { visitor }
    }

    /// Convert a PHP array result to a VisitResult enum.
    fn result_from_array(array: &ext_php_rs::types::ZendHashTable) -> PhpResult<VisitResult> {
        let result_type_val = array
            .get("type")
            .ok_or_else(|| PhpException::default("Visitor result array must have 'type' key".to_string()))?;

        let result_type: String = result_type_val
            .string()
            .ok_or_else(|| PhpException::default("Visitor 'type' must be a string".to_string()))?;

        match result_type.to_lowercase().as_str() {
            "continue" => Ok(VisitResult::Continue),
            "skip" => Ok(VisitResult::Skip),
            "preserve_html" => Ok(VisitResult::PreserveHtml),
            "custom" => {
                let output_val = array.get("output").ok_or_else(|| {
                    PhpException::default("Visitor 'custom' result must have 'output' key".to_string())
                })?;

                let output: String = output_val
                    .string()
                    .ok_or_else(|| PhpException::default("Visitor 'output' must be a string".to_string()))?;

                Ok(VisitResult::Custom(output))
            }
            "error" => {
                let message_val = array.get("message").ok_or_else(|| {
                    PhpException::default("Visitor 'error' result must have 'message' key".to_string())
                })?;

                let message: String = message_val
                    .string()
                    .ok_or_else(|| PhpException::default("Visitor 'message' must be a string".to_string()))?;

                Ok(VisitResult::Error(message))
            }
            unknown => Err(PhpException::default(format!(
                "Unknown visitor result type: {}",
                unknown
            ))),
        }
    }

    /// Convert NodeContext to a PHP array.
    fn context_to_array(ctx: &NodeContext) -> PhpResult<ext_php_rs::boxed::ZBox<ext_php_rs::types::ZendHashTable>> {
        let mut table = ext_php_rs::types::ZendHashTable::new();

        let node_type_str = format!("{:?}", ctx.node_type).to_lowercase();
        table.insert("node_type", node_type_str)?;

        table.insert("tag_name", ctx.tag_name.clone())?;

        let mut attrs = ext_php_rs::types::ZendHashTable::new();
        for (k, v) in &ctx.attributes {
            attrs.insert(k.clone(), v.clone())?;
        }
        table.insert("attributes", attrs)?;

        table.insert("depth", ctx.depth as i64)?;

        table.insert("index_in_parent", ctx.index_in_parent as i64)?;

        match &ctx.parent_tag {
            Some(tag) => table.insert("parent_tag", tag.clone())?,
            None => table.insert("parent_tag", ())?,
        }

        table.insert("is_inline", ctx.is_inline)?;

        Ok(table)
    }

    /// Call a PHP visitor method and convert the result.
    fn call_visitor_method(&self, method_name: &str, args: Vec<Zval>) -> VisitResult {
        if !self.visitor.is_object() {
            return VisitResult::Continue;
        }

        let borrowed_args: Vec<&dyn ext_php_rs::convert::IntoZvalDyn> = args
            .iter()
            .map(|arg| arg as &dyn ext_php_rs::convert::IntoZvalDyn)
            .collect();

        match self.visitor.try_call_method(method_name, borrowed_args) {
            Ok(php_result) => {
                if let Some(array) = php_result.array() {
                    match Self::result_from_array(array) {
                        Ok(result) => result,
                        Err(_) => VisitResult::Continue,
                    }
                } else {
                    VisitResult::Continue
                }
            }
            Err(_) => VisitResult::Continue,
        }
    }

    /// Helper to create a Zval from context
    fn ctx_to_zval(ctx: &NodeContext) -> Zval {
        match Self::context_to_array(ctx) {
            Ok(table) => {
                let mut zval = Zval::new();
                zval.set_hashtable(table);
                zval
            }
            Err(_) => Zval::new(),
        }
    }

    /// Helper to create a Zval from a string
    fn str_to_zval(s: &str) -> Zval {
        let mut zval = Zval::new();
        if zval.set_string(s, false).is_err() {
            return Zval::new();
        }
        zval
    }

    /// Helper to create a Zval from an optional string
    fn opt_str_to_zval(s: Option<&str>) -> Zval {
        match s {
            Some(val) => Self::str_to_zval(val),
            None => Zval::new(), // null
        }
    }

    /// Helper to create a Zval from a u32
    fn u32_to_zval(n: u32) -> Zval {
        let mut zval = Zval::new();
        zval.set_long(n as i64);
        zval
    }

    /// Helper to create a Zval from a usize
    fn usize_to_zval(n: usize) -> Zval {
        let mut zval = Zval::new();
        zval.set_long(n as i64);
        zval
    }

    /// Helper to create a Zval from a bool
    fn bool_to_zval(b: bool) -> Zval {
        let mut zval = Zval::new();
        zval.set_bool(b);
        zval
    }

    /// Helper to create a Zval from a string slice array
    fn strings_to_zval(strings: &[String]) -> Zval {
        let mut table = ext_php_rs::types::ZendHashTable::new();
        for (i, s) in strings.iter().enumerate() {
            if table.insert(i as u64, s.clone()).is_err() {
                return Zval::new();
            }
        }
        let mut zval = Zval::new();
        zval.set_hashtable(table);
        zval
    }
}

impl HtmlVisitor for PhpVisitorBridge {
    fn visit_element_start(&mut self, ctx: &NodeContext) -> VisitResult {
        let args = vec![Self::ctx_to_zval(ctx)];
        self.call_visitor_method("visitElementStart", args)
    }

    fn visit_element_end(&mut self, ctx: &NodeContext, output: &str) -> VisitResult {
        let args = vec![Self::ctx_to_zval(ctx), Self::str_to_zval(output)];
        self.call_visitor_method("visitElementEnd", args)
    }

    fn visit_text(&mut self, ctx: &NodeContext, text: &str) -> VisitResult {
        let args = vec![Self::ctx_to_zval(ctx), Self::str_to_zval(text)];
        self.call_visitor_method("visitText", args)
    }

    fn visit_link(&mut self, ctx: &NodeContext, href: &str, text: &str, title: Option<&str>) -> VisitResult {
        let args = vec![
            Self::ctx_to_zval(ctx),
            Self::str_to_zval(href),
            Self::str_to_zval(text),
            Self::opt_str_to_zval(title),
        ];
        self.call_visitor_method("visitLink", args)
    }

    fn visit_image(&mut self, ctx: &NodeContext, src: &str, alt: &str, title: Option<&str>) -> VisitResult {
        let args = vec![
            Self::ctx_to_zval(ctx),
            Self::str_to_zval(src),
            Self::str_to_zval(alt),
            Self::opt_str_to_zval(title),
        ];
        self.call_visitor_method("visitImage", args)
    }

    fn visit_heading(&mut self, ctx: &NodeContext, level: u32, text: &str, id: Option<&str>) -> VisitResult {
        let args = vec![
            Self::ctx_to_zval(ctx),
            Self::u32_to_zval(level),
            Self::str_to_zval(text),
            Self::opt_str_to_zval(id),
        ];
        self.call_visitor_method("visitHeading", args)
    }

    fn visit_code_block(&mut self, ctx: &NodeContext, lang: Option<&str>, code: &str) -> VisitResult {
        let args = vec![
            Self::ctx_to_zval(ctx),
            Self::opt_str_to_zval(lang),
            Self::str_to_zval(code),
        ];
        self.call_visitor_method("visitCodeBlock", args)
    }

    fn visit_code_inline(&mut self, ctx: &NodeContext, code: &str) -> VisitResult {
        let args = vec![Self::ctx_to_zval(ctx), Self::str_to_zval(code)];
        self.call_visitor_method("visitCodeInline", args)
    }

    fn visit_list_item(&mut self, ctx: &NodeContext, ordered: bool, marker: &str, text: &str) -> VisitResult {
        let args = vec![
            Self::ctx_to_zval(ctx),
            Self::bool_to_zval(ordered),
            Self::str_to_zval(marker),
            Self::str_to_zval(text),
        ];
        self.call_visitor_method("visitListItem", args)
    }

    fn visit_list_start(&mut self, ctx: &NodeContext, ordered: bool) -> VisitResult {
        let args = vec![Self::ctx_to_zval(ctx), Self::bool_to_zval(ordered)];
        self.call_visitor_method("visitListStart", args)
    }

    fn visit_list_end(&mut self, ctx: &NodeContext, ordered: bool, output: &str) -> VisitResult {
        let args = vec![
            Self::ctx_to_zval(ctx),
            Self::bool_to_zval(ordered),
            Self::str_to_zval(output),
        ];
        self.call_visitor_method("visitListEnd", args)
    }

    fn visit_table_start(&mut self, ctx: &NodeContext) -> VisitResult {
        let args = vec![Self::ctx_to_zval(ctx)];
        self.call_visitor_method("visitTableStart", args)
    }

    fn visit_table_row(&mut self, ctx: &NodeContext, cells: &[String], is_header: bool) -> VisitResult {
        let args = vec![
            Self::ctx_to_zval(ctx),
            Self::strings_to_zval(cells),
            Self::bool_to_zval(is_header),
        ];
        self.call_visitor_method("visitTableRow", args)
    }

    fn visit_table_end(&mut self, ctx: &NodeContext, output: &str) -> VisitResult {
        let args = vec![Self::ctx_to_zval(ctx), Self::str_to_zval(output)];
        self.call_visitor_method("visitTableEnd", args)
    }

    fn visit_blockquote(&mut self, ctx: &NodeContext, content: &str, depth: usize) -> VisitResult {
        let args = vec![
            Self::ctx_to_zval(ctx),
            Self::str_to_zval(content),
            Self::usize_to_zval(depth),
        ];
        self.call_visitor_method("visitBlockquote", args)
    }

    fn visit_strong(&mut self, ctx: &NodeContext, text: &str) -> VisitResult {
        let args = vec![Self::ctx_to_zval(ctx), Self::str_to_zval(text)];
        self.call_visitor_method("visitStrong", args)
    }

    fn visit_emphasis(&mut self, ctx: &NodeContext, text: &str) -> VisitResult {
        let args = vec![Self::ctx_to_zval(ctx), Self::str_to_zval(text)];
        self.call_visitor_method("visitEmphasis", args)
    }

    fn visit_strikethrough(&mut self, ctx: &NodeContext, text: &str) -> VisitResult {
        let args = vec![Self::ctx_to_zval(ctx), Self::str_to_zval(text)];
        self.call_visitor_method("visitStrikethrough", args)
    }

    fn visit_underline(&mut self, ctx: &NodeContext, text: &str) -> VisitResult {
        let args = vec![Self::ctx_to_zval(ctx), Self::str_to_zval(text)];
        self.call_visitor_method("visitUnderline", args)
    }

    fn visit_subscript(&mut self, ctx: &NodeContext, text: &str) -> VisitResult {
        let args = vec![Self::ctx_to_zval(ctx), Self::str_to_zval(text)];
        self.call_visitor_method("visitSubscript", args)
    }

    fn visit_superscript(&mut self, ctx: &NodeContext, text: &str) -> VisitResult {
        let args = vec![Self::ctx_to_zval(ctx), Self::str_to_zval(text)];
        self.call_visitor_method("visitSuperscript", args)
    }

    fn visit_mark(&mut self, ctx: &NodeContext, text: &str) -> VisitResult {
        let args = vec![Self::ctx_to_zval(ctx), Self::str_to_zval(text)];
        self.call_visitor_method("visitMark", args)
    }

    fn visit_line_break(&mut self, ctx: &NodeContext) -> VisitResult {
        let args = vec![Self::ctx_to_zval(ctx)];
        self.call_visitor_method("visitLineBreak", args)
    }

    fn visit_horizontal_rule(&mut self, ctx: &NodeContext) -> VisitResult {
        let args = vec![Self::ctx_to_zval(ctx)];
        self.call_visitor_method("visitHorizontalRule", args)
    }

    fn visit_custom_element(&mut self, ctx: &NodeContext, tag_name: &str, html: &str) -> VisitResult {
        let args = vec![
            Self::ctx_to_zval(ctx),
            Self::str_to_zval(tag_name),
            Self::str_to_zval(html),
        ];
        self.call_visitor_method("visitCustomElement", args)
    }

    fn visit_definition_list_start(&mut self, ctx: &NodeContext) -> VisitResult {
        let args = vec![Self::ctx_to_zval(ctx)];
        self.call_visitor_method("visitDefinitionListStart", args)
    }

    fn visit_definition_term(&mut self, ctx: &NodeContext, text: &str) -> VisitResult {
        let args = vec![Self::ctx_to_zval(ctx), Self::str_to_zval(text)];
        self.call_visitor_method("visitDefinitionTerm", args)
    }

    fn visit_definition_description(&mut self, ctx: &NodeContext, text: &str) -> VisitResult {
        let args = vec![Self::ctx_to_zval(ctx), Self::str_to_zval(text)];
        self.call_visitor_method("visitDefinitionDescription", args)
    }

    fn visit_definition_list_end(&mut self, ctx: &NodeContext, output: &str) -> VisitResult {
        let args = vec![Self::ctx_to_zval(ctx), Self::str_to_zval(output)];
        self.call_visitor_method("visitDefinitionListEnd", args)
    }

    fn visit_form(&mut self, ctx: &NodeContext, action: Option<&str>, method: Option<&str>) -> VisitResult {
        let args = vec![
            Self::ctx_to_zval(ctx),
            Self::opt_str_to_zval(action),
            Self::opt_str_to_zval(method),
        ];
        self.call_visitor_method("visitForm", args)
    }

    fn visit_input(
        &mut self,
        ctx: &NodeContext,
        input_type: &str,
        name: Option<&str>,
        value: Option<&str>,
    ) -> VisitResult {
        let args = vec![
            Self::ctx_to_zval(ctx),
            Self::str_to_zval(input_type),
            Self::opt_str_to_zval(name),
            Self::opt_str_to_zval(value),
        ];
        self.call_visitor_method("visitInput", args)
    }

    fn visit_button(&mut self, ctx: &NodeContext, text: &str) -> VisitResult {
        let args = vec![Self::ctx_to_zval(ctx), Self::str_to_zval(text)];
        self.call_visitor_method("visitButton", args)
    }

    fn visit_audio(&mut self, ctx: &NodeContext, src: Option<&str>) -> VisitResult {
        let args = vec![Self::ctx_to_zval(ctx), Self::opt_str_to_zval(src)];
        self.call_visitor_method("visitAudio", args)
    }

    fn visit_video(&mut self, ctx: &NodeContext, src: Option<&str>) -> VisitResult {
        let args = vec![Self::ctx_to_zval(ctx), Self::opt_str_to_zval(src)];
        self.call_visitor_method("visitVideo", args)
    }

    fn visit_iframe(&mut self, ctx: &NodeContext, src: Option<&str>) -> VisitResult {
        let args = vec![Self::ctx_to_zval(ctx), Self::opt_str_to_zval(src)];
        self.call_visitor_method("visitIframe", args)
    }

    fn visit_details(&mut self, ctx: &NodeContext, open: bool) -> VisitResult {
        let args = vec![Self::ctx_to_zval(ctx), Self::bool_to_zval(open)];
        self.call_visitor_method("visitDetails", args)
    }

    fn visit_summary(&mut self, ctx: &NodeContext, text: &str) -> VisitResult {
        let args = vec![Self::ctx_to_zval(ctx), Self::str_to_zval(text)];
        self.call_visitor_method("visitSummary", args)
    }

    fn visit_figure_start(&mut self, ctx: &NodeContext) -> VisitResult {
        let args = vec![Self::ctx_to_zval(ctx)];
        self.call_visitor_method("visitFigureStart", args)
    }

    fn visit_figcaption(&mut self, ctx: &NodeContext, text: &str) -> VisitResult {
        let args = vec![Self::ctx_to_zval(ctx), Self::str_to_zval(text)];
        self.call_visitor_method("visitFigcaption", args)
    }

    fn visit_figure_end(&mut self, ctx: &NodeContext, output: &str) -> VisitResult {
        let args = vec![Self::ctx_to_zval(ctx), Self::str_to_zval(output)];
        self.call_visitor_method("visitFigureEnd", args)
    }
}
