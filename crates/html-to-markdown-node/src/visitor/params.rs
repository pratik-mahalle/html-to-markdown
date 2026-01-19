//! Parameter structs for visitor callback serialization.

use super::types::JsNodeContext;
use serde::Serialize;

/// Parameters for element end callbacks.
#[derive(Serialize)]
pub struct ElementEndParams {
    pub context: JsNodeContext,
    pub output: String,
}

/// Parameters for text-based callbacks.
#[derive(Serialize)]
pub struct TextParams {
    pub context: JsNodeContext,
    pub text: String,
}

/// Parameters for link callbacks.
#[derive(Serialize)]
pub struct LinkParams {
    pub context: JsNodeContext,
    pub href: String,
    pub text: String,
    pub title: Option<String>,
}

/// Parameters for image callbacks.
#[derive(Serialize)]
pub struct ImageParams {
    pub context: JsNodeContext,
    pub src: String,
    pub alt: String,
    pub title: Option<String>,
}

/// Parameters for heading callbacks.
#[derive(Serialize)]
pub struct HeadingParams {
    pub context: JsNodeContext,
    pub level: u32,
    pub text: String,
    pub id: Option<String>,
}

/// Parameters for code block callbacks.
#[derive(Serialize)]
pub struct CodeBlockParams {
    pub context: JsNodeContext,
    pub lang: Option<String>,
    pub code: String,
}

/// Parameters for code inline callbacks.
#[derive(Serialize)]
pub struct CodeInlineParams {
    pub context: JsNodeContext,
    pub code: String,
}

/// Parameters for list item callbacks.
#[derive(Serialize)]
pub struct ListItemParams {
    pub context: JsNodeContext,
    pub ordered: bool,
    pub marker: String,
    pub text: String,
}

/// Parameters for list start callbacks.
#[derive(Serialize)]
pub struct ListStartParams {
    pub context: JsNodeContext,
    pub ordered: bool,
}

/// Parameters for list end callbacks.
#[derive(Serialize)]
pub struct ListEndParams {
    pub context: JsNodeContext,
    pub ordered: bool,
    pub output: String,
}

/// Parameters for table row callbacks.
#[derive(Serialize)]
pub struct TableRowParams {
    pub context: JsNodeContext,
    pub cells: Vec<String>,
    pub is_header: bool,
}

/// Parameters for table end callbacks.
#[derive(Serialize)]
pub struct TableEndParams {
    pub context: JsNodeContext,
    pub output: String,
}

/// Parameters for blockquote callbacks.
#[derive(Serialize)]
pub struct BlockquoteParams {
    pub context: JsNodeContext,
    pub content: String,
    pub depth: usize,
}

/// Parameters for custom element callbacks.
#[derive(Serialize)]
pub struct CustomElementParams {
    pub context: JsNodeContext,
    pub tag_name: String,
    pub html: String,
}

/// Parameters for definition list end callbacks.
#[derive(Serialize)]
pub struct DefinitionListEndParams {
    pub context: JsNodeContext,
    pub output: String,
}

/// Parameters for form callbacks.
#[derive(Serialize)]
pub struct FormParams {
    pub context: JsNodeContext,
    pub action: Option<String>,
    pub method: Option<String>,
}

/// Parameters for input callbacks.
#[derive(Serialize)]
pub struct InputParams {
    pub context: JsNodeContext,
    pub input_type: String,
    pub name: Option<String>,
    pub value: Option<String>,
}

/// Parameters for media callbacks (audio, video, iframe).
#[derive(Serialize)]
pub struct MediaParams {
    pub context: JsNodeContext,
    pub src: Option<String>,
}

/// Parameters for details callbacks.
#[derive(Serialize)]
pub struct DetailsParams {
    pub context: JsNodeContext,
    pub open: bool,
}

/// Parameters for figure end callbacks.
#[derive(Serialize)]
pub struct FigureEndParams {
    pub context: JsNodeContext,
    pub output: String,
}
