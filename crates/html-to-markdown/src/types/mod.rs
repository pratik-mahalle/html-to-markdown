//! Core types for structured HTML extraction results.
//!
//! These types are aligned with kreuzberg's `DocumentStructure` model for seamless integration.

mod document;
mod result;
mod tables;
mod warnings;

pub use document::{AnnotationKind, DocumentNode, DocumentStructure, NodeContent, TextAnnotation};
pub use result::ConversionResult;
pub use tables::{GridCell, TableData, TableGrid};
pub use warnings::{ProcessingWarning, WarningKind};
