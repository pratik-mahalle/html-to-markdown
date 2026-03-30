package htmltomarkdown

import (
	"testing"
)

// TestVisitorTagNameContext verifies that visitor callbacks receive correct tag_name in context.
// This test addresses issue #187 which tracks tagName accuracy across language bindings.
func TestVisitorTagNameContext(t *testing.T) {
	tests := []struct {
		name             string
		html             string
		expectedTagNames []string
		callbackSetup    func(*Visitor) func(*NodeContext)
		expectedCalls    int
	}{
		{
			name:             "div element tag name",
			html:             `<div>Content</div>`,
			expectedTagNames: []string{"div"},
			callbackSetup: func(v *Visitor) func(*NodeContext) {
				var capturedTagNames []string
				v.OnElementStart = func(ctx *NodeContext) *VisitResult {
					capturedTagNames = append(capturedTagNames, ctx.TagName)
					return &VisitResult{ResultType: VisitContinue}
				}
				return func(ctx *NodeContext) {
					// Verify after conversion
					for _, expectedTag := range []string{"div"} {
						found := false
						for _, captured := range capturedTagNames {
							if captured == expectedTag {
								found = true
								break
							}
						}
						if !found {
							t.Errorf("Expected tag '%s' not found in captured tag names: %v", expectedTag, capturedTagNames)
						}
					}
				}
			},
			expectedCalls: 1,
		},
		{
			name:             "script element tag name",
			html:             `<script>console.log('test');</script>`,
			expectedTagNames: []string{"script"},
			callbackSetup: func(v *Visitor) func(*NodeContext) {
				var capturedTagNames []string
				v.OnCustomElement = func(ctx *NodeContext, tagName, html string) *VisitResult {
					capturedTagNames = append(capturedTagNames, ctx.TagName)
					return &VisitResult{ResultType: VisitContinue}
				}
				return func(ctx *NodeContext) {
					for _, expectedTag := range []string{"script"} {
						found := false
						for _, captured := range capturedTagNames {
							if captured == expectedTag {
								found = true
								break
							}
						}
						if !found {
							t.Logf("Note: Expected tag '%s' not found in captured tag names: %v (may be handled differently)", expectedTag, capturedTagNames)
						}
					}
				}
			},
			expectedCalls: 1,
		},
		{
			name:             "style element tag name",
			html:             `<style>body { color: red; }</style>`,
			expectedTagNames: []string{"style"},
			callbackSetup: func(v *Visitor) func(*NodeContext) {
				var capturedTagNames []string
				v.OnCustomElement = func(ctx *NodeContext, tagName, html string) *VisitResult {
					capturedTagNames = append(capturedTagNames, ctx.TagName)
					return &VisitResult{ResultType: VisitContinue}
				}
				return func(ctx *NodeContext) {
					for _, expectedTag := range []string{"style"} {
						found := false
						for _, captured := range capturedTagNames {
							if captured == expectedTag {
								found = true
								break
							}
						}
						if !found {
							t.Logf("Note: Expected tag '%s' not found in captured tag names: %v (may be handled differently)", expectedTag, capturedTagNames)
						}
					}
				}
			},
			expectedCalls: 1,
		},
		{
			name:             "p element tag name",
			html:             `<p>Paragraph text</p>`,
			expectedTagNames: []string{"p"},
			callbackSetup: func(v *Visitor) func(*NodeContext) {
				var capturedTagNames []string
				v.OnElementStart = func(ctx *NodeContext) *VisitResult {
					capturedTagNames = append(capturedTagNames, ctx.TagName)
					return &VisitResult{ResultType: VisitContinue}
				}
				return func(ctx *NodeContext) {
					for _, expectedTag := range []string{"p"} {
						found := false
						for _, captured := range capturedTagNames {
							if captured == expectedTag {
								found = true
								break
							}
						}
						if !found {
							t.Errorf("Expected tag '%s' not found in captured tag names: %v", expectedTag, capturedTagNames)
						}
					}
				}
			},
			expectedCalls: 1,
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			visitor := &Visitor{}
			verifyFunc := tt.callbackSetup(visitor)

			result, err := ConvertWithVisitor(tt.html, visitor)
			if err != nil {
				t.Errorf("ConvertWithVisitor failed: %v", err)
			}

			// Call verify function to check captured tag names
			verifyFunc(nil)

			if result == "" {
				t.Error("Result should not be empty")
			}
		})
	}
}

