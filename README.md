# Decompile — by VaultWares

> **A tool to rebuild any App, code-agnosticly, with only publicly available information.**

## Quick step-by-step:
1. Build front-end code, extract business logic, infer some things left or right, voilà.
---

## What Is This?

Counterpart to Columbo, this takes the human psychology element out of the equation and focuses on immediate results.

Given a target URL, Decompile will:

1. Reconstruct your application.

---

## Architecture Overview

```
VaultWares Decompile/
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
│   └── agents/                 # vaultwares-adk integration
│       ├── crawler-agent.js
│       ├── deobfuscator-agent.js
│       └── ai-renamer-agent.js
├── vaultwares-themes/               # Git submodule — VaultWare visual theme system
├── vaultwares-adk/    # Git submodule — Multi-agent coordination framework
├── ROADMAP.md
├── TODO.md
├── TASKS.md
├── AGENTS.md
├── package.json
└── .env.example
```
## Submodules

| Submodule | Purpose |
|---|---|
| [`vaultwares-themes`](https://github.com/p-potvin/vaultwares-themes) | VaultWare theme tokens, contrast checker, and Figma-to-code rules |
| [`vaultwares-adk`](https://github.com/p-potvin/vaultwares-adk) | Multi-agent coordination via Redis — heartbeat, status, task dispatch |

---

## Contributing & Agent Dispatch

See [`TASKS.md`](TASKS.md) for a breakdown of work items ready to be dispatched to parallel sub-agents.
See [`ROADMAP.md`](ROADMAP.md) for the full phased development plan.
See [`TODO.md`](TODO.md) for the granular task checklist.

---

*VaultWares Decompile is a VaultWares research tool. It is intended for authorized security testing and educational use only.*
