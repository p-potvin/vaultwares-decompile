# panopticam — Clean-Room Architecture Blueprint

- **Target:** `C:\Users\Administrator\Desktop\Prom-King\panopticam`
- **Extracted:** `Thu, 15 Oct 2026 03:55`
- **Detected Stack:** FastAPI · JavaScript/TypeScript · OpenCV / Vision · PyTorch / CUDA · Python · Python http.server · SQLite · Vector Embeddings
- **Metrics:** 166 files scanned · 1827 proprietary slices · 105 API routes · 229 data models · 23 state stores · 5 feature clusters

## Executive Summary (Gemma 4 E2B Synthesis)

**Executive Architecture Summary: Panopticam System**

The Panopticam system is architected as a high-performance, GPU-accelerated computer vision pipeline designed to process complex visual data. The core architecture is built upon a decoupled microservices approach, utilizing a robust Python/FastAPI backend for API management, a dedicated Vision worker leveraging PyTorch and CUDA for intensive image processing, and a specialized Embedding service utilizing Gemma 2 for contextual representation. This separation ensures that the computationally expensive vision tasks are isolated from the API layer, while the Embedding service provides a dedicated, scalable mechanism for generating and managing vector embeddings, which are crucial for the system's intelligence and retrieval capabilities.

The system's operational mechanism is driven by a clear service interaction flow: the Web subsystem provides the user interface, which communicates with the Backend API to initiate processing requests. These requests are then routed to the Vision worker, which executes the heavy lifting using the workstation's GPU resources. Concurrently, the Embed service handles the generation of vector embeddings, feeding contextual data back into the pipeline. This architecture is fundamentally designed for high-throughput, real-time visual analysis, where the synergy between the FastAPI control plane, the GPU-bound vision engine, and the AI-driven embedding service allows for scalable, intelligent data processing.

## Discovered Services & Entrypoints

| Service | Kind | Bind / Endpoint | Entrypoint | Purpose |
|---|---|---|---|---|
| `backend-src-panopticam-app` | `http-service` | `–` | `backend/src/panopticam/app.py` | HTTP API and static web app. |
| `embed-server` | `embedding-service` | `–` | `embed/server.py` | EmbeddingGemma 2 as a resident HTTP service on the workstation (9 Oct 2026). |
| `vision-worker` | `background-worker` | `–` | `vision/worker.py` | Panopticam vision worker — runs on the workstation's GPU (RTX 3060). |

## Feature Clusters (Semantic Domains)

| Cluster | File | Slices | Routes | Models |
|---|---|---|---|---|
| **Backend** | `features/01-backend.md` | 1115 | 98 | 124 |
| **Deploy** | `features/02-deploy.md` | 1 | 0 | 0 |
| **Embed** | `features/03-embed.md` | 16 | 0 | 1 |
| **Vision** | `features/04-vision.md` | 702 | 7 | 55 |
| **Web** | `features/05-web.md` | 284 | 0 | 49 |

## API Route Summary (`openapi.json`)

