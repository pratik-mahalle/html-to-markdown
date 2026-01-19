pub mod inline_image;
#[cfg(feature = "metadata")]
pub mod metadata;

pub use inline_image::{JsHtmlExtraction, JsInlineImage, JsInlineImageWarning};
#[cfg(feature = "metadata")]
pub use metadata::{
    JsDocumentMetadata, JsExtendedMetadata, JsHeaderMetadata, JsImageMetadata, JsLinkMetadata, JsMetadataConfig,
    JsMetadataExtraction, JsStructuredData, convert_metadata,
};
