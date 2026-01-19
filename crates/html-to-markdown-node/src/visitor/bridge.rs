//! JavaScript visitor bridge for async HTML visitor callbacks.
//!
//! This module provides the bridge between JavaScript visitor callbacks and the Rust
//! AsyncHtmlVisitor trait.

use crate::visitor::params::{
    BlockquoteParams, CodeBlockParams, CodeInlineParams, CustomElementParams, DefinitionListEndParams, DetailsParams,
    ElementEndParams, FigureEndParams, FormParams, HeadingParams, ImageParams, InputParams, LinkParams, ListEndParams,
    ListItemParams, ListStartParams, MediaParams, TableEndParams, TableRowParams, TextParams,
};
use crate::visitor::types::{JsNodeContext, JsVisitResult, VisitorFn};
use async_trait::async_trait;
use html_to_markdown_rs::visitor::{AsyncHtmlVisitor, NodeContext as RustNodeContext, VisitResult as RustVisitResult};
use std::collections::HashMap;

/// Bridge struct that holds optional JavaScript callback functions for each visitor method.
#[cfg(feature = "async-visitor")]
#[allow(dead_code)]
#[derive(Clone)]
pub struct JsVisitorBridge {
    pub visit_element_start_fn: Option<VisitorFn>,
    pub visit_element_end_fn: Option<VisitorFn>,
    pub visit_text_fn: Option<VisitorFn>,
    pub visit_link_fn: Option<VisitorFn>,
    pub visit_image_fn: Option<VisitorFn>,
    pub visit_heading_fn: Option<VisitorFn>,
    pub visit_code_block_fn: Option<VisitorFn>,
    pub visit_code_inline_fn: Option<VisitorFn>,
    pub visit_list_item_fn: Option<VisitorFn>,
    pub visit_list_start_fn: Option<VisitorFn>,
    pub visit_list_end_fn: Option<VisitorFn>,
    pub visit_table_start_fn: Option<VisitorFn>,
    pub visit_table_row_fn: Option<VisitorFn>,
    pub visit_table_end_fn: Option<VisitorFn>,
    pub visit_blockquote_fn: Option<VisitorFn>,
    pub visit_strong_fn: Option<VisitorFn>,
    pub visit_emphasis_fn: Option<VisitorFn>,
    pub visit_strikethrough_fn: Option<VisitorFn>,
    pub visit_underline_fn: Option<VisitorFn>,
    pub visit_subscript_fn: Option<VisitorFn>,
    pub visit_superscript_fn: Option<VisitorFn>,
    pub visit_mark_fn: Option<VisitorFn>,
    pub visit_line_break_fn: Option<VisitorFn>,
    pub visit_horizontal_rule_fn: Option<VisitorFn>,
    pub visit_custom_element_fn: Option<VisitorFn>,
    pub visit_definition_list_start_fn: Option<VisitorFn>,
    pub visit_definition_term_fn: Option<VisitorFn>,
    pub visit_definition_description_fn: Option<VisitorFn>,
    pub visit_definition_list_end_fn: Option<VisitorFn>,
    pub visit_form_fn: Option<VisitorFn>,
    pub visit_input_fn: Option<VisitorFn>,
    pub visit_button_fn: Option<VisitorFn>,
    pub visit_audio_fn: Option<VisitorFn>,
    pub visit_video_fn: Option<VisitorFn>,
    pub visit_iframe_fn: Option<VisitorFn>,
    pub visit_details_fn: Option<VisitorFn>,
    pub visit_summary_fn: Option<VisitorFn>,
    pub visit_figure_start_fn: Option<VisitorFn>,
    pub visit_figcaption_fn: Option<VisitorFn>,
    pub visit_figure_end_fn: Option<VisitorFn>,
}

#[cfg(feature = "async-visitor")]
impl std::fmt::Debug for JsVisitorBridge {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("JsVisitorBridge").finish()
    }
}

#[cfg(feature = "async-visitor")]
unsafe impl Send for JsVisitorBridge {}

#[cfg(feature = "async-visitor")]
unsafe impl Sync for JsVisitorBridge {}