| Method | Pattern | Source | Linked Slices |
|---|---|---|---|
| `GET` | `/api/director` | `backend/src/panopticam/app.py:401` | embed/server.py:do_GET, web/src/api.ts:f4fHere, embed/server.py:main, vision/browser.py:main |
| `POST` | `/api/vision/taste` | `backend/src/panopticam/app.py:418` | embed/server.py:do_POST, backend/src/panopticam/actions.py:create, embed/server.py:do_GET, vision/datasets.py:process |
| `GET` | `/api/taste` | `backend/src/panopticam/app.py:429` | embed/server.py:do_GET, web/src/app.tsx:Toasts, web/src/api.ts:f4fHere, embed/server.py:do_POST |
| `GET` | `/api/moments` | `backend/src/panopticam/app.py:439` | embed/server.py:do_GET, backend/src/panopticam/actions.py:pinned_hits, vision/browser.py:main, embed/server.py:main |
| `GET` | `/api/moments/{mid}/similar` | `backend/src/panopticam/app.py:446` | embed/server.py:do_GET, backend/src/panopticam/actions.py:pinned_hits, embed/server.py:main, embed/server.py:do_POST |
| `GET` | `/api/moments/{mid}/{part}` | `backend/src/panopticam/app.py:454` | embed/server.py:do_GET, backend/src/panopticam/actions.py:pinned_hits, vision/browser.py:main, embed/server.py:main |
| `POST` | `/api/moments/{mid}/keep` | `backend/src/panopticam/app.py:468` | backend/src/panopticam/actions.py:pinned_hits, web/src/components/Alerts.tsx:save, embed/server.py:do_GET, embed/server.py:main |
| `POST` | `/api/moments/{mid}/rate` | `backend/src/panopticam/app.py:479` | embed/server.py:do_POST, embed/server.py:do_GET, embed/server.py:main, backend/src/panopticam/actions.py:label_target |
| `POST` | `/api/vision/moments/{mid}/life` | `backend/src/panopticam/app.py:494` | vision/browser.py:main, backend/src/panopticam/actions.py:live_count, vision/experiments/eg2_embed.py:main, embed/server.py:do_GET |
| `POST` | `/api/vision/moments/vecs` | `backend/src/panopticam/app.py:514` | embed/server.py:do_POST, embed/server.py:do_GET, backend/src/panopticam/actions.py:create, vision/experiments/eg2_embed.py:main |
| `DELETE` | `/api/moments/{mid}` | `backend/src/panopticam/app.py:526` | backend/src/panopticam/alerts.py:delete, embed/server.py:do_GET, backend/src/panopticam/actions.py:delete, backend/src/panopticam/actions.py:prune_long |
| `GET` | `/api/rooms` | `backend/src/panopticam/app.py:532` | embed/server.py:do_GET, web/src/components/Datasets.tsx:sources, web/src/components/Browse.tsx:roomQuery, backend/src/panopticam/actions.py:pinned_hits |
| `GET` | `/api/tags` | `backend/src/panopticam/app.py:627` | embed/server.py:do_GET, backend/src/panopticam/actions.py:pinned_hits, web/src/app.tsx:AllPages, vision/datasets.py:process |
| `POST` | `/api/backup` | `backend/src/panopticam/app.py:632` | vision/backups.py:run, embed/server.py:do_GET, embed/server.py:do_POST, vision/backups.py:pull |
| `GET` | `/api/status` | `backend/src/panopticam/app.py:641` | backend/src/panopticam/actions.py:status, embed/server.py:do_GET, backend/src/panopticam/actions.py:pinned_hits, vision/browser.py:main |
| `GET` | `/api/debug/heap` | `backend/src/panopticam/app.py:659` | embed/server.py:do_GET, backend/src/panopticam/actions.py:pinned_hits, vision/datasets.py:process, embed/server.py:do_POST |
| `GET` | `/api/stills` | `backend/src/panopticam/app.py:687` | embed/server.py:do_GET, embed/server.py:video_frames, backend/src/panopticam/actions.py:status, backend/src/panopticam/actions.py:pinned_hits |
| `GET` | `/api/datasets` | `backend/src/panopticam/app.py:704` | web/src/components/Datasets.tsx:Datasets, embed/server.py:do_GET, vision/datasets.py:process, web/src/components/Datasets.tsx:sources |
| `GET` | `/api/datasets/{file}/photo/{i}` | `backend/src/panopticam/app.py:740` | embed/server.py:do_GET, vision/datasets.py:process, embed/server.py:video_frames, vision/datasets.py:cycle |
| `GET` | `/api/datasets/{file}` | `backend/src/panopticam/app.py:760` | embed/server.py:do_GET, web/src/components/Datasets.tsx:Datasets, vision/datasets.py:process, vision/datasets.py:_archive |
| `POST` | `/api/datasets/{file}` | `backend/src/panopticam/app.py:769` | embed/server.py:do_POST, vision/datasets.py:process, embed/server.py:do_GET, vision/datasets.py:_archive |
| `POST` | `/api/datasets/{file}/back` | `backend/src/panopticam/app.py:783` | embed/server.py:do_POST, vision/datasets.py:process, embed/server.py:do_GET, vision/datasets.py:_archive |
| `POST` | `/api/vision/gallery` | `backend/src/panopticam/app.py:798` | embed/server.py:do_GET, embed/server.py:do_POST, vision/browser.py:main, backend/src/panopticam/actions.py:create |
| `POST` | `/api/vision/datasets/offloaded` | `backend/src/panopticam/app.py:819` | vision/datasets.py:process, embed/server.py:do_POST, backend/src/panopticam/actions.py:load, embed/server.py:do_GET |
| `POST` | `/api/vision/datasets/local` | `backend/src/panopticam/app.py:831` | vision/datasets.py:process, embed/server.py:do_POST, embed/server.py:do_GET, web/src/components/Datasets.tsx:sources |
| `GET` | `/api/vision/datasets` | `backend/src/panopticam/app.py:838` | web/src/components/Datasets.tsx:Datasets, embed/server.py:do_GET, vision/datasets.py:process, embed/server.py:do_POST |
| `POST` | `/api/vision/datasets` | `backend/src/panopticam/app.py:846` | embed/server.py:do_POST, vision/datasets.py:process, web/src/components/Datasets.tsx:Datasets, embed/server.py:do_GET |
| `GET` | `/api/harvest` | `backend/src/panopticam/app.py:860` | embed/server.py:do_GET, vision/datasets.py:process, embed/server.py:do_POST, embed/server.py:main |
| `GET` | `/api/telegram` | `backend/src/panopticam/app.py:865` | embed/server.py:do_GET, web/src/components/Alerts.tsx:TelegramPrefs, embed/server.py:main, embed/server.py:_json |
| `PUT` | `/api/telegram` | `backend/src/panopticam/app.py:870` | embed/server.py:do_GET, embed/server.py:do_POST, embed/server.py:_json, backend/src/panopticam/actions.py:status |
| `GET` | `/api/rooms/{site}/{room}` | `backend/src/panopticam/app.py:877` | embed/server.py:do_GET, web/src/components/Datasets.tsx:sources, web/src/components/Browse.tsx:roomQuery, vision/atlas.py:_stage |
| `GET` | `/api/traits/top` | `backend/src/panopticam/app.py:907` | embed/server.py:do_GET, web/src/app.tsx:AllPages, web/src/app.tsx:TopBar, vision/browser.py:main |
| `GET` | `/api/soon/calibration` | `backend/src/panopticam/app.py:1088` | embed/server.py:do_GET, embed/server.py:do_POST, backend/src/panopticam/alerts.py:load, backend/src/panopticam/actions.py:upgrade |
| `GET` | `/api/soon` | `backend/src/panopticam/app.py:1112` | embed/server.py:do_GET, embed/server.py:do_POST, vision/browser.py:_start_tunnel, embed/server.py:main |
| `GET` | `/api/rooms/{site}/{room}/stream` | `backend/src/panopticam/app.py:1150` | embed/server.py:do_GET, web/src/components/Datasets.tsx:sources, web/src/components/Browse.tsx:roomQuery, embed/server.py:_json |
| `GET` | `/api/camsoda/token` | `backend/src/panopticam/app.py:1168` | embed/server.py:do_GET, embed/server.py:do_POST, embed/server.py:main, vision/browser.py:main |
| `GET` | `/api/favorites` | `backend/src/panopticam/app.py:1185` | embed/server.py:do_GET, backend/src/panopticam/actions.py:pinned_hits, web/src/api.ts:f4fHere, web/src/app.tsx:AllPages |
| `PUT` | `/api/favorites/{site}/{room}` | `backend/src/panopticam/app.py:1201` | embed/server.py:do_GET, web/src/components/Datasets.tsx:sources, backend/src/panopticam/actions.py:pinned_hits, backend/src/panopticam/actions.py:create |
| `POST` | `/api/rooms/{site}/{room}/record` | `backend/src/panopticam/app.py:1216` | web/src/components/Datasets.tsx:sources, backend/src/panopticam/actions.py:create, embed/server.py:_json, embed/server.py:do_GET |
| `GET` | `/api/recordings` | `backend/src/panopticam/app.py:1225` | embed/server.py:do_GET, backend/src/panopticam/actions.py:pinned_hits, embed/server.py:_json, web/src/components/Datasets.tsx:sources |

