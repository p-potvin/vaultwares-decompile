// VaultWares Decompile — IPC Handler Registry
// All privileged Node.js operations are dispatched here from the main process.

import { BrowserWindow } from 'electron';
import { crawlSite } from '../crawler/crawler.js';
import { beautify } from '../deobfuscator/beautifier.js';
import { transform } from '../deobfuscator/ast-transformer.js';
import { detectBackend, generate } from '../ai/local-model.js';
import { renameIdentifiers } from '../ai/renamer-agent.js';
import { listVaults, readVault, writeVault } from '../crypto/vault.js';

/** @type {Map<string, { stop: () => void }>} Active crawl sessions */
const activeCrawls = new Map();

/**
 * Registers all IPC handlers on the provided ipcMain instance.
 * @param {Electron.IpcMain} ipcMain
 */
export function registerIpcHandlers(ipcMain) {
  // ── Crawler ──────────────────────────────────────────────────────────────

  ipcMain.handle('crawl:start', async (event, { url, options = {} }) => {
    const sender = event.sender;

    const emitProgress = (progressEvent) => {
      if (!sender.isDestroyed()) {
        sender.send('crawl:progress', progressEvent);
      }
    };

    try {
      const result = await crawlSite(url, options, emitProgress);
      return { ok: true, result };
    } catch (err) {
      emitProgress({ type: 'error', payload: { message: err.message } });
      return { ok: false, error: err.message };
    }
  });

  ipcMain.handle('crawl:stop', async (_event, sessionId) => {
    const session = activeCrawls.get(sessionId);
    if (session) {
      session.stop();
      activeCrawls.delete(sessionId);
    }
    return { ok: true };
  });

  // ── Request Replay ───────────────────────────────────────────────────────

  ipcMain.handle('request:replay', async (_event, route) => {
    const { method, url, headers = {}, body } = route;

    const startTime = Date.now();
    try {
      const { default: https } = await import('https');
      const { default: http } = await import('http');
      const parsedUrl = new URL(url);
      const client = parsedUrl.protocol === 'https:' ? https : http;

      const responseData = await new Promise((resolve, reject) => {
        const bodyStr = body ? JSON.stringify(body) : undefined;
        const reqOptions = {
          method: method.toUpperCase(),
          headers: {
            ...headers,
            ...(bodyStr ? { 'Content-Length': Buffer.byteLength(bodyStr) } : {}),
          },
        };

        const req = client.request(url, reqOptions, (res) => {
          const chunks = [];
          res.on('data', (chunk) => chunks.push(chunk));
          res.on('end', () => {
            resolve({
              status: res.statusCode,
              headers: res.headers,
              body: Buffer.concat(chunks).toString('utf-8'),
              durationMs: Date.now() - startTime,
            });
          });
        });

        req.on('error', reject);
        if (bodyStr) req.write(bodyStr);
        req.end();
      });

      return { ok: true, ...responseData };
    } catch (err) {
      return { ok: false, error: err.message, durationMs: Date.now() - startTime };
    }
  });

  // ── JS Deobfuscator ──────────────────────────────────────────────────────

  ipcMain.handle('js:deobfuscate', async (event, { code, transforms = [] }) => {
    try {
      const beautified = await beautify(code);
      const transformed = transforms.length > 0 ? transform(beautified, transforms) : beautified;
      return { ok: true, original: code, beautified: transformed };
    } catch (err) {
      return { ok: false, error: err.message };
    }
  });

  // ── AI Renamer ───────────────────────────────────────────────────────────

  ipcMain.handle('ai:rename', async (event, { code, modelName }) => {
    const sender = event.sender;

    const onProgress = (progress) => {
      if (!sender.isDestroyed()) {
        sender.send('ai:rename:progress', progress);
      }
    };

    try {
      const result = await renameIdentifiers(code, { model: modelName, onProgress });
      return { ok: true, ...result };
    } catch (err) {
      return { ok: false, error: err.message };
    }
  });

  // ── Vault ────────────────────────────────────────────────────────────────

  ipcMain.handle('vault:list', async () => {
    try {
      const sessions = await listVaults();
      return { ok: true, sessions };
    } catch (err) {
      return { ok: false, error: err.message };
    }
  });

  ipcMain.handle('vault:load', async (_event, sessionId) => {
    try {
      const data = await readVault(sessionId);
      return { ok: true, data };
    } catch (err) {
      return { ok: false, error: err.message };
    }
  });

  ipcMain.handle('vault:save', async (_event, session) => {
    try {
      await writeVault(session.sessionId, session);
      return { ok: true };
    } catch (err) {
      return { ok: false, error: err.message };
    }
  });
}
