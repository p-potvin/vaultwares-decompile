# Deconstructed — Task Dispatch Guide for Sub-Agents

> This file enables parallel development by providing self-contained task specifications that can be dispatched to independent sub-agents without coordination overhead. Each task includes: context, interface contracts, acceptance criteria, and the exact files to create or modify.

---

## How to Use This File

1. Pick a task block.
2. Dispatch it to a sub-agent with full context from this block.
3. The sub-agent commits only the files listed in **Outputs**.
4. Do not assign tasks with overlapping **Outputs** to concurrent agents.

---

## TASK-01 · Project Scaffold & package.json

**Agent role:** Node.js / Electron specialist  
**Depends on:** Nothing  
**Outputs:** `package.json`, `vite.config.js`, `.env.example`, `electron-builder.yml`, `src/main/index.js`, `src/main/preload.js`, `src/main/ipc-handlers.js`, `src/renderer/index.html`, `src/renderer/main.jsx`, `src/renderer/App.jsx`

### Context

This is an Electron 30 desktop application. The renderer is React 18 + Vite. The main process is Node.js (ESM). The crawler runs in the main process (Node.js has Playwright; renderer does not). Communication between renderer and main uses Electron IPC with a `contextBridge` preload.

### Required npm dependencies

```json
{
  "electron": "^30.0.0",
  "crawlee": "^3.11.0",
  "playwright": "^1.44.0",
  "@babel/core": "^7.24.0",
  "@babel/traverse": "^7.24.0",
  "@babel/generator": "^7.24.0",
  "@babel/parser": "^7.24.0",
  "js-beautify": "^1.15.0",
  "prettier": "^3.2.0",
  "diff": "^5.2.0",
  "source-map": "^0.7.4",
  "react": "^18.3.0",
  "react-dom": "^18.3.0",
  "vite": "^5.2.0",
  "@vitejs/plugin-react": "^4.2.0",
  "electron-builder": "^24.13.0",
  "electron-updater": "^6.1.0"
}
```

### IPC channel contract

Define these channels in `src/main/ipc-handlers.js`:

| Channel | Direction | Payload |
|---|---|---|
| `crawl:start` | renderer → main | `{ url: string, options: CrawlOptions }` |
| `crawl:stop` | renderer → main | `{ sessionId: string }` |
| `crawl:progress` | main → renderer | `CrawlProgressEvent` |
| `request:replay` | renderer → main | `{ route: RouteSchema }` |
| `request:replay:response` | main → renderer | `{ status, headers, body, durationMs }` |
| `js:deobfuscate` | renderer → main | `{ assetId: string, transforms: string[] }` |
| `js:deobfuscate:result` | main → renderer | `{ assetId, original, beautified }` |
| `ai:rename` | renderer → main | `{ assetId: string, modelName: string }` |
| `ai:rename:progress` | main → renderer | `{ assetId, chunk, total, renames }` |
| `vault:list` | renderer → main | `{}` |
| `vault:load` | renderer → main | `{ sessionId: string }` |
| `vault:save` | renderer → main | `{ session: SessionData }` |

### Acceptance criteria

- `npm start` launches the Electron window without errors.
- The renderer shows the app shell (navigation bar + empty crawler view).
- `npm run build` produces distributable artifacts in `dist/`.
- No `nodeIntegration: true` anywhere.

---

## TASK-02 · Web Crawler Module

**Agent role:** Crawlee / Playwright specialist  
**Depends on:** TASK-01 (package.json must exist so npm packages are available)  
**Outputs:** `src/crawler/crawler.js`, `src/crawler/api-interceptor.js`, `src/crawler/route-mapper.js`, `src/crawler/js-downloader.js`

### Context

Use `PlaywrightCrawler` from `crawlee`. The crawler is called from the Electron main process (`src/main/ipc-handlers.js`) — NOT from the renderer. Results are emitted to the renderer via IPC events.

