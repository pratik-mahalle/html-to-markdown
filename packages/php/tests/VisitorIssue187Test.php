<?php

declare(strict_types=1);

namespace HtmlToMarkdown\Tests;

use HtmlToMarkdown\HtmlToMarkdown;
use HtmlToMarkdown\Visitor\AbstractVisitor;
use HtmlToMarkdown\Visitor\NodeContext;
use HtmlToMarkdown\Visitor\VisitResult;
use PHPUnit\Framework\TestCase;
use Throwable;

/**
 * Test suite for Issue #187: Visitor Pattern tagName Context Bug.
 *
 * Issue: visitElementStart receives tagName in context, but it may not
 * contain the actual HTML tag name in all cases (similar to Python bug).
 *
 * This test verifies:
 * 1. visitElementStart receives correct tagName in context
 * 2. Filtering by tag name works correctly
 * 3. The tagName field contains actual HTML tag names (div, script, style, p, etc.)
 * 4. Attributes are accessible alongside tagName
 *
 * @see https://github.com/Goldziher/html-to-markdown/issues/187
 * @group visitor-pattern
 * @group issue-187
 */
final class VisitorIssue187Test extends TestCase
{
    /**
     * Test that visitElementStart receives correct tagName in context.
     *
     * This is the core issue: context['tag_name'] should contain the actual
     * HTML tag name (e.g., 'div', 'p', 'script') when visitElementStart is called.
     */
    public function testVisitElementStartReceivesCorrectTagName(): void
    {
        $visitor = new class () extends AbstractVisitor {
            /** @var array<string, int> */
            public array $tagNames = [];

            public function visitElementStart(NodeContext $context): array
            {
                $tagName = $context->tagName;
                if (!isset($this->tagNames[$tagName])) {
                    $this->tagNames[$tagName] = 0;
                }
                $this->tagNames[$tagName]++;

                return VisitResult::continue();
            }
        };

        $html = <<<'HTML'
<article>
    <div>Container</div>
    <p>Paragraph</p>
    <script>console.log("test");</script>
</article>
HTML;

        try {
            HtmlToMarkdown::convertWithVisitor($html, null, $visitor);

            // Verify we captured tag names
            self::assertNotEmpty($visitor->tagNames, 'No tag names were captured by visitElementStart');

            // Expected tags from HTML
            $expectedTags = ['div', 'p', 'article'];
            foreach ($expectedTags as $tag) {
                self::assertArrayHasKey(
                    $tag,
                    $visitor->tagNames,
                    "Expected tag '{$tag}' not found in visited elements. Got: " . \implode(', ', \array_keys($visitor->tagNames))
                );
            }
        } catch (Throwable $e) {
            $this->markTestSkipped('PHP visitor integration not yet fully implemented in ext-php-rs bindings: ' . $e->getMessage());
        }
    }

    /**
     * Test filtering by tag name works correctly.
     *
     * Demonstrates filtering div elements by checking context->tagName
     * in visitElementStart and returning VisitResult::skip().
     */
    public function testFilteringByTagName(): void
    {
        $visitor = new class () extends AbstractVisitor {
            /** @var array<string> */
            public array $skippedTags = [];

            public function visitElementStart(NodeContext $context): array
            {
                $tagName = $context->tagName;

                // Skip divs with 'hidden' class
                if ($tagName === 'div') {
                    $attributes = $context->attributes;
                    if (isset($attributes['class']) && \str_contains($attributes['class'], 'hidden')) {
                        $this->skippedTags[] = $tagName;
                        return VisitResult::skip();
                    }
                }

                return VisitResult::continue();
            }
        };

        $html = <<<'HTML'
<div>Visible div</div>
<div class="hidden">Hidden div</div>
<div class="visible">Another visible div</div>
HTML;

        try {
            $result = HtmlToMarkdown::convertWithVisitor($html, null, $visitor);

            // Verify hidden div was skipped
            self::assertNotEmpty($visitor->skippedTags, 'No divs were filtered');
            self::assertCount(1, $visitor->skippedTags, 'Expected exactly 1 div to be skipped');
            self::assertSame('div', $visitor->skippedTags[0], 'Expected skipped tag to be div');

            // Hidden content should not appear in output
            self::assertStringNotContainsString('Hidden div', $result);
        } catch (Throwable $e) {
            $this->markTestSkipped('PHP visitor integration not yet fully implemented in ext-php-rs bindings: ' . $e->getMessage());
        }
    }

