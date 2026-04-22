//! HTML preprocessing and validation helpers.
//!
//! This module contains helper functions for preprocessing HTML before conversion,
//! including validation and normalization checks.

use crate::converter::dom_context::DomContext;
use crate::converter::main_helpers::is_inline_element;
use crate::converter::utility::attributes::element_has_navigation_hint;
use crate::converter::utility::content::normalized_tag_name;
use crate::options::ConversionOptions;

/// Check if an inline ancestor element is allowed to contain block-level elements.
pub fn inline_ancestor_allows_block(tag_name: &str) -> bool {
    matches!(tag_name, "a" | "ins" | "del")
}

/// Detect block elements that were incorrectly nested under inline ancestors.
///
/// Excludes elements inside `<pre>` or `<code>` blocks, as they have special
/// whitespace preservation rules and should not be repaired.
pub fn has_inline_block_misnest(dom_ctx: &DomContext, parser: &tl::Parser) -> bool {
    for handle in dom_ctx.node_map.iter().flatten() {
        if let Some(tl::Node::Tag(_tag)) = handle.get(parser) {
            let is_block = dom_ctx
                .tag_info(handle.get_inner(), parser)
                .map(|info| info.is_block)
                .unwrap_or(false);
            if is_block {
                // Check if this block element or any ancestor is pre/code
                let mut check_parent = Some(handle.get_inner());
                let mut inside_preformatted = false;
                while let Some(node_id) = check_parent {
                    if let Some(info) = dom_ctx.tag_info(node_id, parser) {
                        if matches!(info.name.as_str(), "pre" | "code") {
                            inside_preformatted = true;
                            break;
                        }
                    }
                    check_parent = dom_ctx.parent_of(node_id);
                }

                // Skip misnesting check for elements inside pre/code blocks
                if inside_preformatted {
                    continue;
                }

                let mut current = dom_ctx.parent_of(handle.get_inner());
                while let Some(parent_id) = current {
                    if let Some(parent_info) = dom_ctx.tag_info(parent_id, parser) {
                        if is_inline_element(&parent_info.name) && !inline_ancestor_allows_block(&parent_info.name) {
                            return true;
                        }
                    } else if let Some(parent_handle) = dom_ctx.node_handle(parent_id) {
                        if let Some(tl::Node::Tag(parent_tag)) = parent_handle.get(parser) {
                            let parent_name = normalized_tag_name(parent_tag.name().as_utf8_str());
                            if is_inline_element(&parent_name) && !inline_ancestor_allows_block(&parent_name) {
                                return true;
                            }
                        }
                    }
                    current = dom_ctx.parent_of(parent_id);
                }
            }
        }
    }

    false
}

/// Determine if a node should be dropped during preprocessing.
///
/// Behavior depends on the [`PreprocessingPreset`]:
///
/// - **Minimal**: Only scripts/styles are stripped (handled elsewhere). This function
///   drops nothing — all structural elements are preserved.
/// - **Standard** (default): Drops `<nav>` unconditionally. Drops `<header>`, `<footer>`,
///   and `<aside>` only when they have navigation hints (class/role/aria attributes
///   indicating site chrome). Drops `<form>` when `remove_forms` is enabled.
/// - **Aggressive**: Drops `<nav>`, `<footer>`, `<aside>` unconditionally. Drops
///   `<header>` only with navigation hints (to preserve article titles). Drops `<form>`
///   when `remove_forms` is enabled.
pub fn should_drop_for_preprocessing(
    _node_handle: &tl::NodeHandle,
    tag_name: &str,
    tag: &tl::HTMLTag,
    _parser: &tl::Parser,
    _dom_ctx: &DomContext,
    options: &ConversionOptions,
) -> bool {
    use crate::options::PreprocessingPreset;

    if !options.preprocessing.enabled {
        return false;
    }

    let preset = options.preprocessing.preset;

    // Minimal preset: drop nothing here (scripts/styles handled in earlier pipeline stage).
    if preset == PreprocessingPreset::Minimal {
        return false;
    }

    // Form removal — applies to both Standard and Aggressive when enabled.
    if options.preprocessing.remove_forms && tag_name == "form" {
        return true;
    }

    // Navigation removal — only when the flag is enabled.
    if !options.preprocessing.remove_navigation {
        return false;
    }

    let has_nav_hint = element_has_navigation_hint(tag);
    let is_aggressive = preset == PreprocessingPreset::Aggressive;

    // <nav> is always navigation — drop in both Standard and Aggressive.
    if tag_name == "nav" {
        return true;
    }

    if tag_name == "header" {
        // Drop <header> only with navigation hints (e.g. class="site-header",
        // role="navigation"). A plain <header> often wraps article titles like
        // <header><h1>Title</h1></header> — dropping it loses content.
        // This applies to BOTH Standard and Aggressive.
        return has_nav_hint;
    }

    if tag_name == "footer" || tag_name == "aside" {
        // Standard: drop only with navigation hints.
        // Aggressive: drop unconditionally — footer/aside content is rarely
        // the primary article content.
        return is_aggressive || has_nav_hint;
    }

    false
}
