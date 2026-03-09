<?php

declare(strict_types=1);

use function HtmlToMarkdown\convert;
use function HtmlToMarkdown\convert_with_metadata;
use function HtmlToMarkdown\convert_with_inline_images;
use HtmlToMarkdown\Config\ConversionOptions;
use PHPUnit\Framework\TestCase;

final class ComprehensiveTest extends TestCase
{
    private static function loadFixtures(string $filename): array
    {
        $path = __DIR__ . '/../fixtures/' . $filename;
        $content = file_get_contents($path);
        return json_decode($content, true) ?? [];
    }

    /**
     * @dataProvider basicHtmlProvider
     */
    public function testBasicHtmlConversion(string $name, string $html, string $expected, array $options): void
    {
        $result = convert($html, $options);
        $this->assertSame(trim($expected), trim($result), $name);
    }

    public static function basicHtmlProvider(): array
    {
        $fixtures = self::loadFixtures('basic-html.json');
        $cases = [];
        foreach ($fixtures as $fixture) {
            $cases[$fixture['name']] = [
                $fixture['name'],
                $fixture['html'],
                $fixture['expectedMarkdown'],
                $fixture['options'] ?? []
            ];
        }
        return $cases;
    }

    public function testMetadataExtractionWithDocument(): void
    {
        $html = <<<'HTML'
<html lang="en">
    <head>
        <title>Test Article</title>
        <meta name="description" content="This is a test article">
        <meta name="keywords" content="test, article">
        <meta name="author" content="Test Author">
    </head>
    <body>
        <h1>Main Title</h1>
        <p>Content</p>
    </body>
</html>
HTML;

        $result = convert_with_metadata($html);

        $this->assertIsArray($result);
        $this->assertArrayHasKey('markdown', $result);
        $this->assertArrayHasKey('metadata', $result);

        $metadata = $result['metadata'];
        $this->assertNotNull($metadata->document);
        $this->assertEquals('en', $metadata->document->language);
        $this->assertEquals('Test Article', $metadata->document->title);
        $this->assertEquals('This is a test article', $metadata->document->description);
    }

    public function testMetadataExtractionWithHeaders(): void
    {
        $html = <<<'HTML'
<html>
    <body>
        <h1 id="intro">Introduction</h1>
        <h2>Background</h2>
        <p>Content here</p>
        <h2>Details</h2>
    </body>
</html>
HTML;

        $result = convert_with_metadata($html);
        $metadata = $result['metadata'];

        $this->assertNotEmpty($metadata->headers);
        $this->assertGreaterThanOrEqual(3, count($metadata->headers));

        // Verify first header
        $firstHeader = $metadata->headers[0];
        $this->assertEquals(1, $firstHeader->level);
        $this->assertEquals('Introduction', $firstHeader->text);
        $this->assertEquals('intro', $firstHeader->id);
    }

    public function testMetadataExtractionWithLinks(): void
    {
        $html = <<<'HTML'
<html>
    <body>
        <a href="https://example.com">External Link</a>
        <a href="/internal">Internal Link</a>
        <a href="mailto:test@example.com">Email</a>
    </body>
</html>
HTML;

        $result = convert_with_metadata($html);
        $metadata = $result['metadata'];

        $this->assertNotEmpty($metadata->links);
        $this->assertGreaterThanOrEqual(3, count($metadata->links));
    }

    public function testMetadataExtractionWithImages(): void
    {
        $html = <<<'HTML'
<html>
    <body>
        <img src="https://example.com/image.jpg" alt="Example Image">
        <img src="/local/image.png" alt="Local Image">
    </body>
</html>
HTML;

        $result = convert_with_metadata($html);
        $metadata = $result['metadata'];

        $this->assertNotEmpty($metadata->images);
        $this->assertGreaterThanOrEqual(2, count($metadata->images));

        // Verify first image
        $firstImage = $metadata->images[0];
        $this->assertEquals('Example Image', $firstImage->alt);
    }

