# VaultWares Decompile — TODO

> Tracked by LonelyManager · Sources: ROADMAP.md · Updated: 2026-04-20

---

## Phase 0 — Project Scaffold

- [x] Add `vaultwares-themes` as git submodule
- [x] Add `vaultwares-adk` as git submodule
- [x] Write `ROADMAP.md`
- [x] Write `TODO.md`
- [x] Write `TASKS.md`
- [x] Write `AGENTS.md`
- [x] Update `README.md` with VaultWare branding
- [ ] Create `package.json` with all required dependencies
- [ ] Create `src/` directory scaffold (main, renderer, crawler, deobfuscator, ai, crypto, agents)
- [ ] Create `.env.example` with all configuration keys
- [ ] Create module interface contracts (`@typedef` JSDoc blocks) in each module index
- [ ] Wire `electron-builder` configuration in `package.json`
- [ ] Create `vite.config.js` for renderer build
- [ ] Create base `index.html` for renderer entry point

---

## Theme Migration — Solarized Warm Light

- [x] Create `src/renderer/theme.js` with centralised theme tokens (Solarized Warm Light default, Solarized Dark, Cyberpunk Cinder)
- [x] Update `src/renderer/App.jsx` to import from theme module; add theme switcher
- [x] Update `src/main/index.js` BrowserWindow backgroundColor to `#FDF6E3`
- [x] Update `AGENTS.md` VaultWare Branding section for new design philosophy
- [x] Update `TODO.md` with theme migration items
- [x] Add TASK-11 to `TASKS.md` for theme migration task spec

---

## Phase 1 — Crawler Engine

- [ ] **1.1** Set up `PlaywrightCrawler` in `src/crawler/crawler.js`
  - [ ] Configure launch options (headless, sandboxed)
  - [ ] Configure maximum concurrency and request queue
  - [ ] Handle SPA navigation events (`pushState`, `hashchange`)
  - [ ] Follow `<a>` links, `fetch()` calls, and `XMLHttpRequest` calls
- [ ] **1.2** Build request interceptor in `src/crawler/api-interceptor.js`
  - [ ] Hook `page.on('request')` to capture all outgoing calls
  - [ ] Capture: method, URL, headers (redacted auth values), request body
  - [ ] Capture response status, content-type, and a truncated response sample
  - [ ] Filter out asset requests (images, fonts, CSS) — keep only XHR/fetch/GraphQL
- [ ] **1.3** Build route normaliser in `src/crawler/route-mapper.js`
  - [ ] Detect numeric IDs (`/user/123` → `/user/:id`)
  - [ ] Detect UUIDs (`/item/550e...` → `/item/:uuid`)
  - [ ] Detect slugs (`/post/my-great-post` → `/post/:slug`)
  - [ ] Deduplicate normalised routes
  - [ ] Group routes by base path and HTTP method
- [ ] **1.4** Build JS asset harvester in `src/crawler/js-downloader.js`
  - [ ] Detect `<script src>` tags and dynamically injected scripts
  - [ ] Download and store to `~/.vaultwares-decompile/assets/<session-id>/`
  - [ ] Compute and store SHA-256 fingerprint per asset
  - [ ] Detect and log webpack chunk naming patterns
- [ ] **1.5** Implement crawl session persistence
  - [ ] Serialise crawl result to JSON
  - [ ] Encrypt and write to vault (Phase 5 dependency — stub for now)
  - [ ] Implement `loadSession(id)` and `listSessions()` helpers
- [ ] **1.6** IPC event bridge
  - [ ] Define event schema in `src/main/ipc-handlers.js`
  - [ ] Emit `crawl:start` with target URL and timestamp
  - [ ] Emit `crawl:route-found` with route object
  - [ ] Emit `crawl:asset-downloaded` with asset metadata
  - [ ] Emit `crawl:complete` with summary stats
  - [ ] Emit `crawl:error` with error details and partial results
- [ ] **1.7** `CrawlerAgent` class in `src/agents/crawler-agent.js`
  - [ ] Extend `ExtrovertAgent` from `vaultwares-adk`
  - [ ] Register `start_crawl` and `stop_crawl` task handlers
  - [ ] Broadcast status updates every 30 seconds during active crawl

---

## Phase 2 — API Explorer

