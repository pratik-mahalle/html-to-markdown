//! Type helpers and error utilities for Ruby bindings.

use magnus::{Error, Ruby, Symbol, TryConvert, Value};

/// Create an ArgumentError.
pub fn arg_error(message: impl Into<String>) -> Error {
    let ruby = Ruby::get().expect("Ruby not initialised");
    Error::new(ruby.exception_arg_error(), message.into())
}

/// Create a RuntimeError.
pub fn runtime_error(message: impl Into<String>) -> Error {
    let ruby = Ruby::get().expect("Ruby not initialised");
    Error::new(ruby.exception_runtime_error(), message.into())
}

/// Convert a Ruby Symbol or String to a Rust String.
pub fn symbol_to_string(value: Value) -> Result<String, Error> {
    if let Some(symbol) = Symbol::from_value(value) {
        Ok(symbol.name()?.to_string())
    } else {
        String::try_convert(value)
    }
}
