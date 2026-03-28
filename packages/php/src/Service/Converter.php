<?php

declare(strict_types=1);

namespace HtmlToMarkdown\Service;

use HtmlToMarkdown\Bridge\ExtensionBridge;
use HtmlToMarkdown\Config\ConversionOptions;
use HtmlToMarkdown\Contract\ExtensionBridge as ExtensionBridgeContract;

/**
 * @phpstan-import-type ConversionOptionsInput from \HtmlToMarkdown\Config\ConversionOptions
 */

final class Converter
{
    public function __construct(
        private readonly ExtensionBridgeContract $bridge,
    ) {
    }

    public static function create(): self
    {
        return new self(new ExtensionBridge());
    }

    /**
     * @param ConversionOptions|ConversionOptionsInput|null $options
     * @phpstan-param ConversionOptions|array<string, mixed>|null $options
     *
     * @return array<string, mixed>
     */
    public function convert(string $html, ConversionOptions|array|null $options = null): array
    {
        return $this->bridge->convert($html, $this->normalizeOptions($options));
    }

    /**
     * @phpstan-param ConversionOptions|array<string, mixed>|null $options
     * @phpstan-return ConversionOptionsInput|null
     */
    private function normalizeOptions(ConversionOptions|array|null $options): ?array
    {
        if ($options === null) {
            return null;
        }

        if ($options instanceof ConversionOptions) {
            $payload = $options->toArray();
        } else {
            $payload = ConversionOptions::fromArray($options)->toArray();
        }

        return $payload === [] ? null : $payload;
    }
}
