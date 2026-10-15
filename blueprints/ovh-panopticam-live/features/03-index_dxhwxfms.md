# Feature Blueprint: Index DXHWxfms

> Subsystem `Index DXHWxfms` covering 33 code slices, 27 API routes, and 0 data models.

## UI Screens & Components

| View / Component | Kind | Source | Rendered Elements | Visible Copy / Controls | Bound Routes |
|---|---|---|---|---|---|
| **`index-DXHWxfms`** | `component` | `index-DXHWxfms.js` | JSX | – | /api/actions, /api/actions/{e}, /api/actions/{e}/labels, /api/actions/{e}/pinned, /api/actions/{e}/rooms, /api/activity, /api/datasets, /api/events, /api/favorites, /api/favorites/{e}/{t}, /api/kv/{e}, /api/labels, /api/labels/counts, /api/links, /api/merges, /api/merges/batches, /api/merges/batches/reviewed, /api/merges/history, /api/merges/two-people, /api/merges/undo, /api/moments/{e.id}/video.mp4, /api/moments/{e}, /api/moments/{e}/keep, /api/moments/{e}/rate, /api/moments/{w.id}/video.mp4, /api/performers/{e}, /api/performers/{e}/drop, /api/recordings, /api/recordings/{e}, /api/recordings/{e}/file, /api/recordings/{n.job_id}/video, /api/rooms/{e}/{t}, /api/rooms/{e}/{t}/identity, /api/rooms/{e}/{t}/record, /api/rules, /api/rules/{e.id}, /api/rules/{e}, /api/soon/calibration, /api/speech/langs, /api/status, /api/tags, /api/telegram, /api/whois |

## API Surface

| Method | Pattern | Handler / Source | Request Fields | Response Fields |
|---|---|---|---|---|
| `GET` | `/api/labels` | `index-DXHWxfms.js:1` | – | – |
| `GET` | `/api/rooms/{e}/{t}` | `index-DXHWxfms.js:1` | – | – |
| `GET` | `/api/favorites/{e}/{t}` | `index-DXHWxfms.js:1` | – | – |
| `GET` | `/api/rooms/{e}/{t}/record` | `index-DXHWxfms.js:1` | – | – |
| `GET` | `/api/recordings/{e}/file` | `index-DXHWxfms.js:1` | – | – |
| `GET` | `/api/recordings/{e}` | `index-DXHWxfms.js:1` | – | – |
| `GET` | `/api/rules/{e.id}` | `index-DXHWxfms.js:1` | – | – |
| `GET` | `/api/rules/{e}` | `index-DXHWxfms.js:1` | – | – |
| `GET` | `/api/rooms/{e}/{t}/identity` | `index-DXHWxfms.js:1` | – | – |
| `GET` | `/api/merges/two-people` | `index-DXHWxfms.js:1` | – | – |
| `GET` | `/api/merges/undo` | `index-DXHWxfms.js:1` | – | – |
| `GET` | `/api/merges/batches/reviewed` | `index-DXHWxfms.js:1` | – | – |
| `GET` | `/api/whois` | `index-DXHWxfms.js:1` | – | – |
| `GET` | `/api/links` | `index-DXHWxfms.js:1` | – | – |
| `GET` | `/api/performers/{e}` | `index-DXHWxfms.js:1` | – | – |
| `GET` | `/api/performers/{e}/drop` | `index-DXHWxfms.js:1` | – | – |
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

## Proprietary Logic Slices

### `define` (`index-DXHWxfms.js:16-17`)
- **Category:** `business_logic` · **Proprietary Score:** `0.69`
- **Signature:** `function define(f)`
- **Behavior:** Proprietary routine `define` (1 lines)

### `minified_api_caller_1` (`index-DXHWxfms.js:1-1`)
- **Category:** `api_handler` · **Proprietary Score:** `0.69`
- **Signature:** `bundle_slice(1) -> fetch('/api/tags?limit=${e}')`
- **Behavior:** Minified bundle slice invoking `/api/tags?limit=${e}`
- **Triggers / Routes:** `/api/tags`

### `minified_api_caller_2` (`index-DXHWxfms.js:1-1`)
- **Category:** `api_handler` · **Proprietary Score:** `0.69`
- **Signature:** `bundle_slice(2) -> fetch('/api/speech/langs')`
- **Behavior:** Minified bundle slice invoking `/api/speech/langs`
- **Triggers / Routes:** `/api/speech/langs`

### `minified_api_caller_3` (`index-DXHWxfms.js:1-1`)
- **Category:** `api_handler` · **Proprietary Score:** `0.69`
- **Signature:** `bundle_slice(3) -> fetch('/api/labels/counts?family=${e}')`
- **Behavior:** Minified bundle slice invoking `/api/labels/counts?family=${e}`
- **Triggers / Routes:** `/api/labels/counts`

