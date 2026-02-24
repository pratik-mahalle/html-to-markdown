//! R list type conversions for binding results.

use extendr_api::prelude::*;
use html_to_markdown_rs::InlineImage;
use html_to_markdown_rs::metadata::{
    DocumentMetadata, ExtendedMetadata, HeaderMetadata, ImageMetadata, LinkMetadata, StructuredData,
};
use std::collections::HashMap;

/// Convert ExtendedMetadata into an R list.
pub fn metadata_to_robj(metadata: ExtendedMetadata) -> Robj {
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
