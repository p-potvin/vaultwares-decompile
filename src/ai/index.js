/**
 * @typedef {Object} RenameProgress
 * @property {number} chunkIndex
 * @property {number} totalChunks
 * @property {RenameProposal[]} proposals
 */

/**
 * @typedef {Object} RenameResult
 * @property {string} renamedCode
 * @property {RenameProposal[]} proposals
 */

/**
 * @typedef {Object} RenameProposal
 * @property {string} original
 * @property {string} proposed
 * @property {number} confidence  - 0.0 to 1.0
 */

export { detectBackend, generate } from './local-model.js';
export { buildRenamePrompt } from './prompt-builder.js';
export { renameIdentifiers } from './renamer-agent.js';
