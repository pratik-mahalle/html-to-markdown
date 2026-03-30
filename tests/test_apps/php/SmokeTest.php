<?php

declare(strict_types=1);

use function HtmlToMarkdown\convert;
use PHPUnit\Framework\TestCase;

final class SmokeTest extends TestCase
{
    public function testPackageLoads(): void
    {
        $this->assertTrue(extension_loaded('html_to_markdown'));
    }

    public function testBasicConversionFunctionInterface(): void
    {
        $html = '<p>Hello World</p>';
        $result = convert($html);
        $this->assertStringContainsString('Hello World', $result);
    }

    public function testBasicConversionExtensionInterface(): void
    {
        $html = '<p>Hello World</p>';
        $result = html_to_markdown_convert($html);
        $this->assertStringContainsString('Hello World', $result);
    }

    public function testHeadingConversion(): void
    {
        $html = '<h1>Title</h1>';
        $result = convert($html);
        $this->assertStringContainsString('Title', $result);
    }

    public function testEmptyInput(): void
    {
        $result = convert('');
        $this->assertSame('', $result);
    }

    public function testParagraphConversion(): void
    {
        $html = '<p>Hello <strong>World</strong></p>';
        $result = convert($html);
        $this->assertStringContainsString('Hello', $result);
        $this->assertStringContainsString('World', $result);
    }

    public function testListConversion(): void
    {
        $html = '<ul><li>Item 1</li><li>Item 2</li></ul>';
        $result = convert($html);
        $this->assertStringContainsString('Item 1', $result);
        $this->assertStringContainsString('Item 2', $result);
    }

    public function testLinkConversion(): void
    {
        $html = '<a href="https://example.com">Example</a>';
        $result = convert($html);
        $this->assertStringContainsString('Example', $result);
        $this->assertStringContainsString('https://example.com', $result);
    }

    public function testCodeConversion(): void
    {
        $html = '<code>console.log()</code>';
        $result = convert($html);
        $this->assertStringContainsString('console.log', $result);
    }

    public function testBlockquoteConversion(): void
    {
        $html = '<blockquote>Quote text</blockquote>';
        $result = convert($html);
        $this->assertStringContainsString('Quote text', $result);
    }
}