### Interface contract

```js
// src/crawler/crawler.js
/**
 * @param {string} startUrl
 * @param {CrawlOptions} options
 * @param {(event: CrawlProgressEvent) => void} onProgress
 * @returns {Promise<CrawlResult>}
 */
export async function crawlSite(startUrl, options, onProgress) {}

/**
 * @typedef {Object} CrawlOptions
 * @property {number} maxPages        - Maximum pages to visit (default: 100)
 * @property {number} maxConcurrency  - Parallel browser tabs (default: 3)
 * @property {string[]} excludePatterns - URL patterns to skip (regex strings)
 * @property {boolean} followExternalLinks - Follow links outside the start domain
 */

/**
 * @typedef {Object} CrawlResult
 * @property {RouteSchema[]} routes   - All discovered API routes
 * @property {JsAsset[]} jsAssets     - All downloaded JS bundle assets
 * @property {string} sessionId       - UUID for this crawl session
 * @property {number} pagesVisited
 * @property {number} durationMs
 */

/**
 * @typedef {Object} CrawlProgressEvent
 * @property {'route_found'|'asset_downloaded'|'page_crawled'|'error'} type
 * @property {*} payload
 */
```

### Route normalisation rules

```
/user/42           → /user/:id         (positive integer)
/user/abc123       → /user/:id         (alphanumeric, length 3-20)
/item/550e8400-... → /item/:uuid       (UUID v1-v5)
/post/my-slug-here → /post/:slug       (kebab-case, 3+ chars, no digits)
/api/v1/resource   → unchanged         (keep version segments literal)
```

### JS asset download rules

- Store assets to `~/.deconstructed/sessions/<sessionId>/assets/`
- Filename: `<SHA256[:8]>_<original-filename>` (max 80 chars)
- Skip assets smaller than 1 KB (likely inline stubs)
- Skip assets where MIME type is not `application/javascript` or `text/javascript`

### Acceptance criteria

- `crawlSite('https://httpbin.org', ...)` completes without throwing.
- At least one route is captured for a site with XHR calls.
- Downloaded assets are written to the correct directory.
- Route normaliser correctly handles all 5 pattern types listed above.

---

## TASK-03 · JS Deobfuscator Pipeline

**Agent role:** Babel AST / JavaScript tooling specialist  
**Depends on:** TASK-01  
**Outputs:** `src/deobfuscator/beautifier.js`, `src/deobfuscator/ast-transformer.js`, `src/deobfuscator/module-splitter.js`

### Context

The deobfuscator receives raw JS text (a downloaded bundle) and returns a cleaned, formatted, human-readable version. It runs in the Electron main process.

### Interface contract

```js
// src/deobfuscator/beautifier.js
/**
 * @param {string} code - Raw minified JavaScript
 * @returns {Promise<string>} - Indented, formatted code
 */
export async function beautify(code) {}

// src/deobfuscator/ast-transformer.js
/**
 * @param {string} code - JavaScript source (may be minified or beautified)
 * @param {string[]} transforms - Which transforms to apply
 * @returns {string} - Transformed code
 *
 * Available transforms (each is a string key):
 *   'hex-literals'       - 0x68656c6c6f → string or decimal
 *   'unicode-escapes'    - \u0068ello → hello
 *   'constant-folding'   - 1 + 2 → 3, "a" + "b" → "ab"
 *   'dead-code'          - remove always-false branches
 *   'boolean-literals'   - !0 → true, !1 → false
 *   'sequence-expansion' - a=1,b=2 → separate statements
 *   'rename'             - apply a provided rename map
 */
export function transform(code, transforms, options = {}) {}

// src/deobfuscator/module-splitter.js
/**
 * @param {string} code - A webpack bundle
 * @returns {WebpackModule[]} - Extracted module objects
 *
 * @typedef {Object} WebpackModule
 * @property {string|number} id
 * @property {string} code
 * @property {string[]} dependencies - module IDs this module requires
 */
export function splitWebpackBundle(code) {}
```

