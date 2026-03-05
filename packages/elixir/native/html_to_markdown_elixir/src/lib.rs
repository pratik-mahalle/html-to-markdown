#![allow(clippy::let_unit_value, deprecated)]

use std::collections::HashMap;

use html_to_markdown_rs::convert_with_metadata as convert_with_metadata_inner;
use html_to_markdown_rs::convert_with_tables as convert_with_tables_inner;
use html_to_markdown_rs::metadata::{
    DocumentMetadata, ExtendedMetadata, HeaderMetadata, ImageMetadata, LinkMetadata, StructuredData,
};
use html_to_markdown_rs::{
    ConversionOptions, HtmlExtraction, InlineImage, convert as convert_inner,
    convert_with_inline_images as convert_with_inline_images_inner,
};

mod options;
mod profiling;
mod types;
mod visitor;

use options::{
    INVALID_OPTION_ERROR, decode_inline_image_config, decode_metadata_config, decode_options_term,
    take_invalid_option_message,
};
use types::{
    DocumentMetadataTerm, ExtendedMetadataTerm, HeaderMetadataTerm, ImageMetadataTerm, InlineImageTerm,
    InlineImageWarningTerm, LinkMetadataTerm, StructuredDataTerm, TableDataTerm, TableExtractionTerm,
};

use rustler::types::binary::{Binary, OwnedBinary};
use rustler::{Encoder, Env, Error, NifResult, ResourceArc, Term};
use std::path::PathBuf;

struct OptionsHandleResource(ConversionOptions);

rustler::init!(
    "Elixir.HtmlToMarkdown.Native",
    [
        convert,
        convert_with_options_map,
        convert_with_handle,
        create_options_handle,
        convert_with_inline_images,
        convert_with_metadata,
        start_profiling,
        stop_profiling,
        convert_with_visitor,
        convert_with_tables
    ],
    load = on_load
);

#[allow(non_local_definitions)]
fn on_load(env: Env, _info: Term) -> bool {
    let _ = rustler::resource!(OptionsHandleResource, env);
    true
}

mod atoms {
    rustler::atoms! {
        ok,
        error,
        invalid_option,
        conversion_failed,
        atx,
        atx_closed,
        underlined,
        spaces,
        tabs,
        normalized,
        strict,
        minimal,
        standard,
        aggressive,
        backslash,
        indented,
        backticks,
        tildes,
        img_data_uri,
        svg_element,
    }
}

#[rustler::nif(schedule = "DirtyCpu")]
fn convert<'a>(env: Env<'a>, html: String) -> NifResult<Term<'a>> {
    match profiling::maybe_profile(|| convert_inner(&html, None)) {
        Ok(markdown) => Ok((atoms::ok(), markdown).encode(env)),
        Err(err) => Ok((atoms::error(), err.to_string()).encode(env)),
    }
}

#[rustler::nif(schedule = "DirtyCpu")]
fn convert_with_options_map<'a>(env: Env<'a>, html: String, options_term: Term<'a>) -> NifResult<Term<'a>> {
    let options = match decode_options_term(options_term) {
        Ok(options) => options,
        Err(err) => return handle_invalid_option_error(env, err),
    };

    match profiling::maybe_profile(|| convert_inner(&html, Some(options.clone()))) {
        Ok(markdown) => Ok((atoms::ok(), markdown).encode(env)),
        Err(err) => Ok((atoms::error(), err.to_string()).encode(env)),
    }
}

#[rustler::nif(schedule = "DirtyCpu")]
fn convert_with_handle<'a>(
    env: Env<'a>,
    html: String,
    handle: ResourceArc<OptionsHandleResource>,
) -> NifResult<Term<'a>> {
    match profiling::maybe_profile(|| convert_inner(&html, Some(handle.0.clone()))) {
        Ok(markdown) => Ok((atoms::ok(), markdown).encode(env)),
        Err(err) => Ok((atoms::error(), err.to_string()).encode(env)),
    }
}

#[rustler::nif]
fn start_profiling<'a>(env: Env<'a>, output: String, frequency: Option<i32>) -> NifResult<Term<'a>> {
    let freq = frequency.unwrap_or(1000);
    match profiling::start(PathBuf::from(output), freq) {
        Ok(()) => Ok(atoms::ok().encode(env)),
        Err(err) => Ok((atoms::error(), err.to_string()).encode(env)),
    }
}

