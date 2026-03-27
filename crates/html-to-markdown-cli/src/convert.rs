#![allow(clippy::all, clippy::pedantic, clippy::nursery, missing_docs)]

use crate::args::Cli;
use crate::output::output_debug_info;
use html_to_markdown_rs::{
    ConversionOptions, MetadataConfig, OutputFormat, PreprocessingOptions, convert, convert_with_metadata,
    metadata::DEFAULT_MAX_STRUCTURED_DATA_SIZE,
};
use serde_json::json;

pub fn build_conversion_options(cli: &Cli) -> ConversionOptions {
    let defaults = ConversionOptions::default();

    let preprocessing = PreprocessingOptions {
        enabled: cli.preprocess,
        preset: cli.preset.map(Into::into).unwrap_or_default(),
        remove_navigation: !cli.keep_navigation,
        remove_forms: !cli.keep_forms,
    };

    ConversionOptions {
        heading_style: cli.heading_style.map_or(defaults.heading_style, Into::into),
        list_indent_type: cli.list_indent_type.map_or(defaults.list_indent_type, Into::into),
        list_indent_width: cli.list_indent_width.map_or(defaults.list_indent_width, |w| w as usize),
        bullets: cli.bullets.clone().unwrap_or(defaults.bullets),
        strong_em_symbol: cli.strong_em_symbol.unwrap_or(defaults.strong_em_symbol),
        escape_asterisks: cli.escape_asterisks,
        escape_underscores: cli.escape_underscores,
        escape_misc: cli.escape_misc,
        escape_ascii: cli.escape_ascii,
        code_language: cli.code_language.clone().unwrap_or(defaults.code_language),
        autolinks: cli.autolinks,
        default_title: cli.default_title,
        br_in_tables: cli.br_in_tables,
        highlight_style: cli.highlight_style.map_or(defaults.highlight_style, Into::into),
        extract_metadata: cli.extract_metadata,
        whitespace_mode: cli.whitespace_mode.map_or(defaults.whitespace_mode, Into::into),
        strip_newlines: cli.strip_newlines,
        wrap: cli.wrap,
        wrap_width: cli.wrap_width.map_or(defaults.wrap_width, |w| w as usize),
        convert_as_inline: cli.convert_as_inline,
        sub_symbol: cli.sub_symbol.clone().unwrap_or(defaults.sub_symbol),
        sup_symbol: cli.sup_symbol.clone().unwrap_or(defaults.sup_symbol),
        newline_style: cli.newline_style.map_or(defaults.newline_style, Into::into),
        code_block_style: cli.code_block_style.map_or(defaults.code_block_style, Into::into),
        keep_inline_images_in: cli
            .keep_inline_images_in
            .clone()
            .unwrap_or(defaults.keep_inline_images_in),
        skip_images: false,
        preprocessing,
        encoding: cli.encoding.clone(),
        debug: cli.debug,
        strip_tags: cli.strip_tags.clone().unwrap_or(defaults.strip_tags),
        preserve_tags: Vec::new(),
        output_format: cli.output_format.map_or(OutputFormat::default(), Into::into),
        include_document_structure: false,
        extract_images: false,
        max_image_size: 5_242_880,
        capture_svg: false,
        infer_dimensions: true,
    }
}

pub fn perform_conversion(
    html: &str,
    options: ConversionOptions,
    cli: &Cli,
) -> Result<String, Box<dyn std::error::Error>> {
    let output_content = if cli.with_metadata {
        let metadata_config = MetadataConfig {
            extract_document: cli.extract_document,
            extract_headers: cli.extract_headers,
            extract_links: cli.extract_links,
            extract_images: cli.extract_images,
            extract_structured_data: cli.extract_structured_data,
            max_structured_data_size: DEFAULT_MAX_STRUCTURED_DATA_SIZE,
        };

        let (markdown, metadata) = convert_with_metadata(html, Some(options), metadata_config, None)
            .map_err(|e| format!("Error converting HTML with metadata: {e}"))?;

        output_debug_info(
            cli,
            &format!("Generated {} bytes of markdown with metadata", markdown.len()),
        );

        let output = json!({
            "markdown": markdown,
            "metadata": metadata
        });

        serde_json::to_string_pretty(&output).map_err(|e| format!("Error serializing JSON: {e}"))?
    } else {
        let markdown = convert(html, Some(options)).map_err(|e| format!("Error converting HTML: {e}"))?;

        output_debug_info(cli, &format!("Generated {} bytes of markdown", markdown.len()));

        markdown
    };

    Ok(output_content)
}
