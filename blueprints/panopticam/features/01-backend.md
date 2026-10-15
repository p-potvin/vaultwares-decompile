# Feature Blueprint: Backend

> Subsystem `Backend` covering 1115 code slices, 98 API routes, and 124 data models.

## Clean-Room Behavioral Summary (Gemma 4 E2B)

This subsystem is a machine learning backend designed for action recognition and preference modeling. It manages the lifecycle of media clips, processing them through proprietary feature extraction routines (`featurize`, `widen`) to generate rich feature vectors (`clip_feats`, `long_feats`). The core functionality involves training and refining action models using class-balanced logistic regression (`_fit`), dimensionality reduction (`pca_fit`), and held-out AUC evaluation (`cv_auc`). The system stores structured data on actions, labels, and clips, allowing for retrieval of moments, similarity searches, and the management of user preferences via vision-based inputs and rating mechanisms.

To rebuild this subsystem, focus on three primary layers: Data Persistence, Feature Engineering, and Model Training. The data layer requires a relational database schema to store `clip_feats`, `long_feats`, and action metadata. The feature engineering layer must implement the proprietary NumPy routines (`featurize`, `widen`) to transform raw media into the required input vectors. The model layer should be implemented using standard ML frameworks to replicate the core logic: defining action classes, calculating targets (`label_target`), and executing the classification pipeline (`fit_head`, `fit`). The data flow is strictly sequential: media ingestion feeds feature extraction, features populate the database, and the trained models are used for inference and preference updates via the defined API routes.

## API Surface

