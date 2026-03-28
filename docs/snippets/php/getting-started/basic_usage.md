```php
use HtmlToMarkdown\Service\Converter;
use function HtmlToMarkdown\convert;

// Object-oriented usage
$converter = Converter::create();
$result = $converter->convert('<h1>Hello</h1><p>This is <strong>fast</strong>!</p>');
$markdown = $result['content'];

// Procedural helper
$result = convert('<h1>Hello</h1>');
$markdown = $result['content'];
```
