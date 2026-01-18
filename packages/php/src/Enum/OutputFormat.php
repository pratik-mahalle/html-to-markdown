<?php

declare(strict_types=1);

namespace HtmlToMarkdown\Enum;

use HtmlToMarkdown\Exception\InvalidOption;

enum OutputFormat: string
{
    case MARKDOWN = 'markdown';
    case DJOT = 'djot';

    public static function fromString(string $value): self
    {
        try {
            return self::from($value);
        } catch (\ValueError) {
            throw InvalidOption::forEnum('output_format', $value, self::cases());
        }
    }
}
