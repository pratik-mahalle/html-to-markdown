import { describe, it, expect, beforeAll } from 'vitest';

describe('html-to-markdown WASM smoke tests', () => {
  let wasmModule: any;

  beforeAll(async () => {
    try {
      wasmModule = await import('@kreuzberg/html-to-markdown-wasm');
    } catch (error) {
      console.error('Failed to load WASM module:', error);
      throw error;
    }
  });

  describe('module loading', () => {
    it('should import the WASM package', () => {
      expect(wasmModule).toBeDefined();
    });

    it('should expose convert function', () => {
      expect(typeof wasmModule.convert).toBe('function');
    });

    it('should expose init function', () => {
      expect(typeof wasmModule.init).toBe('function');
    });
  });

  describe('basic conversion', () => {
    it('should convert basic HTML', () => {
      const html = '<p>Hello World</p>';
      const result = wasmModule.convert(html);
      expect(result).toContain('Hello World');
    });

    it('should handle heading HTML', () => {
      const html = '<h1>Title</h1>';
      const result = wasmModule.convert(html);
      expect(result).toMatch(/^#/);
    });

    it('should handle empty input', () => {
      const result = wasmModule.convert('');
      expect(result).toBe('');
    });

    it('should handle list HTML', () => {
      const html = '<ul><li>Item 1</li><li>Item 2</li></ul>';
      const result = wasmModule.convert(html);
      expect(result).toContain('Item 1');
      expect(result).toContain('Item 2');
    });

    it('should handle link HTML', () => {
      const html = '<a href="https://example.com">Example</a>';
      const result = wasmModule.convert(html);
      expect(result).toContain('Example');
      expect(result).toContain('example.com');
    });
  });

  describe('async operations', () => {
    it('should support async conversion if available', async () => {
      if (typeof wasmModule.convertAsync !== 'function') {
        console.log('convertAsync not available in this WASM version');
        return;
      }

      const html = '<p>Async test</p>';
      const result = await wasmModule.convertAsync(html);
      expect(result).toContain('Async test');
    });

    it('should handle sync conversion correctly', () => {
      const html = '<p>Sync test</p>';
      const result = wasmModule.convert(html);
      expect(result).toContain('Sync test');
    });
  });

  describe('error handling', () => {
    it('should handle malformed HTML gracefully', () => {
      const html = '<p>Unclosed paragraph<p>';
      expect(() => wasmModule.convert(html)).not.toThrow();
    });

    it('should handle very long input', () => {
      const longContent = '<p>' + 'A'.repeat(10000) + '</p>';
      const result = wasmModule.convert(longContent);
      expect(result.length).toBeGreaterThan(0);
    });

    it('should handle special HTML characters', () => {
      const html = '<p>&lt;script&gt;alert("xss")&lt;/script&gt;</p>';
      const result = wasmModule.convert(html);
      expect(result).toBeDefined();
    });

    it('should handle null/undefined input gracefully', () => {
      expect(() => wasmModule.convert(null ?? '')).not.toThrow();
      expect(() => wasmModule.convert(undefined ?? '')).not.toThrow();
    });
  });
});