| Method | Pattern | Handler / Source | Request Fields | Response Fields |
|---|---|---|---|---|
| `GET` | `/api/director` | `backend/src/panopticam/app.py:401` | – | – |
| `POST` | `/api/vision/taste` | `backend/src/panopticam/app.py:418` | – | – |
| `GET` | `/api/taste` | `backend/src/panopticam/app.py:429` | – | – |
| `GET` | `/api/moments` | `backend/src/panopticam/app.py:439` | – | – |
| `GET` | `/api/moments/{mid}/similar` | `backend/src/panopticam/app.py:446` | – | – |
| `GET` | `/api/moments/{mid}/{part}` | `backend/src/panopticam/app.py:454` | – | – |
| `POST` | `/api/moments/{mid}/keep` | `backend/src/panopticam/app.py:468` | – | – |
| `POST` | `/api/moments/{mid}/rate` | `backend/src/panopticam/app.py:479` | – | – |
| `POST` | `/api/vision/moments/{mid}/life` | `backend/src/panopticam/app.py:494` | – | – |
| `POST` | `/api/vision/moments/vecs` | `backend/src/panopticam/app.py:514` | – | – |
| `DELETE` | `/api/moments/{mid}` | `backend/src/panopticam/app.py:526` | – | – |
| `GET` | `/api/rooms` | `backend/src/panopticam/app.py:532` | – | – |
| `GET` | `/api/tags` | `backend/src/panopticam/app.py:627` | – | – |
| `POST` | `/api/backup` | `backend/src/panopticam/app.py:632` | – | – |
| `GET` | `/api/status` | `backend/src/panopticam/app.py:641` | – | – |
| `GET` | `/api/debug/heap` | `backend/src/panopticam/app.py:659` | – | – |
| `GET` | `/api/stills` | `backend/src/panopticam/app.py:687` | – | – |
| `GET` | `/api/datasets` | `backend/src/panopticam/app.py:704` | – | – |
| `GET` | `/api/datasets/{file}/photo/{i}` | `backend/src/panopticam/app.py:740` | – | – |
| `GET` | `/api/datasets/{file}` | `backend/src/panopticam/app.py:760` | – | – |
| `POST` | `/api/datasets/{file}` | `backend/src/panopticam/app.py:769` | – | – |
| `POST` | `/api/datasets/{file}/back` | `backend/src/panopticam/app.py:783` | – | – |
| `POST` | `/api/vision/gallery` | `backend/src/panopticam/app.py:798` | – | – |
| `POST` | `/api/vision/datasets/offloaded` | `backend/src/panopticam/app.py:819` | – | – |
| `POST` | `/api/vision/datasets/local` | `backend/src/panopticam/app.py:831` | – | – |
| `GET` | `/api/vision/datasets` | `backend/src/panopticam/app.py:838` | – | – |
| `POST` | `/api/vision/datasets` | `backend/src/panopticam/app.py:846` | – | – |
| `GET` | `/api/harvest` | `backend/src/panopticam/app.py:860` | – | – |
| `GET` | `/api/telegram` | `backend/src/panopticam/app.py:865` | – | – |
| `PUT` | `/api/telegram` | `backend/src/panopticam/app.py:870` | – | – |
| `GET` | `/api/rooms/{site}/{room}` | `backend/src/panopticam/app.py:877` | – | – |
| `GET` | `/api/traits/top` | `backend/src/panopticam/app.py:907` | – | – |
| `GET` | `/api/soon/calibration` | `backend/src/panopticam/app.py:1088` | – | – |
| `GET` | `/api/soon` | `backend/src/panopticam/app.py:1112` | – | – |
| `GET` | `/api/rooms/{site}/{room}/stream` | `backend/src/panopticam/app.py:1150` | – | – |
| `GET` | `/api/camsoda/token` | `backend/src/panopticam/app.py:1168` | – | – |
| `GET` | `/api/favorites` | `backend/src/panopticam/app.py:1185` | – | – |
| `PUT` | `/api/favorites/{site}/{room}` | `backend/src/panopticam/app.py:1201` | – | – |
| `POST` | `/api/rooms/{site}/{room}/record` | `backend/src/panopticam/app.py:1216` | – | – |
| `GET` | `/api/recordings` | `backend/src/panopticam/app.py:1225` | – | – |
| `DELETE` | `/api/recordings/{job_id}/file` | `backend/src/panopticam/app.py:1307` | – | – |
| `GET` | `/api/recordings/{job_id}/video` | `backend/src/panopticam/app.py:1335` | – | – |
| `DELETE` | `/api/recordings/{job_id}` | `backend/src/panopticam/app.py:1365` | – | – |
| `GET` | `/api/rules` | `backend/src/panopticam/app.py:1376` | – | – |
| `POST` | `/api/rules` | `backend/src/panopticam/app.py:1381` | – | – |
| `PUT` | `/api/rules/{rule_id}` | `backend/src/panopticam/app.py:1390` | – | – |
| `DELETE` | `/api/rules/{rule_id}` | `backend/src/panopticam/app.py:1401` | – | – |
| `GET` | `/api/activity` | `backend/src/panopticam/app.py:1407` | – | – |
| `POST` | `/api/vision/notify` | `backend/src/panopticam/app.py:1432` | – | – |
| `POST` | `/api/mail/test` | `backend/src/panopticam/app.py:1440` | role | – |
| `GET` | `/api/vision/work` | `backend/src/panopticam/app.py:1449` | role | – |
| `POST` | `/api/vision/results` | `backend/src/panopticam/app.py:1457` | role | – |
| `GET` | `/api/vision/clips` | `backend/src/panopticam/app.py:1466` | – | – |
| `POST` | `/api/vision/clip-results` | `backend/src/panopticam/app.py:1493` | – | – |
| `GET` | `/api/speech/work` | `backend/src/panopticam/app.py:1520` | – | – |
| `POST` | `/api/speech/results` | `backend/src/panopticam/app.py:1537` | – | – |
| `GET` | `/api/heard` | `backend/src/panopticam/app.py:1544` | – | – |
| `GET` | `/api/speech/langs` | `backend/src/panopticam/app.py:1564` | – | – |
| `GET` | `/api/labels/counts` | `backend/src/panopticam/app.py:1570` | – | – |
| `POST` | `/api/labels` | `backend/src/panopticam/app.py:1583` | – | – |
| `GET` | `/api/vision/thumb` | `backend/src/panopticam/app.py:1592` | – | – |
| `GET` | `/api/vision/links` | `backend/src/panopticam/app.py:1614` | – | – |
| `GET` | `/api/review` | `backend/src/panopticam/app.py:1621` | – | – |
| `GET` | `/api/rooms/{site}/{room}/identity` | `backend/src/panopticam/app.py:1635` | – | – |
| `POST` | `/api/whois` | `backend/src/panopticam/app.py:1664` | – | – |
| `GET` | `/api/whois/gallery/{name}/photo` | `backend/src/panopticam/app.py:1707` | – | – |
| `GET` | `/api/merges` | `backend/src/panopticam/app.py:1741` | – | – |
| `POST` | `/api/merges` | `backend/src/panopticam/app.py:1786` | – | – |
| `GET` | `/api/merges/jobs/{job}` | `backend/src/panopticam/app.py:1810` | – | – |
| `POST` | `/api/merges/two-people` | `backend/src/panopticam/app.py:1823` | – | – |
| `POST` | `/api/merges/undo` | `backend/src/panopticam/app.py:1833` | – | – |
| `GET` | `/api/merges/history` | `backend/src/panopticam/app.py:1840` | – | – |
| `POST` | `/api/merges/batch` | `backend/src/panopticam/app.py:1908` | – | – |
| `GET` | `/api/merges/batches` | `backend/src/panopticam/app.py:1922` | – | – |
| `POST` | `/api/merges/reconcile` | `backend/src/panopticam/app.py:1933` | – | – |
| `POST` | `/api/merges/batches/reviewed` | `backend/src/panopticam/app.py:1970` | – | – |
| `GET` | `/api/actions` | `backend/src/panopticam/app.py:2002` | – | – |
| `POST` | `/api/actions` | `backend/src/panopticam/app.py:2009` | – | – |
| `PUT` | `/api/actions/{aid}` | `backend/src/panopticam/app.py:2016` | – | – |
| `PUT` | `/api/actions/{aid}/pinned` | `backend/src/panopticam/app.py:2026` | – | – |
| `DELETE` | `/api/actions/{aid}` | `backend/src/panopticam/app.py:2032` | – | – |
| `GET` | `/api/actions/{aid}/rooms` | `backend/src/panopticam/app.py:2039` | – | – |
| `GET` | `/api/actions/{aid}/review` | `backend/src/panopticam/app.py:2053` | – | – |
| `POST` | `/api/actions/{aid}/labels` | `backend/src/panopticam/app.py:2067` | – | – |
| `GET` | `/api/clips/{site}/{room}/{part}` | `backend/src/panopticam/app.py:2076` | – | – |
| `GET` | `/api/top` | `backend/src/panopticam/app.py:2126` | – | – |
| `GET` | `/api/performers` | `backend/src/panopticam/app.py:2184` | – | – |
| `GET` | `/api/performers/{pid}` | `backend/src/panopticam/app.py:2222` | – | – |
| `PUT` | `/api/performers/{pid}` | `backend/src/panopticam/app.py:2283` | – | – |
| `POST` | `/api/performers/{pid}/drop` | `backend/src/panopticam/app.py:2293` | – | – |
| `PUT` | `/api/links` | `backend/src/panopticam/app.py:2308` | – | – |
| `GET` | `/api/kv/{k}` | `backend/src/panopticam/app.py:2316` | – | – |
| `PUT` | `/api/kv/{k}` | `backend/src/panopticam/app.py:2322` | – | – |
| `GET` | `/api/chat/{site}/{room}/stream` | `backend/src/panopticam/app.py:2334` | – | – |
| `GET` | `/api/chat/top` | `backend/src/panopticam/app.py:2376` | – | – |
| `GET` | `/api/chat/{site}/{room}/tips` | `backend/src/panopticam/app.py:2402` | – | – |
| `GET` | `/api/events` | `backend/src/panopticam/app.py:2408` | – | – |
| `GET` | `/{path:path}` | `backend/src/panopticam/app.py:2436` | – | – |

## Data Models & Tables

