use ext_php_rs::binary::Binary;
use ext_php_rs::boxed::ZBox;
use ext_php_rs::prelude::*;
use ext_php_rs::types::ZendHashTable;
#[cfg(feature = "metadata")]
use html_to_markdown_rs::metadata::{
    DocumentMetadata, HeaderMetadata, HtmlMetadata, ImageMetadata, LinkMetadata, StructuredData, TextDirection,
};
use html_to_markdown_rs::{HtmlExtraction, InlineImage, InlineImageWarning};
use std::collections::BTreeMap;

use crate::types::table_capacity;

/// Build a PHP hash table from an HtmlExtraction result.
pub fn build_html_extraction(extraction: HtmlExtraction) -> PhpResult<ZBox<ZendHashTable>> {
    let mut result = ZendHashTable::new();
    result.insert("markdown", extraction.markdown)?;
    result.insert("inline_images", build_inline_images(extraction.inline_images)?)?;
    result.insert("warnings", build_warnings(extraction.warnings)?)?;
    Ok(result)
}

/// Build a PHP hash table from inline images vector.
fn build_inline_images(images: Vec<InlineImage>) -> PhpResult<ZBox<ZendHashTable>> {
    let mut table = ZendHashTable::with_capacity(table_capacity(images.len()));

    for image in images {
        table.push(build_inline_image_entry(image)?)?;
    }

    Ok(table)
}

/// Build a PHP hash table for a single inline image entry.
fn build_inline_image_entry(image: InlineImage) -> PhpResult<ZBox<ZendHashTable>> {
    let mut entry = ZendHashTable::new();
    entry.insert("data", Binary::from(image.data))?;
    entry.insert("format", image.format.to_string())?;

    match image.filename {
        Some(filename) => entry.insert("filename", filename)?,
        None => entry.insert("filename", ())?,
    }
    match image.description {
        Some(description) => entry.insert("description", description)?,
        None => entry.insert("description", ())?,
    }

    match image.dimensions {
        Some((width, height)) => {
            let mut dims = ZendHashTable::with_capacity(2);
            dims.push(i64::from(width))?;
            dims.push(i64::from(height))?;
            entry.insert("dimensions", dims)?;
        }
        None => entry.insert("dimensions", ())?,
    }

    entry.insert("source", image.source.to_string())?;
    entry.insert("attributes", build_attribute_table(image.attributes)?)?;

    Ok(entry)
}

/// Build a PHP hash table from a BTreeMap of attributes.
fn build_attribute_table(attributes: BTreeMap<String, String>) -> PhpResult<ZBox<ZendHashTable>> {
    let mut table = ZendHashTable::with_capacity(table_capacity(attributes.len()));

    for (key, value) in attributes {
        table.insert(key, value)?;
    }

    Ok(table)
}

/// Build a PHP hash table from inline image warnings.
fn build_warnings(warnings: Vec<InlineImageWarning>) -> PhpResult<ZBox<ZendHashTable>> {
    let mut table = ZendHashTable::with_capacity(table_capacity(warnings.len()));

    for warning in warnings {
        let mut entry = ZendHashTable::new();
        entry.insert("index", warning.index as i64)?;
        entry.insert("message", warning.message)?;
        table.push(entry)?;
    }

    Ok(table)
}

