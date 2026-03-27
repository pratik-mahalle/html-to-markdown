#![allow(clippy::all, clippy::pedantic, clippy::nursery, missing_docs)]
#![deny(clippy::correctness, clippy::suspicious)]
#![warn(clippy::all)]
#![allow(clippy::all, clippy::pedantic, clippy::nursery)]

// Module declarations
mod convert;
mod enums;
mod options;

#[cfg(feature = "js-bindings")]
pub mod inline_images;

#[cfg(feature = "js-bindings")]
pub use inline_images::{WasmHtmlExtraction, WasmInlineImage, WasmInlineImageConfig, WasmInlineImageWarning};

#[cfg(feature = "wasmtime-testing")]
mod wasmtime;

#[cfg(all(test, feature = "js-bindings"))]
mod tests;

// Re-export public API
pub use enums::{
    WasmCodeBlockStyle, WasmHeadingStyle, WasmHighlightStyle, WasmListIndentType, WasmNewlineStyle, WasmOutputFormat,
    WasmPreprocessingPreset, WasmWhitespaceMode,
};
pub use options::{WasmConversionOptions, WasmPreprocessingOptions};

#[cfg(feature = "js-bindings")]
pub use options::WasmConversionOptionsHandle;

#[cfg(all(feature = "js-bindings", feature = "metadata"))]
pub use options::WasmMetadataConfig;

#[cfg(feature = "js-bindings")]
pub use convert::{
    convert, convert_bytes, convert_bytes_with_inline_images, convert_bytes_with_options_handle,
    convert_with_inline_images, convert_with_options_handle, create_conversion_options_handle, extract,
};

#[cfg(all(feature = "js-bindings", feature = "metadata"))]
pub use convert::{convert_bytes_with_metadata, convert_with_metadata};

#[cfg(all(feature = "js-bindings", feature = "visitor"))]
pub use convert::convert_with_tables;

/// Initialize panic hook for better error messages in the browser
#[cfg(feature = "js-bindings")]
#[wasm_bindgen::prelude::wasm_bindgen(start)]
pub fn init() {
    console_error_panic_hook::set_once();
}
