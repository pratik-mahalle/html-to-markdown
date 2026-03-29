import { readFileSync } from "node:fs";
import { join } from "node:path";
import { describe, expect, it } from "vitest";
import { convert } from "./dist-node/html_to_markdown_wasm.js";

const loadTestDoc = (path: string): string => {
	const fullPath = join(__dirname, "../../test_documents", path);
	return readFileSync(fullPath, "utf-8");
};

/** Extract the markdown content string from a v3 convert() result object. */
const convertContent = (html: string, options?: unknown): string => {
	const result = convert(html, options as never);
	return (result as unknown as { content: string | null }).content ?? "";
};

describe("html-to-markdown-wasm - WebAssembly Bindings", () => {
	describe("Basic Conversion", () => {
		it("should convert simple HTML to markdown", () => {
			const html = "<h1>Hello World</h1>";
			const result = convert(html);
			expect(result.content).toContain("Hello World");
		});

		it("should handle empty HTML", () => {
			const result = convert("");
			expect(result.content ?? "").toBe("");
		});

		it("should handle plain text", () => {
			const result = convert("Just text");
			expect(result.content).toBe("Just text\n");
		});

		it("should handle null options", () => {
			const html = "<h1>Test</h1>";
			const result = convert(html, undefined);
			expect(result.content).toContain("Test");
		});

		it("should return object with required fields", () => {
			const result = convert("<p>Hello</p>");
			expect(result).toHaveProperty("content");
			expect(result).toHaveProperty("metadata");
			expect(result).toHaveProperty("tables");
			expect(result).toHaveProperty("warnings");
			expect(result).toHaveProperty("document");
			expect(Array.isArray(result.tables)).toBe(true);
			expect(Array.isArray(result.warnings)).toBe(true);
		});
	});

	describe("Heading Styles", () => {
		it("should use ATX style", () => {
			const html = "<h1>Test</h1><h2>Subtest</h2>";
			const markdown = convertContent(html, { headingStyle: "atx" });
			expect(markdown).toMatch(/^#\s+Test/m);
			expect(markdown).toMatch(/^##\s+Subtest/m);
		});

		it("should use underlined style", () => {
			const html = "<h1>Test</h1><h2>Subtest</h2>";
			const markdown = convertContent(html, { headingStyle: "underlined" });
			expect(markdown).toMatch(/^Test\n=+/m);
			expect(markdown).toMatch(/^Subtest\n-+/m);
		});

		it("should use ATX closed style", () => {
			const html = "<h1>Test</h1>";
			const markdown = convertContent(html, { headingStyle: "atxClosed" });
			expect(markdown).toMatch(/^#\s+Test\s+#/m);
		});
	});

	describe("Code Block Styles", () => {
		it("should use backticks for code blocks", () => {
			const html = "<pre><code>const x = 1;</code></pre>";
			const markdown = convertContent(html, { codeBlockStyle: "backticks" });
			expect(markdown).toContain("```");
			expect(markdown).toContain("const x = 1;");
		});

		it("should use tildes for code blocks", () => {
			const html = "<pre><code>const x = 1;</code></pre>";
			const markdown = convertContent(html, { codeBlockStyle: "tildes" });
			expect(markdown).toContain("~~~");
			expect(markdown).toContain("const x = 1;");
		});

		it("should use indentation for code blocks", () => {
			const html = "<pre><code>const x = 1;</code></pre>";
			const markdown = convertContent(html, { codeBlockStyle: "indented" });
			expect(markdown).toContain("    const x = 1;");
		});

		it("should convert code blocks with language class", () => {
			const html = '<pre><code class="language-python">print("hello")</code></pre>';
			const markdown = convertContent(html, { codeBlockStyle: "backticks" });
			expect(markdown).toContain("```");
			expect(markdown).toContain('print("hello")');
		});
	});

	describe("List Options", () => {
		it("should use custom bullets", () => {
			const html = "<ul><li>Item 1</li><li>Item 2</li></ul>";
			const markdown = convertContent(html, { bullets: "+" });
			expect(markdown).toContain("+ Item 1");
			expect(markdown).toContain("+ Item 2");
		});

		it("should use custom list indentation", () => {
			const html = "<ul><li>Item 1<ul><li>Nested</li></ul></li></ul>";
			const markdown = convertContent(html, { listIndentWidth: 4 });
			expect(markdown).toContain("    - Nested");
		});

		it("should use tab indentation", () => {
			const html = "<ul><li>Item 1<ul><li>Nested</li></ul></li></ul>";
			const markdown = convertContent(html, { listIndentType: "tabs" });
			expect(markdown).toMatch(/\t-\sNested/);
		});
	});

	describe("Text Formatting", () => {
		it("should format bold text", () => {
			const html = "<p><strong>bold</strong></p>";
			const markdown = convertContent(html);
			expect(markdown).toContain("**bold**");
		});

		it("should format italic text", () => {
			const html = "<p><em>italic</em></p>";
			const markdown = convertContent(html);
			expect(markdown).toContain("*italic*");
		});

		it("should use underscore for emphasis", () => {
			const html = "<p><strong>bold</strong></p>";
			const markdown = convertContent(html, { strongEmSymbol: "_" });
			expect(markdown).toContain("__bold__");
		});

		it("should escape asterisks", () => {
			const html = "<p>2 * 3 = 6</p>";
			const markdown = convertContent(html, { escapeAsterisks: true });
			expect(markdown).toContain("\\*");
		});

		it("should escape underscores", () => {
			const html = "<p>snake_case</p>";
			const markdown = convertContent(html, { escapeUnderscores: true });
			expect(markdown).toContain("\\_");
		});
	});

	describe("Newline Styles", () => {
		it("should use spaces for line breaks", () => {
			const html = "<p>Line 1<br>Line 2</p>";
			const markdown = convertContent(html, { newlineStyle: "spaces" });
			expect(markdown).toContain("  \n");
		});

		it("should use backslash for line breaks", () => {
			const html = "<p>Line 1<br>Line 2</p>";
			const markdown = convertContent(html, { newlineStyle: "backslash" });
			expect(markdown).toContain("\\\n");
		});
	});

	describe("Table Options", () => {
		it("should convert tables", () => {
			const html = "<table><tr><th>Header</th></tr><tr><td>Cell</td></tr></table>";
			const markdown = convertContent(html);
			expect(markdown).toContain("| Header |");
			expect(markdown).toContain("| Cell |");
		});

		it("should use br in tables", () => {
			// When brInTables is FALSE (default), HTML <br> tags are removed and text flows together
			// When brInTables is TRUE, HTML <br> tags should convert to markdown line breaks
			// This test checks that with brInTables=true, we get proper markdown line breaks
			const html = "<table><tr><td>Line 1<br>Line 2</td></tr></table>";

			// With brInTables: true, <br> should become markdown line breaks (not literal <br>)
			const markdownWithBr = convertContent(html, { brInTables: true });
			// The output should contain markdown line breaks (spaces-style: two spaces before newline)
			expect(markdownWithBr).toContain("Line 1");
			expect(markdownWithBr).toContain("Line 2");
			// Should NOT contain literal <br> tags
			expect(markdownWithBr).not.toContain("<br>");

			// Verify it contains either spaces-style or backslash-style line break
			const hasSpacesStyle = markdownWithBr.includes("Line 1  \n");
			const hasBackslashStyle = markdownWithBr.includes("Line 1\\\n");
			expect(hasSpacesStyle || hasBackslashStyle).toBe(true);
		});
	});

	describe("Highlight Styles", () => {
		it("should use double equals for highlights", () => {
			const html = "<mark>highlighted</mark>";
			const markdown = convertContent(html, { highlightStyle: "doubleEqual" });
			expect(markdown).toContain("==highlighted==");
		});

		it("should use HTML for highlights", () => {
			const html = "<mark>highlighted</mark>";
			const markdown = convertContent(html, { highlightStyle: "html" });
			expect(markdown).toContain("<mark>highlighted</mark>");
		});

		it("should use bold for highlights", () => {
			const html = "<mark>highlighted</mark>";
			const markdown = convertContent(html, { highlightStyle: "bold" });
			expect(markdown).toContain("**highlighted**");
		});

		it("should use no formatting for highlights", () => {
			const html = "<mark>highlighted</mark>";
			const markdown = convertContent(html, { highlightStyle: "none" });
			expect(markdown.trim()).toBe("highlighted");
		});
	});

	describe("Whitespace Handling", () => {
		it("should normalize whitespace", () => {
			const html = "<p>Multiple    spaces</p>";
			const markdown = convertContent(html, { whitespaceMode: "normalized" });
			expect(markdown).not.toContain("    ");
		});

		it("should preserve strict whitespace", () => {
			const html = "<p>Multiple    spaces</p>";
			const markdown = convertContent(html, { whitespaceMode: "strict" });
			expect(markdown).toContain("    ");
		});

		it("should strip newlines when enabled", () => {
			const html = "<p>Line\n\nwith\n\nnewlines</p>";
			const markdown = convertContent(html, { stripNewlines: true });
			expect(markdown).not.toContain("\n\n");
		});
	});

	describe("Text Wrapping", () => {
		it("should accept wrap configuration", () => {
			const html = "<p>A very long line of text that should be wrapped at a certain width</p>";
			const markdown = convertContent(html, { wrap: true, wrapWidth: 40 });
			expect(markdown).toBeTruthy();
		});
	});

	describe("Special Elements", () => {
		it("should convert links", () => {
			const html = '<a href="https://example.com">Link</a>';
			const markdown = convertContent(html);
			expect(markdown).toContain("[Link](https://example.com)");
		});

		it("should convert images", () => {
			const html = '<img src="image.png" alt="Test">';
			const markdown = convertContent(html);
			expect(markdown).toContain("![Test](image.png)");
		});

		it("should use autolinks", () => {
			const html = '<a href="https://example.com">https://example.com</a>';
			const markdown = convertContent(html, { autolinks: true });
			expect(markdown).toContain("<https://example.com>");
		});

		it("should convert as inline", () => {
			const html = "<div>Block</div>";
			const markdown = convertContent(html, { convertAsInline: true });
			expect(markdown.trim()).toBe("Block");
		});
	});

	describe("Preprocessing", () => {
		it("should accept preprocessing options", () => {
			const html = "<nav>Navigation</nav><article>Content</article>";
			const markdown = convertContent(html, {
				preprocessing: {
					enabled: true,
					preset: "aggressive",
					removeNavigation: true,
					removeForms: true,
				},
			});
			expect(markdown).toBeTruthy();
		});

		it("should use minimal preset", () => {
			const html = "<div>Content</div>";
			const markdown = convertContent(html, {
				preprocessing: {
					enabled: true,
					preset: "minimal",
				},
			});
			expect(markdown).toContain("Content");
		});

		it("should use standard preset", () => {
			const html = "<div>Content</div>";
			const markdown = convertContent(html, {
				preprocessing: {
					enabled: true,
					preset: "standard",
				},
			});
			expect(markdown).toContain("Content");
		});

		it("should allow preprocessing without preset", () => {
			const html = "<div>Content</div>";
			const markdown = convertContent(html, {
				preprocessing: {
					enabled: true,
				},
			});
			expect(markdown).toContain("Content");
		});

		it("should work with preprocessing disabled", () => {
			const html = "<nav>Navigation</nav>";
			const markdown = convertContent(html, {
				preprocessing: {
					enabled: false,
				},
			});
			expect(markdown).toContain("Navigation");
		});
	});

	describe("Advanced Options", () => {
		it("should add default title", () => {
			const html = "<p>Content without title</p>";
			const markdown = convertContent(html, { defaultTitle: true });
			expect(markdown).toBeTruthy();
		});

		it("should extract metadata", () => {
			const html = "<html><head><title>Test</title></head><body><p>Content</p></body></html>";
			const markdown = convertContent(html, { extractMetadata: true });
			expect(markdown).toContain("Content");
		});

		it("should disable metadata extraction", () => {
			const html = "<html><head><title>Test</title></head><body><p>Content</p></body></html>";
			const markdown = convertContent(html, { extractMetadata: false });
			expect(markdown).toContain("Content");
		});

		it("should accept subscript symbol", () => {
			const html = "<p>H<sub>2</sub>O</p>";
			const markdown = convertContent(html, { subSymbol: "~" });
			expect(markdown).toContain("~2~");
		});

		it("should accept superscript symbol", () => {
			const html = "<p>x<sup>2</sup></p>";
			const markdown = convertContent(html, { supSymbol: "^" });
			expect(markdown).toContain("^2^");
		});

		it("should accept strip tags list", () => {
			const html = "<p>Keep this</p><script>Remove this</script>";
			const markdown = convertContent(html, { stripTags: ["script"] });
			expect(markdown).toContain("Keep this");
		});
	});

	describe("Edge Cases", () => {
		it("should handle malformed HTML", () => {
			const html = "<div><p>Unclosed paragraph";
			const markdown = convertContent(html);
			expect(markdown).toContain("Unclosed paragraph");
		});

		it("should handle nested formatting", () => {
			const html = "<strong><em>Bold and italic</em></strong>";
			const markdown = convertContent(html);
			expect(markdown).toContain("***Bold and italic***");
		});

		it("should handle special characters", () => {
			const html = "<p>&lt;special&gt;</p>";
			const markdown = convertContent(html);
			expect(markdown).toContain("<special>");
		});

		it("should handle empty elements", () => {
			const html = "<p></p><div></div>";
			const markdown = convertContent(html);
			expect(markdown).toBeDefined();
		});

		it("should handle very long text", () => {
			const html = `<p>${"A ".repeat(10000)}</p>`;
			const markdown = convertContent(html);
			expect(markdown).toBeTruthy();
			expect(markdown.length).toBeGreaterThan(10000);
		});

		it("should handle deeply nested elements", () => {
			let html = "<div>";
			for (let i = 0; i < 50; i++) {
				html += "<div>";
			}
			html += "Deep content";
			for (let i = 0; i < 50; i++) {
				html += "</div>";
			}
			html += "</div>";
			const markdown = convertContent(html);
			expect(markdown).toContain("Deep content");
		});
	});

	describe("Performance", () => {
		it("should handle small documents efficiently", () => {
			const html = "<p>Small document</p>";
			const start = Date.now();
			for (let i = 0; i < 1000; i++) {
				convertContent(html);
			}
			const duration = Date.now() - start;
			expect(duration).toBeLessThan(1000);
		});

		it("should handle medium documents", () => {
			const html = `<div>${"<p>Paragraph</p>".repeat(100)}</div>`;
			const markdown = convertContent(html);
			expect(markdown).toContain("Paragraph");
		});
	});

	describe("Preserve Tags", () => {
		it("should preserve simple table as HTML", () => {
			const html = "<p>Before table</p><table><tr><td>Cell 1</td><td>Cell 2</td></tr></table><p>After table</p>";
			const markdown = convertContent(html, { preserveTags: ["table"] });
			expect(markdown).toContain("<table>");
			expect(markdown).toContain("<tr>");
			expect(markdown).toContain("<td>Cell 1</td>");
			expect(markdown).toContain("Before table");
			expect(markdown).toContain("After table");
		});

		it("should preserve table with attributes", () => {
			const html = '<div class="content"><table id="data" class="styled"><tr><td>Value</td></tr></table></div>';
			const markdown = convertContent(html, { preserveTags: ["table"] });
			expect(markdown).toContain('<table id="data" class="styled">');
			expect(markdown).toContain("<tr>");
			expect(markdown).toContain("<td>Value</td>");
			expect(markdown).toContain("</table>");
		});

		it("should preserve multiple tag types", () => {
			const html =
				'<p>Text</p><table><tr><td>Table</td></tr></table><form><input type="text"/></form><div>More text</div>';
			const markdown = convertContent(html, { preserveTags: ["table", "form"] });
			expect(markdown).toContain("<table>");
			expect(markdown).toContain("<form>");
			expect(markdown).toContain("Text");
			expect(markdown).toContain("More text");
		});

		it("should preserve nested content in tags", () => {
			const html = "<table><tr><td><strong>Bold</strong> and <em>italic</em></td></tr></table>";
			const markdown = convertContent(html, { preserveTags: ["table"] });
			expect(markdown).toContain("<table>");
			expect(markdown).toContain("<strong>Bold</strong>");
			expect(markdown).toContain("<em>italic</em>");
			expect(markdown).toContain("</table>");
		});

		it("should convert tables normally without preserve_tags", () => {
			const html = "<table><tr><td>Cell</td></tr></table>";
			const markdown = convertContent(html, { preserveTags: [] });
			expect(markdown).not.toContain("<table>");
			expect(markdown).toContain("Cell");
		});

		it("should work with both preserve and strip tags", () => {
			const html = "<p>Text</p><table><tr><td>Table content</td></tr></table><div>Div content</div>";
			const markdown = convertContent(html, { preserveTags: ["table"], stripTags: ["div"] });
			expect(markdown).toContain("<table>");
			expect(markdown).toContain("Table content");
			expect(markdown).not.toContain("<div>");
			expect(markdown).toContain("Div content");
			expect(markdown).toContain("Text");
		});
	});

	describe("Real-World Documents", () => {
		it("should convert Wikipedia timeline", () => {
			const html = loadTestDoc("html/wikipedia/lists_timeline.html");
			const markdown = convertContent(html);
			expect(markdown).toBeTruthy();
			expect(markdown.length).toBeGreaterThan(100);
		});

		it("should convert Wikipedia countries table", () => {
			const html = loadTestDoc("html/wikipedia/tables_countries.html");
			const markdown = convertContent(html);
			expect(markdown).toBeTruthy();
			expect(markdown).toContain("|");
		});

		it("should convert Wikipedia Python article", () => {
			const html = loadTestDoc("html/wikipedia/medium_python.html");
			const markdown = convertContent(html);
			expect(markdown).toBeTruthy();
			expect(markdown.length).toBeGreaterThan(1000);
		});

		it("should convert small HTML document", () => {
			const html = loadTestDoc("html/wikipedia/small_html.html");
			const markdown = convertContent(html);
			expect(markdown).toBeTruthy();
		});
	});
});
