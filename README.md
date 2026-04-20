# Deconstructed — by VaultWares

> **A privacy-first security research tool for API mapping, JS deobfuscation, and intelligent code reconstruction.**

![VaultWares](https://img.shields.io/badge/VaultWares-Privacy%20First-CB4B16?style=flat-square)
![License](https://img.shields.io/badge/License-MIT-073642?style=flat-square)
![Status](https://img.shields.io/badge/Status-In%20Development-D4AF37?style=flat-square)
![PQC](https://img.shields.io/badge/Encryption-Post--Quantum-800020?style=flat-square)

---

## What Is This?

Many companies attempt to protect their frontend logic through minification, obfuscation, and webpack bundling. **Deconstructed** is a research tool that demonstrates how (un)effective those techniques actually are.

Given a target URL, Deconstructed will:

1. **Crawl & Map** — Use the [Crawlee](https://crawlee.dev/js/docs/quick-start) library to spider the site's routes, intercept all network requests, and build a live API map.
2. **API Explorer** — Present every discovered route in a Swagger/Postman-style panel with HTTP headers and request payloads already prefilled, so you can test each endpoint without manual setup.
3. **JS Deobfuscator** — Download minified/obfuscated JavaScript bundles, beautify them, and render the human-readable source side-by-side with the original.
4. **AI Reconstructor** — Feed the beautified code to a local AI model (no third-party API calls) that renames variables, functions, and parameters to semantically meaningful identifiers — producing code that may be more readable than the original.

---

## VaultWares Philosophy

| Principle | What It Means Here |
|---|---|
| 🔒 **Privacy First** | All crawling, analysis, and AI inference run locally. Nothing leaves your machine. |
| 🛡️ **Security Second** | Encryption is applied where necessary; it does not slow down the primary workflow. |
| ⚛️ **Post-Quantum Cryptography** | Sensitive data (session tokens, local vault keys) use KEM-based key encapsulation — ready for a post-quantum world. |
| 🤖 **Agentic Architecture** | Internal processing pipelines are built on the `vaultwares-agentciation` multi-agent framework — observable, restartable, and auditable. |

---

## Architecture Overview

```
deconstructed/
├── src/
│   ├── main/                   # Electron main process
│   │   ├── index.js            # App entry point
│   │   ├── ipc-handlers.js     # IPC bridge (renderer ↔ main)
│   │   └── preload.js          # Context-isolated preload script
│   ├── crawler/                # Crawlee-based site spider
│   │   ├── crawler.js          # Playwright crawler setup
│   │   ├── api-interceptor.js  # XHR / fetch request logger
│   │   ├── route-mapper.js     # Aggregates intercepted routes into API map
│   │   └── js-downloader.js    # Downloads and stores minified JS assets
│   ├── deobfuscator/           # JS beautification pipeline
│   │   ├── beautifier.js       # Prettier / js-beautify wrapper
│   │   ├── ast-transformer.js  # Babel AST transforms (dead-code removal, unminify)
│   │   └── module-splitter.js  # Splits webpack bundles back into logical modules
│   ├── ai/                     # Local AI inference for code renaming
│   │   ├── renamer-agent.js    # Orchestrates AI calls for semantic renaming
│   │   ├── prompt-builder.js   # Constructs code-analysis prompts
│   │   └── local-model.js      # Ollama / llama.cpp client (no 3rd-party APIs)
│   ├── crypto/                 # Post-Quantum Cryptography utilities
│   │   ├── kem.js              # Key Encapsulation Mechanism (ML-KEM / Kyber)
│   │   └── vault.js            # Encrypted local storage for session data
│   ├── renderer/               # Electron renderer (React + VaultWare themes)
│   │   ├── App.jsx
│   │   ├── views/
│   │   │   ├── CrawlerView.jsx         # Target URL input + progress + route list
│   │   │   ├── ApiExplorerView.jsx     # Swagger-style panel per route
│   │   │   └── JsDeobfuscatorView.jsx  # Side-by-side JS viewer + AI rename panel
│   │   └── components/
│   │       ├── RouteCard.jsx
│   │       ├── RequestEditor.jsx
│   │       └── CodeViewer.jsx
│   └── agents/                 # vaultwares-agentciation integration
│       ├── crawler-agent.js
│       ├── deobfuscator-agent.js
│       └── ai-renamer-agent.js
├── vault-themes/               # Git submodule — VaultWare visual theme system
├── vaultwares-agentciation/    # Git submodule — Multi-agent coordination framework
├── ROADMAP.md
├── TODO.md
├── TASKS.md
├── AGENTS.md
├── package.json
└── .env.example
```

---

## Quick Start

```bash
# 1. Clone with submodules
git clone --recurse-submodules https://github.com/p-potvin/deconstructed-website-a-la-mode
cd deconstructed-website-a-la-mode

# 2. Install dependencies
npm install

# 3. Copy and configure environment
cp .env.example .env

# 4. Start the Electron app
npm start
```

### Requirements

| Dependency | Version | Purpose |
|---|---|---|
| Node.js | ≥ 20 | Runtime |
| Electron | ≥ 30 | Desktop GUI framework |
| Crawlee | ≥ 3 | Web crawler / route mapper |
| Playwright | ≥ 1.44 | Browser automation (via Crawlee) |
| Ollama (optional) | latest | Local AI inference for code renaming |
| Redis | ≥ 7 | Agent coordination (vaultwares-agentciation) |

---

## Security & Privacy

- **No telemetry.** Zero data is sent to external servers.
- **Local AI only.** The AI code-renaming engine calls Ollama or a llama.cpp server running on your machine.
- **PQC-ready vault.** Session data (intercepted headers, credentials) is encrypted with ML-KEM (CRYSTALS-Kyber) key encapsulation and AES-256-GCM symmetric encryption.
- **Context isolation.** The Electron renderer runs with `contextIsolation: true` and `nodeIntegration: false`. All privileged operations go through the IPC bridge.

---

## Submodules

| Submodule | Purpose |
|---|---|
| [`vault-themes`](https://github.com/p-potvin/vault-themes) | VaultWare theme tokens, contrast checker, and Figma-to-code rules |
| [`vaultwares-agentciation`](https://github.com/p-potvin/vaultwares-agentciation) | Multi-agent coordination via Redis — heartbeat, status, task dispatch |

---

## Contributing & Agent Dispatch

See [`TASKS.md`](TASKS.md) for a breakdown of work items ready to be dispatched to parallel sub-agents.
See [`ROADMAP.md`](ROADMAP.md) for the full phased development plan.
See [`TODO.md`](TODO.md) for the granular task checklist.

---

*Deconstructed is a VaultWares research tool. It is intended for authorized security testing and educational use only.*
