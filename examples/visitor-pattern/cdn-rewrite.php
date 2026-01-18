<?php

declare(strict_types=1);

namespace HtmlToMarkdown\Examples;

use HtmlToMarkdown\Converter;

/**
 * CDN URL Rewriting Example.
 *
 * Demonstrates how to rewrite image and link URLs to use a new CDN domain.
 * Useful for content migration, multi-CDN strategies, or URL standardization.
 */
readonly class CdnRewriter {
    public function __construct(
        private string $oldCdn,
        private string $newCdn,
        private int $rewrites = 0,
    ) {}

    public function visitImage(array $ctx, string $src, ?string $alt, ?string $title): array {
        if (str_starts_with($src, $this->oldCdn)) {
            $newSrc = str_replace($this->oldCdn, $this->newCdn, $src, 1);
            return ['type' => 'custom', 'output' => "![{$alt}]({$newSrc})"];
        }
        return ['type' => 'continue'];
    }

    public function visitLink(array $ctx, string $href, string $text, ?string $title): array {
        if (str_starts_with($href, $this->oldCdn)) {
            $newHref = str_replace($this->oldCdn, $this->newCdn, $href, 1);
            return ['type' => 'custom', 'output' => "[{$text}]({$newHref})"];
        }
        return ['type' => 'continue'];
    }
}

// Example usage
$html = <<<HTML
    <h1>Content Migration Example</h1>
    <p>We're migrating from our old CDN to a new one.</p>
    <img src="https://old-cdn.example.com/images/hero.jpg" alt="Hero image" width="800">
    <p>Download our <a href="https://old-cdn.example.com/files/guide.pdf">guide</a>.</p>
    <p>External link: <a href="https://other.com/page">Other site</a></p>
    <img src="https://other-cdn.com/image.png" alt="Other CDN">
    HTML;

$visitor = new CdnRewriter(
    oldCdn: 'https://old-cdn.example.com',
    newCdn: 'https://new-cdn.example.com',
);

$markdown = Converter::convertWithVisitor($html, $visitor);
echo $markdown;
