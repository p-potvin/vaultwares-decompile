/**
 * @typedef {Object} WebpackModule
 * @property {string|number} id
 * @property {string} code
 * @property {string[]} dependencies - module IDs this module requires
 */

export { beautify } from './beautifier.js';
export { transform } from './ast-transformer.js';
export { splitWebpackBundle } from './module-splitter.js';
