<?php

declare(strict_types=1);

namespace HtmlToMarkdown\Examples;

use HtmlToMarkdown\Converter;

/**
 * Content Filtering Example.
 *
 * Demonstrates how to remove unwanted elements like ads, tracking pixels,
 * and specific element types during conversion.
 */
class ContentFilter {
    private int $filtered = 0;

    public function visitElementStart(array $ctx): array {
        // Remove divs with class="ad" or class="tracking"
        if ($ctx['tag_name'] === 'div') {
            $classes = $ctx['attributes']['class'] ?? '';
            if (str_contains($classes, 'ad') || str_contains($classes, 'tracking')) {
                $this->filtered++;
                return ['type' => 'skip'];
            }
        }
        return ['type' => 'continue'];
    }

    public function visitImage(array $ctx, string $src, ?string $alt, ?string $title): array {
        // Remove tracking pixels (1x1 images)
        $width = $ctx['attributes']['width'] ?? null;
        $height = $ctx['attributes']['height'] ?? null;

        if ($width === '1' && $height === '1') {
            $this->filtered++;
            return ['type' => 'skip'];
        }

        return ['type' => 'continue'];
    }

    public function getFilterCount(): int {
        return $this->filtered;
    }
}

// Example usage
$html = <<<HTML
    <h1>Article with Ads</h1>
    <p>Main content here.</p>
    <div class="ad">
        <h3>Advertisement</h3>
        <p>Buy our product!</p>
    </div>
    <p>More content.</p>
    <img src="tracking.gif" alt="" width="1" height="1">
    <div class="tracking">
        <script>analytics_code();</script>
    </div>
    <p>Final paragraph.</p>
    HTML;

$visitor = new ContentFilter();
$markdown = Converter::convertWithVisitor($html, $visitor);

echo "Markdown output:\n";
echo $markdown;
echo "\n\nFiltered elements: {$visitor->getFilterCount()}\n";