- **`clip_feats`** (sql_table, `backend/src/panopticam/actions.py:85`): `key: text`, `ts: integer`, `v: blob`, `a: blob,`, `motion: real`, `loud: real`
- **`long_feats`** (sql_table, `backend/src/panopticam/actions.py:93`): `key: text`, `ts: integer`, `v: blob`, `a: blob`, `motion: real`, `loud: real`, `prof: blob`
- **`actions`** (sql_table, `backend/src/panopticam/actions.py:102`): `id: integer`, `name: text`, `visual: text`, `sound: text`, `created: integer`, `w0: blob`, `w: blob`, `trained: integer`
- **`action_labels`** (sql_table, `backend/src/panopticam/actions.py:112`): `action_id: integer`, `key: text`, `ts: integer`, `y: integer`, `x: blob`, `at: integer`
- **`SoundText`** (dataclass, `backend/src/panopticam/actions.py:345`): `dir: any`, `used: any`
- **`Actions`** (dataclass, `backend/src/panopticam/actions.py:404`): `store: any`, `index: any`, `vtext: any`, `stext: any`, `feats: any`, `long: any`, `actions: any`, `labelled: any`, `truth: any`, `on_clip: any`
- **`Rule`** (dataclass, `backend/src/panopticam/alerts.py:71`): `id: int`, `name: str`, `enabled: bool`, `event: str`, `sites: list[str]`, `genders: list[str]`, `tags: list[str]`, `q: str`, `keys: list[str]`, `min_viewers: int`, `threshold: int`, `spike_ratio: float`, `states: list[str]`, `action_id: int`, `action_p: float`, `soon_p: float`, `soon_within: int`, `tip_min: int`, `habit: str`, `habit_share: float`, `habit_min: int`, `phrase: str`, `query: dict[str, Any]`, `notify: bool`, `record: bool`, `wall: bool`, `cooldown: int`
- **`Alerts`** (dataclass, `backend/src/panopticam/alerts.py:160`): `store: any`, `rules: any`, `habit_of: any`, `public_twins: any`
- **`TasteHead`** (pydantic, `backend/src/panopticam/app.py:409`): `w: list[float]`, `b: float`, `auc: float | None`, `n: int`, `minutes: int`, `liked: int`
- **`MomentKeep`** (pydantic, `backend/src/panopticam/app.py:464`): `kept: bool`
- **`MomentRate`** (pydantic, `backend/src/panopticam/app.py:475`): `rating: int | None`
- **`MomentLife`** (pydantic, `backend/src/panopticam/app.py:490`): `life: float`
- **`MomentVec`** (pydantic, `backend/src/panopticam/app.py:503`): `id: int`, `v: list[float]`, `a: list[float] | None`, `p_toy: float | None`
- **`MomentVecs`** (pydantic, `backend/src/panopticam/app.py:510`): `items: list[MomentVec]`
- **`DatasetVerdict`** (pydantic, `backend/src/panopticam/app.py:694`): `verdict: str | None`, `drop: list[int]`, `name: str | None`
- **`GalleryNames`** (pydantic, `backend/src/panopticam/app.py:794`): `names: list[str]`
- **`IngestReport`** (pydantic, `backend/src/panopticam/app.py:806`): `file: str`, `outcome: str`, `identity: str | None`, `added: int`, `total: int | None`, `detail: str`
- **`OffloadBody`** (pydantic, `backend/src/panopticam/app.py:815`): `sizes: dict[str, int]`
- **`LocalArchivesBody`** (pydantic, `backend/src/panopticam/app.py:827`): `items: list[dict[str, Any]]`
- **`FavoriteBody`** (pydantic, `backend/src/panopticam/app.py:1179`): `on: bool`, `note: str | None`, `auto_record: bool | None`
- **`RecordBody`** (pydantic, `backend/src/panopticam/app.py:1212`): `height: int | None`
- **`NotifyBody`** (pydantic, `backend/src/panopticam/app.py:1428`): `text: str`
- **`ClipResult`** (pydantic, `backend/src/panopticam/app.py:1484`): `key: str`, `ts: int`, `v: str`, `a: str | None`, `motion: float`, `loud: float`
- **`SpeechResult`** (pydantic, `backend/src/panopticam/app.py:1527`): `key: str`, `nospeech: float`, `langs: list[list[Any]]`, `text: str | None`, `logprob: float | None`, `rep: bool`
- **`LabelVerdict`** (pydantic, `backend/src/panopticam/app.py:1576`): `key: str`, `family: str`, `value: str`, `verdict: str`
- **`MergeBody`** (pydantic, `backend/src/panopticam/app.py:1759`): `keep: str`, `gone: str`, `rename: bool`
- **`PairBody`** (pydantic, `backend/src/panopticam/app.py:1818`): `a: str`, `b: str`
- **`UndoBody`** (pydantic, `backend/src/panopticam/app.py:1829`): `id: str`
- **`BatchPair`** (pydantic, `backend/src/panopticam/app.py:1852`): `keep: str`, `gone: str`, `score: float`, `photos_gone: int`, `why: str`
- **`BatchBody`** (pydantic, `backend/src/panopticam/app.py:1860`): `pairs: list[BatchPair]`, `label: str`
- **`ReconcileBody`** (pydantic, `backend/src/panopticam/app.py:1927`): `batch: str`, `pairs: list[BatchPair]`
- **`ReviewedBody`** (pydantic, `backend/src/panopticam/app.py:1964`): `batch: str`, `gone: str`, `ok: bool`
- **`ActionBody`** (pydantic, `backend/src/panopticam/app.py:1985`): `name: str`, `visual: str`, `sound: str`
- **`LabelBody`** (pydantic, `backend/src/panopticam/app.py:1991`): `key: str`, `ts: int`, `y: int`
- **`PinBody`** (pydantic, `backend/src/panopticam/app.py:2022`): `pinned: bool`
- **`RenameBody`** (pydantic, `backend/src/panopticam/app.py:2279`): `label: str | None`
- **`DropBody`** (pydantic, `backend/src/panopticam/app.py:2289`): `key: str`
- **`LinkBody`** (pydantic, `backend/src/panopticam/app.py:2301`): `a: str`, `b: str`, `same: bool | None`
- **`Backups`** (dataclass, `backend/src/panopticam/backup.py:78`): `store: any`, `db: any`, `ready: any`, `last: any`, `error: any`
- **`chat_tips`** (sql_table, `backend/src/panopticam/chat.py:82`): `key: text`, `ts: real`, `user: text`, `amount: integer`, `message: text`
- **`chat_toys`** (sql_table, `backend/src/panopticam/chat.py:91`): `key: text`, `ts: real`, `text: text`, `secs: integer,`, `level: text`
- **`CsCodec`** (dataclass, `backend/src/panopticam/chat.py:136`): `events: any`, `templates: any`
- **`ChatRoom`** (dataclass, `backend/src/panopticam/chat.py:241`): `key: any`, `auto: any`, `buf: any`, `listeners: any`, `linger_until: any`, `offline_since: any`, `paid: any`, `task: any`, `connected: any`, `fails: any`, `up_since: any`, `runs: any`, `toy_marks: any`, `live_run: any`
- **`Chat`** (dataclass, `backend/src/panopticam/chat.py:274`): `store: any`, `index: any`, `rooms: any`, `on_status: any`, `stats: any`
- **`ChatTruth`** (dataclass, `backend/src/panopticam/chattruth.py:85`): `chat: any`, `actions: any`, `sampler: any`, `pending: any`, `stats: any`
- **`Sample`** (dataclass, `backend/src/panopticam/clips.py:76`): `key: str`, `ts: int`, `seconds: float`, `long: bool`
- **`Sampler`** (dataclass, `backend/src/panopticam/clips.py:197`): `index: any`, `alive: any`, `dir: any`, `limiter: any`, `sampled: any`, `ready: any`, `stats: any`, `enabled: any`, `long_dir: any`, `long_at: any`, `long_pending: any`, `wanted: any`, `long_wanted: any`, `long_credits: any`, `long_busy: any`, `on_long: any`
- **`Settings`** (dataclass, `backend/src/panopticam/config.py:12`): `host: str`, `gc_threshold: str`, `port: int`, `data_dir: Path`, `static_dir: Path`, `chaturbate_every: float`, `camsoda_every: float`, `stripchat_every: float`, `flirt4free_every: float`, `bongacams_every: float`, `cam4_every: float`, `chaturbate_concurrency: int`, `room_calls_per_second: float`, `clip_samples_per_second: float`, `long_clips_per_day: int`, `toy_frames_per_day: int`, `room_frames_per_day: int`, `datasets_keep_all: bool`, `poll: bool`, `proxies: list[str]`, `zipper_api_base: str`, `zipper_api_key: str`, `telegram_token: str`, `telegram_chat: str`, `mail_to: str`, `mail_from: str`, `mail_host: str`, `mail_helo: str`
- **`dataset_verdicts`** (sql_table, `backend/src/panopticam/datasets.py:63`): `file: text`, `verdict: text`, `dropped: integer`, `ts: integer`
- **`dataset_ingest`** (sql_table, `backend/src/panopticam/datasets.py:69`): `file: text`, `outcome: text`, `identity: text`, `added: integer`, `detail: text`, `ts: integer`
- **`Datasets`** (dataclass, `backend/src/panopticam/datasets.py:99`): `store: any`, `root: any`, `verdicts: any`, `ingest: any`
- **`FaceFilter`** (dataclass, `backend/src/panopticam/facefilter.py:43`): `model: any`
- **`Forecast`** (dataclass, `backend/src/panopticam/forecast.py:38`): `chance: float`, `eta: int | None`, `days: int`, `share: float`
- **`room_shows`** (sql_table, `backend/src/panopticam/habits.py:49`): `key: text`, `body: text`, `ts: integer`
- **`Habits`** (dataclass, `backend/src/panopticam/habits.py:106`): `speech: Any`, `traits: any`, `index: any`, `store: any`, `rec: any`, `live: any`, `floor: any`, `calib: any`, `ids: any`, `words: any`, `on_paid: any`
- **`harvested`** (sql_table, `backend/src/panopticam/harvest.py:80`): `key: text`, `ts: integer`, `items: integer`, `status: text`, `error: text`
- **`harvest_media`** (sql_table, `backend/src/panopticam/harvest.py:91`): `key: text`, `set_id: text`, `kind: text`, `url: text`, `status: text`, `file: text`, `sha256: text`, `bytes: integer`, `ts: integer`
- **`Harvester`** (dataclass, `backend/src/panopticam/harvest.py:107`): `store: any`, `index: any`, `vision: any`, `root: any`, `session: any`, `direct: any`, `current: any`, `done_today: any`, `last_error: any`, `filter: any`, `filtered: any`
- **`Check`** (dataclass, `backend/src/panopticam/health.py:38`): `ok: bool`, `detail: str`, `since: float`, `pending: bool`, `warned: bool`
- **`Watchdog`** (dataclass, `backend/src/panopticam/health.py:82`): `sites: Callable[[], dict[str, Any]]`, `role_seen: Callable[[], dict[str, float]]`, `send: Callable[[str], None]`, `paths: list[Path]`, `extra: Callable[[], dict[str, tuple[bool, str]]] | None`, `hold_s: float`, `started: float`, `checks: dict[str, Check]`, `_task: asyncio.Task[None] | None`
- **`LoopLag`** (dataclass, `backend/src/panopticam/heap.py:164`): `last: any`, `late: any`, `stalls: any`
- **`Tracked`** (dataclass, `backend/src/panopticam/index.py:39`): `room: Room`, `recent: deque[tuple[float, int]]`, `last_listed: float`
- **`SiteState`** (dataclass, `backend/src/panopticam/index.py:86`): `sweeps: int`, `last_ok: float`, `last_error: str`, `last_ms: int`, `count: int`, `backoff: float`
- **`Index`** (dataclass, `backend/src/panopticam/index.py:105`): `store: any`, `alerts: any`, `habits: any`, `rooms: any`, `state: any`, `favorites: any`, `tags: any`, `twins: any`, `states: any`, `personas: any`, `persona_names: any`
- **`labels`** (sql_table, `backend/src/panopticam/labels.py:40`): `subject: text`, `family: text`, `value: text`, `state: text`, `since: integer`, `checked: integer`
- **`room_hosts`** (sql_table, `backend/src/panopticam/labels.py:49`): `key: text`
- **`Labels`** (dataclass, `backend/src/panopticam/labels.py:140`): `store: any`, `index: any`, `vision: any`, `traits: any`, `speech: any`, `habits: any`, `sets: any`, `checked: any`, `hosts: any`, `behaviours: any`, `locked_now: any`, `calibration: any`
- **`Mailer`** (dataclass, `backend/src/panopticam/mail.py:36`): `sent: any`, `dropped: any`, `last_error: any`
- **`identity_merges`** (sql_table, `backend/src/panopticam/merges.py:30`): `id: text`, `keep: text`, `gone: text`, `ts: integer`, `changes: text`, `undone: integer`
- **`merge_verdicts`** (sql_table, `backend/src/panopticam/merges.py:38`): `a: text`, `b: text`, `verdict: integer`, `ts: integer`
- **`Merges`** (dataclass, `backend/src/panopticam/merges.py:52`): `store: any`, `vision: any`, `stills: any`, `datasets: any`, `rooms_ready: any`, `not_same: any`
- **`Room`** (dataclass, `backend/src/panopticam/models.py:10`): `site: str`, `room: str`, `name: str`, `viewers: int`, `followers: int | None`, `gender: str`, `subject: str`, `tags: list[str]`, `thumb: str`, `clip: str`, `show: str`, `started_at: int | None`, `is_new: bool`, `country: str`, `height: int | None`, `playable: bool`, `play: dict[str, Any] | None`, `extra: dict[str, Any]`
- **`Level`** (dataclass, `backend/src/panopticam/models.py:65`): `height: int`, `url: str`, `width: int | None`, `bandwidth: int | None`, `audio_url: str | None`
- **`Stream`** (dataclass, `backend/src/panopticam/models.py:75`): `site: str`, `room: str`, `live: bool`, `status: str`, `levels: list[Level]`, `master: str | None`, `master_text: str | None`, `preview: dict[str, str] | None`, `info: dict[str, Any]`, `page: str`, `fetched_at: float`, `expires: bool`
- **`moments`** (sql_table, `backend/src/panopticam/moments.py:103`): `id: integer`, `key: text`, `name: text`, `ts: integer`, `at: integer`, `score: real`, `viewers: integer`, `why: text`, `parts: text`, `tokens: integer`, `toy: integer`, `bytes: integer`, `kept: integer`, `rating: integer,`
- **`Moments`** (dataclass, `backend/src/panopticam/moments.py:136`): `store: any`, `index: any`, `chat: any`, `actions: any`, `sampler: any`, `dir: any`, `pending: any`, `last: any`, `day: any`, `today: any`, `credits: any`, `stats: any`, `record: any`, `traits: any`, `live_taste: any`
- **`FetchError`** (dataclass, `backend/src/panopticam/net.py:34`): `url: any`, `status: any`
- **`RateLimiter`** (dataclass, `backend/src/panopticam/net.py:60`): `rate: any`, `tokens: any`, `at: any`, `lock: any`
- **`Net`** (dataclass, `backend/src/panopticam/net.py:87`): `room_limiter: any`, `status: any`
- **`Telegram`** (dataclass, `backend/src/panopticam/notify.py:50`): `store: any`, `q: any`, `dropped: any`, `http: any`, `prefs: any`
- **`PersonDetector`** (dataclass, `backend/src/panopticam/persons.py:28`): `model: any`
- **`Recorder`** (dataclass, `backend/src/panopticam/recorder.py:60`): `store: any`, `http: any`
- **`RoomFrames`** (dataclass, `backend/src/panopticam/roomframes.py:63`): `index: any`, `vision: any`, `sampler: any`, `root: any`, `day: any`, `today: any`, `per_room: any`, `last: any`, `hashes: any`, `stats: any`, `persons: any`
- **`Site`** (dataclass, `backend/src/panopticam/sites.py:36`): `id: str`, `label: str`
- **`Camsoda`** (dataclass, `backend/src/panopticam/sites.py:189`): `edges: any`
- **`Flirt4Free`** (dataclass, `backend/src/panopticam/sites.py:358`): `Listing: each service's home page (girls, guys, trans) embeds every live`, `names: any`
- **`Bongacams`** (dataclass, `backend/src/panopticam/sites.py:591`): `edges: any`
- **`Cam4`** (dataclass, `backend/src/panopticam/sites.py:703`): `names: any`, `masters: any`
- **`room_speech`** (sql_table, `backend/src/panopticam/speech.py:49`): `key: text`
- **`heard`** (sql_table, `backend/src/panopticam/speech.py:50`): `id: integer`, `lang: text`
- **`Speech`** (dataclass, `backend/src/panopticam/speech.py:143`): `index: any`, `store: any`, `sampler: any`, `rec: any`, `speaks: any`, `english: any`, `queue: any`, `heard: any`, `confident: any`, `rests: any`, `recent: any`, `lines: any`, `dropped: any`
- **`stills`** (sql_table, `backend/src/panopticam/stills.py:82`): `key: text`, `visits: integer`, `frames: integer`, `last: integer`, `done: integer`
- **`Stills`** (dataclass, `backend/src/panopticam/stills.py:98`): `store: any`, `index: any`, `sampler: any`, `root: any`, `limit: any`, `state: any`, `done_24h: any`, `last_error: any`, `filter: any`, `dropped: any`, `duplicates: any`
- **`rooms`** (sql_table, `backend/src/panopticam/store.py:23`): `key: text`, `site: text`, `room: text`, `name: text`, `gender: text`, `thumb: text`, `first_seen: integer`, `last_seen: integer`, `peak_viewers: integer`, `peak_at: integer`, `followers: integer`, `sessions: integer`, `live_seconds: integer`
- **`sessions`** (sql_table, `backend/src/panopticam/store.py:39`): `id: integer`, `key: text`, `started: integer`, `ended: integer`, `peak: integer`, `viewer_sum: integer`, `samples: integer`
- **`samples`** (sql_table, `backend/src/panopticam/store.py:51`): `key: text`, `ts: integer`, `viewers: integer`
- **`fav_samples`** (sql_table, `backend/src/panopticam/store.py:59`): `key: text`, `ts: integer`, `viewers: integer`
- **`favorites`** (sql_table, `backend/src/panopticam/store.py:66`): `key: text`, `added: integer`, `note: text`, `auto_record: integer`
- **`recordings`** (sql_table, `backend/src/panopticam/store.py:73`): `job_id: text`, `key: text`, `height: integer`, `started: integer`, `ended: integer`, `status: text`, `published: text`, `error: text`
- **`rules`** (sql_table, `backend/src/panopticam/store.py:84`): `id: integer`, `body: text`
- **`activity`** (sql_table, `backend/src/panopticam/store.py:89`): `id: integer`, `ts: integer`, `kind: text`, `key: text`, `rule_id: integer`, `rule_name: text`, `data: text`
- **`forecast_log`** (sql_table, `backend/src/panopticam/store.py:102`): `ts: integer`, `key: text`, `horizon: integer`, `chance: real`, `share: real`, `eta: integer`
- **`kv`** (sql_table, `backend/src/panopticam/store.py:112`): `k: text`, `v: text`
- **`Store`** (dataclass, `backend/src/panopticam/store.py:128`): `path: any`, `db: any`
- **`LiveTaste`** (dataclass, `backend/src/panopticam/taste.py:50`): `store: any`, `w: any`, `b: any`, `meta: any`, `recent: any`, `scope: any`
- **`ToyFrames`** (dataclass, `backend/src/panopticam/toyframes.py:62`): `index: any`, `sampler: any`, `root: any`, `queue: any`, `day: any`, `taken: any`, `per_room: any`, `pending: any`, `last_neg: any`, `stats: any`
- **`room_traits`** (sql_table, `backend/src/panopticam/traits.py:38`): `key: text`, `body: text`, `ts: integer`
- **`room_heads`** (sql_table, `backend/src/panopticam/traits.py:43`): `key: text`, `n: integer`, `v: blob`, `ts: integer`
- **`Traits`** (dataclass, `backend/src/panopticam/traits.py:156`): `store: any`, `rooms: any`, `vocab: any`, `sound_vocab: any`, `hair_bias: any`
- **`vision`** (sql_table, `backend/src/panopticam/vision.py:111`): `key: text`, `ts: integer`, `clip: blob`
- **`face_reads`** (sql_table, `backend/src/panopticam/vision.py:116`): `key: text`, `ts: integer`
- **`faces`** (sql_table, `backend/src/panopticam/vision.py:120`): `key: text`, `ts: integer`, `score: real`, `emb: blob`
- **`personas`** (sql_table, `backend/src/panopticam/vision.py:127`): `key: text`, `persona_id: integer`, `label: text`, `verified: integer`, `match: real`, `ts: integer`
- **`persona_names`** (sql_table, `backend/src/panopticam/vision.py:136`): `persona_id: integer`, `label: text`, `ts: integer`
- **`guests`** (sql_table, `backend/src/panopticam/vision.py:143`): `key: text`, `persona_id: integer`, `label: text`, `verified: integer`, `sim: real`, `confirmed: integer`, `ts: integer`
- **`links`** (sql_table, `backend/src/panopticam/vision.py:154`): `a: text`, `b: text`, `verdict: integer`, `ts: integer`
- **`TextEncoder`** (dataclass, `backend/src/panopticam/vision.py:180`): `dir: any`, `used: any`
- **`Vision`** (dataclass, `backend/src/panopticam/vision.py:243`): `store: any`, `index: any`, `traits: any`, `text: any`, `clip: any`, `face: any`, `links: any`, `states: any`, `changes: any`, `persona: any`, `names: any`, `face_seen: any`, `views: any`, `worker_seen: any`, `role_seen: any`, `tried: any`, `processed: any`, `twins: any`, `twins_at: any`, `candidates: any`
- **`favorites`** (sql_table, `backend/tests/test_backup.py:12`): `key: text`
- **`action_labels`** (sql_table, `backend/tests/test_backup.py:13`): `action_id: integer,`
- **`faces`** (sql_table, `backend/tests/test_backup.py:16`): `key: text,`
- **`dataset_ingest`** (sql_table, `backend/tests/test_merges.py:28`): `file: text`, `added: integer,`, `await: store.c.execute("insert`, `datasets: =`, `ready: =`, `m: =`, `await: m.load()`, `for: key,`, `await: store.c.execute("insert`, `await: v.rename(7,`, `yield: m,`, `await: store.close()`, `async: def`, `got: =`, `assert: got["stills"]`, `assert: stills.state["chaturbate:a7"]["identity"]`, `assert: datasets.ingest["camila7__________abcdefgh.zip"]["identity"]`, `assert: v.persona["chaturbate:a7"]["id"]`, `assert: v.names`, `assert: (ready`, `rows: =`, `assert: (rows[0]["persona_id"],`, `await: m.revert("j1")`, `assert: stills.state["chaturbate:a7"]["identity"]`, `assert: datasets.ingest["camila7__________abcdefgh.zip"]["identity"]`, `assert: v.names[7]`, `assert: (await`, `async: def`, `await: m.two_people("zed",`, `m2: =`, `await: m2.load()`, `assert: ("amy",`
- **`Idx`** (dataclass, `backend/tests/test_roomframes.py:11`): `rooms: any`
- **`FakeIndex`** (dataclass, `backend/tests/test_toyframes.py:16`): `rooms: any`
- **`personas`** (sql_table, `vision/atlas.py:89`): `id: integer`, `kfc_name: text`, `label: text`, `created: integer`, `sightings: integer`
- **`guests`** (sql_table, `vision/atlas.py:108`): `room: text`, `persona_id: integer`, `ts: integer`, `sim: real`
- **`Datasets`** (dataclass, `vision/datasets.py:90`): `gallery: any`, `atlas: any`, `faces: any`, `nudity: any`, `prior_name: any`, `staging: any`, `incoming: any`, `pending: any`, `out: any`
- **`moments`** (sql_table, `vision/moments_embed.py:62`): `file: text`, `key: text,`, `v: blob,`, `a: blob,`
- **`Speech`** (dataclass, `vision/speech.py:125`): `encoder: any`, `decoder: any`, `past: any`, `present: any`, `name: any`, `codes: any`, `lang_ids: any`, `nospeech_id: any`, `transcribe_id: any`, `notimestamps_id: any`, `pieces: any`
- **`Room`** (interface, `web/src/api.ts:5`): `key: string`, `speaks: string[]`, `english: 'fluent' | 'some' | 'none'`, `wearing: string`, `labels: Record<string, string[]>`, `doing: number[]`, `yours: number`, `said: string`, `site: SiteId`, `room: string`, `name: string`, `viewers: number`, `followers: number | null`, `gender: string`, `subject: string`, `tags: string[]`, `thumb: string`, `clip: string`, `show: string`, `started_at: number | null`, `is_new: boolean`, `country: string`, `height: number | null`, `playable: boolean`, `extra: Record<string, unknown>`, `trend: number`, `real: number`, `heat: number`, `favorite: boolean`, `uptime: number | null`, `twins: string[]`, `also: { key: string; site: SiteId; room: string; viewers: number; show: string }[]`, `private_on: { key: string; site: SiteId; show: string }[]`, `habits: string[]`, `viewers_all: number`, `performer: string`, `persona_id: number`, `state: 'nude' | 'topless' | 'partial' | 'covered' | 'empty'`, `state_since: number`, `match: number`
- **`Level`** (interface, `web/src/api.ts:111`): `height: number`, `url: string`, `width: number | null`, `bandwidth: number | null`, `audio_url: string | null`
- **`Stream`** (interface, `web/src/api.ts:119`): `site: SiteId`, `room: string`, `live: boolean`, `status: string`, `levels: Level[]`, `master: string | null`, `master_text: string | null`, `preview: { kind: 'hls' | 'mp4'; url: string } | null`, `info: Record<string, any>`, `page: string`, `fetched_at: number`, `expires: boolean`
- **`Traits`** (interface, `web/src/api.ts:212`): `since: number`, `looks: number`, `clips: number`, `long: number`, `states: Record<string, number>`, `loud_db: number`, `motion: number`, `actions: { id: number; name: string; share: number; mean_p: number; clips: number }[]`, `profile: Record<string, number>`, `looks_like: Record<string, { tag: string; share: number }[]>`, `tag_looks: number`
- **`Forecast`** (interface, `web/src/api.ts:258`): `chance: number`, `eta: number | null`, `days: number`, `share: number`
- **`Rule`** (interface, `web/src/api.ts:282`): `id: number`, `name: string`, `enabled: boolean`, `event: RuleEvent`, `sites: string[]`, `genders: string[]`, `tags: string[]`, `q: string`, `keys: string[]`, `min_viewers: number`, `threshold: number`, `spike_ratio: number`, `states: string[]`, `notify: boolean`, `record: boolean`, `wall: boolean`, `cooldown: number`, `action_id: number`, `action_p: number`, `soon_p: number`, `soon_within: number`, `tip_min: number`, `phrase: string`, `query: Record<string, unknown>`, `habit: string`, `habit_share: number`, `habit_min: number`
- **`SiteState`** (interface, `web/src/api.ts:464`): `sweeps: number`, `last_ok: number`, `last_error: string`, `last_ms: number`, `count: number`