/// Build a PHP hash table from a ConversionResult.
pub fn build_conversion_result(result: html_to_markdown_rs::ConversionResult) -> PhpResult<ZBox<ZendHashTable>> {
    let mut table = ZendHashTable::new();

    // content: Option<String>
    match result.content {
        Some(content) => table.insert("content", content)?,
        None => table.insert("content", ())?,
    }

    // document: not yet exposed
    table.insert("document", ())?;

    // metadata
    #[cfg(feature = "metadata")]
    table.insert("metadata", build_extended_metadata(result.metadata)?)?;
    #[cfg(not(feature = "metadata"))]
    table.insert("metadata", ())?;

    // tables: Vec<types::TableData> with grid (TableGrid) and markdown
    let mut tables_array = ZendHashTable::with_capacity(table_capacity(result.tables.len()));
    for t in result.tables {
        let mut entry = ZendHashTable::new();
        // grid
        let mut grid_entry = ZendHashTable::new();
        grid_entry.insert("rows", t.grid.rows as i64)?;
        grid_entry.insert("cols", t.grid.cols as i64)?;
        let mut cells_array = ZendHashTable::with_capacity(table_capacity(t.grid.cells.len()));
        for cell in t.grid.cells {
            let mut cell_entry = ZendHashTable::new();
            cell_entry.insert("content", cell.content)?;
            cell_entry.insert("row", cell.row as i64)?;
            cell_entry.insert("col", cell.col as i64)?;
            cell_entry.insert("row_span", cell.row_span as i64)?;
            cell_entry.insert("col_span", cell.col_span as i64)?;
            cell_entry.insert("is_header", cell.is_header)?;
            cells_array.push(cell_entry)?;
        }
        grid_entry.insert("cells", cells_array)?;
        entry.insert("grid", grid_entry)?;
        entry.insert("markdown", t.markdown)?;
        tables_array.push(entry)?;
    }
    table.insert("tables", tables_array)?;

    // images
    #[cfg(feature = "inline-images")]
    {
        let mut images_array = ZendHashTable::with_capacity(table_capacity(result.images.len()));
        for image in result.images {
            images_array.push(build_inline_image_entry(image)?)?;
        }
        table.insert("images", images_array)?;
    }
    #[cfg(not(feature = "inline-images"))]
    table.insert("images", ZendHashTable::new())?;

    // warnings
    let mut warnings_array = ZendHashTable::with_capacity(table_capacity(result.warnings.len()));
    for warning in result.warnings {
        let mut entry = ZendHashTable::new();
        entry.insert("message", warning.message)?;
        let kind = match warning.kind {
            html_to_markdown_rs::WarningKind::ImageExtractionFailed => "image_extraction_failed",
            html_to_markdown_rs::WarningKind::EncodingFallback => "encoding_fallback",
            html_to_markdown_rs::WarningKind::TruncatedInput => "truncated_input",
            html_to_markdown_rs::WarningKind::MalformedHtml => "malformed_html",
            html_to_markdown_rs::WarningKind::SanitizationApplied => "sanitization_applied",
        };
        entry.insert("kind", kind)?;
        warnings_array.push(entry)?;
    }
    table.insert("warnings", warnings_array)?;

    Ok(table)
}

/// Build a PHP hash table from a table extraction result (requires visitor feature).
#[cfg(feature = "visitor")]
pub fn build_tables_extraction(result: html_to_markdown_rs::ConversionWithTables) -> PhpResult<ZBox<ZendHashTable>> {
    let mut table = ZendHashTable::new();
    table.insert("content", result.content)?;

    // Build tables array
    let mut tables_array = ZendHashTable::with_capacity(table_capacity(result.tables.len()));
    for t in result.tables {
        let mut entry = ZendHashTable::new();
        // cells: Vec<Vec<String>> -> array of arrays
        let mut cells_array = ZendHashTable::with_capacity(table_capacity(t.cells.len()));
        for row in t.cells {
            let mut row_array = ZendHashTable::with_capacity(table_capacity(row.len()));
            for cell in row {
                row_array.push(cell)?;
            }
            cells_array.push(row_array)?;
        }
        entry.insert("cells", cells_array)?;
        entry.insert("markdown", t.markdown)?;
        // is_header_row: Vec<bool> -> array of booleans
        let mut header_array = ZendHashTable::with_capacity(table_capacity(t.is_header_row.len()));
        for is_header in t.is_header_row {
            header_array.push(is_header)?;
        }
        entry.insert("is_header_row", header_array)?;
        tables_array.push(entry)?;
    }
    table.insert("tables", tables_array)?;

    // metadata
    #[cfg(feature = "metadata")]
    match result.metadata {
        Some(metadata) => table.insert("metadata", build_extended_metadata(metadata)?)?,
        None => table.insert("metadata", ())?,
    };
    #[cfg(not(feature = "metadata"))]
    table.insert("metadata", ())?;

    Ok(table)
}

/// Build a PHP hash table from metadata extraction (requires metadata feature).
#[cfg(feature = "metadata")]
pub fn build_metadata_extraction(markdown: String, metadata: HtmlMetadata) -> PhpResult<ZBox<ZendHashTable>> {
    let mut result = ZendHashTable::new();
    result.insert("markdown", markdown)?;
    result.insert("metadata", build_extended_metadata(metadata)?)?;
    Ok(result)
}

#[cfg(feature = "metadata")]
fn build_extended_metadata(metadata: HtmlMetadata) -> PhpResult<ZBox<ZendHashTable>> {
    let mut table = ZendHashTable::new();
    table.insert("document", build_document_metadata(metadata.document)?)?;
    table.insert("headers", build_headers_array(metadata.headers)?)?;
    table.insert("links", build_links_array(metadata.links)?)?;
    table.insert("images", build_images_array(metadata.images)?)?;
    table.insert(
        "structured_data",
        build_structured_data_array(metadata.structured_data)?,
    )?;
    Ok(table)
}

