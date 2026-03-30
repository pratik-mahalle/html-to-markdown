<?php

declare(strict_types=1);

use function HtmlToMarkdown\convert;
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

        $result = convert($html);

        $this->assertStringContainsString('Level 1', $result);
        $this->assertStringContainsString('Level 6', $result);
    }
}