## Stateful Workers & Stores (`models/stores.json`)

- **`Handler`** (`worker_queue`, `embed/server.py`): keys `[]` · methods `[log_message, _json, do_GET, do_POST]`
- **`ClipWorker`** (`worker_queue`, `vision/clipwork.py`): keys `[sound, api, frames_embedded, embed, done, t0]` · methods `[__init__, one, run]`
- **`SpeechWorker`** (`worker_queue`, `vision/speech.py`): keys `[api, speech, every, done]` · methods `[__init__, run, start]`
- **`ComponentState(web/src/app.tsx)`** (`react_state`, `web/src/app.tsx`): keys `[connected, open]` · methods `[]`
- **`ComponentState(web/src/components/Actions.tsx)`** (`react_state`, `web/src/components/Actions.tsx`): keys `[ready, shown, name, visual, sound, busy, queue, skipped]` · methods `[]`
- **`ComponentState(web/src/components/Alerts.tsx)`** (`react_state`, `web/src/components/Alerts.tsx`): keys `[list, list, r, tags, keys, busy, p, rules]` · methods `[]`
- **`ComponentState(web/src/components/Browse.tsx)`** (`react_state`, `web/src/components/Browse.tsx`): keys `[v, q, vq, tags, langs, hairs, open, list]` · methods `[]`
- **`ComponentState(web/src/components/Charts.tsx)`** (`react_state`, `web/src/components/Charts.tsx`): keys `[hover]` · methods `[]`
- **`ComponentState(web/src/components/Datasets.tsx)`** (`react_state`, `web/src/components/Datasets.tsx`): keys `[drop, name, data, tab, skipped, busy, history, front]` · methods `[]`
- **`ComponentState(web/src/components/Director.tsx)`** (`react_state`, `web/src/components/Director.tsx`): keys `[women, favs, sound, held, picks, error, cut, log]` · methods `[]`
- **`ComponentState(web/src/components/Favorites.tsx)`** (`react_state`, `web/src/components/Favorites.tsx`): keys `[rows]` · methods `[]`
- **`ComponentState(web/src/components/Heard.tsx)`** (`react_state`, `web/src/components/Heard.tsx`): keys `[q, lang, live, lines, more, total, langs, err]` · methods `[]`
- **`ComponentState(web/src/components/Merge.tsx)`** (`react_state`, `web/src/components/Merge.tsx`): keys `[busy, batches, open, pairs, meta, keeper, skipped, busy]` · methods `[]`
- **`ComponentState(web/src/components/Moments.tsx)`** (`react_state`, `web/src/components/Moments.tsx`): keys `[frame, view, order, sound, data, error, cur, like]` · methods `[]`
- **`ComponentState(web/src/components/Performers.tsx)`** (`react_state`, `web/src/components/Performers.tsx`): keys `[q, scope, live, limit, data, gone, d, err]` · methods `[]`
- **`ComponentState(web/src/components/Player.tsx)`** (`react_state`, `web/src/components/Player.tsx`): keys `[state]` · methods `[]`
- **`ComponentState(web/src/components/Recordings.tsx)`** (`react_state`, `web/src/components/Recordings.tsx`): keys `[data, playing, videoError]` · methods `[]`
- **`ComponentState(web/src/components/Review.tsx)`** (`react_state`, `web/src/components/Review.tsx`): keys `[queue, skipped, done]` · methods `[]`
- **`ComponentState(web/src/components/RoomCard.tsx)`** (`react_state`, `web/src/components/RoomCard.tsx`): keys `[src, hover, playing]` · methods `[]`
- **`ComponentState(web/src/components/RoomPanel.tsx)`** (`react_state`, `web/src/components/RoomPanel.tsx`): keys `[lines, tips, live, open, found, detail, stream, pstate]` · methods `[]`
- **`ComponentState(web/src/components/Top.tsx)`** (`react_state`, `web/src/components/Top.tsx`): keys `[span, by, site, gender, q, limit, sort, favs]` · methods `[]`
- **`ComponentState(web/src/components/Wall.tsx)`** (`react_state`, `web/src/components/Wall.tsx`): keys `[state, levels, nonce, over, state, source, cands, error]` · methods `[]`
- **`ComponentState(web/src/components/Whois.tsx)`** (`react_state`, `web/src/components/Whois.tsx`): keys `[photo, res, pick, busy, err, over]` · methods `[]`

## Runtime Views & Workers (`state-machine.json`)

- Handler (embed/server.py)
- HTML View: OnlyFans to follow (onlyfans-to-follow.html)
- ClipWorker (vision/clipwork.py)
- SpeechWorker (vision/speech.py)
- HTML View: Panopticam (web/index.html)

