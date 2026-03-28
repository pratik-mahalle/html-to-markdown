#![allow(clippy::all, clippy::pedantic, clippy::nursery, missing_docs)]

use html_to_markdown_rs::{
    ConversionOptions, convert as convert_rs,
    convert_with_inline_images as convert_with_inline_images_inner, error::ConversionError, safety::guard_panic,
};

#[cfg(feature = "visitor")]
use html_to_markdown_rs::convert_with_tables as convert_with_tables_inner;

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

#[cfg(feature = "metadata")]
use html_to_markdown_rs::metadata::HtmlMetadata as RustHtmlMetadata;

#[cfg(feature = "visitor")]
use conversion::tables_result_to_ruby;

#[cfg(feature = "visitor")]
use visitor::RubyVisitorWrapper;

use magnus::prelude::*;
use magnus::{Error, Ruby, TryConvert, Value, function, scan_args::scan_args};


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

    guard_panic(|| {
        profiling::maybe_profile(|| convert_rs(&html, Some(options)).map(|r| r.content.unwrap_or_default()))
    })
    .map_err(conversion_error)
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
fn convert_with_tables_fn(ruby: &Ruby, args: &[Value]) -> Result<Value, Error> {
    let parsed = scan_args::<(String,), (Option<Value>, Option<Value>), (), (), (), ()>(args)?;
    let html = parsed.required.0;
    let options = build_conversion_options(ruby, parsed.optional.0)?;

    #[cfg(feature = "metadata")]
    let metadata_config = Some(build_metadata_config(ruby, parsed.optional.1)?);
    #[cfg(not(feature = "metadata"))]
    let metadata_config: Option<()> = None;

    let result =
        guard_panic(|| convert_with_tables_inner(&html, Some(options), metadata_config)).map_err(conversion_error)?;

    tables_result_to_ruby(ruby, result)
}
fn convert_full_fn(ruby: &Ruby, args: &[Value]) -> Result<Value, Error> {
    let parsed = scan_args::<(String,), (Option<Value>,), (), (), (), ()>(args)?;
    let html = parsed.required.0;
    let options = build_conversion_options(ruby, parsed.optional.0)?;

    let result = guard_panic(|| {
        profiling::maybe_profile(|| html_to_markdown_rs::convert(&html, Some(options.clone())))
    })
    .map_err(conversion_error)?;

    let hash = ruby.hash_new();

    // content: Option<String>
    match result.content {
        Some(ref s) => hash.aset(ruby.intern("content"), s.as_str())?,
        None => hash.aset(ruby.intern("content"), ruby.qnil())?,
    }

    // document: not yet exposed
    hash.aset(ruby.intern("document"), ruby.qnil())?;

    // metadata
    #[cfg(feature = "metadata")]
    {
        let metadata_value = extended_metadata_to_ruby(ruby, result.metadata)?;
        hash.aset(ruby.intern("metadata"), metadata_value)?;
    }
    #[cfg(not(feature = "metadata"))]
    hash.aset(ruby.intern("metadata"), ruby.qnil())?;

    // tables: Vec<TableData> with grid and markdown
    {
        let tables_array = ruby.ary_new();
        for table in &result.tables {
            let table_hash = ruby.hash_new();
            let grid_hash = ruby.hash_new();
            grid_hash.aset(ruby.intern("rows"), table.grid.rows as i64)?;
            grid_hash.aset(ruby.intern("cols"), table.grid.cols as i64)?;
            let cells_array = ruby.ary_new();
            for cell in &table.grid.cells {
                let cell_hash = ruby.hash_new();
                cell_hash.aset(ruby.intern("content"), cell.content.as_str())?;
                cell_hash.aset(ruby.intern("row"), cell.row as i64)?;
                cell_hash.aset(ruby.intern("col"), cell.col as i64)?;
                cell_hash.aset(ruby.intern("row_span"), cell.row_span as i64)?;
                cell_hash.aset(ruby.intern("col_span"), cell.col_span as i64)?;
                cell_hash.aset(ruby.intern("is_header"), cell.is_header)?;
                cells_array.push(cell_hash)?;
            }
            grid_hash.aset(ruby.intern("cells"), cells_array)?;
            table_hash.aset(ruby.intern("grid"), grid_hash)?;
            table_hash.aset(ruby.intern("markdown"), table.markdown.as_str())?;
            tables_array.push(table_hash)?;
        }
        hash.aset(ruby.intern("tables"), tables_array)?;
    }

    // images
    #[cfg(feature = "inline-images")]
    {
        use conversion::inline_image_to_value;
        let images_array = ruby.ary_new();
        for image in result.images {
            let image_value = inline_image_to_value(ruby, image)?;
            images_array.push(image_value)?;
        }
        hash.aset(ruby.intern("images"), images_array)?;
    }
    #[cfg(not(feature = "inline-images"))]
    {
        let empty = ruby.ary_new();
        hash.aset(ruby.intern("images"), empty)?;
    }

    // warnings
    {
        let warnings_array = ruby.ary_new();
        for warning in &result.warnings {
            let w_hash = ruby.hash_new();
            w_hash.aset(ruby.intern("message"), warning.message.as_str())?;
            let kind_str = match warning.kind {
                html_to_markdown_rs::WarningKind::ImageExtractionFailed => "image_extraction_failed",
                html_to_markdown_rs::WarningKind::EncodingFallback => "encoding_fallback",
                html_to_markdown_rs::WarningKind::TruncatedInput => "truncated_input",
                html_to_markdown_rs::WarningKind::MalformedHtml => "malformed_html",
                html_to_markdown_rs::WarningKind::SanitizationApplied => "sanitization_applied",
            };
            w_hash.aset(ruby.intern("kind"), kind_str)?;
            warnings_array.push(w_hash)?;
        }
        hash.aset(ruby.intern("warnings"), warnings_array)?;
    }

    Ok(hash.as_value())
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
    module.define_singleton_method("convert", function!(convert_full_fn, -1))?;
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
    {
        module.define_singleton_method("convert_with_tables", function!(convert_with_tables_fn, -1))?;
    }

    #[cfg(feature = "profiling")]
    {
        module.define_singleton_method("start_profiling", function!(start_profiling_fn, -1))?;
        module.define_singleton_method("stop_profiling", function!(stop_profiling_fn, -1))?;
    }

    Ok(())
}
