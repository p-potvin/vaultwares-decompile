# Feature Blueprint: Openapi

> Subsystem `Openapi` covering 33 code slices, 97 API routes, and 32 data models.

## API Surface

| Method | Pattern | Handler / Source | Request Fields | Response Fields |
|---|---|---|---|---|
| `GET` | `/api/director` | `openapi.json:1` | – | – |
| `POST` | `/api/vision/taste` | `openapi.json:1` | – | – |
| `GET` | `/api/taste` | `openapi.json:1` | – | – |
| `GET` | `/api/moments` | `openapi.json:1` | – | – |
| `GET` | `/api/moments/{mid}/similar` | `openapi.json:1` | – | – |
| `GET` | `/api/moments/{mid}/{part}` | `openapi.json:1` | – | – |
| `POST` | `/api/moments/{mid}/keep` | `openapi.json:1` | – | – |
| `POST` | `/api/moments/{mid}/rate` | `openapi.json:1` | – | – |
| `POST` | `/api/vision/moments/{mid}/life` | `openapi.json:1` | – | – |
| `POST` | `/api/vision/moments/vecs` | `openapi.json:1` | – | – |
| `DELETE` | `/api/moments/{mid}` | `openapi.json:1` | – | – |
| `GET` | `/api/rooms` | `openapi.json:1` | – | – |
| `GET` | `/api/tags` | `openapi.json:1` | – | – |
| `POST` | `/api/backup` | `openapi.json:1` | – | – |
| `GET` | `/api/status` | `openapi.json:1` | – | – |
| `GET` | `/api/debug/heap` | `openapi.json:1` | – | – |
| `GET` | `/api/stills` | `openapi.json:1` | – | – |
| `GET` | `/api/datasets` | `openapi.json:1` | – | – |
| `GET` | `/api/datasets/{file}/photo/{i}` | `openapi.json:1` | – | – |
| `GET` | `/api/datasets/{file}` | `openapi.json:1` | – | – |
| `POST` | `/api/datasets/{file}` | `openapi.json:1` | – | – |
| `POST` | `/api/datasets/{file}/back` | `openapi.json:1` | – | – |
| `POST` | `/api/vision/gallery` | `openapi.json:1` | – | – |
| `POST` | `/api/vision/datasets/offloaded` | `openapi.json:1` | – | – |
| `POST` | `/api/vision/datasets/local` | `openapi.json:1` | – | – |
| `GET` | `/api/vision/datasets` | `openapi.json:1` | – | – |
| `POST` | `/api/vision/datasets` | `openapi.json:1` | – | – |
| `GET` | `/api/harvest` | `openapi.json:1` | – | – |
| `GET` | `/api/telegram` | `openapi.json:1` | – | – |
| `PUT` | `/api/telegram` | `openapi.json:1` | – | – |
| `GET` | `/api/rooms/{site}/{room}` | `openapi.json:1` | – | – |
| `GET` | `/api/traits/top` | `openapi.json:1` | – | – |
| `GET` | `/api/soon/calibration` | `openapi.json:1` | – | – |
| `GET` | `/api/soon` | `openapi.json:1` | – | – |
| `GET` | `/api/rooms/{site}/{room}/stream` | `openapi.json:1` | – | – |
| `GET` | `/api/camsoda/token` | `openapi.json:1` | – | – |
| `GET` | `/api/favorites` | `openapi.json:1` | – | – |
| `PUT` | `/api/favorites/{site}/{room}` | `openapi.json:1` | – | – |
| `POST` | `/api/rooms/{site}/{room}/record` | `openapi.json:1` | – | – |
| `GET` | `/api/recordings` | `openapi.json:1` | – | – |
| `DELETE` | `/api/recordings/{job_id}/file` | `openapi.json:1` | – | – |
| `GET` | `/api/recordings/{job_id}/video` | `openapi.json:1` | – | – |
| `DELETE` | `/api/recordings/{job_id}` | `openapi.json:1` | – | – |
| `GET` | `/api/rules` | `openapi.json:1` | – | – |
| `POST` | `/api/rules` | `openapi.json:1` | – | – |
| `PUT` | `/api/rules/{rule_id}` | `openapi.json:1` | – | – |
| `DELETE` | `/api/rules/{rule_id}` | `openapi.json:1` | – | – |
| `GET` | `/api/activity` | `openapi.json:1` | – | – |
| `POST` | `/api/vision/notify` | `openapi.json:1` | – | – |
| `POST` | `/api/mail/test` | `openapi.json:1` | – | – |
| `GET` | `/api/vision/work` | `openapi.json:1` | – | – |
| `POST` | `/api/vision/results` | `openapi.json:1` | – | – |
| `GET` | `/api/vision/clips` | `openapi.json:1` | – | – |
| `POST` | `/api/vision/clip-results` | `openapi.json:1` | – | – |
| `GET` | `/api/speech/work` | `openapi.json:1` | – | – |
| `POST` | `/api/speech/results` | `openapi.json:1` | – | – |
| `GET` | `/api/heard` | `openapi.json:1` | – | – |
| `GET` | `/api/speech/langs` | `openapi.json:1` | – | – |
| `GET` | `/api/labels/counts` | `openapi.json:1` | – | – |
| `POST` | `/api/labels` | `openapi.json:1` | – | – |
| `GET` | `/api/vision/thumb` | `openapi.json:1` | – | – |
| `GET` | `/api/vision/links` | `openapi.json:1` | – | – |
| `GET` | `/api/review` | `openapi.json:1` | – | – |
| `GET` | `/api/rooms/{site}/{room}/identity` | `openapi.json:1` | – | – |
| `POST` | `/api/whois` | `openapi.json:1` | – | – |
| `GET` | `/api/whois/gallery/{name}/photo` | `openapi.json:1` | – | – |
| `GET` | `/api/merges` | `openapi.json:1` | – | – |
| `POST` | `/api/merges` | `openapi.json:1` | – | – |
| `GET` | `/api/merges/jobs/{job}` | `openapi.json:1` | – | – |
| `POST` | `/api/merges/two-people` | `openapi.json:1` | – | – |
| `POST` | `/api/merges/undo` | `openapi.json:1` | – | – |
| `GET` | `/api/merges/history` | `openapi.json:1` | – | – |
| `POST` | `/api/merges/batch` | `openapi.json:1` | – | – |
| `GET` | `/api/merges/batches` | `openapi.json:1` | – | – |
| `POST` | `/api/merges/reconcile` | `openapi.json:1` | – | – |
| `POST` | `/api/merges/batches/reviewed` | `openapi.json:1` | – | – |
| `GET` | `/api/actions` | `openapi.json:1` | – | – |
| `POST` | `/api/actions` | `openapi.json:1` | – | – |
| `PUT` | `/api/actions/{aid}` | `openapi.json:1` | – | – |
| `DELETE` | `/api/actions/{aid}` | `openapi.json:1` | – | – |
| `PUT` | `/api/actions/{aid}/pinned` | `openapi.json:1` | – | – |
| `GET` | `/api/actions/{aid}/rooms` | `openapi.json:1` | – | – |
| `GET` | `/api/actions/{aid}/review` | `openapi.json:1` | – | – |
| `POST` | `/api/actions/{aid}/labels` | `openapi.json:1` | – | – |
| `GET` | `/api/clips/{site}/{room}/{part}` | `openapi.json:1` | – | – |
| `GET` | `/api/top` | `openapi.json:1` | – | – |
| `GET` | `/api/performers` | `openapi.json:1` | – | – |
| `GET` | `/api/performers/{pid}` | `openapi.json:1` | – | – |
| `PUT` | `/api/performers/{pid}` | `openapi.json:1` | – | – |
| `POST` | `/api/performers/{pid}/drop` | `openapi.json:1` | – | – |
| `PUT` | `/api/links` | `openapi.json:1` | – | – |
| `GET` | `/api/kv/{k}` | `openapi.json:1` | – | – |
| `PUT` | `/api/kv/{k}` | `openapi.json:1` | – | – |
| `GET` | `/api/chat/{site}/{room}/stream` | `openapi.json:1` | – | – |
| `GET` | `/api/chat/top` | `openapi.json:1` | – | – |
| `GET` | `/api/chat/{site}/{room}/tips` | `openapi.json:1` | – | – |
| `GET` | `/api/events` | `openapi.json:1` | – | – |