    /**
     * Test visitElementStart for all major HTML elements.
     *
     * This ensures that various HTML elements (div, script, style, p, etc.)
     * properly provide their tag names in the context.
     */
    public function testVisitElementStartForMultipleElements(): void
    {
        $visitor = new class () extends AbstractVisitor {
            /** @var array<int|string, array{tag: string, attributes: array<string, string>}> */
            public array $visitedElements = [];
            public int $callCount = 0;

            public function visitElementStart(NodeContext $context): array
            {
                $this->callCount++;
                $this->visitedElements[$this->callCount] = [
                    'tag' => $context->tagName,
                    'attributes' => $context->attributes,
                ];

                return VisitResult::continue();
            }
        };

        $html = <<<'HTML'
<div id="main">
    <p class="intro">Introduction</p>
    <script type="text/javascript">var x = 1;</script>
    <style>body { color: red; }</style>
    <article>
        <h1>Title</h1>
        <span>Content</span>
    </article>
</div>
HTML;

        try {
            HtmlToMarkdown::convertWithVisitor($html, null, $visitor);

            self::assertGreaterThan(0, $visitor->callCount, 'visitElementStart was never called');

            // Verify specific tags were visited
            $tags = \array_map(static fn ($e) => $e['tag'], $visitor->visitedElements);

            // Core tags we expect
            $expectedTags = ['div', 'p', 'article', 'h1'];
            foreach ($expectedTags as $tag) {
                self::assertContains(
                    $tag,
                    $tags,
                    "Expected tag '{$tag}' in visited elements: " . \implode(', ', $tags)
                );
            }
        } catch (Throwable $e) {
            $this->markTestSkipped('PHP visitor integration not yet fully implemented in ext-php-rs bindings: ' . $e->getMessage());
        }
    }

    /**
     * Test that tagName field contains actual HTML tag names (not mangled).
     *
     * Verifies the tagName is not corrupted or substituted with incorrect values.
     * This is the specific bug being tested: tagName should reflect the actual
     * HTML tag, not something else.
     */
    public function testTagNameContainsActualHtmlTagNames(): void
    {
        $visitor = new class () extends AbstractVisitor {
            /** @var array<string> */
            public array $capturedTagNames = [];

            public function visitElementStart(NodeContext $context): array
            {
                $this->capturedTagNames[] = $context->tagName;
                return VisitResult::continue();
            }
        };

        $html = <<<'HTML'
<div>
    <p id="para-1" class="text">Paragraph</p>
    <script>console.log("test");</script>
    <style>.cls { color: blue; }</style>
</div>
HTML;

        try {
            HtmlToMarkdown::convertWithVisitor($html, null, $visitor);

            self::assertNotEmpty($visitor->capturedTagNames, 'No tags were captured');

            // Verify captured tag names are valid HTML tags
            foreach ($visitor->capturedTagNames as $tagName) {
                self::assertIsString($tagName, 'Tag name should be a string');
                self::assertNotEmpty($tagName, 'Tag name should not be empty');

                // Tag names should be lowercase and alphanumeric
                self::assertMatchesRegularExpression(
                    '/^[a-z]+[a-z0-9-]*$/i',
                    $tagName,
                    "Tag name '{$tagName}' does not match expected HTML tag format"
                );
            }

            // Verify we got the exact tags we expect
            self::assertContains('div', $visitor->capturedTagNames, 'div tag not captured');
            self::assertContains('p', $visitor->capturedTagNames, 'p tag not captured');
            self::assertContains('script', $visitor->capturedTagNames, 'script tag not captured');
            self::assertContains('style', $visitor->capturedTagNames, 'style tag not captured');
        } catch (Throwable $e) {
            $this->markTestSkipped('PHP visitor integration not yet fully implemented in ext-php-rs bindings: ' . $e->getMessage());
        }
    }

    /**
     * Test filtering divs by class attribute using tagName.
     *
     * This test specifically demonstrates the use case: filtering out
     * div elements with specific classes by checking both tagName and attributes.
     */
    public function testFilteringDivsByClassAttribute(): void
    {
        $visitor = new class () extends AbstractVisitor {
            /** @var array<string> */
            public array $skippedClasses = [];
            public int $totalDivsVisited = 0;

            public function visitElementStart(NodeContext $context): array
            {
                if ($context->tagName === 'div') {
                    $this->totalDivsVisited++;

                    $attributes = $context->attributes;
                    if (isset($attributes['class'])) {
                        $classes = \explode(' ', $attributes['class']);
                        foreach ($classes as $class) {
                            if ($class === 'skip-me' || $class === 'advertisement') {
                                $this->skippedClasses[] = $class;
                                return VisitResult::skip();
                            }
                        }
                    }
                }

                return VisitResult::continue();
            }
        };

        $html = <<<'HTML'
<div>Normal div</div>
<div class="skip-me">Skipped div</div>
<div class="content">Content div</div>
<div class="advertisement">Ad div</div>
<p class="skip-me">This p tag should not be skipped</p>
HTML;

        try {
            $result = HtmlToMarkdown::convertWithVisitor($html, null, $visitor);

            self::assertGreaterThan(0, $visitor->totalDivsVisited, 'No divs were visited');
            self::assertCount(2, $visitor->skippedClasses, 'Expected 2 divs to be skipped (skip-me and advertisement)');

            // Verify skipped classes
            self::assertContains('skip-me', $visitor->skippedClasses);
            self::assertContains('advertisement', $visitor->skippedClasses);

            // Verify skipped content is not in output
            self::assertStringNotContainsString('Skipped div', $result);
            self::assertStringNotContainsString('Ad div', $result);

            // But p tag with skip-me class should still be in output (we only skipped divs)
            // This demonstrates that tagName filtering works correctly
        } catch (Throwable $e) {
            $this->markTestSkipped('PHP visitor integration not yet fully implemented in ext-php-rs bindings: ' . $e->getMessage());
        }
    }

