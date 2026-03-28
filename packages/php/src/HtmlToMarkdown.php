<?php

declare(strict_types=1);

namespace HtmlToMarkdown;

use HtmlToMarkdown\Config\ConversionOptions;
use HtmlToMarkdown\Service\Converter as ConverterService;

/**
 * @phpstan-import-type ConversionOptionsInput from \HtmlToMarkdown\Config\ConversionOptions
 */

final class HtmlToMarkdown
{
    /**
     * @param ConversionOptions|ConversionOptionsInput|null $options
     * @phpstan-param ConversionOptions|array<string, mixed>|null $options
     *
     * @return array<string, mixed>
     */
    public static function convert(string $html, ConversionOptions|array|null $options = null): array
    {
        return ConverterService::create()->convert($html, $options);
    }
}
