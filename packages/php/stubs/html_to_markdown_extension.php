<?php

declare(strict_types=1);

/**
 * Stub declarations for the html_to_markdown PHP extension functions.
 * These stubs allow PHPStan to analyze code that calls extension functions
 * that may not be available in the current PHP environment during analysis.
 */

/**
 * @param array<string, mixed>|null $options
 * @return array<string, mixed>
 */
function html_to_markdown_convert(string $html, ?array $options = null): array
{
    throw new \RuntimeException('html_to_markdown extension not loaded');
}
