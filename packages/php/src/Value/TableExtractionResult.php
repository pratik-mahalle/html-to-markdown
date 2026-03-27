<?php

declare(strict_types=1);

namespace HtmlToMarkdown\Value;

final readonly class TableExtractionResult
{
    /**
     * @param list<TableData> $tables
     */
    public function __construct(
        public string $content,
        public ?HtmlMetadata $metadata,
        public array $tables,
    ) {
    }

    /**
     * @param array<string, mixed> $payload
     */
    public static function fromExtensionPayload(array $payload): self
    {
        $contentValue = $payload['content'] ?? null;
        $content = \is_string($contentValue) ? $contentValue : '';

        $metadata = null;
        $metadataPayload = $payload['metadata'] ?? null;
        if (\is_array($metadataPayload)) {
            /** @var array<string, mixed> $metadataPayload */
            $metadata = HtmlMetadata::fromExtensionPayload($metadataPayload);
        }

        $tables = [];
        $rawTablesValue = $payload['tables'] ?? null;
        $rawTables = \is_array($rawTablesValue) ? $rawTablesValue : [];
        foreach ($rawTables as $table) {
            if (\is_array($table)) {
                /** @var array<string, mixed> $table */
                $tables[] = TableData::fromExtensionPayload($table);
            }
        }

        return new self(
            content: $content,
            metadata: $metadata,
            tables: $tables,
        );
    }

    /**
     * @return array{content: string, metadata: array<string, mixed>|null,
     *               tables: list<array{cells: list<list<string>>, markdown: string, is_header_row: list<bool>}>}
     */
    public function toArray(): array
    {
        return [
            'content' => $this->content,
            'metadata' => $this->metadata?->toArray(),
            'tables' => \array_map(
                static fn (TableData $t): array => $t->toArray(),
                $this->tables,
            ),
        ];
    }
}
