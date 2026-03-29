import { rm, writeFile } from "node:fs/promises";
import { Readable } from "node:stream";
import { describe, expect, it } from "vitest";
import { convert, convertFile, convertStream } from "../src/index";

const BASIC_HTML = `
<html lang="en">
	<head>
		<title>Test Article</title>
		<meta name="description" content="A test article about markdown">
		<meta name="author" content="Test Author">
		<meta name="keywords" content="markdown, html, conversion">
		<meta property="og:title" content="OG Title">
		<meta property="og:description" content="OG Description">
		<meta name="twitter:card" content="summary">
		<link rel="canonical" href="https://example.com/article">
	</head>
	<body>
		<h1 id="main-title">Main Title</h1>
		<h2>Section 1</h2>
		<p>Some text with <a href="/page">internal link</a> and <a href="https://example.com">external link</a>.</p>
		<h3>Subsection</h3>
		<p>More content with <a href="mailto:test@example.com">email link</a> and <a href="#section">anchor</a>.</p>
		<img src="https://example.com/image.jpg" alt="Test Image" title="Test Title">
		<img src="data:image/png;base64,iVBORw0KGgo=" alt="Embedded Image">
	</body>
</html>
`;

describe("html-to-markdown metadata extraction (TypeScript)", () => {
	it("extracts document metadata", () => {
		const result = convert(BASIC_HTML, { extractMetadata: true });

		expect(result).toHaveProperty("content");
		expect(result).toHaveProperty("metadata");

		const metadata = JSON.parse(result.metadata!);
		expect(metadata.document.title).toBe("Test Article");
		expect(metadata.document.description).toBe("A test article about markdown");
		expect(metadata.document.author).toBe("Test Author");
		expect(metadata.document.language).toBe("en");
		expect(metadata.document.canonical_url).toBe("https://example.com/article");

		expect(metadata.document.keywords).toContain("markdown");
		expect(metadata.document.keywords).toContain("html");
		expect(metadata.document.keywords).toContain("conversion");

		expect(metadata.document.open_graph).toHaveProperty("title", "OG Title");
		expect(metadata.document.open_graph).toHaveProperty("description", "OG Description");

		expect(metadata.document.twitter_card).toHaveProperty("card", "summary");
	});

	it("extracts header metadata with hierarchy", () => {
		const result = convert(BASIC_HTML, { extractMetadata: true });
		const metadata = JSON.parse(result.metadata!);
		const headers = metadata.headers;

		expect(headers.length).toBeGreaterThanOrEqual(3);

		const h1 = headers.find((h: { level: number }) => h.level === 1);
		expect(h1).toBeDefined();
		expect(h1?.text).toBe("Main Title");
		expect(h1?.id).toBe("main-title");

		const h2 = headers.find((h: { level: number }) => h.level === 2);
		expect(h2).toBeDefined();
		expect(h2?.text).toBe("Section 1");

		const h3 = headers.find((h: { level: number }) => h.level === 3);
		expect(h3).toBeDefined();
		expect(h3?.text).toBe("Subsection");
	});

	it("extracts and classifies links", () => {
		const result = convert(BASIC_HTML, { extractMetadata: true });
		const metadata = JSON.parse(result.metadata!);
		const links = metadata.links;

		expect(links.length).toBeGreaterThanOrEqual(4);

		const internalLink = links.find((l: { href: string }) => l.href === "/page");
		expect(internalLink).toBeDefined();
		expect(internalLink?.link_type).toBe("internal");
		expect(internalLink?.text).toBe("internal link");

		const externalLink = links.find((l: { href: string }) => l.href === "https://example.com");
		expect(externalLink).toBeDefined();
		expect(externalLink?.link_type).toBe("external");

		const emailLink = links.find((l: { href: string }) => l.href === "mailto:test@example.com");
		expect(emailLink).toBeDefined();
		expect(emailLink?.link_type).toBe("email");
		expect(emailLink?.text).toBe("email link");

		const anchorLink = links.find((l: { href: string }) => l.href === "#section");
		expect(anchorLink).toBeDefined();
		expect(anchorLink?.link_type).toBe("anchor");
	});

	it("extracts image metadata with types", () => {
		const result = convert(BASIC_HTML, { extractMetadata: true });
		const metadata = JSON.parse(result.metadata!);
		const images = metadata.images;

		expect(images.length).toBeGreaterThanOrEqual(2);

		const externalImg = images.find((i: { src: string }) => i.src === "https://example.com/image.jpg");
		expect(externalImg).toBeDefined();
		expect(externalImg?.image_type).toBe("external");
		expect(externalImg?.alt).toBe("Test Image");
		expect(externalImg?.title).toBe("Test Title");

		const dataUriImg = images.find((i: { src: string }) => i.src.startsWith("data:image"));
		expect(dataUriImg).toBeDefined();
		expect(dataUriImg?.image_type).toBe("data_uri");
		expect(dataUriImg?.alt).toBe("Embedded Image");
	});

	it("converts from file with metadata", async () => {
		const path = "tmp-metadata-test.html";
		await writeFile(path, BASIC_HTML, "utf8");

		try {
			const result = await convertFile(path, { extractMetadata: true });

			expect(result.content).toContain("Main Title");
			const metadata = JSON.parse(result.metadata!);
			expect(metadata.document.title).toBe("Test Article");
			expect(metadata.document.author).toBe("Test Author");
		} finally {
			await rm(path, { force: true });
		}
	});

	it("converts from stream with metadata", async () => {
		const stream = Readable.from([BASIC_HTML]);
		const result = await convertStream(stream, { extractMetadata: true });

		expect(result.content).toContain("Main Title");
		const metadata = JSON.parse(result.metadata!);
		expect(metadata.document.title).toBe("Test Article");
		expect(metadata.headers.length).toBeGreaterThan(0);
	});

	it("handles HTML with minimal metadata", () => {
		const minimalHtml = "<h1>Title</h1><p>Content</p>";
		const result = convert(minimalHtml, { extractMetadata: true });

		const metadata = JSON.parse(result.metadata!);
		expect(metadata.document.title).toBeUndefined();
		expect(metadata.document.description).toBeUndefined();

		expect(metadata.headers.length).toBe(1);
		expect(metadata.headers[0].text).toBe("Title");
	});

	it("extracts markdown with metadata", () => {
		const result = convert(BASIC_HTML, { extractMetadata: true });

		expect(result.content).toContain("Main Title");
		expect(result.content).toContain("Section 1");
		expect(result.content).toContain("Subsection");
		expect(result.content).toContain("internal link");
	});

	it("handles structured data size limits", () => {
		const htmlWithLargeJson = `
		<html>
			<head>
				<script type="application/ld+json">
					${JSON.stringify({ "@type": "Article", content: "x".repeat(10000) })}
				</script>
			</head>
			<body><p>Test</p></body>
		</html>
		`;

		const result1 = convert(htmlWithLargeJson, { extractMetadata: true });
		expect(result1).toHaveProperty("metadata");

		const result2 = convert(htmlWithLargeJson, { extractMetadata: true });
		expect(result2).toHaveProperty("metadata");
	});

	it("handles special characters in metadata", () => {
		const specialHtml = `
		<html lang="fr">
			<head>
				<title>Café & Restaurant</title>
				<meta name="description" content="A guide to French cuisine">
			</head>
			<body>
				<h1>Délicieux!</h1>
				<a href="/résumé">Résumé</a>
			</body>
		</html>
		`;

		const result = convert(specialHtml, { extractMetadata: true });
		const metadata = JSON.parse(result.metadata!);

		expect(metadata.document.title).toBe("Café & Restaurant");
		expect(metadata.document.language).toBe("fr");
		expect(metadata.headers[0].text).toBe("Délicieux!");
	});
});