#[cfg(feature = "async-visitor")]
impl JsVisitorBridge {
    /// Create a new visitor bridge with no callbacks registered.
    #[allow(dead_code)]
    pub const fn new() -> Self {
        Self {
            visit_element_start_fn: None,
            visit_element_end_fn: None,
            visit_text_fn: None,
            visit_link_fn: None,
            visit_image_fn: None,
            visit_heading_fn: None,
            visit_code_block_fn: None,
            visit_code_inline_fn: None,
            visit_list_item_fn: None,
            visit_list_start_fn: None,
            visit_list_end_fn: None,
            visit_table_start_fn: None,
            visit_table_row_fn: None,
            visit_table_end_fn: None,
            visit_blockquote_fn: None,
            visit_strong_fn: None,
            visit_emphasis_fn: None,
            visit_strikethrough_fn: None,
            visit_underline_fn: None,
            visit_subscript_fn: None,
            visit_superscript_fn: None,
            visit_mark_fn: None,
            visit_line_break_fn: None,
            visit_horizontal_rule_fn: None,
            visit_custom_element_fn: None,
            visit_definition_list_start_fn: None,
            visit_definition_term_fn: None,
            visit_definition_description_fn: None,
            visit_definition_list_end_fn: None,
            visit_form_fn: None,
            visit_input_fn: None,
            visit_button_fn: None,
            visit_audio_fn: None,
            visit_video_fn: None,
            visit_iframe_fn: None,
            visit_details_fn: None,
            visit_summary_fn: None,
            visit_figure_start_fn: None,
            visit_figcaption_fn: None,
            visit_figure_end_fn: None,
        }
    }

    /// Convert a Rust NodeContext to a JS-compatible JsNodeContext.
    #[allow(dead_code)]
    fn node_context_to_js(ctx: &RustNodeContext) -> JsNodeContext {
        let mut attributes = HashMap::new();
        for (k, v) in &ctx.attributes {
            attributes.insert(k.clone(), v.clone());
        }

        JsNodeContext {
            node_type: format!("{:?}", ctx.node_type),
            tag_name: ctx.tag_name.clone(),
            attributes,
            depth: ctx.depth as u32,
            index_in_parent: ctx.index_in_parent as u32,
            parent_tag: ctx.parent_tag.clone(),
            is_inline: ctx.is_inline,
        }
    }

    /// Convert a JsVisitResult back to a Rust VisitResult.
    #[allow(dead_code)]
    fn visit_result_from_js(js_result: &JsVisitResult) -> RustVisitResult {
        match js_result.result_type.to_lowercase().as_str() {
            "continue" => RustVisitResult::Continue,
            "custom" => RustVisitResult::Custom(js_result.output.clone().unwrap_or_default()),
            "skip" => RustVisitResult::Skip,
            "preservehtml" => RustVisitResult::PreserveHtml,
            "error" => RustVisitResult::Error(js_result.output.clone().unwrap_or_else(|| "Unknown error".to_string())),
            _ => RustVisitResult::Continue,
        }
    }

    /// Serialize visitor parameters to JSON string.
    #[allow(dead_code)]
    fn serialize_params<T: serde::Serialize>(params: &T) -> std::result::Result<String, serde_json::Error> {
        serde_json::to_string(params)
    }

    /// Deserialize visitor result from JSON string.
    #[allow(dead_code)]
    fn deserialize_result(json: &str) -> std::result::Result<JsVisitResult, serde_json::Error> {
        serde_json::from_str(json)
    }

    /// Call a visitor function with the given JSON params and return the result.
    #[allow(dead_code)]
    async fn call_visitor(&self, tsfn: &VisitorFn, json_params: String) -> RustVisitResult {
        let Ok(promise) = tsfn.call_async(json_params).await else {
            return RustVisitResult::Continue;
        };

        let Ok(result_json) = promise.await else {
            return RustVisitResult::Continue;
        };

        let Ok(js_result) = Self::deserialize_result(&result_json) else {
            return RustVisitResult::Continue;
        };

        Self::visit_result_from_js(&js_result)
    }
}

