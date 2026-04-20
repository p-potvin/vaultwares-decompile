// Deconstructed — Post-Quantum Key Encapsulation Mechanism
// Uses ML-KEM (CRYSTALS-Kyber, NIST FIPS 203) for key encapsulation.
// Symmetric encryption uses AES-256-GCM.

import { MlKem768 } from 'mlkem';
import { createCipheriv, createDecipheriv, randomBytes } from 'crypto';

const AES_KEY_LENGTH = 32; // 256 bits
const GCM_IV_LENGTH = 12;  // 96 bits (NIST recommended for GCM)
const GCM_AUTH_TAG_LENGTH = 16; // 128 bits

/**
 * Generates an ML-KEM-768 keypair.
 *
 * @returns {Promise<{ publicKey: Uint8Array, privateKey: Uint8Array }>}
 */
export async function generateKeypair() {
  const kem = new MlKem768();
  const [publicKey, secretKey] = await kem.generateKeyPair();
  return { publicKey, privateKey: secretKey };
}

/**
 * Encapsulates a shared secret using the recipient's public key.
 *
 * @param {Uint8Array} publicKey
 * @returns {Promise<{ ciphertext: Uint8Array, sharedSecret: Uint8Array }>}
 */
export async function encapsulate(publicKey) {
  const kem = new MlKem768();
  const [cipherText, sharedSecret] = await kem.encap(publicKey);
  return { ciphertext: cipherText, sharedSecret };
}

/**
 * Decapsulates the shared secret from a KEM ciphertext using the private key.
 * The private key buffer should be zeroed by the caller after this returns.
 *
 * @param {Uint8Array} ciphertext
 * @param {Uint8Array} privateKey
 * @returns {Promise<Uint8Array>} sharedSecret
 */
export async function decapsulate(ciphertext, privateKey) {
  const kem = new MlKem768();
  return kem.decap(ciphertext, privateKey);
}

/**
 * Encrypts plaintext using AES-256-GCM with the provided shared secret.
 *
 * @param {string | Buffer} plaintext
 * @param {Uint8Array} sharedSecret - 32-byte key (uses first 32 bytes of longer keys)
 * @returns {{ iv: Buffer, ciphertext: Buffer, authTag: Buffer }}
 */
export function encryptAesGcm(plaintext, sharedSecret) {
  const key = Buffer.from(sharedSecret).subarray(0, AES_KEY_LENGTH);
  const iv = randomBytes(GCM_IV_LENGTH);
  const cipher = createCipheriv('aes-256-gcm', key, iv);

  const plaintextBuf = Buffer.isBuffer(plaintext) ? plaintext : Buffer.from(plaintext, 'utf-8');
  const ciphertext = Buffer.concat([cipher.update(plaintextBuf), cipher.final()]);
  const authTag = cipher.getAuthTag();

  return { iv, ciphertext, authTag };
}

/**
 * Decrypts AES-256-GCM ciphertext using the provided shared secret.
 *
 * @param {{ iv: Buffer, ciphertext: Buffer, authTag: Buffer }} encrypted
 * @param {Uint8Array} sharedSecret
 * @returns {string} Decrypted plaintext (UTF-8)
 */
export function decryptAesGcm({ iv, ciphertext, authTag }, sharedSecret) {
  const key = Buffer.from(sharedSecret).subarray(0, AES_KEY_LENGTH);
  const decipher = createDecipheriv('aes-256-gcm', key, iv);
  decipher.setAuthTag(authTag);

  const plaintext = Buffer.concat([decipher.update(ciphertext), decipher.final()]);
  return plaintext.toString('utf-8');
}

/**
 * Zeroes a Buffer or Uint8Array in memory.
 * Call this after any sensitive key or secret material is no longer needed.
 *
 * @param {Buffer | Uint8Array} buf
 */
export function zeroBuffer(buf) {
  if (buf) buf.fill(0);
}
