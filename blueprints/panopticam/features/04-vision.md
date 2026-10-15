# Feature Blueprint: Vision

> Subsystem `Vision` covering 702 code slices, 7 API routes, and 55 data models.

## API Surface

| Method | Pattern | Handler / Source | Request Fields | Response Fields |
|---|---|---|---|---|
| `POST` | `/merge` | `vision/recordings.py:216` | id, pairs | results |
| `POST` | `/merge/undo` | `vision/recordings.py:244` | id, pairs, keep, gone, rename | results, recycled |
| `POST` | `/merge/batch` | `vision/recordings.py:246` | pairs, keep, gone, rename | results, recycled |
| `POST` | `/rec` | `vision/recordings.py:261` | – | recycled, files |
| `POST` | `/index` | `vision/recordings.py:280` | – | files |
| `POST` | `/merge/candidates` | `vision/recordings.py:289` | – | – |
| `GET` | `/api/vision/moments/vecs` | `vision/test_moments_embed.py:49` | – | have |

## Data Models & Tables

- **`Datasets`** (dataclass, `backend/src/panopticam/datasets.py:99`): `store: any`, `root: any`, `verdicts: any`, `ingest: any`
- **`moments`** (sql_table, `backend/src/panopticam/moments.py:103`): `id: integer`, `key: text`, `name: text`, `ts: integer`, `at: integer`, `score: real`, `viewers: integer`, `why: text`, `parts: text`, `tokens: integer`, `toy: integer`, `bytes: integer`, `kept: integer`, `rating: integer,`
- **`Speech`** (dataclass, `backend/src/panopticam/speech.py:143`): `index: any`, `store: any`, `sampler: any`, `rec: any`, `speaks: any`, `english: any`, `queue: any`, `heard: any`, `confident: any`, `rests: any`, `recent: any`, `lines: any`, `dropped: any`
- **`personas`** (sql_table, `backend/src/panopticam/vision.py:127`): `key: text`, `persona_id: integer`, `label: text`, `verified: integer`, `match: real`, `ts: integer`
- **`guests`** (sql_table, `backend/src/panopticam/vision.py:143`): `key: text`, `persona_id: integer`, `label: text`, `verified: integer`, `sim: real`, `confirmed: integer`, `ts: integer`
- **`Embedder`** (dataclass, `embed/server.py:133`): `torch: any`, `cuda: any`, `proc: any`, `model: any`, `prompts: any`, `lock: any`, `served: any`, `errors: any`
- **`personas`** (sql_table, `vision/atlas.py:89`): `id: integer`, `kfc_name: text`, `label: text`, `created: integer`, `sightings: integer`
- **`sightings`** (sql_table, `vision/atlas.py:96`): `id: integer`, `room: text`, `ts: integer`, `persona_id: integer`, `score: real`, `match: real,`, `emb: blob`
- **`meta`** (sql_table, `vision/atlas.py:107`): `k: text`
- **`guests`** (sql_table, `vision/atlas.py:108`): `room: text`, `persona_id: integer`, `ts: integer`, `sim: real`
- **`dataset_names`** (sql_table, `vision/atlas.py:115`): `persona_id: integer`, `name: text`, `created: integer`
- **`Verdict`** (dataclass, `vision/atlas.py:124`): `persona_id: int`, `label: str`, `verified: bool`
- **`Atlas`** (dataclass, `vision/atlas.py:137`): `path: any`, `gallery: any`, `lock: any`, `db: any`, `kfc_names: any`, `kfc_mat: any`, `kfc_owner: any`, `p_sum: any`, `p_n: any`, `p_label: any`, `p_kfc: any`, `inbox_today: any`, `inbox_day: any`
- **`BackupPull`** (dataclass, `vision/backups.py:64`): `dest: any`, `atlas_db: any`
- **`Datasets`** (dataclass, `vision/datasets.py:90`): `gallery: any`, `atlas: any`, `faces: any`, `nudity: any`, `prior_name: any`, `staging: any`, `incoming: any`, `pending: any`, `out: any`
- **`Embedder`** (dataclass, `vision/experiments/eg2_embed.py:59`): `proc: any`, `model: any`
- **`ArchiveRooms`** (dataclass, `vision/ingest.py:106`): `folder: any`, `cache: any`, `first: any`, `name: any`, `ready: any`, `lock: any`
- **`GalleryLock`** (dataclass, `vision/ingest.py:177`): `path: any`, `local: any`, `fh: any`
- **`Claims`** (dataclass, `vision/ingest.py:231`): `dir: any`
- **`GalleryHost`** (dataclass, `vision/ingest.py:280`): `log_path: Path`, `proc: any`, `failures: any`, `lock: any`, `used: any`
- **`Ingest`** (dataclass, `vision/ingest.py:393`): `on_change: Callable[[], object] | None`, `gallery: any`, `api: any`, `remote: any`, `rclone: any`, `root: any`, `changed: any`, `reread_at: any`, `write_lock: any`, `claims: any`, `claim_lock: any`, `taken: any`, `busy: any`, `queue: any`, `offloaded_at: any`, `local_sources: any`, `host: any`, `rooms: any`, `taggers: any`
- **`LiveTasteFit`** (dataclass, `vision/live_taste.py:81`): `emb: any`, `sent_n: any`
- **`LocalSources`** (dataclass, `vision/local_sources.py:78`): `roots: list[Path] | None`, `gallery: any`, `queue: any`, `api: any`, `state_path: any`, `lock: any`, `state: any`
- **`MemGuard`** (dataclass, `vision/memguard.py:87`): `hosts: any`, `notify: any`, `last: any`, `reported: any`, `told: any`, `embed_restarted: any`, `embed_down_since: any`, `embed_told_down: any`
- **`Merger`** (dataclass, `vision/merge.py:106`): `rooms_dir: Path | None`, `gallery: any`, `atlas: any`, `lock: any`, `ingest_root: any`, `on_change: any`, `journals: any`, `busy: any`
- **`MergeDesk`** (dataclass, `vision/merge.py:574`): `merger: any`, `atlas: any`, `pairs: any`, `at: any`, `computing: any`
- **`MomentsArchive`** (dataclass, `vision/moments_archive.py:54`): `dest: any`, `after_days: any`, `saved: any`
- **`moments`** (sql_table, `vision/moments_embed.py:62`): `file: text`, `key: text,`, `v: blob,`, `a: blob,`
- **`probes`** (sql_table, `vision/moments_embed.py:72`): `name: text`
- **`MomentsEmbed`** (dataclass, `vision/moments_embed.py:111`): `folder: any`, `db: any`, `lock: any`, `api: any`, `live: any`
- **`MomentsPull`** (dataclass, `vision/moments_pull.py:232`): `tastes: Callable[[], dict[str, float]] | None`, `dest: any`, `base: any`, `token: any`, `api: any`, `life: any`
- **`Liveliness`** (dataclass, `vision/moment_quality.py:75`): `clap_dir: any`, `model_path: any`, `threads: any`, `model: any`
- **`NudeNet`** (dataclass, `vision/nudity.py:35`): `sess: any`, `inp: any`
- **`profiles`** (sql_table, `vision/onlyfans.py:73`): `handle: text`, `source: text`, `added: integer`, `status: text`, `subscribed: integer`, `visited: integer`, `media: integer`, `note: text`
- **`subs`** (sql_table, `vision/onlyfans.py:83`): `ts: integer`
- **`meta`** (sql_table, `vision/onlyfans.py:84`): `k: text`
- **`refusals`** (sql_table, `vision/onlyfans.py:85`): `ts: integer`
- **`Library`** (dataclass, `vision/recordings.py:78`): `folder: any`, `cache: any`
- **`Tunnel`** (dataclass, `vision/recordings.py:381`): `proc: any`, `port: any`
- **`Review`** (dataclass, `vision/review_toys.py:59`): `cls: any`, `drag: any`, `mouse: any`, `history: any`, `kept: any`
- **`Sound`** (dataclass, `vision/sound.py:96`): `audio: any`
- **`Speech`** (dataclass, `vision/speech.py:125`): `encoder: any`, `decoder: any`, `past: any`, `present: any`, `name: any`, `codes: any`, `lang_ids: any`, `nospeech_id: any`, `transcribe_id: any`, `notimestamps_id: any`, `pieces: any`
- **`identities`** (sql_table, `vision/test_atlas.py:33`): `id: integer`
- **`face_crops`** (sql_table, `vision/test_atlas.py:34`): `identity_id: integer,`
- **`config`** (sql_table, `vision/test_atlas.py:35`): `key: text,`
- **`identities`** (sql_table, `vision/test_atlas.py:222`): `id: integer`
- **`face_crops`** (sql_table, `vision/test_atlas.py:223`): `identity_id: integer,`
- **`config`** (sql_table, `vision/test_atlas.py:224`): `key: text,`
- **`sightings`** (sql_table, `vision/test_backups.py:19`): `room: text,`, `out: =`, `restored: =`, `assert: sqlite3.connect(restored).execute("select`, `assert: not`
- **`identities`** (sql_table, `vision/test_ingest.py:125`): `id: integer`
- **`face_crops`** (sql_table, `vision/test_ingest.py:126`): `identity_id: integer,`
- **`identities`** (sql_table, `vision/test_merge.py:16`): `id: integer`, `threshold: real`
- **`face_crops`** (sql_table, `vision/test_merge.py:18`): `id: integer`, `model_name: text`, `gender: integer,`, `is_exemplar: integer`
- **`task_logs`** (sql_table, `vision/test_merge.py:22`): `id: integer`, `duration_ms: integer`, `no_face_count: integer`
- **`config`** (sql_table, `vision/test_merge.py:25`): `key: text`
- **`ToyProposals`** (dataclass, `vision/toys.py:34`): `folder: any`, `out: any`, `det: any`, `failed: any`
- **`ToyPull`** (dataclass, `vision/toys.py:86`): `proposals: ToyProposals | None`, `dest: any`, `remote: any`, `what: any`
- **`PanopticamToyDetector`** (dataclass, `vision/toy_detector_bridge.py:34`): `weights: any`, `mtime: any`, `detector: any`
- **`Face`** (dataclass, `vision/videos.py:41`): `emb: np.ndarray`, `score: float`, `frame: int`
- **`Limiter`** (dataclass, `vision/worker.py:115`): `gap: any`, `next: any`, `lock: any`
- **`Models`** (dataclass, `vision/worker.py:227`): `siglip: any`, `faces: any`, `nudity: any`

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

### `_row` (`backend/src/panopticam/actions.py:461-469`)
- **Category:** `business_logic` · **Proprietary Score:** `0.79`
- **Signature:** `def _row(r: dict[str, Any]) -> dict[str, Any]`
- **Behavior:** Proprietary routine `_row` (8 lines)

