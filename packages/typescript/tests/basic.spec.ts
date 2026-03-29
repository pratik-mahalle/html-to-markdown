import { rm, writeFile } from "node:fs/promises";
import { Readable } from "node:stream";
import { describe, expect, it } from "vitest";
import { convert, convertFile, convertStream, JsHeadingStyle, JsOutputFormat } from "../src/index";

const FIXTURE_HTML = "<h1>Hello</h1><p>Markdown</p>";

describe("html-to-markdown (TypeScript package)", () => {
	it("converts inline HTML strings", () => {
		const result = convert(FIXTURE_HTML, { headingStyle: JsHeadingStyle.Atx });
		expect(result.content).toContain("# Hello");
		expect(result.content).toContain("Markdown");
	});

	it("converts files", async () => {
		const path = "tmp-test.html";
		await writeFile(path, FIXTURE_HTML, "utf8");
		try {
			const result = await convertFile(path);
			expect(result.content).toContain("Hello");
		} finally {
			await rm(path, { force: true });
		}
	});

	it("converts streams", async () => {
		const stream = Readable.from([FIXTURE_HTML]);
		const result = await convertStream(stream, { headingStyle: JsHeadingStyle.Atx });
		expect(result.content).toContain("# Hello");
	});

	it("converts to Markdown output format", () => {
		const result = convert(FIXTURE_HTML, { outputFormat: JsOutputFormat.Markdown });
		expect(result.content).toContain("# Hello");
		expect(result.content).toContain("Markdown");
	});

	it("converts to Djot output format", () => {
		const result = convert(FIXTURE_HTML, { outputFormat: JsOutputFormat.Djot });
		expect(result.content).toContain("Hello");
		expect(result.content).toContain("Markdown");
	});
});
