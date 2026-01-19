#![allow(clippy::all, clippy::pedantic, clippy::nursery, missing_docs)]

use html_to_markdown_rs::{
    ConversionOptions, convert as convert_inner, convert_with_inline_images as convert_with_inline_images_inner,
    error::ConversionError, safety::guard_panic,
};

#[cfg(feature = "visitor")]
use html_to_markdown_rs::convert_with_visitor as convert_with_visitor_inner;

#[cfg(feature = "metadata")]
use html_to_markdown_rs::convert_with_metadata as convert_with_metadata_inner;

mod conversion;
mod options;
mod profiling;
mod types;

#[cfg(feature = "visitor")]
mod visitor;

use conversion::{build_inline_image_config, extraction_to_value};
use options::build_conversion_options;
use types::{arg_error, runtime_error};

#[cfg(feature = "metadata")]
use conversion::{build_metadata_config, extended_metadata_to_ruby};

#[cfg(feature = "visitor")]
use visitor::RubyVisitorWrapper;

use magnus::prelude::*;
use magnus::{Error, Ruby, TryConvert, Value, function, scan_args::scan_args};

#[cfg(feature = "visitor")]
use std::panic::AssertUnwindSafe;

#[cfg(feature = "profiling")]
use std::path::PathBuf;

#[derive(Clone)]
#[magnus::wrap(class = "HtmlToMarkdown::Options", free_immediately)]
struct OptionsHandle(ConversionOptions);

fn conversion_error(err: ConversionError) -> Error {
    match err {
        ConversionError::ConfigError(msg) => arg_error(msg),
        ConversionError::Panic(message) => {
            runtime_error(format!("html-to-markdown panic during conversion: {message}"))
        }
        other => runtime_error(other.to_string()),
    }
}

fn convert_fn(ruby: &Ruby, args: &[Value]) -> Result<String, Error> {
    let parsed = scan_args::<(String,), (Option<Value>,), (), (), (), ()>(args)?;
    let html = parsed.required.0;
    let options = build_conversion_options(ruby, parsed.optional.0)?;

    guard_panic(|| profiling::maybe_profile(|| convert_inner(&html, Some(options)))).map_err(conversion_error)
}

fn options_handle_fn(ruby: &Ruby, args: &[Value]) -> Result<OptionsHandle, Error> {
    let parsed = scan_args::<(), (Option<Value>,), (), (), (), ()>(args)?;
    let options = build_conversion_options(ruby, parsed.optional.0)?;
    Ok(OptionsHandle(options))
}

fn convert_with_options_handle_fn(_ruby: &Ruby, args: &[Value]) -> Result<String, Error> {
    let parsed = scan_args::<(String, &OptionsHandle), (), (), (), (), ()>(args)?;
    let html = parsed.required.0;
    let handle = parsed.required.1;
    let options = handle.0.clone();

    guard_panic(|| profiling::maybe_profile(|| convert_inner(&html, Some(options)))).map_err(conversion_error)
}

#[cfg(feature = "inline-images")]
fn convert_with_inline_images_fn(ruby: &Ruby, args: &[Value]) -> Result<Value, Error> {
    let parsed = scan_args::<(String,), (Option<Value>, Option<Value>), (), (), (), ()>(args)?;
    let html = parsed.required.0;
    let options = build_conversion_options(ruby, parsed.optional.0)?;
    let config = build_inline_image_config(ruby, parsed.optional.1)?;

    let extraction = guard_panic(|| convert_with_inline_images_inner(&html, Some(options), config, None))
        .map_err(conversion_error)?;

    extraction_to_value(ruby, extraction)
}

#[cfg(feature = "inline-images")]
fn convert_with_inline_images_handle_fn(ruby: &Ruby, args: &[Value]) -> Result<Value, Error> {
    let parsed = scan_args::<(String, &OptionsHandle), (Option<Value>,), (), (), (), ()>(args)?;
    let html = parsed.required.0;
    let handle = parsed.required.1;
    let options = handle.0.clone();
    let config = build_inline_image_config(ruby, parsed.optional.0)?;

    let extraction = guard_panic(|| convert_with_inline_images_inner(&html, Some(options), config, None))
        .map_err(conversion_error)?;

    extraction_to_value(ruby, extraction)
}

#[cfg(feature = "metadata")]
fn convert_with_metadata_fn(ruby: &Ruby, args: &[Value]) -> Result<Value, Error> {
    let parsed = scan_args::<(String,), (Option<Value>, Option<Value>, Option<Value>), (), (), (), ()>(args)?;
    let html = parsed.required.0;
    let options = build_conversion_options(ruby, parsed.optional.0)?;
    let metadata_config = build_metadata_config(ruby, parsed.optional.1)?;
    let _visitor = parsed.optional.2;

    let (markdown, metadata) = guard_panic(|| convert_with_metadata_inner(&html, Some(options), metadata_config, None))
        .map_err(conversion_error)?;

    let array = ruby.ary_new();
    array.push(markdown)?;
    array.push(extended_metadata_to_ruby(ruby, metadata)?)?;

    Ok(array.as_value())
}