#[cfg(feature = "metadata")]
fn build_document_metadata(doc: DocumentMetadata) -> PhpResult<ZBox<ZendHashTable>> {
    let mut table = ZendHashTable::new();

    match doc.title {
        Some(title) => table.insert("title", title)?,
        None => table.insert("title", ())?,
    }

    match doc.description {
        Some(description) => table.insert("description", description)?,
        None => table.insert("description", ())?,
    }

    table.insert("keywords", doc.keywords)?;

    match doc.author {
        Some(author) => table.insert("author", author)?,
        None => table.insert("author", ())?,
    }

    match doc.canonical_url {
        Some(url) => table.insert("canonical_url", url)?,
        None => table.insert("canonical_url", ())?,
    }

    match doc.base_href {
        Some(href) => table.insert("base_href", href)?,
        None => table.insert("base_href", ())?,
    }

    match doc.language {
        Some(lang) => table.insert("language", lang)?,
        None => table.insert("language", ())?,
    }

    table.insert("text_direction", text_direction_to_string(doc.text_direction))?;
    table.insert("open_graph", build_string_map(doc.open_graph)?)?;
    table.insert("twitter_card", build_string_map(doc.twitter_card)?)?;
    table.insert("meta_tags", build_string_map(doc.meta_tags)?)?;

    Ok(table)
}

#[cfg(feature = "metadata")]
fn build_headers_array(headers: Vec<HeaderMetadata>) -> PhpResult<ZBox<ZendHashTable>> {
    let mut array = ZendHashTable::with_capacity(table_capacity(headers.len()));

    for header in headers {
        let mut entry = ZendHashTable::new();
        entry.insert("level", i64::from(header.level))?;
        entry.insert("text", header.text)?;

        match header.id {
            Some(id) => entry.insert("id", id)?,
            None => entry.insert("id", ())?,
        }

        entry.insert("depth", header.depth as i64)?;
        entry.insert("html_offset", header.html_offset as i64)?;

        array.push(entry)?;
    }

    Ok(array)
}

#[cfg(feature = "metadata")]
fn build_links_array(links: Vec<LinkMetadata>) -> PhpResult<ZBox<ZendHashTable>> {
    let mut array = ZendHashTable::with_capacity(table_capacity(links.len()));

    for link in links {
        let mut entry = ZendHashTable::new();
        entry.insert("href", link.href)?;
        entry.insert("text", link.text)?;

        match link.title {
            Some(title) => entry.insert("title", title)?,
            None => entry.insert("title", ())?,
        }

        entry.insert("link_type", link.link_type.to_string())?;
        entry.insert("rel", link.rel)?;
        entry.insert("attributes", build_string_map(link.attributes)?)?;

        array.push(entry)?;
    }

    Ok(array)
}

#[cfg(feature = "metadata")]
fn build_images_array(images: Vec<ImageMetadata>) -> PhpResult<ZBox<ZendHashTable>> {
    let mut array = ZendHashTable::with_capacity(table_capacity(images.len()));

    for image in images {
        let mut entry = ZendHashTable::new();
        entry.insert("src", image.src)?;

        match image.alt {
            Some(alt) => entry.insert("alt", alt)?,
            None => entry.insert("alt", ())?,
        }

        match image.title {
            Some(title) => entry.insert("title", title)?,
            None => entry.insert("title", ())?,
        }

        match image.dimensions {
            Some((width, height)) => {
                let mut dims = ZendHashTable::with_capacity(2);
                dims.push(i64::from(width))?;
                dims.push(i64::from(height))?;
                entry.insert("dimensions", dims)?;
            }
            None => entry.insert("dimensions", ())?,
        }

        entry.insert("image_type", image.image_type.to_string())?;
        entry.insert("attributes", build_string_map(image.attributes)?)?;

        array.push(entry)?;
    }

    Ok(array)
}

#[cfg(feature = "metadata")]
fn build_structured_data_array(data: Vec<StructuredData>) -> PhpResult<ZBox<ZendHashTable>> {
    let mut array = ZendHashTable::with_capacity(table_capacity(data.len()));

    for item in data {
        let mut entry = ZendHashTable::new();
        entry.insert("data_type", item.data_type.to_string())?;
        entry.insert("raw_json", item.raw_json)?;

        match item.schema_type {
            Some(schema_type) => entry.insert("schema_type", schema_type)?,
            None => entry.insert("schema_type", ())?,
        }

        array.push(entry)?;
    }

    Ok(array)
}

#[cfg(feature = "metadata")]
fn build_string_map(map: BTreeMap<String, String>) -> PhpResult<ZBox<ZendHashTable>> {
    let mut table = ZendHashTable::with_capacity(table_capacity(map.len()));

    for (key, value) in map {
        table.insert(key, value)?;
    }

    Ok(table)
}

#[cfg(feature = "metadata")]
fn text_direction_to_string(direction: Option<TextDirection>) -> String {
    direction.map(|dir| dir.to_string()).unwrap_or_default()
}