## Data Models & Tables

- **`ActionBody`** (openapi_schema, `openapi.json:1`): `name: string`, `visual: string`, `sound: string`
- **`BatchBody`** (openapi_schema, `openapi.json:1`): `pairs: array`, `label: string`
- **`BatchPair`** (openapi_schema, `openapi.json:1`): `keep: string`, `gone: string`, `score: number`, `photos_gone: integer`, `why: string`
- **`ClipResult`** (openapi_schema, `openapi.json:1`): `key: string`, `ts: integer`, `v: string`, `a: any`, `motion: number`, `loud: number`
- **`DatasetVerdict`** (openapi_schema, `openapi.json:1`): `verdict: any`, `drop: array`, `name: any`
- **`DropBody`** (openapi_schema, `openapi.json:1`): `key: string`
- **`FavoriteBody`** (openapi_schema, `openapi.json:1`): `on: boolean`, `note: any`, `auto_record: any`
- **`GalleryNames`** (openapi_schema, `openapi.json:1`): `names: array`
- **`HTTPValidationError`** (openapi_schema, `openapi.json:1`): `detail: array`
- **`IngestReport`** (openapi_schema, `openapi.json:1`): `file: string`, `outcome: string`, `identity: any`, `added: integer`, `total: any`, `detail: string`
- **`LabelBody`** (openapi_schema, `openapi.json:1`): `key: string`, `ts: integer`, `y: integer`
- **`LabelVerdict`** (openapi_schema, `openapi.json:1`): `key: string`, `family: string`, `value: string`, `verdict: string`
- **`LinkBody`** (openapi_schema, `openapi.json:1`): `a: string`, `b: string`, `same: any`
- **`LocalArchivesBody`** (openapi_schema, `openapi.json:1`): `items: array`
- **`MergeBody`** (openapi_schema, `openapi.json:1`): `keep: string`, `gone: string`, `rename: boolean`
- **`MomentKeep`** (openapi_schema, `openapi.json:1`): `kept: boolean`
- **`MomentLife`** (openapi_schema, `openapi.json:1`): `life: number`
- **`MomentRate`** (openapi_schema, `openapi.json:1`): `rating: any`
- **`MomentVec`** (openapi_schema, `openapi.json:1`): `id: integer`, `v: array`, `a: any`, `p_toy: any`
- **`MomentVecs`** (openapi_schema, `openapi.json:1`): `items: array`
- **`NotifyBody`** (openapi_schema, `openapi.json:1`): `text: string`
- **`OffloadBody`** (openapi_schema, `openapi.json:1`): `sizes: object`
- **`PairBody`** (openapi_schema, `openapi.json:1`): `a: string`, `b: string`
- **`PinBody`** (openapi_schema, `openapi.json:1`): `pinned: boolean`
- **`ReconcileBody`** (openapi_schema, `openapi.json:1`): `batch: string`, `pairs: array`
- **`RecordBody`** (openapi_schema, `openapi.json:1`): `height: any`
- **`RenameBody`** (openapi_schema, `openapi.json:1`): `label: any`
- **`ReviewedBody`** (openapi_schema, `openapi.json:1`): `batch: string`, `gone: string`, `ok: boolean`
- **`SpeechResult`** (openapi_schema, `openapi.json:1`): `key: string`, `nospeech: number`, `langs: array`, `text: any`, `logprob: any`, `rep: boolean`
- **`TasteHead`** (openapi_schema, `openapi.json:1`): `w: array`, `b: number`, `auc: any`, `n: integer`, `minutes: integer`, `liked: integer`
- **`UndoBody`** (openapi_schema, `openapi.json:1`): `id: string`
- **`ValidationError`** (openapi_schema, `openapi.json:1`): `loc: array`, `msg: string`, `type: string`, `input: any`, `ctx: object`

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

