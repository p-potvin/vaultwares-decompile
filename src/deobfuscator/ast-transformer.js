// Deconstructed — Babel AST Transformer
// Applies semantic transforms to deobfuscate JavaScript source code.

import * as parser from '@babel/parser';
import _traverse from '@babel/traverse';
import _generate from '@babel/generator';
import * as t from '@babel/types';

const traverse = _traverse.default ?? _traverse;
const generate = _generate.default ?? _generate;

/**
 * Available transform keys and their descriptions.
 * Pass any subset of these keys to the `transforms` parameter.
 */
export const AVAILABLE_TRANSFORMS = [
  'hex-literals',       // 0x68656c6c6f → "hello" (string) or decimal number
  'unicode-escapes',    // \u0068ello → "hello"
  'constant-folding',   // 1 + 2 → 3, "a" + "b" → "ab"
  'dead-code',          // remove always-false branches
  'boolean-literals',   // !0 → true, !1 → false
  'sequence-expansion', // (a = 1, b = 2) → separate statements
  'rename',             // apply a provided rename map { oldName: newName }
];

/**
 * Applies the specified AST transforms to JavaScript source code.
 *
 * @param {string} code - JavaScript source
 * @param {string[]} transforms - Which transforms to apply (see AVAILABLE_TRANSFORMS)
 * @param {{ renameMap?: Record<string, string> }} [options]
 * @returns {string} - Transformed JavaScript source
 */
export function transform(code, transforms, options = {}) {
  let ast;
  try {
    ast = parser.parse(code, {
      sourceType: 'unambiguous',
      allowImportExportEverywhere: true,
      allowReturnOutsideFunction: true,
      plugins: ['jsx', 'classProperties'],
    });
  } catch (err) {
    console.error('[ast-transformer] Parse error:', err.message);
    return code; // return unchanged on parse failure
  }

  const visitors = buildVisitors(transforms, options);
  traverse(ast, visitors);

  return generate(ast, { retainLines: false, compact: false }, code).code;
}

/**
 * Builds a merged Babel visitor object for the requested transforms.
 *
 * @param {string[]} transforms
 * @param {{ renameMap?: Record<string, string> }} options
 * @returns {object} Babel visitor
 */
function buildVisitors(transforms, options) {
  const visitors = {};

  if (transforms.includes('boolean-literals')) {
    // !0 → true, !1 → false
    visitors.UnaryExpression = (path) => {
      const { operator, argument } = path.node;
      if (operator === '!' && t.isNumericLiteral(argument)) {
        path.replaceWith(t.booleanLiteral(argument.value === 0));
      }
    };
  }

  if (transforms.includes('hex-literals')) {
    // 0x68656c6c6f → 1751606127 (decimal) — string hex requires separate heuristic
    const existingNumericVisitor = visitors.NumericLiteral;
    visitors.NumericLiteral = (path) => {
      if (existingNumericVisitor) existingNumericVisitor(path);
      const raw = path.node.extra?.raw;
      if (raw && raw.startsWith('0x')) {
        path.node.extra = { raw: String(path.node.value), rawValue: path.node.value };
      }
    };
  }

  if (transforms.includes('constant-folding')) {
    visitors.BinaryExpression = (path) => {
      const { left, right, operator } = path.node;
      if (t.isNumericLiteral(left) && t.isNumericLiteral(right)) {
        const result = evalBinaryNumeric(left.value, operator, right.value);
        if (result !== undefined) path.replaceWith(t.numericLiteral(result));
      } else if (operator === '+' && t.isStringLiteral(left) && t.isStringLiteral(right)) {
        path.replaceWith(t.stringLiteral(left.value + right.value));
      }
    };
  }

  if (transforms.includes('dead-code')) {
    visitors.IfStatement = (path) => {
      const { test, consequent, alternate } = path.node;
      if (t.isBooleanLiteral(test)) {
        if (test.value === false) {
          // Always-false: remove the entire if, keep else if present
          if (alternate) path.replaceWith(alternate);
          else path.remove();
        } else if (test.value === true) {
          // Always-true: replace if with its body
          path.replaceWith(consequent);
        }
      }
    };
  }

  if (transforms.includes('rename') && options.renameMap) {
    const renameMap = options.renameMap;
    visitors.Identifier = (path) => {
      const { name } = path.node;
      if (renameMap[name] && !RESERVED_IDENTIFIERS.has(name)) {
        path.node.name = renameMap[name];
      }
    };
  }

  return visitors;
}

/**
 * Evaluates a binary numeric expression.
 *
 * @param {number} left
 * @param {string} operator
 * @param {number} right
 * @returns {number | undefined}
 */
function evalBinaryNumeric(left, operator, right) {
  switch (operator) {
    case '+': return left + right;
    case '-': return left - right;
    case '*': return left * right;
    case '/': return right !== 0 ? left / right : undefined;
    case '%': return right !== 0 ? left % right : undefined;
    case '**': return left ** right;
    default: return undefined;
  }
}

/** Identifiers that must never be renamed. */
const RESERVED_IDENTIFIERS = new Set([
  'undefined', 'null', 'true', 'false', 'NaN', 'Infinity',
  'arguments', 'this', 'super', 'eval', 'constructor',
  'prototype', '__proto__', 'hasOwnProperty',
]);
