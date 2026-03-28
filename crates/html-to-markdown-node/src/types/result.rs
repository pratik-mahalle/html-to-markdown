use napi::bindgen_prelude::*;
use napi_derive::napi;

/// A single cell in a structured table grid.
#[napi(object)]
pub struct JsGridCell {
    pub content: String,
    pub row: u32,
    pub col: u32,
    pub row_span: u32,
    pub col_span: u32,
    pub is_header: bool,
}

/// Structured table grid with cell-level data.
#[napi(object)]
pub struct JsTableGrid {
    pub rows: u32,
    pub cols: u32,
    pub cells: Vec<JsGridCell>,
}

/// A table extracted by the v3 `convert()` API.
#[napi(object)]
pub struct JsConversionTable {
    pub grid: JsTableGrid,
    pub markdown: String,
}

/// Non-fatal warning emitted during conversion.
#[napi(object)]
pub struct JsConversionWarning {
    /// Human-readable warning message.
    pub message: String,
    /// Warning kind identifier.
    pub kind: String,
}

/// Result of the v3 `convert()` API.
#[napi(object)]
pub struct JsConversionResult {
    /// Converted text output (markdown, djot, or plain text). Null when output is suppressed.
    pub content: Option<String>,
    /// Structured document tree serialized as a JSON string, or null.
    pub document: Option<String>,
    /// Extracted HTML metadata serialized as a JSON string, or null.
    pub metadata: Option<String>,
    /// All tables found in the HTML, in document order.
    pub tables: Vec<JsConversionTable>,
    /// Non-fatal processing warnings.
    pub warnings: Vec<JsConversionWarning>,
}

/// Build a `JsConversionResult` from a Rust `ConversionResult`.
pub fn build_conversion_result(result: html_to_markdown_rs::ConversionResult) -> Result<JsConversionResult> {
    let document = match result.document {
        Some(doc) => {
            let s = serde_json::to_string(&doc).map_err(|e| Error::new(Status::GenericFailure, e.to_string()))?;
            Some(s)
        }
        None => None,
    };

    #[cfg(feature = "metadata")]
    let metadata = {
        let s =
            serde_json::to_string(&result.metadata).map_err(|e| Error::new(Status::GenericFailure, e.to_string()))?;
        Some(s)
    };
    #[cfg(not(feature = "metadata"))]
    let metadata: Option<String> = None;

    let tables = result
        .tables
        .into_iter()
        .map(|t| JsConversionTable {
            grid: JsTableGrid {
                rows: t.grid.rows,
                cols: t.grid.cols,
                cells: t
                    .grid
                    .cells
                    .into_iter()
                    .map(|c| JsGridCell {
                        content: c.content,
                        row: c.row,
                        col: c.col,
                        row_span: c.row_span,
                        col_span: c.col_span,
                        is_header: c.is_header,
                    })
                    .collect(),
            },
            markdown: t.markdown,
        })
        .collect();

    let warnings = result
        .warnings
        .into_iter()
        .map(|w| JsConversionWarning {
            message: w.message,
            kind: format!("{:?}", w.kind),
        })
        .collect();

    Ok(JsConversionResult {
        content: result.content,
        document,
        metadata,
        tables,
        warnings,
    })
}
