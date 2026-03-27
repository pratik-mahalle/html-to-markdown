//! NIF map type definitions for Elixir bindings.

use rustler::NifMap;
use rustler::types::binary::Binary;
use std::collections::HashMap;

#[derive(NifMap)]
pub struct InlineImageWarningTerm {
    pub index: i64,
    pub message: String,
}

#[derive(NifMap)]
pub struct InlineImageTerm<'a> {
    pub data: Binary<'a>,
    pub format: String,
    pub filename: Option<String>,
    pub description: Option<String>,
    pub dimensions: Option<(u32, u32)>,
    pub source: String,
    pub attributes: HashMap<String, String>,
}

#[derive(NifMap)]
pub struct DocumentMetadataTerm {
    pub title: Option<String>,
    pub description: Option<String>,
    pub keywords: Vec<String>,
    pub author: Option<String>,
    pub canonical_url: Option<String>,
    pub base_href: Option<String>,
    pub language: Option<String>,
    pub text_direction: Option<String>,
    pub open_graph: HashMap<String, String>,
    pub twitter_card: HashMap<String, String>,
    pub meta_tags: HashMap<String, String>,
}

#[derive(NifMap)]
pub struct HeaderMetadataTerm {
    pub level: u8,
    pub text: String,
    pub id: Option<String>,
    pub depth: u64,
    pub html_offset: u64,
}

#[derive(NifMap)]
pub struct LinkMetadataTerm {
    pub href: String,
    pub text: String,
    pub title: Option<String>,
    pub link_type: String,
    pub rel: Vec<String>,
    pub attributes: HashMap<String, String>,
}

#[derive(NifMap)]
pub struct ImageMetadataTerm {
    pub src: String,
    pub alt: Option<String>,
    pub title: Option<String>,
    pub dimensions: Option<(u32, u32)>,
    pub image_type: String,
    pub attributes: HashMap<String, String>,
}

#[derive(NifMap)]
pub struct StructuredDataTerm {
    pub data_type: String,
    pub raw_json: String,
    pub schema_type: Option<String>,
}

#[derive(NifMap)]
pub struct ExtendedMetadataTerm {
    pub document: DocumentMetadataTerm,
    pub headers: Vec<HeaderMetadataTerm>,
    pub links: Vec<LinkMetadataTerm>,
    pub images: Vec<ImageMetadataTerm>,
    pub structured_data: Vec<StructuredDataTerm>,
}

#[derive(NifMap)]
pub struct TableDataTerm {
    pub cells: Vec<Vec<String>>,
    pub markdown: String,
    pub is_header_row: Vec<bool>,
}

#[derive(NifMap)]
pub struct TableExtractionTerm {
    pub content: String,
    pub metadata: Option<ExtendedMetadataTerm>,
    pub tables: Vec<TableDataTerm>,
}

#[derive(NifMap)]
pub struct GridCellTerm {
    pub content: String,
    pub row: u32,
    pub col: u32,
    pub row_span: u32,
    pub col_span: u32,
    pub is_header: bool,
}

#[derive(NifMap)]
pub struct TableGridTerm {
    pub rows: u32,
    pub cols: u32,
    pub cells: Vec<GridCellTerm>,
}

#[derive(NifMap)]
pub struct ExtractTableTerm {
    pub grid: TableGridTerm,
    pub markdown: String,
}

#[derive(NifMap)]
pub struct WarningTerm {
    pub message: String,
    pub kind: String,
}

#[derive(NifMap)]
pub struct ConversionResultTerm {
    pub content: Option<String>,
    pub metadata: ExtendedMetadataTerm,
    pub tables: Vec<ExtractTableTerm>,
    pub warnings: Vec<WarningTerm>,
}