- [ ] **2.1** Define route data model (`RouteSchema`) in `src/types/route.js`
  - [ ] Fields: `id`, `method`, `url`, `pattern`, `pathParams`, `queryParams`, `headers`, `body`, `responseSchema`, `responseSample`
- [ ] **2.2** Build `ApiExplorerView.jsx`
  - [ ] Route list panel (left): grouped by base path, filterable by method and keyword
  - [ ] Detail panel (right): shows selected route's full request spec
  - [ ] Empty state when no crawl has been run
- [ ] **2.3** Build `RequestEditor.jsx`
  - [ ] Method selector (dropdown: GET, POST, PUT, PATCH, DELETE, OPTIONS)
  - [ ] URL bar with template variable substitution (`/user/:id` → input field per param)
  - [ ] Header grid: key-value pairs, add/remove rows, toggle visibility
  - [ ] Body editor: tab between JSON, form-data, and raw; JSON editor with validation
  - [ ] Send button → IPC call to main process → display response
- [ ] **2.4** Implement live request replay in `src/main/ipc-handlers.js`
  - [ ] `request:replay` handler using Node.js `https` module (no CORS)
  - [ ] Return `{ status, headers, body, durationMs }`
  - [ ] Respect redirect chains; cap at 5 redirects
- [ ] **2.5** OpenAPI 3.0 exporter in `src/crawler/route-mapper.js`
  - [ ] Build `openapi.yaml` from route map
  - [ ] Infer parameter types from captured samples
  - [ ] Write to user-selected file path via `dialog.showSaveDialog`
- [ ] **2.6** Postman collection exporter
  - [ ] Build `postman_collection.json` (v2.1 schema) from route map
  - [ ] Write to user-selected file path
- [ ] **2.7** Apply `vaultwares-themes` tokens throughout API Explorer UI
  - [ ] Import theme token helper from `vaultwares-themes/theme_manager.py` JS port
  - [ ] No hardcoded colour values in any component

---

## Phase 3 — JS Deobfuscator

- [ ] **3.1** Build asset inventory panel in `JsDeobfuscatorView.jsx`
  - [ ] List all downloaded JS assets (file name, size, SHA-256 fingerprint)
  - [ ] Allow manual import of JS files not captured by crawler
  - [ ] Show bundle-type badge (webpack, rollup, parcel, ESM, unknown)
- [ ] **3.2** Implement beautifier pipeline in `src/deobfuscator/beautifier.js`
  - [ ] Run `js-beautify` as first pass (indentation normalisation)
  - [ ] Run `prettier` as second pass (consistent formatting)
  - [ ] Handle parse errors gracefully — fall back to partial output
- [ ] **3.3** Implement Babel AST transforms in `src/deobfuscator/ast-transformer.js`
  - [ ] Hex literal → decimal / string (`0x68656c6c6f` → `"hello"`)
  - [ ] Unicode escape sequences → UTF-8 literals
  - [ ] Constant folding (`1 + 2` → `3`, `"a" + "b"` → `"ab"`)
  - [ ] Dead-code elimination (always-false conditions)
  - [ ] Sequence expression expansion (`a = 1, b = 2` → separate statements)
  - [ ] Boolean literal restoration (`!0` → `true`, `!1` → `false`)
  - [ ] `typeof undefined === "undefined"` guard removal
- [ ] **3.4** Implement webpack chunk splitter in `src/deobfuscator/module-splitter.js`
  - [ ] Detect webpack IIFE / factory pattern (`(function(modules){...})({...})`)
  - [ ] Extract individual module functions keyed by module ID
  - [ ] Map module IDs to output files when chunk manifests are available
- [ ] **3.5** Source-map resolver
  - [ ] Check `//# sourceMappingURL=` comment in each file
  - [ ] Fetch the `.map` file if URL is public
  - [ ] Apply source map using `source-map` npm package
  - [ ] Display original file names and line numbers in the viewer
- [ ] **3.6** `CodeViewer.jsx` side-by-side component
  - [ ] Left pane: original minified code (read-only)
  - [ ] Right pane: beautified / transformed code (read-only)
  - [ ] Syntax highlighting via `shiki` or `prism-react-renderer`
  - [ ] Synchronised vertical scroll between panes
  - [ ] Line-count and byte-count stats in the panel header
