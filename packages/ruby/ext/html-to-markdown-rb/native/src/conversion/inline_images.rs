//! Inline image configuration and conversion functions.

use html_to_markdown_rs::InlineImage;
use magnus::prelude::*;
use magnus::{Error, Ruby, Value};

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