#[cfg(feature = "async-visitor")]
#[async_trait]
impl AsyncHtmlVisitor for JsVisitorBridge {
    async fn visit_element_start(&mut self, ctx: &RustNodeContext) -> RustVisitResult {
        let Some(ref tsfn) = self.visit_element_start_fn else {
            return RustVisitResult::Continue;
        };
        let js_ctx = Self::node_context_to_js(ctx);
        let Ok(json_params) = Self::serialize_params(&js_ctx) else {
            return RustVisitResult::Continue;
        };
        self.call_visitor(tsfn, json_params).await
    }

    async fn visit_element_end(&mut self, ctx: &RustNodeContext, output: &str) -> RustVisitResult {
        let Some(ref tsfn) = self.visit_element_end_fn else {
            return RustVisitResult::Continue;
        };
        let params = ElementEndParams {
            context: Self::node_context_to_js(ctx),
            output: output.to_string(),
        };
        let Ok(json_params) = Self::serialize_params(&params) else {
            return RustVisitResult::Continue;
        };
        self.call_visitor(tsfn, json_params).await
    }

    async fn visit_text(&mut self, ctx: &RustNodeContext, text: &str) -> RustVisitResult {
        let Some(ref tsfn) = self.visit_text_fn else {
            return RustVisitResult::Continue;
        };
        let params = TextParams {
            context: Self::node_context_to_js(ctx),
            text: text.to_string(),
        };
        let Ok(json_params) = Self::serialize_params(&params) else {
            return RustVisitResult::Continue;
        };
        self.call_visitor(tsfn, json_params).await
    }

    async fn visit_link(
        &mut self,
        ctx: &RustNodeContext,
        href: &str,
        text: &str,
        title: Option<&str>,
    ) -> RustVisitResult {
        let Some(ref tsfn) = self.visit_link_fn else {
            return RustVisitResult::Continue;
        };
        let params = LinkParams {
            context: Self::node_context_to_js(ctx),
            href: href.to_string(),
            text: text.to_string(),
            title: title.map(|s| s.to_string()),
        };
        let Ok(json_params) = Self::serialize_params(&params) else {
            return RustVisitResult::Continue;
        };
        self.call_visitor(tsfn, json_params).await
    }

    async fn visit_image(
        &mut self,
        ctx: &RustNodeContext,
        src: &str,
        alt: &str,
        title: Option<&str>,
    ) -> RustVisitResult {
        let Some(ref tsfn) = self.visit_image_fn else {
            return RustVisitResult::Continue;
        };
        let params = ImageParams {
            context: Self::node_context_to_js(ctx),
            src: src.to_string(),
            alt: alt.to_string(),
            title: title.map(|s| s.to_string()),
        };
        let Ok(json_params) = Self::serialize_params(&params) else {
            return RustVisitResult::Continue;
        };
        self.call_visitor(tsfn, json_params).await
    }

    async fn visit_heading(
        &mut self,
        ctx: &RustNodeContext,
        level: u32,
        text: &str,
        id: Option<&str>,
    ) -> RustVisitResult {
        let Some(ref tsfn) = self.visit_heading_fn else {
            return RustVisitResult::Continue;
        };
        let params = HeadingParams {
            context: Self::node_context_to_js(ctx),
            level,
            text: text.to_string(),
            id: id.map(|s| s.to_string()),
        };
        let Ok(json_params) = Self::serialize_params(&params) else {
            return RustVisitResult::Continue;
        };
        self.call_visitor(tsfn, json_params).await
    }

    async fn visit_code_block(&mut self, ctx: &RustNodeContext, lang: Option<&str>, code: &str) -> RustVisitResult {
        let Some(ref tsfn) = self.visit_code_block_fn else {
            return RustVisitResult::Continue;
        };
        let params = CodeBlockParams {
            context: Self::node_context_to_js(ctx),
            lang: lang.map(|s| s.to_string()),
            code: code.to_string(),
        };
        let Ok(json_params) = Self::serialize_params(&params) else {
            return RustVisitResult::Continue;
        };
        self.call_visitor(tsfn, json_params).await
    }

    async fn visit_code_inline(&mut self, ctx: &RustNodeContext, code: &str) -> RustVisitResult {
        let Some(ref tsfn) = self.visit_code_inline_fn else {
            return RustVisitResult::Continue;
        };
        let params = CodeInlineParams {
            context: Self::node_context_to_js(ctx),
            code: code.to_string(),
        };
        let Ok(json_params) = Self::serialize_params(&params) else {
            return RustVisitResult::Continue;
        };
        self.call_visitor(tsfn, json_params).await
    }

