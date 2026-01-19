use napi_derive::napi;
use std::collections::HashMap;
use std::sync::Arc;

/// Node context for visitor callbacks
#[cfg(feature = "async-visitor")]
#[napi(object)]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JsNodeContext {
    pub node_type: String,
    pub tag_name: String,
    pub attributes: HashMap<String, String>,
    pub depth: u32,
    pub index_in_parent: u32,
    pub parent_tag: Option<String>,
    pub is_inline: bool,
}

/// Result of visitor callback
#[cfg(feature = "async-visitor")]
#[napi(object)]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JsVisitResult {
    #[napi(js_name = "type")]
    #[serde(rename = "type")]
    pub result_type: String,
    pub output: Option<String>,
}

/// Type alias for `ThreadsafeFunction` wrapping visitor callbacks
///
/// Each visitor method callback is wrapped as an Arc-based `ThreadsafeFunction`
/// that accepts a String parameter (JSON) and returns a Promise<String> (JSON result).
#[cfg(feature = "async-visitor")]
pub type VisitorFn = Arc<
    napi::threadsafe_function::ThreadsafeFunction<
        String,
        napi::bindgen_prelude::Promise<String>,
        String,
        napi::Status,
        false,
    >,
>;