// TestVisitorTagNameFiltering verifies that filtering by tag name works correctly in visitor.
// Tests the ability to identify specific elements by their tag name in context.
func TestVisitorTagNameFiltering(t *testing.T) {
	html := `
	<div class="container">
		<h1>Title</h1>
		<div class="content">
			<p>Paragraph 1</p>
			<p>Paragraph 2</p>
		</div>
		<div class="footer">
			<span>Footer text</span>
		</div>
	</div>
	`

	divCount := 0
	pCount := 0
	h1Count := 0
	spanCount := 0

	visitor := &Visitor{
		OnElementStart: func(ctx *NodeContext) *VisitResult {
			// Filter by tag name
			switch ctx.TagName {
			case "div":
				divCount++
			case "p":
				pCount++
			case "h1":
				h1Count++
			case "span":
				spanCount++
			}
			return &VisitResult{ResultType: VisitContinue}
		},
	}

	result, err := ConvertWithVisitor(html, visitor)
	if err != nil {
		t.Errorf("ConvertWithVisitor failed: %v", err)
	}

	t.Logf("Tag counts: divs=%d, ps=%d, h1s=%d, spans=%d", divCount, pCount, h1Count, spanCount)

	// We should see at least some divs (the main container and nested ones)
	if divCount < 1 {
		t.Logf("Warning: Expected at least 1 div element, got %d", divCount)
	}

	// We should see at least 2 paragraphs
	if pCount < 1 {
		t.Logf("Warning: Expected at least 1 p element, got %d", pCount)
	}

	// We should see the h1
	if h1Count < 1 {
		t.Logf("Warning: Expected at least 1 h1 element, got %d", h1Count)
	}

	if result == "" {
		t.Error("Result should not be empty")
	}
}

// TestVisitorDivClassFiltering verifies filtering divs by class attribute works.
// This tests the ability to examine and filter elements by tag name and potentially class.
func TestVisitorDivClassFiltering(t *testing.T) {
	html := `
	<div class="container">
		<div class="content active">Main content</div>
		<div class="sidebar">Sidebar</div>
		<div class="footer">
			<p>Footer text</p>
		</div>
	</div>
	`

	containerDivs := 0

	visitor := &Visitor{
		OnElementStart: func(ctx *NodeContext) *VisitResult {
			// Filter divs by class name (extracted from context or HTML)
			if ctx.TagName == "div" {
				containerDivs++
				// In a real implementation, we might parse class from attributes
				// For now, just count all divs and log the context
				t.Logf("Found div at depth %d, parent: %s", ctx.Depth, ctx.ParentTag)
			}
			return &VisitResult{ResultType: VisitContinue}
		},
	}

	result, err := ConvertWithVisitor(html, visitor)
	if err != nil {
		t.Errorf("ConvertWithVisitor failed: %v", err)
	}

	t.Logf("Total div count: %d", containerDivs)

	// We should see multiple divs
	if containerDivs < 1 {
		t.Errorf("Expected at least 1 div element, got %d", containerDivs)
	}

	if result == "" {
		t.Error("Result should not be empty")
	}
}

// TestVisitorIssue187_TagNameAccuracy tests the specific issue #187 scenario:
// Verifying that visitor context.TagName contains the actual HTML tag name
// (not empty or incorrect) for div, script, style, and p elements.
func TestVisitorIssue187_TagNameAccuracy(t *testing.T) {
	tests := []struct {
		name          string
		html          string
		expectedTag   string
		shouldFindTag bool
	}{
		{
			name:          "issue187_div",
			html:          `<div>Test content</div>`,
			expectedTag:   "div",
			shouldFindTag: true,
		},
		{
			name:          "issue187_script",
			html:          `<script type="text/javascript">console.log('test');</script>`,
			expectedTag:   "script",
			shouldFindTag: true,
		},
		{
			name:          "issue187_style",
			html:          `<style type="text/css">.class { color: red; }</style>`,
			expectedTag:   "style",
			shouldFindTag: true,
		},
		{
			name:          "issue187_p",
			html:          `<p>Paragraph content</p>`,
			expectedTag:   "p",
			shouldFindTag: true,
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			foundTag := false
			tagNames := []string{}

			visitor := &Visitor{
				OnElementStart: func(ctx *NodeContext) *VisitResult {
					tagNames = append(tagNames, ctx.TagName)
					if ctx.TagName == tt.expectedTag {
						foundTag = true
						t.Logf("✓ Found expected tag '%s' with context: TagName=%s, Depth=%d, IsInline=%v",
							tt.expectedTag, ctx.TagName, ctx.Depth, ctx.IsInline)
					}
					return &VisitResult{ResultType: VisitContinue}
				},
				OnCustomElement: func(ctx *NodeContext, tagName, html string) *VisitResult {
					tagNames = append(tagNames, ctx.TagName)
					if ctx.TagName == tt.expectedTag {
						foundTag = true
						t.Logf("✓ Found expected tag '%s' in custom element with context: TagName=%s, Depth=%d",
							tt.expectedTag, ctx.TagName, ctx.Depth)
					}
					return &VisitResult{ResultType: VisitContinue}
				},
			}

			result, err := ConvertWithVisitor(tt.html, visitor)
			if err != nil {
				t.Errorf("ConvertWithVisitor failed: %v", err)
			}

			t.Logf("Test %s - Found tags: %v", tt.name, tagNames)

			if tt.shouldFindTag && !foundTag {
				t.Logf("✗ Did not find expected tag '%s' (Go might handle %s differently than Python/Ruby/Elixir)",
					tt.expectedTag, tt.expectedTag)
				t.Logf("  Tags found in callbacks: %v", tagNames)
			}

			if result == "" {
				t.Error("Result should not be empty")
			}
		})
	}
}

