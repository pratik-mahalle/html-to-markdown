//! Visitor pattern FFI bridge for C-compatible languages
//!
//! This module provides a C-compatible FFI interface to the visitor pattern
//! implemented in the core html-to-markdown library. It allows languages like
//! Go, Java, C#, and others to implement custom HTML element processing logic.
//!
//! # Module Organization
//!
//! - `types`: Core FFI types (results, node types, attributes, context)
//! - `callbacks_core`: Core callback function type definitions
//! - `callbacks_extra`: Extended callback function type definitions
//! - `registry`: Visitor callbacks registry structure
//! - `wrapper`: Internal visitor wrapper implementation
//! - `visitor_impl_core`: Core visitor trait method implementations
//! - `visitor_impl_extra`: Additional visitor trait method implementations
//! - `lifecycle`: C FFI functions for visitor lifecycle management

pub mod types;

#[cfg(feature = "visitor")]
pub mod callbacks_core;

#[cfg(feature = "visitor")]
pub mod callbacks_extra;

#[cfg(feature = "visitor")]
pub mod registry;

#[cfg(feature = "visitor")]
pub mod wrapper;

#[cfg(feature = "visitor")]
pub mod visitor_impl;

#[cfg(feature = "visitor")]
pub mod lifecycle;

// Re-export all public types and functions for convenience
#[cfg(feature = "visitor")]
pub use types::*;

#[cfg(feature = "visitor")]
pub use callbacks_core::*;

#[cfg(feature = "visitor")]
pub use callbacks_extra::*;

#[cfg(feature = "visitor")]
pub use registry::*;

#[cfg(feature = "visitor")]
pub use lifecycle::*;
