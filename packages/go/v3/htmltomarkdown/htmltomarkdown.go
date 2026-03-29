// Package htmltomarkdown provides Go bindings for the html-to-markdown Rust library.
//
// This package uses cgo to call the C FFI interface exposed by the Rust library.
// It provides a simple, idiomatic Go API for converting HTML to Markdown.
//
// Example usage:
//
//	import "github.com/kreuzberg-dev/html-to-markdown/packages/go/v3/htmltomarkdown"
//
//	func main() {
//	    html := "<h1>Hello World</h1><p>This is a paragraph.</p>"
//	    markdown, err := htmltomarkdown.Convert(html)
//	    if err != nil {
//	        log.Fatal(err)
//	    }
//	    fmt.Println(markdown)
//	}
package htmltomarkdown

// #include <stdlib.h>
// #include <stdbool.h>
// #include <stdint.h>
//
// char* html_to_markdown_convert_proxy(const char* html, const char* options_json);
// void html_to_markdown_free_string_proxy(char* s);
// const char* html_to_markdown_version_proxy(void);
// const char* html_to_markdown_last_error_proxy(void);
// bool html_to_markdown_profile_start_proxy(const char* output, int32_t frequency);
// bool html_to_markdown_profile_stop_proxy(void);
import "C"
import (
	"encoding/json"
	"errors"
	"unsafe"
)

const unknownValue = "unknown"

// Warning represents a warning emitted during HTML to Markdown conversion.
type Warning struct {
	Message string `json:"message"`
	Kind    string `json:"kind"`
}

// TableData represents a table extracted during conversion.
type TableData struct {
	Cells      [][]string `json:"cells"`
	Markdown   string     `json:"markdown"`
	IsHeaderRow []bool    `json:"is_header_row"`
}

// ConversionResult holds the structured output from HTML to Markdown conversion.
type ConversionResult struct {
	Content  *string           `json:"content"`
	Document json.RawMessage   `json:"document"`
	Metadata json.RawMessage   `json:"metadata"`
	Tables   []TableData       `json:"tables"`
	Images   []json.RawMessage `json:"images"`
	Warnings []Warning         `json:"warnings"`
}

// MetadataResult holds the markdown content and extended metadata from conversion.
// Convert converts HTML to Markdown.
//
// It returns a ConversionResult containing the converted content, metadata,
// tables, images, and warnings. All fields are available in a single call.
//
// An optional JSON options string can be passed to configure conversion behavior.
// When omitted, default options are used.
//
// Example:
//
//	result, err := htmltomarkdown.Convert("<h1>Title</h1>")
//	if err != nil {
//	    log.Fatal(err)
//	}
//	if result.Content != nil {
//	    fmt.Println(*result.Content)
//	}
//
// With options:
//
//	opts := `{"heading_style":"setext"}`
//	result, err := htmltomarkdown.Convert("<h1>Title</h1>", opts)
func Convert(html string, optionsJSON ...string) (*ConversionResult, error) {
	if html == "" {
		empty := ""
		return &ConversionResult{Content: &empty}, nil
	}
	if err := ensureFFILoaded(); err != nil {
		return nil, err
	}

	cHTML := C.CString(html)
	defer C.free(unsafe.Pointer(cHTML))

	var cOptions *C.char
	if len(optionsJSON) > 0 && optionsJSON[0] != "" {
		cOptions = C.CString(optionsJSON[0])
		defer C.free(unsafe.Pointer(cOptions))
	}

	cResult := C.html_to_markdown_convert_proxy(cHTML, cOptions)
	if cResult == nil {
		errMsg := C.html_to_markdown_last_error_proxy()
		if errMsg != nil {
			return nil, errors.New(C.GoString(errMsg))
		}
		return nil, errors.New("html to markdown conversion failed")
	}
	defer C.html_to_markdown_free_string_proxy(cResult)

	jsonStr := C.GoString(cResult)

	var result ConversionResult
	if err := json.Unmarshal([]byte(jsonStr), &result); err != nil {
		return nil, errors.New("failed to parse conversion result JSON: " + err.Error())
	}

	return &result, nil
}

// MustConvert is like Convert but panics if an error occurs.
//
// An optional JSON options string can be passed to configure conversion behavior.
//
// Example:
//
//	result := htmltomarkdown.MustConvert("<h1>Title</h1>")
//	fmt.Println(*result.Content)
func MustConvert(html string, optionsJSON ...string) *ConversionResult {
	result, err := Convert(html, optionsJSON...)
	if err != nil {
		panic(err)
	}
	return result
}

// Version returns the version string of the underlying html-to-markdown library.
//
// Example:
//
//	version := htmltomarkdown.Version()
//	fmt.Printf("Using html-to-markdown version: %s\n", version)
func Version() string {
	if err := ensureFFILoaded(); err != nil {
		return unknownValue
	}
	cVersion := C.html_to_markdown_version_proxy()
	if cVersion == nil {
		return unknownValue
	}
	return C.GoString(cVersion)
}

// StartProfiling begins Rust-side profiling and writes a flamegraph to outputPath.
func StartProfiling(outputPath string, frequency int) error {
	if outputPath == "" {
		return errors.New("output path is required")
	}
	if err := ensureFFILoaded(); err != nil {
		return err
	}
	if frequency <= 0 {
		frequency = 1000
	}
	cOutput := C.CString(outputPath)
	defer C.free(unsafe.Pointer(cOutput))

	ok := C.html_to_markdown_profile_start_proxy(cOutput, C.int32_t(frequency))
	if !bool(ok) {
		errMsg := C.html_to_markdown_last_error_proxy()
		if errMsg != nil {
			return errors.New(C.GoString(errMsg))
		}
		return errors.New("profiling start failed")
	}
	return nil
}

