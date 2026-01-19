#[cfg(all(test, feature = "js-bindings"))]
mod wasm_tests {
    use crate::enums::WasmHeadingStyle;
    use crate::options::WasmConversionOptions;
    use wasm_bindgen::JsValue;
    use wasm_bindgen_test::*;

    wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    fn test_convert_basic() {
        let html = "<h1>Hello World</h1>".to_string();
        let result = crate::convert::convert(html, JsValue::UNDEFINED);
        assert!(result.is_ok());
        let markdown = result.unwrap();
        assert!(markdown.contains("Hello World"));
    }

    #[wasm_bindgen_test]
    fn test_convert_with_options() {
        let html = "<h1>Hello</h1>".to_string();
        let options = WasmConversionOptions {
            heading_style: Some(WasmHeadingStyle::Atx),
            list_indent_type: None,
            list_indent_width: None,
            bullets: None,
            strong_em_symbol: None,
            escape_asterisks: None,
            escape_underscores: None,
            escape_misc: None,
            escape_ascii: None,
            code_language: None,
            autolinks: None,
            default_title: None,
            br_in_tables: None,
            hocr_spatial_tables: None,
            highlight_style: None,
            extract_metadata: None,
            whitespace_mode: None,
            strip_newlines: None,
            wrap: None,
            wrap_width: None,
            convert_as_inline: None,
            sub_symbol: None,
            sup_symbol: None,
            newline_style: None,
            code_block_style: None,
            keep_inline_images_in: None,
            skip_images: None,
            preprocessing: None,
            encoding: None,
            debug: None,
            strip_tags: None,
            preserve_tags: None,
            output_format: None,
        };

        let js_options = serde_wasm_bindgen::to_value(&options).unwrap();
        let result = crate::convert::convert(html, js_options);
        assert!(result.is_ok());
    }

    #[cfg(feature = "metadata")]
    #[wasm_bindgen_test]
    fn test_metadata_config_new() {
        let config = crate::options::WasmMetadataConfig::new();
        assert!(config.extract_headers());
        assert!(config.extract_links());
        assert!(config.extract_images());
        assert!(config.extract_structured_data());
        assert_eq!(
            config.max_structured_data_size(),
            html_to_markdown_rs::DEFAULT_MAX_STRUCTURED_DATA_SIZE
        );
    }

    #[cfg(feature = "metadata")]
    #[wasm_bindgen_test]
    fn test_metadata_config_setters() {
        let mut config = crate::options::WasmMetadataConfig::new();

        config.set_extract_headers(false);
        assert!(!config.extract_headers());

        config.set_extract_links(false);
        assert!(!config.extract_links());

        config.set_extract_images(false);
        assert!(!config.extract_images());

        config.set_extract_structured_data(false);
        assert!(!config.extract_structured_data());

        config.set_max_structured_data_size(500_000);
        assert_eq!(config.max_structured_data_size(), 500_000);
    }

    #[cfg(feature = "metadata")]
    #[wasm_bindgen_test]
    fn test_convert_with_metadata_basic() {
        let html = "<h1>Hello World</h1>".to_string();
        let config = crate::options::WasmMetadataConfig::new();

        let result = crate::convert::convert_with_metadata(html, JsValue::UNDEFINED, Some(config));
        assert!(result.is_ok());

        let obj = result.unwrap();
        assert!(js_sys::Reflect::has(&obj, &JsValue::from_str("markdown")).unwrap());
        assert!(js_sys::Reflect::has(&obj, &JsValue::from_str("metadata")).unwrap());
    }

    #[cfg(feature = "metadata")]
    #[wasm_bindgen_test]
    fn test_convert_with_metadata_with_headers() {
        let html = r#"<html><head><title>Test</title></head><body><h1 id="main">Main Title</h1><h2>Subsection</h2></body></html>"#
            .to_string();
        let config = crate::options::WasmMetadataConfig::new();

        let result = crate::convert::convert_with_metadata(html, JsValue::UNDEFINED, Some(config));
        assert!(result.is_ok());

        let obj = result.unwrap();
        let markdown = js_sys::Reflect::get(&obj, &JsValue::from_str("markdown")).unwrap();
        let markdown_str = markdown.as_string().unwrap();
        assert!(markdown_str.contains("Main Title"));
    }

    #[cfg(feature = "metadata")]
    #[wasm_bindgen_test]
    fn test_convert_bytes_with_metadata() {
        let html_bytes = vec![60, 104, 49, 62, 72, 101, 108, 108, 111, 60, 47, 104, 49, 62];
        let uint8 = js_sys::Uint8Array::from(&html_bytes[..]);
        let config = crate::options::WasmMetadataConfig::new();

        let result = crate::convert::convert_bytes_with_metadata(uint8, JsValue::UNDEFINED, Some(config));
        assert!(result.is_ok());

        let obj = result.unwrap();
        assert!(js_sys::Reflect::has(&obj, &JsValue::from_str("markdown")).unwrap());
        assert!(js_sys::Reflect::has(&obj, &JsValue::from_str("metadata")).unwrap());
    }
}
