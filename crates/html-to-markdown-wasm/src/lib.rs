#![allow(clippy::all, clippy::pedantic, clippy::nursery, missing_docs)]
#![deny(clippy::correctness, clippy::suspicious)]

// Module declarations
mod convert;
mod enums;
mod options;

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
pub use convert::convert;

/// Initialize panic hook for better error messages in the browser
#[cfg(feature = "js-bindings")]
#[wasm_bindgen::prelude::wasm_bindgen(start)]
pub fn init() {
    console_error_panic_hook::set_once();
}