## Proprietary Logic Slices

### `main` (`backend/scripts/tune_forecast.py:20-46`)
- **Category:** `business_logic` · **Proprietary Score:** `0.79`
- **Signature:** `def main() -> None`
- **Behavior:** `main` — queries/mutates SQLite storage

### `featurize` (`backend/src/panopticam/actions.py:124-136`)
- **Category:** `business_logic` · **Proprietary Score:** `0.79`
- **Signature:** `def featurize(v: np.ndarray, a: np.ndarray | None, motion: float, loud: float,
              prof: np.ndarray | None = None) -> np.ndarray`
- **Behavior:** Proprietary routine `featurize` (12 lines)

### `widen` (`backend/src/panopticam/actions.py:137-144`)
- **Category:** `business_logic` · **Proprietary Score:** `0.79`
- **Signature:** `def widen(x: np.ndarray) -> np.ndarray`
- **Behavior:** A vector (features or weights) from before long clips, with zeros for the profile.

### `_sigmoid` (`backend/src/panopticam/actions.py:145-148`)
- **Category:** `business_logic` · **Proprietary Score:** `0.79`
- **Signature:** `def _sigmoid(z: np.ndarray) -> np.ndarray`
- **Behavior:** Proprietary routine `_sigmoid` (3 lines)

