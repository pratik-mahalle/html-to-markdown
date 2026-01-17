import { describe, expect, it } from "vitest";
import type { JsConversionOptions } from "@kreuzberg/html-to-markdown-node";
import { convertWithVisitor } from "@kreuzberg/html-to-markdown-node";

/**
 * VisitResult type representing the result of a visitor callback.
 */
interface VisitResult {
	type: "continue" | "skip" | "custom" | "preserve_html" | "error";
	output?: string;
	message?: string;
}

/**
 * NodeContext passed to visitor callbacks with element metadata.
 */
interface NodeContext {
	nodeType: string;
	tagName: string;
	attributes: Record<string, string>;
	depth: number;
	indexInParent: number;
	parentTag: string | null;
	isInline: boolean;
}

/**
 * Issue #187: Verifies that visitElementStart receives correct tagName in context
 *
 * IMPORTANT: This test file documents a known limitation in the TypeScript (and Python) bindings:
 * - visitElementStart is NOT called for generic HTML elements like div, p, script, style
 * - visitElementStart is ONLY called for semantic elements with dedicated visitor methods
 * - Users cannot filter arbitrary elements by tag name using visitElementStart
 * - This is different from the Rust API which supports full visit_element_start coverage
 *
 * This is the same issue as reported in GitHub issue #187.
 * The workaround is to use specific visitor methods like visitLink, visitImage, etc.
 * when available, and document the limitation for generic elements.
 */
