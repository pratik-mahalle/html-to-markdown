<?php

declare(strict_types=1);

namespace HtmlToMarkdown\Bridge;

use HtmlToMarkdown\Contract\ExtensionBridge as ExtensionBridgeContract;
use HtmlToMarkdown\Exception\ConversionFailed;
use HtmlToMarkdown\Exception\ExtensionNotLoaded;

/**
 * @phpstan-import-type ConversionOptionsInput from \HtmlToMarkdown\Config\ConversionOptions
 */

final class ExtensionBridge implements ExtensionBridgeContract
{
    private const CONVERT_FUNCTION = 'html_to_markdown_convert';

    /**
     * @param ConversionOptionsInput|null $options
     *
     * @return array<string, mixed>
     */
    public function convert(string $html, ?array $options = null): array
    {
        if (!\function_exists(self::CONVERT_FUNCTION)) {
            throw ExtensionNotLoaded::create();
        }

        try {
            return \html_to_markdown_convert($html, $options);
        } catch (\Throwable $exception) {
            throw ConversionFailed::withMessage($exception->getMessage());
        }
    }
}
