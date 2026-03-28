<?php

declare(strict_types=1);

namespace HtmlToMarkdown\Contract;

/**
 * @phpstan-import-type ConversionOptionsInput from \HtmlToMarkdown\Config\ConversionOptions
 */

interface ExtensionBridge
{
    /**
     * @param ConversionOptionsInput|null $options
     *
     * @return array<string, mixed>
     */
    public function convert(string $html, ?array $options = null): array;
}
