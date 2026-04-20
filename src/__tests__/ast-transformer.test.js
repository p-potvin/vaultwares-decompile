// Unit tests for the Babel AST transformer (ast-transformer.js)

import { transform } from '../deobfuscator/ast-transformer.js';

describe('transform — boolean-literals', () => {
  test('replaces !0 with true', () => {
    const result = transform('var x = !0;', ['boolean-literals']);
    expect(result).toContain('true');
    expect(result).not.toContain('!0');
  });

  test('replaces !1 with false', () => {
    const result = transform('var x = !1;', ['boolean-literals']);
    expect(result).toContain('false');
    expect(result).not.toContain('!1');
  });
});

describe('transform — constant-folding', () => {
  test('folds numeric addition', () => {
    const result = transform('var x = 1 + 2;', ['constant-folding']);
    expect(result).toContain('3');
    expect(result).not.toContain('1 + 2');
  });

  test('folds string concatenation', () => {
    const result = transform('var x = "hello" + " world";', ['constant-folding']);
    expect(result).toContain('"hello world"');
  });

  test('folds numeric subtraction', () => {
    const result = transform('var x = 10 - 3;', ['constant-folding']);
    expect(result).toContain('7');
  });
});

describe('transform — rename', () => {
  test('renames identifiers according to rename map', () => {
    const code = 'function a(b) { return b + 1; }';
    const result = transform(code, ['rename'], { renameMap: { a: 'incrementBy1', b: 'value' } });
    expect(result).toContain('incrementBy1');
    expect(result).toContain('value');
    expect(result).not.toContain('function a(');
  });

  test('does not rename reserved identifiers', () => {
    const code = 'var x = undefined;';
    const result = transform(code, ['rename'], { renameMap: { undefined: 'renamed' } });
    expect(result).toContain('undefined');
    expect(result).not.toContain('renamed');
  });
});

describe('transform — error handling', () => {
  test('returns original code on parse error', () => {
    const invalid = 'this is not {{{ valid javascript';
    const result = transform(invalid, ['boolean-literals']);
    expect(result).toBe(invalid);
  });
});

describe('transform — multiple transforms', () => {
  test('applies boolean-literals and constant-folding together', () => {
    const code = 'var a = !0; var b = 2 + 3;';
    const result = transform(code, ['boolean-literals', 'constant-folding']);
    expect(result).toContain('true');
    expect(result).toContain('5');
    expect(result).not.toContain('!0');
    expect(result).not.toContain('2 + 3');
  });
});
