/**
 * @typedef {Object} VaultMeta
 * @property {string} sessionId
 * @property {string} targetUrl
 * @property {Date} createdAt
 * @property {number} sizeBytes
 */

export { generateKeypair, encapsulate, decapsulate, encryptAesGcm, decryptAesGcm, zeroBuffer } from './kem.js';
export { writeVault, readVault, listVaults } from './vault.js';
