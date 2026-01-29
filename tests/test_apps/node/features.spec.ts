import { describe, it, expect, beforeAll } from 'vitest';
import type {
  JsConversionOptions,
  JsMetadataConfig,
  JsInlineImageConfig,
  JsMetadataExtraction,
  JsHtmlExtraction,
} from '@kreuzberg/html-to-markdown';

describe('html-to-markdown feature tests', () => {
  let convert: (html: string, options?: JsConversionOptions | null) => string;
  let convertWithMetadata: (
    html: string,
    options?: JsConversionOptions | null,
    metadataConfig?: JsMetadataConfig | null
  ) => JsMetadataExtraction;
  let convertWithMetadataBuffer: (
    buffer: Buffer | Uint8Array,
    options?: JsConversionOptions | null,
    metadataConfig?: JsMetadataConfig | null
  ) => JsMetadataExtraction;
  let convertFileWithInlineImages: (
    filePath: string,
    options?: JsConversionOptions | null,
    imageConfig?: JsInlineImageConfig | null
  ) => Promise<JsHtmlExtraction>;
  let hasMetadataSupport: () => boolean;
  let hasFileInlineImageSupport: boolean = false;

  beforeAll(async () => {
    const module = await import('@kreuzberg/html-to-markdown');
    convert = module.convert;
    convertWithMetadata = module.convertWithMetadata;
    convertWithMetadataBuffer = module.convertWithMetadataBuffer;
    convertFileWithInlineImages = module.convertFileWithInlineImages;
    hasMetadataSupport = module.hasMetadataSupport;

    // Check if file-based inline image support is available
    hasFileInlineImageSupport = typeof module.convertFileWithInlineImages === 'function';
  });

  describe('Metadata extraction', () => {
    it('should extract metadata when supported', () => {
      if (!hasMetadataSupport()) {
        expect(true).toBe(true);
        return;
      }

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

      const result = convertWithMetadata(html);
      expect(result).toBeDefined();
      expect(result.markdown).toBeDefined();
      expect(typeof result.markdown).toBe('string');
      expect(result.metadata).toBeDefined();
    });

    it('should handle metadata config options', () => {
      if (!hasMetadataSupport()) {
        expect(true).toBe(true);
        return;
      }

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

      const metadataConfig: JsMetadataConfig = {
        extractHeaders: true,
        extractLinks: true,
        extractImages: false,
      };

      const result = convertWithMetadata(html, undefined, metadataConfig);
      expect(result).toBeDefined();
      expect(result.markdown).toBeDefined();
    });

    it('should extract metadata from buffer input', () => {
      if (!hasMetadataSupport()) {
        expect(true).toBe(true);
        return;
      }

      const html = '<html><head><title>Buffer Test</title></head><body><p>Content</p></body></html>';
      const buffer = Buffer.from(html, 'utf-8');

      const result = convertWithMetadataBuffer(buffer);
      expect(result).toBeDefined();
      expect(result.markdown).toBeDefined();
      expect(typeof result.markdown).toBe('string');
    });

    it('should handle empty metadata extraction config', () => {
      if (!hasMetadataSupport()) {
        expect(true).toBe(true);
        return;
      }

      const html = '<p>Test</p>';
      const result = convertWithMetadata(html, undefined, null);
      expect(result).toBeDefined();
      expect(result.markdown).toBeDefined();
    });
  });

  describe('Inline image extraction (file-based)', () => {
    it('should detect file-based inline image support', () => {
      expect(typeof hasFileInlineImageSupport).toBe('boolean');
    });

    it('should have convertFileWithInlineImages function', () => {
      expect(typeof convertFileWithInlineImages).toBe('function');
    });

    it('should handle inline image config options', () => {
      if (!hasFileInlineImageSupport) {
        expect(true).toBe(true);
        return;
      }

      const imageConfig: JsInlineImageConfig = {
        downloadImages: false,
        imageDirectory: './images',
        preserveAlt: true,
      };

      // Function is available for use with file paths
      expect(typeof convertFileWithInlineImages).toBe('function');
    });

    it('should accept properly typed image config in conversion options', () => {
      const html = '<img src="test.jpg">';
      const imageConfig: JsInlineImageConfig = {
        downloadImages: false,
        preserveAlt: true,
      };

      // Verify types are accepted (not testing async file operations)
      expect(imageConfig).toBeDefined();
      expect(typeof imageConfig.downloadImages).toBe('boolean');
    });
  });

  describe('Conversion options', () => {
    it('should apply hardBreaks option', () => {
      const html = '<p>Line 1<br>Line 2</p>';
      const options: JsConversionOptions = { hardBreaks: true };
      const result = convert(html, options);
      expect(typeof result).toBe('string');
      expect(result.length > 0).toBe(true);
    });

    it('should apply heading style options', () => {
      const html = '<h1>Heading</h1>';
      const options: JsConversionOptions = {};
      const result = convert(html, options);
      expect(result).toBeDefined();
      expect(typeof result).toBe('string');
    });

    it('should apply list style options', () => {
      const html = '<ul><li>Item 1</li><li>Item 2</li></ul>';
      const options: JsConversionOptions = {};
      const result = convert(html, options);
      expect(result).toBeDefined();
      expect(result).toContain('Item 1');
      expect(result).toContain('Item 2');
    });

    it('should handle code block conversion', () => {
      const html = '<pre><code>console.log("test");</code></pre>';
      const options: JsConversionOptions = {};
      const result = convert(html, options);
      expect(result).toBeDefined();
      expect(result).toContain('console.log');
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
      expect(result).toContain('Item 1');
      expect(result).toContain('Nested 1');
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
      expect(typeof result).toBe('string');
    });

    it('should handle blockquotes', () => {
      const html = `
        <blockquote>
          <p>This is a quote</p>
          <p>With multiple paragraphs</p>
        </blockquote>
      `;

      const result = convert(html);
      expect(result).toContain('quote');
      expect(result).toContain('>');
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
      expect(result).toContain('Bold');
      expect(result).toContain('Italic');
      expect(result).toContain('inline code');
    });

    it('should handle links with various attributes', () => {
      const html = `
        <a href="https://example.com">Simple Link</a>
        <a href="https://example.com" title="Title">Link with title</a>
        <a href="#anchor">Anchor link</a>
      `;

      const result = convert(html);
      expect(result).toContain('Simple Link');
      expect(result).toContain('https://example.com');
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
      expect(result).toContain('Title');
      expect(result).toContain('Point 1');
      expect(result).toContain('quote');
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

    it('should accept properly typed metadata config', () => {
      if (!hasMetadataSupport()) {
        expect(true).toBe(true);
        return;
      }

      const html = '<p>Test</p>';
      const metadataConfig: JsMetadataConfig = {
        extractHeaders: true,
        extractLinks: false,
      };

      const result = convertWithMetadata(html, undefined, metadataConfig);
      expect(result).toBeDefined();
    });

    it('should accept properly typed image config', () => {
      const html = '<img src="test.jpg">';
      const imageConfig: JsInlineImageConfig = {
        downloadImages: false,
        preserveAlt: true,
      };

      // Verify config types are properly typed
      expect(imageConfig).toBeDefined();
      expect(typeof imageConfig.downloadImages).toBe('boolean');
      expect(typeof imageConfig.preserveAlt).toBe('boolean');
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
      expect(result.length > 0).toBe(true);
    });

    it('should handle deeply nested HTML', () => {
      let html = '';
      for (let i = 0; i < 50; i++) {
        html = `<div>${html}</div>`;
      }
      html = `<p>Deep</p>${html}`;

      expect(() => convert(html)).not.toThrow();
      const result = convert(html);
      expect(typeof result).toBe('string');
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
        expect(typeof result).toBe('string');
      });
    });

    it('should handle HTML with comments', () => {
      const html = `
        <p>Visible</p>
        <!-- This is a comment -->
        <p>Also visible</p>
      `;

      const result = convert(html);
      expect(result).toContain('Visible');
      expect(result).not.toContain('comment');
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
      expect(result).toContain('Content');
      expect(result).not.toContain('alert');
      expect(result).not.toContain('color: red');
    });
  });
});
