//! R list type conversions for binding results.

use extendr_api::prelude::*;
use html_to_markdown_rs::metadata::{
    DocumentMetadata, HeaderMetadata, HtmlMetadata, ImageMetadata, LinkMetadata, StructuredData,
};
use html_to_markdown_rs::{ConversionWithTables, InlineImage, TableData};
use std::collections::HashMap;

/// Convert HtmlMetadata into an R list.
pub fn metadata_to_robj(metadata: HtmlMetadata) -> Robj {
    list!(
        document = document_metadata_to_robj(metadata.document),
        headers = List::from_values(metadata.headers.into_iter().map(header_metadata_to_robj)),
        links = List::from_values(metadata.links.into_iter().map(link_metadata_to_robj)),
        images = List::from_values(metadata.images.into_iter().map(image_metadata_to_robj)),
        structured_data = List::from_values(metadata.structured_data.into_iter().map(structured_data_to_robj))
    )
    .into()
}

fn document_metadata_to_robj(doc: DocumentMetadata) -> Robj {
    list!(
        title = option_to_robj(doc.title),
        description = option_to_robj(doc.description),
        keywords = doc.keywords,
        author = option_to_robj(doc.author),
        canonical_url = option_to_robj(doc.canonical_url),
        base_href = option_to_robj(doc.base_href),
        language = option_to_robj(doc.language),
        text_direction = option_to_robj(doc.text_direction.map(|td| td.to_string())),
        open_graph = hashmap_to_robj(doc.open_graph.into_iter().collect()),
        twitter_card = hashmap_to_robj(doc.twitter_card.into_iter().collect()),
        meta_tags = hashmap_to_robj(doc.meta_tags.into_iter().collect())
    )
    .into()
}

fn header_metadata_to_robj(header: HeaderMetadata) -> Robj {
    list!(
        level = header.level as i32,
        text = header.text,
        id = option_to_robj(header.id),
        depth = header.depth as i32,
        html_offset = header.html_offset as i32
    )
    .into()
}

fn link_metadata_to_robj(link: LinkMetadata) -> Robj {
    list!(
        href = link.href,
        text = link.text,
        title = option_to_robj(link.title),
        link_type = link.link_type.to_string(),
        rel = link.rel,
        attributes = hashmap_to_robj(link.attributes.into_iter().collect())
    )
    .into()
}

fn image_metadata_to_robj(image: ImageMetadata) -> Robj {
    list!(
        src = image.src,
        alt = option_to_robj(image.alt),
        title = option_to_robj(image.title),
        dimensions = match image.dimensions {
            Some((w, h)) => Robj::from(list!(width = w as i32, height = h as i32)),
            None => ().into(),
        },
        image_type = image.image_type.to_string(),
        attributes = hashmap_to_robj(image.attributes.into_iter().collect())
    )
    .into()
}

fn structured_data_to_robj(data: StructuredData) -> Robj {
    list!(
        data_type = data.data_type.to_string(),
        raw_json = data.raw_json,
        schema_type = option_to_robj(data.schema_type)
    )
    .into()
}

/// Convert an InlineImage into an R list.
pub fn inline_image_to_robj(image: InlineImage) -> Robj {
    list!(
        data = image.data,
        format = image.format.to_string(),
        filename = option_to_robj(image.filename),
        description = option_to_robj(image.description),
        dimensions = match image.dimensions {
            Some((w, h)) => Robj::from(list!(width = w as i32, height = h as i32)),
            None => ().into(),
        },
        source = image.source.to_string(),
        attributes = hashmap_to_robj(image.attributes.into_iter().collect())
    )
    .into()
}

/// Convert an inline image warning into an R list.
pub fn inline_image_warning_to_robj(index: usize, message: String) -> Robj {
    Robj::from(list!(index = index as i32, message = message))
}

fn option_to_robj(opt: Option<String>) -> Robj {
    match opt {
        Some(s) => s.into(),
        None => ().into(),
    }
}

fn hashmap_to_robj(map: HashMap<String, String>) -> Robj {
    let names: Vec<&str> = map.keys().map(|k| k.as_str()).collect();
    let values: Vec<Robj> = map.values().map(|v| v.into_robj()).collect();
    let mut list = List::from_values(values);
    let _ = list.set_names(names);
    list.into()
}

/// Convert a TableData into an R list.
pub fn table_data_to_robj(table: TableData) -> Robj {
    let cells: Vec<Robj> = table.cells.into_iter().map(|row| Robj::from(row)).collect();

    list!(
        cells = List::from_values(cells),
        markdown = table.markdown,
        is_header_row = table.is_header_row
    )
    .into()
}

/// Convert a ConversionResult into an R list.
pub fn conversion_result_to_robj(result: html_to_markdown_rs::ConversionResult) -> Robj {
    let content_robj: Robj = match result.content {
        Some(s) => s.into(),
        None => ().into(),
    };

    #[cfg(feature = "metadata")]
    let metadata_robj = metadata_to_robj(result.metadata);
    #[cfg(not(feature = "metadata"))]
    let metadata_robj: Robj = ().into();

    let tables: Vec<Robj> = result
        .tables
        .into_iter()
        .map(|t| {
            let cells: Vec<Robj> = t
                .grid
                .cells
                .into_iter()
                .map(|c| {
                    list!(
                        content = c.content,
                        row = c.row as i32,
                        col = c.col as i32,
                        row_span = c.row_span as i32,
                        col_span = c.col_span as i32,
                        is_header = c.is_header
                    )
                    .into()
                })
                .collect();
            list!(
                grid = list!(
                    rows = t.grid.rows as i32,
                    cols = t.grid.cols as i32,
                    cells = List::from_values(cells)
                ),
                markdown = t.markdown
            )
            .into()
        })
        .collect();

    let warnings: Vec<Robj> = result
        .warnings
        .into_iter()
        .map(|w| {
            let kind = match w.kind {
                html_to_markdown_rs::WarningKind::ImageExtractionFailed => {
                    "image_extraction_failed"
                }
                html_to_markdown_rs::WarningKind::EncodingFallback => "encoding_fallback",
                html_to_markdown_rs::WarningKind::TruncatedInput => "truncated_input",
                html_to_markdown_rs::WarningKind::MalformedHtml => "malformed_html",
                html_to_markdown_rs::WarningKind::SanitizationApplied => "sanitization_applied",
            };
            list!(message = w.message, kind = kind).into()
        })
        .collect();

    list!(
        content = content_robj,
        metadata = metadata_robj,
        tables = List::from_values(tables),
        warnings = List::from_values(warnings)
    )
    .into()
}

/// Convert a ConversionWithTables into an R list.
pub fn table_extraction_to_robj(result: ConversionWithTables) -> Robj {
    let tables: Vec<Robj> = result.tables.into_iter().map(table_data_to_robj).collect();

    let metadata_robj = match result.metadata {
        Some(meta) => metadata_to_robj(meta),
        None => ().into(),
    };

    list!(
        content = result.content,
        metadata = metadata_robj,
        tables = List::from_values(tables)
    )
    .into()
}
