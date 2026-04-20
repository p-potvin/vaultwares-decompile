// Unit tests for the AI prompt builder (prompt-builder.js)

import { buildRenamePrompt, parseRenameResponse } from '../ai/prompt-builder.js';

describe('buildRenamePrompt', () => {
  test('returns a non-empty string', () => {
    const prompt = buildRenamePrompt('var a = 1;', ['a']);
    expect(typeof prompt).toBe('string');
    expect(prompt.length).toBeGreaterThan(50);
  });

  test('includes the code chunk in the prompt', () => {
    const code = 'function e(t) { return t * 2; }';
    const prompt = buildRenamePrompt(code, ['e', 't']);
    expect(prompt).toContain(code);
  });

  test('includes all identifiers in the prompt', () => {
    const prompt = buildRenamePrompt('var a, b, c;', ['a', 'b', 'c']);
    expect(prompt).toContain('a');
    expect(prompt).toContain('b');
    expect(prompt).toContain('c');
  });

  test('includes the JSON output instruction', () => {
    const prompt = buildRenamePrompt('var x = 1;', ['x']);
    expect(prompt.toLowerCase()).toContain('json');
  });
});

describe('parseRenameResponse', () => {
  test('parses a valid JSON response', () => {
    const response = JSON.stringify({
      renames: [
        { original: 'a', proposed: 'userToken', confidence: 0.9 },
        { original: 'b', proposed: 'userId', confidence: 0.85 },
      ],
    });
    const result = parseRenameResponse(response);
    expect(result.length).toBe(2);
    expect(result[0].original).toBe('a');
    expect(result[0].proposed).toBe('userToken');
    expect(result[0].confidence).toBe(0.9);
  });

  test('returns empty array for invalid JSON', () => {
    expect(parseRenameResponse('not json at all')).toEqual([]);
  });

  test('strips markdown code fences', () => {
    const response = '```json\n{"renames":[{"original":"x","proposed":"count","confidence":0.8}]}\n```';
    const result = parseRenameResponse(response);
    expect(result.length).toBe(1);
    expect(result[0].proposed).toBe('count');
  });

  test('filters out malformed entries', () => {
    const response = JSON.stringify({
      renames: [
        { original: 'a', proposed: 'valid', confidence: 0.9 },
        { original: 'b' }, // missing proposed and confidence
        { original: 'c', proposed: 'also-valid', confidence: 0.7 },
      ],
    });
    const result = parseRenameResponse(response);
    expect(result.length).toBe(2);
  });

  test('returns empty array when renames is not an array', () => {
    expect(parseRenameResponse('{"renames": "not-array"}')).toEqual([]);
  });
});