- [ ] **3.7** Diff highlighting
  - [ ] Compute diff between original and transformed using `diff` npm package
  - [ ] Highlight added lines (green) and removed lines (red) in gutter

---

## Phase 4 — AI Code Reconstructor

- [ ] **4.1** Build Ollama client in `src/ai/local-model.js`
  - [ ] `GET /api/tags` to verify connection and list available models
  - [ ] `POST /api/generate` with streaming response
  - [ ] Model selector UI: dropdown populated from available Ollama models
- [ ] **4.2** Build llama.cpp fallback
  - [ ] `POST /v1/completions` (OpenAI-compatible endpoint at `localhost:8080`)
  - [ ] Auto-detect which backend is available on startup
- [ ] **4.3** Implement chunked file analysis in `src/ai/renamer-agent.js`
  - [ ] Estimate token count per function/class block using character heuristic
  - [ ] Split file into semantic chunks ≤ 4096 tokens
  - [ ] Preserve scope boundaries — never split inside a function body
- [ ] **4.4** Build prompt templates in `src/ai/prompt-builder.js`
  - [ ] System prompt: instructs model to return JSON rename map only
  - [ ] User prompt: includes code chunk + existing identifier list
  - [ ] Few-shot examples: 3 reference examples showing obfuscated → readable renames
  - [ ] Anti-hallucination guard: instruct model to leave identifiers unchanged if unsure
- [ ] **4.5** Implement AST rename pass in `src/deobfuscator/ast-transformer.js`
  - [ ] Accept rename map `{ "a": "userAuthToken", ... }`
  - [ ] Apply renames consistently across all scopes using `@babel/traverse`
  - [ ] Avoid renaming reserved words or globally-defined identifiers
- [ ] **4.6** Confidence scoring
  - [ ] Parse model response for optional `"confidence": 0.85` fields
  - [ ] Flag renames below threshold (default 0.7) in the UI with a warning badge
- [ ] **4.7** Human-review panel in `JsDeobfuscatorView.jsx`
  - [ ] List all proposed renames with original name, proposed name, and confidence
  - [ ] Accept / reject individual renames with a toggle
  - [ ] "Apply All" and "Apply High-Confidence" bulk actions
  - [ ] Re-run AI pass on rejected items with user-supplied context hint
- [ ] **4.8** `AiRenamerAgent` class in `src/agents/ai-renamer-agent.js`
  - [ ] Extend `ExtrovertAgent`
  - [ ] Queue rename jobs per chunk to Redis
  - [ ] Broadcast progress (`chunk X of Y complete`)

---

## Phase 5 — Post-Quantum Vault

- [ ] **5.1** Integrate `mlkem` npm package in `src/crypto/kem.js`
  - [ ] `generateKeypair()` → `{ publicKey, privateKey }`
  - [ ] `encapsulate(publicKey)` → `{ ciphertext, sharedSecret }`
  - [ ] `decapsulate(ciphertext, privateKey)` → `sharedSecret`
- [ ] **5.2** AES-256-GCM wrapper in `src/crypto/kem.js`
  - [ ] `encrypt(plaintext, sharedSecret)` → `{ iv, ciphertext, authTag }`
  - [ ] `decrypt({ iv, ciphertext, authTag }, sharedSecret)` → `plaintext`
- [ ] **5.3** Vault I/O in `src/crypto/vault.js`
  - [ ] Session directory: `~/.vaultwares-decompile/vault/<session-id>/`
  - [ ] `writeVault(sessionId, data)` — serialise, encrypt, write
  - [ ] `readVault(sessionId)` — read, decrypt, deserialise
  - [ ] `listVaults()` — return metadata for all saved sessions
- [ ] **5.4** Key rotation
  - [ ] Generate new KEM keypair on each app launch
  - [ ] Re-encrypt any "keep" sessions with the new public key on load
- [ ] **5.5** Memory hygiene
  - [ ] Zero private key buffer immediately after `decapsulate`
  - [ ] Zero shared secret buffer after AES key derivation
  - [ ] Clear sensitive IPC payloads from `webContents` memory after transmission
- [ ] **5.6** Vault status UI in Settings panel
  - [ ] Show current key fingerprint (SHA-256 of public key, first 12 hex chars)
  - [ ] Show number of saved sessions and total vault size
  - [ ] "Clear Vault" button with confirmation modal

