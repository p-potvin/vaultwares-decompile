// VaultWares Decompile — Webpack Bundle Splitter
// Extracts individual modules from a webpack IIFE bundle.

import * as parser from '@babel/parser';
import _traverse from '@babel/traverse';
import _generate from '@babel/generator';
import * as t from '@babel/types';

const traverse = _traverse.default ?? _traverse;
const generate = _generate.default ?? _generate;

/**
 * Detects whether the given code looks like a webpack bundle.
 *
 * @param {string} code
 * @returns {boolean}
 */
export function isWebpackBundle(code) {
  return (
    /\/\*\*\* \d+/m.test(code) ||                      // webpack v4 module comment
    /\bwebpackJsonp\b/.test(code) ||                    // webpack v3 runtime
    /\bwebpackChunkName\b/.test(code) ||                // named chunks
    /\(function\(modules\)\s*\{/.test(code) ||          // webpack v4 factory IIFE
    /\(__webpack_require__\s*=/.test(code)              // webpack require bootstrap
  );
}

/**
 * Splits a webpack bundle back into individual module objects.
 *
 * @param {string} code - A webpack bundle (v4 or v5 IIFE format)
 * @returns {WebpackModule[]}
 */
export function splitWebpackBundle(code) {
  let ast;
  try {
    ast = parser.parse(code, {
      sourceType: 'unambiguous',
      allowImportExportEverywhere: true,
      allowReturnOutsideFunction: true,
    });
  } catch {
    return [];
  }

  const modules = [];

  traverse(ast, {
    // webpack v4: modules are properties of the object passed to the factory IIFE
    // Pattern: (function(modules) { ... })({ 0: function(module, exports, __webpack_require__) { ... }, ... })
    CallExpression(path) {
      const { callee, arguments: args } = path.node;

      if (
        t.isFunctionExpression(callee) &&
        args.length === 1 &&
        (t.isObjectExpression(args[0]) || t.isArrayExpression(args[0]))
      ) {
        const moduleContainer = args[0];

        if (t.isObjectExpression(moduleContainer)) {
          for (const prop of moduleContainer.properties) {
            if (!t.isObjectProperty(prop) || !t.isFunctionExpression(prop.value)) continue;
            const moduleId = t.isNumericLiteral(prop.key)
              ? prop.key.value
              : t.isStringLiteral(prop.key)
              ? prop.key.value
              : String(prop.key.name || '');

            const dependencies = extractWebpackDependencies(prop.value);
            const moduleCode = generate(prop.value, {}, code).code;

            modules.push({ id: moduleId, code: moduleCode, dependencies });
          }
        } else if (t.isArrayExpression(moduleContainer)) {
          moduleContainer.elements.forEach((el, index) => {
            if (!el || !t.isFunctionExpression(el)) return;
            const dependencies = extractWebpackDependencies(el);
            const moduleCode = generate(el, {}, code).code;
            modules.push({ id: index, code: moduleCode, dependencies });
          });
        }

        // Stop traversal once the factory is found
        path.stop();
      }
    },
  });

  return modules;
}

/**
 * Extracts the module IDs required by a webpack module function.
 *
 * @param {import('@babel/types').FunctionExpression} funcNode
 * @returns {string[]}
 */
function extractWebpackDependencies(funcNode) {
  const deps = new Set();
  traverse(funcNode, {
    CallExpression(innerPath) {
      const { callee, arguments: args } = innerPath.node;
      if (
        (t.isIdentifier(callee, { name: '__webpack_require__' }) ||
          t.isIdentifier(callee, { name: 'r' })) &&
        args.length === 1 &&
        (t.isNumericLiteral(args[0]) || t.isStringLiteral(args[0]))
      ) {
        deps.add(String(args[0].value));
      }
    },
  });
  return [...deps];
}

/**
 * @typedef {Object} WebpackModule
 * @property {string|number} id
 * @property {string} code
 * @property {string[]} dependencies
 */