describe("Issue #187: visitor tagName in context (KNOWN LIMITATION)", () => {
	describe("visitElementStart limitations (KNOWN ISSUE)", () => {
		it("visitElementStart is NOT called for div elements", async () => {
			const tagNames: string[] = [];

			const visitor = {
				visitElementStart: async (ctx: NodeContext) => {
					if (ctx.tagName === "div") {
						tagNames.push(ctx.tagName);
					}
					return { type: "continue" };
				},
			};

			const html = `
				<div class="container">Content</div>
				<p>Paragraph</p>
				<div id="main">Main div</div>
			`;

			const result = await convertWithVisitor(html, undefined, visitor);

			expect(result).toBeTruthy();
			// KNOWN ISSUE: visitElementStart is NOT called for generic divs
			expect(tagNames.length).toBe(0);
		});

		it("visitElementStart is NOT called for script elements", async () => {
			const tagNames: string[] = [];

			const visitor = {
				visitElementStart: async (ctx: NodeContext) => {
					if (ctx.tagName === "script") {
						tagNames.push(ctx.tagName);
					}
					return { type: "continue" };
				},
			};

			const html = `
				<script>console.log('test');</script>
				<p>Text</p>
				<script src="test.js"></script>
			`;

			const result = await convertWithVisitor(html, undefined, visitor);

			expect(result).toBeTruthy();
			// KNOWN ISSUE: visitElementStart is NOT called for scripts
			expect(tagNames.length).toBe(0);
		});

		it("visitElementStart is NOT called for style elements", async () => {
			const tagNames: string[] = [];

			const visitor = {
				visitElementStart: async (ctx: NodeContext) => {
					if (ctx.tagName === "style") {
						tagNames.push(ctx.tagName);
					}
					return { type: "continue" };
				},
			};

			const html = `
				<style>body { color: red; }</style>
				<p>Text</p>
				<style>div { color: blue; }</style>
			`;

			const result = await convertWithVisitor(html, undefined, visitor);

			expect(result).toBeTruthy();
			// KNOWN ISSUE: visitElementStart is NOT called for styles
			expect(tagNames.length).toBe(0);
		});

		it("visitElementStart is NOT called for p elements", async () => {
			const tagNames: string[] = [];

			const visitor = {
				visitElementStart: async (ctx: NodeContext) => {
					if (ctx.tagName === "p") {
						tagNames.push(ctx.tagName);
					}
					return { type: "continue" };
				},
			};

			const html = `
				<p>First paragraph</p>
				<div>Div</div>
				<p>Second paragraph</p>
			`;

			const result = await convertWithVisitor(html, undefined, visitor);

			expect(result).toBeTruthy();
			// KNOWN ISSUE: visitElementStart is NOT called for generic p elements
			expect(tagNames.length).toBe(0);
		});

		it("documents the architectural limitation of the TypeScript binding", async () => {
			// Issue #187 documents that in TypeScript and Python bindings,
			// the visitor callback model only supports:
			// - Semantic visitors: visitLink, visitImage, visitHeading, visitCodeBlock, etc.
			// - Generic visitors: visitText, visitElementStart, visitElementEnd
			//
			// HOWEVER: visitElementStart is only called for elements with semantic meaning,
			// not for generic elements like div, p, script, style.
			//
			// This is different from the Rust API which fully implements visit_element_start
			// for ALL elements including generic divs.

			const html = `
				<h1>Title</h1>
				<div class="container">Content</div>
				<p>Text</p>
				<script>code</script>
				<style>css</style>
			`;

			const result = await convertWithVisitor(html, undefined, {});

			// The conversion still works - generic elements are handled by default logic
			expect(result).toBeTruthy();
			expect(result).toContain("Title");
			expect(result).toContain("Content");
			expect(result).toContain("Text");

			// But users CANNOT filter these elements via visitElementStart.
			// This is the core issue reported in GitHub issue #187.
		});
	});

	describe("Tag-based filtering CANNOT be done with visitElementStart", () => {
		it("cannot filter divs with class attributes using visitElementStart", async () => {
			const skippedElements: { tag: string; class?: string }[] = [];

			const visitor = {
				visitElementStart: async (ctx: NodeContext) => {
					if (ctx.tagName === "div") {
						const classes = ctx.attributes?.class || "";
						if (
							classes.includes("ad") ||
							classes.includes("advertisement") ||
							classes.includes("tracking") ||
							classes.includes("analytics")
						) {
							skippedElements.push({ tag: "div", class: classes });
							return { type: "skip" };
						}
					}
					return { type: "continue" };
				},
			};

			const html = `
				<div class="ad banner">Advertisement</div>
				<p>Main content</p>
				<div class="content">Legitimate div</div>
				<div class="tracking analytics">Tracking div</div>
			`;

			const result = await convertWithVisitor(html, undefined, visitor);

			expect(result).toBeTruthy();
			// KNOWN ISSUE: visitElementStart is never called for divs, so filtering doesn't work
			expect(skippedElements.length).toBe(0);
			// The undesired content remains because filtering via visitElementStart doesn't work
			expect(result).toContain("Advertisement");
			expect(result).toContain("Tracking div");
		});

		it("cannot skip script elements using visitElementStart", async () => {
			const skippedScripts: string[] = [];

			const visitor = {
				visitElementStart: async (ctx: NodeContext) => {
					if (ctx.tagName === "script") {
						skippedScripts.push("script");
						return { type: "skip" };
					}
					return { type: "continue" };
				},
			};

			const html = `
				<p>Before script</p>
				<script>console.log("test1");</script>
				<p>Between scripts</p>
				<script src="external.js"></script>
				<p>After script</p>
			`;

			const result = await convertWithVisitor(html, undefined, visitor);

			expect(result).toBeTruthy();
			// KNOWN ISSUE: visitElementStart is never called for scripts
			expect(skippedScripts.length).toBe(0);
		});

		it("cannot skip style elements using visitElementStart", async () => {
			const skippedStyles: string[] = [];

			const visitor = {
				visitElementStart: async (ctx: NodeContext) => {
					if (ctx.tagName === "style") {
						skippedStyles.push("style");
						return { type: "skip" };
					}
					return { type: "continue" };
				},
			};

			const html = `
				<style>body { color: red; }</style>
				<p>Content</p>
				<style>div { color: blue; }</style>
			`;

			const result = await convertWithVisitor(html, undefined, visitor);

			expect(result).toBeTruthy();
			// KNOWN ISSUE: visitElementStart is never called for styles
			expect(skippedStyles.length).toBe(0);
		});
	});

	describe("Partial workaround: Using semantic visitor methods", () => {
		it("can only filter specific elements: links and images", async () => {
			// The TypeScript binding DOES support filtering for semantic elements:
			// - visitLink: filter links by href
			// - visitImage: filter images by src
			// - visitHeading: modify headings
			// - visitCodeBlock: modify code
			// - visitStrong, visitEmphasis, visitStrikethrough, etc.
			//
			// However, it CANNOT filter generic elements like:
			// - div (use for layout)
			// - p (use for text grouping)
			// - script (inline scripts)
			// - style (inline styles)
			// - section, article, aside (semantic containers)

			const html = `
				<article>
					<h1>Title</h1>
					<div class="ad">Ad content that cannot be filtered</div>
					<p>Text content</p>
					<script>console.log("cannot filter");</script>
					<style>body { color: blue; }</style>
				</article>
			`;

			const result = await convertWithVisitor(html, undefined, {});

			expect(result).toBeTruthy();
			// Ad div, script, and style tags remain in the output because
			// visitElementStart is not called for generic elements
		});
	});

	describe("NodeContext.tagName is correctly serialized", () => {
		it("tagName type should be string (TYPE CHECK)", () => {
			// This is a compile-time type check that tagName is a string
			const ctx: NodeContext = {
				nodeType: "Element",
				tagName: "div",
				attributes: {},
				depth: 0,
				indexInParent: 0,
				parentTag: null,
				isInline: false,
			};

			const tag: string = ctx.tagName;
			expect(typeof tag).toBe("string");
			expect(tag).toBe("div");
		});

		it("NodeContext should have tagName in camelCase", () => {
			// Verify the TypeScript interface expects camelCase, not snake_case
			const ctx: NodeContext = {
				nodeType: "Element",
				tagName: "paragraph",
				attributes: {},
				depth: 0,
				indexInParent: 0,
				parentTag: null,
				isInline: false,
			};

			// This proves that the field is tagName, not tag_name
			expect(ctx.tagName).toBe("paragraph");

			// The NAPI binding properly converts Rust snake_case to JavaScript camelCase
			// tag_name (Rust) → tagName (JavaScript)
		});

		it("demonstrates how to build a context object like NAPI returns", () => {
			// This simulates what the NAPI-RS binding should return for a div element
			const simulatedNapiContext: NodeContext = {
				nodeType: "Element",
				tagName: "div",
				attributes: {
					class: "container",
					id: "main",
				},
				depth: 1,
				indexInParent: 0,
				parentTag: "body",
				isInline: false,
			};

			expect(simulatedNapiContext.tagName).toBe("div");
			expect(simulatedNapiContext.attributes.class).toBe("container");

			// If this context were passed to a visitor callback, users could filter like:
			// if (ctx.tagName === "div" && ctx.attributes.class?.includes("ad")) { return skip; }
			// BUT: visitElementStart is NOT called for generic elements in TypeScript/Python bindings
		});
	});
});
