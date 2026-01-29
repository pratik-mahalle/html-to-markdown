package main

import (
	"strings"
	"testing"
	htmltomarkdown "github.com/kreuzberg-dev/html-to-markdown/packages/go/v2/htmltomarkdown"
)

// TestVersion verifies that the version API works correctly.
func TestVersion(t *testing.T) {
	version := htmltomarkdown.Version()
	if version == "" || version == "unknown" {
		t.Errorf("expected non-empty version, got: %s", version)
	}
	// Verify it follows semver pattern roughly
	if !strings.Contains(version, ".") {
		t.Errorf("expected version to contain '.', got: %s", version)
	}
}

// TestConvertWithMetadata verifies HTML conversion with metadata extraction.
func TestConvertWithMetadata(t *testing.T) {
	html := `
	<html>
		<head>
			<title>Test Page</title>
			<meta name="description" content="Test description">
		</head>
		<body>
			<h1>Main Title</h1>
			<p>Main content here.</p>
		</body>
	</html>
	`

	result, err := htmltomarkdown.ConvertWithMetadata(html)
	if err != nil {
		t.Fatalf("conversion with metadata failed: %v", err)
	}

	if result.Markdown == "" {
		t.Errorf("expected non-empty markdown, got: %s", result.Markdown)
	}

	// Check that title was extracted
	if result.Metadata.Document.Title == nil {
		t.Errorf("expected title to be extracted")
	}
	if result.Metadata.Document.Title != nil && *result.Metadata.Document.Title != "Test Page" {
		t.Errorf("expected title 'Test Page', got: %s", *result.Metadata.Document.Title)
	}
}

// TestConvertComplexHTML tests conversion of more complex HTML structures.
func TestConvertComplexHTML(t *testing.T) {
	tests := []struct {
		name           string
		html           string
		expectedSubstr string
	}{
		{
			name:           "nested lists",
			html:           "<ul><li>Item 1<ul><li>Nested</li></ul></li></ul>",
			expectedSubstr: "Item 1",
		},
		{
			name:           "code block",
			html:           "<pre><code>function() {}</code></pre>",
			expectedSubstr: "function",
		},
		{
			name:           "tables",
			html:           "<table><tr><td>Cell 1</td><td>Cell 2</td></tr></table>",
			expectedSubstr: "Cell 1",
		},
		{
			name:           "image",
			html:           "<img src='https://example.com/image.png' alt='Example Image'>",
			expectedSubstr: "Example Image",
		},
		{
			name:           "mixed formatting",
			html:           "<p><strong>Bold</strong> and <em>italic</em> and <u>underline</u></p>",
			expectedSubstr: "Bold",
		},
		{
			name:           "horizontal rule",
			html:           "<p>Before</p><hr><p>After</p>",
			expectedSubstr: "Before",
		},
		{
			name:           "line breaks",
			html:           "<p>Line 1<br>Line 2<br>Line 3</p>",
			expectedSubstr: "Line 1",
		},
		{
			name:           "nested div structure",
			html:           "<div><div><p>Nested content</p></div></div>",
			expectedSubstr: "Nested content",
		},
	}

	for _, test := range tests {
		t.Run(test.name, func(t *testing.T) {
			result, err := htmltomarkdown.Convert(test.html)
			if err != nil {
				t.Fatalf("conversion failed: %v", err)
			}

			if !strings.Contains(result, test.expectedSubstr) {
				t.Errorf("expected result to contain '%s', got: %s", test.expectedSubstr, result)
			}
		})
	}
}

// TestErrorHandling tests error handling for edge cases.
func TestErrorHandling(t *testing.T) {
	tests := []struct {
		name        string
		html        string
		shouldError bool
	}{
		{
			name:        "valid HTML",
			html:        "<p>Valid</p>",
			shouldError: false,
		},
		{
			name:        "malformed HTML",
			html:        "<p>Unclosed paragraph",
			shouldError: false, // HTML parser is lenient
		},
		{
			name:        "empty string",
			html:        "",
			shouldError: false,
		},
		{
			name:        "whitespace only",
			html:        "   \n\t  ",
			shouldError: false,
		},
	}

	for _, test := range tests {
		t.Run(test.name, func(t *testing.T) {
			result, err := htmltomarkdown.Convert(test.html)

			if test.shouldError && err == nil {
				t.Errorf("expected error but got none, result: %s", result)
			}
			if !test.shouldError && err != nil {
				t.Errorf("expected no error but got: %v", err)
			}
		})
	}
}

// TestMustConvert verifies the MustConvert panic behavior.
func TestMustConvert(t *testing.T) {
	// Valid HTML should not panic
	result := htmltomarkdown.MustConvert("<p>Test</p>")
	if result == "" {
		t.Errorf("expected non-empty result")
	}

	// Empty string should not panic (no error)
	result = htmltomarkdown.MustConvert("")
	if result != "" {
		t.Errorf("expected empty result for empty input")
	}
}

