<?php

declare(strict_types=1);

namespace HtmlToMarkdown\Value;

final readonly class TableData
{
    /**
     * @param list<list<string>> $cells
     * @param list<bool> $isHeaderRow
     */
    public function __construct(
        public array $cells,
        public string $markdown,
        public array $isHeaderRow,
    ) {
    }

    /**
     * @param array<string, mixed> $payload
     */
    public static function fromExtensionPayload(array $payload): self
    {
        $cells = [];
        $rawCellsValue = $payload['cells'] ?? null;
        $rawCells = \is_array($rawCellsValue) ? $rawCellsValue : [];
        foreach ($rawCells as $row) {
            if (\is_array($row)) {
                $cells[] = \array_values(\array_map('\strval', $row));
            }
        }

        $markdownValue = $payload['markdown'] ?? null;
        $markdown = \is_string($markdownValue) ? $markdownValue : '';

        $isHeaderRow = [];
        $rawHeaderRowValue = $payload['is_header_row'] ?? null;
        $rawHeaderRow = \is_array($rawHeaderRowValue) ? $rawHeaderRowValue : [];
        foreach ($rawHeaderRow as $flag) {
            $isHeaderRow[] = (bool) $flag;
        }

        return new self(
            cells: $cells,
            markdown: $markdown,
            isHeaderRow: $isHeaderRow,
        );
    }

    /**
     * @return array{cells: list<list<string>>, markdown: string, is_header_row: list<bool>}
     */
    public function toArray(): array
    {
        return [
            'cells' => $this->cells,
            'markdown' => $this->markdown,
            'is_header_row' => $this->isHeaderRow,
        ];
    }
}
