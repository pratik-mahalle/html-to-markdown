use html_to_markdown_rs::metadata::{
    DocumentMetadata as RustDocumentMetadata, ExtendedMetadata as RustExtendedMetadata,
    HeaderMetadata as RustHeaderMetadata, ImageMetadata as RustImageMetadata, LinkMetadata as RustLinkMetadata,
    MetadataConfig as RustMetadataConfig, StructuredData as RustStructuredData,
};
use napi_derive::napi;
use std::collections::HashMap;

/// Metadata extraction configuration
#[cfg(feature = "metadata")]
#[napi(object)]
pub struct JsMetadataConfig {
    #[napi(js_name = "extract_document")]
    pub extract_document: Option<bool>,
    #[napi(js_name = "extract_headers")]
    pub extract_headers: Option<bool>,
    #[napi(js_name = "extract_links")]
    pub extract_links: Option<bool>,
    #[napi(js_name = "extract_images")]
    pub extract_images: Option<bool>,
    #[napi(js_name = "extract_structured_data")]
    pub extract_structured_data: Option<bool>,
    #[napi(js_name = "max_structured_data_size")]
    pub max_structured_data_size: Option<i64>,
}

#[cfg(feature = "metadata")]
impl From<JsMetadataConfig> for RustMetadataConfig {
    fn from(val: JsMetadataConfig) -> Self {
        let update = html_to_markdown_rs::MetadataConfigUpdate {
            extract_document: val.extract_document,
            extract_headers: val.extract_headers,
            extract_links: val.extract_links,
            extract_images: val.extract_images,
            extract_structured_data: val.extract_structured_data,
            max_structured_data_size: val.max_structured_data_size.map(|value| value as usize),
        };
        Self::from(update)
    }
}

/// Document-level metadata
#[cfg(feature = "metadata")]
#[napi(object)]
pub struct JsDocumentMetadata {
    pub title: Option<String>,
    pub description: Option<String>,
    pub keywords: Vec<String>,
    pub author: Option<String>,
    #[napi(js_name = "canonical_url")]
    pub canonical_url: Option<String>,
    #[napi(js_name = "base_href")]
    pub base_href: Option<String>,
    pub language: Option<String>,
    #[napi(js_name = "text_direction")]
    pub text_direction: Option<String>,
    #[napi(js_name = "open_graph")]
    pub open_graph: HashMap<String, String>,
    #[napi(js_name = "twitter_card")]
    pub twitter_card: HashMap<String, String>,
    #[napi(js_name = "meta_tags")]
    pub meta_tags: HashMap<String, String>,
}

/// Header element metadata
#[cfg(feature = "metadata")]
#[napi(object)]
pub struct JsHeaderMetadata {
    pub level: u32,
    pub text: String,
    pub id: Option<String>,
    pub depth: u32,
    #[napi(js_name = "html_offset")]
    pub html_offset: u32,
}

/// Hyperlink metadata
#[cfg(feature = "metadata")]
#[napi(object)]
pub struct JsLinkMetadata {
    pub href: String,
    pub text: String,
    pub title: Option<String>,
    #[napi(js_name = "link_type")]
    pub link_type: String,
    pub rel: Vec<String>,
    pub attributes: HashMap<String, String>,
}

/// Image metadata
#[cfg(feature = "metadata")]
#[napi(object)]
pub struct JsImageMetadata {
    pub src: String,
    pub alt: Option<String>,
    pub title: Option<String>,
    pub dimensions: Option<Vec<u32>>,
    #[napi(js_name = "image_type")]
    pub image_type: String,
    pub attributes: HashMap<String, String>,
}

/// Structured data (JSON-LD, Microdata, `RDFa`)
#[cfg(feature = "metadata")]
#[napi(object)]
pub struct JsStructuredData {
    #[napi(js_name = "data_type")]
    pub data_type: String,
    #[napi(js_name = "raw_json")]
    pub raw_json: String,
    #[napi(js_name = "schema_type")]
    pub schema_type: Option<String>,
}

/// Complete extracted metadata
#[cfg(feature = "metadata")]
#[napi(object)]
pub struct JsExtendedMetadata {
    pub document: JsDocumentMetadata,
    pub headers: Vec<JsHeaderMetadata>,
    pub links: Vec<JsLinkMetadata>,
    pub images: Vec<JsImageMetadata>,
    pub structured_data: Vec<JsStructuredData>,
}

/// Result of conversion with metadata extraction
#[cfg(feature = "metadata")]
#[napi(object)]
pub struct JsMetadataExtraction {
    pub markdown: String,
    pub metadata: JsExtendedMetadata,
}

#[cfg(feature = "metadata")]
pub fn convert_document_metadata(doc: RustDocumentMetadata) -> JsDocumentMetadata {
    JsDocumentMetadata {
        title: doc.title,
        description: doc.description,
        keywords: doc.keywords,
        author: doc.author,
        canonical_url: doc.canonical_url,
        base_href: doc.base_href,
        language: doc.language,
        text_direction: doc.text_direction.map(|dir| dir.to_string()),
        open_graph: doc.open_graph.into_iter().collect(),
        twitter_card: doc.twitter_card.into_iter().collect(),
        meta_tags: doc.meta_tags.into_iter().collect(),
    }
}

#[cfg(feature = "metadata")]
pub fn convert_headers(headers: Vec<RustHeaderMetadata>) -> Vec<JsHeaderMetadata> {
    headers
        .into_iter()
        .map(|h| JsHeaderMetadata {
            level: u32::from(h.level),
            text: h.text,
            id: h.id,
            depth: h.depth as u32,
            html_offset: h.html_offset as u32,
        })
        .collect()
}

#[cfg(feature = "metadata")]
pub fn convert_links(links: Vec<RustLinkMetadata>) -> Vec<JsLinkMetadata> {
    links
        .into_iter()
        .map(|l| JsLinkMetadata {
            href: l.href,
            text: l.text,
            title: l.title,
            link_type: l.link_type.to_string(),
            rel: l.rel,
            attributes: l.attributes.into_iter().collect(),
        })
        .collect()
}

#[cfg(feature = "metadata")]
pub fn convert_images(images: Vec<RustImageMetadata>) -> Vec<JsImageMetadata> {
    images
        .into_iter()
        .map(|i| JsImageMetadata {
            src: i.src,
            alt: i.alt,
            title: i.title,
            dimensions: i.dimensions.map(|(w, h)| vec![w, h]),
            image_type: i.image_type.to_string(),
            attributes: i.attributes.into_iter().collect(),
        })
        .collect()
}

#[cfg(feature = "metadata")]
pub fn convert_structured_data(data: Vec<RustStructuredData>) -> Vec<JsStructuredData> {
    data.into_iter()
        .map(|d| JsStructuredData {
            data_type: d.data_type.to_string(),
            raw_json: d.raw_json,
            schema_type: d.schema_type,
        })
        .collect()
}

#[cfg(feature = "metadata")]
pub fn convert_metadata(metadata: RustExtendedMetadata) -> JsExtendedMetadata {
    JsExtendedMetadata {
        document: convert_document_metadata(metadata.document),
        headers: convert_headers(metadata.headers),
        links: convert_links(metadata.links),
        images: convert_images(metadata.images),
        structured_data: convert_structured_data(metadata.structured_data),
    }
}
