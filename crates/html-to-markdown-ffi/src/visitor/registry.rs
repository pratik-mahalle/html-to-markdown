//! Visitor callbacks registry structure.
//!
//! This module contains the `HtmlToMarkdownVisitorCallbacks` struct that groups
//! together all the callback function pointers and user data for the visitor pattern.

use super::callbacks_core::{
    HtmlToMarkdownVisitBlockquoteCallback, HtmlToMarkdownVisitCodeBlockCallback, HtmlToMarkdownVisitCodeInlineCallback,
    HtmlToMarkdownVisitElementEndCallback, HtmlToMarkdownVisitElementStartCallback,
    HtmlToMarkdownVisitEmphasisCallback, HtmlToMarkdownVisitHeadingCallback, HtmlToMarkdownVisitImageCallback,
    HtmlToMarkdownVisitLinkCallback, HtmlToMarkdownVisitListEndCallback, HtmlToMarkdownVisitListItemCallback,
    HtmlToMarkdownVisitListStartCallback, HtmlToMarkdownVisitMarkCallback, HtmlToMarkdownVisitStrikethroughCallback,
    HtmlToMarkdownVisitStrongCallback, HtmlToMarkdownVisitSubscriptCallback, HtmlToMarkdownVisitSuperscriptCallback,
    HtmlToMarkdownVisitTableEndCallback, HtmlToMarkdownVisitTableRowCallback, HtmlToMarkdownVisitTableStartCallback,
    HtmlToMarkdownVisitTextCallback, HtmlToMarkdownVisitUnderlineCallback,
};
use super::callbacks_extra::{
    HtmlToMarkdownVisitAudioCallback, HtmlToMarkdownVisitButtonCallback, HtmlToMarkdownVisitCustomElementCallback,
    HtmlToMarkdownVisitDefinitionDescriptionCallback, HtmlToMarkdownVisitDefinitionListEndCallback,
    HtmlToMarkdownVisitDefinitionListStartCallback, HtmlToMarkdownVisitDefinitionTermCallback,
    HtmlToMarkdownVisitDetailsCallback, HtmlToMarkdownVisitFigcaptionCallback, HtmlToMarkdownVisitFigureEndCallback,
    HtmlToMarkdownVisitFigureStartCallback, HtmlToMarkdownVisitFormCallback, HtmlToMarkdownVisitHorizontalRuleCallback,
    HtmlToMarkdownVisitIframeCallback, HtmlToMarkdownVisitInputCallback, HtmlToMarkdownVisitLineBreakCallback,
    HtmlToMarkdownVisitSummaryCallback, HtmlToMarkdownVisitVideoCallback,
};

///
/// # Example
///
/// ```c
/// html_to_markdown_visitor_callbacks_t callbacks = {
///     .user_data = my_context_ptr,
///     .visit_text = my_visit_text_fn,
///     .visit_link = my_visit_link_fn,
///     // Leave others NULL for defaults
///     .visit_image = NULL,
///     .visit_heading = NULL,
/// };
/// ```
#[repr(C)]
#[derive(Clone)]
pub struct HtmlToMarkdownVisitorCallbacks {
    /// User-provided context pointer passed to all callbacks
    pub user_data: *mut std::ffi::c_void,

    /// Called for text nodes (most frequent)
    pub visit_text: Option<HtmlToMarkdownVisitTextCallback>,

    /// Called before entering any element
    pub visit_element_start: Option<HtmlToMarkdownVisitElementStartCallback>,

    /// Called after exiting any element
    pub visit_element_end: Option<HtmlToMarkdownVisitElementEndCallback>,

    /// Called for anchor links
    pub visit_link: Option<HtmlToMarkdownVisitLinkCallback>,

    /// Called for images
    pub visit_image: Option<HtmlToMarkdownVisitImageCallback>,

    /// Called for headings
    pub visit_heading: Option<HtmlToMarkdownVisitHeadingCallback>,

    /// Called for code blocks
    pub visit_code_block: Option<HtmlToMarkdownVisitCodeBlockCallback>,