### `minified_api_caller_4` (`index-DXHWxfms.js:1-1`)
- **Category:** `api_handler` · **Proprietary Score:** `0.69`
- **Signature:** `bundle_slice(4) -> fetch('/api/labels')`
- **Behavior:** Minified bundle slice invoking `/api/labels`
- **Triggers / Routes:** `/api/labels`

### `minified_api_caller_5` (`index-DXHWxfms.js:1-1`)
- **Category:** `api_handler` · **Proprietary Score:** `0.69`
- **Signature:** `bundle_slice(5) -> fetch('/api/status')`
- **Behavior:** Minified bundle slice invoking `/api/status`
- **Triggers / Routes:** `/api/status`

### `minified_api_caller_6` (`index-DXHWxfms.js:1-1`)
- **Category:** `api_handler` · **Proprietary Score:** `0.69`
- **Signature:** `bundle_slice(6) -> fetch('/api/rooms/${e}/${t}?hours=${n}')`
- **Behavior:** Minified bundle slice invoking `/api/rooms/${e}/${t}?hours=${n}`
- **Triggers / Routes:** `/api/rooms/{e}/{t}`

### `minified_api_caller_7` (`index-DXHWxfms.js:1-1`)
- **Category:** `api_handler` · **Proprietary Score:** `0.69`
- **Signature:** `bundle_slice(7) -> fetch('/api/favorites')`
- **Behavior:** Minified bundle slice invoking `/api/favorites`
- **Triggers / Routes:** `/api/favorites`

### `minified_api_caller_8` (`index-DXHWxfms.js:1-1`)
- **Category:** `api_handler` · **Proprietary Score:** `0.69`
- **Signature:** `bundle_slice(8) -> fetch('/api/favorites/${e}/${t}')`
- **Behavior:** Minified bundle slice invoking `/api/favorites/${e}/${t}`
- **Triggers / Routes:** `/api/favorites/{e}/{t}`

### `minified_api_caller_9` (`index-DXHWxfms.js:1-1`)
- **Category:** `api_handler` · **Proprietary Score:** `0.69`
- **Signature:** `bundle_slice(9) -> fetch('/api/rooms/${e}/${t}/record')`
- **Behavior:** Minified bundle slice invoking `/api/rooms/${e}/${t}/record`
- **Triggers / Routes:** `/api/rooms/{e}/{t}/record`

### `minified_api_caller_10` (`index-DXHWxfms.js:1-1`)
- **Category:** `api_handler` · **Proprietary Score:** `0.69`
- **Signature:** `bundle_slice(10) -> fetch('/api/recordings')`
- **Behavior:** Minified bundle slice invoking `/api/recordings`
- **Triggers / Routes:** `/api/recordings`

### `minified_api_caller_11` (`index-DXHWxfms.js:1-1`)
- **Category:** `api_handler` · **Proprietary Score:** `0.69`
- **Signature:** `bundle_slice(11) -> fetch('/api/recordings/${e}/file')`
- **Behavior:** Minified bundle slice invoking `/api/recordings/${e}/file`
- **Triggers / Routes:** `/api/recordings/{e}/file`

### `minified_api_caller_12` (`index-DXHWxfms.js:1-1`)
- **Category:** `api_handler` · **Proprietary Score:** `0.69`
- **Signature:** `bundle_slice(12) -> fetch('/api/recordings/${e}')`
- **Behavior:** Minified bundle slice invoking `/api/recordings/${e}`
- **Triggers / Routes:** `/api/recordings/{e}`

### `minified_api_caller_13` (`index-DXHWxfms.js:1-1`)
- **Category:** `api_handler` · **Proprietary Score:** `0.69`
- **Signature:** `bundle_slice(13) -> fetch('/api/rules')`
- **Behavior:** Minified bundle slice invoking `/api/rules`
- **Triggers / Routes:** `/api/rules`

### `minified_api_caller_14` (`index-DXHWxfms.js:1-1`)
- **Category:** `api_handler` · **Proprietary Score:** `0.69`
- **Signature:** `bundle_slice(14) -> fetch('/api/rules/${e.id}')`
- **Behavior:** Minified bundle slice invoking `/api/rules/${e.id}`
- **Triggers / Routes:** `/api/rules/{e.id}`

### `minified_api_caller_15` (`index-DXHWxfms.js:1-1`)
- **Category:** `api_handler` · **Proprietary Score:** `0.69`
- **Signature:** `bundle_slice(15) -> fetch('/api/rules')`
- **Behavior:** Minified bundle slice invoking `/api/rules`
- **Triggers / Routes:** `/api/rules`

### `minified_api_caller_16` (`index-DXHWxfms.js:1-1`)
- **Category:** `api_handler` · **Proprietary Score:** `0.69`
- **Signature:** `bundle_slice(16) -> fetch('/api/rules/${e}')`
- **Behavior:** Minified bundle slice invoking `/api/rules/${e}`
- **Triggers / Routes:** `/api/rules/{e}`