// TestMustConvertWithMetadata verifies the MustConvertWithMetadata function.
func TestMustConvertWithMetadata(t *testing.T) {
	html := "<h1>Title</h1><p>Content</p>"
	result := htmltomarkdown.MustConvertWithMetadata(html)

	if result.Markdown == "" {
		t.Errorf("expected non-empty markdown")
	}
	if !strings.Contains(result.Markdown, "Title") {
		t.Errorf("expected markdown to contain 'Title', got: %s", result.Markdown)
	}
}

// TestSpecialCharacters tests handling of special characters and entities.
func TestSpecialCharacters(t *testing.T) {
	tests := []struct {
		name string
		html string
	}{
		{
			name: "HTML entities",
			html: "<p>&lt;tag&gt; &amp; &quot;quoted&quot;</p>",
		},
		{
			name: "unicode characters",
			html: "<p>Hello 世界 مرحبا мир</p>",
		},
		{
			name: "emoji",
			html: "<p>Hello 👋 World 🌍</p>",
		},
		{
			name: "special symbols",
			html: "<p>€ £ ¥ © ® ™</p>",
		},
	}

	for _, test := range tests {
		t.Run(test.name, func(t *testing.T) {
			result, err := htmltomarkdown.Convert(test.html)
			if err != nil {
				t.Fatalf("conversion failed: %v", err)
			}

			if result == "" {
				t.Errorf("expected non-empty result for special characters")
			}
		})
	}
}

// TestMemorySafety tests multiple conversions to verify memory is handled safely.
func TestMemorySafety(t *testing.T) {
	htmls := []string{
		"<p>Test 1</p>",
		"<h1>Test 2</h1>",
		"<ul><li>Test 3</li></ul>",
		"",
		"<div><p>Test 4</p></div>",
	}

	// Perform multiple conversions to ensure no memory leaks or corruption
	for i := 0; i < 10; i++ {
		for _, html := range htmls {
			result, err := htmltomarkdown.Convert(html)
			if err != nil {
				t.Fatalf("conversion %d failed: %v", i, err)
			}
			if html != "" && result == "" {
				t.Errorf("expected non-empty result for non-empty input at iteration %d", i)
			}
		}
	}
}

// TestLargeHTML tests handling of large HTML documents.
func TestLargeHTML(t *testing.T) {
	// Create large HTML with many elements
	builder := strings.Builder{}
	builder.WriteString("<div>")
	for i := 0; i < 100; i++ {
		builder.WriteString("<p>Paragraph ")
		builder.WriteString(string(rune(i)))
		builder.WriteString("</p>")
	}
	builder.WriteString("</div>")

	html := builder.String()
	result, err := htmltomarkdown.Convert(html)
	if err != nil {
		t.Fatalf("conversion of large HTML failed: %v", err)
	}

	if result == "" {
		t.Errorf("expected non-empty result for large HTML")
	}

	// Verify content is preserved
	if !strings.Contains(result, "Paragraph") {
		t.Errorf("expected result to contain 'Paragraph'")
	}
}

// TestConsistentOutput verifies that multiple conversions of the same HTML produce consistent results.
func TestConsistentOutput(t *testing.T) {
	html := "<h1>Title</h1><p>Paragraph with <strong>bold</strong> and <em>italic</em>.</p>"

	result1, err1 := htmltomarkdown.Convert(html)
	if err1 != nil {
		t.Fatalf("first conversion failed: %v", err1)
	}

	result2, err2 := htmltomarkdown.Convert(html)
	if err2 != nil {
		t.Fatalf("second conversion failed: %v", err2)
	}

	if result1 != result2 {
		t.Errorf("inconsistent results:\nFirst:  %s\nSecond: %s", result1, result2)
	}
}

// TestSequentialConversions tests that conversions can be done in sequence without issues.
func TestSequentialConversions(t *testing.T) {
	conversions := []struct {
		name string
		html string
	}{
		{"heading", "<h2>Heading</h2>"},
		{"paragraph", "<p>Paragraph</p>"},
		{"link", "<a href='#'>Link</a>"},
		{"bold", "<b>Bold</b>"},
		{"list", "<ul><li>Item</li></ul>"},
	}

	for _, conv := range conversions {
		result, err := htmltomarkdown.Convert(conv.html)
		if err != nil {
			t.Fatalf("conversion of %s failed: %v", conv.name, err)
		}
		if result == "" && conv.html != "" {
			t.Errorf("expected non-empty result for %s", conv.name)
		}
	}
}

// TestMetadataHeaders tests that headers are properly extracted.
func TestMetadataHeaders(t *testing.T) {
	html := `
	<h1>Main Title</h1>
	<p>Intro paragraph</p>
	<h2>Section One</h2>
	<p>Content</p>
	<h3>Subsection</h3>
	<p>More content</p>
	`

	result, err := htmltomarkdown.ConvertWithMetadata(html)
	if err != nil {
		t.Fatalf("conversion failed: %v", err)
	}

	if len(result.Metadata.Headers) == 0 {
		t.Errorf("expected headers to be extracted")
	}

	// Verify headers are in order and have correct levels
	if len(result.Metadata.Headers) >= 1 && result.Metadata.Headers[0].Level != 1 {
		t.Errorf("expected first header level 1, got %d", result.Metadata.Headers[0].Level)
	}
}