#[rustler::nif]
fn stop_profiling<'a>(env: Env<'a>) -> NifResult<Term<'a>> {
    match profiling::stop() {
        Ok(()) => Ok(atoms::ok().encode(env)),
        Err(err) => Ok((atoms::error(), err.to_string()).encode(env)),
    }
}

#[rustler::nif(schedule = "DirtyCpu")]
fn convert_with_visitor<'a>(
    env: Env<'a>,
    html: String,
    options_term: Term<'a>,
    visitor_pid: Term<'a>,
) -> NifResult<Term<'a>> {
    let options = if let Ok(handle) = options_term.decode::<ResourceArc<OptionsHandleResource>>() {
        handle.0.clone()
    } else {
        match decode_options_term(options_term) {
            Ok(options) => options,
            Err(err) => return handle_invalid_option_error(env, err),
        }
    };

    match visitor::convert_with_visitor(&html, options, env, visitor_pid) {
        Ok(markdown) => Ok((atoms::ok(), markdown).encode(env)),
        Err(err) => Ok((atoms::error(), err.to_string()).encode(env)),
    }
}

#[rustler::nif(schedule = "DirtyCpu")]
fn create_options_handle<'a>(env: Env<'a>, options_term: Term<'a>) -> NifResult<Term<'a>> {
    match decode_options_term(options_term) {
        Ok(options) => {
            let resource = ResourceArc::new(OptionsHandleResource(options));
            Ok((atoms::ok(), resource).encode(env))
        }
        Err(err) => handle_invalid_option_error(env, err),
    }
}

#[rustler::nif(schedule = "DirtyCpu")]
fn convert_with_inline_images<'a>(
    env: Env<'a>,
    html: String,
    options_term: Term<'a>,
    config_term: Term<'a>,
) -> NifResult<Term<'a>> {
    let options = match decode_options_term(options_term) {
        Ok(options) => options,
        Err(err) => return handle_invalid_option_error(env, err),
    };
    let config = match decode_inline_image_config(config_term) {
        Ok(config) => config,
        Err(err) => return handle_invalid_option_error(env, err),
    };

    let options = options.clone();
    let config = config.clone();
    match profiling::maybe_profile(|| {
        convert_with_inline_images_inner(&html, Some(options.clone()), config.clone(), None)
    }) {
        Ok(HtmlExtraction {
            markdown,
            inline_images,
            warnings,
        }) => {
            let images = inline_images
                .into_iter()
                .map(|image| build_inline_image(env, image))
                .collect::<NifResult<Vec<_>>>()?;

            let warning_terms: Vec<InlineImageWarningTerm> = warnings
                .into_iter()
                .map(|warning| InlineImageWarningTerm {
                    index: warning.index as i64,
                    message: warning.message,
                })
                .collect();

            Ok((atoms::ok(), (markdown, images, warning_terms)).encode(env))
        }
        Err(err) => Ok((atoms::error(), err.to_string()).encode(env)),
    }
}

#[rustler::nif(schedule = "DirtyCpu")]
fn convert_with_metadata<'a>(
    env: Env<'a>,
    html: String,
    options_term: Term<'a>,
    config_term: Term<'a>,
) -> NifResult<Term<'a>> {
    let options = match decode_options_term(options_term) {
        Ok(options) => options,
        Err(err) => return handle_invalid_option_error(env, err),
    };
    let config = match decode_metadata_config(config_term) {
        Ok(config) => config,
        Err(err) => return handle_invalid_option_error(env, err),
    };

    let options = options.clone();
    let config = config.clone();
    match profiling::maybe_profile(|| convert_with_metadata_inner(&html, Some(options.clone()), config.clone(), None)) {
        Ok((markdown, metadata)) => Ok((atoms::ok(), (markdown, build_metadata(metadata))).encode(env)),
        Err(err) => Ok((atoms::error(), err.to_string()).encode(env)),
    }
}

