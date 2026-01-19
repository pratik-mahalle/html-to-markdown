//! Ruby visitor wrapper and bridge utilities.

use crate::types::{arg_error, symbol_to_string};
use html_to_markdown_rs::visitor::{NodeContext, NodeType, VisitResult};
use magnus::prelude::*;
use magnus::{Error, RHash, Ruby, TryConvert, Value};

/// Wrapper for a Ruby visitor object that implements the HtmlVisitor trait.
#[derive(Clone)]
pub struct RubyVisitorWrapper {
    pub ruby_visitor: Value,
    pub last_error: std::rc::Rc<std::cell::RefCell<Option<String>>>,
}

impl RubyVisitorWrapper {
    pub fn new(ruby_visitor: Value) -> Self {
        Self {
            ruby_visitor,
            last_error: std::rc::Rc::new(std::cell::RefCell::new(None)),
        }
    }

    pub fn utf8_str(&self, ruby: &Ruby, s: &str) -> Value {
        if let Ok(val) = ruby.eval::<Value>(&format!("String.new({s:?}, encoding: 'UTF-8')")) {
            val
        } else {
            let str_val = ruby.str_from_slice(s.as_bytes());
            str_val.as_value()
        }
    }

    pub fn call_visitor_method(&self, method_name: &str, args: &[Value]) -> Result<VisitResult, Error> {
        let ruby = Ruby::get().expect("Ruby not initialized");

        let result: Value = match args.len() {
            0 => match self.ruby_visitor.funcall::<&str, (), Value>(method_name, ()) {
                Ok(val) => val,
                Err(e) => {
                    *self.last_error.borrow_mut() = Some(format!("Visitor error in {method_name}: {e}"));
                    return Err(e);
                }
            },
            1 => match self
                .ruby_visitor
                .funcall::<&str, (Value,), Value>(method_name, (args[0],))
            {
                Ok(val) => val,
                Err(e) => {
                    *self.last_error.borrow_mut() = Some(format!("Visitor error in {method_name}: {e}"));
                    return Err(e);
                }
            },
            2 => match self
                .ruby_visitor
                .funcall::<&str, (Value, Value), Value>(method_name, (args[0], args[1]))
            {
                Ok(val) => val,
                Err(e) => {
                    *self.last_error.borrow_mut() = Some(format!("Visitor error in {method_name}: {e}"));
                    return Err(e);
                }
            },
            3 => match self
                .ruby_visitor
                .funcall::<&str, (Value, Value, Value), Value>(method_name, (args[0], args[1], args[2]))
            {
                Ok(val) => val,
                Err(e) => {
                    *self.last_error.borrow_mut() = Some(format!("Visitor error in {method_name}: {e}"));
                    return Err(e);
                }
            },
            4 => match self
                .ruby_visitor
                .funcall::<&str, (Value, Value, Value, Value), Value>(method_name, (args[0], args[1], args[2], args[3]))
            {
                Ok(val) => val,
                Err(e) => {
                    *self.last_error.borrow_mut() = Some(format!("Visitor error in {method_name}: {e}"));
                    return Err(e);
                }
            },
            _ => {
                return Err(arg_error(format!(
                    "Unsupported number of visitor method arguments: {}",
                    args.len()
                )));
            }
        };

        let hash = RHash::from_value(result)
            .ok_or_else(|| arg_error(format!("visitor method {method_name} must return a Hash")))?;

        let type_value: Value = hash
            .get(ruby.intern("type"))
            .ok_or_else(|| arg_error(format!("visitor method {method_name} result Hash must have :type key")))?;

        let type_str = symbol_to_string(type_value)?;

        match type_str.as_str() {
            "continue" => Ok(VisitResult::Continue),
            "custom" => {
                let output_value: Value = hash.get(ruby.intern("output")).ok_or_else(|| {
                    arg_error(format!(
                        "visitor method {method_name} with type :custom must provide :output string"
                    ))
                })?;
                let output = String::try_convert(output_value)?;
                Ok(VisitResult::Custom(output))
            }
            "skip" => Ok(VisitResult::Skip),
            "preserve_html" => Ok(VisitResult::PreserveHtml),
            "error" => {
                let message_value: Value = hash.get(ruby.intern("message")).ok_or_else(|| {
                    arg_error(format!(
                        "visitor method {method_name} with type :error must provide :message string"
                    ))
                })?;
                let message = String::try_convert(message_value)?;
                Ok(VisitResult::Error(message))
            }
            other => Err(arg_error(format!(
                "visitor method {method_name} returned invalid type: {other}"
            ))),
        }
    }

