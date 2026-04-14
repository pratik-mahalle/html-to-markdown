<?php

declare(strict_types=1);

namespace {

    use Html\To\Markdown\Rs\HtmlToMarkdownRs;

    if (!\function_exists('html_to_markdown_convert')) {
        /**
         * Convert HTML to Markdown and return the content string.
         *
         * Delegates to the native Rust extension via the HtmlToMarkdownRs facade.
         * Options are not currently supported in this convenience wrapper — call
         * HtmlToMarkdownRs::convert() directly for full control.
         *
         * @param string               $html    The HTML string to convert.
         * @param array<string, mixed> $options Reserved for future use.
         *
         * @throws \Html\To\Markdown\Rs\HtmlToMarkdownRsException on conversion error.
         */
        function html_to_markdown_convert(string $html, array $options = []): string
        {
            $result = HtmlToMarkdownRs::convert($html, null);

            return $result->getContent() ?? '';
        }
    }

} // end namespace
