import { readFile } from "node:fs/promises";
import type { Readable } from "node:stream";

import {
	convertToString as convertHtml,
	convertWithInlineImages as convertHtmlWithInlineImages,
	convertWithMetadata as convertHtmlWithMetadata,
	convertWithMetadataBuffer as convertHtmlWithMetadataBuffer,
	convertWithTables as convertHtmlWithTables,
	type JsConversionOptions,
	type JsHtmlExtraction,
	type JsInlineImageConfig,
	type JsMetadataConfig,
	type JsMetadataExtraction,
	type JsTableExtraction,
} from "@kreuzberg/html-to-markdown-node";

export * from "@kreuzberg/html-to-markdown-node";

const normalizeInlineImageConfig = (config?: JsInlineImageConfig | null): JsInlineImageConfig | null | undefined => {
	if (!config) {
		return config;
	}

	const maxDecodedSizeBytes = config.maxDecodedSizeBytes;
	if (typeof maxDecodedSizeBytes === "number") {
		return { ...config, maxDecodedSizeBytes: BigInt(maxDecodedSizeBytes) };
	}

	return config;
};

/**
 * Check if metadata extraction functionality is available.
 *
 * @returns true if convertWithMetadata is available, false otherwise
 */
export function hasMetadataSupport(): boolean {
	try {
		return typeof convertHtmlWithMetadata === "function";
	} catch {
		return false;
	}
}

/**
 * Convert the contents of an HTML file to Markdown.
 */
export async function convertFile(filePath: string, options?: JsConversionOptions | null | undefined): Promise<string> {
	const html = await readFile(filePath, "utf8");
	return convertHtml(html, options ?? undefined);
}

/**
 * Convert an HTML file and collect inline images.
 */
export async function convertFileWithInlineImages(
	filePath: string,
	options?: JsConversionOptions | null | undefined,
	imageConfig?: JsInlineImageConfig | null | undefined,
): Promise<JsHtmlExtraction> {
	const html = await readFile(filePath, "utf8");
	return convertHtmlWithInlineImages(
		html,
		options ?? undefined,
		normalizeInlineImageConfig(imageConfig) ?? undefined,
	);
}

/**
 * Convert HTML streamed from stdin or another readable stream.
 */
export async function convertStream(
	stream: Readable | AsyncIterable<string | Buffer>,
	options?: JsConversionOptions | null | undefined,
): Promise<string> {
	let html = "";

	for await (const chunk of stream as AsyncIterable<string | Buffer>) {
		html += typeof chunk === "string" ? chunk : chunk.toString("utf8");
	}

	return convertHtml(html, options ?? undefined);
}

/**
 * Convert HTML from a stream and collect inline images.
 */
export async function convertStreamWithInlineImages(
	stream: Readable | AsyncIterable<string | Buffer>,
	options?: JsConversionOptions | null | undefined,
	imageConfig?: JsInlineImageConfig | null | undefined,
): Promise<JsHtmlExtraction> {
	let html = "";

	for await (const chunk of stream as AsyncIterable<string | Buffer>) {
		html += typeof chunk === "string" ? chunk : chunk.toString("utf8");
	}

	return convertHtmlWithInlineImages(
		html,
		options ?? undefined,
		normalizeInlineImageConfig(imageConfig) ?? undefined,
	);
}

