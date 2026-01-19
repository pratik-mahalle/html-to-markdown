//! Visitor support for Ruby bindings.
//!
//! This module provides the bridge between Ruby visitor objects and the Rust HtmlVisitor trait.

#[cfg(feature = "visitor")]
pub mod bridge;

#[cfg(feature = "visitor")]
pub mod callbacks;

#[cfg(feature = "visitor")]
pub use bridge::RubyVisitorWrapper;