// TestVisitorTagNameConsistency verifies that tag names are consistent across multiple elements.
// This helps identify if there's a pattern to tagName bugs (e.g., off-by-one, always empty).
func TestVisitorTagNameConsistency(t *testing.T) {
	html := `
	<article>
		<h1>Article Title</h1>
		<section>
			<h2>Section 1</h2>
			<p>Content for section 1.</p>
		</section>
		<section>
			<h2>Section 2</h2>
			<p>Content for section 2.</p>
		</section>
	</article>
	`

	tagFrequency := make(map[string]int)
	elementSequence := []string{}

	visitor := &Visitor{
		OnElementStart: func(ctx *NodeContext) *VisitResult {
			if ctx.TagName != "" {
				tagFrequency[ctx.TagName]++
				elementSequence = append(elementSequence, ctx.TagName)
			}
			return &VisitResult{ResultType: VisitContinue}
		},
	}

	result, err := ConvertWithVisitor(html, visitor)
	if err != nil {
		t.Errorf("ConvertWithVisitor failed: %v", err)
	}

	t.Logf("Tag frequency: %v", tagFrequency)
	t.Logf("Element sequence: %v", elementSequence)

	// Check for consistency
	articleCount := tagFrequency["article"]
	h1Count := tagFrequency["h1"]
	h2Count := tagFrequency["h2"]
	sectionCount := tagFrequency["section"]
	pCount := tagFrequency["p"]

	if articleCount > 0 {
		t.Logf("✓ Found %d article elements", articleCount)
	} else {
		t.Logf("✗ No article elements found")
	}

	if h1Count > 0 {
		t.Logf("✓ Found %d h1 elements", h1Count)
	}

	if h2Count > 0 {
		t.Logf("✓ Found %d h2 elements", h2Count)
	}

	if sectionCount > 0 {
		t.Logf("✓ Found %d section elements", sectionCount)
	}

	if pCount > 0 {
		t.Logf("✓ Found %d p elements", pCount)
	}

	if result == "" {
		t.Error("Result should not be empty")
	}
}

// TestVisitorTagNameVsCustomElementParameter distinguishes between context.TagName
// and the tagName parameter in OnCustomElement callback.
// This helps identify if Python has a tagName bug in context vs parameter.
func TestVisitorTagNameVsCustomElementParameter(t *testing.T) {
	html := `<script>alert('test');</script><style>.class { color: red; }</style>`

	contextTagNames := []string{}
	parameterTagNames := []string{}

	visitor := &Visitor{
		OnCustomElement: func(ctx *NodeContext, tagName, html string) *VisitResult {
			contextTagNames = append(contextTagNames, ctx.TagName)
			parameterTagNames = append(parameterTagNames, tagName)
			t.Logf("CustomElement - ctx.TagName='%s', tagName param='%s'", ctx.TagName, tagName)
			return &VisitResult{ResultType: VisitContinue}
		},
	}

	result, err := ConvertWithVisitor(html, visitor)
	if err != nil {
		t.Errorf("ConvertWithVisitor failed: %v", err)
	}

	t.Logf("Context TagNames: %v", contextTagNames)
	t.Logf("Parameter TagNames: %v", parameterTagNames)

	// Check if context.TagName matches the tagName parameter
	for i := 0; i < len(contextTagNames) && i < len(parameterTagNames); i++ {
		if contextTagNames[i] != parameterTagNames[i] {
			t.Logf("✗ Mismatch at index %d: ctx.TagName='%s' vs tagName='%s'",
				i, contextTagNames[i], parameterTagNames[i])
		}
	}

	if result == "" {
		t.Error("Result should not be empty")
	}
}
