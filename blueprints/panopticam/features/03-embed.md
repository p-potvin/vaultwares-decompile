# Feature Blueprint: Embed

> Subsystem `Embed` covering 16 code slices, 0 API routes, and 1 data models.

## Clean-Room Behavioral Summary (Gemma 4 E2B)

The Embed subsystem is a machine learning service responsible for generating high-dimensional vector embeddings from input data. Its primary function is to ingest a list of items (dictionaries), process them through a configured model, and output a list of corresponding float vectors. The data flow begins with input items, which are processed by proprietary routines for media handling (image loading, video frame extraction, audio wave manipulation) and then passed to the core inference engine (`one` or `embed`). The system manages hardware resources by selecting the optimal execution device (CPU, CUDA, or specialized hardware) based on configuration, ensuring efficient tensor operations during the embedding process.

To rebuild this subsystem, a developer must implement a modular pipeline centered on model inference and resource management. The core architecture requires defining a `Model` interface capable of loading and executing inference (via `__init__` and `one` methods). The developer must also implement the device selection logic (`choose_device`) and integrate the proprietary media processing utilities (`ffmpeg`, `video_frames`, etc.) into the input preparation stage. The primary entry point, `embed`, should orchestrate these steps: selecting the device, preparing the input data, executing the model inference, and returning the resulting embedding vectors. Monitoring and logging capabilities (`health`, `log_message`) should be implemented as auxiliary components for operational visibility.

## Data Models & Tables

- **`Embedder`** (dataclass, `embed/server.py:133`): `torch: any`, `cuda: any`, `proc: any`, `model: any`, `prompts: any`, `lock: any`, `served: any`, `errors: any`
- **`Embedder`** (dataclass, `vision/experiments/eg2_embed.py:59`): `proc: any`, `model: any`

## Proprietary Logic Slices

### `choose_device` (`embed/server.py:70-82`)
- **Category:** `business_logic` · **Proprietary Score:** `0.65`
- **Signature:** `def choose_device(want: str) -> tuple[str, str]`
- **Behavior:** (CUDA_VISIBLE_DEVICES value or "", label): the wanted card, else any card with room, else the CPU.

### `ffmpeg` (`embed/server.py:83-90`)
- **Category:** `business_logic` · **Proprietary Score:** `0.65`
- **Signature:** `def ffmpeg(args: list[str], data: bytes | None = None) -> bytes`
- **Behavior:** Proprietary routine `ffmpeg` (7 lines)

### `video_frames` (`embed/server.py:91-105`)
- **Category:** `business_logic` · **Proprietary Score:** `0.65`
- **Signature:** `def video_frames(path: str, n: int) -> Any`
- **Behavior:** Proprietary routine `video_frames` (14 lines)

### `audio_wave` (`embed/server.py:106-119`)
- **Category:** `business_logic` · **Proprietary Score:** `0.65`
- **Signature:** `def audio_wave(src: str | None, data: bytes | None, seconds: float) -> Any`
- **Behavior:** Proprietary routine `audio_wave` (13 lines)

### `load_image` (`embed/server.py:122-132`)
- **Category:** `business_logic` · **Proprietary Score:** `0.65`
- **Signature:** `def load_image(src) -> Any`
- **Behavior:** RGB, at most ``IMAGE_SIDE`` px a side; JPEGs are decoded at 1/2-1/8 scale by the decoder itself (``draft``).

### `__init__` (`embed/server.py:134-148`)
- **Category:** `business_logic` · **Proprietary Score:** `0.65`
- **Signature:** `def __init__(self) -> None`
- **Behavior:** `__init__` — runs model inference / tensor embedding

### `one` (`embed/server.py:149-188`)
- **Category:** `business_logic` · **Proprietary Score:** `0.65`
- **Signature:** `def one(self, item: dict) -> list[float]`
- **Behavior:** `one` — runs model inference / tensor embedding

### `_run` (`embed/server.py:190-198`)
- **Category:** `business_logic` · **Proprietary Score:** `0.65`
- **Signature:** `def _run(project: str, caller: str | None) -> Any`
- **Behavior:** A ModelRun for one input (a no-op context without the ADK).

### `embed` (`embed/server.py:199-222`)
- **Category:** `business_logic` · **Proprietary Score:** `0.65`
- **Signature:** `def embed(self, items: list[dict], dim: int, project: str = "unknown", caller: str | None = None) -> list[list[float]]`
- **Behavior:** Proprietary routine `embed` (23 lines)

### `health` (`embed/server.py:223-233`)
- **Category:** `business_logic` · **Proprietary Score:** `0.65`
- **Signature:** `def health(self) -> dict`
- **Behavior:** `health` — runs model inference / tensor embedding

### `_telemetry_stats` (`embed/server.py:234-245`)
- **Category:** `business_logic` · **Proprietary Score:** `0.65`
- **Signature:** `def _telemetry_stats() -> dict | bool`
- **Behavior:** The ADK recorder's counters (queued / drained / posted / spooled …), or False without it.

### `log_message` (`embed/server.py:251-253`)
- **Category:** `business_logic` · **Proprietary Score:** `0.65`
- **Signature:** `def log_message(self, fmt: str, *args) -> None`
- **Behavior:** Proprietary routine `log_message` (2 lines)

### `_json` (`embed/server.py:254-261`)
- **Category:** `api_handler` · **Proprietary Score:** `0.65`
- **Signature:** `def _json(self, code: int, body: dict) -> None`
- **Behavior:** Proprietary routine `_json` (7 lines)

### `do_GET` (`embed/server.py:262-266`)
- **Category:** `api_handler` · **Proprietary Score:** `0.65`
- **Signature:** `def do_GET(self) -> None`
- **Behavior:** Proprietary routine `do_GET` (4 lines)

### `do_POST` (`embed/server.py:267-292`)
- **Category:** `api_handler` · **Proprietary Score:** `0.65`
- **Signature:** `def do_POST(self) -> None`
- **Behavior:** `do_POST` — runs model inference / tensor embedding

### `main` (`embed/server.py:293-313`)
- **Category:** `business_logic` · **Proprietary Score:** `0.65`
- **Signature:** `def main() -> None`
- **Behavior:** Proprietary routine `main` (20 lines)

