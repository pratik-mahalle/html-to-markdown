#![allow(clippy::all, clippy::pedantic, clippy::nursery, missing_docs)]

use crate::validators::{
    CliCodeBlockStyle, CliHeadingStyle, CliHighlightStyle, CliListIndentType, CliNewlineStyle, CliOutputFormat,
    CliPreprocessingPreset, CliWhitespaceMode, validate_bullets, validate_strong_em_symbol,
};
use clap::{Parser, ValueEnum};
use std::path::PathBuf;

/// Convert HTML to Markdown
///
/// A fast, powerful HTML to Markdown converter with comprehensive
/// customization options. Uses the html5ever parser for standards-compliant
/// HTML processing.
#[derive(Parser)]
#[command(name = "html-to-markdown")]
#[command(version)]
#[command(about, long_about = None)]
#[command(after_help = "EXAMPLES:
    # Basic conversion from stdin
    echo '<h1>Title</h1><p>Content</p>' | html-to-markdown

    # Convert file to stdout
    html-to-markdown input.html

    # Convert and save to file
    html-to-markdown input.html -o output.md

    # Extract metadata with conversion (JSON output to stdout)
    html-to-markdown input.html --with-metadata

    # Extract specific metadata types
    html-to-markdown input.html --with-metadata --extract-headers --extract-links

    # Extract all metadata and save output
    html-to-markdown input.html --with-metadata \\
        --extract-document --extract-headers --extract-links --extract-images \\
        -o output.json

    # Generate shell completions
    html-to-markdown --generate-completion bash > html-to-markdown.bash
    html-to-markdown --generate-completion zsh > _html-to-markdown

    # Generate man page
    html-to-markdown --generate-man > html-to-markdown.1

    # Web scraping with preprocessing
    html-to-markdown page.html --preprocess --preset aggressive

    # Fetch remote HTML and convert
    html-to-markdown --url https://example.com > output.md

    # Discord/Slack-friendly (2-space indents)
    html-to-markdown input.html --list-indent-width 2

    # Custom heading and list styles
    html-to-markdown input.html \\
        --heading-style atx \\
        --bullets '*' \\
        --list-indent-width 2

For more information: https://github.com/kreuzberg-dev/html-to-markdown
")]
pub struct Cli {
    /// Input HTML file (use \"-\" or omit for stdin)
    #[arg(value_name = "FILE")]
    pub input: Option<String>,

    /// Fetch HTML from a URL (alternative to file/stdin)
    #[arg(long, value_name = "URL", conflicts_with = "input")]
    pub url: Option<String>,

    /// User-Agent header when fetching via --url (default mimics a real browser)
    #[arg(long = "user-agent", value_name = "UA", requires = "url")]
    pub user_agent: Option<String>,

    /// Output file (default: stdout)
    #[arg(short = 'o', long = "output", value_name = "FILE")]
    pub output: Option<PathBuf>,

    /// Generate shell completion script
    #[arg(long = "generate-completion", value_name = "SHELL", value_enum)]
    pub generate_completion: Option<Shell>,

    /// Generate man page
    #[arg(long = "generate-man")]
    pub generate_man: bool,

    /// Heading style
    ///
    /// Controls how headings are formatted in the output:
    /// - 'atx': # for h1, ## for h2, etc. (default, `CommonMark`)
    /// - 'underlined': h1 uses ===, h2 uses ---
    /// - 'atx-closed': # Title # with closing hashes
    #[arg(long, value_name = "STYLE")]
    #[arg(help_heading = "Heading Options")]
    pub heading_style: Option<CliHeadingStyle>,

    /// List indentation type
    #[arg(long, value_name = "TYPE")]
    #[arg(help_heading = "List Options")]
    pub list_indent_type: Option<CliListIndentType>,

    /// Spaces per list indent level
    ///
    /// Default is 2 (`CommonMark` standard). Use 4 for wider indentation.
    #[arg(long, value_name = "N", value_parser = clap::value_parser!(u8).range(1..=8))]
    #[arg(help_heading = "List Options")]
    pub list_indent_width: Option<u8>,

    /// Bullet characters for unordered lists
    ///
    /// Characters cycle through nesting levels. Default "-" uses hyphen
    /// consistently. "*+-" uses * for level 1, + for level 2, - for level 3.
    #[arg(short = 'b', long, value_name = "CHARS")]
    #[arg(help_heading = "List Options")]
    #[arg(value_parser = validate_bullets)]
    pub bullets: Option<String>,

    /// Symbol for bold and italic
    ///
    /// Choose '*' (default) or '_' for **bold** and *italic* text
    #[arg(long, value_name = "CHAR")]
    #[arg(help_heading = "Text Formatting")]
    #[arg(value_parser = validate_strong_em_symbol)]
    pub strong_em_symbol: Option<char>,

