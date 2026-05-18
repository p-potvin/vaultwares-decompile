# VaultWares Decompile — Development Roadmap

> VaultWares | Privacy First · Security Second · Post-Quantum Ready

---

## Vision

VaultWares Decompile is a desktop security-research tool that exposes the gap between "obfuscated" and "secure". It crawls a target site, maps its API surface, downloads its JS bundles, and then uses local AI to reconstruct semantically meaningful source code — all without a single byte of data leaving the user's machine.

---

## Phase 0 — Project Scaffold *(current)*

**Goal:** Lay the technical foundation so multiple agents can develop modules in parallel without merge conflicts.

| # | Deliverable | Description |
|---|---|---|
| 0.1 | Repository structure | `src/` directories for main, renderer, crawler, deobfuscator, ai, crypto, agents |
| 0.2 | Submodules wired | `vault-themes` and `vaultwares-agentciation` present and initialised |
| 0.3 | `package.json` | Electron 30 + Crawlee 3 + Playwright + Babel + Prettier dependencies |
| 0.4 | `.env.example` | All required runtime configuration keys documented |
| 0.5 | `ROADMAP.md`, `TODO.md`, `TASKS.md`, `AGENTS.md` | This document and companion planning files |
| 0.6 | Module interface contracts | TypeScript-style JSDoc `@typedef` blocks in each module's `index.js` so agents can code against stable interfaces |

---

## Phase 1 — Crawler Engine

**Goal:** Given a starting URL, automatically discover all routes and intercept every API call made by the page.

| # | Deliverable | Description |
|---|---|---|
| 1.1 | Playwright crawler setup | `CheerioCrawler` + `PlaywrightCrawler` hybrid; follows `<a>` links and SPA navigation events |
| 1.2 | Request interceptor | Hooks into `page.on('request')` to capture method, URL, headers, and POST body for every XHR/fetch call |
| 1.3 | Route deduplication | Normalises parameterised URLs (e.g. `/user/123` → `/user/:id`) using path-pattern heuristics |
| 1.4 | JS asset harvester | Detects `<script src>` and dynamically loaded `import()` chunks; downloads and stores them locally |
| 1.5 | Crawl session persistence | Saves the crawl result (routes, assets) to an encrypted local vault so sessions are resumable |
| 1.6 | IPC events | Emits progress events to the renderer (`crawl:start`, `crawl:route-found`, `crawl:asset-downloaded`, `crawl:complete`) |
| 1.7 | Agent integration | `CrawlerAgent` extends `ExtrovertAgent`; broadcasts status and heartbeat to Redis during long crawls |

---

## Phase 2 — API Explorer (Swagger-on-the-fly)

**Goal:** Present every intercepted route as an interactive, Postman-style request panel.

| # | Deliverable | Description |
|---|---|---|
| 2.1 | Route data model | JSON schema for a captured route: `{ method, url, pattern, headers, body, response_sample }` |
| 2.2 | `ApiExplorerView` | Left-panel route list + right-panel request editor; grouped by path prefix |
| 2.3 | `RequestEditor` component | Editable method selector, URL bar, header key-value grid, body textarea (JSON / form-data) |
| 2.4 | Live request replay | Sends the request through the Electron main process (bypassing CORS) and displays the response |
| 2.5 | Export to OpenAPI 3.0 | One-click export of the full API map as an `openapi.yaml` file |
| 2.6 | Export to Postman Collection | One-click export as `postman_collection.json` |
| 2.7 | VaultWare theme tokens | All colours come from `vault-themes/theme_manager.py`-equivalent JS tokens; no hardcoded values |

---

## Phase 3 — JS Deobfuscator

**Goal:** Transform webpack bundles and minified code into structured, readable JavaScript.

| # | Deliverable | Description |
|---|---|---|
| 3.1 | Bundle inventory | Asset list panel showing all downloaded JS files with size, hash, and estimated bundle type |
| 3.2 | Beautifier pipeline | `js-beautify` + `prettier` pass; normalises indentation, line breaks, and quote style |
| 3.3 | Babel AST transforms | Dead-code elimination, constant folding, string concatenation resolution, hex/unicode literal decoding |
| 3.4 | Webpack chunk splitter | Detects webpack module factory patterns and splits the bundle back into logical module boundaries |
| 3.5 | Source-map resolver | If source maps are publicly available (`.map` files), fetches and applies them automatically |
| 3.6 | Side-by-side viewer | `CodeViewer` component showing original (left) and beautified (right) with syntax highlighting |
| 3.7 | Diff highlighting | Highlights the delta between original and transformed to show exactly what changed |

---

## Phase 4 — AI Code Reconstructor

**Goal:** Use a locally running language model to rename obfuscated identifiers to semantically meaningful names.

