```php
use HtmlToMarkdown\Visitor\AbstractVisitor;
use HtmlToMarkdown\Visitor\NodeContext;
use HtmlToMarkdown\Visitor\VisitResult;
use HtmlToMarkdown\Config\ConversionOptions;
use HtmlToMarkdown\Service\Converter;

class CustomVisitor extends AbstractVisitor
{
    public function visitImage(NodeContext $context, string $src, string $alt, ?string $title): array
    {
        // Skip all images
        return VisitResult::skip();
    }

    public function visitLink(NodeContext $context, string $href, string $text, ?string $title): array
    {
        // Custom link handling
        return VisitResult::custom("[{$text}]({$href})");
    }
}

$converter = Converter::create();
$result = $converter->convert(
    '<a href="/page">Link</a><img src="pic.png" alt="pic">',
    new ConversionOptions(visitor: new CustomVisitor())
);
$markdown = $result['content'];
```
