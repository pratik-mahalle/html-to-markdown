//! Inline image configuration and conversion functions.

use crate::types::{arg_error, symbol_to_string};
use html_to_markdown_rs::{
    DEFAULT_INLINE_IMAGE_LIMIT, HtmlExtraction, InlineImage, InlineImageConfig, InlineImageConfigUpdate,
    InlineImageWarning,
};
use magnus::prelude::*;
use magnus::r_hash::ForEach;
use magnus::{Error, RHash, Ruby, TryConvert, Value};

pub fn build_inline_image_config(_ruby: &Ruby, config: Option<Value>) -> Result<InlineImageConfig, Error> {
    let mut update = InlineImageConfigUpdate::default();

    let Some(config) = config else {
        return Ok(InlineImageConfig::new(DEFAULT_INLINE_IMAGE_LIMIT));
    };

    if config.is_nil() {
        return Ok(InlineImageConfig::new(DEFAULT_INLINE_IMAGE_LIMIT));
    }

    let hash = RHash::from_value(config).ok_or_else(|| arg_error("inline image config must be provided as a Hash"))?;

    hash.foreach(|key: Value, val: Value| {
        let key_name = symbol_to_string(key)?;
        match key_name.as_str() {
            "max_decoded_size_bytes" => {
                update.max_decoded_size_bytes = Some(u64::try_convert(val)?);
            }
            "filename_prefix" => {
                update.filename_prefix = if val.is_nil() {
                    None
                } else {
                    Some(String::try_convert(val)?)
                };
            }
            "capture_svg" => {
                update.capture_svg = Some(bool::try_convert(val)?);
            }
            "infer_dimensions" => {
                update.infer_dimensions = Some(bool::try_convert(val)?);
            }
            _ => {}
        }
        Ok(ForEach::Continue)
    })?;

    Ok(InlineImageConfig::from_update(update))
}

pub fn inline_image_to_value(ruby: &Ruby, image: InlineImage) -> Result<Value, Error> {
    let InlineImage {
        data,
        format,
        filename,
        description,
        dimensions,
        source,
        attributes,
    } = image;

    let hash = ruby.hash_new();
    let data_value = ruby.str_from_slice(&data);
    hash.aset(ruby.intern("data"), data_value)?;

    let format_value = format.to_string();
    hash.aset(ruby.intern("format"), format_value)?;

    match filename {
        Some(name) => hash.aset(ruby.intern("filename"), name)?,
        None => hash.aset(ruby.intern("filename"), ruby.qnil())?,
    }

    match description {
        Some(desc) => hash.aset(ruby.intern("description"), desc)?,
        None => hash.aset(ruby.intern("description"), ruby.qnil())?,
    }

    if let Some((width, height)) = dimensions {
        let dims = ruby.ary_new();
        dims.push(i64::from(width))?;
        dims.push(i64::from(height))?;
        hash.aset(ruby.intern("dimensions"), dims)?;
    } else {
        hash.aset(ruby.intern("dimensions"), ruby.qnil())?;
    }

    let source_value = source.to_string();
    hash.aset(ruby.intern("source"), source_value)?;

    let attrs = ruby.hash_new();
    for (key, value) in attributes {
        attrs.aset(key, value)?;
    }
    hash.aset(ruby.intern("attributes"), attrs)?;

    Ok(hash.as_value())
}

pub fn warning_to_value(ruby: &Ruby, warning: InlineImageWarning) -> Result<Value, Error> {
    let hash = ruby.hash_new();
    hash.aset(ruby.intern("index"), warning.index as i64)?;
    hash.aset(ruby.intern("message"), warning.message)?;
    Ok(hash.as_value())
}

pub fn extraction_to_value(ruby: &Ruby, extraction: HtmlExtraction) -> Result<Value, Error> {
    let hash = ruby.hash_new();
    hash.aset(ruby.intern("markdown"), extraction.markdown)?;

    let inline_images = ruby.ary_new();
    for image in extraction.inline_images {
        inline_images.push(inline_image_to_value(ruby, image)?)?;
    }
    hash.aset(ruby.intern("inline_images"), inline_images)?;

    let warnings = ruby.ary_new();
    for warning in extraction.warnings {
        warnings.push(warning_to_value(ruby, warning)?)?;
    }
    hash.aset(ruby.intern("warnings"), warnings)?;

    Ok(hash.as_value())
}
