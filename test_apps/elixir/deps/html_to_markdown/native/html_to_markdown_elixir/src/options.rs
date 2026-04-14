//! Option parsing for Elixir bindings.

use std::cell::RefCell;
use std::collections::HashMap;

use html_to_markdown_rs::metadata::MetadataConfig;
use html_to_markdown_rs::{
    CodeBlockStyle, ConversionOptions, ConversionOptionsUpdate, DEFAULT_INLINE_IMAGE_LIMIT, HeadingStyle,
    HighlightStyle, InlineImageConfig, InlineImageConfigUpdate, ListIndentType, MetadataConfigUpdate, NewlineStyle,
    PreprocessingOptionsUpdate, PreprocessingPreset, WhitespaceMode,
};
use rustler::{Error, NifResult, Term};

pub const INVALID_OPTION_ERROR: &str = "invalid_option";

thread_local! {
    static LAST_INVALID_OPTION: RefCell<Option<String>> = const { RefCell::new(None) };
}

pub fn set_invalid_option_message(message: impl Into<String>) {
    LAST_INVALID_OPTION.with(|slot| {
        *slot.borrow_mut() = Some(message.into());
    });
}

pub fn take_invalid_option_message() -> Option<String> {
    LAST_INVALID_OPTION.with(|slot| slot.borrow_mut().take())
}

pub fn bad_option(field: &'static str) -> Error {
    bad_option_msg(field, format!("invalid value for {field}"))
}

pub fn bad_option_msg(field: &'static str, message: impl Into<String>) -> Error {
    let message = message.into();
    set_invalid_option_message(format!("{field}: {message}"));
    Error::Atom(INVALID_OPTION_ERROR)
}

pub fn decode_options_term(term: Term) -> NifResult<ConversionOptions> {
    if matches!(term.atom_to_string(), Ok(name) if name == "nil") {
        return Ok(ConversionOptions::default());
    }

    let map: HashMap<String, Term> = term
        .decode()
        .map_err(|_| bad_option_msg("options", "must be provided as a map"))?;
    apply_options(map)
}

pub fn decode_metadata_config(term: Term) -> NifResult<MetadataConfig> {
    if matches!(term.atom_to_string(), Ok(name) if name == "nil") {
        return Ok(MetadataConfig::default());
    }

    let map: HashMap<String, Term> = term
        .decode()
        .map_err(|_| bad_option_msg("metadata_config", "must be provided as a map"))?;

    let mut update = MetadataConfigUpdate::default();

    for (key, value) in map.into_iter() {
        match key.as_str() {
            "extract_document" => update.extract_document = Some(decode_bool(value, "extract_document")?),
            "extract_headers" => update.extract_headers = Some(decode_bool(value, "extract_headers")?),
            "extract_links" => update.extract_links = Some(decode_bool(value, "extract_links")?),
            "extract_images" => update.extract_images = Some(decode_bool(value, "extract_images")?),
            "extract_structured_data" => {
                update.extract_structured_data = Some(decode_bool(value, "extract_structured_data")?)
            }
            "max_structured_data_size" => {
                update.max_structured_data_size = Some(decode_positive_integer(value, "max_structured_data_size")?)
            }
            _ => {}
        }
    }

    Ok(MetadataConfig::from(update))
}