### `_fit` (`backend/src/panopticam/actions.py:149-165`)
- **Category:** `business_logic` · **Proprietary Score:** `0.79`
- **Signature:** `def _fit(Z: np.ndarray, y: np.ndarray, theta0: np.ndarray, lam: np.ndarray, steps: int, lr: float) -> np.ndarray`
- **Behavior:** Class-balanced logistic regression by full-batch Adam, L2-anchored to ``theta0`` (``Z`` carries a trailing column of ones for the bias).

### `modality_mask` (`backend/src/panopticam/actions.py:166-175`)
- **Category:** `business_logic` · **Proprietary Score:** `0.79`
- **Signature:** `def modality_mask(visual: str, sound: str) -> np.ndarray`
- **Behavior:** The feature dims an action may learn from: a sound-only action never learns pixels.

### `cv_auc` (`backend/src/panopticam/actions.py:176-188`)
- **Category:** `business_logic` · **Proprietary Score:** `0.79`
- **Signature:** `def cv_auc(fit, X: np.ndarray, y: np.ndarray) -> float`
- **Behavior:** Held-out AUC of ``fit(X, y) -> predict`` (leave-one-out up to 40 labels, else 5 folds).

### `sides` (`backend/src/panopticam/actions.py:189-193`)
- **Category:** `business_logic` · **Proprietary Score:** `0.79`
- **Signature:** `def sides(y: np.ndarray) -> tuple[int, int]`
- **Behavior:** (clear yes, clear no): "in between" answers (0.5) are on neither side.

