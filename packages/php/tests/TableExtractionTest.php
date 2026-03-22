<?php

declare(strict_types=1);

namespace HtmlToMarkdown\Tests;

use HtmlToMarkdown\HtmlToMarkdown;
use HtmlToMarkdown\Value\TableData;
use HtmlToMarkdown\Value\TableExtractionResult;

use function HtmlToMarkdown\convert_with_tables;

final class TableExtractionTest extends TestCase
{
    public function testConvertWithTablesExtractsSimpleTable(): void
    {
        $html = '<table><thead><tr><th>Name</th><th>Age</th></tr></thead>'
            . '<tbody><tr><td>Alice</td><td>30</td></tr></tbody></table>';
        $result = convert_with_tables($html);

        self::assertInstanceOf(TableExtractionResult::class, $result);
        self::assertNotEmpty($result->content);
        self::assertCount(1, $result->tables);

        $table = $result->tables[0];
        self::assertInstanceOf(TableData::class, $table);
        self::assertGreaterThanOrEqual(2, \count($table->cells));
        self::assertSame(['Name', 'Age'], $table->cells[0]);
        self::assertSame(['Alice', '30'], $table->cells[1]);
        self::assertNotEmpty($table->markdown);
        self::assertNotEmpty($table->isHeaderRow);
        self::assertTrue($table->isHeaderRow[0]);
    }

    public function testConvertWithTablesReturnsEmptyTablesForNonTableHtml(): void
    {
        $result = convert_with_tables('<p>Hello world</p>');

        self::assertInstanceOf(TableExtractionResult::class, $result);
        self::assertNotEmpty($result->content);
        self::assertCount(0, $result->tables);
    }

    public function testConvertWithTablesExtractsMultipleTables(): void
    {
        $html = '<table><tr><th>A</th></tr><tr><td>1</td></tr></table>'
            . '<p>text</p>'
            . '<table><tr><th>B</th></tr><tr><td>2</td></tr></table>';

        $result = convert_with_tables($html);

        self::assertCount(2, $result->tables);
    }

    public function testConvertWithTablesIncludesMetadata(): void
    {
        $html = '<html><head><title>Test</title></head>'
            . '<body><table><tr><th>Col</th></tr><tr><td>Val</td></tr></table></body></html>';

        $result = convert_with_tables($html);

        self::assertNotNull($result->metadata);
        self::assertSame('Test', $result->metadata->document->title);
    }

    public function testConvertWithTablesContentIncludesTableText(): void
    {
        $html = '<table><tr><th>Header</th></tr><tr><td>Value</td></tr></table>';
        $result = convert_with_tables($html);

        self::assertStringContainsString('Header', $result->content);
        self::assertStringContainsString('Value', $result->content);
    }

    public function testConvertWithTablesHandlesSpecialCharacters(): void
    {
        $html = '<table><tr><td>a &amp; b</td><td>c &lt; d</td></tr></table>';
        $result = convert_with_tables($html);

        self::assertCount(1, $result->tables);
        $table = $result->tables[0];
        self::assertSame('a & b', $table->cells[0][0]);
    }

    public function testConvertWithTablesAcceptsOptions(): void
    {
        $html = '<table><tr><th>H</th></tr><tr><td>V</td></tr></table>';
        $result = convert_with_tables($html, ['heading_style' => 'atx']);

        self::assertCount(1, $result->tables);
    }

    public function testConvertWithTablesViaFacade(): void
    {
        $html = '<table><tr><th>X</th></tr><tr><td>Y</td></tr></table>';
        $result = HtmlToMarkdown::convertWithTables($html);

        self::assertInstanceOf(TableExtractionResult::class, $result);
        self::assertCount(1, $result->tables);
    }

    public function testTableDataToArray(): void
    {
        $html = '<table><tr><th>A</th></tr><tr><td>1</td></tr></table>';
        $result = convert_with_tables($html);

        $array = $result->toArray();
        self::assertArrayHasKey('content', $array);
        self::assertArrayHasKey('tables', $array);
        self::assertArrayHasKey('metadata', $array);
        self::assertCount(1, $array['tables']);
        self::assertArrayHasKey('cells', $array['tables'][0]);
        self::assertArrayHasKey('markdown', $array['tables'][0]);
        self::assertArrayHasKey('is_header_row', $array['tables'][0]);
    }
}
