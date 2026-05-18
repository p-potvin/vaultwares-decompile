// VaultWares Decompile — JS Asset Downloader
// Downloads and stores minified JavaScript bundles from crawled pages.

import { createHash } from 'crypto';
import { mkdir, writeFile } from 'fs/promises';
import path from 'path';
import os from 'os';

const VAULT_DIR = path.join(os.homedir(), '.vaultwares-decompile', 'sessions');
const MIN_ASSET_SIZE_BYTES = 1024;
const JS_MIME_TYPES = new Set(['application/javascript', 'text/javascript', 'application/x-javascript']);

/**
 * Downloads a JS asset from a URL and saves it to the session's assets directory.
 * Returns null if the asset should be skipped (too small, wrong MIME type, fetch failure).
 *
 * @param {string} url
 * @param {string} sessionId
 * @returns {Promise<JsAsset | null>}
 */
export async function downloadJsAsset(url, sessionId) {
  try {
    const { default: https } = await import('https');
    const { default: http } = await import('http');
    const parsedUrl = new URL(url);
    const client = parsedUrl.protocol === 'https:' ? https : http;

    const { contentType, buffer } = await new Promise((resolve, reject) => {
      client.get(url, (res) => {
        const chunks = [];
        res.on('data', (c) => chunks.push(c));
        res.on('end', () =>
          resolve({ contentType: res.headers['content-type'] || '', buffer: Buffer.concat(chunks) })
        );
      }).on('error', reject);
    });

    const mimeType = contentType.split(';')[0].trim().toLowerCase();
    if (!JS_MIME_TYPES.has(mimeType)) return null;
    if (buffer.length < MIN_ASSET_SIZE_BYTES) return null;

    const sha256 = createHash('sha256').update(buffer).digest('hex');
    const originalFilename = path.basename(parsedUrl.pathname) || 'bundle.js';
    const safeFilename = sanitiseFilename(`${sha256.slice(0, 8)}_${originalFilename}`);

    const assetDir = path.join(VAULT_DIR, sessionId, 'assets');
    await mkdir(assetDir, { recursive: true });
    await writeFile(path.join(assetDir, safeFilename), buffer);

    return {
      id: sha256,
      filename: safeFilename,
      originalUrl: url,
      sizeBytes: buffer.length,
      sha256,
      localPath: path.join(assetDir, safeFilename),
    };
  } catch {
    return null;
  }
}

/**
 * Sanitises a filename to prevent path traversal and overly long names.
 *
 * @param {string} filename
 * @returns {string}
 */
function sanitiseFilename(filename) {
  return filename
    .replace(/[^a-zA-Z0-9._\-]/g, '_') // allow only safe characters
    .slice(0, 80);                        // cap length
}

/**
 * @typedef {Object} JsAsset
 * @property {string} id          - SHA-256 hex digest (used as unique ID)
 * @property {string} filename    - Sanitised local filename
 * @property {string} originalUrl - Source URL
 * @property {number} sizeBytes
 * @property {string} sha256
 * @property {string} localPath   - Absolute local file path
 */