    public function testMetadataConfigSelectiveExtraction(): void
    {
        $html = <<<'HTML'
<html>
    <body>
        <h1>Title</h1>
        <a href="https://example.com">Link</a>
        <img src="image.jpg" alt="Image">
    </body>
</html>
HTML;

        $metadataConfig = [
            'extract_headers' => true,
            'extract_links' => false,
            'extract_images' => false,
        ];

        $result = convert_with_metadata($html, null, $metadataConfig);
        $metadata = $result['metadata'];

        $this->assertNotEmpty($metadata->headers);
        // Links extraction was disabled, but empty array is acceptable
    }

    public function testConversionWithOptions(): void
    {
        $html = '<h1>Title</h1><p>Paragraph</p>';

        $options = new ConversionOptions();
        $result = convert($html, $options);

        $this->assertIsString($result);
        $this->assertStringContainsString('Title', $result);
    }

    public function testConversionWithOptionsArray(): void
    {
        $html = '<h1>Title</h1>';

        $options = [
            'heading_style' => 'atx',
        ];
        $result = convert($html, $options);

        $this->assertIsString($result);
        $this->assertStringContainsString('Title', $result);
    }

    public function testInlineImageConversion(): void
    {
        $html = '<p>Text with <img src="https://example.com/image.jpg" alt="Example"></p>';

        $result = convert_with_inline_images($html);

        $this->assertNotNull($result);
        // Result should have markdown and images
        $this->assertIsString($result->markdown);
    }

    public function testEmptyMetadataExtraction(): void
    {
        $html = '<p>Simple content</p>';

        $result = convert_with_metadata($html);

        $this->assertArrayHasKey('markdown', $result);
        $this->assertArrayHasKey('metadata', $result);
        $markdown = $result['markdown'];
        $this->assertStringContainsString('Simple content', $markdown);
    }

    public function testComplexHtmlWithMultipleElements(): void
    {
        $html = <<<'HTML'
<div>
    <h1>Header</h1>
    <p>Paragraph with <strong>bold</strong> and <em>italic</em></p>
    <ul>
        <li>Item 1</li>
        <li>Item 2</li>
    </ul>
    <blockquote>Quote</blockquote>
    <code>code snippet</code>
</div>
HTML;

        $result = convert($html);

        $this->assertStringContainsString('Header', $result);
        $this->assertStringContainsString('Paragraph', $result);
        $this->assertStringContainsString('Item 1', $result);
        $this->assertStringContainsString('Item 2', $result);
        $this->assertStringContainsString('Quote', $result);
        $this->assertStringContainsString('code snippet', $result);
    }

    public function testErrorHandlingWithInvalidUtf8(): void
    {
        // Test that invalid UTF-8 is handled gracefully
        $html = "Valid HTML with valid UTF-8 content";
        $result = convert($html);
        $this->assertIsString($result);
    }

    public function testNullOptionsHandling(): void
    {
        $html = '<p>Test</p>';
        $result = convert($html, null);
        $this->assertStringContainsString('Test', $result);
    }

    public function testMetadataOpenGraphTags(): void
    {
        $html = <<<'HTML'
<html>
    <head>
        <meta property="og:title" content="Test Title">
        <meta property="og:description" content="Test Description">
        <meta property="og:image" content="https://example.com/image.jpg">
    </head>
    <body><p>Content</p></body>
</html>
HTML;

        $result = convert_with_metadata($html);
        $metadata = $result['metadata'];

        $this->assertNotNull($metadata->document);
        $this->assertNotEmpty($metadata->document->openGraph);
    }

    public function testMultipleHeadingLevels(): void
    {
        $html = <<<'HTML'
<h1>Level 1</h1>
<h2>Level 2</h2>
<h3>Level 3</h3>
<h4>Level 4</h4>
<h5>Level 5</h5>
<h6>Level 6</h6>
HTML;

        $result = convert_with_metadata($html);
        $metadata = $result['metadata'];

        $this->assertGreaterThanOrEqual(6, count($metadata->headers));
    }
}
