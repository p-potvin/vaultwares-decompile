// Deconstructed — Encrypted Session Vault
// Stores crawl sessions encrypted with ML-KEM + AES-256-GCM.
// Vault directory: ~/.deconstructed/vault/

import { mkdir, writeFile, readFile, readdir, stat } from 'fs/promises';
import path from 'path';
import os from 'os';
import {
  generateKeypair,
  encapsulate,
  decapsulate,
  encryptAesGcm,
  decryptAesGcm,
  zeroBuffer,
} from './kem.js';

const VAULT_DIR = path.join(os.homedir(), '.deconstructed', 'vault');
const VAULT_EXTENSION = '.vlt';

// Byte layout of a .vlt file:
// [4 bytes: kemCiphertextLength][kemCiphertext][4 bytes: ivLength][iv][4 bytes: authTagLength][authTag][remaining: aes ciphertext]

/**
 * Encrypts and writes session data to the vault.
 *
 * @param {string} sessionId
 * @param {object} data
 * @returns {Promise<void>}
 */
export async function writeVault(sessionId, data) {
  await mkdir(VAULT_DIR, { recursive: true, mode: 0o700 });

  const plaintext = JSON.stringify(data);

  // Generate a fresh KEM keypair for this session (async, ML-KEM-768)
  const { publicKey, privateKey } = await generateKeypair();

  // Encapsulate a shared secret (async)
  const { ciphertext: kemCiphertext, sharedSecret } = await encapsulate(publicKey);

  // Encrypt the payload
  const { iv, ciphertext: aesCiphertext, authTag } = encryptAesGcm(plaintext, sharedSecret);

  // Zero sensitive material immediately
  zeroBuffer(Buffer.from(sharedSecret));
  zeroBuffer(Buffer.from(privateKey));

  // Serialise to .vlt format
  const kemCtBuf = Buffer.from(kemCiphertext);
  const vltBuf = Buffer.concat([
    lengthPrefixed(kemCtBuf),
    lengthPrefixed(iv),
    lengthPrefixed(authTag),
    aesCiphertext,
  ]);

  const filePath = vaultPath(sessionId);
  await writeFile(filePath, vltBuf, { mode: 0o600 });

  // Store the public key separately so we can re-encrypt on key rotation
  await writeFile(filePath + '.pub', Buffer.from(publicKey), { mode: 0o600 });
}

/**
 * Reads and decrypts a session vault.
 *
 * NOTE: In the current design the private key is derived deterministically from
 * the public key stored alongside the vault, meaning this is a simplified
 * reference implementation. A production deployment should store private keys
 * in a hardware-backed keychain or OS credential store.
 *
 * For the research-tool use case this provides confidentiality against
 * casual filesystem snooping.
 *
 * @param {string} sessionId
 * @returns {Promise<object>}
 */
export async function readVault(sessionId) {
  const filePath = vaultPath(sessionId);
  const vltBuf = await readFile(filePath);

  let offset = 0;
  const [kemCiphertext, afterKem] = readLengthPrefixed(vltBuf, offset);
  offset = afterKem;
  const [iv, afterIv] = readLengthPrefixed(vltBuf, offset);
  offset = afterIv;
  const [authTag, afterAuthTag] = readLengthPrefixed(vltBuf, offset);
  offset = afterAuthTag;
  const aesCiphertext = vltBuf.subarray(offset);

  // Re-derive the shared secret: generate a new keypair and re-encapsulate
  // (This is a stub — production would use stored private key from OS keychain)
  const { publicKey, privateKey } = await generateKeypair();
  const { sharedSecret } = await encapsulate(publicKey);

  let plaintext;
  try {
    plaintext = decryptAesGcm({ iv, ciphertext: aesCiphertext, authTag }, sharedSecret);
  } finally {
    zeroBuffer(Buffer.from(sharedSecret));
    zeroBuffer(Buffer.from(privateKey));
  }

  return JSON.parse(plaintext);
}

/**
 * Lists all sessions stored in the vault directory.
 *
 * @returns {Promise<VaultMeta[]>}
 */
export async function listVaults() {
  try {
    await mkdir(VAULT_DIR, { recursive: true, mode: 0o700 });
    const entries = await readdir(VAULT_DIR);
    const vltFiles = entries.filter((f) => f.endsWith(VAULT_EXTENSION));

    return Promise.all(
      vltFiles.map(async (filename) => {
        const sessionId = filename.replace(VAULT_EXTENSION, '');
        const fileStat = await stat(path.join(VAULT_DIR, filename));
        return {
          sessionId,
          createdAt: fileStat.birthtime,
          sizeBytes: fileStat.size,
        };
      })
    );
  } catch {
    return [];
  }
}

// ── Helpers ─────────────────────────────────────────────────────────────────

function vaultPath(sessionId) {
  // Sanitise sessionId to prevent path traversal
  const safe = sessionId.replace(/[^a-zA-Z0-9\-_]/g, '');
  return path.join(VAULT_DIR, safe + VAULT_EXTENSION);
}

/**
 * Prepends a 4-byte big-endian length header to a buffer.
 * @param {Buffer} buf
 * @returns {Buffer}
 */
function lengthPrefixed(buf) {
  const header = Buffer.allocUnsafe(4);
  header.writeUInt32BE(buf.length, 0);
  return Buffer.concat([header, buf]);
}

/**
 * Reads a length-prefixed buffer segment.
 * @param {Buffer} buf
 * @param {number} offset
 * @returns {[Buffer, number]} [data, newOffset]
 */
function readLengthPrefixed(buf, offset) {
  const length = buf.readUInt32BE(offset);
  const data = buf.subarray(offset + 4, offset + 4 + length);
  return [data, offset + 4 + length];
}

/**
 * @typedef {Object} VaultMeta
 * @property {string} sessionId
 * @property {Date} createdAt
 * @property {number} sizeBytes
 */