### `minified_api_caller_17` (`index-DXHWxfms.js:1-1`)
- **Category:** `api_handler` · **Proprietary Score:** `0.69`
- **Signature:** `bundle_slice(17) -> fetch('/api/activity?limit=${e}')`
- **Behavior:** Minified bundle slice invoking `/api/activity?limit=${e}`
- **Triggers / Routes:** `/api/activity`

### `minified_api_caller_18` (`index-DXHWxfms.js:1-1`)
- **Category:** `api_handler` · **Proprietary Score:** `0.69`
- **Signature:** `bundle_slice(18) -> fetch('/api/telegram')`
- **Behavior:** Minified bundle slice invoking `/api/telegram`
- **Triggers / Routes:** `/api/telegram`

### `minified_api_caller_19` (`index-DXHWxfms.js:1-1`)
- **Category:** `api_handler` · **Proprietary Score:** `0.69`
- **Signature:** `bundle_slice(19) -> fetch('/api/telegram')`
- **Behavior:** Minified bundle slice invoking `/api/telegram`
- **Triggers / Routes:** `/api/telegram`

### `minified_api_caller_20` (`index-DXHWxfms.js:1-1`)
- **Category:** `api_handler` · **Proprietary Score:** `0.69`
- **Signature:** `bundle_slice(20) -> fetch('/api/datasets')`
- **Behavior:** Minified bundle slice invoking `/api/datasets`
- **Triggers / Routes:** `/api/datasets`

### `minified_api_caller_21` (`index-DXHWxfms.js:1-1`)
- **Category:** `api_handler` · **Proprietary Score:** `0.69`
- **Signature:** `bundle_slice(21) -> fetch('/api/rooms/${e}/${t}/identity')`
- **Behavior:** Minified bundle slice invoking `/api/rooms/${e}/${t}/identity`
- **Triggers / Routes:** `/api/rooms/{e}/{t}/identity`

### `minified_api_caller_22` (`index-DXHWxfms.js:1-1`)
- **Category:** `api_handler` · **Proprietary Score:** `0.69`
- **Signature:** `bundle_slice(22) -> fetch('/api/merges?offset=${e}&limit=${t}&min_score=${n}')`
- **Behavior:** Minified bundle slice invoking `/api/merges?offset=${e}&limit=${t}&min_score=${n}`
- **Triggers / Routes:** `/api/merges`

### `minified_api_caller_23` (`index-DXHWxfms.js:1-1`)
- **Category:** `api_handler` · **Proprietary Score:** `0.69`
- **Signature:** `bundle_slice(23) -> fetch('/api/merges')`
- **Behavior:** Minified bundle slice invoking `/api/merges`
- **Triggers / Routes:** `/api/merges`

### `minified_api_caller_24` (`index-DXHWxfms.js:1-1`)
- **Category:** `api_handler` · **Proprietary Score:** `0.69`
- **Signature:** `bundle_slice(24) -> fetch('/api/merges/two-people')`
- **Behavior:** Minified bundle slice invoking `/api/merges/two-people`
- **Triggers / Routes:** `/api/merges/two-people`

### `minified_api_caller_25` (`index-DXHWxfms.js:1-1`)
- **Category:** `api_handler` · **Proprietary Score:** `0.69`
- **Signature:** `bundle_slice(25) -> fetch('/api/merges/undo')`
- **Behavior:** Minified bundle slice invoking `/api/merges/undo`
- **Triggers / Routes:** `/api/merges/undo`

### `minified_api_caller_26` (`index-DXHWxfms.js:1-1`)
- **Category:** `api_handler` · **Proprietary Score:** `0.69`
- **Signature:** `bundle_slice(26) -> fetch('/api/merges/history')`
- **Behavior:** Minified bundle slice invoking `/api/merges/history`
- **Triggers / Routes:** `/api/merges/history`

### `minified_api_caller_27` (`index-DXHWxfms.js:1-1`)
- **Category:** `api_handler` · **Proprietary Score:** `0.69`
- **Signature:** `bundle_slice(27) -> fetch('/api/merges/batches')`
- **Behavior:** Minified bundle slice invoking `/api/merges/batches`
- **Triggers / Routes:** `/api/merges/batches`

### `minified_api_caller_28` (`index-DXHWxfms.js:1-1`)
- **Category:** `api_handler` · **Proprietary Score:** `0.69`
- **Signature:** `bundle_slice(28) -> fetch('/api/merges/batches/reviewed')`
- **Behavior:** Minified bundle slice invoking `/api/merges/batches/reviewed`
- **Triggers / Routes:** `/api/merges/batches/reviewed`

### `minified_api_caller_29` (`index-DXHWxfms.js:1-1`)
- **Category:** `api_handler` · **Proprietary Score:** `0.69`
- **Signature:** `bundle_slice(29) -> fetch('/api/whois')`
- **Behavior:** Minified bundle slice invoking `/api/whois`
- **Triggers / Routes:** `/api/whois`