fn apply_options(map: HashMap<String, Term>) -> NifResult<ConversionOptions> {
    let mut update = ConversionOptionsUpdate::default();

    for (key, value) in map.into_iter() {
        match key.as_str() {
            "heading_style" => update.heading_style = Some(parse_heading_style(value)?),
            "list_indent_type" => update.list_indent_type = Some(parse_list_indent_type(value)?),
            "list_indent_width" => {
                update.list_indent_width = Some(decode_positive_integer(value, "list_indent_width")?)
            }
            "bullets" => update.bullets = Some(decode_string(value, "bullets")?),
            "strong_em_symbol" => {
                let symbol = decode_string(value, "strong_em_symbol")?;
                let mut chars = symbol.chars();
                let ch = chars.next().ok_or_else(|| bad_option("strong_em_symbol"))?;
                update.strong_em_symbol = Some(ch);
            }
            "escape_asterisks" => update.escape_asterisks = Some(decode_bool(value, "escape_asterisks")?),
            "escape_underscores" => update.escape_underscores = Some(decode_bool(value, "escape_underscores")?),
            "escape_misc" => update.escape_misc = Some(decode_bool(value, "escape_misc")?),
            "escape_ascii" => update.escape_ascii = Some(decode_bool(value, "escape_ascii")?),
            "code_language" => update.code_language = Some(decode_string(value, "code_language")?),
            "encoding" => update.encoding = Some(decode_string(value, "encoding")?),
            "autolinks" => update.autolinks = Some(decode_bool(value, "autolinks")?),
            "default_title" => update.default_title = Some(decode_bool(value, "default_title")?),
            "keep_inline_images_in" => {
                update.keep_inline_images_in = Some(decode_string_list(value, "keep_inline_images_in")?)
            }
            "br_in_tables" => update.br_in_tables = Some(decode_bool(value, "br_in_tables")?),
            "hocr_spatial_tables" => update.hocr_spatial_tables = Some(decode_bool(value, "hocr_spatial_tables")?),
            "highlight_style" => update.highlight_style = Some(parse_highlight_style(value)?),
            "extract_metadata" => update.extract_metadata = Some(decode_bool(value, "extract_metadata")?),
            "whitespace_mode" => update.whitespace_mode = Some(parse_whitespace_mode(value)?),
            "strip_newlines" => update.strip_newlines = Some(decode_bool(value, "strip_newlines")?),
            "wrap" => update.wrap = Some(decode_bool(value, "wrap")?),
            "wrap_width" => update.wrap_width = Some(decode_positive_integer(value, "wrap_width")?),
            "strip_tags" => update.strip_tags = Some(decode_string_list(value, "strip_tags")?),
            "preserve_tags" => update.preserve_tags = Some(decode_string_list(value, "preserve_tags")?),
            "convert_as_inline" => update.convert_as_inline = Some(decode_bool(value, "convert_as_inline")?),
            "sub_symbol" => update.sub_symbol = Some(decode_string(value, "sub_symbol")?),
            "sup_symbol" => update.sup_symbol = Some(decode_string(value, "sup_symbol")?),
            "newline_style" => update.newline_style = Some(parse_newline_style(value)?),
            "code_block_style" => update.code_block_style = Some(parse_code_block_style(value)?),
            "preprocessing" => update.preprocessing = Some(decode_preprocessing(value)?),
            "debug" => update.debug = Some(decode_bool(value, "debug")?),
            _ => {}
        }
    }

    Ok(ConversionOptions::from(update))
}

fn decode_preprocessing(term: Term) -> NifResult<PreprocessingOptionsUpdate> {
    let map: HashMap<String, Term> = term
        .decode()
        .map_err(|_| bad_option_msg("preprocessing", "must be provided as a map"))?;

    let mut update = PreprocessingOptionsUpdate::default();

    for (key, value) in map.into_iter() {
        match key.as_str() {
            "enabled" => update.enabled = Some(decode_bool(value, "preprocessing.enabled")?),
            "preset" => update.preset = Some(parse_preset(value)?),
            "remove_navigation" => {
                update.remove_navigation = Some(decode_bool(value, "preprocessing.remove_navigation")?)
            }
            "remove_forms" => update.remove_forms = Some(decode_bool(value, "preprocessing.remove_forms")?),
            _ => {}
        }
    }

    Ok(update)
}

pub fn decode_inline_image_config(term: Term) -> NifResult<InlineImageConfig> {
    if matches!(term.atom_to_string(), Ok(name) if name == "nil") {
        return Ok(InlineImageConfig::new(DEFAULT_INLINE_IMAGE_LIMIT));
    }

    let map: HashMap<String, Term> = term
        .decode()
        .map_err(|_| bad_option_msg("inline_image_config", "must be provided as a map"))?;

    let max_bytes = match map.get("max_decoded_size_bytes") {
        Some(value) => match value.decode::<i64>().map_err(|_| {
            bad_option_msg(
                "inline_image_config.max_decoded_size_bytes",
                "must be a positive integer",
            )
        })? {
            v if v > 0 => v as u64,
            _ => {
                return Err(bad_option_msg(
                    "max_decoded_size_bytes",
                    "max_decoded_size_bytes must be greater than zero",
                ));
            }
        },
        None => DEFAULT_INLINE_IMAGE_LIMIT,
    };

    let mut update = InlineImageConfigUpdate::default();
    update.max_decoded_size_bytes = Some(max_bytes);

    if let Some(value) = map.get("filename_prefix") {
        let prefix = value
            .decode::<String>()
            .map_err(|_| bad_option_msg("inline_image_config.filename_prefix", "must be a string"))?;
        if !prefix.trim().is_empty() {
            update.filename_prefix = Some(prefix);
        }
    }

    if let Some(value) = map.get("capture_svg") {
        update.capture_svg = Some(
            value
                .decode::<bool>()
                .map_err(|_| bad_option_msg("inline_image_config.capture_svg", "must be a boolean"))?,
        );
    }

    if let Some(value) = map.get("infer_dimensions") {
        update.infer_dimensions = Some(
            value
                .decode::<bool>()
                .map_err(|_| bad_option_msg("inline_image_config.infer_dimensions", "must be a boolean"))?,
        );
    }

    Ok(InlineImageConfig::from_update(update))
}