    /// Called for inline code
    pub visit_code_inline: Option<HtmlToMarkdownVisitCodeInlineCallback>,

    /// Called for list items
    pub visit_list_item: Option<HtmlToMarkdownVisitListItemCallback>,

    /// Called before processing a list
    pub visit_list_start: Option<HtmlToMarkdownVisitListStartCallback>,

    /// Called after processing a list
    pub visit_list_end: Option<HtmlToMarkdownVisitListEndCallback>,

    /// Called before processing a table
    pub visit_table_start: Option<HtmlToMarkdownVisitTableStartCallback>,

    /// Called for table rows
    pub visit_table_row: Option<HtmlToMarkdownVisitTableRowCallback>,

    /// Called after processing a table
    pub visit_table_end: Option<HtmlToMarkdownVisitTableEndCallback>,

    /// Called for blockquotes
    pub visit_blockquote: Option<HtmlToMarkdownVisitBlockquoteCallback>,

    /// Called for strong text
    pub visit_strong: Option<HtmlToMarkdownVisitStrongCallback>,

    /// Called for emphasis text
    pub visit_emphasis: Option<HtmlToMarkdownVisitEmphasisCallback>,

    /// Called for strikethrough text
    pub visit_strikethrough: Option<HtmlToMarkdownVisitStrikethroughCallback>,

    /// Called for underline text
    pub visit_underline: Option<HtmlToMarkdownVisitUnderlineCallback>,

    /// Called for subscript text
    pub visit_subscript: Option<HtmlToMarkdownVisitSubscriptCallback>,

    /// Called for superscript text
    pub visit_superscript: Option<HtmlToMarkdownVisitSuperscriptCallback>,

    /// Called for mark text
    pub visit_mark: Option<HtmlToMarkdownVisitMarkCallback>,

    /// Called for line breaks
    pub visit_line_break: Option<HtmlToMarkdownVisitLineBreakCallback>,

    /// Called for horizontal rules
    pub visit_horizontal_rule: Option<HtmlToMarkdownVisitHorizontalRuleCallback>,

    /// Called for custom elements
    pub visit_custom_element: Option<HtmlToMarkdownVisitCustomElementCallback>,

    /// Called before processing a definition list
    pub visit_definition_list_start: Option<HtmlToMarkdownVisitDefinitionListStartCallback>,

    /// Called for definition terms
    pub visit_definition_term: Option<HtmlToMarkdownVisitDefinitionTermCallback>,

    /// Called for definition descriptions
    pub visit_definition_description: Option<HtmlToMarkdownVisitDefinitionDescriptionCallback>,

    /// Called after processing a definition list
    pub visit_definition_list_end: Option<HtmlToMarkdownVisitDefinitionListEndCallback>,

    /// Called for form elements
    pub visit_form: Option<HtmlToMarkdownVisitFormCallback>,

    /// Called for input elements
    pub visit_input: Option<HtmlToMarkdownVisitInputCallback>,

    /// Called for button elements
    pub visit_button: Option<HtmlToMarkdownVisitButtonCallback>,

    /// Called for audio elements
    pub visit_audio: Option<HtmlToMarkdownVisitAudioCallback>,

    /// Called for video elements
    pub visit_video: Option<HtmlToMarkdownVisitVideoCallback>,

    /// Called for iframe elements
    pub visit_iframe: Option<HtmlToMarkdownVisitIframeCallback>,

    /// Called for details elements
    pub visit_details: Option<HtmlToMarkdownVisitDetailsCallback>,

    /// Called for summary elements
    pub visit_summary: Option<HtmlToMarkdownVisitSummaryCallback>,

    /// Called before processing a figure
    pub visit_figure_start: Option<HtmlToMarkdownVisitFigureStartCallback>,

    /// Called for figcaption elements
    pub visit_figcaption: Option<HtmlToMarkdownVisitFigcaptionCallback>,

    /// Called after processing a figure
    pub visit_figure_end: Option<HtmlToMarkdownVisitFigureEndCallback>,
}