/**
 * Convert HTML to Markdown and extract comprehensive metadata.
 *
 * Extracts document metadata (title, description, language, etc.), headers,
 * links, images, and structured data (JSON-LD, Microdata, RDFa).
 *
 * @param html HTML content to convert
 * @param options Optional conversion configuration. Supports `skipImages` to skip image conversion
 * @param metadataConfig Optional metadata extraction configuration
 * @returns Object with converted markdown and extracted metadata
 *
 * @example
 * ```ts
 * import { convertWithMetadata } from 'html-to-markdown';
 *
 * const html = `
 *   <html lang="en">
 *     <head>
 *       <title>My Article</title>
 *       <meta name="description" content="An interesting article">
 *     </head>
 *     <body>
 *       <h1>Main Title</h1>
 *       <p>Content with <a href="/page">link</a></p>
 *     </body>
 *   </html>
 * `;
 *
 * const { markdown, metadata } = convertWithMetadata(html, undefined, {
 *   extractHeaders: true,
 *   extractLinks: true,
 *   extractImages: true,
 * });
 *
 * console.log(metadata.document.title);    // "My Article"
 * console.log(metadata.headers.length);    // 1
 * console.log(metadata.links.length);      // 1
 * ```
 */
export function convertWithMetadata(
	html: string,
	options?: JsConversionOptions | null | undefined,
	metadataConfig?: JsMetadataConfig | null | undefined,
): JsMetadataExtraction {
	return convertHtmlWithMetadata(html, options ?? undefined, metadataConfig ?? undefined);
}

/**
 * Convert HTML from Buffer/Uint8Array to Markdown with metadata extraction.
 *
 * Avoids creating intermediate JavaScript strings by accepting raw bytes.
 * Auto-detects UTF-8 encoding.
 */
export function convertWithMetadataBuffer(
	html: Buffer | Uint8Array,
	options?: JsConversionOptions | null | undefined,
	metadataConfig?: JsMetadataConfig | null | undefined,
): JsMetadataExtraction {
	const input = Buffer.isBuffer(html) ? html : Buffer.from(html);
	return convertHtmlWithMetadataBuffer(input, options ?? undefined, metadataConfig ?? undefined);
}

/**
 * Convert the contents of an HTML file to Markdown with metadata extraction.
 */
export async function convertFileWithMetadata(
	filePath: string,
	options?: JsConversionOptions | null | undefined,
	metadataConfig?: JsMetadataConfig | null | undefined,
): Promise<JsMetadataExtraction> {
	const html = await readFile(filePath, "utf8");
	return convertWithMetadata(html, options ?? undefined, metadataConfig ?? undefined);
}

/**
 * Convert HTML streamed from stdin or another readable stream with metadata extraction.
 */
export async function convertStreamWithMetadata(
	stream: Readable | AsyncIterable<string | Buffer>,
	options?: JsConversionOptions | null | undefined,
	metadataConfig?: JsMetadataConfig | null | undefined,
): Promise<JsMetadataExtraction> {
	let html = "";

	for await (const chunk of stream as AsyncIterable<string | Buffer>) {
		html += typeof chunk === "string" ? chunk : chunk.toString("utf8");
	}

	return convertWithMetadata(html, options ?? undefined, metadataConfig ?? undefined);
}

/**
 * Convert HTML to Markdown with structured table extraction.
 *
 * Returns converted content alongside all tables found in the HTML,
 * with each table's cell data and rendered markdown.
 *
 * @param html HTML content to convert
 * @param options Optional conversion configuration
 * @param metadataConfig Optional metadata extraction configuration
 * @returns Object with converted content, extracted tables, and optional metadata
 *
 * @example
 * ```ts
 * import { convertWithTables } from 'html-to-markdown';
 *
 * const html = `
 *   <table>
 *     <tr><th>Name</th><th>Age</th></tr>
 *     <tr><td>Alice</td><td>30</td></tr>
 *   </table>
 * `;
 *
 * const { content, tables } = convertWithTables(html);
 * console.log(tables[0].cells);       // [["Name", "Age"], ["Alice", "30"]]
 * console.log(tables[0].isHeaderRow); // [true, false]
 * ```
 */
export function convertWithTables(
	html: string,
	options?: JsConversionOptions | null | undefined,
	metadataConfig?: JsMetadataConfig | null | undefined,
): JsTableExtraction {
	return convertHtmlWithTables(html, options ?? undefined, metadataConfig ?? undefined);
}

