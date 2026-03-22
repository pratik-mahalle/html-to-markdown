<?php

declare(strict_types=1);

/**
 * Stub declarations for the html_to_markdown PHP extension functions.
 * These stubs allow PHPStan to analyze code that calls extension functions
 * that may not be available in the current PHP environment during analysis.
 */

/**
 * @param array<string, mixed>|null $options
 */
function html_to_markdown_convert(string $html, ?array $options = null): string
{
    throw new \RuntimeException('html_to_markdown extension not loaded');
}

/**
 * @param array<string, mixed>|null $options
 * @param array<string, mixed>|null $config
 * @return array<string, mixed>
 */
function html_to_markdown_convert_with_inline_images(
    string $html,
    ?array $options = null,
    ?array $config = null,
): array {
    throw new \RuntimeException('html_to_markdown extension not loaded');
}

/**
 * @param array<string, mixed>|null $options
 * @param array<string, mixed>|null $metadataConfig
 * @return array<string, mixed>
 */
function html_to_markdown_convert_with_metadata(
    string $html,
    ?array $options = null,
    ?array $metadataConfig = null,
): array {
    throw new \RuntimeException('html_to_markdown extension not loaded');
}

/**
 * @param array<string, mixed>|null $options
 */
function html_to_markdown_convert_with_visitor(
    string $html,
    ?array $options = null,
    mixed $visitor = null,
): string {
    throw new \RuntimeException('html_to_markdown extension not loaded');
}

/**
 * @param array<string, mixed>|null $options
 * @param array<string, mixed>|null $metadataConfig
 * @return array<string, mixed>
 */
function html_to_markdown_convert_with_tables(
    string $html,
    ?array $options = null,
    ?array $metadataConfig = null,
): array {
    throw new \RuntimeException('html_to_markdown extension not loaded');
}
