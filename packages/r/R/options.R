#' Create conversion options for html-to-markdown.
#'
#' Returns a named list of conversion options to pass to conversion functions.
#' All parameters are optional; NULL values are omitted.
#'
#' @param heading_style Style for headings: "atx", "atx_closed", or "underlined".
#' @param list_indent_type Indent type for lists: "spaces" or "tabs".
#' @param list_indent_width Number of spaces/tabs per indent level.
#' @param bullets Characters to use for bullet points.
#' @param strong_em_symbol Character for strong/emphasis: "*" or "_".
#' @param escape_asterisks Whether to escape asterisks.
#' @param escape_underscores Whether to escape underscores.
#' @param escape_misc Whether to escape miscellaneous characters.
#' @param escape_ascii Whether to escape ASCII characters.
#' @param code_language Default language for code blocks.
#' @param encoding Input encoding (e.g., "utf-8").
#' @param autolinks Whether to use autolinks for URLs.
#' @param default_title Whether to use default title attributes.
#' @param keep_inline_images_in Tags to preserve inline images in.
#' @param br_in_tables Whether to use br tags in table cells.
#' @param highlight_style Highlight style: "double_equal", "html", "bold", "none".
#' @param extract_metadata Whether to extract metadata.
#' @param whitespace_mode Whitespace handling: "normalized" or "strict".
#' @param strip_newlines Whether to strip newlines.
#' @param wrap Whether to wrap text.
#' @param wrap_width Maximum line width for wrapping.
#' @param strip_tags Tags to strip from output.
#' @param preserve_tags Tags to preserve in output.
#' @param convert_as_inline Whether to convert as inline.
#' @param sub_symbol Symbol for subscript.
#' @param sup_symbol Symbol for superscript.
#' @param newline_style Newline style: "spaces" or "backslash".
#' @param code_block_style Code block style: "indented", "backticks", or "tildes".
#' @param preprocessing Named list with preprocessing options.
#' @param debug Whether to enable debug output.
#' @return A named list of conversion options.
#' @export
conversion_options <- function(
  heading_style = NULL,
  list_indent_type = NULL,
  list_indent_width = NULL,
  bullets = NULL,
  strong_em_symbol = NULL,
  escape_asterisks = NULL,
  escape_underscores = NULL,
  escape_misc = NULL,
  escape_ascii = NULL,
  code_language = NULL,
  encoding = NULL,
  autolinks = NULL,
  default_title = NULL,
  keep_inline_images_in = NULL,
  br_in_tables = NULL,
  highlight_style = NULL,
  extract_metadata = NULL,
  whitespace_mode = NULL,
  strip_newlines = NULL,
  wrap = NULL,
  wrap_width = NULL,
  strip_tags = NULL,
  preserve_tags = NULL,
  convert_as_inline = NULL,
  sub_symbol = NULL,
  sup_symbol = NULL,
  newline_style = NULL,
  code_block_style = NULL,
  preprocessing = NULL,
  debug = NULL
) {
  opts <- list()

  if (!is.null(heading_style)) opts$heading_style <- heading_style
  if (!is.null(list_indent_type)) opts$list_indent_type <- list_indent_type
  if (!is.null(list_indent_width)) opts$list_indent_width <- as.integer(list_indent_width)
  if (!is.null(bullets)) opts$bullets <- bullets
  if (!is.null(strong_em_symbol)) opts$strong_em_symbol <- strong_em_symbol
  if (!is.null(escape_asterisks)) opts$escape_asterisks <- escape_asterisks
  if (!is.null(escape_underscores)) opts$escape_underscores <- escape_underscores
  if (!is.null(escape_misc)) opts$escape_misc <- escape_misc
  if (!is.null(escape_ascii)) opts$escape_ascii <- escape_ascii
  if (!is.null(code_language)) opts$code_language <- code_language
  if (!is.null(encoding)) opts$encoding <- encoding
  if (!is.null(autolinks)) opts$autolinks <- autolinks
  if (!is.null(default_title)) opts$default_title <- default_title
  if (!is.null(keep_inline_images_in)) opts$keep_inline_images_in <- keep_inline_images_in
  if (!is.null(br_in_tables)) opts$br_in_tables <- br_in_tables
  if (!is.null(highlight_style)) opts$highlight_style <- highlight_style
  if (!is.null(extract_metadata)) opts$extract_metadata <- extract_metadata
  if (!is.null(whitespace_mode)) opts$whitespace_mode <- whitespace_mode
  if (!is.null(strip_newlines)) opts$strip_newlines <- strip_newlines
  if (!is.null(wrap)) opts$wrap <- wrap
  if (!is.null(wrap_width)) opts$wrap_width <- as.integer(wrap_width)
  if (!is.null(strip_tags)) opts$strip_tags <- strip_tags
  if (!is.null(preserve_tags)) opts$preserve_tags <- preserve_tags
  if (!is.null(convert_as_inline)) opts$convert_as_inline <- convert_as_inline
  if (!is.null(sub_symbol)) opts$sub_symbol <- sub_symbol
  if (!is.null(sup_symbol)) opts$sup_symbol <- sup_symbol
  if (!is.null(newline_style)) opts$newline_style <- newline_style
  if (!is.null(code_block_style)) opts$code_block_style <- code_block_style
  if (!is.null(preprocessing)) opts$preprocessing <- preprocessing
  if (!is.null(debug)) opts$debug <- debug

  opts
}