// TestMetadataLinks tests that links are properly extracted.
func TestMetadataLinks(t *testing.T) {
	html := `
	<p><a href="https://example.com">External Link</a></p>
	<p><a href="/internal">Internal Link</a></p>
	<p><a href="mailto:test@example.com">Email</a></p>
	<p><a href="#anchor">Anchor</a></p>
	`

	result, err := htmltomarkdown.ConvertWithMetadata(html)
	if err != nil {
		t.Fatalf("conversion failed: %v", err)
	}

	if len(result.Metadata.Links) == 0 {
		t.Errorf("expected links to be extracted")
	}

	// Verify at least one link was captured
	foundLink := false
	for _, link := range result.Metadata.Links {
		if strings.Contains(link.Text, "Link") {
			foundLink = true
			break
		}
	}
	if !foundLink {
		t.Errorf("expected to find a link with 'Link' in text")
	}
}

// TestMetadataImages tests that images are properly extracted.
func TestMetadataImages(t *testing.T) {
	html := `
	<img src="https://example.com/image.png" alt="Test Image">
	<img src="/local/image.jpg" alt="Local Image">
	<img src="data:image/png;base64,..." alt="Data URI Image">
	`

	result, err := htmltomarkdown.ConvertWithMetadata(html)
	if err != nil {
		t.Fatalf("conversion failed: %v", err)
	}

	if len(result.Metadata.Images) == 0 {
		t.Errorf("expected images to be extracted")
	}

	// Verify at least one image was captured
	for _, img := range result.Metadata.Images {
		if img.Alt != nil && *img.Alt != "" {
			return // Found an image with alt text
		}
	}
}

// TestMetadataDescriptionAndKeywords tests document metadata extraction.
func TestMetadataDescriptionAndKeywords(t *testing.T) {
	html := `
	<head>
		<meta name="description" content="This is a test page">
		<meta name="keywords" content="test,keywords,go">
	</head>
	<body>
		<h1>Test</h1>
	</body>
	`

	result, err := htmltomarkdown.ConvertWithMetadata(html)
	if err != nil {
		t.Fatalf("conversion failed: %v", err)
	}

	// Description might be extracted
	if result.Metadata.Document.Description != nil {
		if !strings.Contains(*result.Metadata.Document.Description, "test") {
			t.Errorf("expected description to contain 'test'")
		}
	}

	// Keywords might be extracted
	if len(result.Metadata.Document.Keywords) > 0 {
		hasTest := false
		for _, kw := range result.Metadata.Document.Keywords {
			if strings.Contains(kw, "test") {
				hasTest = true
				break
			}
		}
		if !hasTest {
			t.Logf("keywords found but none contain 'test': %v", result.Metadata.Document.Keywords)
		}
	}
}

// TestFFIVersionInfo tests that version information is available.
func TestFFIVersionInfo(t *testing.T) {
	version := htmltomarkdown.Version()
	if version == "" {
		t.Errorf("expected non-empty version")
	}

	if version == "unknown" {
		t.Logf("version is 'unknown', FFI may not be fully initialized")
	}

	// Version should follow semantic versioning pattern (roughly)
	parts := strings.Split(version, ".")
	if len(parts) < 2 {
		t.Logf("version doesn't follow expected pattern: %s", version)
	}
}

// TestRegressionHTMLPreservation tests that HTML structure is preserved in conversion.
func TestRegressionHTMLPreservation(t *testing.T) {
	testCases := []struct {
		name            string
		html            string
		expectedIncludes []string
	}{
		{
			name: "multiple paragraphs",
			html: "<p>First</p><p>Second</p><p>Third</p>",
			expectedIncludes: []string{"First", "Second", "Third"},
		},
		{
			name: "mixed list types",
			html: "<ul><li>Bullet 1</li></ul><ol><li>Number 1</li></ol>",
			expectedIncludes: []string{"Bullet 1", "Number 1"},
		},
		{
			name: "nested emphasis",
			html: "<p><strong><em>Bold and italic</em></strong></p>",
			expectedIncludes: []string{"Bold", "italic"},
		},
	}

	for _, tc := range testCases {
		t.Run(tc.name, func(t *testing.T) {
			result, err := htmltomarkdown.Convert(tc.html)
			if err != nil {
				t.Fatalf("conversion failed: %v", err)
			}

			for _, expected := range tc.expectedIncludes {
				if !strings.Contains(result, expected) {
					t.Errorf("expected result to contain '%s', got: %s", expected, result)
				}
			}
		})
	}
}