### `label_target` (`backend/src/panopticam/actions.py:194-199`)
- **Category:** `business_logic` · **Proprietary Score:** `0.79`
- **Signature:** `def label_target(y: int) -> float`
- **Behavior:** A stored answer as a training target: 1 yes, 0 no, 2 "in between" (operator, 6 Oct 2026: many "dancing" clips are neither, and yes or no was simply wrong) → 0.5.

### `auc` (`backend/src/panopticam/actions.py:200-206`)
- **Category:** `business_logic` · **Proprietary Score:** `0.79`
- **Signature:** `def auc(p: np.ndarray, y: np.ndarray) -> float`
- **Behavior:** Proprietary routine `auc` (6 lines)

### `fit_head` (`backend/src/panopticam/actions.py:207-253`)
- **Category:** `business_logic` · **Proprietary Score:** `0.79`
- **Signature:** `def fit_head(X: np.ndarray, y: np.ndarray, w0: np.ndarray, b0: float,
             mask: np.ndarray | None = None, basis: tuple[np.ndarray, np.ndarray] | None = None,
             ) -> tuple[np.ndarray, float, dict[str, Any]]`
- **Behavior:** Refine the prompt's head with labels, never making it worse than the prompt. Candidates, each judged by held-out AUC on the labels themselves: * the prompt alone; * the prompt's score plus the clip's position on ``basis`` — the main axes (PCA) of what live clips vary along in this action's modality, so a few dozen labels fit ~20 numbers instead of memorising 1.3k; * with 50+ labels on each side, every weight of the modality, pulled towards the prompt. The first one is kept unless another beats it by a clear margin. (The old Rocchio fit scored *below* the prompt on held-out labels: 0.25–0.31 against 0.71–0.72 on the first real ones.)

