<?php

declare(strict_types=1);

namespace HtmlToMarkdown\Tests;

use HtmlToMarkdown\HtmlToMarkdown;
use HtmlToMarkdown\Value\ExtendedMetadata;

use function HtmlToMarkdown\convert_with_metadata;

final class MetadataExtractionTest extends TestCase
{
    public function testConvertWithMetadataReturnsMarkdownAndMetadata(): void
    {
        $html = '<html><head><title>Test Page</title></head><body><p>Content</p></body></html>';
        $result = convert_with_metadata($html);

        self::assertArrayHasKey('markdown', $result);
        self::assertArrayHasKey('metadata', $result);
        self::assertInstanceOf(ExtendedMetadata::class, $result['metadata']);
    }

    public function testMetadataExtractionWithTitle(): void
    {
        $html = '<html><head><title>My Page Title</title></head><body><p>Content</p></body></html>';
        $result = convert_with_metadata($html);

        self::assertSame('My Page Title', $result['metadata']->document->title);
    }

    public function testMetadataExtractionWithDescription(): void
    {
        $html = '<html><head><meta name="description" content="Page description"></head>'
            . '<body><p>Content</p></body></html>';
        $result = convert_with_metadata($html);

        self::assertSame('Page description', $result['metadata']->document->description);
    }

    public function testMetadataExtractionWithKeywords(): void
    {
        $html = '<html><head><meta name="keywords" content="keyword1, keyword2, keyword3"></head>'
            . '<body><p>Content</p></body></html>';
        $result = convert_with_metadata($html);

        self::assertNotEmpty($result['metadata']->document->keywords);
    }

    public function testMetadataExtractionWithAuthor(): void
    {
        $html = '<html><head><meta name="author" content="John Doe"></head><body><p>Content</p></body></html>';
        $result = convert_with_metadata($html);

        self::assertSame('John Doe', $result['metadata']->document->author);
    }

    public function testMetadataExtractionWithCanonicalLink(): void
    {
        $html = '<html><head><link rel="canonical" href="https://example.com/page"></head>'
            . '<body><p>Content</p></body></html>';
        $result = convert_with_metadata($html);

        self::assertSame('https://example.com/page', $result['metadata']->document->canonicalUrl);
    }

    public function testMetadataExtractionWithBaseHref(): void
    {
        $html = '<html><head><base href="https://example.com/"></head><body><p>Content</p></body></html>';
        $result = convert_with_metadata($html);

        self::assertSame('https://example.com/', $result['metadata']->document->baseHref);
    }

    public function testMetadataExtractionWithLanguage(): void
    {
        $html = '<html lang="en"><head><title>Test</title></head><body><p>Content</p></body></html>';
        $result = convert_with_metadata($html);

        self::assertSame('en', $result['metadata']->document->language);
    }

    public function testMetadataExtractionWithTextDirection(): void
    {
        $html = '<html dir="rtl"><head><title>Test</title></head><body><p>Content</p></body></html>';
        $result = convert_with_metadata($html);

        self::assertSame('rtl', $result['metadata']->document->textDirection);
    }

    public function testMetadataExtractionWithOpenGraph(): void
    {
        $html = '<html><head>
            <meta property="og:title" content="OG Title">
            <meta property="og:description" content="OG Description">
        </head><body><p>Content</p></body></html>';
        $result = convert_with_metadata($html);

        self::assertNotEmpty($result['metadata']->document->openGraph);
    }

    public function testMetadataExtractionWithHeaders(): void
    {
        $html = '<html><body><h1>Header 1</h1><h2>Header 2</h2></body></html>';
        $result = convert_with_metadata($html);

        self::assertGreaterThanOrEqual(2, \count($result['metadata']->headers));
    }

    public function testMetadataExtractionWithLinks(): void
    {
        $html = '<html><body><a href="https://example.com">Link 1</a>'
            . '<a href="https://example2.com">Link 2</a></body></html>';
        $result = convert_with_metadata($html);

        self::assertNotEmpty($result['metadata']->links);
    }

