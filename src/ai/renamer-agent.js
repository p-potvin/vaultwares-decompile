// Deconstructed — AI Rename Agent
// Orchestrates chunked AI analysis and applies rename proposals via Babel.

import * as parser from '@babel/parser';
import _traverse from '@babel/traverse';
import * as t from '@babel/types';
import { generate as aiGenerate, detectBackend } from './local-model.js';
import { buildRenamePrompt, parseRenameResponse } from './prompt-builder.js';
import { transform } from '../deobfuscator/ast-transformer.js';

const traverse = _traverse.default ?? _traverse;

const CHUNK_TOKEN_LIMIT = 4000;
// Rough heuristic: 1 token ≈ 4 characters for JavaScript code
const CHARS_PER_TOKEN = 4;
const MAX_CHUNK_CHARS = CHUNK_TOKEN_LIMIT * CHARS_PER_TOKEN;

const RESERVED_IDENTIFIERS = new Set([
  'undefined', 'null', 'true', 'false', 'NaN', 'Infinity',
  'arguments', 'this', 'super', 'eval', 'constructor',
  'prototype', '__proto__', 'hasOwnProperty',
]);

/** Identifiers shorter than this threshold are candidates for renaming */
const SHORT_NAME_THRESHOLD = 3;

/**
 * Renames obfuscated identifiers in a JavaScript file using a local AI model.
 *
 * @param {string} code - Full beautified JS source
 * @param {{ model: string, onProgress?: (p: RenameProgress) => void }} options
 * @returns {Promise<RenameResult>}
 */
export async function renameIdentifiers(code, options = {}) {
  const { model, onProgress = () => {} } = options;

  const backend = await detectBackend();
  if (!backend.available) {
    throw new Error(
      'No local AI backend is available. Start Ollama or llama.cpp to use AI renaming.'
    );
  }

  const selectedModel = model || backend.models[0];
  if (!selectedModel) {
    throw new Error('No AI models found. Pull a model with: ollama pull llama3');
  }

  // Split code into chunks preserving scope boundaries
  const chunks = splitIntoChunks(code);
  const allProposals = [];

  for (let i = 0; i < chunks.length; i++) {
    const chunk = chunks[i];
    const identifiers = extractShortIdentifiers(chunk);

    if (identifiers.length === 0) continue;

    const prompt = buildRenamePrompt(chunk, identifiers);

    let responseText;
    try {
      responseText = await aiGenerate(prompt, { model: selectedModel, backend: backend.backend });
    } catch (err) {
      console.error(`[renamer-agent] AI call failed for chunk ${i + 1}:`, err.message);
      continue;
    }

    let proposals = parseRenameResponse(responseText);

    // Retry once if response was not valid JSON
    if (proposals.length === 0 && responseText.trim().length > 0) {
      const retryPrompt = prompt + '\nYour previous response was not valid JSON. Respond with JSON only.';
      try {
        const retryText = await aiGenerate(retryPrompt, { model: selectedModel, backend: backend.backend });
        proposals = parseRenameResponse(retryText);
      } catch { /* give up on this chunk */ }
    }

    allProposals.push(...proposals);

    onProgress({ chunkIndex: i + 1, totalChunks: chunks.length, proposals });
  }

  // Build a single rename map from all accepted proposals
  const renameMap = {};
  for (const proposal of allProposals) {
    if (!RESERVED_IDENTIFIERS.has(proposal.original)) {
      renameMap[proposal.original] = proposal.proposed;
    }
  }

  // Apply renames via Babel AST transform
  const renamedCode = Object.keys(renameMap).length > 0
    ? transform(code, ['rename'], { renameMap })
    : code;

  return { renamedCode, proposals: allProposals };
}

/**
 * Splits code into chunks that respect approximate token limits.
 * Splits at top-level function/class boundaries where possible.
 *
 * @param {string} code
 * @returns {string[]}
 */
function splitIntoChunks(code) {
  if (code.length <= MAX_CHUNK_CHARS) return [code];

  // Simple split by top-level blank lines as a heuristic
  const blocks = code.split(/\n{2,}/);
  const chunks = [];
  let current = '';

  for (const block of blocks) {
    if ((current + block).length > MAX_CHUNK_CHARS && current.length > 0) {
      chunks.push(current.trim());
      current = block;
    } else {
      current += (current ? '\n\n' : '') + block;
    }
  }

  if (current.trim()) chunks.push(current.trim());
  return chunks;
}

/**
 * Extracts short/obfuscated identifiers from a JS code chunk.
 *
 * @param {string} code
 * @returns {string[]}
 */
function extractShortIdentifiers(code) {
  const identifiers = new Set();

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

  traverse(ast, {
    Identifier(path) {
      const { name } = path.node;
      if (
        name.length < SHORT_NAME_THRESHOLD &&
        !RESERVED_IDENTIFIERS.has(name) &&
        /^[a-zA-Z_$]/.test(name)
      ) {
        identifiers.add(name);
      }
    },
  });

  return [...identifiers];
}

/**
 * @typedef {Object} RenameProgress
 * @property {number} chunkIndex
 * @property {number} totalChunks
 * @property {Array<{original: string, proposed: string, confidence: number}>} proposals
 */

/**
 * @typedef {Object} RenameResult
 * @property {string} renamedCode
 * @property {Array<{original: string, proposed: string, confidence: number}>} proposals
 */