### `pca_fit` (`backend/src/panopticam/actions.py:236-247`)
- **Category:** `business_logic` · **Proprietary Score:** `0.79`
- **Signature:** `def pca_fit(Xt: np.ndarray, yt: np.ndarray) -> Any`
- **Behavior:** Proprietary routine `pca_fit` (11 lines)

### `feats` (`backend/src/panopticam/actions.py:238-240`)
- **Category:** `business_logic` · **Proprietary Score:** `0.79`
- **Signature:** `def feats(Z) -> Any`
- **Behavior:** Proprietary routine `feats` (2 lines)

### `full_fit` (`backend/src/panopticam/actions.py:248-253`)
- **Category:** `business_logic` · **Proprietary Score:** `0.79`
- **Signature:** `def full_fit(Xt: np.ndarray, yt: np.ndarray) -> Any`
- **Behavior:** Proprietary routine `full_fit` (5 lines)

### `direction_fit` (`backend/src/panopticam/actions.py:254-266`)
- **Category:** `business_logic` · **Proprietary Score:** `0.79`
- **Signature:** `def direction_fit(Xt: np.ndarray, yt: np.ndarray) -> Any`
- **Behavior:** Proprietary routine `direction_fit` (12 lines)

### `feats` (`backend/src/panopticam/actions.py:260-262`)
- **Category:** `business_logic` · **Proprietary Score:** `0.79`
- **Signature:** `def feats(Z) -> Any`
- **Behavior:** Proprietary routine `feats` (2 lines)

