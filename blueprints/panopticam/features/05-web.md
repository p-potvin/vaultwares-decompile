# Feature Blueprint: Web

> Subsystem `Web` covering 284 code slices, 0 API routes, and 49 data models.

## Clean-Room Behavioral Summary (Gemma 4 E2B)

This subsystem is a complex, stateful content and social platform managing user interactions, content streams, and site configurations. Its core functionality revolves around processing and enforcing `Rule` definitions (which dictate behavior, thresholds, and actions) against real-time `Room` and `Stream` entities. Data flows from persistent configurations (`Rule`, `Traits`) and operational states (`SiteState`) into dynamic views (`Room`, `Stream`), which are then consumed by the application's state management layer (`useEvents`). The system uses predictive data (`Forecast`) to inform user experiences and manages the lifecycle of content delivery and user engagement.

To rebuild this, establish a clear separation between the domain models and the UI layer. The architecture requires a robust state management system capable of handling high-frequency events (`useEvents`) and complex asynchronous operations (polling, error handling via `SiteState`). The core logic must implement a rule engine that evaluates user actions against defined `Rule` parameters to trigger state transitions in the `Room` and `Stream` models. The frontend should be structured around a component hierarchy driven by the `AllPages` structure, utilizing standard React hooks for event handling and UI feedback (Toasts, TopBar).

## Data Models & Tables

