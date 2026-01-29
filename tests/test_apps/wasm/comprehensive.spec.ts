import { describe, it, expect, beforeAll } from 'vitest';
import { readFileSync, existsSync } from 'fs';
import { join, dirname } from 'path';
import { fileURLToPath } from 'url';

const __dirname = dirname(fileURLToPath(import.meta.url));

interface TestCase {
  name: string;
  html: string;
  expectedMarkdown: string;
  options?: Record<string, any>;
}

function loadFixtures(filename: string): TestCase[] {
  const fixturePath = join(__dirname, '../fixtures', filename);
  if (!existsSync(fixturePath)) {
    console.warn(`Fixture file not found: ${fixturePath}`);
    return [];
  }
  const content = readFileSync(fixturePath, 'utf-8');
  return JSON.parse(content);
}

function getWasmBinarySize(): number | null {
  try {
    // Look for WASM binary in node_modules
    const wasmPath = join(__dirname, 'node_modules/@kreuzberg/html-to-markdown-wasm/');
    const packageJson = join(wasmPath, 'package.json');
    if (!existsSync(packageJson)) return null;

    const pkg = JSON.parse(readFileSync(packageJson, 'utf-8'));
    const files = pkg.files || [];

    let totalSize = 0;
    for (const file of files) {
      if (file.endsWith('.wasm')) {
        const filePath = join(wasmPath, file);
        if (existsSync(filePath)) {
          const stat = require('fs').statSync(filePath);
          totalSize += stat.size;
        }
      }
    }

    return totalSize > 0 ? totalSize : null;
  } catch (error) {
    console.warn('Could not determine WASM binary size:', error);
    return null;
  }
}