### Babel transform implementation notes

- Use `@babel/parser` → `@babel/traverse` → `@babel/generator` pipeline.
- Each transform is a separate `visitor` object; compose them before traversal.
- On parse error, log the error and return the original `code` unchanged.
- Do not mutate the input; always return a new string.

### Acceptance criteria

- `beautify(uglifiedCode)` returns the same logical code, formatted.
- Each transform key produces the correct output for a provided test input.
- `splitWebpackBundle` correctly identifies at least 3 modules in a sample webpack v4 bundle.
- A 500 KB bundle is processed in under 5 seconds.

---

## TASK-04 · AI Code Renamer

**Agent role:** LLM integration / prompt engineering specialist  
**Depends on:** TASK-01, TASK-03 (needs `transform` with `'rename'` support)  
**Outputs:** `src/ai/local-model.js`, `src/ai/prompt-builder.js`, `src/ai/renamer-agent.js`

### Context

The AI renamer connects to a **locally running** Ollama instance (`localhost:11434`) or llama.cpp OpenAI-compatible server (`localhost:8080`). **No external API calls.** The model is asked to produce a JSON rename map from obfuscated identifiers to semantic names.

### Interface contract

```js
// src/ai/local-model.js
/**
 * @returns {Promise<{available: boolean, backend: 'ollama'|'llamacpp'|null, models: string[]}>}
 */
export async function detectBackend() {}

/**
 * @param {string} prompt
 * @param {object} options
 * @param {string} options.model
 * @param {(token: string) => void} [options.onToken]
 * @returns {Promise<string>} - Complete response text
 */
export async function generate(prompt, options) {}

// src/ai/prompt-builder.js
/**
 * @param {string} codeChunk     - A chunk of beautified but still obfuscated JS
 * @param {string[]} identifiers - List of identifiers to rename in this chunk
 * @returns {string}             - Complete prompt string (system + user)
 */
export function buildRenamePrompt(codeChunk, identifiers) {}

// src/ai/renamer-agent.js
/**
 * @param {string} code       - Full beautified JS file
 * @param {object} options
 * @param {string} options.model
 * @param {(progress: RenameProgress) => void} options.onProgress
 * @returns {Promise<RenameResult>}
 *
 * @typedef {Object} RenameProgress
 * @property {number} chunkIndex
 * @property {number} totalChunks
 * @property {RenameProposal[]} proposals
 *
 * @typedef {Object} RenameResult
 * @property {string} renamedCode
 * @property {RenameProposal[]} proposals
 *
 * @typedef {Object} RenameProposal
 * @property {string} original
 * @property {string} proposed
 * @property {number} confidence  - 0.0 to 1.0
 */
export async function renameIdentifiers(code, options) {}
```

### Prompt engineering rules

- System prompt must instruct the model to return **only** valid JSON — no prose.
- JSON schema: `{ "renames": [{ "original": "a", "proposed": "userToken", "confidence": 0.9 }] }`
- Include 3 few-shot examples in the system prompt.
- If the model returns non-JSON, retry once with an explicit correction prompt.
- Never rename: `arguments`, `this`, `undefined`, `null`, `true`, `false`, `NaN`, `Infinity`.

### Acceptance criteria

- `detectBackend()` returns `{ available: false }` gracefully when no local model server is running.
- `buildRenamePrompt` returns a string that parses as a valid prompt (includes system + user sections).
- `renameIdentifiers` applied to a 200-line obfuscated snippet returns at least 5 rename proposals.
- All rename proposals are applied consistently across the output code (no stale references to old names).

---

## TASK-05 · Post-Quantum Crypto Vault

**Agent role:** Cryptography / Node.js security specialist  
**Depends on:** TASK-01  
**Outputs:** `src/crypto/kem.js`, `src/crypto/vault.js`