    async fn visit_list_item(
        &mut self,
        ctx: &RustNodeContext,
        ordered: bool,
        marker: &str,
        text: &str,
    ) -> RustVisitResult {
        let Some(ref tsfn) = self.visit_list_item_fn else {
            return RustVisitResult::Continue;
        };
        let params = ListItemParams {
            context: Self::node_context_to_js(ctx),
            ordered,
            marker: marker.to_string(),
            text: text.to_string(),
        };
        let Ok(json_params) = Self::serialize_params(&params) else {
            return RustVisitResult::Continue;
        };
        self.call_visitor(tsfn, json_params).await
    }

    async fn visit_list_start(&mut self, ctx: &RustNodeContext, ordered: bool) -> RustVisitResult {
        let Some(ref tsfn) = self.visit_list_start_fn else {
            return RustVisitResult::Continue;
        };
        let params = ListStartParams {
            context: Self::node_context_to_js(ctx),
            ordered,
        };
        let Ok(json_params) = Self::serialize_params(&params) else {
            return RustVisitResult::Continue;
        };
        self.call_visitor(tsfn, json_params).await
    }

    async fn visit_list_end(&mut self, ctx: &RustNodeContext, ordered: bool, output: &str) -> RustVisitResult {
        let Some(ref tsfn) = self.visit_list_end_fn else {
            return RustVisitResult::Continue;
        };
        let params = ListEndParams {
            context: Self::node_context_to_js(ctx),
            ordered,
            output: output.to_string(),
        };
        let Ok(json_params) = Self::serialize_params(&params) else {
            return RustVisitResult::Continue;
        };
        self.call_visitor(tsfn, json_params).await
    }

    async fn visit_table_start(&mut self, ctx: &RustNodeContext) -> RustVisitResult {
        let Some(ref tsfn) = self.visit_table_start_fn else {
            return RustVisitResult::Continue;
        };
        let js_ctx = Self::node_context_to_js(ctx);
        let Ok(json_params) = Self::serialize_params(&js_ctx) else {
            return RustVisitResult::Continue;
        };
        self.call_visitor(tsfn, json_params).await
    }

    async fn visit_table_row(&mut self, ctx: &RustNodeContext, cells: &[String], is_header: bool) -> RustVisitResult {
        let Some(ref tsfn) = self.visit_table_row_fn else {
            return RustVisitResult::Continue;
        };
        let params = TableRowParams {
            context: Self::node_context_to_js(ctx),
            cells: cells.to_vec(),
            is_header,
        };
        let Ok(json_params) = Self::serialize_params(&params) else {
            return RustVisitResult::Continue;
        };
        self.call_visitor(tsfn, json_params).await
    }

    async fn visit_table_end(&mut self, ctx: &RustNodeContext, output: &str) -> RustVisitResult {
        let Some(ref tsfn) = self.visit_table_end_fn else {
            return RustVisitResult::Continue;
        };
        let params = TableEndParams {
            context: Self::node_context_to_js(ctx),
            output: output.to_string(),
        };
        let Ok(json_params) = Self::serialize_params(&params) else {
            return RustVisitResult::Continue;
        };
        self.call_visitor(tsfn, json_params).await
    }

    async fn visit_blockquote(&mut self, ctx: &RustNodeContext, content: &str, depth: usize) -> RustVisitResult {
        let Some(ref tsfn) = self.visit_blockquote_fn else {
            return RustVisitResult::Continue;
        };
        let params = BlockquoteParams {
            context: Self::node_context_to_js(ctx),
            content: content.to_string(),
            depth,
        };
        let Ok(json_params) = Self::serialize_params(&params) else {
            return RustVisitResult::Continue;
        };
        self.call_visitor(tsfn, json_params).await
    }

    async fn visit_strong(&mut self, ctx: &RustNodeContext, text: &str) -> RustVisitResult {
        let Some(ref tsfn) = self.visit_strong_fn else {
            return RustVisitResult::Continue;
        };
        let params = TextParams {
            context: Self::node_context_to_js(ctx),
            text: text.to_string(),
        };
        let Ok(json_params) = Self::serialize_params(&params) else {
            return RustVisitResult::Continue;
        };
        self.call_visitor(tsfn, json_params).await
    }

