<?php

declare(strict_types=1);

namespace HtmlToMarkdown;

use HtmlToMarkdown\Config\ConversionOptions;

/**
 * @phpstan-import-type ConversionOptionsInput from \HtmlToMarkdown\Config\ConversionOptions
 */

/**
 * @param ConversionOptions|array<string, mixed>|null $options
 * @phpstan-param ConversionOptions|array<string, mixed>|null $options
 *
 * @return array<string, mixed>
 */
function convert(string $html, ConversionOptions|array|null $options = null): array
{
    return HtmlToMarkdown::convert($html, $options);
}
