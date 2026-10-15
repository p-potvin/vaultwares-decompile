# Chat with Live Cam Girls for Free | Flirt4Free — Clean-Room Architecture Blueprint

- **Target:** `https://www.flirt4free.com/?service=girls`
- **Extracted:** `Thu, 15 Oct 2026 05:38`
- **Detected Stack:** JavaScript/TypeScript · Web SPA / HTTP
- **Metrics:** 49 files scanned · 169 proprietary slices · 30 API routes · 1 data models · 15 state stores · 18 UI views/components · 18 feature clusters

## Domain & Content Context (What the App Parses & Shows)

- **Content & Domain Signals:** `director`, `flirt4free`, `lovense`, `lush`, `nude`, `performer`, `private_show`, `tippers`, `tokens`, `viewers`
- **Media & Inference Modalities:** CLAP Audio/Acoustic Liveliness Scoring · HLS Live Video Playback · HLS Live Video Streams (.m3u8) · Speech-to-Text Transcription & Language ID
- **Referenced External Domains / Providers:** `a.exoclick.com`, `affiliates.flirt4free.com`, `aomedia.org`, `brothercake.com`, `cdn.filestackcontent.com`, `cdn5.vscdns.com`, `code.google.com`, `d10lpsik1i8c69.cloudfront.net`, `flirt4free.com`, `fonts.googleapis.com`, `google.com`, `helpcenter.getadblock.com`, `hw-cdn2.adtng.com`, `javascript.nwbox.com`, `live-screencaps.vscdns.com`, `mpsnare.iesnare.com`, `ow.ly`, `studios.flirt4free.com`, `support.apple.com`, `support.google.com`, `twemoji.maxcdn.com`, `twitter.com`, `us-central1-vsm-bigdata.cloudfunctions.net`, `video-gpu001-ll-tx.vs3.com`

## UI Pages, Views & Visual Anatomy (`models/ui-views.json`)