---

## Phase 6 — Electron Shell & UX

- [ ] **6.1** Multi-window support
  - [ ] Main window: `BrowserWindow` with 1280×800 minimum size
  - [ ] Detachable code viewer: secondary `BrowserWindow` for side-by-side analysis
  - [ ] `windowState` persistence (position, size, last-opened view)
- [ ] **6.2** System tray icon
  - [ ] Show/hide main window on click
  - [ ] Context menu: "New Crawl", "Open Session", "Quit"
- [ ] **6.3** Auto-update via `electron-updater`
  - [ ] Check for updates on launch (background, non-blocking)
  - [ ] Prompt user when update is ready: "Restart to Update"
- [ ] **6.4** Onboarding wizard (first-run only)
  - [ ] Step 1: Enter target URL (with `https://` enforcement)
  - [ ] Step 2: Detect Ollama / llama.cpp availability; link to installation guide if missing
  - [ ] Step 3: Select VaultWare theme (theme preview with contrast badge)
  - [ ] Step 4: Review privacy statement (locally-only processing)
- [ ] **6.5** Keyboard shortcuts
  - [ ] `Ctrl+N` / `Cmd+N` — New crawl
  - [ ] `Ctrl+R` / `Cmd+R` — Replay selected request
  - [ ] `Ctrl+E` / `Cmd+E` — Export (OpenAPI / Postman)
  - [ ] `Ctrl+,` / `Cmd+,` — Open settings
  - [ ] `Esc` — Cancel active crawl
- [ ] **6.6** Accessibility
  - [ ] Run `axe-core` audit on all views
  - [ ] Fix all critical / serious violations
  - [ ] Verify contrast ratios using `vaultwares-themes` `check_contrast` helper
- [ ] **6.7** Packaging
  - [ ] `electron-builder` config for macOS (`.dmg` + code signing)
  - [ ] `electron-builder` config for Windows (`.exe` NSIS installer)
  - [ ] `electron-builder` config for Linux (`.AppImage`)
  - [ ] GitHub Actions release workflow

---

## Phase 7 — Testing & Hardening

- [ ] **7.1** Unit tests (Jest)
  - [ ] Route normaliser: 30+ test cases for URL pattern detection
  - [ ] AST transforms: verify each transform with known input/output pairs
  - [ ] Prompt builder: snapshot tests for generated prompts
  - [ ] KEM/AES: encrypt → decrypt roundtrip correctness
- [ ] **7.2** Integration tests
  - [ ] Spin up Express mock server with 20 API routes
  - [ ] Run full crawl against it; assert captured route count and patterns
  - [ ] Replay 5 captured routes; assert response status correctness
- [ ] **7.3** Security hardening
  - [ ] `npm audit` — zero high/critical vulnerabilities
  - [ ] Electron CSP: `Content-Security-Policy` header in main window
  - [ ] No `eval` in renderer; ESLint `no-eval` rule enforced
  - [ ] `contextIsolation: true`, `nodeIntegration: false`, `sandbox: true`
- [ ] **7.4** PQC test vectors
  - [ ] Run ML-KEM against NIST FIPS 203 Known Answer Tests (KATs)
  - [ ] Assert all 100 vectors pass
- [ ] **7.5** Performance profiling
  - [ ] Crawl benchmark: 100-page SPA < 60 s on 4-core / 8 GB machine
  - [ ] Deobfuscator benchmark: 500 KB bundle transformed in < 5 s
  - [ ] AI rename benchmark: single 4k-token chunk < 30 s on CPU-only Ollama
- [ ] **7.6** Fuzzing
  - [ ] Fuzz route normaliser with Radamsa-generated URLs
  - [ ] Fuzz AST parser with malformed JS inputs (must not crash, must return error)

---

## Phase 8 — Documentation & Release

- [ ] **8.1** User guide (`docs/user-guide.md`)
- [ ] **8.2** Developer guide (`docs/developer-guide.md`) + ADR directory (`docs/adr/`)
- [ ] **8.3** `SECURITY.md` with responsible disclosure policy
- [ ] **8.4** `CHANGELOG.md` (conventional commits format)
- [ ] **8.5** v1.0.0 GitHub release with signed binaries and checksums

---

*This TODO is the live task board. Check items off as work is completed.*
