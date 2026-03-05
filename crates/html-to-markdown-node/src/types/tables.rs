use html_to_markdown_rs::TableData as RustTableData;
use napi_derive::napi;

/// Extracted data from a single HTML `<table>` element.
#[napi(object)]
pub struct JsTableData {
    /// Table cells organized as rows x columns.
    pub cells: Vec<Vec<String>>,
    /// Complete rendered table in the target output format.
    pub markdown: String,
    /// Per-row flag indicating whether the row was inside `<thead>`.
    pub is_header_row: Vec<bool>,
}

impl From<RustTableData> for JsTableData {
    fn from(val: RustTableData) -> Self {
        Self {
            cells: val.cells,
            markdown: val.markdown,
            is_header_row: val.is_header_row,
        }
    }
}

/// Result of HTML-to-Markdown conversion with extracted table data.
#[cfg(feature = "metadata")]
#[napi(object)]
pub struct JsTableExtraction {
    /// Converted markdown/djot/plain text content.
    pub content: String,
    /// Extended metadata (present when metadata extraction was requested).
    pub metadata: Option<super::metadata::JsExtendedMetadata>,
    /// All tables found in the HTML, in document order.
    pub tables: Vec<JsTableData>,
}

#[cfg(not(feature = "metadata"))]
#[napi(object)]
pub struct JsTableExtraction {
    /// Converted markdown/djot/plain text content.
    pub content: String,
    /// All tables found in the HTML, in document order.
    pub tables: Vec<JsTableData>,
}

/// Convert a vector of Rust `TableData` into JS-compatible `JsTableData`.
pub fn convert_tables(tables: Vec<RustTableData>) -> Vec<JsTableData> {
    tables.into_iter().map(Into::into).collect()
}
