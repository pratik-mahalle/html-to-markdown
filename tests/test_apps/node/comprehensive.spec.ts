import { describe, it, expect, beforeAll } from 'vitest';
import { readFileSync } from 'fs';
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
  const content = readFileSync(fixturePath, 'utf-8');
  const fixtures = JSON.parse(content);
  return Array.isArray(fixtures) ? fixtures : [];
}

describe('comprehensive html-to-markdown tests', () => {
  let convert: any;

  beforeAll(async () => {
    const module = await import('@kreuzberg/html-to-markdown');
    convert = module.convert;
  });

  describe('Basic HTML fixtures', () => {
    const basicFixtures = loadFixtures('basic-html.json');

    basicFixtures.forEach((testCase) => {
      it(testCase.name, () => {
        const result = convert(testCase.html, testCase.options);
        expect(result.trim()).toBe(testCase.expectedMarkdown.trim());
      });
    });
  });

  describe('Complex HTML fixtures', () => {
    const complexFixtures = loadFixtures('complex-html.json');

    complexFixtures.forEach((testCase) => {
      it(testCase.name, () => {
        const result = convert(testCase.html, testCase.options);
        expect(result.trim()).toBe(testCase.expectedMarkdown.trim());
      });
    });
  });

  describe('Edge cases', () => {
    const edgeCaseFixtures = loadFixtures('edge-cases.json');

    edgeCaseFixtures.forEach((testCase) => {
      it(testCase.name, () => {
        const result = convert(testCase.html, testCase.options);
        expect(result.trim()).toBe(testCase.expectedMarkdown.trim());
      });
    });
  });

  describe('Real-world HTML', () => {
    const realWorldFixtures = loadFixtures('real-world.json');

    realWorldFixtures.forEach((testCase) => {
      it(testCase.name, () => {
        const result = convert(testCase.html, testCase.options);
        expect(result.trim()).toBe(testCase.expectedMarkdown.trim());
      });
    });
  });
});
