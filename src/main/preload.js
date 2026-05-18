// VaultWares Decompile — Context-Isolated Preload Script
// Exposes a safe, minimal API surface to the renderer via contextBridge.

import { contextBridge, ipcRenderer } from 'electron';

/**
 * All renderer ↔ main communication goes through window.electronAPI.
 * The renderer has NO direct access to Node.js or Electron internals.
 */
contextBridge.exposeInMainWorld('electronAPI', {
  // ── Crawler ──────────────────────────────────────────────────────────────
  startCrawl: (payload) => ipcRenderer.invoke('crawl:start', payload),
  stopCrawl: (sessionId) => ipcRenderer.invoke('crawl:stop', sessionId),
  onCrawlProgress: (callback) => {
    const listener = (_event, data) => callback(data);
    ipcRenderer.on('crawl:progress', listener);
    return () => ipcRenderer.removeListener('crawl:progress', listener);
  },

  // ── Request Replay ───────────────────────────────────────────────────────
  replayRequest: (route) => ipcRenderer.invoke('request:replay', route),

  // ── JS Deobfuscator ──────────────────────────────────────────────────────
  deobfuscate: (payload) => ipcRenderer.invoke('js:deobfuscate', payload),
  onDeobfuscateResult: (callback) => {
    const listener = (_event, data) => callback(data);
    ipcRenderer.on('js:deobfuscate:result', listener);
    return () => ipcRenderer.removeListener('js:deobfuscate:result', listener);
  },

  // ── AI Renamer ───────────────────────────────────────────────────────────
  aiRename: (payload) => ipcRenderer.invoke('ai:rename', payload),
  onAiRenameProgress: (callback) => {
    const listener = (_event, data) => callback(data);
    ipcRenderer.on('ai:rename:progress', listener);
    return () => ipcRenderer.removeListener('ai:rename:progress', listener);
  },

  // ── Vault (encrypted session storage) ───────────────────────────────────
  listVaults: () => ipcRenderer.invoke('vault:list'),
  loadVault: (sessionId) => ipcRenderer.invoke('vault:load', sessionId),
  saveVault: (session) => ipcRenderer.invoke('vault:save', session),
});
