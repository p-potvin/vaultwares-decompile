# Panopticam — Clean-Room Architecture Blueprint

- **Target:** `https://ovh-panopticam.tail1764b8.ts.net`
- **Extracted:** `Thu, 15 Oct 2026 05:37`
- **Detected Stack:** JavaScript/TypeScript · React · Web SPA / HTTP
- **Metrics:** 2 files scanned · 33 proprietary slices · 125 API routes · 33 data models · 0 state stores · 2 UI views/components · 4 feature clusters

## Domain & Content Context (What the App Parses & Shows)

- **Content & Domain Signals:** `bongacams`, `cam4`, `camsoda`, `chaturbate`, `director`, `flirt4free`, `lovense`, `lush`, `moments`, `nude`, `performer`, `personas`, `recordings`, `stills`, `stripchat`, `tippers`, `tokens`, `topless`, `toy_runs`, `twins`, `viewers`
- **Media & Inference Modalities:** FFmpeg Video/Audio Transcoding & Frame Extraction · HLS Live Video Playback · HLS Live Video Streams (.m3u8) · Speech-to-Text Transcription & Language ID
- **Referenced External Domains / Providers:** `aomedia.org`, `fonts.googleapis.com`, `fonts.gstatic.com`

## UI Pages, Views & Visual Anatomy (`models/ui-views.json`)

| View / Screen | Kind | Source File | Rendered Elements | Visible Copy / Controls | Bound API Routes |
|---|---|---|---|---|---|
| **`Panopticam`** | `dom_section` | `index.html` | JSX | – | – |
| **`index-DXHWxfms`** | `component` | `index-DXHWxfms.js` | JSX | – | /api/actions, /api/actions/{e}, /api/actions/{e}/labels, /api/actions/{e}/pinned, /api/actions/{e}/rooms, /api/activity, /api/datasets, /api/events, /api/favorites, /api/favorites/{e}/{t}, /api/kv/{e}, /api/labels, /api/labels/counts, /api/links, /api/merges, /api/merges/batches, /api/merges/batches/reviewed, /api/merges/history, /api/merges/two-people, /api/merges/undo, /api/moments/{e.id}/video.mp4, /api/moments/{e}, /api/moments/{e}/keep, /api/moments/{e}/rate, /api/moments/{w.id}/video.mp4, /api/performers/{e}, /api/performers/{e}/drop, /api/recordings, /api/recordings/{e}, /api/recordings/{e}/file, /api/recordings/{n.job_id}/video, /api/rooms/{e}/{t}, /api/rooms/{e}/{t}/identity, /api/rooms/{e}/{t}/record, /api/rules, /api/rules/{e.id}, /api/rules/{e}, /api/soon/calibration, /api/speech/langs, /api/status, /api/tags, /api/telegram, /api/whois |

## Feature Clusters (Semantic Domains)

| Cluster | File | Slices | Routes | Models |
|---|---|---|---|---|
| **Api** | `features/01-api.md` | 0 | 0 | 1 |
| **Https:** | `features/02-https.md` | 33 | 1 | 0 |
| **Index DXHWxfms** | `features/03-index_dxhwxfms.md` | 33 | 27 | 0 |
| **Openapi** | `features/04-openapi.md` | 33 | 97 | 32 |

## API Route Summary (`openapi.json`)