| # | Deliverable | Description |
|---|---|---|
| 4.1 | Ollama integration | `local-model.js` client connects to the Ollama REST API on `localhost:11434` |
| 4.2 | llama.cpp fallback | If Ollama is not available, falls back to a llama.cpp OpenAI-compatible server on `localhost:8080` |
| 4.3 | Chunked analysis | Splits large files into semantic chunks (≤ 4k tokens) preserving scope boundaries |
| 4.4 | Prompt engineering | System prompt instructs the model to return a JSON rename map `{ "a": "userAuthToken", ... }` |
| 4.5 | AST rename pass | Applies the rename map through Babel's `@babel/traverse` for referentially consistent renaming |
| 4.6 | Confidence scoring | Model returns a 0–1 confidence per rename; low-confidence names are flagged for manual review |
| 4.7 | Human-review panel | Side-by-side diff of AI-renamed vs. beautified, with inline accept/reject per rename |
| 4.8 | Agent pipeline | `AiRenamerAgent` extends `ExtrovertAgent`; chunked rename jobs are queued and monitored via Redis |

---

## Phase 5 — Post-Quantum Vault

**Goal:** Encrypt all sensitive session data (headers, tokens, intercepted payloads) using PQC key encapsulation.

| # | Deliverable | Description |
|---|---|---|
| 5.1 | ML-KEM (Kyber) KEM | Implement or bind `ml-kem` npm package for key encapsulation |
| 5.2 | AES-256-GCM symmetric layer | Wrap Kyber-encapsulated symmetric key with AES-256-GCM for bulk encryption |
| 5.3 | Encrypted session vault | `vault.js` — read/write encrypted JSON blobs to `~/.vaultwares-decompile/vault/` |
| 5.4 | Key rotation | On each session start, generate a fresh KEM keypair; old sessions retain their own keys |
| 5.5 | Memory hygiene | Sensitive buffers are zeroed after use using `Buffer.fill(0)` |
| 5.6 | Vault UI | Settings panel showing vault status, key fingerprint, and manual clear option |

---

## Phase 6 — Electron Shell & UX Polish

**Goal:** Deliver a complete, branded Electron desktop app with excellent UX.

| # | Deliverable | Description |
|---|---|---|
| 6.1 | Window management | Multi-window support: main app + detached code viewer windows |
| 6.2 | Tray icon | System tray with quick-start crawl action |
| 6.3 | Auto-update | `electron-updater` integration for silent background updates |
| 6.4 | Onboarding wizard | First-run wizard: enter target URL, check Ollama availability, select VaultWare theme |
| 6.5 | Keyboard shortcuts | Full keyboard navigation; crawl, replay request, export — all accessible without mouse |
| 6.6 | Accessibility | WCAG AA contrast targets enforced via `vault-themes` contrast checker |
| 6.7 | Packaging | `electron-builder` produces `.dmg`, `.exe`, and `.AppImage` distributable artifacts |

---

## Phase 7 — Testing & Hardening

**Goal:** Ensure correctness, security, and performance under adversarial inputs.

| # | Deliverable | Description |
|---|---|---|
| 7.1 | Unit tests | Jest tests for crawler route normalisation, AST transforms, rename prompt builder |
| 7.2 | Integration tests | End-to-end crawl against a local test server (Express mock) verifying route capture |
| 7.3 | Security audit | `npm audit` + `electron-builder` CSP header review; no `eval`, no `nodeIntegration` |
| 7.4 | PQC test vectors | Run ML-KEM against NIST FIPS 203 test vectors to verify correctness |
| 7.5 | Performance profiling | Crawl of a 100-page SPA must complete in < 60 s on modest hardware |
| 7.6 | Fuzzing | AFL++ / Radamsa fuzz the route normaliser and AST parser inputs |

---

## Phase 8 — Documentation & Release

| # | Deliverable | Description |
|---|---|---|
| 8.1 | User guide | Markdown docs covering installation, usage, and ethical use guidelines |
| 8.2 | Developer guide | Architecture decision records (ADRs), module API reference |
| 8.3 | Security disclosure policy | `SECURITY.md` with responsible disclosure process |
| 8.4 | Changelog | Conventional-commits-based `CHANGELOG.md` |
| 8.5 | v1.0.0 release | GitHub release with signed binaries |

---

## Milestone Schedule (Indicative)

| Milestone | Phase(s) | Target |
|---|---|---|
| M0 — Scaffold | 0 | Week 1 |
| M1 — Crawler MVP | 1 | Week 2–3 |
| M2 — API Explorer MVP | 2 | Week 3–4 |
| M3 — Deobfuscator MVP | 3 | Week 4–5 |
| M4 — AI Rename MVP | 4 | Week 5–7 |
| M5 — PQC Vault | 5 | Week 7–8 |
| M6 — UX Polish | 6 | Week 8–9 |
| M7 — Testing & Hardening | 7 | Week 9–10 |
| M8 — Release | 8 | Week 11 |

---

*This roadmap is maintained by the LonelyManager agent. Task-level tracking lives in [`TODO.md`](TODO.md). Sub-agent dispatch instructions are in [`TASKS.md`](TASKS.md).*