#[rustler::nif(schedule = "DirtyCpu")]
fn convert_with_tables<'a>(
    env: Env<'a>,
    html: String,
    options_term: Term<'a>,
    config_term: Term<'a>,
) -> NifResult<Term<'a>> {
    let options = match decode_options_term(options_term) {
        Ok(options) => options,
        Err(err) => return handle_invalid_option_error(env, err),
    };
    let config = match decode_metadata_config(config_term) {
        Ok(config) => config,
        Err(err) => return handle_invalid_option_error(env, err),
    };

    match profiling::maybe_profile(|| convert_with_tables_inner(&html, Some(options.clone()), Some(config.clone()))) {
        Ok(result) => {
            let tables: Vec<TableDataTerm> = result
                .tables
                .into_iter()
                .map(|t| TableDataTerm {
                    cells: t.cells,
                    markdown: t.markdown,
                    is_header_row: t.is_header_row,
                })
                .collect();

            let metadata = result.metadata.map(build_metadata);

            let extraction = TableExtractionTerm {
                content: result.content,
                metadata,
                tables,
            };

            Ok((atoms::ok(), extraction).encode(env))
        }
        Err(err) => Ok((atoms::error(), err.to_string()).encode(env)),
    }
}

fn build_metadata(metadata: ExtendedMetadata) -> ExtendedMetadataTerm {
    ExtendedMetadataTerm {
        document: build_document_metadata(metadata.document),
        headers: metadata.headers.into_iter().map(build_header_metadata).collect(),
        links: metadata.links.into_iter().map(build_link_metadata).collect(),
        images: metadata.images.into_iter().map(build_image_metadata).collect(),
        structured_data: metadata
            .structured_data
            .into_iter()
            .map(build_structured_data)
            .collect(),
    }
}

fn build_document_metadata(metadata: DocumentMetadata) -> DocumentMetadataTerm {
    DocumentMetadataTerm {
        title: metadata.title,
        description: metadata.description,
        keywords: metadata.keywords,
        author: metadata.author,
        canonical_url: metadata.canonical_url,
        base_href: metadata.base_href,
        language: metadata.language,
        text_direction: metadata.text_direction.map(|td| td.to_string()),
        open_graph: metadata.open_graph.into_iter().collect(),
        twitter_card: metadata.twitter_card.into_iter().collect(),
        meta_tags: metadata.meta_tags.into_iter().collect(),
    }
}

fn build_header_metadata(metadata: HeaderMetadata) -> HeaderMetadataTerm {
    HeaderMetadataTerm {
        level: metadata.level,
        text: metadata.text,
        id: metadata.id,
        depth: metadata.depth as u64,
        html_offset: metadata.html_offset as u64,
    }
}

fn build_link_metadata(metadata: LinkMetadata) -> LinkMetadataTerm {
    LinkMetadataTerm {
        href: metadata.href,
        text: metadata.text,
        title: metadata.title,
        link_type: metadata.link_type.to_string(),
        rel: metadata.rel,
        attributes: metadata.attributes.into_iter().collect(),
    }
}

fn build_image_metadata(metadata: ImageMetadata) -> ImageMetadataTerm {
    ImageMetadataTerm {
        src: metadata.src,
        alt: metadata.alt,
        title: metadata.title,
        dimensions: metadata.dimensions,
        image_type: metadata.image_type.to_string(),
        attributes: metadata.attributes.into_iter().collect(),
    }
}

fn build_structured_data(metadata: StructuredData) -> StructuredDataTerm {
    StructuredDataTerm {
        data_type: metadata.data_type.to_string(),
        raw_json: metadata.raw_json,
        schema_type: metadata.schema_type,
    }
}

fn build_inline_image<'a>(env: Env<'a>, image: InlineImage) -> NifResult<InlineImageTerm<'a>> {
    let InlineImage {
        data,
        format,
        filename,
        description,
        dimensions,
        source,
        attributes,
    } = image;

    let mut binary = OwnedBinary::new(data.len()).ok_or(Error::BadArg)?;
    binary.as_mut_slice().copy_from_slice(&data);
    let binary = Binary::from_owned(binary, env);

    let mut attr_map = HashMap::with_capacity(attributes.len());
    for (key, value) in attributes {
        attr_map.insert(key, value);
    }

    Ok(InlineImageTerm {
        data: binary,
        format: format.to_string(),
        filename,
        description,
        dimensions,
        source: source.to_string(),
        attributes: attr_map,
    })
}

fn handle_invalid_option_error<'a>(env: Env<'a>, err: Error) -> NifResult<Term<'a>> {
    match err {
        Error::Atom(atom) if atom == INVALID_OPTION_ERROR => {
            let reason = take_invalid_option_message().unwrap_or_else(|| "invalid option".to_string());
            Ok((atoms::error(), reason).encode(env))
        }
        other => Err(other),
    }
}