| Method | Pattern | Source | Linked Slices |
|---|---|---|---|
| `GET` | `/openapi.json` | `https://ovh-panopticam.tail1764b8.ts.net:1` | index-DXHWxfms.js:minified_api_caller_10, index-DXHWxfms.js:minified_api_caller_9, index-DXHWxfms.js:minified_api_caller_20 |
| `GET` | `/api/director` | `openapi.json:1` | index-DXHWxfms.js:minified_api_caller_20, index-DXHWxfms.js:minified_api_caller_10, index-DXHWxfms.js:minified_api_caller_24 |
| `POST` | `/api/vision/taste` | `openapi.json:1` | index-DXHWxfms.js:minified_api_caller_7, index-DXHWxfms.js:minified_api_caller_2, index-DXHWxfms.js:minified_api_caller_8 |
| `GET` | `/api/taste` | `openapi.json:1` | index-DXHWxfms.js:minified_api_caller_5, index-DXHWxfms.js:minified_api_caller_7, index-DXHWxfms.js:minified_api_caller_2 |
| `GET` | `/api/moments` | `openapi.json:1` | index-DXHWxfms.js:minified_api_caller_9, index-DXHWxfms.js:minified_api_caller_10, index-DXHWxfms.js:minified_api_caller_26 |
| `GET` | `/api/moments/{mid}/similar` | `openapi.json:1` | index-DXHWxfms.js:minified_api_caller_7, index-DXHWxfms.js:minified_api_caller_24, index-DXHWxfms.js:minified_api_caller_10 |
| `GET` | `/api/moments/{mid}/{part}` | `openapi.json:1` | index-DXHWxfms.js:minified_api_caller_10, index-DXHWxfms.js:minified_api_caller_2, index-DXHWxfms.js:minified_api_caller_9 |
| `POST` | `/api/moments/{mid}/keep` | `openapi.json:1` | index-DXHWxfms.js:minified_api_caller_9, index-DXHWxfms.js:minified_api_caller_10, index-DXHWxfms.js:minified_api_caller_5 |
| `POST` | `/api/moments/{mid}/rate` | `openapi.json:1` | index-DXHWxfms.js:minified_api_caller_10, index-DXHWxfms.js:minified_api_caller_9, index-DXHWxfms.js:minified_api_caller_12 |
| `POST` | `/api/vision/moments/{mid}/life` | `openapi.json:1` | index-DXHWxfms.js:minified_api_caller_9, index-DXHWxfms.js:minified_api_caller_10, index-DXHWxfms.js:minified_api_caller_24 |
| `POST` | `/api/vision/moments/vecs` | `openapi.json:1` | index-DXHWxfms.js:minified_api_caller_20, index-DXHWxfms.js:minified_api_caller_24, index-DXHWxfms.js:minified_api_caller_21 |
| `DELETE` | `/api/moments/{mid}` | `openapi.json:1` | index-DXHWxfms.js:minified_api_caller_9, index-DXHWxfms.js:minified_api_caller_10, index-DXHWxfms.js:minified_api_caller_25 |
| `GET` | `/api/rooms` | `openapi.json:1` | index-DXHWxfms.js:minified_api_caller_9, index-DXHWxfms.js:minified_api_caller_21, index-DXHWxfms.js:minified_api_caller_6 |
| `GET` | `/api/tags` | `openapi.json:1` | index-DXHWxfms.js:minified_api_caller_1, index-DXHWxfms.js:minified_api_caller_4, index-DXHWxfms.js:minified_api_caller_2 |
| `POST` | `/api/backup` | `openapi.json:1` | index-DXHWxfms.js:minified_api_caller_10, index-DXHWxfms.js:minified_api_caller_25, index-DXHWxfms.js:minified_api_caller_20 |
| `GET` | `/api/status` | `openapi.json:1` | index-DXHWxfms.js:minified_api_caller_5, index-DXHWxfms.js:minified_api_caller_4, index-DXHWxfms.js:minified_api_caller_2 |
| `GET` | `/api/debug/heap` | `openapi.json:1` | index-DXHWxfms.js:minified_api_caller_10, index-DXHWxfms.js:minified_api_caller_20, index-DXHWxfms.js:minified_api_caller_5 |
| `GET` | `/api/stills` | `openapi.json:1` | index-DXHWxfms.js:minified_api_caller_5, index-DXHWxfms.js:minified_api_caller_10, index-DXHWxfms.js:minified_api_caller_7 |
| `GET` | `/api/datasets` | `openapi.json:1` | index-DXHWxfms.js:minified_api_caller_20, index-DXHWxfms.js:minified_api_caller_4, index-DXHWxfms.js:minified_api_caller_28 |
| `GET` | `/api/datasets/{file}/photo/{i}` | `openapi.json:1` | index-DXHWxfms.js:minified_api_caller_20, index-DXHWxfms.js:minified_api_caller_4, index-DXHWxfms.js:minified_api_caller_11 |
| `GET` | `/api/datasets/{file}` | `openapi.json:1` | index-DXHWxfms.js:minified_api_caller_20, index-DXHWxfms.js:minified_api_caller_11, index-DXHWxfms.js:minified_api_caller_4 |
| `POST` | `/api/datasets/{file}` | `openapi.json:1` | index-DXHWxfms.js:minified_api_caller_20, index-DXHWxfms.js:minified_api_caller_11, index-DXHWxfms.js:minified_api_caller_28 |
| `POST` | `/api/datasets/{file}/back` | `openapi.json:1` | index-DXHWxfms.js:minified_api_caller_20, index-DXHWxfms.js:minified_api_caller_25, index-DXHWxfms.js:minified_api_caller_11 |
| `POST` | `/api/vision/gallery` | `openapi.json:1` | index-DXHWxfms.js:minified_api_caller_20, index-DXHWxfms.js:minified_api_caller_21, index-DXHWxfms.js:minified_api_caller_10 |
| `POST` | `/api/vision/datasets/offloaded` | `openapi.json:1` | index-DXHWxfms.js:minified_api_caller_20, index-DXHWxfms.js:minified_api_caller_23, index-DXHWxfms.js:minified_api_caller_28 |
| `POST` | `/api/vision/datasets/local` | `openapi.json:1` | index-DXHWxfms.js:minified_api_caller_20, index-DXHWxfms.js:minified_api_caller_4, index-DXHWxfms.js:minified_api_caller_23 |
| `GET` | `/api/vision/datasets` | `openapi.json:1` | index-DXHWxfms.js:minified_api_caller_20, index-DXHWxfms.js:minified_api_caller_4, index-DXHWxfms.js:minified_api_caller_2 |
| `POST` | `/api/vision/datasets` | `openapi.json:1` | index-DXHWxfms.js:minified_api_caller_20, index-DXHWxfms.js:minified_api_caller_23, index-DXHWxfms.js:minified_api_caller_28 |
| `GET` | `/api/harvest` | `openapi.json:1` | index-DXHWxfms.js:minified_api_caller_5, index-DXHWxfms.js:minified_api_caller_28, index-DXHWxfms.js:minified_api_caller_27 |
| `GET` | `/api/telegram` | `openapi.json:1` | index-DXHWxfms.js:minified_api_caller_18, index-DXHWxfms.js:minified_api_caller_19, index-DXHWxfms.js:minified_api_caller_7 |
| `PUT` | `/api/telegram` | `openapi.json:1` | index-DXHWxfms.js:minified_api_caller_18, index-DXHWxfms.js:minified_api_caller_19, index-DXHWxfms.js:minified_api_caller_7 |
| `GET` | `/api/rooms/{site}/{room}` | `openapi.json:1` | index-DXHWxfms.js:minified_api_caller_9, index-DXHWxfms.js:minified_api_caller_21, index-DXHWxfms.js:minified_api_caller_6 |
| `GET` | `/api/traits/top` | `openapi.json:1` | index-DXHWxfms.js:minified_api_caller_20, index-DXHWxfms.js:minified_api_caller_7, index-DXHWxfms.js:minified_api_caller_4 |
| `GET` | `/api/soon/calibration` | `openapi.json:1` | index-DXHWxfms.js:minified_api_caller_20, index-DXHWxfms.js:minified_api_caller_10, index-DXHWxfms.js:minified_api_caller_4 |
| `GET` | `/api/soon` | `openapi.json:1` | index-DXHWxfms.js:minified_api_caller_20, index-DXHWxfms.js:minified_api_caller_29, index-DXHWxfms.js:minified_api_caller_5 |
| `GET` | `/api/rooms/{site}/{room}/stream` | `openapi.json:1` | index-DXHWxfms.js:minified_api_caller_9, index-DXHWxfms.js:minified_api_caller_21, index-DXHWxfms.js:minified_api_caller_6 |
| `GET` | `/api/camsoda/token` | `openapi.json:1` | index-DXHWxfms.js:minified_api_caller_10, index-DXHWxfms.js:minified_api_caller_18, index-DXHWxfms.js:minified_api_caller_19 |
| `GET` | `/api/favorites` | `openapi.json:1` | index-DXHWxfms.js:minified_api_caller_7, index-DXHWxfms.js:minified_api_caller_8, index-DXHWxfms.js:minified_api_caller_20 |
| `PUT` | `/api/favorites/{site}/{room}` | `openapi.json:1` | index-DXHWxfms.js:minified_api_caller_7, index-DXHWxfms.js:minified_api_caller_8, index-DXHWxfms.js:minified_api_caller_9 |
| `POST` | `/api/rooms/{site}/{room}/record` | `openapi.json:1` | index-DXHWxfms.js:minified_api_caller_9, index-DXHWxfms.js:minified_api_caller_21, index-DXHWxfms.js:minified_api_caller_10 |
| `GET` | `/api/recordings` | `openapi.json:1` | index-DXHWxfms.js:minified_api_caller_10, index-DXHWxfms.js:minified_api_caller_11, index-DXHWxfms.js:minified_api_caller_9 |
| `DELETE` | `/api/recordings/{job_id}/file` | `openapi.json:1` | index-DXHWxfms.js:minified_api_caller_11, index-DXHWxfms.js:minified_api_caller_10, index-DXHWxfms.js:minified_api_caller_9 |
| `GET` | `/api/recordings/{job_id}/video` | `openapi.json:1` | index-DXHWxfms.js:minified_api_caller_10, index-DXHWxfms.js:minified_api_caller_11, index-DXHWxfms.js:minified_api_caller_12 |
| `DELETE` | `/api/recordings/{job_id}` | `openapi.json:1` | index-DXHWxfms.js:minified_api_caller_10, index-DXHWxfms.js:minified_api_caller_9, index-DXHWxfms.js:minified_api_caller_11 |
| `GET` | `/api/rules` | `openapi.json:1` | index-DXHWxfms.js:minified_api_caller_13, index-DXHWxfms.js:minified_api_caller_15, index-DXHWxfms.js:minified_api_caller_16 |