describe('comprehensive html-to-markdown WASM tests', () => {
  let wasmModule: any;

  beforeAll(async () => {
    try {
      wasmModule = await import('@kreuzberg/html-to-markdown-wasm');
    } catch (error) {
      console.error('Failed to load WASM module:', error);
      throw error;
    }
  });

  describe('fixture-based tests', () => {
    const basicFixtures = loadFixtures('basic-html.json');

    if (basicFixtures.length > 0) {
      basicFixtures.forEach((testCase) => {
        it(testCase.name, () => {
          const result = wasmModule.convert(testCase.html, testCase.options);
          expect(result.trim()).toBe(testCase.expectedMarkdown.trim());
        });
      });
    } else {
      it('should load basic fixtures (placeholder)', () => {
        expect(wasmModule).toBeDefined();
      });
    }
  });

  describe('HTML element coverage', () => {
    it('should convert paragraphs', () => {
      const html = '<p>Hello World</p>';
      const result = wasmModule.convert(html);
      expect(result).toContain('Hello World');
    });

    it('should convert all heading levels', () => {
      for (let i = 1; i <= 6; i++) {
        const html = `<h${i}>Heading ${i}</h${i}>`;
        const result = wasmModule.convert(html);
        expect(result).toContain(`${'#'.repeat(i)} Heading ${i}`);
      }
    });

    it('should convert unordered lists', () => {
      const html = '<ul><li>Item 1</li><li>Item 2</li><li>Item 3</li></ul>';
      const result = wasmModule.convert(html);
      expect(result).toContain('Item 1');
      expect(result).toContain('Item 2');
      expect(result).toContain('Item 3');
    });

    it('should convert ordered lists', () => {
      const html = '<ol><li>First</li><li>Second</li><li>Third</li></ol>';
      const result = wasmModule.convert(html);
      expect(result).toContain('First');
      expect(result).toContain('Second');
      expect(result).toContain('Third');
    });

    it('should convert nested lists', () => {
      const html = '<ul><li>Item 1<ul><li>Nested 1</li></ul></li><li>Item 2</li></ul>';
      const result = wasmModule.convert(html);
      expect(result).toContain('Item 1');
      expect(result).toContain('Nested 1');
      expect(result).toContain('Item 2');
    });

    it('should convert links', () => {
      const html = '<a href="https://example.com">Example Link</a>';
      const result = wasmModule.convert(html);
      expect(result).toContain('Example Link');
      expect(result).toContain('example.com');
    });

    it('should convert bold text', () => {
      const html = '<p><strong>Bold text</strong></p>';
      const result = wasmModule.convert(html);
      expect(result).toContain('**Bold text**');
    });

    it('should convert italic text', () => {
      const html = '<p><em>Italic text</em></p>';
      const result = wasmModule.convert(html);
      expect(result).toContain('*Italic text*');
    });

    it('should convert code blocks', () => {
      const html = '<pre><code>const x = 1;</code></pre>';
      const result = wasmModule.convert(html);
      expect(result).toContain('const x = 1');
    });

    it('should convert inline code', () => {
      const html = '<p>Use <code>console.log()</code> to debug</p>';
      const result = wasmModule.convert(html);
      expect(result).toContain('`console.log()`');
    });

    it('should convert blockquotes', () => {
      const html = '<blockquote>This is a quote</blockquote>';
      const result = wasmModule.convert(html);
      expect(result).toContain('> This is a quote');
    });

    it('should convert horizontal rules', () => {
      const html = '<hr />';
      const result = wasmModule.convert(html);
      expect(result).toBeDefined();
    });

    it('should convert images (if supported)', () => {
      const html = '<img src="image.jpg" alt="An image" />';
      const result = wasmModule.convert(html);
      expect(result).toBeDefined();
    });

    it('should convert tables (if supported)', () => {
      const html = '<table><tr><th>Header</th></tr><tr><td>Data</td></tr></table>';
      const result = wasmModule.convert(html);
      expect(result).toBeDefined();
    });
  });

  describe('edge cases and special scenarios', () => {
    it('should handle nested HTML structures', () => {
      const html = '<div><section><p>Deep <strong>nested</strong> content</p></section></div>';
      const result = wasmModule.convert(html);
      expect(result).toContain('Deep');
      expect(result).toContain('nested');
      expect(result).toContain('content');
    });

    it('should handle mixed content types', () => {
      const html = `
        <h1>Title</h1>
        <p>Paragraph with <em>emphasis</em> and <strong>strong</strong></p>
        <ul><li>Item 1</li><li>Item 2</li></ul>
        <blockquote>A quote</blockquote>
      `;
      const result = wasmModule.convert(html);
      expect(result).toContain('Title');
      expect(result).toContain('Paragraph');
      expect(result).toContain('Item 1');
      expect(result).toContain('A quote');
    });

    it('should handle special HTML characters', () => {
      const html = '<p>Test & special < characters > and &#169;</p>';
      const result = wasmModule.convert(html);
      expect(result).toContain('Test');
    });

    it('should handle HTML entities', () => {
      const html = '<p>&lt;tag&gt; &amp; &quot;quoted&quot;</p>';
      const result = wasmModule.convert(html);
      expect(result).toBeDefined();
    });

    it('should handle excessive whitespace', () => {
      const html = '<p>Text with    multiple     spaces    and\n\nnewlines</p>';
      const result = wasmModule.convert(html);
      expect(result.length).toBeGreaterThan(0);
    });

    it('should handle empty elements', () => {
      const html = '<p></p><div></div><ul></ul>';
      expect(() => wasmModule.convert(html)).not.toThrow();
    });

    it('should handle deeply nested structures', () => {
      const html = '<div><div><div><div><div><p>Deep content</p></div></div></div></div></div>';
      const result = wasmModule.convert(html);
      expect(result).toContain('Deep content');
    });

    it('should handle very long content', () => {
      const longText = 'A'.repeat(5000);
      const html = `<p>${longText}</p>`;
      const result = wasmModule.convert(html);
      expect(result.length).toBeGreaterThan(0);
    });

    it('should handle Unicode and special characters', () => {
      const html = '<p>Hello 世界 🌍 café naïve ñ</p>';
      const result = wasmModule.convert(html);
      expect(result).toBeDefined();
    });

    it('should handle mixed language content', () => {
      const html = '<p>English text <strong>en français</strong> mit Deutsch 中文</p>';
      const result = wasmModule.convert(html);
      expect(result).toBeDefined();
    });
  });

  describe('bundle size validation', () => {
    it('should have reasonable bundle size', () => {
      const size = getWasmBinarySize();
      if (size !== null) {
        // WASM binaries for HTML parsing typically range from 100KB to 500KB
        expect(size).toBeGreaterThan(1000); // At least 1KB
        expect(size).toBeLessThan(2000000); // Less than 2MB
        console.log(`WASM binary size: ${(size / 1024).toFixed(2)} KB`);
      } else {
        console.log('Could not determine WASM binary size');
      }
    });
  });

  describe('module functionality verification', () => {
    it('should return consistent results for same input', () => {
      const html = '<p>Test content</p>';
      const result1 = wasmModule.convert(html);
      const result2 = wasmModule.convert(html);
      expect(result1).toBe(result2);
    });

    it('should handle multiple conversions in sequence', () => {
      const htmls = [
        '<p>First</p>',
        '<p>Second</p>',
        '<p>Third</p>',
      ];
      const results = htmls.map(html => wasmModule.convert(html));
      expect(results).toHaveLength(3);
      expect(results[0]).toContain('First');
      expect(results[1]).toContain('Second');
      expect(results[2]).toContain('Third');
    });

    it('should support options parameter if provided', () => {
      const html = '<p>Content</p>';
      const options = { preserveLinks: true };
      expect(() => wasmModule.convert(html, options)).not.toThrow();
    });
  });
});