### Context

Session data (intercepted headers, request bodies) may contain sensitive credentials. This module encrypts that data using ML-KEM (CRYSTALS-Kyber) for key encapsulation and AES-256-GCM for bulk encryption. Everything is local — no network calls.

Use the `mlkem` npm package (NIST FIPS 203 compliant). Add it to `package.json`.

### Interface contract

```js
// src/crypto/kem.js
/** @returns {{ publicKey: Uint8Array, privateKey: Uint8Array }} */
export function generateKeypair() {}

/** @returns {{ ciphertext: Uint8Array, sharedSecret: Uint8Array }} */
export function encapsulate(publicKey) {}

/** @returns {Uint8Array} sharedSecret */
export function decapsulate(ciphertext, privateKey) {}

/**
 * @param {string|Buffer} plaintext
 * @param {Uint8Array} sharedSecret
 * @returns {{ iv: Buffer, ciphertext: Buffer, authTag: Buffer }}
 */
export function encryptAesGcm(plaintext, sharedSecret) {}

/**
 * @param {{ iv: Buffer, ciphertext: Buffer, authTag: Buffer }} encrypted
 * @param {Uint8Array} sharedSecret
 * @returns {string}
 */
export function decryptAesGcm(encrypted, sharedSecret) {}

// src/crypto/vault.js
/**
 * @param {string} sessionId
 * @param {object} data
 * @returns {Promise<void>}
 */
export async function writeVault(sessionId, data) {}

/**
 * @param {string} sessionId
 * @returns {Promise<object>}
 */
export async function readVault(sessionId) {}

/** @returns {Promise<VaultMeta[]>} */
export async function listVaults() {}

/**
 * @typedef {Object} VaultMeta
 * @property {string} sessionId
 * @property {string} targetUrl
 * @property {Date} createdAt
 * @property {number} sizeBytes
 */
```

### Security requirements

- Generate a fresh KEM keypair on each `writeVault` call.
- Private key buffer must be zeroed immediately after `decapsulate` returns.
- Shared secret buffer must be zeroed immediately after AES key derivation.
- Vault directory: `~/.deconstructed/vault/` — create with `chmod 700`.
- Each session is stored as `<sessionId>.vlt` (binary: KEM ciphertext header + AES ciphertext).

### Acceptance criteria

- `writeVault` then `readVault` round-trips arbitrary JSON without data loss.
- Private key and shared secret buffers are zeroed in memory after use (verified by reading buffer after operation).
- `listVaults` returns correct metadata for all saved sessions.
- `ml-kem` KEM output passes a basic correctness check: `decapsulate(encapsulate(pk).ciphertext, sk)` equals the same shared secret.

---

## TASK-06 · Renderer UI — Crawler View

**Agent role:** React / Electron renderer specialist  
**Depends on:** TASK-01 (App shell must exist)  
**Outputs:** `src/renderer/views/CrawlerView.jsx`, `src/renderer/components/RouteCard.jsx`, `src/renderer/components/ProgressBar.jsx`

### Context

The Crawler View is the first screen the user sees. It lets them enter a target URL, start a crawl, watch progress in real time, and see discovered routes appear as cards.

### Component spec: `CrawlerView`

```
┌─────────────────────────────────────────────────────────┐
│  🔍 Target URL  [https://example.com          ] [Crawl] │
│                                                         │
│  ▓▓▓▓▓▓▓▓▓▓░░░░░░░░░░  48 routes found · 12 assets    │
│                                                         │
│  ┌─ Routes ─────────────────────────────────────────┐  │
│  │  GET  /api/users          (3 intercepted calls)   │  │
│  │  POST /api/auth/login     (1 intercepted call)    │  │
│  │  GET  /api/products/:id   (7 intercepted calls)   │  │
│  └───────────────────────────────────────────────────┘  │
└─────────────────────────────────────────────────────────┘
```