    /// Escape asterisk (*) characters
    #[arg(long)]
    #[arg(help_heading = "Text Formatting")]
    pub escape_asterisks: bool,

    /// Escape underscore (_) characters
    #[arg(long)]
    #[arg(help_heading = "Text Formatting")]
    pub escape_underscores: bool,

    /// Escape misc Markdown characters
    ///
    /// Escape characters like [, ], <, >, #, etc.
    #[arg(long)]
    #[arg(help_heading = "Text Formatting")]
    pub escape_misc: bool,

    /// Escape all ASCII punctuation
    ///
    /// For strict `CommonMark` spec compliance (usually not needed)
    #[arg(long)]
    #[arg(help_heading = "Text Formatting")]
    pub escape_ascii: bool,

    /// Symbol to wrap subscript text
    ///
    /// Example: "~" wraps <sub>text</sub> as ~text~
    #[arg(long, value_name = "SYMBOL")]
    #[arg(help_heading = "Text Formatting")]
    pub sub_symbol: Option<String>,

    /// Symbol to wrap superscript text
    ///
    /// Example: "^" wraps <sup>text</sup> as ^text^
    #[arg(long, value_name = "SYMBOL")]
    #[arg(help_heading = "Text Formatting")]
    pub sup_symbol: Option<String>,

    /// Line break style
    ///
    /// How to represent <br> tags:
    /// - 'backslash': Backslash at end of line (default, `CommonMark`)
    /// - 'spaces': Two spaces at end of line
    #[arg(long, value_name = "STYLE")]
    #[arg(help_heading = "Text Formatting")]
    pub newline_style: Option<CliNewlineStyle>,

    /// Code block style
    ///
    /// How to format code blocks:
    /// - 'indented': 4-space indentation (default, `CommonMark`)
    /// - 'backticks': Fenced with backticks (```)
    /// - 'tildes': Fenced with tildes (~~~)
    #[arg(long, value_name = "STYLE")]
    #[arg(help_heading = "Code Blocks")]
    pub code_block_style: Option<CliCodeBlockStyle>,

    /// Default language for code blocks
    ///
    /// Sets the language for fenced code blocks when not specified in HTML
    #[arg(short = 'l', long, value_name = "LANG")]
    #[arg(help_heading = "Code Blocks")]
    pub code_language: Option<String>,

    /// Convert URLs to autolinks
    ///
    /// When link text equals href, use <url> instead of [url](url)
    #[arg(short = 'a', long)]
    #[arg(help_heading = "Links")]
    pub autolinks: bool,

    /// Add default title to links
    ///
    /// Use href as link title when no title attribute exists
    #[arg(long)]
    #[arg(help_heading = "Links")]
    pub default_title: bool,

    /// Keep inline images in specific elements
    ///
    /// Comma-separated list of HTML elements where images should remain
    /// as markdown (not converted to alt text). Example: "a,strong"
    #[arg(long, value_name = "ELEMENTS", value_delimiter = ',')]
    #[arg(help_heading = "Images")]
    pub keep_inline_images_in: Option<Vec<String>>,

    /// Use <br> in table cells
    ///
    /// Preserve line breaks in table cells using <br> tags instead of
    /// converting to spaces
    #[arg(long)]
    #[arg(help_heading = "Tables")]
    pub br_in_tables: bool,

    /// Disable spatial table reconstruction for hOCR documents
    #[arg(long = "no-hocr-spatial-tables")]
    #[arg(help_heading = "Tables")]
    pub no_hocr_spatial_tables: bool,

    /// Style for <mark> elements
    ///
    /// How to represent highlighted text:
    /// - 'double-equal': ==text== (default)
    /// - 'html': <mark>text</mark>
    /// - 'bold': **text**
    /// - 'none': plain text
    #[arg(long, value_name = "STYLE")]
    #[arg(help_heading = "Highlighting")]
    pub highlight_style: Option<CliHighlightStyle>,

    /// Extract metadata from HTML
    ///
    /// Extract title and meta tags as HTML comment header
    #[arg(long)]
    #[arg(help_heading = "Metadata")]
    pub extract_metadata: bool,

    /// Extract comprehensive metadata and output as JSON
    ///
    /// When enabled, output will be JSON with "markdown" and "metadata" keys.
    /// Use --extract-document, --extract-headers, etc. to control what metadata is extracted.
    #[arg(long)]
    #[arg(help_heading = "Metadata")]
    pub with_metadata: bool,

    /// Extract document-level metadata
    ///
    /// Requires --with-metadata. Extracts title, description, charset, language, etc.
    #[arg(long)]
    #[arg(help_heading = "Metadata")]
    #[arg(requires = "with_metadata")]
    pub extract_document: bool,

    /// Extract header elements
    ///
    /// Requires --with-metadata. Extracts h1-h6 headers with hierarchy.
    #[arg(long)]
    #[arg(help_heading = "Metadata")]
    #[arg(requires = "with_metadata")]
    pub extract_headers: bool,

