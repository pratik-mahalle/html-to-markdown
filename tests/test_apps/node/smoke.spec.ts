import { describe, it, expect, beforeAll } from 'vitest';
import type { JsConversionOptions, JsConversionResult } from '@kreuzberg/html-to-markdown';

describe('html-to-markdown smoke tests', () => {
  let convert: (html: string, options?: JsConversionOptions | null) => JsConversionResult;

  beforeAll(async () => {
    const module = await import('@kreuzberg/html-to-markdown');
    convert = module.convert;
  });

  describe('Package imports and basic functionality', () => {
    it('should import the package successfully', async () => {
      const htmlToMarkdown = await import('@kreuzberg/html-to-markdown');
      expect(htmlToMarkdown).toBeDefined();
      expect(typeof htmlToMarkdown.convert).toBe('function');
    });

    it('should convert basic HTML to Markdown', () => {
      const html = '<p>Hello World</p>';
      const result = convert(html);
      expect(result.content).toContain('Hello World');
      expect(typeof result.content).toBe('string');
    });

    it('should handle heading conversion', () => {
      const html = '<h1>Title</h1>';
      const result = convert(html);
      expect(result.content).toMatch(/^#\s+Title/);
    });

    it('should handle empty input gracefully', () => {
      const result = convert('');
      expect(result.content).toBe('');
    });

    it('should preserve text content through conversion', () => {
      const html = '<div><p>Line 1</p><p>Line 2</p></div>';
      const result = convert(html);
      expect(result.content).toContain('Line 1');
      expect(result.content).toContain('Line 2');
    });
  });

  describe('Error handling', () => {
    it('should handle malformed HTML gracefully', () => {
      const html = '<p>Unclosed paragraph';
      expect(() => convert(html)).not.toThrow();
      const result = convert(html);
      expect(typeof result.content).toBe('string');
    });

    it('should handle HTML with special characters', () => {
      const html = '<p>&lt;script&gt;alert("test")&lt;/script&gt;</p>';
      expect(() => convert(html)).not.toThrow();
      const result = convert(html);
      expect(result.content).toContain('script');
    });
  });

  describe('Result structure', () => {
    it('should return a result with content field', () => {
      const html = '<p>Test</p>';
      const result = convert(html);
      expect(result).toBeDefined();
      expect(result.content).toBeDefined();
      expect(typeof result.content).toBe('string');
    });

    it('should return a result with warnings field', () => {
      const html = '<p>Test</p>';
      const result = convert(html);
      expect(result.warnings).toBeDefined();
    });
  });

  describe('Options handling', () => {
    it('should accept conversion options', () => {
      const html = '<p>Test</p>';
      const options: JsConversionOptions = { hardBreaks: true };
      expect(() => convert(html, options)).not.toThrow();
      const result = convert(html, options);
      expect(typeof result.content).toBe('string');
    });

    it('should handle null options', () => {
      const html = '<p>Test</p>';
      expect(() => convert(html, null)).not.toThrow();
      const result = convert(html, null);
      expect(typeof result.content).toBe('string');
    });

    it('should handle undefined options', () => {
      const html = '<p>Test</p>';
      expect(() => convert(html, undefined)).not.toThrow();
      const result = convert(html, undefined);
      expect(typeof result.content).toBe('string');
    });
  });
});
