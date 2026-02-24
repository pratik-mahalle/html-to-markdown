//! Visitor pattern integration for R bindings.
//!
//! Currently implements a simplified version that performs standard
//! HTML->Markdown conversion. A full implementation would use R callback
//! functions for each visitor event.

use html_to_markdown_rs::{ConversionOptions, Result, convert as convert_inner};

/// Convert HTML to Markdown with a visitor.
///
/// Currently performs standard conversion. The visitor parameter is
/// accepted for API parity but not yet invoked.
pub fn convert_with_visitor(html: &str, options: ConversionOptions) -> Result<String> {
    convert_inner(html, Some(options))
}
