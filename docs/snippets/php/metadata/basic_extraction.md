```php
use HtmlToMarkdown\Config\ConversionOptions;
use HtmlToMarkdown\Service\Converter;

$html = '<html><head><title>Example</title></head><body><h1>Welcome</h1><a href="https://example.com">Link</a></body></html>';

$converter = Converter::create();
$result = $converter->convert(
    $html,
    new ConversionOptions(
        headingStyle: 'Atx',
        extractMetadata: true,
        extractHeaders: true,
        extractLinks: true,
        extractImages: true,
    )
);

echo $result['content'];
echo $result['metadata']->document->title;
foreach ($result['metadata']->links as $link) {
    echo $link->href . ': ' . $link->text;
}
```
