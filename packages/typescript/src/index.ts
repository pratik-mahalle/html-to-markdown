import { readFile } from "node:fs/promises";
import type { Readable } from "node:stream";

import {
	convert as convertHtml,
	type JsConversionOptions,
	type JsConversionResult,
} from "@kreuzberg/html-to-markdown-node";

export * from "@kreuzberg/html-to-markdown-node";

/**
 * Convert HTML to Markdown, returning a structured result with content, tables, and warnings.
 *
 * @param html HTML content to convert
 * @param options Optional conversion configuration
 * @returns JsConversionResult with content, tables, and warnings fields
 *
 * @example
 * ```ts
 * import { convert } from 'html-to-markdown';
 *
 * const result = convert('<h1>Hello</h1><p>World</p>');
 * console.log(result.content);   // '# Hello\n\nWorld'
 * console.log(result.tables);    // []
 * console.log(result.warnings);  // []
 * ```
 */
export function convert(html: string, options?: JsConversionOptions | null | undefined): JsConversionResult {
	return convertHtml(html, options ?? undefined);
}

/**
 * Convert the contents of an HTML file to Markdown.
 */
export async function convertFile(
	filePath: string,
	options?: JsConversionOptions | null | undefined,
): Promise<JsConversionResult> {
	const html = await readFile(filePath, "utf8");
	return convertHtml(html, options ?? undefined);
}

/**
 * Convert HTML streamed from stdin or another readable stream.
 */
export async function convertStream(
	stream: Readable | AsyncIterable<string | Buffer>,
	options?: JsConversionOptions | null | undefined,
): Promise<JsConversionResult> {
	let html = "";

	for await (const chunk of stream as AsyncIterable<string | Buffer>) {
		html += typeof chunk === "string" ? chunk : chunk.toString("utf8");
	}

	return convertHtml(html, options ?? undefined);
}
