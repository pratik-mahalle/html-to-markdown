//! Text processing module for HTML to Markdown conversion.
//!
//! This module provides utilities for normalizing, escaping, and processing text content
//! extracted from HTML documents during the conversion to Markdown format.

mod escaping;
mod normalization;
mod processing;

pub use processing::dedent_code_block;