fn decode_string_list(term: Term, field: &'static str) -> NifResult<Vec<String>> {
    let list: Vec<Term> = term
        .decode()
        .map_err(|_| bad_option_msg(field, "must be provided as a list of strings"))?;
    let mut values = Vec::with_capacity(list.len());
    for entry in list {
        let value = entry
            .decode::<String>()
            .map_err(|_| bad_option_msg(field, "must contain only strings"))?;
        values.push(value);
    }
    Ok(values)
}

fn decode_positive_integer(term: Term, field: &'static str) -> NifResult<usize> {
    let value = term
        .decode::<i64>()
        .map_err(|_| bad_option_msg(field, format!("{field} must be a positive integer")))?;
    if value <= 0 {
        Err(bad_option_msg(field, format!("{field} must be greater than zero")))
    } else {
        Ok(value as usize)
    }
}

fn parse_heading_style(term: Term) -> NifResult<HeadingStyle> {
    let value = decode_atom_or_string(term)?;
    match value.as_str() {
        "atx" => Ok(HeadingStyle::Atx),
        "atx_closed" => Ok(HeadingStyle::AtxClosed),
        "underlined" => Ok(HeadingStyle::Underlined),
        _ => Err(bad_option_msg("heading_style", format!("invalid value: {value}"))),
    }
}

fn parse_list_indent_type(term: Term) -> NifResult<ListIndentType> {
    let value = decode_atom_or_string(term)?;
    match value.as_str() {
        "spaces" => Ok(ListIndentType::Spaces),
        "tabs" => Ok(ListIndentType::Tabs),
        _ => Err(bad_option_msg("list_indent_type", format!("invalid value: {value}"))),
    }
}

fn parse_highlight_style(term: Term) -> NifResult<HighlightStyle> {
    let value = decode_atom_or_string(term)?.replace('-', "_");
    match value.as_str() {
        "double_equal" => Ok(HighlightStyle::DoubleEqual),
        "html" => Ok(HighlightStyle::Html),
        "bold" => Ok(HighlightStyle::Bold),
        "none" => Ok(HighlightStyle::None),
        _ => Err(bad_option_msg("highlight_style", format!("invalid value: {value}"))),
    }
}

fn parse_whitespace_mode(term: Term) -> NifResult<WhitespaceMode> {
    let value = decode_atom_or_string(term)?;
    match value.as_str() {
        "normalized" => Ok(WhitespaceMode::Normalized),
        "strict" => Ok(WhitespaceMode::Strict),
        _ => Err(bad_option_msg("whitespace_mode", format!("invalid value: {value}"))),
    }
}

fn parse_newline_style(term: Term) -> NifResult<NewlineStyle> {
    let value = decode_atom_or_string(term)?;
    match value.as_str() {
        "spaces" => Ok(NewlineStyle::Spaces),
        "backslash" => Ok(NewlineStyle::Backslash),
        _ => Err(bad_option_msg("newline_style", format!("invalid value: {value}"))),
    }
}

fn parse_code_block_style(term: Term) -> NifResult<CodeBlockStyle> {
    let value = decode_atom_or_string(term)?;
    match value.as_str() {
        "indented" => Ok(CodeBlockStyle::Indented),
        "backticks" => Ok(CodeBlockStyle::Backticks),
        "tildes" => Ok(CodeBlockStyle::Tildes),
        _ => Err(bad_option_msg("code_block_style", format!("invalid value: {value}"))),
    }
}

fn parse_preset(term: Term) -> NifResult<PreprocessingPreset> {
    let value = decode_atom_or_string(term)?.replace('-', "_");
    match value.as_str() {
        "minimal" => Ok(PreprocessingPreset::Minimal),
        "aggressive" => Ok(PreprocessingPreset::Aggressive),
        "standard" => Ok(PreprocessingPreset::Standard),
        _ => Err(bad_option_msg(
            "preprocessing.preset",
            format!("invalid value: {value}"),
        )),
    }
}

fn decode_bool(term: Term, field: &'static str) -> NifResult<bool> {
    term.decode::<bool>()
        .map_err(|_| bad_option_msg(field, "must be a boolean"))
}

fn decode_string(term: Term, field: &'static str) -> NifResult<String> {
    term.decode::<String>()
        .map_err(|_| bad_option_msg(field, "must be a string"))
}

fn decode_atom_or_string(term: Term) -> NifResult<String> {
    if let Ok(atom) = term.atom_to_string() {
        Ok(atom)
    } else {
        term.decode::<String>()
    }
}
