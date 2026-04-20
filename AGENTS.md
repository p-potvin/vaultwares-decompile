# AGENTS.md — Deconstructed

This file defines the agent behaviour rules for AI coding agents working in this repository.

## Project Identity

**Deconstructed** is a VaultWares security-research desktop tool built with Electron (main process) and React (renderer). It crawls websites, maps APIs, deobfuscates JavaScript, and uses local AI to reconstruct human-readable code.

---

## Submodule Rules

Two submodules are present:

| Path | Purpose |
|---|---|
| `vault-themes/` | VaultWare theme tokens, contrast checker, `VaultThemeManager` |
| `vaultwares-agentciation/` | Multi-agent coordination framework (Redis, heartbeat, task dispatch) |

- **Never modify files inside submodule directories.** Treat them as read-only dependencies.
- When importing theme tokens, use the JavaScript equivalent of `VaultThemeManager.export_theme_tokens()`.
- When adding agent functionality, extend `ExtrovertAgent` from `vaultwares-agentciation`.

---

## Architecture Rules

### Electron Security (non-negotiable)

- `contextIsolation: true` — always.
- `nodeIntegration: false` — always.
- `sandbox: true` — always.
- All Node.js APIs are accessed from the **main process only**, not the renderer.
- Renderer communicates with main via `contextBridge` and the IPC channels defined in `src/main/ipc-handlers.js`.
- Never expose raw `ipcRenderer` to the renderer — only the abstracted `window.electronAPI` object.

### Privacy First

- Zero external API calls. All inference runs locally (Ollama / llama.cpp).
- No analytics, telemetry, or crash reporting to third-party services.
- If a feature requires a network call, it must be explicitly opted in by the user via a UI toggle.

### Post-Quantum Cryptography

- All sensitive session data (intercepted headers, tokens, request payloads) must be encrypted via `src/crypto/vault.js` before being written to disk.
- Key encapsulation uses ML-KEM (CRYSTALS-Kyber) via the `ml-kem` npm package.
- Symmetric encryption uses AES-256-GCM.
- Private keys and shared secrets must be zeroed from memory immediately after use.

---

## Code Style Rules

### General

- Language: JavaScript (ESM — `"type": "module"` in `package.json`).
- No TypeScript (use JSDoc `@typedef` for type documentation instead).
- Imports: use `import ... from '...'` (ESM). No `require()` in new code.
- Functions: prefer named exports over default exports.
- Error handling: always log errors with context; never swallow silently.

### UI / React

- All colours must use theme tokens from `vault-themes`. No hardcoded hex values.
- Component files: `PascalCase.jsx`.
- Utility/hook files: `camelCase.js`.
- Use functional components with hooks only (no class components).
- Do not use `useEffect` for data fetching — use IPC event listeners via `useEffect` with cleanup.

### Crawler

- Crawlee `PlaywrightCrawler` must run with a request queue, not an array.
- Never log intercepted `Authorization` header values — mask them as `Bearer [REDACTED]`.
- Asset download filenames must be sanitised before writing to disk (no path traversal).

### AI / Prompt Engineering

- Prompts must explicitly instruct the model to return JSON only.
- Always validate the model's JSON response before applying renames.
- Never rename JavaScript reserved words or built-in globals.
- If the model is unavailable, surface a clear user-facing error — do not crash silently.

---

## Naming Conventions

| Type | Convention | Example |
|---|---|---|
| React component file | PascalCase | `ApiExplorerView.jsx` |
| Module/utility file | camelCase | `route-mapper.js` |
| IPC channel name | `domain:action` | `crawl:start` |
| Agent class | PascalCase + "Agent" suffix | `CrawlerAgent` |
| Vault session file | UUID + `.vlt` | `550e8400-...-.vlt` |
| Theme token reference | camelCase prefixed | `themeTokens.accent` |

---

## Task Execution Rules

1. **Read `TASKS.md` before starting work.** Pick the highest-priority unclaimed task.
2. **Only modify files listed in the task's `Outputs` section.** Do not modify shared files unless the task explicitly says so.
3. **Run `npm test` before committing.** All existing tests must pass.
4. **Check off completed tasks in `TODO.md`** as part of your commit.
5. **Broadcast status** via the `vaultwares-agentciation` framework if you are running as an autonomous agent (Redis must be available).

---

## VaultWare Branding in UI

The default theme for Deconstructed is **Cyberpunk Cinder** (`cyberpunk-cinder`):

| Token | Value |
|---|---|
| `background` | `#073642` |
| `accent` | `#CB4B16` |
| `text_primary` | `#F8FAFC` |
| `text_secondary` | `#CBD5E1` |

Use `VaultThemeManager.get_theme_by_slug('cyberpunk-cinder')` (or the JS port) for all token lookups.

The app name in all UI labels is **"Deconstructed"** — not "deconstructed-website-a-la-mode".
