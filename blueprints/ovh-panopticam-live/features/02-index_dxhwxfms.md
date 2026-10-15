# Feature Blueprint: Index DXHWxfms

> Subsystem `Index DXHWxfms` covering 1 code slices, 43 API routes, and 0 data models.

## UI Screens & Components

| View / Component | Kind | Source | Rendered Elements | Visible Copy / Controls | Bound Routes |
|---|---|---|---|---|---|
| **`index-DXHWxfms`** | `component` | `index-DXHWxfms.js` | JSX | – | /api/actions, /api/actions/{e}, /api/actions/{e}/labels, /api/actions/{e}/pinned, /api/actions/{e}/rooms, /api/activity, /api/datasets, /api/events, /api/favorites, /api/favorites/{e}/{t}, /api/kv/{e}, /api/labels, /api/labels/counts, /api/links, /api/merges, /api/merges/batches, /api/merges/batches/reviewed, /api/merges/history, /api/merges/two-people, /api/merges/undo, /api/moments/{e.id}/video.mp4, /api/moments/{e}, /api/moments/{e}/keep, /api/moments/{e}/rate, /api/moments/{w.id}/video.mp4, /api/performers/{e}, /api/performers/{e}/drop, /api/recordings, /api/recordings/{e}, /api/recordings/{e}/file, /api/recordings/{n.job_id}/video, /api/rooms/{e}/{t}, /api/rooms/{e}/{t}/identity, /api/rooms/{e}/{t}/record, /api/rules, /api/rules/{e.id}, /api/rules/{e}, /api/soon/calibration, /api/speech/langs, /api/status, /api/tags, /api/telegram, /api/whois |

## API Surface

| Method | Pattern | Handler / Source | Request Fields | Response Fields |
|---|---|---|---|---|
| `GET` | `/api/tags` | `index-DXHWxfms.js:1` | – | – |
| `GET` | `/api/speech/langs` | `index-DXHWxfms.js:1` | – | – |
| `GET` | `/api/labels/counts` | `index-DXHWxfms.js:1` | – | – |
| `GET` | `/api/labels` | `index-DXHWxfms.js:1` | – | – |
| `GET` | `/api/status` | `index-DXHWxfms.js:1` | – | – |
| `GET` | `/api/rooms/{e}/{t}` | `index-DXHWxfms.js:1` | – | – |
| `GET` | `/api/favorites` | `index-DXHWxfms.js:1` | – | – |
| `GET` | `/api/favorites/{e}/{t}` | `index-DXHWxfms.js:1` | – | – |
| `GET` | `/api/rooms/{e}/{t}/record` | `index-DXHWxfms.js:1` | – | – |
| `GET` | `/api/recordings` | `index-DXHWxfms.js:1` | – | – |
| `GET` | `/api/recordings/{e}/file` | `index-DXHWxfms.js:1` | – | – |
| `GET` | `/api/recordings/{e}` | `index-DXHWxfms.js:1` | – | – |
| `GET` | `/api/rules` | `index-DXHWxfms.js:1` | – | – |
| `GET` | `/api/rules/{e.id}` | `index-DXHWxfms.js:1` | – | – |
| `GET` | `/api/rules/{e}` | `index-DXHWxfms.js:1` | – | – |
| `GET` | `/api/activity` | `index-DXHWxfms.js:1` | – | – |
| `GET` | `/api/telegram` | `index-DXHWxfms.js:1` | – | – |
| `GET` | `/api/datasets` | `index-DXHWxfms.js:1` | – | – |
| `GET` | `/api/rooms/{e}/{t}/identity` | `index-DXHWxfms.js:1` | – | – |
| `GET` | `/api/merges` | `index-DXHWxfms.js:1` | – | – |
| `GET` | `/api/merges/two-people` | `index-DXHWxfms.js:1` | – | – |
| `GET` | `/api/merges/undo` | `index-DXHWxfms.js:1` | – | – |
| `GET` | `/api/merges/history` | `index-DXHWxfms.js:1` | – | – |
| `GET` | `/api/merges/batches` | `index-DXHWxfms.js:1` | – | – |
| `GET` | `/api/merges/batches/reviewed` | `index-DXHWxfms.js:1` | – | – |
| `GET` | `/api/whois` | `index-DXHWxfms.js:1` | – | – |
| `GET` | `/api/links` | `index-DXHWxfms.js:1` | – | – |
| `GET` | `/api/performers/{e}` | `index-DXHWxfms.js:1` | – | – |
| `GET` | `/api/performers/{e}/drop` | `index-DXHWxfms.js:1` | – | – |
| `GET` | `/api/soon/calibration` | `index-DXHWxfms.js:1` | – | – |
| `GET` | `/api/actions` | `index-DXHWxfms.js:1` | – | – |
| `GET` | `/api/actions/{e}` | `index-DXHWxfms.js:1` | – | – |
| `GET` | `/api/actions/{e}/pinned` | `index-DXHWxfms.js:1` | – | – |
| `GET` | `/api/actions/{e}/rooms` | `index-DXHWxfms.js:1` | – | – |
| `GET` | `/api/actions/{e}/labels` | `index-DXHWxfms.js:1` | – | – |
| `GET` | `/api/kv/{e}` | `index-DXHWxfms.js:1` | – | – |
| `GET` | `/api/moments/{e}/keep` | `index-DXHWxfms.js:1` | – | – |
| `GET` | `/api/moments/{e}` | `index-DXHWxfms.js:1` | – | – |
| `GET` | `/api/moments/{e}/rate` | `index-DXHWxfms.js:1` | – | – |
| `GET` | `/api/moments/{w.id}/video.mp4` | `index-DXHWxfms.js:43` | – | – |
| `GET` | `/api/moments/{e.id}/video.mp4` | `index-DXHWxfms.js:43` | – | – |
| `GET` | `/api/recordings/{n.job_id}/video` | `index-DXHWxfms.js:43` | – | – |
| `GET` | `/api/events` | `index-DXHWxfms.js:43` | – | – |

## Proprietary Logic Slices

### `define` (`index-DXHWxfms.js:16-17`)
- **Category:** `business_logic` · **Proprietary Score:** `0.69`
- **Signature:** `function define(f)`
- **Behavior:** Proprietary routine `define` (1 lines)