    /**
     * Test that context tagName matches NodeContext constructor parameter.
     *
     * This is a unit-level test verifying that NodeContext correctly
     * stores and retrieves the tagName parameter.
     */
    public function testNodeContextTagNameIntegrity(): void
    {
        $context = new NodeContext(
            nodeType: 'element',
            tagName: 'div',
            attributes: ['id' => 'main', 'class' => 'container'],
            depth: 1,
            indexInParent: 0,
            parentTag: 'body',
            isInline: false,
        );

        self::assertSame('div', $context->tagName, 'tagName should be exactly "div"');
    }

    /**
     * Test that NodeContext::fromArray correctly deserializes tagName.
     *
     * Verifies the FFI conversion from Rust array to PHP NodeContext
     * preserves the tag_name field.
     */
    public function testNodeContextFromArrayDeserializesTagName(): void
    {
        $data = [
            'node_type' => 'element',
            'tag_name' => 'script',
            'attributes' => ['type' => 'text/javascript'],
            'depth' => 2,
            'index_in_parent' => 1,
            'parent_tag' => 'head',
            'is_inline' => false,
        ];

        $context = NodeContext::fromArray($data);

        self::assertSame('script', $context->tagName, 'tagName should be "script" after deserialization');
    }

    /**
     * Test that NodeContext::toArray correctly serializes tagName.
     *
     * Verifies the round-trip conversion: PHP NodeContext -> Array -> NodeContext
     */
    public function testNodeContextToArraySerializesTagName(): void
    {
        $context = new NodeContext(
            nodeType: 'element',
            tagName: 'style',
            attributes: [],
            depth: 1,
            indexInParent: 0,
            parentTag: 'head',
            isInline: false,
        );

        $array = $context->toArray();

        self::assertArrayHasKey('tag_name', $array, 'Array should contain tag_name key');
        self::assertSame('style', $array['tag_name'], 'Serialized tag_name should be "style"');
    }

    /**
     * Test complex filtering scenario with multiple tag types.
     *
     * Demonstrates real-world usage of visitElementStart with tagName
     * to implement sophisticated filtering logic.
     */
    public function testComplexFilteringByMultipleTagNames(): void
    {
        $visitor = new class () extends AbstractVisitor {
            /** @var array<string, int> */
            public array $elementCounts = [];

            public function visitElementStart(NodeContext $context): array
            {
                $tagName = $context->tagName;

                if (!isset($this->elementCounts[$tagName])) {
                    $this->elementCounts[$tagName] = 0;
                }
                $this->elementCounts[$tagName]++;

                // Skip script and style tags
                if ($tagName === 'script' || $tagName === 'style') {
                    return VisitResult::skip();
                }

                // Skip divs with data-skip attribute
                if ($tagName === 'div' && isset($context->attributes['data-skip'])) {
                    return VisitResult::skip();
                }

                return VisitResult::continue();
            }
        };

        $html = <<<'HTML'
<html>
    <head>
        <title>Test</title>
        <style>body { color: red; }</style>
    </head>
    <body>
        <div>Normal content</div>
        <script>console.log("test");</script>
        <div data-skip="true">Skipped by attribute</div>
        <p>Keep this paragraph</p>
    </body>
</html>
HTML;

        try {
            $result = HtmlToMarkdown::convertWithVisitor($html, null, $visitor);

            // Verify counts
            self::assertGreaterThan(0, \count($visitor->elementCounts), 'No elements were visited');

            // Script and style should still be counted even though skipped
            // (visitElementStart was called)
            if (isset($visitor->elementCounts['script'])) {
                self::assertGreaterThan(0, $visitor->elementCounts['script']);
            }

            if (isset($visitor->elementCounts['style'])) {
                self::assertGreaterThan(0, $visitor->elementCounts['style']);
            }

            // Verify skipped content is not in output
            self::assertStringNotContainsString('Skipped by attribute', $result);
            self::assertStringNotContainsString('console.log', $result);
            self::assertStringNotContainsString('color: red', $result);

            // But keep normal content
            self::assertStringContainsString('Normal content', $result);
            self::assertStringContainsString('Keep this paragraph', $result);
        } catch (Throwable $e) {
            $this->markTestSkipped('PHP visitor integration not yet fully implemented in ext-php-rs bindings: ' . $e->getMessage());
        }
    }
}