    public function testMetadataExtractionWithImages(): void
    {
        $html = '<html><body><img src="https://example.com/image.jpg" alt="Test Image"></body></html>';
        $result = convert_with_metadata($html);

        self::assertNotEmpty($result['metadata']->images);
    }

    public function testMetadataExtractionWithOptions(): void
    {
        $html = '<html><head><title>Test</title></head><body><p>Content</p></body></html>';
        $options = ['heading_style' => 'atx'];
        $result = convert_with_metadata($html, $options);

        self::assertSame('Test', $result['metadata']->document->title);
    }

    public function testMetadataExtractionWithConfig(): void
    {
        $html = '<html><body><h1>Header</h1><a href="https://example.com">Link</a></body></html>';
        $config = [
            'extract_headers' => true,
            'extract_links' => true,
            'extract_images' => false,
        ];
        $result = convert_with_metadata($html, null, $config);

        self::assertNotNull($result['metadata']->headers);
        self::assertNotNull($result['metadata']->links);
        self::assertNotNull($result['metadata']->images);
    }

    public function testMetadataExtractionViaFacade(): void
    {
        $html = '<html><head><title>Test</title></head><body><p>Content</p></body></html>';
        $result = HtmlToMarkdown::convertWithMetadata($html);

        self::assertArrayHasKey('markdown', $result);
        self::assertArrayHasKey('metadata', $result);
        self::assertInstanceOf(ExtendedMetadata::class, $result['metadata']);
    }

    public function testMetadataHeadersHaveRequiredFields(): void
    {
        $html = '<html><body><h1 id="header-1">Test Header</h1></body></html>';
        $result = convert_with_metadata($html);

        if (\count($result['metadata']->headers) > 0) {
            $header = $result['metadata']->headers[0];
            self::assertGreaterThanOrEqual(1, $header->level);
            self::assertNotEmpty($header->text);
            self::assertGreaterThanOrEqual(0, $header->depth);
            self::assertGreaterThanOrEqual(0, $header->htmlOffset);
        }
    }

    public function testMetadataLinksHaveRequiredFields(): void
    {
        $html = '<html><body><a href="https://example.com" title="Example">Link</a></body></html>';
        $result = convert_with_metadata($html);

        if (\count($result['metadata']->links) > 0) {
            $link = $result['metadata']->links[0];
            self::assertNotEmpty($link->href);
            self::assertNotEmpty($link->text);
            self::assertNotEmpty($link->linkType);
            if (\count($link->rel) > 0) {
                self::assertNotEmpty($link->rel[0]);
            }
        }
    }

    public function testMetadataImagesHaveRequiredFields(): void
    {
        $html = '<html><body><img src="https://example.com/image.jpg" alt="Test" title="Image"></body></html>';
        $result = convert_with_metadata($html);

        if (\count($result['metadata']->images) > 0) {
            $image = $result['metadata']->images[0];
            self::assertNotEmpty($image->src);
            self::assertNotEmpty($image->imageType);
        }
    }

    public function testMetadataExtractionWithMultipleMetaTags(): void
    {
        $html = '<html><head>
            <title>Page Title</title>
            <meta name="description" content="Page description">
            <meta name="author" content="John Doe">
            <link rel="canonical" href="https://example.com/page">
        </head><body><p>Content</p></body></html>';
        $result = convert_with_metadata($html);

        self::assertSame('Page Title', $result['metadata']->document->title);
        self::assertSame('Page description', $result['metadata']->document->description);
        self::assertSame('John Doe', $result['metadata']->document->author);
        self::assertSame('https://example.com/page', $result['metadata']->document->canonicalUrl);
    }

    public function testExtendedMetadataToArray(): void
    {
        $html = '<html><head><title>Test</title></head><body><p>Content</p></body></html>';
        $result = convert_with_metadata($html);

        $array = $result['metadata']->toArray();
        self::assertArrayHasKey('document', $array);
        self::assertArrayHasKey('headers', $array);
        self::assertArrayHasKey('links', $array);
        self::assertArrayHasKey('images', $array);
        self::assertArrayHasKey('structured_data', $array);
    }
}