- **`Rule`** (dataclass, `backend/src/panopticam/alerts.py:71`): `id: int`, `name: str`, `enabled: bool`, `event: str`, `sites: list[str]`, `genders: list[str]`, `tags: list[str]`, `q: str`, `keys: list[str]`, `min_viewers: int`, `threshold: int`, `spike_ratio: float`, `states: list[str]`, `action_id: int`, `action_p: float`, `soon_p: float`, `soon_within: int`, `tip_min: int`, `habit: str`, `habit_share: float`, `habit_min: int`, `phrase: str`, `query: dict[str, Any]`, `notify: bool`, `record: bool`, `wall: bool`, `cooldown: int`
- **`Forecast`** (dataclass, `backend/src/panopticam/forecast.py:38`): `chance: float`, `eta: int | None`, `days: int`, `share: float`
- **`SiteState`** (dataclass, `backend/src/panopticam/index.py:86`): `sweeps: int`, `last_ok: float`, `last_error: str`, `last_ms: int`, `count: int`, `backoff: float`
- **`Room`** (dataclass, `backend/src/panopticam/models.py:10`): `site: str`, `room: str`, `name: str`, `viewers: int`, `followers: int | None`, `gender: str`, `subject: str`, `tags: list[str]`, `thumb: str`, `clip: str`, `show: str`, `started_at: int | None`, `is_new: bool`, `country: str`, `height: int | None`, `playable: bool`, `play: dict[str, Any] | None`, `extra: dict[str, Any]`
- **`Level`** (dataclass, `backend/src/panopticam/models.py:65`): `height: int`, `url: str`, `width: int | None`, `bandwidth: int | None`, `audio_url: str | None`
- **`Stream`** (dataclass, `backend/src/panopticam/models.py:75`): `site: str`, `room: str`, `live: bool`, `status: str`, `levels: list[Level]`, `master: str | None`, `master_text: str | None`, `preview: dict[str, str] | None`, `info: dict[str, Any]`, `page: str`, `fetched_at: float`, `expires: bool`
- **`Traits`** (dataclass, `backend/src/panopticam/traits.py:156`): `store: any`, `rooms: any`, `vocab: any`, `sound_vocab: any`, `hair_bias: any`
- **`Room`** (interface, `web/src/api.ts:5`): `key: string`, `speaks: string[]`, `english: 'fluent' | 'some' | 'none'`, `wearing: string`, `labels: Record<string, string[]>`, `doing: number[]`, `yours: number`, `said: string`, `site: SiteId`, `room: string`, `name: string`, `viewers: number`, `followers: number | null`, `gender: string`, `subject: string`, `tags: string[]`, `thumb: string`, `clip: string`, `show: string`, `started_at: number | null`, `is_new: boolean`, `country: string`, `height: number | null`, `playable: boolean`, `extra: Record<string, unknown>`, `trend: number`, `real: number`, `heat: number`, `favorite: boolean`, `uptime: number | null`, `twins: string[]`, `also: { key: string; site: SiteId; room: string; viewers: number; show: string }[]`, `private_on: { key: string; site: SiteId; show: string }[]`, `habits: string[]`, `viewers_all: number`, `performer: string`, `persona_id: number`, `state: 'nude' | 'topless' | 'partial' | 'covered' | 'empty'`, `state_since: number`, `match: number`
- **`DirectorPick`** (interface, `web/src/api.ts:68`): `room: Room`, `score: number`, `happening: boolean`, `parts: { tip: number; toy: number; heat: number; action: number }`, `why: string[]`
- **`Moment`** (interface, `web/src/api.ts:78`): `id: number`, `key: string`, `name: string`, `ts: number`, `at: number`, `score: number`, `viewers: number`, `why: string[]`, `parts: { tip: number; toy: number; heat: number; action: number }`, `tokens: number`, `toy: number`, `bytes: number`, `kept: boolean`, `live: boolean`, `has_video: boolean`, `rating: number | null`, `life: number | null`, `taste: number`, `similar: boolean`, `p_toy: number | null`, `sim: number`
- **`Level`** (interface, `web/src/api.ts:111`): `height: number`, `url: string`, `width: number | null`, `bandwidth: number | null`, `audio_url: string | null`
- **`Stream`** (interface, `web/src/api.ts:119`): `site: SiteId`, `room: string`, `live: boolean`, `status: string`, `levels: Level[]`, `master: string | null`, `master_text: string | null`, `preview: { kind: 'hls' | 'mp4'; url: string } | null`, `info: Record<string, any>`, `page: string`, `fetched_at: number`, `expires: boolean`
- **`RoomRecord`** (interface, `web/src/api.ts:134`): `key: string`, `site: SiteId`, `room: string`, `name: string`, `gender: string`, `thumb: string`, `first_seen: number`, `last_seen: number`, `peak_viewers: number`, `peak_at: number | null`, `followers: number | null`, `sessions: number`, `live_seconds: number`
- **`Session`** (interface, `web/src/api.ts:150`): `started: number`, `ended: number | null`, `peak: number`, `viewer_sum: number`, `samples: number`
- **`LabelFamily`** (interface, `web/src/api.ts:159`): `on: 'person' | 'room' | 'off'`, `values: { value: string; state: LabelState }[]`, `vetoed: string[]`, `suggest: string[]`, `since_day: number | null`, `hosts: number`
- **`LabelDetail`** (interface, `web/src/api.ts:169`): `hair: LabelFamily`, `lang: LabelFamily`, `place: LabelFamily`, `look: LabelFamily`, `room: LabelFamily`, `habit: { vetoed: string[]; pinned: string[] }`
- **`RoomDetail`** (interface, `web/src/api.ts:178`): `key: string`, `live: Room | null`, `trend: number`, `record: RoomRecord | null`, `favorite: Favorite | null`, `history: [number, number][]`, `sessions: Session[]`, `page: string`, `forecast: Forecast | null`, `guests: Guest[]`, `traits: Traits | null`, `habits: Habit[]`, `private_now: string | null`, `speech: { speaks: string[]; heard: Record<string, number>; tries: number; english?: 'fluent' | 'some' | 'none' | null; english_logprob?: number | null } | null`, `heard: HeardLine[]`, `labels: LabelDetail`
- **`HeardLine`** (interface, `web/src/api.ts:207`): `id: number | null; ts: number; key: string; lang: string; text: string`, `site: string; room?: string; name?: string; live?: boolean; thumb?: string | null; viewers?: number | null; speaks?: string[]`
- **`Traits`** (interface, `web/src/api.ts:212`): `since: number`, `looks: number`, `clips: number`, `long: number`, `states: Record<string, number>`, `loud_db: number`, `motion: number`, `actions: { id: number; name: string; share: number; mean_p: number; clips: number }[]`, `profile: Record<string, number>`, `looks_like: Record<string, { tag: string; share: number }[]>`, `tag_looks: number`
- **`TipRow`** (interface, `web/src/api.ts:228`): `key: string; site: string; room: string; name: string; thumb?: string | null; persona?: number | null`, `live: Room | null; tokens: number; tips: number; tippers: number; biggest: number`, `per_hour: number | null; toy_runs: number`
- **`HabitRow`** (interface, `web/src/api.ts:236`): `key: string`, `site: string`, `room: string`, `name: string`, `thumb: string | null`, `share: number`, `n: number`, `live: Room | null`, `persona: number | null`
- **`Guest`** (interface, `web/src/api.ts:248`): `id: number`, `label: string`, `verified: boolean`, `times: number`, `last: number`, `best: number`
- **`Forecast`** (interface, `web/src/api.ts:258`): `chance: number`, `eta: number | null`, `days: number`, `share: number`
- **`Favorite`** (interface, `web/src/api.ts:267`): `key: string`, `added: number`, `note: string`, `auto_record: number`
- **`Rule`** (interface, `web/src/api.ts:282`): `id: number`, `name: string`, `enabled: boolean`, `event: RuleEvent`, `sites: string[]`, `genders: string[]`, `tags: string[]`, `q: string`, `keys: string[]`, `min_viewers: number`, `threshold: number`, `spike_ratio: number`, `states: string[]`, `notify: boolean`, `record: boolean`, `wall: boolean`, `cooldown: number`, `action_id: number`, `action_p: number`, `soon_p: number`, `soon_within: number`, `tip_min: number`, `phrase: string`, `query: Record<string, unknown>`, `habit: string`, `habit_share: number`, `habit_min: number`
- **`Activity`** (interface, `web/src/api.ts:317`): `id: number`, `ts: number`, `kind: RuleEvent | 'fav_online' | 'fav_offline'`, `key: string`, `rule_id: number | null`, `rule_name: string | null`, `data: Record<string, any>`, `name: string`, `thumb: string | null`, `live: boolean`
- **`WhoisRoom`** (interface, `web/src/api.ts:331`): `key: string`, `name: string`, `thumb: string | null`, `live: boolean`, `viewers: number | null`, `last_seen: number | null`, `sim: number`, `faces: number`, `likely: boolean`, `persona: number | null`, `performer: string | null`, `verified: boolean`
- **`WhoisFace`** (interface, `web/src/api.ts:346`): `box: [number, number, number, number]`, `score: number`, `gallery: { name: string; score: number; photos: number; likely: boolean; rooms: WhoisRoom[] }[]`, `personas: { id: number; label: string; kfc: string | null; score: number; likely: boolean; rooms: WhoisRoom[] }[]`, `rooms: WhoisRoom[]`
- **`MergePair`** (interface, `web/src/api.ts:355`): `a: string; b: string; score: number`, `crops_a: number; crops_b: number; photos_a: number; photos_b: number`, `rooms_a: WhoisRoom[]; rooms_b: WhoisRoom[]`
- **`IdentityMatch`** (interface, `web/src/api.ts:365`): `key: string`, `sim: number | null`, `faces: number`, `same_site: boolean`, `confirmed: boolean`, `persona: boolean`, `likely: boolean`, `name: string`, `thumb: string | null`, `live: boolean`, `viewers: number | null`, `last_seen: number | null`
- **`PerformerRoom`** (interface, `web/src/api.ts:381`): `key: string`, `site: SiteId`, `room: string`, `name: string`, `thumb: string | null`, `live: boolean`, `viewers: number | null`, `row: Room | null`, `last_seen: number | null`, `live_seconds: number`, `sessions: number`, `peak_viewers: number`
- **`Performer`** (interface, `web/src/api.ts:397`): `id: number`, `label: string`, `verified: boolean`, `named: boolean`, `live: number`, `viewers: number`, `rooms: PerformerRoom[]`, `sites: SiteId[]`, `live_seconds: number`, `last_seen: number`
- **`PerformerDetail`** (interface, `web/src/api.ts:412`): `id: number`, `label: string`, `verified: boolean`, `named: boolean`, `rooms: PerformerRoom[]`, `sessions: (Session & { key: string })[]`, `live_seconds: number`, `forecast: Forecast | null`, `guest_in: { key: string; times: number; last: number; best: number; host: { id: number; label: string } | null }[]`, `labels: LabelDetail | null`, `label_key: string | null`, `room_labels: Record<string, Record<string, string[]>>`, `habits: Record<string, string[]>`, `tips: {`, `days: number; tokens: number; followed: string[]`, `rooms: { key: string; tokens: number; tips: number; tippers: number; biggest: number }[]`, `tippers: { user: string; tokens: number; tips: number }[]`, `moments: { id: number; key: string; name: string; ts: number; score: number; viewers: number; why: string[]`, `tokens: number; toy: number; kept: boolean; rating: number | null; has_video: boolean }[]`, `gallery: string | null`
- **`Recording`** (interface, `web/src/api.ts:440`): `job_id: string`, `key: string`, `height: number | null`, `started: number`, `ended: number | null`, `status: string`, `error: string | null`, `name: string`, `file: string | null`, `playable: boolean | null`, `job: {`, `status: string`, `bytes_done: number | null`, `progress: number | null`, `speed: string | null`, `error: string | null`, `save_dir: string | null`
- **`SiteState`** (interface, `web/src/api.ts:464`): `sweeps: number`, `last_ok: number`, `last_error: string`, `last_ms: number`, `count: number`
- **`Status`** (interface, `web/src/api.ts:472`): `version: string`, `recording: boolean`, `telegram: boolean`, `rooms: number`, `sites: Record<SiteId, SiteState>`, `health: { check: string; ok: boolean; detail: string; since: number }[]`, `rss_mb: number | null`
- **`MergeBatch`** (interface, `web/src/api.ts:484`): `id: string`, `label: string`, `at: number`, `state: 'running' | 'done'`, `count: number`, `items: { keep: string; gone: string; score: number; why: string; id?: string; photos?: number; start?: number`, `error: string; warn?: string; reviewed?: 'ok' | 'undone' }[]`
- **`Query`** (interface, `web/src/api.ts:494`): `site: string`, `gender: string`, `q: string`, `tags: string`, `min_viewers: number`, `new: boolean`, `favorites: boolean`, `all_shows: boolean`, `sort: string`, `offset: number`, `limit: number`, `seed: number`, `keys: string`, `vq: string`, `like: string`, `state: string`, `min_age: number`, `max_age: number`, `action: number`, `doing: string`, `yours: boolean`, `fold: boolean`, `habit: string`, `speaks: string`, `english: string`, `wearing: string`, `label: string`, `said: string`
- **`ActionInfo`** (interface, `web/src/api.ts:534`): `id: number`, `name: string`, `visual: string`, `sound: string`, `created: number`, `trained: number | null`, `visual_used: string`, `sound_used: string`, `pinned: boolean`, `head: { head?: string; n?: number; auc?: Record<string, number> }`, `chat: { days: number; yes: number; no: number; auc?: number; precision?: number | null; recall?: number } | null`, `yes: number`, `no: number`, `partly: number`, `live: number`, `scored: number`
- **`ActionClip`** (interface, `web/src/api.ts:558`): `key: string`, `ts: number`, `p: number`, `why: 'likely' | 'unsure'`, `name: string`, `sound: boolean`, `long: boolean`
- **`TopRow`** (interface, `web/src/api.ts:569`): `rank: number`, `key: string`, `site: string`, `room: string`, `name: string`, `gender: string | null`, `thumb: string | null`, `live: boolean`, `viewers: number | null`, `seconds: number`, `sessions: number`, `peak: number`, `avg: number`, `real_avg: number | null`, `persona: number | null`
- **`Calibration`** (interface, `web/src/api.ts:601`): `days: number`, `horizon: number`, `n: number`, `brier: number | null`, `brier_share: number | null`, `came: number | null`, `buckets: { lo: number; hi: number; n: number; chance: number | null; came: number | null }[]`
- **`Dataset`** (interface, `web/src/api.ts:612`): `file: string`, `name: string`, `hash: string`, `size: number`, `made: number`, `count: number`, `gallery: boolean`, `keys: (string | null)[]`, `guessed: string`, `verdict: 'keep' | 'reject' | null`, `dropped: number`, `target: string | null`, `auto: boolean`, `ingest: { outcome: 'created' | 'added' | 'full' | 'skipped' | 'failed'; identity: string | null; added: number; detail: string; ts: number } | null`
- **`DatasetRoom`** (interface, `web/src/api.ts:633`): `key: string`, `name: string`, `live: boolean`, `thumb: string | null`, `persona: number | null`, `persona_name: string | null`
- **`DatasetList`** (interface, `web/src/api.ts:642`): `archives: Dataset[]`, `rooms: Record<string, DatasetRoom>`, `totals: { todo: number; keep: number; reject: number; photos: number; ingested: number }`, `gallery: string[]`, `stills: { enabled: boolean; done: number; in_progress: number; done_24h: number; frames: number; filtered_out: number; duplicates?: number; rounds_waiting?: number; rounds_due?: number }`
- **`AskPart`** (interface, `web/src/ask.ts:17`): `label: string`, `text: string`
- **`Asked`** (interface, `web/src/ask.ts:24`): `patch: Partial<Filters>`, `parts: AskPart[]`, `vq: string`
- **`Props`** (interface, `web/src/components/Player.tsx:34`): `site: SiteId`, `room: string`, `mode: 'full' | 'preview'`, `muted: boolean`, `height: number`, `clip: string`, `class: string`, `onState: (s: PlayerState, stream?: Stream) => void`, `nonce: number`
- **`Pair`** (interface, `web/src/components/Review.tsx:18`): `a: Room`, `b: Room`, `sim: number`, `persona_a: { label: string; verified: boolean } | null`, `persona_b: { label: string; verified: boolean } | null`
- **`ChatEvent`** (interface, `web/src/components/RoomPanel.tsx:12`): `amount: number; secs?: number | null; level?: string | null; menu?: boolean; mod?: boolean; fan?: boolean; host?: boolean }`
- **`Filters`** (interface, `web/src/state.ts:120`): `sites: SiteId[]`, `genders: string[]`, `q: string`, `tags: string[]`, `minViewers: number`, `newOnly: boolean`, `favoritesOnly: boolean`, `allShows: boolean`, `sort: string`, `size: 's' | 'm' | 'l'`, `vq: string`, `states: string[]`, `age: string`, `like: string`, `likeName: string`, `action: number`, `doing: number[]`, `fold: boolean`, `habits: string[]`, `speaks: string`, `english: string`, `hair: string`, `glasses: boolean`, `tattoos: boolean`, `wearing: string`, `place: string`, `ask: string`, `said: string`
- **`WallTile`** (interface, `web/src/state.ts:192`): `key: string`, `height: number`
- **`Wall`** (interface, `web/src/state.ts:198`): `name: string`, `tiles: WallTile[]`
- **`Walls`** (interface, `web/src/state.ts:203`): `active: number`, `walls: Wall[]`
- **`Toast`** (interface, `web/src/state.ts:327`): `id: number`, `text: string`, `kind: 'info' | 'error' | 'live'`, `action: { label: string; run: () => void }`
- **`Entry`** (interface, `web/src/thumbs.ts:13`): `refresh: () => void`, `visible: boolean`, `at: number`