// StopProfiling stops Rust-side profiling and flushes the flamegraph.
func StopProfiling() error {
	if err := ensureFFILoaded(); err != nil {
		return err
	}
	ok := C.html_to_markdown_profile_stop_proxy()
	if !bool(ok) {
		errMsg := C.html_to_markdown_last_error_proxy()
		if errMsg != nil {
			return errors.New(C.GoString(errMsg))
		}
		return errors.New("profiling stop failed")
	}
	return nil
}

// TextDirection represents the directionality of text content.
//
// This enum is used to indicate whether text flows left-to-right (as in English)
// or right-to-left (as in Arabic, Hebrew, etc.).
type TextDirection string

const (
	TextDirectionLTR  TextDirection = "ltr"
	TextDirectionRTL  TextDirection = "rtl"
	TextDirectionAuto TextDirection = "auto"
)

// LinkType represents the classification of a hyperlink.
//
// Links are categorized based on their href value and document context,
// useful for filtering and analysis.
type LinkType string

const (
	LinkTypeAnchor   LinkType = "anchor"
	LinkTypeInternal LinkType = "internal"
	LinkTypeExternal LinkType = "external"
	LinkTypeEmail    LinkType = "email"
	LinkTypePhone    LinkType = "phone"
	LinkTypeOther    LinkType = "other"
)

// ImageType represents the classification of an image source.
//
// Images are categorized based on their source to determine how they should
// be handled and processed.
type ImageType string

const (
	ImageTypeDataURI   ImageType = "data_uri"
	ImageTypeInlineSVG ImageType = "inline_svg"
	ImageTypeExternal  ImageType = "external"
	ImageTypeRelative  ImageType = "relative"
)

// StructuredDataType represents the format of structured data markup.
//
// This identifies which schema/format is used for machine-readable structured data.
type StructuredDataType string

const (
	StructuredDataTypeJSONLD    StructuredDataType = "json_ld"
	StructuredDataTypeMicrodata StructuredDataType = "microdata"
	StructuredDataTypeRDFa      StructuredDataType = "rdfa"
)

// OutputFormat represents the target markup language format for conversion output.
//
// Specifies whether the conversion should produce standard Markdown (CommonMark compatible)
// or Djot lightweight markup language.
type OutputFormat string

const (
	// OutputFormatMarkdown produces standard Markdown output (CommonMark compatible). Default.
	OutputFormatMarkdown OutputFormat = "markdown"
	// OutputFormatDjot produces Djot lightweight markup language output.
	OutputFormatDjot OutputFormat = "djot"
)

// DocumentMetadata contains document-level metadata extracted from head and top-level elements.
//
// This includes metadata typically used by search engines, social media platforms,
// and browsers for document indexing and presentation.
type DocumentMetadata struct {
	Title *string `json:"title,omitempty"`

	Description *string `json:"description,omitempty"`

	Keywords []string `json:"keywords,omitempty"`

	Author *string `json:"author,omitempty"`

	CanonicalURL *string `json:"canonical_url,omitempty"`

	BaseHref *string `json:"base_href,omitempty"`

	Language *string `json:"language,omitempty"`

	TextDirection *TextDirection `json:"text_direction,omitempty"`

	OpenGraph map[string]string `json:"open_graph,omitempty"`

	TwitterCard map[string]string `json:"twitter_card,omitempty"`

	MetaTags map[string]string `json:"meta_tags,omitempty"`
}

// HeaderMetadata contains header element metadata with hierarchy tracking.
//
// Captures heading elements (h1-h6) with their text content, identifiers,
// and position in the document structure.
type HeaderMetadata struct {
	Level uint8 `json:"level"`

	Text string `json:"text"`

	ID *string `json:"id,omitempty"`

	Depth uint32 `json:"depth"`

	HTMLOffset uint32 `json:"html_offset"`
}

// LinkMetadata contains hyperlink metadata with categorization and attributes.
//
// Represents <a> elements with parsed href values, text content, and link type classification.
type LinkMetadata struct {
	Href string `json:"href"`

	Text string `json:"text"`

	Title *string `json:"title,omitempty"`

	LinkType LinkType `json:"link_type"`

	Rel []string `json:"rel,omitempty"`

	Attributes map[string]string `json:"attributes,omitempty"`
}

// ImageMetadata contains image metadata with source and dimensions.
//
// Captures <img> elements and inline <svg> elements with metadata
// for image analysis and optimization.
type ImageMetadata struct {
	Src string `json:"src"`

	Alt *string `json:"alt,omitempty"`

	Title *string `json:"title,omitempty"`

	Dimensions *[2]uint32 `json:"dimensions,omitempty"`

	ImageType ImageType `json:"image_type"`

	Attributes map[string]string `json:"attributes,omitempty"`
}

// StructuredData represents a structured data block (JSON-LD, Microdata, or RDFa).
//
// Represents machine-readable structured data found in the document.
// JSON-LD blocks are collected as raw JSON strings for flexibility.
type StructuredData struct {
	DataType StructuredDataType `json:"data_type"`

	RawJSON string `json:"raw_json"`

	SchemaType *string `json:"schema_type,omitempty"`
}

// ExtendedMetadata is the comprehensive metadata extraction result from an HTML document.
//
// Contains all extracted metadata types in a single structure,
// suitable for serialization and transmission across language boundaries.
type ExtendedMetadata struct {
	Document DocumentMetadata `json:"document"`

	Headers []HeaderMetadata `json:"headers,omitempty"`

	Links []LinkMetadata `json:"links,omitempty"`

	Images []ImageMetadata `json:"images,omitempty"`

	StructuredData []StructuredData `json:"structured_data,omitempty"`
}