| View / Screen | Kind | Source File | Rendered Elements | Visible Copy / Controls | Bound API Routes |
|---|---|---|---|---|---|
| **`Chat with Live Cam Girls for Free | Flirt4Free`** | `dom_section` | `index.html` | <header> (×2), <nav> (×2), <main> (×3), <section> (×6), <form> (×17), <video> (×16), <table> (×4) | Are you sure you want to refresh the page? · BECOME A VIP MEMBER · BOOST AMOUNT: · BOOST ME! · BUY MORE CREDITS · Chrome on Android | – |
| **`inline_1`** | `component` | `inline_1.js` | JSX | – | – |
| **`inline_2`** | `component` | `inline_2.js` | JSX | – | – |
| **`inline_48`** | `component` | `inline_48.js` | <input> | Click a model name to go to their bio page · Matching Models | /models/bios/MODEL/bio.php |
| **`inline_58`** | `component` | `inline_58.js` | JSX | – | – |
| **`model_access_2026_09_29_01`** | `component` | `model_access_2026_09_29_01.js` | JSX | Cannot connect to chat server · Insufficient Funds · Save · Search users · Upload | /dm-app/dm.php |
| **`registration_validation_2026_03_26_mn7vumkn.min`** | `component` | `registration_validation_2026_03_26_mn7vumkn.min.js` | JSX | Address is not valid · This field is required | – |
| **`vsmplayer_2026_10_05_muvonapf.min`** | `component` | `vsmplayer_2026_10_05_muvonapf.min.js` | <input> | +i+ · +l+ | – |
| **`vs-player_2026_10_13_mv7cakwh.min`** | `component` | `vs-player_2026_10_13_mv7cakwh.min.js` | <input> | Auto · Back to home page · Quality · Retry | /chat/error.php, /ws |
| **`overlay_2026_10_14_mv8bx8qq.min`** | `component` | `overlay_2026_10_14_mv8bx8qq.min.js` | <img> | – | – |
| **`hls.min`** | `component` | `hls.min.js` | JSX | – | – |
| **`freshchat_2025_08_22_men4v8rc.min`** | `component` | `freshchat_2025_08_22_men4v8rc.min.js` | <img> | – | – |
| **`user_account_2026_06_02_mpwto51z.min`** | `component` | `user_account_2026_06_02_mpwto51z.min.js` | JSX | – | /lookup-username.php, /lookup.php |
| **`show_modal_2025_03_12_m86bqe2a.min`** | `component` | `show_modal_2025_03_12_m86bqe2a.min.js` | JSX | – | – |
| **`vs-previewer_2025_08_11_me7goa16.min`** | `component` | `vs-previewer_2025_08_11_me7goa16.min.js` | JSX | – | – |
| **`tracker_2026_07_28_ms4yeok9.min`** | `component` | `tracker_2026_07_28_ms4yeok9.min.js` | JSX | – | /logout.php, /webservices/fe-feedback.php |
| **`chatroom_2026_10_14_01`** | `component` | `chatroom_2026_10_14_01.js` | <button>, <form>, <iframe>, <img>, <input>, <select>, <svg>, <textarea> | 2-Way Interactive - · A-Z · Add · Add to Home Screen · Adult Star · All | /about.php, /chat/booted.php, /chat/error.php, /phone.php, /photos/index.php, /reviews.php, /vip/member-benefits.php, /webservices/fanclub-join.php, /webservices/live-chat.php, /webservices/notes.php, /webservices/one-click-chat.php, /webservices/power-boost.php, /webservices/rooms-list.php, /webservices/voting-contests.php, /ws, /ws/billing/customer.php, /ws/users/block-model.php |
| **`billing_modal_2026_10_14_mv8duwhx.min`** | `component` | `billing_modal_2026_10_14_mv8duwhx.min.js` | <iframe> | ${Translator.getTranslation( · CREDITS · Change · Privacy Policy · Terms & Conditions | /privacy.php, /terms.php, /webservices/billing-interface.php, /ws/chat/error.php, /ws/rooms/relog.php |

## Feature Clusters (Semantic Domains)

| Cluster | File | Slices | Routes | Models |
|---|---|---|---|---|
| **Billing Modal 2026 10 14 Mv8duwhx** | `features/01-billing_modal_2026_10_14_mv8duwhx.md` | 152 | 5 | 0 |
| **Cam2cam 2026 06 01 Mpviem65** | `features/02-cam2cam_2026_06_01_mpviem65.md` | 152 | 1 | 0 |
| **Chatroom 2026 10 14 01** | `features/03-chatroom_2026_10_14_01.md` | 169 | 15 | 0 |
| **Hls** | `features/04-hls.md` | 1 | 0 | 0 |
| **Inline 1** | `features/05-inline_1.md` | 1 | 0 | 0 |
| **Inline 10** | `features/06-inline_10.md` | 1 | 0 | 0 |
| **Inline 16** | `features/07-inline_16.md` | 0 | 0 | 1 |
| **Inline 35** | `features/08-inline_35.md` | 2 | 0 | 0 |
| **Inline 48** | `features/09-inline_48.md` | 149 | 1 | 0 |
| **Inline 57** | `features/10-inline_57.md` | 152 | 1 | 0 |
| **Model Access 2026 09 29 01** | `features/11-model_access_2026_09_29_01.md` | 152 | 1 | 0 |
| **Overlay 2026 10 14 Mv8bx8qq** | `features/12-overlay_2026_10_14_mv8bx8qq.md` | 5 | 0 | 0 |
| **Registration Validation 2026 03 26 Mn7vumkn** | `features/13-registration_validation_2026_03_26_mn7vumkn.md` | 1 | 0 | 0 |
| **Tracker 2026 07 28 Ms4yeok9** | `features/14-tracker_2026_07_28_ms4yeok9.md` | 152 | 2 | 0 |
| **User Account 2026 06 02 Mpwto51z** | `features/15-user_account_2026_06_02_mpwto51z.md` | 152 | 2 | 0 |
| **User Notifications 2026 02 02 03** | `features/16-user_notifications_2026_02_02_03.md` | 152 | 1 | 0 |
| **Vs Player 2026 10 13 Mv7cakwh** | `features/17-vs_player_2026_10_13_mv7cakwh.md` | 152 | 1 | 0 |
| **Vs Previewer 2025 08 11 Me7goa16** | `features/18-vs_previewer_2025_08_11_me7goa16.md` | 1 | 0 | 0 |

## API Route Summary (`openapi.json`)

| Method | Pattern | Source | Linked Slices |
|---|---|---|---|
| `GET` | `/models/bios/MODEL/bio.php` | `inline_48.js:233` | inline_48.js:minified_api_caller_1, inline_57.js:minified_api_caller_1, tracker_2026_07_28_ms4yeok9.min.js:minified_api_caller_1 |
| `GET` | `/webservices/save-iovation-blackbox.php` | `inline_57.js:24` | inline_57.js:minified_api_caller_1, tracker_2026_07_28_ms4yeok9.min.js:minified_api_caller_1, tracker_2026_07_28_ms4yeok9.min.js:minified_api_caller_2 |
| `GET` | `/dm-app/dm.php` | `model_access_2026_09_29_01.js:456` | chatroom_2026_10_14_01.js:dataHandler, model_access_2026_09_29_01.js:buildLightbox, billing_modal_2026_10_14_mv8duwhx.min.js:minified_api_caller_2 |
| `GET` | `/chat/error.php` | `vs-player_2026_10_13_mv7cakwh.min.js:1` | vs-player_2026_10_13_mv7cakwh.min.js:decorateObserver, vs-player_2026_10_13_mv7cakwh.min.js:StreamSignalling, vs-player_2026_10_13_mv7cakwh.min.js:WebRtcConnectionManager, vs-player_2026_10_13_mv7cakwh.min.js:PlaybackSignalling, vs-player_2026_10_13_mv7cakwh.min.js:WebRTCMonitor, vs-player_2026_10_13_mv7cakwh.min.js:WebRTCMonitorV2 |
| `GET` | `/lookup.php` | `user_account_2026_06_02_mpwto51z.min.js:1` | user_account_2026_06_02_mpwto51z.min.js:minified_api_caller_1, user_account_2026_06_02_mpwto51z.min.js:minified_api_caller_2, user_account_2026_06_02_mpwto51z.min.js:minified_api_caller_3, user_account_2026_06_02_mpwto51z.min.js:minified_api_caller_4 |
| `GET` | `/lookup-username.php` | `user_account_2026_06_02_mpwto51z.min.js:1` | user_account_2026_06_02_mpwto51z.min.js:minified_api_caller_1, user_account_2026_06_02_mpwto51z.min.js:minified_api_caller_2, user_account_2026_06_02_mpwto51z.min.js:minified_api_caller_3, user_account_2026_06_02_mpwto51z.min.js:minified_api_caller_4 |
| `GET` | `/webservices/fe-feedback.php` | `tracker_2026_07_28_ms4yeok9.min.js:1` | tracker_2026_07_28_ms4yeok9.min.js:minified_api_caller_1, tracker_2026_07_28_ms4yeok9.min.js:minified_api_caller_2, chatroom_2026_10_14_01.js:dataHandler |
| `GET` | `/logout.php` | `tracker_2026_07_28_ms4yeok9.min.js:1` | tracker_2026_07_28_ms4yeok9.min.js:minified_api_caller_1, tracker_2026_07_28_ms4yeok9.min.js:minified_api_caller_2, user_account_2026_06_02_mpwto51z.min.js:minified_api_caller_4 |
| `GET` | `/ws/billing/customer.php` | `chatroom_2026_10_14_01.js:253` | billing_modal_2026_10_14_mv8duwhx.min.js:minified_api_caller_3, billing_modal_2026_10_14_mv8duwhx.min.js:minified_api_caller_5, chatroom_2026_10_14_01.js:dataHandler |
| `GET` | `/vip/member-benefits.php` | `chatroom_2026_10_14_01.js:277` | chatroom_2026_10_14_01.js:dataHandler, billing_modal_2026_10_14_mv8duwhx.min.js:minified_api_caller_3, billing_modal_2026_10_14_mv8duwhx.min.js:minified_api_caller_5 |
| `GET` | `/webservices/one-click-chat.php` | `chatroom_2026_10_14_01.js:301` | chatroom_2026_10_14_01.js:zipCodeListener, chatroom_2026_10_14_01.js:dataHandler, chatroom_2026_10_14_01.js:listenForClickOnRoom, chatroom_2026_10_14_01.js:handleInput |
| `GET` | `/webservices/live-chat.php` | `chatroom_2026_10_14_01.js:313` | chatroom_2026_10_14_01.js:dataHandler, chatroom_2026_10_14_01.js:listenForClickOnRoom, billing_modal_2026_10_14_mv8duwhx.min.js:minified_api_caller_5 |
| `GET` | `/chat/booted.php` | `chatroom_2026_10_14_01.js:377` | chatroom_2026_10_14_01.js:dataHandler, chatroom_2026_10_14_01.js:attemptLogin |
| `GET` | `/ws/users/block-model.php` | `chatroom_2026_10_14_01.js:703` | billing_modal_2026_10_14_mv8duwhx.min.js:minified_api_caller_3, chatroom_2026_10_14_01.js:dataHandler, billing_modal_2026_10_14_mv8duwhx.min.js:minified_api_caller_5 |
| `GET` | `/about.php` | `chatroom_2026_10_14_01.js:846` | chatroom_2026_10_14_01.js:dataHandler, chatroom_2026_10_14_01.js:handleInput |
| `GET` | `/phone.php` | `chatroom_2026_10_14_01.js:848` | chatroom_2026_10_14_01.js:dataHandler, tracker_2026_07_28_ms4yeok9.min.js:minified_api_caller_2, billing_modal_2026_10_14_mv8duwhx.min.js:minified_api_caller_5 |
| `GET` | `/webservices/rooms-list.php` | `chatroom_2026_10_14_01.js:1346` | chatroom_2026_10_14_01.js:unmuteAudioOnRoomClick, chatroom_2026_10_14_01.js:dataHandler, chatroom_2026_10_14_01.js:listenForClickOnRoom, chatroom_2026_10_14_01.js:evlist |
| `GET` | `/webservices/notes.php` | `chatroom_2026_10_14_01.js:1464` | chatroom_2026_10_14_01.js:dataHandler, billing_modal_2026_10_14_mv8duwhx.min.js:minified_api_caller_5, tracker_2026_07_28_ms4yeok9.min.js:minified_api_caller_1 |
| `GET` | `/webservices/power-boost.php` | `chatroom_2026_10_14_01.js:1509` | chatroom_2026_10_14_01.js:displayMessage, chatroom_2026_10_14_01.js:handleInput, chatroom_2026_10_14_01.js:dataHandler, tracker_2026_07_28_ms4yeok9.min.js:minified_api_caller_1 |
| `GET` | `/webservices/voting-contests.php` | `chatroom_2026_10_14_01.js:1602` | chatroom_2026_10_14_01.js:dataHandler, tracker_2026_07_28_ms4yeok9.min.js:minified_api_caller_1, billing_modal_2026_10_14_mv8duwhx.min.js:minified_api_caller_3 |
| `GET` | `/webservices/fanclub-join.php` | `chatroom_2026_10_14_01.js:1876` | chatroom_2026_10_14_01.js:dataHandler, chatroom_2026_10_14_01.js:testPlayerVersion, chatroom_2026_10_14_01.js:handleInput |
| `GET` | `/reviews.php` | `chatroom_2026_10_14_01.js:1985` | chatroom_2026_10_14_01.js:dataHandler, chatroom_2026_10_14_01.js:listenForClickOnRoom, chatroom_2026_10_14_01.js:evlist |
| `GET` | `/photos/index.php` | `chatroom_2026_10_14_01.js:2013` | chatroom_2026_10_14_01.js:dataHandler, chatroom_2026_10_14_01.js:loadModelImages, chatroom_2026_10_14_01.js:handleInput |
| `GET` | `/faq.php` | `cam2cam_2026_06_01_mpviem65.min.js:1` | cam2cam_2026_06_01_mpviem65.min.js:minified_api_caller_1, cam2cam_2026_06_01_mpviem65.min.js:minified_api_caller_2, tracker_2026_07_28_ms4yeok9.min.js:minified_api_caller_1 |
| `GET` | `/terms.php` | `billing_modal_2026_10_14_mv8duwhx.min.js:1` | billing_modal_2026_10_14_mv8duwhx.min.js:BillingPluginValidator, billing_modal_2026_10_14_mv8duwhx.min.js:minified_api_caller_1, billing_modal_2026_10_14_mv8duwhx.min.js:minified_api_caller_2, billing_modal_2026_10_14_mv8duwhx.min.js:minified_api_caller_3, billing_modal_2026_10_14_mv8duwhx.min.js:minified_api_caller_4, billing_modal_2026_10_14_mv8duwhx.min.js:minified_api_caller_5 |
| `GET` | `/privacy.php` | `billing_modal_2026_10_14_mv8duwhx.min.js:1` | billing_modal_2026_10_14_mv8duwhx.min.js:BillingPluginValidator, billing_modal_2026_10_14_mv8duwhx.min.js:minified_api_caller_1, billing_modal_2026_10_14_mv8duwhx.min.js:minified_api_caller_2, billing_modal_2026_10_14_mv8duwhx.min.js:minified_api_caller_3, billing_modal_2026_10_14_mv8duwhx.min.js:minified_api_caller_4, billing_modal_2026_10_14_mv8duwhx.min.js:minified_api_caller_5 |
| `GET` | `/ws/rooms/relog.php` | `billing_modal_2026_10_14_mv8duwhx.min.js:1` | billing_modal_2026_10_14_mv8duwhx.min.js:BillingPluginValidator, billing_modal_2026_10_14_mv8duwhx.min.js:minified_api_caller_1, billing_modal_2026_10_14_mv8duwhx.min.js:minified_api_caller_2, billing_modal_2026_10_14_mv8duwhx.min.js:minified_api_caller_3, billing_modal_2026_10_14_mv8duwhx.min.js:minified_api_caller_4, billing_modal_2026_10_14_mv8duwhx.min.js:minified_api_caller_5 |
| `GET` | `/webservices/billing-interface.php` | `billing_modal_2026_10_14_mv8duwhx.min.js:1` | billing_modal_2026_10_14_mv8duwhx.min.js:BillingPluginValidator, billing_modal_2026_10_14_mv8duwhx.min.js:minified_api_caller_1, billing_modal_2026_10_14_mv8duwhx.min.js:minified_api_caller_2, billing_modal_2026_10_14_mv8duwhx.min.js:minified_api_caller_3, billing_modal_2026_10_14_mv8duwhx.min.js:minified_api_caller_4, billing_modal_2026_10_14_mv8duwhx.min.js:minified_api_caller_5 |
| `GET` | `/ws/chat/error.php` | `billing_modal_2026_10_14_mv8duwhx.min.js:1` | billing_modal_2026_10_14_mv8duwhx.min.js:BillingPluginValidator, billing_modal_2026_10_14_mv8duwhx.min.js:minified_api_caller_1, billing_modal_2026_10_14_mv8duwhx.min.js:minified_api_caller_2, billing_modal_2026_10_14_mv8duwhx.min.js:minified_api_caller_3, billing_modal_2026_10_14_mv8duwhx.min.js:minified_api_caller_4, billing_modal_2026_10_14_mv8duwhx.min.js:minified_api_caller_5 |
| `GET` | `/ws/notifications/client/notification-delete.php` | `user_notifications_2026_02_02_03.js:12` | user_notifications_2026_02_02_03.js:minified_api_caller_1, billing_modal_2026_10_14_mv8duwhx.min.js:minified_api_caller_5, billing_modal_2026_10_14_mv8duwhx.min.js:minified_api_caller_3 |

## Stateful Workers & Stores (`models/stores.json`)

- **`window.__homePageGridConfig__`** (`window_global_store`, `inline_4.js`): keys `[col, class_name, sample_size, sample_size_hd, spotlight_col, spotlight_row, spotlight_sample_size, spotlight_sample_size_hd]` · methods `[]`
- **`window.__homePageConfig__`** (`window_global_store`, `inline_12.js`): keys `[Exoclick, retargeting, advertiser_id, config, cookie, Login, Registration]` · methods `[]`
- **`window.__homePageData__`** (`window_global_store`, `inline_16.js`): keys `[models, hsas_badge, credit_tier_badge, sexy_selfie_badge, jumpstart_badge, is_hls, video_blocked, room_status_char]` · methods `[]`
- **`window.CHAT_SERVER_COMMANDS_RAW`** (`window_global_store`, `model_access_2026_09_29_01.js`): keys `[]` · methods `[]`
- **`window.CHAT_EVENTS_TO_COMMANDS`** (`window_global_store`, `model_access_2026_09_29_01.js`): keys `[]` · methods `[]`
- **`window.CHAT_COMMANDS_TO_EVENTS`** (`window_global_store`, `model_access_2026_09_29_01.js`): keys `[]` · methods `[]`
- **`window.CHAT_CONNECTION_EVENTS`** (`window_global_store`, `model_access_2026_09_29_01.js`): keys `[]` · methods `[]`
- **`window.DM_MESSENGER_EVENT`** (`window_global_store`, `model_access_2026_09_29_01.js`): keys `[NO_ERROR, GENERAL_SYSTEM_ERROR, INSUFFICIENT_CREDITS, NOT_FOUND, ALREADY_REPORTED, INVALID_COMMAND, INVALID_ARGUEMENTS]` · methods `[]`
- **`window.DM_VIEW_EVENT`** (`window_global_store`, `model_access_2026_09_29_01.js`): keys `[NO_ERROR, GENERAL_SYSTEM_ERROR, INSUFFICIENT_CREDITS, NOT_FOUND, ALREADY_REPORTED, INVALID_COMMAND, INVALID_ARGUEMENTS, INACTIVE_RECIPIENT]` · methods `[]`
- **`window.DMDebug`** (`window_global_store`, `model_access_2026_09_29_01.js`): keys `[]` · methods `[]`
- **`window.DMMessenger`** (`window_global_store`, `model_access_2026_09_29_01.js`): keys `[model]` · methods `[]`
- **`window.DMConversationController`** (`window_global_store`, `model_access_2026_09_29_01.js`): keys `[open]` · methods `[]`
- **`window.DMConversationModel`** (`window_global_store`, `model_access_2026_09_29_01.js`): keys `[value]` · methods `[]`
- **`window.DMUpsellController`** (`window_global_store`, `model_access_2026_09_29_01.js`): keys `[]` · methods `[]`
- **`window.VSMP`** (`window_global_store`, `vsmplayer_2026_10_05_muvonapf.min.js`): keys `[]` · methods `[]`

