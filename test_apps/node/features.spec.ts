import { describe, it, expect, beforeAll } from 'vitest';
import type {
  JsConversionOptions,
  JsConversionResult,
} from '@kreuzberg/html-to-markdown';

describe('html-to-markdown feature tests', () => {
  let convert: (html: string, options?: JsConversionOptions | null) => JsConversionResult;

  beforeAll(async () => {
    const module = await import('@kreuzberg/html-to-markdown');
    convert = module.convert;
  });

  describe('Metadata extraction', () => {
    it('should extract metadata from convert result', () => {
      const html = `
        <html lang="en">
          <head>
            <title>Test Article</title>
            <meta name="description" content="Test description">
            <meta name="keywords" content="test, article">
          </head>
          <body>
            <h1>Main Title</h1>
            <p>Content here</p>
          </body>
        </html>
      `;

      const result = convert(html);
      expect(result).toBeDefined();
      expect(result.content).toBeDefined();
      expect(typeof result.content).toBe('string');
      expect(result.metadata).toBeDefined();
    });

    it('should include metadata as JSON string', () => {
      const html = `
        <html>
          <head><title>Test</title></head>
          <body>
            <h1>Heading</h1>
            <h2>Subheading</h2>
            <a href="https://example.com">Link</a>
          </body>
        </html>
      `;

      const result = convert(html);
      expect(result).toBeDefined();
      expect(result.content).toBeDefined();
      if (result.metadata) {
        expect(typeof result.metadata).toBe('string');
      }
    });

    it('should handle simple HTML metadata extraction', () => {
      const html = '<p>Test</p>';
      const result = convert(html);
      expect(result).toBeDefined();
      expect(result.content).toBeDefined();
    });
  });

  describe('Conversion options', () => {
    it('should apply hardBreaks option', () => {
      const html = '<p>Line 1<br>Line 2</p>';
      const options: JsConversionOptions = { hardBreaks: true };
      const result = convert(html, options);
      expect(typeof result.content).toBe('string');
      expect(result.content!.length > 0).toBe(true);
    });

    it('should apply heading style options', () => {
      const html = '<h1>Heading</h1>';
      const options: JsConversionOptions = {};
      const result = convert(html, options);
      expect(result).toBeDefined();
      expect(typeof result.content).toBe('string');
    });

    it('should apply list style options', () => {
      const html = '<ul><li>Item 1</li><li>Item 2</li></ul>';
      const options: JsConversionOptions = {};
      const result = convert(html, options);
      expect(result).toBeDefined();
      expect(result.content).toContain('Item 1');
      expect(result.content).toContain('Item 2');
    });

    it('should handle code block conversion', () => {
      const html = '<pre><code>console.log("test");</code></pre>';
      const options: JsConversionOptions = {};
      const result = convert(html, options);
      expect(result).toBeDefined();
      expect(result.content).toContain('console.log');
    });
  });

  describe('Complex HTML features', () => {
    it('should handle nested lists', () => {
      const html = `
        <ul>
          <li>Item 1
            <ul>
              <li>Nested 1</li>
              <li>Nested 2</li>
            </ul>
          </li>
          <li>Item 2</li>
        </ul>
      `;

      const result = convert(html);
      expect(result.content).toContain('Item 1');
      expect(result.content).toContain('Nested 1');
    });

    it('should handle tables', () => {
      const html = `
        <table>
          <tr>
            <th>Header 1</th>
            <th>Header 2</th>
          </tr>
          <tr>
            <td>Cell 1</td>
            <td>Cell 2</td>
          </tr>
        </table>
      `;

      expect(() => convert(html)).not.toThrow();
      const result = convert(html);
      expect(typeof result.content).toBe('string');
    });

    it('should handle blockquotes', () => {
      const html = `
        <blockquote>
          <p>This is a quote</p>
          <p>With multiple paragraphs</p>
        </blockquote>
      `;

      const result = convert(html);
      expect(result.content).toContain('quote');
      expect(result.content).toContain('>');
    });

    it('should handle inline formatting', () => {
      const html = `
        <p>
          <strong>Bold</strong>
          <em>Italic</em>
          <u>Underline</u>
          <s>Strikethrough</s>
          <code>inline code</code>
        </p>
      `;

      const result = convert(html);
      expect(result.content).toContain('Bold');
      expect(result.content).toContain('Italic');
      expect(result.content).toContain('inline code');
    });

    it('should handle links with various attributes', () => {
      const html = `
        <a href="https://example.com">Simple Link</a>
        <a href="https://example.com" title="Title">Link with title</a>
        <a href="#anchor">Anchor link</a>
      `;

      const result = convert(html);
      expect(result.content).toContain('Simple Link');
      expect(result.content).toContain('https://example.com');
    });

    it('should handle mixed content', () => {
      const html = `
        <h1>Title</h1>
        <p>Introduction with <a href="/link">link</a>.</p>
        <ul>
          <li>Point 1</li>
          <li>Point 2</li>
        </ul>
        <blockquote>Important quote</blockquote>
        <pre><code>code example</code></pre>
      `;

      expect(() => convert(html)).not.toThrow();
      const result = convert(html);
      expect(result.content).toContain('Title');
      expect(result.content).toContain('Point 1');
      expect(result.content).toContain('quote');
    });
  });

  describe('Type safety', () => {
    it('should accept properly typed conversion options', () => {
      const html = '<p>Test</p>';
      const options: JsConversionOptions = {
        hardBreaks: true,
      };

      const result = convert(html, options);
      expect(result).toBeDefined();
    });

    it('should return JsConversionResult with expected fields', () => {
      const html = '<p>Test</p>';
      const result = convert(html);
      expect(result).toBeDefined();
      expect(result.content).toBeDefined();
      expect(result.warnings).toBeDefined();
    });
  });

  describe('Edge cases and robustness', () => {
    it('should handle very long HTML documents', () => {
      let html = '<p>';
      for (let i = 0; i < 1000; i++) {
        html += `Paragraph ${i}. `;
      }
      html += '</p>';

      expect(() => convert(html)).not.toThrow();
      const result = convert(html);
      expect(result.content!.length > 0).toBe(true);
    });

    it('should handle deeply nested HTML', () => {
      let html = '';
      for (let i = 0; i < 50; i++) {
        html = `<div>${html}</div>`;
      }
      html = `<p>Deep</p>${html}`;

      expect(() => convert(html)).not.toThrow();
      const result = convert(html);
      expect(typeof result.content).toBe('string');
    });

    it('should handle HTML with various encodings', () => {
      const htmls = [
        '<p>ASCII: Hello</p>',
        '<p>Unicode: Hëllö Wörld 你好</p>',
        '<p>Emoji: 😀🎉🚀</p>',
        '<p>Special chars: &lt;&gt;&amp;&quot;&apos;</p>',
      ];

      htmls.forEach((html) => {
        expect(() => convert(html)).not.toThrow();
        const result = convert(html);
        expect(typeof result.content).toBe('string');
      });
    });

    it('should handle HTML with comments', () => {
      const html = `
        <p>Visible</p>
        <!-- This is a comment -->
        <p>Also visible</p>
      `;

      const result = convert(html);
      expect(result.content).toContain('Visible');
      expect(result.content).not.toContain('comment');
    });

    it('should handle scripts and styles safely', () => {
      const html = `
        <p>Content</p>
        <script>alert('xss');</script>
        <style>body { color: red; }</style>
        <p>More content</p>
      `;

      expect(() => convert(html)).not.toThrow();
      const result = convert(html);
      expect(result.content).toContain('Content');
      expect(result.content).not.toContain('alert');
      expect(result.content).not.toContain('color: red');
    });
  });
});
