# Feature Blueprint: Billing Modal 2026 10 14 Mv8duwhx

> Subsystem `Billing Modal 2026 10 14 Mv8duwhx` covering 152 code slices, 5 API routes, and 0 data models.

## UI Screens & Components

| View / Component | Kind | Source | Rendered Elements | Visible Copy / Controls | Bound Routes |
|---|---|---|---|---|---|
| **`billing_modal_2026_10_14_mv8duwhx.min`** | `component` | `billing_modal_2026_10_14_mv8duwhx.min.js` | <iframe> | ${Translator.getTranslation( · CREDITS · Change · Privacy Policy · Terms & Conditions | /privacy.php, /terms.php, /webservices/billing-interface.php, /ws/chat/error.php, /ws/rooms/relog.php |

## API Surface

| Method | Pattern | Handler / Source | Request Fields | Response Fields |
|---|---|---|---|---|
| `GET` | `/terms.php` | `billing_modal_2026_10_14_mv8duwhx.min.js:1` | – | – |
| `GET` | `/privacy.php` | `billing_modal_2026_10_14_mv8duwhx.min.js:1` | – | – |
| `GET` | `/ws/rooms/relog.php` | `billing_modal_2026_10_14_mv8duwhx.min.js:1` | – | – |
| `GET` | `/webservices/billing-interface.php` | `billing_modal_2026_10_14_mv8duwhx.min.js:1` | – | – |
| `GET` | `/ws/chat/error.php` | `billing_modal_2026_10_14_mv8duwhx.min.js:1` | – | – |

## Proprietary Logic Slices

### `piiScrub` (`inline_1.js:21-47`)
- **Category:** `business_logic` · **Proprietary Score:** `0.79`
- **Signature:** `function piiScrub(inputURL)`
- **Behavior:** Proprietary routine `piiScrub` (26 lines)
- **Calls:** `chatroom_2026_10_14_01.js:replace`

### `hideOrDisplayPassword` (`inline_10.js:42-76`)
- **Category:** `business_logic` · **Proprietary Score:** `0.72`
- **Signature:** `function hideOrDisplayPassword()`
- **Behavior:** Proprietary routine `hideOrDisplayPassword` (34 lines)
- **Calls:** `chatroom_2026_10_14_01.js:getElementById`

### `setUTMCookies` (`inline_35.js:22-63`)
- **Category:** `business_logic` · **Proprietary Score:** `0.65`
- **Signature:** `function setUTMCookies()`
- **Behavior:** Proprietary routine `setUTMCookies` (41 lines)

### `piiScrub` (`inline_35.js:76-102`)
- **Category:** `business_logic` · **Proprietary Score:** `0.65`
- **Signature:** `function piiScrub(inputURL)`
- **Behavior:** Proprietary routine `piiScrub` (26 lines)
- **Calls:** `chatroom_2026_10_14_01.js:replace`

### `setupDeferredActions` (`inline_48.js:116-171`)
- **Category:** `business_logic` · **Proprietary Score:** `0.79`
- **Signature:** `function setupDeferredActions()`
- **Behavior:** Proprietary routine `setupDeferredActions` (55 lines)

### `executeDeferredActions` (`inline_48.js:181-231`)
- **Category:** `business_logic` · **Proprietary Score:** `0.79`
- **Signature:** `function executeDeferredActions()`
- **Behavior:** Proprietary routine `executeDeferredActions` (50 lines)

### `minified_api_caller_1` (`inline_48.js:230-243`)
- **Category:** `api_handler` · **Proprietary Score:** `0.79`
- **Signature:** `bundle_slice(1) -> fetch('/models/bios/MODEL/bio.php')`
- **Behavior:** Minified bundle slice invoking `/models/bios/MODEL/bio.php`
- **Triggers / Routes:** `/models/bios/MODEL/bio.php`
- **Calls:** `chatroom_2026_10_14_01.js:getElementById`

### `minified_api_caller_1` (`inline_57.js:14-37`)
- **Category:** `api_handler` · **Proprietary Score:** `0.79`
- **Signature:** `bundle_slice(1) -> fetch('/webservices/save-iovation-blackbox.php')`
- **Behavior:** Minified bundle slice invoking `/webservices/save-iovation-blackbox.php`
- **Triggers / Routes:** `/webservices/save-iovation-blackbox.php`

### `buildLightbox` (`model_access_2026_09_29_01.js:14-15`)
- **Category:** `business_logic` · **Proprietary Score:** `0.99`
- **Signature:** `function buildLightbox()`
- **Behavior:** Proprietary routine `buildLightbox` (1 lines)
- **Calls:** `chatroom_2026_10_14_01.js:createElement`, `model_access_2026_09_29_01.js:closeLightbox`
- **Called By:** `model_access_2026_09_29_01.js:openLightbox`

### `lightboxOpen` (`model_access_2026_09_29_01.js:15-16`)
- **Category:** `business_logic` · **Proprietary Score:** `0.99`
- **Signature:** `function lightboxOpen()`
- **Behavior:** Proprietary routine `lightboxOpen` (1 lines)
- **Called By:** `model_access_2026_09_29_01.js:init`

### `openLightbox` (`model_access_2026_09_29_01.js:16-17`)
- **Category:** `business_logic` · **Proprietary Score:** `0.99`
- **Signature:** `function openLightbox(img)`
- **Behavior:** Proprietary routine `openLightbox` (1 lines)
- **Calls:** `model_access_2026_09_29_01.js:buildLightbox`
- **Called By:** `model_access_2026_09_29_01.js:init`

### `closeLightbox` (`model_access_2026_09_29_01.js:17-18`)
- **Category:** `business_logic` · **Proprietary Score:** `0.99`
- **Signature:** `function closeLightbox()`
- **Behavior:** Proprietary routine `closeLightbox` (1 lines)
- **Called By:** `model_access_2026_09_29_01.js:buildLightbox`

### `resolveSelectors` (`model_access_2026_09_29_01.js:18-20`)
- **Category:** `business_logic` · **Proprietary Score:** `0.99`
- **Signature:** `function resolveSelectors(options)`
- **Behavior:** Proprietary routine `resolveSelectors` (2 lines)
- **Called By:** `model_access_2026_09_29_01.js:init`

### `init` (`model_access_2026_09_29_01.js:20-25`)
- **Category:** `business_logic` · **Proprietary Score:** `0.99`
- **Signature:** `function init(root,options)`
- **Behavior:** Proprietary routine `init` (5 lines)
- **Calls:** `model_access_2026_09_29_01.js:lightboxOpen`, `model_access_2026_09_29_01.js:openLightbox`, `model_access_2026_09_29_01.js:refresh`, `model_access_2026_09_29_01.js:resolveSelectors`

### `refresh` (`model_access_2026_09_29_01.js:25-28`)
- **Category:** `business_logic` · **Proprietary Score:** `0.99`
- **Signature:** `function refresh(root)`
- **Behavior:** Proprietary routine `refresh` (3 lines)
- **Calls:** `chatroom_2026_10_14_01.js:createElement`
- **Called By:** `model_access_2026_09_29_01.js:init`

### `buildChatServerEventsConstants` (`model_access_2026_09_29_01.js:28-30`)
- **Category:** `business_logic` · **Proprietary Score:** `0.99`
- **Signature:** `function buildChatServerEventsConstants()`
- **Behavior:** Proprietary routine `buildChatServerEventsConstants` (2 lines)

### `zeroCheck` (`model_access_2026_09_29_01.js:184-185`)
- **Category:** `business_logic` · **Proprietary Score:** `0.99`
- **Signature:** `function zeroCheck(number)`
- **Behavior:** Proprietary routine `zeroCheck` (1 lines)

### `handleKeyPress` (`model_access_2026_09_29_01.js:279-280`)
- **Category:** `business_logic` · **Proprietary Score:** `0.99`
- **Signature:** `function handleKeyPress(e)`
- **Behavior:** Proprietary routine `handleKeyPress` (1 lines)

### `handleKeyUp` (`model_access_2026_09_29_01.js:279-283`)
- **Category:** `business_logic` · **Proprietary Score:** `0.99`
- **Signature:** `function handleKeyUp(e)`
- **Behavior:** Proprietary routine `handleKeyUp` (4 lines)

### `onSaveSettingsSuccess` (`model_access_2026_09_29_01.js:597-598`)
- **Category:** `business_logic` · **Proprietary Score:** `0.99`
- **Signature:** `function onSaveSettingsSuccess()`
- **Behavior:** Proprietary routine `onSaveSettingsSuccess` (1 lines)

### `getTypeRank` (`model_access_2026_09_29_01.js:516-517`)
- **Category:** `business_logic` · **Proprietary Score:** `0.99`
- **Signature:** `function getTypeRank(file)`
- **Behavior:** Proprietary routine `getTypeRank` (1 lines)

### `Validator` (`registration_validation_2026_03_26_mn7vumkn.min.js:1-2`)
- **Category:** `business_logic` · **Proprietary Score:** `0.79`
- **Signature:** `function Validator(e,t,r)`
- **Behavior:** Proprietary routine `Validator` (1 lines)
- **Calls:** `chatroom_2026_10_14_01.js:getElementById`

### `decorateObserver` (`vs-player_2026_10_13_mv7cakwh.min.js:1-2`)
- **Category:** `business_logic` · **Proprietary Score:** `0.99`
- **Signature:** `function decorateObserver(e)`
- **Behavior:** Proprietary routine `decorateObserver` (1 lines)

### `StreamSignalling` (`vs-player_2026_10_13_mv7cakwh.min.js:1-2`)
- **Category:** `business_logic` · **Proprietary Score:** `0.99`
- **Signature:** `function StreamSignalling(e)`
- **Behavior:** Proprietary routine `StreamSignalling` (1 lines)

### `WebRtcConnectionManager` (`vs-player_2026_10_13_mv7cakwh.min.js:1-2`)
- **Category:** `business_logic` · **Proprietary Score:** `0.99`
- **Signature:** `function WebRtcConnectionManager()`
- **Behavior:** Proprietary routine `WebRtcConnectionManager` (1 lines)

### `PlaybackSignalling` (`vs-player_2026_10_13_mv7cakwh.min.js:1-2`)
- **Category:** `business_logic` · **Proprietary Score:** `0.99`
- **Signature:** `function PlaybackSignalling(e)`
- **Behavior:** Proprietary routine `PlaybackSignalling` (1 lines)

### `WebRTCMonitor` (`vs-player_2026_10_13_mv7cakwh.min.js:1-2`)
- **Category:** `business_logic` · **Proprietary Score:** `0.99`
- **Signature:** `function WebRTCMonitor(e)`
- **Behavior:** Proprietary routine `WebRTCMonitor` (1 lines)

### `WebRTCMonitorV2` (`vs-player_2026_10_13_mv7cakwh.min.js:1-2`)
- **Category:** `business_logic` · **Proprietary Score:** `0.99`
- **Signature:** `function WebRTCMonitorV2(e)`
- **Behavior:** Proprietary routine `WebRTCMonitorV2` (1 lines)

### `resizeOverlay` (`overlay_2026_10_14_mv8bx8qq.min.js:1-2`)
- **Category:** `business_logic` · **Proprietary Score:** `0.79`
- **Signature:** `function resizeOverlay()`
- **Behavior:** Proprietary routine `resizeOverlay` (1 lines)
- **Calls:** `chatroom_2026_10_14_01.js:getElementById`

### `popOverlay` (`overlay_2026_10_14_mv8bx8qq.min.js:1-2`)
- **Category:** `business_logic` · **Proprietary Score:** `0.79`
- **Signature:** `function popOverlay(e,t)`
- **Behavior:** Proprietary routine `popOverlay` (1 lines)
- **Calls:** `chatroom_2026_10_14_01.js:getElementById`, `overlay_2026_10_14_mv8bx8qq.min.js:popClose`
- **Called By:** `overlay_2026_10_14_mv8bx8qq.min.js:popPendingReg`