## Proprietary Logic Slices

### `splitKey` (`web/src/api.ts:790-794`)
- **Category:** `business_logic` · **Proprietary Score:** `0.86`
- **Signature:** `function splitKey(key: string)`
- **Behavior:** Proprietary routine `splitKey` (4 lines)

### `f4fHere` (`web/src/api.ts:798-801`)
- **Category:** `business_logic` · **Proprietary Score:** `0.86`
- **Signature:** `function f4fHere()`
- **Behavior:** Proprietary routine `f4fHere` (3 lines)

### `setF4fHere` (`web/src/api.ts:801-804`)
- **Category:** `business_logic` · **Proprietary Score:** `0.86`
- **Signature:** `function setF4fHere(on: boolean)`
- **Behavior:** Proprietary routine `setF4fHere` (3 lines)

### `useEvents` (`web/src/app.tsx:50-78`)
- **Category:** `ui_component` · **Proprietary Score:** `0.99`
- **Signature:** `function useEvents()`
- **Behavior:** Proprietary routine `useEvents` (28 lines)
- **Triggers / Routes:** `/api/events`

### `TopBar` (`web/src/app.tsx:101-102`)
- **Category:** `ui_component` · **Proprietary Score:** `0.99`
- **Signature:** `function TopBar({ connected }: { connected: boolean })`
- **Behavior:** Proprietary routine `TopBar` (1 lines)

