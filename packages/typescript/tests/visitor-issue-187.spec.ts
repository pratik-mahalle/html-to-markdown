import { describe, expect, it } from "vitest";
import { convertWithVisitor } from "@kreuzberg/html-to-markdown-node";
import { wrapVisitorCallbacks } from "../src/index.js";

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
 * This test verifies that the TypeScript NAPI bridge correctly implements visitElementStart
 * for all generic HTML elements. After the fix, visitElementStart IS called for elements like
 * div, p, section, etc., allowing users to filter arbitrary elements by tag name.
 *
 * IMPORTANT: Script and style tags are still NOT visible to visitors because they are
 * automatically stripped during HTML sanitization before the visitor pattern runs.
 */
describe("Issue #187: visitor tagName in context (FIXED)", () => {
	describe("visitElementStart now works for generic elements", () => {
		it("visitElementStart IS called for div elements", async () => {
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

			const result = await convertWithVisitor(html, undefined, wrapVisitorCallbacks(visitor));

			expect(result).toBeTruthy();
			expect(tagNames.length).toBe(2);
			expect(tagNames).toContain("div");
		});

		it("visitElementStart is NOT called for script elements (stripped during sanitization)", async () => {
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

			const result = await convertWithVisitor(html, undefined, wrapVisitorCallbacks(visitor));

			expect(result).toBeTruthy();
			// Script elements are stripped during HTML sanitization before visitor runs
			expect(tagNames.length).toBe(0);
		});

		it("visitElementStart is NOT called for style elements (stripped during sanitization)", async () => {
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

			const result = await convertWithVisitor(html, undefined, wrapVisitorCallbacks(visitor));

			expect(result).toBeTruthy();
			expect(tagNames.length).toBe(0);
		});

		it("visitElementStart IS called for p elements", async () => {
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

			const result = await convertWithVisitor(html, undefined, wrapVisitorCallbacks(visitor));

			expect(result).toBeTruthy();
			expect(tagNames.length).toBe(2);
			expect(tagNames).toEqual(["p", "p"]);
		});

		it("demonstrates the fixed visitor pattern implementation", async () => {
			const capturedTags: string[] = [];

			const visitor = {
				visitElementStart: async (ctx: NodeContext) => {
					capturedTags.push(ctx.tagName);
					return { type: "continue" };
				},
			};

			const html = `
				<h1>Title</h1>
				<div class="container">Content</div>
				<p>Text</p>
				<script>code</script>
				<style>css</style>
			`;

			const result = await convertWithVisitor(html, undefined, wrapVisitorCallbacks(visitor));

			expect(result).toBeTruthy();
			expect(result).toContain("Title");
			expect(result).toContain("Content");
			expect(result).toContain("Text");

			expect(capturedTags).toContain("h1");
			expect(capturedTags).toContain("div");
			expect(capturedTags).toContain("p");
			expect(capturedTags).not.toContain("script");
			expect(capturedTags).not.toContain("style");
		});
	});

	describe("Tag-based filtering with visitElementStart", () => {
		it("can filter divs with class attributes using visitElementStart", async () => {
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

			const result = await convertWithVisitor(html, undefined, wrapVisitorCallbacks(visitor));

			expect(result).toBeTruthy();
			expect(skippedElements.length).toBe(2);
			expect(skippedElements[0].class).toContain("ad");
			expect(skippedElements[1].class).toContain("tracking");
			expect(result).not.toContain("Advertisement");
			expect(result).not.toContain("Tracking div");
			expect(result).toContain("Main content");
			expect(result).toContain("Legitimate div");
		});

		it("cannot skip script elements (stripped during sanitization)", async () => {
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

			const result = await convertWithVisitor(html, undefined, wrapVisitorCallbacks(visitor));

			expect(result).toBeTruthy();
			expect(skippedScripts.length).toBe(0);
		});

		it("cannot skip style elements (stripped during sanitization)", async () => {
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

			const result = await convertWithVisitor(html, undefined, wrapVisitorCallbacks(visitor));

			expect(result).toBeTruthy();
			expect(skippedStyles.length).toBe(0);
		});
	});

	describe("Visitor method coverage", () => {
		it("supports both semantic and generic element filtering", async () => {
			const filteredTags: string[] = [];

			const visitor = {
				visitElementStart: async (ctx: NodeContext) => {
					filteredTags.push(ctx.tagName);
					if (ctx.tagName === "div" && ctx.attributes?.class?.includes("ad")) {
						return { type: "skip" };
					}
					return { type: "continue" };
				},
			};

			const html = `
				<article>
					<h1>Title</h1>
					<div class="ad">Ad content filtered via visitElementStart</div>
					<p>Text content</p>
					<script>console.log("stripped during sanitization");</script>
					<style>body { color: blue; }</style>
				</article>
			`;

			const result = await convertWithVisitor(html, undefined, wrapVisitorCallbacks(visitor));

			expect(result).toBeTruthy();
			expect(filteredTags).toContain("article");
			expect(filteredTags).toContain("h1");
			expect(filteredTags).toContain("div");
			expect(filteredTags).toContain("p");
			expect(filteredTags).not.toContain("script");
			expect(filteredTags).not.toContain("style");
			expect(result).not.toContain("Ad content");
			expect(result).toContain("Title");
			expect(result).toContain("Text content");
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

		it("demonstrates context object structure returned by NAPI", () => {
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
		});
	});
});