    /// Extract link elements
    ///
    /// Requires --with-metadata. Extracts anchor tags with types (internal, external, etc.).
    #[arg(long)]
    #[arg(help_heading = "Metadata")]
    #[arg(requires = "with_metadata")]
    pub extract_links: bool,

    /// Extract image elements
    ///
    /// Requires --with-metadata. Extracts img tags with sources and metadata.
    #[arg(long)]
    #[arg(help_heading = "Metadata")]
    #[arg(requires = "with_metadata")]
    pub extract_images: bool,

    /// Extract structured data
    ///
    /// Requires --with-metadata. Extracts JSON-LD, Microdata, and `RDFa` blocks.
    #[arg(long)]
    #[arg(help_heading = "Metadata")]
    #[arg(requires = "with_metadata")]
    pub extract_structured_data: bool,

    /// Whitespace handling mode
    ///
    /// How to handle whitespace in HTML:
    /// - 'normalized': Clean up excess whitespace (default)
    /// - 'strict': Preserve whitespace as-is
    #[arg(long, value_name = "MODE")]
    #[arg(help_heading = "Whitespace")]
    pub whitespace_mode: Option<CliWhitespaceMode>,

    /// Strip newlines from input
    ///
    /// Remove all newlines from HTML before processing (useful for
    /// minified HTML)
    #[arg(long)]
    #[arg(help_heading = "Whitespace")]
    pub strip_newlines: bool,

    /// Enable text wrapping
    ///
    /// Wrap output lines at --wrap-width columns
    #[arg(short = 'w', long)]
    #[arg(help_heading = "Wrapping")]
    pub wrap: bool,

    /// Wrap width in columns
    ///
    /// Column width for text wrapping when --wrap is enabled
    #[arg(long, value_name = "N", value_parser = clap::value_parser!(u16).range(20..=500))]
    #[arg(help_heading = "Wrapping")]
    pub wrap_width: Option<u16>,

    /// Treat block elements as inline
    ///
    /// Convert block-level elements without adding paragraph breaks
    #[arg(long)]
    #[arg(help_heading = "Element Handling")]
    pub convert_as_inline: bool,

    /// HTML tags to strip
    ///
    /// Comma-separated list of HTML tags to strip (output only text content,
    /// no markdown conversion). Example: "script,style"
    #[arg(long, value_name = "TAGS", value_delimiter = ',')]
    #[arg(help_heading = "Element Handling")]
    pub strip_tags: Option<Vec<String>>,

    /// Enable HTML preprocessing
    ///
    /// Clean up HTML before conversion (removes navigation, ads, forms, etc.)
    #[arg(short = 'p', long)]
    #[arg(help_heading = "Preprocessing")]
    pub preprocess: bool,

    /// Preprocessing aggressiveness preset
    ///
    /// How aggressively to clean HTML:
    /// - 'minimal': Basic cleanup only
    /// - 'standard': Balanced cleaning (default)
    /// - 'aggressive': Maximum cleaning for web scraping
    #[arg(long, value_name = "LEVEL")]
    #[arg(help_heading = "Preprocessing")]
    #[arg(requires = "preprocess")]
    pub preset: Option<CliPreprocessingPreset>,

    /// Keep navigation elements
    ///
    /// Don't remove <nav>, menus, etc. during preprocessing
    #[arg(long)]
    #[arg(help_heading = "Preprocessing")]
    #[arg(requires = "preprocess")]
    pub keep_navigation: bool,

    /// Keep form elements
    ///
    /// Don't remove <form>, <input>, etc. during preprocessing
    #[arg(long)]
    #[arg(help_heading = "Preprocessing")]
    #[arg(requires = "preprocess")]
    pub keep_forms: bool,

    /// Input character encoding
    ///
    /// Encoding to use when reading input files (e.g., 'utf-8', 'latin-1')
    #[arg(short = 'e', long, value_name = "ENCODING", default_value = "utf-8")]
    #[arg(help_heading = "Parsing")]
    pub encoding: String,

    /// Enable debug mode
    ///
    /// Output diagnostic warnings and information
    #[arg(long)]
    #[arg(help_heading = "Debugging")]
    pub debug: bool,

    /// Output format (markdown or djot)
    ///
    /// Choose the output format:
    /// - 'markdown': Standard Markdown (CommonMark compatible, default)
    /// - 'djot': Djot lightweight markup language
    #[arg(short = 'f', long = "output-format", value_name = "FORMAT")]
    #[arg(help_heading = "Output Format")]
    pub output_format: Option<CliOutputFormat>,
}

#[derive(Copy, Clone, PartialEq, Eq, ValueEnum)]
#[allow(clippy::enum_variant_names)]
pub enum Shell {
    Bash,
    Zsh,
    Fish,
    PowerShell,
    Elvish,
}