### State management

```js
const [targetUrl, setTargetUrl] = useState('');
const [crawlState, setCrawlState] = useState('idle'); // 'idle'|'running'|'done'|'error'
const [routes, setRoutes] = useState([]);
const [assets, setAssets] = useState([]);
const [progress, setProgress] = useState({ pages: 0, routes: 0, assets: 0 });
```

### IPC wiring

- On "Crawl" click: call `window.electronAPI.startCrawl({ url: targetUrl })`
- Listen: `window.electronAPI.onCrawlProgress(event => ...)` — update state
- On route card click: navigate to API Explorer with route pre-selected
- On asset click: navigate to JS Deobfuscator with asset pre-selected

### Acceptance criteria

- URL input validates `https://` prefix; shows inline error for invalid URLs.
- Progress bar updates in real time as IPC events arrive.
- Route cards render method badge (coloured: GET=blue, POST=green, etc.) and pattern.
- "Stop" button cancels the active crawl via IPC.
- All colours come from theme tokens — no hardcoded hex values.

---

## TASK-07 · Renderer UI — API Explorer View

**Agent role:** React / Electron renderer specialist  
**Depends on:** TASK-06 (routes state must be defined)  
**Outputs:** `src/renderer/views/ApiExplorerView.jsx`, `src/renderer/components/RequestEditor.jsx`, `src/renderer/components/ResponseViewer.jsx`

### Component spec: `ApiExplorerView`

```
┌───────────────┬─────────────────────────────────────────┐
│ /api          │  POST /api/auth/login                    │
│  ├ POST /auth │                                         │
│  └ GET /users │  Headers                                 │
│ /graphql      │  ┌────────────────┬───────────────────┐ │
│  └ POST /     │  │ Content-Type   │ application/json  │ │
│               │  │ Authorization  │ Bearer ●●●●●●    │ │
│               │  └────────────────┴───────────────────┘ │
│               │                                         │
│               │  Body                                   │
│               │  { "email": "...", "password": "..." }  │
│               │                                         │
│               │  [Send Request]  [Export]               │
│               │                                         │
│               │  Response 200 OK · 142ms               │
│               │  { "token": "eyJ..." }                  │
└───────────────┴─────────────────────────────────────────┘
```

### Acceptance criteria

- Left panel groups routes by first path segment.
- Clicking a route populates the right panel with all captured headers and body.
- "Send Request" fires IPC call and displays the response below.
- Authorization header values are masked (`Bearer ●●●●●●`) by default with a toggle to reveal.
- "Export" button shows dropdown: "OpenAPI 3.0" / "Postman Collection".

---

## TASK-08 · Renderer UI — JS Deobfuscator View

**Agent role:** React / Electron renderer specialist  
**Depends on:** TASK-06 (assets state must be defined)  
**Outputs:** `src/renderer/views/JsDeobfuscatorView.jsx`, `src/renderer/components/CodeViewer.jsx`

### Component spec: `JsDeobfuscatorView`

```
┌─────────────────┬──────────────────────────────────────────────────────────┐
│ Assets          │   chunk.abc123.js (482 KB)                               │
│ ─────────────   │                                                          │
│ chunk.abc123.js │  [Beautify] [Transform ▼] [AI Rename ▼] [Export]        │
│ vendor.min.js   │                                                          │
│ app.bundle.js   │  ┌──── Original ─────────────┬─── Beautified ─────────┐ │
│                 │  │ !function(e){var t=e.doc  │ function init(element){ │ │
│                 │  │ ument;t.createElement("d  │   var doc = element.doc │ │
│                 │  │ iv")}(window);            │   ument;                │ │
│                 │  │                           │   doc.createElement(    │ │
│                 │  │                           │     "div"               │ │
│                 │  │                           │   );                    │ │
│                 │  └───────────────────────────┴─────────────────────────┘ │
│                 │                                                          │
│                 │  AI Rename Proposals (12)                                │
│                 │  ✓ e → element   ✓ t → doc   ? n → ???  [Apply All]     │
└─────────────────┴──────────────────────────────────────────────────────────┘
```