### `AllPages` (`web/src/app.tsx:149-150`)
- **Category:** `ui_component` · **Proprietary Score:** `0.99`
- **Signature:** `function AllPages({ here }: { here: Route['name'] })`
- **Behavior:** Proprietary routine `AllPages` (1 lines)

### `Toasts` (`web/src/app.tsx:184-198`)
- **Category:** `ui_component` · **Proprietary Score:** `0.99`
- **Signature:** `function Toasts()`
- **Behavior:** Proprietary routine `Toasts` (14 lines)

### `App` (`web/src/app.tsx:199-235`)
- **Category:** `ui_component` · **Proprietary Score:** `0.99`
- **Signature:** `function App()`
- **Behavior:** Proprietary routine `App` (36 lines)

### `close` (`web/src/app.tsx:154-155`)
- **Category:** `ui_component` · **Proprietary Score:** `0.99`
- **Signature:** `function close(e: Event)`
- **Behavior:** Proprietary routine `close` (1 lines)

### `esc` (`web/src/app.tsx:155-156`)
- **Category:** `ui_component` · **Proprietary Score:** `0.99`
- **Signature:** `function esc(e: KeyboardEvent)`
- **Behavior:** Proprietary routine `esc` (1 lines)

### `poll` (`web/src/app.tsx:205-206`)
- **Category:** `ui_component` · **Proprietary Score:** `0.99`
- **Signature:** `function poll()`
- **Behavior:** Proprietary routine `poll` (1 lines)