    async fn visit_emphasis(&mut self, ctx: &RustNodeContext, text: &str) -> RustVisitResult {
        let Some(ref tsfn) = self.visit_emphasis_fn else {
            return RustVisitResult::Continue;
        };
        let params = TextParams {
            context: Self::node_context_to_js(ctx),
            text: text.to_string(),
        };
        let Ok(json_params) = Self::serialize_params(&params) else {
            return RustVisitResult::Continue;
        };
        self.call_visitor(tsfn, json_params).await
    }

    async fn visit_strikethrough(&mut self, ctx: &RustNodeContext, text: &str) -> RustVisitResult {
        let Some(ref tsfn) = self.visit_strikethrough_fn else {
            return RustVisitResult::Continue;
        };
        let params = TextParams {
            context: Self::node_context_to_js(ctx),
            text: text.to_string(),
        };
        let Ok(json_params) = Self::serialize_params(&params) else {
            return RustVisitResult::Continue;
        };
        self.call_visitor(tsfn, json_params).await
    }

    async fn visit_underline(&mut self, ctx: &RustNodeContext, text: &str) -> RustVisitResult {
        let Some(ref tsfn) = self.visit_underline_fn else {
            return RustVisitResult::Continue;
        };
        let params = TextParams {
            context: Self::node_context_to_js(ctx),
            text: text.to_string(),
        };
        let Ok(json_params) = Self::serialize_params(&params) else {
            return RustVisitResult::Continue;
        };
        self.call_visitor(tsfn, json_params).await
    }

    async fn visit_subscript(&mut self, ctx: &RustNodeContext, text: &str) -> RustVisitResult {
        let Some(ref tsfn) = self.visit_subscript_fn else {
            return RustVisitResult::Continue;
        };
        let params = TextParams {
            context: Self::node_context_to_js(ctx),
            text: text.to_string(),
        };
        let Ok(json_params) = Self::serialize_params(&params) else {
            return RustVisitResult::Continue;
        };
        self.call_visitor(tsfn, json_params).await
    }

    async fn visit_superscript(&mut self, ctx: &RustNodeContext, text: &str) -> RustVisitResult {
        let Some(ref tsfn) = self.visit_superscript_fn else {
            return RustVisitResult::Continue;
        };
        let params = TextParams {
            context: Self::node_context_to_js(ctx),
            text: text.to_string(),
        };
        let Ok(json_params) = Self::serialize_params(&params) else {
            return RustVisitResult::Continue;
        };
        self.call_visitor(tsfn, json_params).await
    }

    async fn visit_mark(&mut self, ctx: &RustNodeContext, text: &str) -> RustVisitResult {
        let Some(ref tsfn) = self.visit_mark_fn else {
            return RustVisitResult::Continue;
        };
        let params = TextParams {
            context: Self::node_context_to_js(ctx),
            text: text.to_string(),
        };
        let Ok(json_params) = Self::serialize_params(&params) else {
            return RustVisitResult::Continue;
        };
        self.call_visitor(tsfn, json_params).await
    }

    async fn visit_line_break(&mut self, ctx: &RustNodeContext) -> RustVisitResult {
        let Some(ref tsfn) = self.visit_line_break_fn else {
            return RustVisitResult::Continue;
        };
        let js_ctx = Self::node_context_to_js(ctx);
        let Ok(json_params) = Self::serialize_params(&js_ctx) else {
            return RustVisitResult::Continue;
        };
        self.call_visitor(tsfn, json_params).await
    }

    async fn visit_horizontal_rule(&mut self, ctx: &RustNodeContext) -> RustVisitResult {
        let Some(ref tsfn) = self.visit_horizontal_rule_fn else {
            return RustVisitResult::Continue;
        };
        let js_ctx = Self::node_context_to_js(ctx);
        let Ok(json_params) = Self::serialize_params(&js_ctx) else {
            return RustVisitResult::Continue;
        };
        self.call_visitor(tsfn, json_params).await
    }

