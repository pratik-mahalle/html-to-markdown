use ext_php_rs::prelude::*;
use ext_php_rs::types::{ArrayKey, Zval};
use html_to_markdown_bindings_common::error::error_message;
use html_to_markdown_rs::ConversionError;

/// Convert a ConversionError to a PHP exception.
pub fn to_php_exception(err: ConversionError) -> PhpException {
    PhpException::default(error_message(&err))
}

/// Read a boolean value from a Zval.
pub fn read_bool(value: &Zval, key: &str) -> PhpResult<bool> {
    value
        .bool()
        .ok_or_else(|| PhpException::default(format!("'{key}' must be a boolean (got {:?})", value.get_type())))
}

/// Read a string value from a Zval.
pub fn read_string(value: &Zval, key: &str) -> PhpResult<String> {
    value
        .string()
        .ok_or_else(|| PhpException::default(format!("'{key}' must be a string (got {:?})", value.get_type())))
}

/// Read an unsigned size (usize) value from a Zval.
pub fn read_usize(value: &Zval, key: &str) -> PhpResult<usize> {
    let number = value
        .long()
        .ok_or_else(|| PhpException::default(format!("'{key}' must be an integer (got {:?})", value.get_type())))?;
    if number < 0 {
        return Err(PhpException::default(format!("'{key}' must be a non-negative integer")));
    }
    Ok(number as usize)
}

/// Read an unsigned 64-bit integer value from a Zval.
pub fn read_u64(value: &Zval, key: &str) -> PhpResult<u64> {
    let number = value
        .long()
        .ok_or_else(|| PhpException::default(format!("'{key}' must be an integer (got {:?})", value.get_type())))?;

    if number < 0 {
        return Err(PhpException::default(format!("'{key}' must be a non-negative integer")));
    }

    Ok(number as u64)
}

/// Read a list of strings from a Zval array.
pub fn read_string_list(value: &Zval, key: &str) -> PhpResult<Vec<String>> {
    let array = value
        .array()
        .ok_or_else(|| PhpException::default(format!("'{key}' must be an array of strings")))?;

    let mut result = Vec::with_capacity(array.len());

    for (_, element) in array {
        if element.is_null() {
            continue;
        }

        result.push(read_string(element, key)?);
    }

    Ok(result)
}

/// Parse a single character from a string Zval.
pub fn parse_single_char(value: &Zval, key: &str) -> PhpResult<char> {
    let string = read_string(value, key)?;
    let mut chars = string.chars();
    if let Some(ch) = chars.next() {
        if chars.next().is_some() {
            Err(PhpException::default(format!("'{key}' must be a single character")))
        } else {
            Ok(ch)
        }
    } else {
        Err(PhpException::default(format!("'{key}' must not be empty")))
    }
}

/// Convert an array key to a string.
pub fn key_to_string(key: &ArrayKey<'_>) -> PhpResult<String> {
    String::try_from(key.clone())
        .map_err(|_| PhpException::default("Option keys must be representable as strings".to_string()))
}

/// Calculate capacity for ZendHashTable based on length.
pub fn table_capacity(len: usize) -> u32 {
    len.min(u32::MAX as usize) as u32
}