### `ask` (`web/src/app.tsx:210-211`)
- **Category:** `ui_component` · **Proprietary Score:** `0.99`
- **Signature:** `function ask()`
- **Behavior:** Proprietary routine `ask` (1 lines)

### `rules` (`web/src/ask.ts:64-65`)
- **Category:** `business_logic` · **Proprietary Score:** `0.65`
- **Signature:** `function rules(actions: { id: number; name: string }[])`
- **Behavior:** Proprietary routine `rules` (1 lines)

### `habit` (`web/src/ask.ts:155-159`)
- **Category:** `business_logic` · **Proprietary Score:** `0.65`
- **Signature:** `function habit(p: Partial<Filters>, id: string, label: string)`
- **Behavior:** Proprietary routine `habit` (4 lines)

### `parseAsk` (`web/src/ask.ts:161-162`)
- **Category:** `business_logic` · **Proprietary Score:** `0.65`
- **Signature:** `function parseAsk(text: string, actions: { id: number; name: string }[] = [])`
- **Behavior:** Proprietary routine `parseAsk` (1 lines)

### `undoAsk` (`web/src/ask.ts:205-206`)
- **Category:** `business_logic` · **Proprietary Score:** `0.65`
- **Signature:** `function undoAsk(text: string, actions: { id: number; name: string }[], defaults: Filters)`
- **Behavior:** Proprietary routine `undoAsk` (1 lines)