    async fn visit_custom_element(&mut self, ctx: &RustNodeContext, tag_name: &str, html: &str) -> RustVisitResult {
        let Some(ref tsfn) = self.visit_custom_element_fn else {
            return RustVisitResult::Continue;
        };
        let params = CustomElementParams {
            context: Self::node_context_to_js(ctx),
            tag_name: tag_name.to_string(),
            html: html.to_string(),
        };
        let Ok(json_params) = Self::serialize_params(&params) else {
            return RustVisitResult::Continue;
        };
        self.call_visitor(tsfn, json_params).await
    }

    async fn visit_definition_list_start(&mut self, ctx: &RustNodeContext) -> RustVisitResult {
        let Some(ref tsfn) = self.visit_definition_list_start_fn else {
            return RustVisitResult::Continue;
        };
        let js_ctx = Self::node_context_to_js(ctx);
        let Ok(json_params) = Self::serialize_params(&js_ctx) else {
            return RustVisitResult::Continue;
        };
        self.call_visitor(tsfn, json_params).await
    }

    async fn visit_definition_term(&mut self, ctx: &RustNodeContext, text: &str) -> RustVisitResult {
        let Some(ref tsfn) = self.visit_definition_term_fn else {
            return RustVisitResult::Continue;
        };
        let params = TextParams {
            context: Self::node_context_to_js(ctx),
            text: text.to_string(),
        };
        let Ok(json_params) = Self::serialize_params(&params) else {
            return RustVisitResult::Continue;
        };
        self.call_visitor(tsfn, json_params).await
    }

    async fn visit_definition_description(&mut self, ctx: &RustNodeContext, text: &str) -> RustVisitResult {
        let Some(ref tsfn) = self.visit_definition_description_fn else {
            return RustVisitResult::Continue;
        };
        let params = TextParams {
            context: Self::node_context_to_js(ctx),
            text: text.to_string(),
        };
        let Ok(json_params) = Self::serialize_params(&params) else {
            return RustVisitResult::Continue;
        };
        self.call_visitor(tsfn, json_params).await
    }

    async fn visit_definition_list_end(&mut self, ctx: &RustNodeContext, output: &str) -> RustVisitResult {
        let Some(ref tsfn) = self.visit_definition_list_end_fn else {
            return RustVisitResult::Continue;
        };
        let params = DefinitionListEndParams {
            context: Self::node_context_to_js(ctx),
            output: output.to_string(),
        };
        let Ok(json_params) = Self::serialize_params(&params) else {
            return RustVisitResult::Continue;
        };
        self.call_visitor(tsfn, json_params).await
    }

    async fn visit_form(
        &mut self,
        ctx: &RustNodeContext,
        action: Option<&str>,
        method: Option<&str>,
    ) -> RustVisitResult {
        let Some(ref tsfn) = self.visit_form_fn else {
            return RustVisitResult::Continue;
        };
        let params = FormParams {
            context: Self::node_context_to_js(ctx),
            action: action.map(|s| s.to_string()),
            method: method.map(|s| s.to_string()),
        };
        let Ok(json_params) = Self::serialize_params(&params) else {
            return RustVisitResult::Continue;
        };
        self.call_visitor(tsfn, json_params).await
    }

    async fn visit_input(
        &mut self,
        ctx: &RustNodeContext,
        input_type: &str,
        name: Option<&str>,
        value: Option<&str>,
    ) -> RustVisitResult {
        let Some(ref tsfn) = self.visit_input_fn else {
            return RustVisitResult::Continue;
        };
        let params = InputParams {
            context: Self::node_context_to_js(ctx),
            input_type: input_type.to_string(),
            name: name.map(|s| s.to_string()),
            value: value.map(|s| s.to_string()),
        };
        let Ok(json_params) = Self::serialize_params(&params) else {
            return RustVisitResult::Continue;
        };
        self.call_visitor(tsfn, json_params).await
    }

    async fn visit_button(&mut self, ctx: &RustNodeContext, text: &str) -> RustVisitResult {
        let Some(ref tsfn) = self.visit_button_fn else {
            return RustVisitResult::Continue;
        };
        let params = TextParams {
            context: Self::node_context_to_js(ctx),
            text: text.to_string(),
        };
        let Ok(json_params) = Self::serialize_params(&params) else {
            return RustVisitResult::Continue;
        };
        self.call_visitor(tsfn, json_params).await
    }