    pub fn ruby_to_node_context(&self, ctx: &NodeContext, ruby: &Ruby) -> Result<Value, Error> {
        let hash = ruby.hash_new();

        let node_type_str = match ctx.node_type {
            NodeType::Text => "text",
            NodeType::Element => "element",
            NodeType::Heading => "heading",
            NodeType::Paragraph => "paragraph",
            NodeType::Div => "div",
            NodeType::Blockquote => "blockquote",
            NodeType::Pre => "pre",
            NodeType::Hr => "hr",
            NodeType::List => "list",
            NodeType::ListItem => "list_item",
            NodeType::DefinitionList => "definition_list",
            NodeType::DefinitionTerm => "definition_term",
            NodeType::DefinitionDescription => "definition_description",
            NodeType::Table => "table",
            NodeType::TableRow => "table_row",
            NodeType::TableCell => "table_cell",
            NodeType::TableHeader => "table_header",
            NodeType::TableBody => "table_body",
            NodeType::TableHead => "table_head",
            NodeType::TableFoot => "table_foot",
            NodeType::Link => "link",
            NodeType::Image => "image",
            NodeType::Strong => "strong",
            NodeType::Em => "em",
            NodeType::Code => "code",
            NodeType::Strikethrough => "strikethrough",
            NodeType::Underline => "underline",
            NodeType::Subscript => "subscript",
            NodeType::Superscript => "superscript",
            NodeType::Mark => "mark",
            NodeType::Small => "small",
            NodeType::Br => "br",
            NodeType::Span => "span",
            NodeType::Article => "article",
            NodeType::Section => "section",
            NodeType::Nav => "nav",
            NodeType::Aside => "aside",
            NodeType::Header => "header",
            NodeType::Footer => "footer",
            NodeType::Main => "main",
            NodeType::Figure => "figure",
            NodeType::Figcaption => "figcaption",
            NodeType::Time => "time",
            NodeType::Details => "details",
            NodeType::Summary => "summary",
            NodeType::Form => "form",
            NodeType::Input => "input",
            NodeType::Select => "select",
            NodeType::Option => "option",
            NodeType::Button => "button",
            NodeType::Textarea => "textarea",
            NodeType::Label => "label",
            NodeType::Fieldset => "fieldset",
            NodeType::Legend => "legend",
            NodeType::Audio => "audio",
            NodeType::Video => "video",
            NodeType::Picture => "picture",
            NodeType::Source => "source",
            NodeType::Iframe => "iframe",
            NodeType::Svg => "svg",
            NodeType::Canvas => "canvas",
            NodeType::Ruby => "ruby",
            NodeType::Rt => "rt",
            NodeType::Rp => "rp",
            NodeType::Abbr => "abbr",
            NodeType::Kbd => "kbd",
            NodeType::Samp => "samp",
            NodeType::Var => "var",
            NodeType::Cite => "cite",
            NodeType::Q => "q",
            NodeType::Del => "del",
            NodeType::Ins => "ins",
            NodeType::Data => "data",
            NodeType::Meter => "meter",
            NodeType::Progress => "progress",
            NodeType::Output => "output",
            NodeType::Template => "template",
            NodeType::Slot => "slot",
            NodeType::Html => "html",
            NodeType::Head => "head",
            NodeType::Body => "body",
            NodeType::Title => "title",
            NodeType::Meta => "meta",
            NodeType::LinkTag => "link_tag",
            NodeType::Style => "style",
            NodeType::Script => "script",
            NodeType::Base => "base",
            NodeType::Custom => "custom",
        };
        hash.aset(ruby.intern("node_type"), ruby.intern(node_type_str))?;

        hash.aset(ruby.intern("tag_name"), ctx.tag_name.as_str())?;

        let attrs_hash = ruby.hash_new();
        for (key, value) in &ctx.attributes {
            attrs_hash.aset(key.as_str(), value.as_str())?;
        }
        hash.aset(ruby.intern("attributes"), attrs_hash)?;

        hash.aset(ruby.intern("depth"), ctx.depth as i64)?;

        hash.aset(ruby.intern("index_in_parent"), ctx.index_in_parent as i64)?;

        match &ctx.parent_tag {
            Some(tag) => hash.aset(ruby.intern("parent_tag"), tag.as_str())?,
            None => hash.aset(ruby.intern("parent_tag"), ruby.qnil())?,
        }

        hash.aset(ruby.intern("is_inline"), ctx.is_inline)?;

        Ok(hash.as_value())
    }
}

impl std::fmt::Debug for RubyVisitorWrapper {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RubyVisitorWrapper")
            .field("ruby_visitor", &self.ruby_visitor)
            .finish()
    }
}