/**
 * Convert the contents of an HTML file to Markdown with table extraction.
 */
export async function convertFileWithTables(
	filePath: string,
	options?: JsConversionOptions | null | undefined,
	metadataConfig?: JsMetadataConfig | null | undefined,
): Promise<JsTableExtraction> {
	const html = await readFile(filePath, "utf8");
	return convertWithTables(html, options ?? undefined, metadataConfig ?? undefined);
}

/**
 * Convert HTML streamed from stdin or another readable stream with table extraction.
 */
export async function convertStreamWithTables(
	stream: Readable | AsyncIterable<string | Buffer>,
	options?: JsConversionOptions | null | undefined,
	metadataConfig?: JsMetadataConfig | null | undefined,
): Promise<JsTableExtraction> {
	let html = "";

	for await (const chunk of stream as AsyncIterable<string | Buffer>) {
		html += typeof chunk === "string" ? chunk : chunk.toString("utf8");
	}

	return convertWithTables(html, options ?? undefined, metadataConfig ?? undefined);
}

/**
 * Type for visitor callback that receives parsed context object
 */
type VisitorCallback<TContext = unknown, TResult = { type: string; output?: string }> = (
	context: TContext,
) => Promise<TResult>;

/**
 * Type for wrapped visitor callback that handles JSON strings
 */
type WrappedVisitorCallback = (jsonString: string) => Promise<string>;

/**
 * Wraps a single visitor callback to handle JSON serialization/deserialization automatically.
 *
 * The native NAPI bindings expect visitor callbacks with signature:
 *   `(jsonString: string) => Promise<string>`
 *
 * This wrapper allows you to write callbacks that receive parsed objects:
 *   `(context: NodeContext) => Promise<{type: string}>`
 *
 * @param callback - Visitor callback that receives parsed context object
 * @returns Wrapped callback that handles JSON string conversion
 *
 * @example
 * ```ts
 * const wrappedCallback = wrapVisitorCallback(async (ctx) => {
 *   console.log('Tag name:', ctx.tagName);
 *   return { type: 'continue' };
 * });
 * ```
 */
export function wrapVisitorCallback<TContext, TResult>(
	callback: VisitorCallback<TContext, TResult>,
): WrappedVisitorCallback {
	return async (jsonString: string): Promise<string> => {
		const context = JSON.parse(jsonString) as TContext;
		const result = await callback(context);
		return JSON.stringify(result);
	};
}

/**
 * Type for visitor object with callbacks that receive parsed objects
 */
type VisitorObject = Record<string, VisitorCallback>;

/**
 * Type for wrapped visitor object with JSON-handling callbacks
 */
type WrappedVisitorObject = Record<string, WrappedVisitorCallback>;

/**
 * Wraps all callbacks in a visitor object to handle JSON serialization/deserialization.
 *
 * This is a convenience function to wrap all callbacks in a visitor object at once.
 *
 * @param visitor - Visitor object with callbacks that receive parsed objects
 * @returns Wrapped visitor object with JSON-handling callbacks
 *
 * @example
 * ```ts
 * const visitor = {
 *   visitElementStart: async (ctx: NodeContext) => {
 *     console.log('Tag:', ctx.tagName);
 *     return { type: 'continue' };
 *   },
 *   visitText: async (ctx: NodeContext, text: string) => {
 *     console.log('Text:', text);
 *     return { type: 'continue' };
 *   },
 * };
 *
 * const wrapped = wrapVisitorCallbacks(visitor);
 * const result = await convertWithVisitor(html, undefined, wrapped);
 * ```
 */
export function wrapVisitorCallbacks(visitor: VisitorObject): WrappedVisitorObject {
	const wrapped: WrappedVisitorObject = {};

	for (const [methodName, callback] of Object.entries(visitor)) {
		if (typeof callback === "function") {
			wrapped[methodName] = wrapVisitorCallback(callback);
		}
	}

	return wrapped;
}