### `dropPart` (`web/src/ask.ts:214-219`)
- **Category:** `business_logic` · **Proprietary Score:** `0.65`
- **Signature:** `function dropPart(text: string, part: AskPart)`
- **Behavior:** Proprietary routine `dropPart` (5 lines)

### `MinuteClip` (`web/src/components/Actions.tsx:25-26`)
- **Category:** `ui_component` · **Proprietary Score:** `0.93`
- **Signature:** `function MinuteClip({ clip, sound, onReady }: { clip: ActionClip; sound: boolean; onReady?: (id: string)`
- **Behavior:** Proprietary routine `MinuteClip` (1 lines)

### `Flipbook` (`web/src/components/Actions.tsx:41-44`)
- **Category:** `ui_component` · **Proprietary Score:** `0.93`
- **Signature:** `function Flipbook(props: ClipProps)`
- **Behavior:** Proprietary routine `Flipbook` (3 lines)

### `SpriteFlip` (`web/src/components/Actions.tsx:46-47`)
- **Category:** `ui_component` · **Proprietary Score:** `0.93`
- **Signature:** `function SpriteFlip({ clip, sound, onReady }: ClipProps)`
- **Behavior:** Proprietary routine `SpriteFlip` (1 lines)

### `Editor` (`web/src/components/Actions.tsx:66-67`)
- **Category:** `ui_component` · **Proprietary Score:** `0.93`
- **Signature:** `function Editor({ initial, onDone }: { initial?: ActionInfo; onDone: (a: ActionInfo | null)`
- **Behavior:** Proprietary routine `Editor` (1 lines)

### `Teach` (`web/src/components/Actions.tsx:101-102`)
- **Category:** `ui_component` · **Proprietary Score:** `0.93`
- **Signature:** `function Teach({ action, onChange }: { action: ActionInfo; onChange: (a: ActionInfo)`
- **Behavior:** Proprietary routine `Teach` (1 lines)

### `HeadNote` (`web/src/components/Actions.tsx:227-228`)
- **Category:** `ui_component` · **Proprietary Score:** `0.93`
- **Signature:** `function HeadNote({ a }: { a: ActionInfo })`
- **Behavior:** Proprietary routine `HeadNote` (1 lines)

### `ChatNote` (`web/src/components/Actions.tsx:242-243`)
- **Category:** `ui_component` · **Proprietary Score:** `0.93`
- **Signature:** `function ChatNote({ a }: { a: ActionInfo })`
- **Behavior:** Proprietary routine `ChatNote` (1 lines)

### `Live` (`web/src/components/Actions.tsx:256-257`)
- **Category:** `ui_component` · **Proprietary Score:** `0.93`
- **Signature:** `function Live({ action }: { action: ActionInfo })`
- **Behavior:** Proprietary routine `Live` (1 lines)