### `fit_head_apart` (`backend/src/panopticam/actions.py:302-326`)
- **Category:** `business_logic` · **Proprietary Score:** `0.79`
- **Signature:** `def fit_head_apart(X: np.ndarray, y: np.ndarray, w0: np.ndarray, b0: float, mask: np.ndarray | None,
                   basis: tuple[np.ndarray, np.ndarray] | None) -> tuple[np.ndarray, float, dict[str, Any]]`
- **Behavior:** ``fit_head`` in a process of its own (one, spawned once, kept). In the API's threads the cross-validation's numpy work starved the event loop: ticks 0.5–0.9 s late every few seconds during a refit on the 4-core VPS (5 Oct 2026). Falls back to this thread.

### `train` (`backend/src/panopticam/actions.py:327-333`)
- **Category:** `business_logic` · **Proprietary Score:** `0.79`
- **Signature:** `def train(X: np.ndarray, y: np.ndarray, w0: np.ndarray, b0: float,
          mask: np.ndarray | None = None, basis: tuple[np.ndarray, np.ndarray] | None = None,
          ) -> tuple[np.ndarray, float]`
- **Behavior:** Proprietary routine `train` (6 lines)

### `prompt_variants` (`backend/src/panopticam/actions.py:334-344`)
- **Category:** `business_logic` · **Proprietary Score:** `0.79`
- **Signature:** `def prompt_variants(text: str) -> list[str]`
- **Behavior:** Wordings tried against the labels: CLAP heard "a woman moaning" at AUC 0.97 where "moaning" alone got 0.72 on the same answers.

### `__init__` (`backend/src/panopticam/actions.py:348-355`)
- **Category:** `business_logic` · **Proprietary Score:** `0.79`
- **Signature:** `def __init__(self, model_dir: Path) -> Any`
- **Behavior:** Proprietary routine `__init__` (7 lines)

### `unload_if_idle` (`backend/src/panopticam/actions.py:356-362`)
- **Category:** `business_logic` · **Proprietary Score:** `0.79`
- **Signature:** `def unload_if_idle(self, idle_s: float = 15 * 60) -> bool`
- **Behavior:** Proprietary routine `unload_if_idle` (6 lines)

### `_ensure` (`backend/src/panopticam/actions.py:363-387`)
- **Category:** `business_logic` · **Proprietary Score:** `0.79`
- **Signature:** `def _ensure(self) -> None`
- **Behavior:** `_ensure` — runs model inference / tensor embedding

### `embed` (`backend/src/panopticam/actions.py:388-403`)
- **Category:** `business_logic` · **Proprietary Score:** `0.79`
- **Signature:** `def embed(self, text: str) -> np.ndarray`
- **Behavior:** Proprietary routine `embed` (15 lines)

### `__init__` (`backend/src/panopticam/actions.py:405-428`)
- **Category:** `business_logic` · **Proprietary Score:** `0.79`
- **Signature:** `def __init__(self, store: Store, index: Index, visual_text: TextEncoder) -> Any`
- **Behavior:** Proprietary routine `__init__` (23 lines)

### `load` (`backend/src/panopticam/actions.py:429-459`)
- **Category:** `business_logic` · **Proprietary Score:** `0.79`
- **Signature:** `def load(self) -> None`
- **Behavior:** `load` — queries/mutates SQLite storage

