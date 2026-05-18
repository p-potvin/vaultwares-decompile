// VaultWares Decompile — JavaScript Beautifier
// First pass: normalise indentation and formatting using js-beautify + prettier.

import beautifyLib from 'js-beautify';
import prettier from 'prettier';

const { js: jsBeautify } = beautifyLib;

/**
 * Beautifies raw minified JavaScript through a two-pass pipeline:
 *  1. js-beautify — fixes indentation and line breaks
 *  2. prettier    — enforces consistent code style
 *
 * On parse failure, returns the js-beautify output (best effort).
 *
 * @param {string} code - Raw JavaScript (may be minified)
 * @returns {Promise<string>} - Formatted JavaScript
 */
export async function beautify(code) {
  // Pass 1: js-beautify (handles heavily minified code better)
  const pass1 = jsBeautify(code, {
    indent_size: 2,
    indent_char: ' ',
    max_preserve_newlines: 2,
    preserve_newlines: true,
    keep_array_indentation: false,
    break_chained_methods: false,
    indent_scripts: 'normal',
    brace_style: 'collapse',
    space_before_conditional: true,
    unescape_strings: true,
    jslint_happy: false,
    end_with_newline: true,
    wrap_line_length: 0,
    indent_empty_lines: false,
  });

  // Pass 2: prettier (enforces consistent quotes, semicolons, trailing commas)
  try {
    const pass2 = await prettier.format(pass1, {
      parser: 'babel',
      semi: true,
      singleQuote: true,
      trailingComma: 'es5',
      printWidth: 100,
      tabWidth: 2,
    });
    return pass2;
  } catch {
    // Prettier may fail on edge-case syntax; fall back to js-beautify output
    return pass1;
  }
}