### Acceptance criteria

- Asset list shows file name, size, and SHA-256 fingerprint.
- Side-by-side code panes scroll synchronously.
- Syntax highlighting is applied using `shiki` or `prism-react-renderer`.
- AI rename proposals list shows accept (✓) / reject (✗) toggles per item.
- "Apply All" applies only accepted proposals and reloads the right pane.
- "Export" saves the current right-pane content as a `.js` file.

---

## TASK-09 · Agent Integration Layer

**Agent role:** vaultwares-agentciation / Redis multi-agent specialist  
**Depends on:** TASK-01, TASK-02, TASK-04  
**Outputs:** `src/agents/crawler-agent.js`, `src/agents/deobfuscator-agent.js`, `src/agents/ai-renamer-agent.js`, `src/agents/manager.js`

### Context

All long-running processing tasks (crawl, deobfuscate, AI rename) are wrapped as agents using the `vaultwares-agentciation` framework. Agents report heartbeats and status to Redis. `LonelyManager` monitors them and restarts on failure.

The submodule is at `vaultwares-agentciation/`. Import via:
```js
import path from 'path';
import { createRequire } from 'module';
const require = createRequire(import.meta.url);
const submodulePath = path.resolve('vaultwares-agentciation');
// Use dynamic import after adding submodule root to sys.path equivalent
```

Since the submodule is Python, the JS integration communicates via a Redis channel. Each agent publishes status objects to the `tasks` channel.

For the JS side, implement lightweight Redis pub/sub wrappers using `ioredis`:

### Interface contract

```js
// src/agents/manager.js
export class DeconstructedManager {
  constructor(redisUrl = 'redis://localhost:6379') {}
  async start() {}
  async assignCrawl(targetUrl, options) {} // dispatches to CrawlerAgent
  async assignDeobfuscate(assetId) {}     // dispatches to DeobfuscatorAgent
  async assignAiRename(assetId, model) {} // dispatches to AiRenamerAgent
  getTeamStatus() {}
}
```

### Acceptance criteria

- `DeconstructedManager` starts without throwing when Redis is unavailable (graceful degradation).
- When Redis is available, crawl tasks are published to the `tasks` channel.
- `getTeamStatus()` returns at least `{ crawlerAgent, deobfuscatorAgent, aiRenamerAgent }` objects with status fields.

---

## TASK-10 · Electron Packaging & Release Workflow

**Agent role:** DevOps / electron-builder specialist  
**Depends on:** All other tasks  
**Outputs:** `electron-builder.yml`, `.github/workflows/release.yml`, `scripts/notarize.js`

### electron-builder configuration

```yaml
appId: com.vaultwares.deconstructed
productName: Deconstructed
directories:
  output: dist
  buildResources: build
files:
  - src/**/*
  - package.json
mac:
  category: public.app-category.developer-tools
  hardenedRuntime: true
  entitlements: build/entitlements.mac.plist
  entitlementsInherit: build/entitlements.mac.plist
  afterSign: scripts/notarize.js
win:
  target: nsis
linux:
  target: AppImage
```

### GitHub Actions release workflow

Trigger: `push` to tag `v*.*.*`  
Steps: checkout → setup-node 20 → npm ci → npm run build → electron-builder → upload artifacts

### Acceptance criteria

- `npm run dist` produces at least one distributable artifact.
- The release workflow YAML is syntactically valid.
- `scripts/notarize.js` reads `APPLE_ID` and `APPLE_ID_PASSWORD` from environment (not hardcoded).

---

*All tasks above are independent enough to be executed in parallel once TASK-01 is complete. Assign TASK-01 first, then dispatch TASK-02 through TASK-10 simultaneously.*
