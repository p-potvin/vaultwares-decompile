# AGENTS.md — VaultWares Decompile

> For company-wide rules, read `vaultwares-docs/AGENTS.md` first.

## Project Identity

**VaultWares Decompile** is a VaultWares security-research desktop tool built with Electron (main process) and React (renderer). It crawls websites, maps APIs, deobfuscates JavaScript, and uses local AI to reconstruct human-readable code.

---

## Submodule Rules

Two submodules are present:

| Path | Purpose |
|---|---|
| `vaultwares-themes/` | VaultWare theme tokens, contrast checker, `VaultThemeManager` |
| `vaultwares-adk/` | Multi-agent coordination framework (Redis, heartbeat, task dispatch) |

- **Never modify files inside submodule directories.** Treat them as read-only dependencies.
- When importing theme tokens, use the JavaScript equivalent of `VaultThemeManager.export_theme_tokens()`.
- When adding agent functionality, extend `ExtrovertAgent` from `vaultwares-adk`.

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

- All colours must use theme tokens from `vaultwares-themes`. No hardcoded hex values.
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
5. **Broadcast status** via the `vaultwares-adk` framework if you are running as an autonomous agent (Redis must be available).

---

## VaultWare Branding in UI

### Design Philosophy

VaultWares Decompile uses the **new VaultWares brand language** defined in `vaultwares-themes/Brand/`. The visual identity emphasises:

- **Light-first design** — warm, paper-toned backgrounds for long reading sessions during security research.
- **Solarized palette** — derived from `codex-solarized-light-revisited.json` with VaultWares gold accent.
- **Dark themes available** — Solarized Dark and Cyberpunk Cinder remain selectable for users who prefer darker UIs.
- **Brand typography** — `"Segoe UI Semilight"`, `Inter`, `system-ui` (from `vaultwares-themes/Brand/tokens.ts`).
- **Tagline** — "Privacy first. Security in service." (from `vaultwares-themes/Brand/brand.i18n.ts`).

### Default Theme: Solarized Warm Light (`solarized-warm-light`)

| Token | Value | Source |
|---|---|---|
| `background` | `#FDF6E3` | Solarized base3 |
| `surface` | `#EEE8D5` | Solarized base2 |
| `accent` | `#CC9B21` | VaultWares gold (`tokens.ts`) |
| `textPrimary` | `#002B36` | Solarized base03 / ink |
| `textSecondary` | `#657B83` | Solarized base00 |

### Available Dark Themes

| Theme | Slug | Accent |
|---|---|---|
| Solarized Dark | `solarized-dark` | `#B58900` (Solarized yellow) |
| Cyberpunk Cinder | `cyberpunk-cinder` | `#CB4B16` (Solarized orange) |

### Theme Usage

All theme tokens are centralised in `src/renderer/theme.js`. Use the following API:

```js
import { getTheme, listThemes, DEFAULT_THEME_SLUG, FONT_FAMILY } from './theme.js';

const theme = getTheme('solarized-warm-light');
// theme.background, theme.accent, theme.textPrimary, etc.
```

- Never hardcode hex colour values — always reference a theme token.
- The `VaultThemeManager` in `vaultwares-themes/theme_manager.py` remains the Python source of truth; `src/renderer/theme.js` is the JS equivalent for this project.
- When the user switches themes at runtime, the entire UI recolours via React state.

The app name in all UI labels is **"VaultWares Decompile"** — not "VaultWares Decompile-website-a-la-mode".

<!-- VAULT-THEMES-SUBMODULE:START -->
## vaultwares-themes Submodule

Before UI, branding, or token work, read:
- `vaultwares-themes/AGENTS.md`
- `vaultwares-themes/CONTEXT.md`
<!-- VAULT-THEMES-SUBMODULE:END -->

# TODO: implement new theme-manager from vaultwares-themes
