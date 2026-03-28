```php
use HtmlToMarkdown\Config\ConversionOptions;
use HtmlToMarkdown\Service\Converter;

$html = <<<HTML
<table>
    <tr><th>Name</th><th>Age</th></tr>
    <tr><td>Alice</td><td>30</td></tr>
    <tr><td>Bob</td><td>25</td></tr>
</table>
HTML;

$converter = Converter::create();
$result = $converter->convert($html, new ConversionOptions(extractTables: true));

foreach ($result['tables'] as $table) {
    foreach ($table->cells as $i => $row) {
        $prefix = $table->isHeaderRow[$i] ? 'Header' : 'Row';
        echo "  {$prefix}: " . implode(', ', $row) . "\n";
    }
}
```
