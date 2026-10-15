# Feature Blueprint: Overlay 2026 10 14 Mv8bx8qq

> Subsystem `Overlay 2026 10 14 Mv8bx8qq` covering 5 code slices, 0 API routes, and 0 data models.

## UI Screens & Components

| View / Component | Kind | Source | Rendered Elements | Visible Copy / Controls | Bound Routes |
|---|---|---|---|---|---|
| **`overlay_2026_10_14_mv8bx8qq.min`** | `component` | `overlay_2026_10_14_mv8bx8qq.min.js` | <img> | – | – |

## Proprietary Logic Slices

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

### `popClose` (`overlay_2026_10_14_mv8bx8qq.min.js:1-2`)
- **Category:** `business_logic` · **Proprietary Score:** `0.79`
- **Signature:** `function popClose(e)`
- **Behavior:** Proprietary routine `popClose` (1 lines)
- **Calls:** `chatroom_2026_10_14_01.js:getElementById`
- **Called By:** `overlay_2026_10_14_mv8bx8qq.min.js:popOverlay`

### `popRegistration` (`overlay_2026_10_14_mv8bx8qq.min.js:1-2`)
- **Category:** `business_logic` · **Proprietary Score:** `0.79`
- **Signature:** `function popRegistration(e)`
- **Behavior:** Proprietary routine `popRegistration` (1 lines)
- **Calls:** `chatroom_2026_10_14_01.js:getElementById`
- **Called By:** `chatroom_2026_10_14_01.js:handleEmojiClick`, `chatroom_2026_10_14_01.js:handleTransformMenuTouched`

### `popPendingReg` (`overlay_2026_10_14_mv8bx8qq.min.js:1-2`)
- **Category:** `business_logic` · **Proprietary Score:** `0.79`
- **Signature:** `function popPendingReg()`
- **Behavior:** Proprietary routine `popPendingReg` (1 lines)
- **Calls:** `overlay_2026_10_14_mv8bx8qq.min.js:popOverlay`