#[cfg(feature = "metadata")]
fn convert_with_metadata_handle_fn(ruby: &Ruby, args: &[Value]) -> Result<Value, Error> {
    let parsed = scan_args::<(String, &OptionsHandle), (Option<Value>,), (), (), (), ()>(args)?;
    let html = parsed.required.0;
    let handle = parsed.required.1;
    let options = handle.0.clone();
    let metadata_config = build_metadata_config(ruby, parsed.optional.0)?;

    let (markdown, metadata) = guard_panic(|| convert_with_metadata_inner(&html, Some(options), metadata_config, None))
        .map_err(conversion_error)?;

    let array = ruby.ary_new();
    array.push(markdown)?;
    array.push(extended_metadata_to_ruby(ruby, metadata)?)?;

    Ok(array.as_value())
}

#[cfg(feature = "visitor")]
fn convert_with_visitor_fn(ruby: &Ruby, args: &[Value]) -> Result<String, Error> {
    let parsed = scan_args::<(String,), (Option<Value>, Option<Value>), (), (), (), ()>(args)?;
    let html = parsed.required.0;

    let options = match parsed.optional.0 {
        Some(opt_val) => match <&OptionsHandle>::try_convert(opt_val) {
            Ok(handle) => handle.0.clone(),
            Err(_) => build_conversion_options(ruby, Some(opt_val))?,
        },
        None => ConversionOptions::default(),
    };

    let visitor_value = match parsed.optional.1 {
        Some(val) => {
            if val.is_nil() {
                return guard_panic(AssertUnwindSafe(|| {
                    profiling::maybe_profile(|| convert_inner(&html, Some(options)))
                }))
                .map_err(conversion_error);
            }
            val
        }
        None => return Err(arg_error("visitor argument is required")),
    };

    let visitor_wrapper = RubyVisitorWrapper::new(visitor_value);
    let visitor_handle = std::rc::Rc::new(std::cell::RefCell::new(visitor_wrapper.clone()));

    let result = guard_panic(AssertUnwindSafe(|| {
        profiling::maybe_profile(|| convert_with_visitor_inner(&html, Some(options), Some(visitor_handle)))
    }))
    .map_err(conversion_error)?;

    if let Some(error_msg) = visitor_wrapper.last_error.borrow().as_ref() {
        return Err(runtime_error(error_msg.clone()));
    }

    Ok(result)
}

#[cfg(feature = "profiling")]
fn start_profiling_fn(_ruby: &Ruby, args: &[Value]) -> Result<bool, Error> {
    let output = args.first().ok_or_else(|| arg_error("output_path required"))?;
    let output: String = String::try_convert(*output)?;
    let freq = if let Some(value) = args.get(1) {
        i32::try_convert(*value)?
    } else {
        1000
    };
    profiling::start(PathBuf::from(output), freq).map_err(conversion_error)?;
    Ok(true)
}

#[cfg(feature = "profiling")]
fn stop_profiling_fn(_ruby: &Ruby, _args: &[Value]) -> Result<bool, Error> {
    profiling::stop().map_err(conversion_error)?;
    Ok(true)
}

#[magnus::init]
fn init(ruby: &Ruby) -> Result<(), Error> {
    let module = ruby.define_module("HtmlToMarkdown")?;
    module.define_singleton_method("convert", function!(convert_fn, -1))?;
    module.define_singleton_method("options", function!(options_handle_fn, -1))?;
    module.define_singleton_method("convert_with_options", function!(convert_with_options_handle_fn, -1))?;

    #[cfg(feature = "inline-images")]
    {
        module.define_singleton_method(
            "convert_with_inline_images",
            function!(convert_with_inline_images_fn, -1),
        )?;
        module.define_singleton_method(
            "convert_with_inline_images_handle",
            function!(convert_with_inline_images_handle_fn, -1),
        )?;
    }

    #[cfg(feature = "metadata")]
    {
        module.define_singleton_method("convert_with_metadata", function!(convert_with_metadata_fn, -1))?;
        module.define_singleton_method(
            "convert_with_metadata_handle",
            function!(convert_with_metadata_handle_fn, -1),
        )?;
    }

    #[cfg(feature = "visitor")]
    module.define_singleton_method("convert_with_visitor", function!(convert_with_visitor_fn, -1))?;

    #[cfg(feature = "profiling")]
    {
        module.define_singleton_method("start_profiling", function!(start_profiling_fn, -1))?;
        module.define_singleton_method("stop_profiling", function!(stop_profiling_fn, -1))?;
    }

    Ok(())
}
