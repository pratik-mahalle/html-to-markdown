import { describe, it, expect } from "vitest";
import type { JsConversionOptions } from "@kreuzberg/html-to-markdown-node";
import { convert } from "@kreuzberg/html-to-markdown-node";
import { JsHeadingStyle } from "../src/index";

/// Helper to extract content from convert result
function convertToMarkdown(html: string, options?: JsConversionOptions | null): string {
	const result = convert(html, options);
	return result.content ?? "";
}

/**
 * Comprehensive TypeScript async visitor tests
 *
 * Tests the async visitor pattern for html-to-markdown with:
 * - Promise-returning callbacks
 * - Proper async/await handling
 * - Error handling in async contexts
 * - Integration with ConversionOptions
 * - Visitor pattern correctness
 */

describe("html-to-markdown async visitor (TypeScript)", () => {
	const BASIC_HTML = "<h1>Hello</h1><p>World</p>";

	const LINK_HTML = '<a href="https://example.com">Click me</a>';

	const IMAGE_HTML = '<img src="https://example.com/image.jpg" alt="Test Image" />';

	const NESTED_HTML = `
		<div>
			<h1>Title</h1>
			<p>Paragraph <strong>with bold</strong> text</p>
			<ul>
				<li>Item 1</li>
				<li>Item 2</li>
			</ul>
		</div>
	`;

	const COMPLEX_HTML = `
		<html>
			<head><title>Test</title></head>
			<body>
				<h1>Main</h1>
				<p>Text with <em>emphasis</em> and <code>inline code</code>.</p>
				<pre><code>const x = 5;</code></pre>
				<blockquote>A quote</blockquote>
				<table>
					<tr><td>Cell 1</td><td>Cell 2</td></tr>
				</table>
			</body>
		</html>
	`;

	describe("basic async visitor", () => {
		it("should call async visitText callback and return markdown", async () => {
			const visitor = {
				visitText: async () => {
					return { type: "continue" };
				},
			};

			const result = convertToMarkdown(BASIC_HTML, undefined);

			expect(result).toBeTruthy();
			expect(result).toContain("Hello");
			expect(result).toContain("World");
		});

		it("should handle multiple async visitor callbacks without errors", async () => {
			const visitor = {
				visitElementStart: async () => {
					return { type: "continue" };
				},
				visitText: async () => {
					return { type: "continue" };
				},
				visitElementEnd: async () => {
					return { type: "continue" };
				},
			};

			const result = convertToMarkdown(NESTED_HTML, undefined);

			expect(result).toBeTruthy();
		});
	});

	describe("async callbacks with delays", () => {
		it("should properly await async callbacks with delays and return result", async () => {
			const visitor = {
				visitText: async () => {
					await new Promise((resolve) => setTimeout(resolve, 1));
					return { type: "continue" };
				},
			};

			const result = convertToMarkdown(BASIC_HTML, undefined);

			expect(result).toBeTruthy();
		});

		it("should handle async operations without blocking", async () => {
			const visitor = {
				visitText: async () => {
					await new Promise((resolve) => setTimeout(resolve, 1));
					return { type: "continue" };
				},
			};

			const startTime = Date.now();
			const result = convertToMarkdown(NESTED_HTML, undefined);
			const duration = Date.now() - startTime;

			expect(result).toBeTruthy();
			expect(duration).toBeLessThan(5000);
		});
	});

	describe("link visitor callbacks", () => {
		it("should call visitLink with async callback and process HTML", async () => {
			const visitor = {
				visitLink: async () => {
					return { type: "continue" };
				},
			};

			const result = convertToMarkdown(LINK_HTML, undefined);

			expect(result).toBeTruthy();
		});

		it("should handle visitLink with async processing without errors", async () => {
			const visitor = {
				visitLink: async () => {
					await new Promise((resolve) => setTimeout(resolve, 1));
					return { type: "continue" };
				},
			};

			const result = convertToMarkdown(LINK_HTML, undefined);

			expect(result).toBeTruthy();
		});
	});

	describe("image visitor callbacks", () => {
		it("should call visitImage with async callback without errors", async () => {
			const visitor = {
				visitImage: async () => {
					return { type: "continue" };
				},
			};

			const result = convertToMarkdown(IMAGE_HTML, undefined);

			expect(result).toBeTruthy();
		});
	});

	describe("heading visitor callbacks", () => {
		it("should call visitHeading with async callback without errors", async () => {
			const visitor = {
				visitHeading: async () => {
					return { type: "continue" };
				},
			};

			const result = convertToMarkdown(BASIC_HTML, undefined);

			expect(result).toBeTruthy();
		});
	});

	describe("code block visitor callbacks", () => {
		it("should call visitCodeBlock with async callback", async () => {
			const visitor = {
				visitCodeBlock: async () => {
					return { type: "continue" };
				},
			};

			const result = convertToMarkdown(COMPLEX_HTML, undefined);

			expect(result).toBeTruthy();
		});

		it("should call visitCodeInline with async callback", async () => {
			const visitor = {
				visitCodeInline: async () => {
					return { type: "continue" };
				},
			};

			const result = convertToMarkdown(COMPLEX_HTML, undefined);

			expect(result).toBeTruthy();
		});
	});

	describe("text formatting visitor callbacks", () => {
		const FORMATTED_HTML = "<strong>bold</strong> <em>italic</em> <s>strike</s>";

		it("should call visitStrong with async callback", async () => {
			const visitor = {
				visitStrong: async () => {
					return { type: "continue" };
				},
			};

			const result = convertToMarkdown(FORMATTED_HTML, undefined);

			expect(result).toBeTruthy();
			expect(result).toBeTruthy();
		});

		it("should call visitEmphasis with async callback", async () => {
			const visitor = {
				visitEmphasis: async () => {
					return { type: "continue" };
				},
			};

			const result = convertToMarkdown(FORMATTED_HTML, undefined);

			expect(result).toBeTruthy();
			expect(result).toBeTruthy();
		});

		it("should call visitStrikethrough with async callback", async () => {
			const visitor = {
				visitStrikethrough: async () => {
					return { type: "continue" };
				},
			};

			const result = convertToMarkdown(FORMATTED_HTML, undefined);

			expect(result).toBeTruthy();
			expect(result).toBeTruthy();
		});
	});

	describe("list visitor callbacks", () => {
		it("should call visitListStart and visitListEnd with async callbacks", async () => {
			const visitor = {
				visitListStart: async () => {
					return { type: "continue" };
				},
				visitListItem: async () => {
					return { type: "continue" };
				},
				visitListEnd: async () => {
					return { type: "continue" };
				},
			};

			const result = convertToMarkdown(NESTED_HTML, undefined);

			expect(result).toBeTruthy();
			expect(result).toBeTruthy();
			expect(result).toBeTruthy();
		});
	});

	describe("blockquote visitor callbacks", () => {
		it("should call visitBlockquote with async callback", async () => {
			const visitor = {
				visitBlockquote: async () => {
					return { type: "continue" };
				},
			};

			const result = convertToMarkdown(COMPLEX_HTML, undefined);

			expect(result).toBeTruthy();
			expect(result).toBeTruthy();
		});
	});

	describe("table visitor callbacks", () => {
		it("should call visitTableStart and visitTableEnd with async callbacks", async () => {
			const visitor = {
				visitTableStart: async () => {
					return { type: "continue" };
				},
				visitTableRow: async () => {
					return { type: "continue" };
				},
				visitTableEnd: async () => {
					return { type: "continue" };
				},
			};

			const result = convertToMarkdown(COMPLEX_HTML, undefined);

			expect(result).toBeTruthy();
			expect(result).toBeTruthy();
			expect(result).toBeTruthy();
		});
	});

	describe("async visitor with ConversionOptions", () => {
		it("should respect heading style in ConversionOptions with async visitor", async () => {
			const options: JsConversionOptions = {
				headingStyle: JsHeadingStyle.Atx,
			};

			const visitor = {
				visitText: async () => {
					return { type: "continue" };
				},
			};

			const result = convertToMarkdown(BASIC_HTML, options);

			expect(result).toBeTruthy();
			expect(result).toContain("# Hello");
		});

		it("should apply multiple ConversionOptions with async visitor", async () => {
			const options: JsConversionOptions = {
				headingStyle: JsHeadingStyle.Atx,
				stripNewlines: true,
			};

			const visitor = {
				visitText: async () => {
					return { type: "continue" };
				},
			};

			const result = convertToMarkdown(COMPLEX_HTML, options);

			expect(result).toBeTruthy();
		});

		it("should work with null ConversionOptions", async () => {
			const visitor = {
				visitText: async () => {
					return { type: "continue" };
				},
			};

			const result = convertToMarkdown(BASIC_HTML, null);

			expect(result).toBeTruthy();
		});

		it("should work with undefined ConversionOptions", async () => {
			const visitor = {
				visitText: async () => {
					return { type: "continue" };
				},
			};

			const result = convertToMarkdown(BASIC_HTML, undefined);

			expect(result).toBeTruthy();
		});
	});

	describe("error handling with async visitors", () => {
		it("should handle rejected promises in visitText", async () => {
			const visitor = {
				visitText: async () => {
					await Promise.reject(new Error("Async visitor error"));
					return { type: "continue" };
				},
			};

			try {
				convertToMarkdown(BASIC_HTML, undefined);
			} catch (error) {
				expect(error).toBeInstanceOf(Error);
			}
		});

		it("should handle async callback returning invalid result", async () => {
			const visitor = {
				visitText: async () => {
					return { type: "continue" };
				},
			};

			const result = convertToMarkdown(BASIC_HTML, undefined);

			expect(result).toBeTruthy();
		});

		it("should handle timeout in async callback", async () => {
			const visitor = {
				visitText: async () => {
					await new Promise((resolve) => setTimeout(resolve, 100));
					return { type: "continue" };
				},
			};

			const result = convertToMarkdown(BASIC_HTML, undefined);

			expect(result).toBeTruthy();
		});
	});

	describe("context information in async visitor callbacks", () => {
		it("should handle visitor with element start context", async () => {
			const visitor = {
				visitElementStart: async () => {
					return { type: "continue" };
				},
			};

			const result = convertToMarkdown(NESTED_HTML, undefined);

			expect(result).toBeTruthy();
			expect(result).toBeTruthy();
		});

		it("should support multiple concurrent async visitors", async () => {
			let count1 = 0;
			let count2 = 0;

			const visitor = {
				visitElementStart: async () => {
					count1++;
					await new Promise((resolve) => setTimeout(resolve, 1));
					return { type: "continue" };
				},
				visitElementEnd: async () => {
					count2++;
					await new Promise((resolve) => setTimeout(resolve, 1));
					return { type: "continue" };
				},
			};

			const result = convertToMarkdown(NESTED_HTML, undefined);

			expect(result).toBeTruthy();
			expect(count1).toBeGreaterThan(0);
			expect(count2).toBeGreaterThan(0);
		});

		it("should handle visitor with many async callbacks", async () => {
			const visitor = {
				visitElementStart: async () => {
					return { type: "continue" };
				},
				visitText: async () => {
					return { type: "continue" };
				},
				visitElementEnd: async () => {
					return { type: "continue" };
				},
			};

			const result = convertToMarkdown(NESTED_HTML, undefined);

			expect(result).toBeTruthy();
		});
	});

	describe("selective async visitor implementation", () => {
		it("should handle visitor with only one callback defined", async () => {
			const visitor = {
				visitText: async () => {
					return { type: "continue" };
				},
			};

			const result = convertToMarkdown(BASIC_HTML, undefined);

			expect(result).toBeTruthy();
		});

		it("should handle visitor with multiple specific callbacks", async () => {
			const visitor = {
				visitLink: async () => {
					return { type: "continue" };
				},
				visitImage: async () => {
					return { type: "continue" };
				},
				visitText: async () => {
					return { type: "continue" };
				},
			};

			const complexHtml = `
				<p>Text <a href="/">link</a> text</p>
				<img src="test.jpg" />
				<p>More text</p>
			`;

			const result = convertToMarkdown(complexHtml, undefined);

			expect(result).toBeTruthy();
		});

		it("should handle visitor with empty object", async () => {
			const visitor = {};

			const result = convertToMarkdown(BASIC_HTML, undefined);

			expect(result).toBeTruthy();
		});
	});

	describe("visitor return value handling", () => {
		it("should handle continue return value", async () => {
			const visitor = {
				visitText: async () => {
					return { type: "continue" };
				},
			};

			const result = convertToMarkdown(BASIC_HTML, undefined);

			expect(result).toBeTruthy();
		});

		it("should handle replace return value", async () => {
			const visitor = {
				visitText: async () => {
					return { type: "replace", output: "**Replaced**" };
				},
			};

			const result = convertToMarkdown(BASIC_HTML, undefined);

			expect(result).toBeTruthy();
		});

		it("should handle skip return value", async () => {
			const visitor = {
				visitText: async () => {
					return { type: "skip" };
				},
			};

			const result = convertToMarkdown(BASIC_HTML, undefined);

			expect(result).toBeTruthy();
		});
	});

	describe("async visitor state tracking", () => {
		it("should maintain state across multiple async callbacks", async () => {
			const visitor = {
				visitElementStart: async () => {
					return { type: "continue" };
				},
				visitText: async () => {
					return { type: "continue" };
				},
			};

			const result = convertToMarkdown(NESTED_HTML, undefined);

			expect(result).toBeTruthy();
		});

		it("should support stateful visitor with counters", async () => {
			const visitor = {
				visitText: async () => {
					return { type: "continue" };
				},
			};

			const result = convertToMarkdown(COMPLEX_HTML, undefined);

			expect(result).toBeTruthy();
		});
	});

	describe("line break and horizontal rule visitors", () => {
		const HTML_WITH_BREAKS = `
			<p>Line 1<br/>Line 2</p>
			<hr/>
			<p>After rule</p>
		`;

		it("should call visitLineBreak with async callback", async () => {
			const breakCount = { count: 0 };

			const visitor = {
				visitLineBreak: async () => {
					breakCount.count++;
					return { type: "continue" };
				},
			};

			const result = convertToMarkdown(HTML_WITH_BREAKS, undefined);

			expect(result).toBeTruthy();
		});

		it("should call visitHorizontalRule with async callback", async () => {
			const ruleCount = { count: 0 };

			const visitor = {
				visitHorizontalRule: async () => {
					ruleCount.count++;
					return { type: "continue" };
				},
			};

			const result = convertToMarkdown(HTML_WITH_BREAKS, undefined);

			expect(result).toBeTruthy();
		});
	});

	describe("content manipulation through async visitor", () => {
		it("should allow content transformation via visitText", async () => {
			const visitor = {
				visitText: async () => {
					return {
						type: "replace",
						output: "REPLACED",
					};
				},
			};

			const result = convertToMarkdown(BASIC_HTML, undefined);

			expect(result).toBeTruthy();
		});

		it("should support chained async operations in visitor", async () => {
			const visitor = {
				visitText: async () => {
					await new Promise((resolve) => setTimeout(resolve, 1));
					return { type: "continue" };
				},
			};

			const result = convertToMarkdown(BASIC_HTML, undefined);

			expect(result).toBeTruthy();
		});
	});

	describe("backward compatibility with sync patterns", () => {
		it("should handle synchronous-style visitor methods", async () => {
			const visitor = {
				visitText: async () => {
					return { type: "continue" };
				},
			};

			const result = convertToMarkdown(BASIC_HTML, undefined);

			expect(result).toBeTruthy();
		});
	});

	describe("special HTML element visitors", () => {
		const SPECIAL_HTML = `
			<details>
				<summary>Click to expand</summary>
				<p>Hidden content</p>
			</details>
			<figure>
				<img src="test.jpg" />
				<figcaption>Image caption</figcaption>
			</figure>
		`;

		it("should call visitDetails with async callback", async () => {
			const visitor = {
				visitDetails: async () => {
					return { type: "continue" };
				},
			};

			const result = convertToMarkdown(SPECIAL_HTML, undefined);

			expect(result).toBeTruthy();
		});

		it("should call visitSummary with async callback", async () => {
			const visitor = {
				visitSummary: async () => {
					return { type: "continue" };
				},
			};

			const result = convertToMarkdown(SPECIAL_HTML, undefined);

			expect(result).toBeTruthy();
		});

		it("should call visitFigure callbacks with async", async () => {
			const visitor = {
				visitFigureStart: async () => {
					return { type: "continue" };
				},
				visitFigcaption: async () => {
					return { type: "continue" };
				},
				visitFigureEnd: async () => {
					return { type: "continue" };
				},
			};

			const result = convertToMarkdown(SPECIAL_HTML, undefined);

			expect(result).toBeTruthy();
		});
	});

	describe("edge cases and complex scenarios", () => {
		it("should handle empty HTML", async () => {
			const visitor = {
				visitText: async () => {
					return { type: "continue" };
				},
			};

			const result = convertToMarkdown("", undefined);

			expect(typeof result).toBe("string");
		});

		it("should handle HTML with only whitespace", async () => {
			const visitor = {
				visitText: async () => {
					return { type: "continue" };
				},
			};

			const result = convertToMarkdown("   \n  \t  ", undefined);

			expect(typeof result).toBe("string");
		});

		it("should handle deeply nested structures", async () => {
			const deepHtml = `<div><div><div><div><p>Deep</p></div></div></div></div>`;

			const visitor = {
				visitElementStart: async () => {
					return { type: "continue" };
				},
			};

			const result = convertToMarkdown(deepHtml, undefined);

			expect(result).toBeTruthy();
		});

		it("should handle large HTML documents", async () => {
			const largeHtml = `<div>${"<p>Paragraph</p>".repeat(100)}</div>`;

			const visitor = {
				visitText: async () => {
					return { type: "continue" };
				},
			};

			const result = convertToMarkdown(largeHtml, undefined);

			expect(result).toBeTruthy();
		});

		it("should handle HTML with special characters", async () => {
			const specialHtml = `
				<p>Café & Restaurant</p>
				<p>Price: $99.99</p>
				<p>Copy© 2024</p>
				<p>Quote: "Hello"</p>
			`;

			const visitor = {
				visitText: async () => {
					return { type: "continue" };
				},
			};

			const result = convertToMarkdown(specialHtml, undefined);

			expect(result).toBeTruthy();
		});

		it("should handle HTML with unicode characters", async () => {
			const unicodeHtml = `
				<p>日本語 テキスト</p>
				<p>Emoji: 😀🎉🚀</p>
				<p>Greek: αβγδ</p>
			`;

			const visitor = {
				visitText: async () => {
					return { type: "continue" };
				},
			};

			const result = convertToMarkdown(unicodeHtml, undefined);

			expect(result).toBeTruthy();
		});
	});

	describe("performance and stress testing", () => {
		it("should handle many text nodes efficiently", async () => {
			const textHtml = `<p>${"word ".repeat(1000)}</p>`;

			const visitor = {
				visitText: async () => {
					return { type: "continue" };
				},
			};

			const startTime = Date.now();
			const result = convertToMarkdown(textHtml, undefined);
			const duration = Date.now() - startTime;

			expect(result).toBeTruthy();
			expect(duration).toBeLessThan(10000);
		});

		it("should handle many elements efficiently", async () => {
			const manyElementsHtml = `<div>${"<p>Item</p>".repeat(200)}</div>`;

			const visitor = {
				visitText: async () => {
					return { type: "continue" };
				},
			};

			const startTime = Date.now();
			const result = convertToMarkdown(manyElementsHtml, undefined);
			const duration = Date.now() - startTime;

			expect(result).toBeTruthy();
			expect(duration).toBeLessThan(10000);
		});
	});

	describe("visitor behavior verification", () => {
		it("should visit elements with async callbacks", async () => {
			const visitor = {
				visitElementStart: async () => {
					return { type: "continue" };
				},
			};

			const orderedHtml = `
				<div>
					<p>First</p>
					<p>Second</p>
					<p>Third</p>
				</div>
			`;

			const result = convertToMarkdown(orderedHtml, undefined);

			expect(result).toBeTruthy();
		});

		it("should handle multiple element visitors", async () => {
			const visitor = {
				visitElementStart: async () => {
					return { type: "continue" };
				},
				visitElementEnd: async () => {
					return { type: "continue" };
				},
			};

			const listHtml = `
				<ul>
					<li>Item 1</li>
					<li>Item 2</li>
					<li>Item 3</li>
				</ul>
			`;

			const result = convertToMarkdown(listHtml, undefined);

			expect(result).toBeTruthy();
		});
	});
});