    async fn visit_audio(&mut self, ctx: &RustNodeContext, src: Option<&str>) -> RustVisitResult {
        let Some(ref tsfn) = self.visit_audio_fn else {
            return RustVisitResult::Continue;
        };
        let params = MediaParams {
            context: Self::node_context_to_js(ctx),
            src: src.map(|s| s.to_string()),
        };
        let Ok(json_params) = Self::serialize_params(&params) else {
            return RustVisitResult::Continue;
        };
        self.call_visitor(tsfn, json_params).await
    }

    async fn visit_video(&mut self, ctx: &RustNodeContext, src: Option<&str>) -> RustVisitResult {
        let Some(ref tsfn) = self.visit_video_fn else {
            return RustVisitResult::Continue;
        };
        let params = MediaParams {
            context: Self::node_context_to_js(ctx),
            src: src.map(|s| s.to_string()),
        };
        let Ok(json_params) = Self::serialize_params(&params) else {
            return RustVisitResult::Continue;
        };
        self.call_visitor(tsfn, json_params).await
    }

    async fn visit_iframe(&mut self, ctx: &RustNodeContext, src: Option<&str>) -> RustVisitResult {
        let Some(ref tsfn) = self.visit_iframe_fn else {
            return RustVisitResult::Continue;
        };
        let params = MediaParams {
            context: Self::node_context_to_js(ctx),
            src: src.map(|s| s.to_string()),
        };
        let Ok(json_params) = Self::serialize_params(&params) else {
            return RustVisitResult::Continue;
        };
        self.call_visitor(tsfn, json_params).await
    }

    async fn visit_details(&mut self, ctx: &RustNodeContext, open: bool) -> RustVisitResult {
        let Some(ref tsfn) = self.visit_details_fn else {
            return RustVisitResult::Continue;
        };
        let params = DetailsParams {
            context: Self::node_context_to_js(ctx),
            open,
        };
        let Ok(json_params) = Self::serialize_params(&params) else {
            return RustVisitResult::Continue;
        };
        self.call_visitor(tsfn, json_params).await
    }

    async fn visit_summary(&mut self, ctx: &RustNodeContext, text: &str) -> RustVisitResult {
        let Some(ref tsfn) = self.visit_summary_fn else {
            return RustVisitResult::Continue;
        };
        let params = TextParams {
            context: Self::node_context_to_js(ctx),
            text: text.to_string(),
        };
        let Ok(json_params) = Self::serialize_params(&params) else {
            return RustVisitResult::Continue;
        };
        self.call_visitor(tsfn, json_params).await
    }

    async fn visit_figure_start(&mut self, ctx: &RustNodeContext) -> RustVisitResult {
        let Some(ref tsfn) = self.visit_figure_start_fn else {
            return RustVisitResult::Continue;
        };
        let js_ctx = Self::node_context_to_js(ctx);
        let Ok(json_params) = Self::serialize_params(&js_ctx) else {
            return RustVisitResult::Continue;
        };
        self.call_visitor(tsfn, json_params).await
    }

    async fn visit_figcaption(&mut self, ctx: &RustNodeContext, text: &str) -> RustVisitResult {
        let Some(ref tsfn) = self.visit_figcaption_fn else {
            return RustVisitResult::Continue;
        };
        let params = TextParams {
            context: Self::node_context_to_js(ctx),
            text: text.to_string(),
        };
        let Ok(json_params) = Self::serialize_params(&params) else {
            return RustVisitResult::Continue;
        };
        self.call_visitor(tsfn, json_params).await
    }

    async fn visit_figure_end(&mut self, ctx: &RustNodeContext, output: &str) -> RustVisitResult {
        let Some(ref tsfn) = self.visit_figure_end_fn else {
            return RustVisitResult::Continue;
        };
        let params = FigureEndParams {
            context: Self::node_context_to_js(ctx),
            output: output.to_string(),
        };
        let Ok(json_params) = Self::serialize_params(&params) else {
            return RustVisitResult::Continue;
        };
        self.call_visitor(tsfn, json_params).await
    }
}
